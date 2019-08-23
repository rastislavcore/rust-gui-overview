.PHONY: clean addon install

clean:
	rm -rf node_modules addon-rs/target addon-rs/node/build

addon:
	cd addon-rs && cargo build --release && npm run build

install:
	npm install
