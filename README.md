# Rust meson-next crate

A build dependency crate for running [Meson](https://mesonbuild.com/index.html) to build a native library.

This crate is a simple wrapper that invokes the system's meson binary.

Ensure you have both `meson` and `ninja` installed. Refer to [Meson's manual](https://mesonbuild.com/SimpleStart.html) for specific install instructions for your OS.

This crate is a fork of meson 1.0 by dovee, which is abandoned.

## Build Example

```text
.
├── build.rs
├── Cargo.toml
├── clib
│   ├── meson.build
│   ├── squid.h
│   └── squid.c
└── src
    └── lib.rs
```

build.rs:

```rust
extern crate meson_next as meson;
use std::env;
use std::path::PathBuf;

fn main() {
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    build_path.join("build");
    let build_path = build_path.to_str().unwrap();

    let mut options = HashMap::new();
    options.insert("key", "value");

    let config = meson::Config::new().options(options);

    println!("cargo:rustc-link-lib=squid");
    println!("cargo:rustc-link-search=native={}", build_path);
    meson::build("clib", build_path, config);
}
```

Cargo.toml:

```toml
# ...

[build-dependencies]
meson-next = "1"
```

meson.build:

```text
project('squid', 'c')
shared_library('squid', 'squid.c')
```
