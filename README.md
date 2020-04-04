# camarim

Easy setup logging on FFI libs for mobile devices

> camarim: (pt-br) Backstage room on the theater where actors get ready to show up on stage

---

This crate let you easilly get logging available for FFI libraries, specially when targeting Android and iOS devices.

## Installation

### Add the library as an optional dependency

You can use the library as an optional dependency, to use only during debug.
On `Cargo.toml` add:

```toml
[dependencies.camarim]
version = '0.1.0'
optional = true

[features]
logs = ['camarim']
```

### Add the library on the root of the FFI crate

```rust
#[cfg(feature = "logs")]
camarim::install!();
```

### Build your library with the feature enabled

```sh
cross build --features logs --target x86_64-linux-android
```

### Call the setup to intialize the logger

On the FFI side, use the `camarim_setup_logger` function to setup the logging.

For example, in `Dart`:

```dart
import 'dart:ffi' as ffi;

final ffi.DynamicLibrary _myLib = Platform.isAndroid
    ? ffi.DynamicLibrary.open("libmylib.so")
    : ffi.DynamicLibrary.process();


/// Find `camarim_setup_logger` to call it during your app startup
final startLogger =
    _sledNative.lookupFunction<ffi.Void Function(), void Function()>(
        "camarim_setup_logger");

/// ... declare the your crate exposed functions
```

## TODO

- [ ] WASM and Browser logging?
- [ ] Provide ways to customize log level?

## Relevant issues

- [Re-export C symbols](https://github.com/rust-lang/rfcs/issues/2771) - to remove macro needs
