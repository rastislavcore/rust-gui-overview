.PHONY: rust rust_release qt_debug qt_release release debug

debug: rust_debug qt_release

release: rust_release qt_release

rust_debug:
	cargo build

rust_release:
	cargo build --release

qt_debug:
	mkdir -p target/cpp/debug
	cd target/cpp/debug && qmake CONFIG+=debug ../../../src/cpp/todos.pro && make

qt_release:
	mkdir -p target/cpp/release
	cd target/cpp/release && qmake CONFIG+=release ../../../src/cpp/todos.pro && make

bindings:
	rust_qt_binding_generator bindings.json

clean:
	rm -rf target
