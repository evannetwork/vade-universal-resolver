# Vade Universal Resolver Plugin

[![crates.io](https://img.shields.io/crates/v/vade-universal-resolver.svg)](https://crates.io/crates/vade-universal-resolver)
[![Documentation](https://docs.rs/vade-universal-resolvere/badge.svg)](https://docs.rs/vade-universal-resolver:q)
[![Apache-2 licensed](https://img.shields.io/crates/l/vade-universal-resolver.svg)](./LICENSE.txt)

## About
This crate allows you to resolve DIDs.
For this purpose a [`VadePlugin`] implementation is exported: [`VadeUniversalResolver`].

## VadeUniversalResolver

Supports resolving all did methods provided by the universal resolver api:

- [`did_resolve`]

## Compiling vade_universal_resolver

### "Regular" build

No surprise here:

```sh
cargo build --release
```

### WASM

To compile `vade_universal_resolver` for wasm, use wasm pack.

Also you have to specify whether to build a browser or a nodejs environment.

nodejs:

```sh
wasm-pack build --release --target nodejs
```

browser:

```sh
wasm-pack build --release --target web
```

[`did_create`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_create
[`did_resolve`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_resolve
[`did_update`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_update
[`VadeEvanSubstrate`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html
[`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html
[`VadePlugin`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html