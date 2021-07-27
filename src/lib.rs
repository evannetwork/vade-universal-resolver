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

//! This crate allows you to resolve DIDs.
//! For this purpose a [`VadePlugin`] implementation is exported: [`VadeUniversalResolver`].
//!
//! ## VadeUniversalResolver
//!
//! Supports resolving all did methods provided by the universal resolver api:
//!
//! - [`did_resolve`]
//!
//! ## Compiling vade_universal_resolver
//!
//! ```sh
//! cargo build --release
//! ```

//! [`did_resolve`]: https://docs.rs/vade_evan_substrate/*/vade_evan_substrate/vade_evan_substrate/struct.VadeEvanSubstrate.html#method.did_resolve
//! [`VadeUniversalResolver `]: https://git.slock.it/equs/interop/vade/vade-universal-resolver
//! [`VadePlugin`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html

#[macro_use]

// did
mod vade_universal_resolver;
pub use self::vade_universal_resolver::*;
