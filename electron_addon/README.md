# Electron + NodeJS Cpp Addon

Simple Electron/Rust/NodeJS Cpp Addon

```bash
# see https://electronjs.org/docs/tutorial/using-native-node-modules
export npm_config_target=6.0.3
export npm_config_arch=x64
export npm_config_target_arch=x64
export npm_config_disturl=https://electronjs.org/headers
export npm_config_runtime=electron
export npm_config_build_from_source=true
$ make addon
$ make install
$ npm start
```
