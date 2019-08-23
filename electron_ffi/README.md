# Electron + FFI

Simple Electron/Rust/FFI application

Does not works on NodeJS 12 / Electron > 4.x.x because of node ffi (https://github.com/node-ffi/node-ffi/issues/552).

```bash
# to build ffi for the
export npm_config_target=4.2.9
export npm_config_arch=x64
export npm_config_target_arch=x64
export npm_config_disturl=https://electronjs.org/headers
export npm_config_runtime=electron
export npm_config_build_from_source=true
$ make rust
$ make install
$ npm start
```

See [lib/index.html](lib/index.html) for details about how to call Rust through ffi.
