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

```sh
cargo build --release
```

[`did_resolve`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_resolve
[`VadeUniversalResolver `]: https://git.slock.it/equs/interop/vade/vade-universal-resolver
[`VadePlugin`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html