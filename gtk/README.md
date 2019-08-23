# Gtk

You need to have `libgtk+3` installed.


```bash
$ cargo run
```

## Troubleshooting

### Error building ffi

```bash
$ export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig" cargo run
```
