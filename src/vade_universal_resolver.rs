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

extern crate vade;

use async_trait::async_trait;
use std::{env};
use vade::{AsyncResult, VadePlugin, VadePluginResultValue};
use serde::{Deserialize, Serialize};

const DID_PREFIX: &str = "did:";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidResolverResult {
    pub did_resolution_metadata: serde_json::Value,
    pub did_document_metadata: serde_json::Value,
    pub did_document: serde_json::Value,
}

pub struct ResolverConfig {
    pub resolver_url: String,
}

/// Resolver for DIDs on the Trust&Trace substrate chain
pub struct VadeUniversalResolver {
    config: ResolverConfig,
}

impl VadeUniversalResolver {
    /// Creates new instance of `VadeEvanSubstrate`.
    pub fn new() -> VadeUniversalResolver {
        // Setting default value for resolver url as universal resolver
        let mut url = "https://dev.uniresolver.io/1.0/identifiers/".to_string();
        if !env::var("RESOLVER_URL").is_err() && env::var("RESOLVER_URL").is_ok() {
            // If environment variable is found and it contains some value, it will replace default value
            url = env::var("RESOLVER_URL").ok().unwrap();
        }
        let config = ResolverConfig { resolver_url: url };
        match env_logger::try_init() {
            Ok(_) | Err(_) => (),
        };
        VadeUniversalResolver { config }
    }
}

#[async_trait]
impl VadePlugin for VadeUniversalResolver {
    /// Fetch data about a DID, which returns this DID's DID document.
    ///
    /// # Arguments
    ///
    /// * `did` - did to fetch data for
    async fn did_resolve(
        &mut self,
        did_id: &str,
    ) -> AsyncResult<VadePluginResultValue<Option<String>>> {
        if !did_id.starts_with(DID_PREFIX) {
            return Ok(VadePluginResultValue::Ignored);
        }
        let mut resolver_url = self.config.resolver_url.clone();
        resolver_url.push_str(did_id);

        let did_result = reqwest::blocking::get(&resolver_url)?.text()?;

        let  resolver_result: DidResolverResult = serde_json::from_str(&did_result)?;
        let did_document = resolver_result.did_document.to_string();
        Ok(VadePluginResultValue::Success(Some(did_document)))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Once};

    static INIT: Once = Once::new();

    fn enable_logging() {
        INIT.call_once(|| {
            env_logger::try_init().ok();
        });
    }

    #[tokio::test]
    async fn can_resolve_did() -> AsyncResult<()> {
        enable_logging();
        let mut resolver = VadeUniversalResolver::new();
        let result = resolver.did_resolve( "did:ethr:mainnet:0x3b0BC51Ab9De1e5B7B6E34E5b960285805C41736").await;

        if let VadePluginResultValue::Success(Some(value)) = result.as_ref().unwrap() {
            println!("did resolve result: {}", &value);
        }
        assert_eq!(result.is_ok(), true);

        Ok(())
    }
}