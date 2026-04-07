# cheatscan: Playground

## main.c

Check building against both `libcheatscan.a` (static) and `libcheatscan.dylib` (dynamic).

### libcheatscan.a

```bash
$ cargo build --release
$ clang playground/main.c target/release/libcheatscan.a -Iinclude -O3 -Wl,-dead_strip -o main_static
$ ./main_static
count = 1
result[0] = 1
```

### libcheatscan.dylib

```bash
$ cargo build --release
$ clang playground/main.c -Iinclude -Ltarget/release -lcheatscan -o main_dylib
$ DYLD_LIBRARY_PATH=target/release ./main_dylib
count = 1
result[0] = 1
```

## main.js

Check building WASM.

```bash
$ cargo build --release --target wasm32-unknown-unknown
$ bun run playground/main.js
count = 1
result 0 = 1
```