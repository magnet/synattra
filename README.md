# Synattra
[![Build Status](https://travis-ci.org/magnet/synattra.svg?branch=master)](https://travis-ci.org/magnet/synattra)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/magnet/synattra)
[![Cargo](https://img.shields.io/crates/v/synattra.svg)](
https://crates.io/crates/synattra)
[![Documentation](https://docs.rs/synattra/badge.svg)](
https://docs.rs/synattra)
[![Rust 1.31+](https://img.shields.io/badge/rust-1.31+-lightgray.svg)](
https://www.rust-lang.org)

## A Syn Attribute Parser Toolkit

Synattra extends Syn and provides structures to easily parse custom attributes.

Notably, Synattra provides a `KVOption<K, V>` type that allows parsing attributes in the form `key = value` where `key` can be any token or custom keyword (including Rust keywords!) and value any type that can be parsed from a `TokenStream`. Synattra was extracted from the [Metered project](https://github.com/magnet/metered-rs) which needed expressing type paths as option values (e.g `path::to::GenericType<u32>`), which was not supported by existing attribute parsing systems (neither Syn's own `Meta` parsing facility or alternate crates such as [prom-attire-rs](https://github.com/Nemo157/prom-attire-rs/)).

Synattra also supports single or multiple values, that can take the shape of `Foo` or `[Foo, Bar]`.

Finally Synattra provides some extra types, such as `InvokePath` which represents any invocation handle, macro or not (e.g `foo` or `println!`).

By reusing Syn's design, Synattra parsers are very robust and when they compile, they usually work :-).


## Changelog

* 0.3.0:
  * Updated dependencies to use `syn` 2.0 and `auto_enums` 0.8
* 0.2.0:
  * Updated dependencies to use `syn`, `proc-macro2` and `quote` 1.0


## Required Rust version

Synattra runs on `Rust` stable.


### Design

Synattra is using Syn's design for attribute parsing. You can see it in use in the [Metered project](https://github.com/magnet/metered-rs).


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.