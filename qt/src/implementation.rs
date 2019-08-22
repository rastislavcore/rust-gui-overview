use grep_matcher::{Matcher, Captures};
use grep_regex::RegexMatcher;
use grep_searcher::Searcher;
use grep_searcher::sinks::UTF8;
use htmlescape::encode_minimal;
use crate::interface::*;
use spmc;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::{Mutex, Arc, mpsc};
use std::thread;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use walkdir::{DirEntry, WalkDir};

struct Grepper {
    receiver: mpsc::Receiver<GrepItem>,
    active: Arc<Mutex<bool>>,
}

fn list(mut emit: GrepEmitter, sender: mpsc::Sender<GrepItem>, active: Arc<Mutex<bool>>) {
    for entry in WalkDir::new(::std::env::current_dir().unwrap())
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        if !*active.lock().unwrap() {
            return;
        }
        let item = GrepItem {
            path: entry.path().to_path_buf(),
            line: String::new(),
        };
        if let Err(_) = sender.send(item) {
            return;
        }
        emit.new_data_ready();
    }
    // signal processing is done
    emit.new_data_ready();
}

/// Prepare fragment for display in GUI
///
/// Limit the length to 1000 bytes. Escape for HTML. Highlight the match.
fn string_fragment<M, C, E>(matcher: &M, str: &str) -> String
where
    M: Matcher<Captures = C, Error = E>,
    C: Captures,
    E: Display,
{
    let fragment = if str.len() > 1000 {
        let mut i = 1000;
        while i > 0 && !str.is_char_boundary(i) {
            i -= 1;
        }
        &str[0..i]
    } else {
        str
    };
    let fragment = encode_minimal(fragment);
    let fragment = fragment.as_bytes();
    let mut dst = Vec::new();
    let _ = matcher.replace(&fragment, &mut dst, |m, dst| {
        dst.extend_from_slice(b"<b>");
        dst.extend_from_slice(&fragment[m]);
        dst.extend_from_slice(b"</b>");
        true
    });
    String::from_utf8_lossy(&dst).into_owned()
}

fn search_thread(
    query: &str,
    emit: &mut GrepEmitter,
    sender: mpsc::Sender<GrepItem>,
    active: Arc<Mutex<bool>>,
    receiver: spmc::Receiver<DirEntry>,
) {
    let matcher = RegexMatcher::new(query).unwrap();
    let mut searcher = Searcher::new();
    while *active.lock().unwrap() {
        match receiver.recv() {
            Ok(entry) => {
                let r = searcher.search_path(
                    &matcher,
                    entry.path(),
                    UTF8(|_, line| {
                        let item = GrepItem {
                            path: entry.path().to_path_buf(),
                            line: string_fragment(&matcher, line),
                        };
                        if let Err(e) = sender.send(item) {
                            return Err(::std::io::Error::new(
                                ::std::io::ErrorKind::ConnectionAborted,
                                e,
                            ));
                        }
                        emit.new_data_ready();
                        Ok(false)
                    }),
                );
                if let Err(e) = r {
                    if e.kind() == ::std::io::ErrorKind::ConnectionAborted {
                        // receiver was closed
                        return;
                    }
                }
            }
            Err(_) => {
                // signal that all files (in this thread) have been processed
                emit.new_data_ready();
                return;
            }
        }
    }
}

fn grep(
    query: &str,
    mut emit: GrepEmitter,
    mut item_sender: mpsc::Sender<GrepItem>,
    active: Arc<Mutex<bool>>,
) {
    if query.is_empty() {
        return list(emit, item_sender, active);
    }
    let (mut sender, receiver) = spmc::channel();
    let mut threads = Vec::new();
    for _ in 0..4 {
        let mut sender = item_sender.clone();
        let active = active.clone();
        let receiver = receiver.clone();
        let mut emit = emit.clone();
        let query = query.to_string();
        threads.push(thread::spawn(move || {
            search_thread(&query, &mut emit, sender, active, receiver);
        }));
    }
    for entry in WalkDir::new(::std::env::current_dir().unwrap())
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        if !*active.lock().unwrap() {
            return;
        }
        if let Err(_) = sender.send(entry) {
            break;
        }
    }
    drop(sender);
    for thread in threads {
        let _ = thread.join();
    }
    emit.new_data_ready();
}

impl Grepper {
    fn new(emit: GrepEmitter, query: String) -> Grepper {
        let active = Arc::new(Mutex::new(true));
        let a = active.clone();
        let (mut tx, rx) = mpsc::channel();
        thread::spawn(move || { grep(&query, emit, tx, active); });
        Grepper {
            receiver: rx,
            active: a,
        }
    }
}

impl Drop for Grepper {
    fn drop(&mut self) {
        *self.active.lock().unwrap() = false;
    }
}

struct GrepItem {
    path: PathBuf,
    line: String,
}

pub struct Grep {
    emit: GrepEmitter,
    list: GrepList,
    query: String,
    items: Vec<GrepItem>,
    new_items: Vec<GrepItem>,
    grepper: Option<Grepper>,
    last_signal: SystemTime,
}

impl GrepTrait for Grep {
    fn new(emit: GrepEmitter, list: GrepList) -> Grep {
        Grep {
            emit,
            list,
            query: String::new(),
            items: Vec::new(),
            new_items: Vec::new(),
            grepper: None,
            last_signal: UNIX_EPOCH,
        }
    }
    fn emit(&mut self) -> &mut GrepEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.items.len()
    }
    fn query(&self) -> &str {
        &self.query
    }
    fn busy(&self) -> bool {
        self.grepper.is_some()
    }
    fn set_query(&mut self, query: String) {
        self.query = query;
        self.list.begin_reset_model();
        self.items.clear();
        self.new_items.clear();
        self.last_signal = UNIX_EPOCH;
        self.list.end_reset_model();
        self.grepper = Some(Grepper::new(self.emit.clone(), self.query.clone()));
        self.emit.busy_changed();
    }
    fn name(&self, index: usize) -> String {
        self.items[index].path.display().to_string()
    }
    fn path(&self, index: usize) -> String {
        self.items[index].path.display().to_string()
    }
    fn line(&self, index: usize) -> &str {
        &self.items[index].line
    }
    fn can_fetch_more(&self) -> bool {
        self.busy() || !self.new_items.is_empty()
    }
    fn fetch_more(&mut self) {
        let mut done = false;
        if let Some(ref mut grepper) = self.grepper {
            loop {
                match grepper.receiver.try_recv() {
                    Ok(item) => self.new_items.push(item),
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        done = true;
                        break;
                    }
                }
            }
        }
        if done {
            self.grepper = None;
            self.emit.busy_changed();
        }
        // do not fetch too frequently
        if self.new_items.is_empty() ||
            self.last_signal.elapsed().unwrap() < Duration::new(0, 100_000_000)
        {
            return;
        }
        self.last_signal = SystemTime::now();
        self.list.begin_insert_rows(
            self.items.len(),
            self.items.len() + self.new_items.len() - 1,
        );
        self.items.append(&mut self.new_items);
        self.list.end_insert_rows();
    }
}
