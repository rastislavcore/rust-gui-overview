# Rust GUI ecosystem overview

author: [Sylvain Kerkour (z0mbie42)](https://www.kerkour.fr)

Prepared for: [Bloom](https://gitlab.com/bloom42)

----------

This research document compares the available, production ready GUI options for Rust.

The rating ranges from <span style="color:green">++</span> for very good, to <span style="color:red">--</span> for very bad, with <span style="color:orange">o</span> for mean.

|  | [Electron + Neon](#electron-neon) | [Electron + FFI](#electron-ffi) | [Electron + NodeJS Cpp Addon](#electron-nodejs-cpp-addon) | [Rust Program + Qt static](#rust-program-qt-static) | [Rust program + Qt dynamic](#rust-program-qt-dynamic) | [Cpp program + Rust lib static + Qt static](#cpp-program-rust-lib-static-qt-static) | [Cpp program + Rust lib static + Qt dynamic](#cpp-program-rust-lib-static-qt-dynamic) | [Gtk](#gtk) |
|-|-|-|-|-|-|-|-|-|
| Ease of build | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:red">--</span> | <span style="color:green">+</span> | <span style="color:orange">o</span> | <span style="color:green">+</span> | <span style="color:green">+</span> |
| Build time | <span style="color:green">++</span> | <span style="color:green">++</span>  |<span style="color:green">++</span>  |<span style="color:red">--</span> | <span style="color:green">++</span> | <span style="color:red">--</span> | <span style="color:green">++</span> | <span style="color:orange">o</span> |
| Bundle size | <span style="color:red">-</span> | <span style="color:red">-</span> | <span style="color:red">-</span> |<span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:orange">o</span> |
| Ease of deployment | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:orange">o</span> | <span style="color:green">+</span> | <span style="color:orange">o</span> | <span style="color:red">-</span> |
| Rust interoperability | <span style="color:green">+</span> | <span style="color:orange">o</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span>| <span style="color:green">++</span> |
| Speed of development | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:orange">o</span> | <span style="color:green">+</span> | <span style="color:red">--</span> | <span style="color:green">+</span> | <span style="color:red">--</span> | <span style="color:red">-</span> |
| RAM usage | <span style="color:orange">o</span> | <span style="color:orange">o</span>  | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span>  | <span style="color:orange">o</span>  | <span style="color:green">+</span> |
| CPU usage | <span style="color:red">-</span> | <span style="color:red">-</span> | <span style="color:red">-</span> | <span style="color:green">++</span> |  <span style="color:green">++</span> |  <span style="color:green">++</span> |  <span style="color:green">++</span> | <span style="color:green">++</span> |
| Security | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> |<span style="color:green">+</span> | <span style="color:green">+</span> |
| Look | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:orange">o</span> |
| Responsive UI | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:red">-</span> |
| Framework fixability | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:red">--</span> | <span style="color:red">--</span> | <span style="color:red">--</span> | <span style="color:red">--</span> | <span style="color:red">-</span> |
| Platforms support | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> |
| Built-in features | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">o</span> |
| 3rd party components | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:red">--</span> |
| Debugging | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> |
| Testing | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> |
| Community | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:green">+</span> | <span style="color:red">-</span> |
| License | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:green">++</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:orange">o</span> | <span style="color:green">++</span> |

### Not tested

The following options exists but were not included because they are not mature enough.

* [Flutter desktop](https://github.com/google/flutter-desktop-embedding): not enought mature
* [Relm](https://github.com/antoyo/relm): Gtk overlay, looks good
* [Godot](https://godotengine.org/): too much hacky
* [Unity](https://www.google.com/search?q=unity+rust): too much hacky
* [Azul](https://github.com/maps4print/azul): not enought mature
* [Conrod](https://github.com/PistonDevelopers/conrod): not enought mature
* [Druid](https://github.com/xi-editor/druid): not enought mature
* https://areweguiyet.com
* [Platform specific libraries](https://areweguiyet.com/newsfeed/2019-01-13_rust2019.html): because we need cross-platform app.
* [Sciter](https://crates.io/crates/sciter-rs)
* Web broswer communicating with a Rust local server: too much hacky, insecure? (DNS rebinding attacks) and does not support native features like tray icons.

The most promising seems to be Flutter.

### Ranking

This research result in the following ranking, with the principal criterion being `get the shit done`:

1. [Electron + Neon](#electron-neon)
2. [Rust program + Qt dynamic](#rust-program-qt-dynamic) for devlopment and [Rust Program + Qt static](#rust-program-qt-static) for deployment
3. [Gtk](#gtk), [Cpp program + Rust lib static + Qt static](#cpp-program-rust-lib-static-qt-static), [Cpp program + Rust lib static + Qt dynamic](#cpp-program-rust-lib-static-qt-dynamic), [Electron + NodeJS Cpp Addon](#electron-nodejs-cpp-addon)
4. [Electron + FFI](#electron-ffi)



## Electron

#### Resources

* https://github.com/martpie/museeks

### Electron + neon

Using Electron as GUI framework, compiling Rust to a native Node module through Neon bindings.

See [here for code example](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/electron_neon).

#### Resources

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

See [here for code example](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/electron_ffi).

#### Resources

* https://github.com/wtfil/rust-in-node#direct-ffi-call


### Electron + NodeJS Cpp Addon

Using Electron as GUI framework, compiling Rust as a static c library and calling it through a NodeJS C++ Addon.

See [here for code example](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/electron_addon).

#### Resources

* https://github.com/wtfil/rust-in-node#call-dynamic-library-via-c-addon
* https://nodejs.org/api/addons.html



## Qt

Here we test only QML as it's currently the easiest way to build a good looking Qt application. A few ibservations:

* Compile time are reallyyyyyyyy big when linking Qt statically.
* As QML has it's own JS engine it adds some overhead ([will be improved for Qt6](https://blog.qt.io/blog/2019/08/07/technical-vision-qt-6/)).

3 crates:

* https://github.com/woboq/qmetaobject-rs
* https://github.com/KDE/rust-qt-binding-generator
* https://github.com/rust-qt/ritual

here we will use only `rust-qt-binding-generator`

#### Resources

* https://blog.qt.io/blog/2018/11/15/python-qt-3000-hours-developer-insight/
* https://github.com/shashwatdixit124/IPConnect
* `macdeployqt`
* https://doc.qt.io/qt-5/qtquick-bestpractices.html#performance
* https://woboq.com/blog/qml-vs-cpp-for-application-startup-time.html
* https://retifrav.github.io/blog/2018/02/17/build-qt-statically/


### Rust Program + Qt static

Compiling a cpp static library from QT, then linking to a Rust program and also statically linking Qt.

Really hard to have it compile... need to link statically all libs and plugins.

See [here for code example](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/qt_rust#linking-qt-statically).

#### Resources

* https://doc.qt.io/qt-5/qpa.html#qpa-plugins
* https://jonnyzzz.com/blog/2018/06/13/link-error-3/


### Rust program + Qt dynamic

Compiling a cpp static library from QT, then linking to a Rust program and dynamically linking Qt.

See [here for code example](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/qt_rust#linking-qt-dynamically).

#### Resources

* https://wiki.qt.io/How_to_create_a_library_with_Qt_and_use_it_in_an_application
* https://stackoverflow.com/questions/10034825/how-do-i-build-a-static-library-and-executable-with-qt
* https://news.ycombinator.com/item?id=19296386
* https://doc.qt.io/qt-5.9/osx-deployment.html
* https://gitlab.com/rhn/quemail/tree/master
* https://doc.rust-lang.org/cargo/reference/build-scripts.html


### Cpp program + Rust lib static + Qt static

Compiling Rust as a static c library, linking it statically to a Qt program, and also linking Qt statically.

It works moderately, for example if we want the program to be both a CLI and a GUI.

See [here for code example](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/qt_cpp).

#### Resources

* https://github.com/spieglt/cloaker


### Cpp program + Rust lib static + Qt dynamic

Compiling Rust as a static c library, linking it statically to a Qt program, and linking Qt dynamically.

See [here for code example](https://gitlab.com/z0mbie42/rust_gui_ecosystem_overview/tree/master/qt_cpp).



## Gtk

using the Rust Gtk bindings directly from Rust.

It's the more integrated solution, but it's really, **REALLY** ugly and hard to customize.

### Resources

* https://github.com/gtk-rs/gtk
* https://gtk-rs.org/
* https://gtk-rs.org/docs/gtk/struct.CssProvider.html
* https://medium.com/@alex285/quickly-set-your-gtk-rust-environment-start-coding-9cdfb18b7729
* https://blog.microjoe.org/2018/application-gtk-rust-glade.html
* https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template

### Examples

* https://gtk-rs.org/#projects-using-gtk-rs
* https://github.com/waf/strack
