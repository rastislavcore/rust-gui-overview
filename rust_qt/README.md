# Rust program + Qt

To run these examples you need to have Qt (5) installed.


## Linking Qt Dynamically

```bash
$ make run_debug
```

## Linking Qt Statically

You first need to build Qt statically: https://retifrav.github.io/blog/2018/02/17/build-qt-statically/

```bash
$ QMAKE=[PATH TO STATIC QMAKE] QT_STATIC=1 make run_debug
# e.g. QMAKE=~/Qt/static/5/bin/qmake QT_STATIC=1 make run_debug
```


## License

From an original work by `Jos van den Oever <jos@vandenoever.info>`, AGPL-3.0-or-later, https://anongit.kde.org/scratch/vandenoever/qrep.
