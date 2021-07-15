/*
  Copyright (c) 2018-present evan GmbH.

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

//! This crate allows you to use to resolve DIDs.
//! For this purpose a [`VadePlugin`] implementations is exported: [`VadeUniversalResolver`].
//!
//! ## VadeUniversalResolver
//!
//! Supports creating, updating and getting DIDs and DID documents on substrate, therefore supports:
//!
//
//! - [`did_resolve`]
//!
//!
//!
//! ## Compiling vade_universal_resolver
//!
//! ### "Regular" build
//!
//! No surprise here:
//!
//! ```sh
//! cargo build --release
//! ```
//!
//! ### WASM
//!
//! To compile `vade_universal_resolver` for wasm, use wasm pack.
//!
//! Also you have to specify whether to build a browser or a nodejs environment.
//!
//! nodejs:
//!
//! ```sh
//! wasm-pack build --release --target nodejs
//! ```
//!
//! browser:
//!
//! ```sh
//! wasm-pack build --release --target web
//! ```

//! [`did_create`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_create
//! [`did_resolve`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_resolve
//! [`did_update`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_update
//! [`VadeEvanSubstrate`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html
//! [`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html
//! [`VadePlugin`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html

#[macro_use]

// did
mod vade_universal_resolver;
pub use self::vade_universal_resolver::*;
