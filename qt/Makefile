.PHONY: build_debug build_release clean

debug: qt_debug rust_debug

release: qt_release rust_release

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

qt_release:
	mkdir -p target/cpp/release
	cd target/cpp/release && qmake CONFIG+=release ../../../src/cpp/qgrep.pro && make
	cp target/cpp/release/libqgrep.a target/cpp/libqgrep.a

rust_debug:
	cargo build

rust_release:
	cargo build --release

clean_qt:
	rm -rf target/cpp

clean:
	rm -rf target
