# Rust GUI ecosystem overview

author: [Sylvain Kerkour (z0mbie42)](https://www.kerkour.fr)

----------

This research document compares the available, production ready GUI options for Rust.

The rating ranges from <span style="color:green">++</span> for very good, to <span style="color:red">oo</span> for very bad, with <span style="color:orange">-</span> for mean.

|  | [Electron + Neon](#electron-neon) | [Electron + FFI](#gelectron-ffi) | [Electron + NodeJS Cpp Addon](#electron-nodejs-cpp-addon) | [Rust Program + Qt static](#rust-program-qt-static) | [Rust program + Qt dynamic](#rust-program-qt-dynamic) | [Cpp program + Rust lib static + Qt static](#cpp-program-rust-lib-static-qt-static) | [Cpp program + Rust lib static + Qt dynamic](#cpp-program-rust-lib-static-qt-dynamic) | [Gtk](#gtk) |
|-|-|-|-|-|-|-|-|-|
| Ease of build | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:red">oo</span> | <span style="color:green">+</span> | <span style="color:orange">-</span> | <span style="color:green">+</span> | <span style="color:green">+</span> |
| Build time | <span style="color:green">++</span> | <span style="color:green">++</span>  |<span style="color:green">++</span>  |<span style="color:red">oo</span> | <span style="color:green">++</span> | <span style="color:red">oo</span> | <span style="color:green">++</span> | <span style="color:orange">-</span> |
| Bundle size | <span style="color:red">o</span> | <span style="color:red">o</span> | <span style="color:red">o</span> |<span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:orange">-</span> |
| Ease of deployment | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:orange">-</span> | <span style="color:green">+</span> | <span style="color:orange">-</span> | <span style="color:red">o</span> |
| Rust interoperability | <span style="color:green">+</span> | <span style="color:orange">-</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span>| <span style="color:green">++</span> |
| Speed of devlopment | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:orange">-</span> | <span style="color:red">o</span> | <span style="color:red">o</span> | <span style="color:red">o</span> | <span style="color:red">o</span> | <span style="color:red">oo</span> |
| RAM usage | <span style="color:orange">-</span> | <span style="color:orange">-</span>  | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span>  | <span style="color:orange">-</span>  | <span style="color:green">+</span> |
| CPU usage | <span style="color:red">o</span> | <span style="color:red">o</span> | <span style="color:red">o</span> | <span style="color:green">++</span> |  <span style="color:green">++</span> |  <span style="color:green">++</span> |  <span style="color:green">++</span> | <span style="color:green">++</span> |
| Security | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> |<span style="color:green">+</span> | <span style="color:green">+</span> |
| Look | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:red">o</span> |
| Responsive UI | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:red">oo</span> |
| Framework fixability | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:red">oo</span> | <span style="color:red">oo</span> | <span style="color:red">oo</span> | <span style="color:red">oo</span> | <span style="color:red">o</span> |
| Platforms support | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">-</span> |
| Built-in features | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:red">oo</span> |
| Components library | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:red">oo</span> |
| Debugging | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> |
| Testing | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> |
| Community | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:red">oo</span> |
| License | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:orange">-</span> | <span style="color:green">++</span> |

### Not tested

The following options exists but were not included because they are not mature enough.

* [Flutter desktop](https://github.com/google/flutter-desktop-embedding): not enought mature
* [Godot](https://godotengine.org/): too much hacky
* [Azul](https://github.com/maps4print/azul): not enought mature

The most promising seems to be Flutter.

### Ranking

This research results in the following ranking, with the principal criterion being `get the shit done`:

1. [Electron + Neon](#electron-neon)
2. [Rust program + Qt dynamic](#rust-program-qt-dynamic) for devlopment and [Rust Program + Qt static](#rust-program-qt-static) for deployment,
3. [Cpp program + Rust lib static + Qt static](#cpp-program-rust-lib-static-qt-static), [Cpp program + Rust lib static + Qt dynamic](#cpp-program-rust-lib-static-qt-dynamic), [Electron + NodeJS Cpp Addon](#electron-nodejs-cpp-addon)
4. [Electron + FFI](#gelectron-ffi)
5. [Gtk](#gtk)



## Electron

* https://github.com/martpie/museeks

### Electron + neon

Using Electron as GUI framework, compiling Rust to a native Node module through Neon bindings.

See [here for code](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/electron_neon).

* https://keminglabs.com/blog/building-a-fast-electron-app-with-rust/
* https://neon-bindings.com/docs/electron-apps

#### Examples

* https://github.com/Abdillah/rustodoro
* https://github.com/SubstratumNetwork/SubstratumNode
* https://github.com/genet-app/genet
* https://github.com/ubnt-intrepid/neon-electron
* https://github.com/connorwalsh/granola
* https://github.com/Gricha/electron-rust-example
* https://github.com/shade/rustoku
* https://github.com/mullvad/mullvadvpn-app
* https://github.com/FIL1994/electron-and-rust


### Electron + FFI

Using Electron as GUI framework, compiling Rust as a static c library and calling it through node `ffi`.

* https://github.com/wtfil/rust-in-node#direct-ffi-call


### Electron + NodeJS Cpp Addon

Using Electron as GUI framework, compiling Rust as a static c library and calling it through a NodeJS C++ Addon.

See [here for code](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/electron_addon).


* https://github.com/wtfil/rust-in-node#call-dynamic-library-via-c-addon
* https://nodejs.org/api/addons.html



## Qt

Here we test only QML as it's currently the easiest way to build a good looking Qt application. A few ibservations:

* Compile time are reallyyyyyyyy big when linking Qt statically.
* As QML has it's own JS engine it adds some overhead ([will be improved for Qt6](https://blog.qt.io/blog/2019/08/07/technical-vision-qt-6/)).

<br/>

* https://blog.qt.io/blog/2018/11/15/python-qt-3000-hours-developer-insight/
* https://github.com/woboq/qmetaobject-rs
* https://github.com/KDE/rust-qt-binding-generator
* https://github.com/shashwatdixit124/IPConnect
* `macdeployqt`
* https://doc.qt.io/qt-5/qtquick-bestpractices.html#performance
* https://woboq.com/blog/qml-vs-cpp-for-application-startup-time.html
https://retifrav.github.io/blog/2018/02/17/build-qt-statically/


### Rust Program + Qt static

Compiling a cpp static library from QT, then linking to a Rust program and also statically linking Qt.

See [here for code](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/qt_rust#linking-qt-statically).

* https://doc.qt.io/qt-5/qpa.html#qpa-plugins
* https://jonnyzzz.com/blog/2018/06/13/link-error-3/


Really hard to have it compile... need to link statically all libs and plugins.

#### Troubleshoting

##### Cocoa plugin not found
```
qt.qpa.plugin: Could not find the Qt platform plugin "cocoa" in ""
This application failed to start because no Qt platform plugin could be initialized. Reinstalling the application may fix this problem.
```

Qt plugins have not been imported for static linking in Cpp. See https://doc.qt.io/QtForDeviceCreation/qtee-static-linking.html#adding-qml-imports



### Rust program + Qt dynamic

Compiling a cpp static library from QT, then linking to a Rust program and dynamically linking Qt.

See [here for code](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/qt_rust#linking-qt-dynamically).


* *https://github.com/rust-lang/cargo/issues/5077
* https://github.com/rust-lang/rust/issues/17219
* https://wincent.com/wiki/@executable_path,_@load_path_and_@rpath

<br/>

* https://wiki.qt.io/How_to_create_a_library_with_Qt_and_use_it_in_an_application
* https://stackoverflow.com/questions/10034825/how-do-i-build-a-static-library-and-executable-with-qt
* https://news.ycombinator.com/item?id=19296386
* https://doc.qt.io/qt-5.9/osx-deployment.html
* https://gitlab.com/rhn/quemail/tree/master


### Cpp program + Rust lib static + Qt static

Compiling Rust as a static c library, linking it statically to a Qt program, and also linking Qt statically.

* https://github.com/spieglt/cloaker

It works moderately, for example if we want the program to be both a CLI and a GUI.


### Cpp program + Rust lib static + Qt dynamic

Compiling Rust as a static c library, linking it statically to a Qt program, and linking Qt dynamically.

* https://github.com/rust-qt/ritual



## Gtk

using the Rust Gtk bindings directly from Rust.

ugly ...

* https://github.com/gtk-rs/gtk
* https://gtk-rs.org/
