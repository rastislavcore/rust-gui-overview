.PHONY: build_debug build_release clean

NAME = qt_rust

debug: qt_debug
	cargo build

release: qt_release
	cargo build --release

debug_static: qt_debug_static
	QT_STATIC=1 cargo build

release_static: qt_release_static
	QT_STATIC=1 cargo build --release

run_debug: qt_debug
	cargo run

run_release: qt_release
	cargo run --release

bindings:
	rust_qt_binding_generator bindings.json

qt_debug:
	mkdir -p target/cpp/debug
	cd target/cpp/debug && qmake CONFIG+=debug ../../../src/cpp/qgrep.pro && make
	cp target/cpp/debug/libqgrep.a target/cpp/libqgrep.a

qt_debug_static:
	mkdir -p target/cpp/debug
	cd target/cpp/debug && qmake CONFIG+=debug DEFINES+="_QT_STATIC" ../../../src/cpp/qgrep.pro && make
	cp target/cpp/debug/libqgrep.a target/cpp/libqgrep.a

qt_release:
	mkdir -p target/cpp/release
	cd target/cpp/release && qmake CONFIG+=release ../../../src/cpp/qgrep.pro && make
	cp target/cpp/release/libqgrep.a target/cpp/libqgrep.a

qt_release_static:
	mkdir -p target/cpp/release
	cd target/cpp/release && qmake CONFIG+=release DEFINES+="_QT_STATIC" ../../../src/cpp/qgrep.pro && make
	cp target/cpp/release/libqgrep.a target/cpp/libqgrep.a

clean_qt:
	rm -rf target/cpp

clean:
	rm -rf target

rpath_debug:
	install_name_tool -add_rpath `qmake -query QT_INSTALL_PREFIX`/lib target/debug/$(NAME)
	install_name_tool -add_rpath `qmake -query QT_INSTALL_PREFIX`/plugins/platforms target/debug/$(NAME)

rpath_release:
	install_name_tool -add_rpath `qmake -query QT_INSTALL_PREFIX`/lib target/release/$(NAME)
	install_name_tool -add_rpath `qmake -query QT_INSTALL_PREFIX`/plugins/platforms target/release/$(NAME)
