# Rust program + Qt

To run these examples you need to have Qt (5) installed.


## Linking Qt Dynamically

```bash
# on MacOS
$ make debug
$ make rpath_debug # Fixes LC_RPATH to include Qt
$ make run_debug

# on other platforms
$ make run_debug
```

## Linking Qt Statically

You first need to build Qt statically: https://retifrav.github.io/blog/2018/02/17/build-qt-statically/.

Then uncomment the 1 line following `// static` in [src/main.rs](src/mains.rs).

```bash
$ make clean_qt
$ QMAKE=[PATH TO STATIC QMAKE] QT_STATIC=1 make run_debug
# e.g. QMAKE=~/Qt/static/5/bin/qmake QT_STATIC=1 make run_debug
```

## Screenshots

<img src="qgrep.png" width="180" />


## License

From an original work by `Jos van den Oever <jos@vandenoever.info>`, AGPL-3.0-or-later, https://anongit.kde.org/scratch/vandenoever/qrep.
