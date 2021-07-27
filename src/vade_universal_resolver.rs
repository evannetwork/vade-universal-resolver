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

use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use vade::{VadePlugin, VadePluginResultValue};

const DID_PREFIX: &str = "did:";
const DEFAULT_URL: &str = "https://dev.uniresolver.io/1.0/identifiers/";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidResolverResult {
    pub did_resolution_metadata: serde_json::Value,
    pub did_document_metadata: serde_json::Value,
    pub did_document: serde_json::Value,
}

pub struct UniversalResolverConfig {
    pub resolver_url: String,
}

/// Resolver for DIDs via Universal Resolver
pub struct VadeUniversalResolver {
    config: UniversalResolverConfig,
}

impl VadeUniversalResolver {
    /// Creates new instance of `VadeUniversalResolver`.
    pub fn new(resolver_url: Option<String>) -> VadeUniversalResolver {
        // Setting default value for resolver url as universal resolver
        // If environment variable is found and it contains some value, it will replace default value
        let url = resolver_url.unwrap_or_else(|| DEFAULT_URL.to_string());

        let config = UniversalResolverConfig { resolver_url: url };
        match env_logger::try_init() {
            Ok(_) | Err(_) => (),
        };
        VadeUniversalResolver { config }
    }
}

#[async_trait(?Send)]
impl VadePlugin for VadeUniversalResolver {
    /// Fetch data about a DID, which returns this DID's DID document.
    ///
    /// # Arguments
    ///
    /// * `did` - DID to fetch data for
    async fn did_resolve(
        &mut self,
        did_id: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        if !did_id.starts_with(DID_PREFIX) {
            return Ok(VadePluginResultValue::Ignored);
        }
        let mut resolver_url = self.config.resolver_url.clone();
        resolver_url.push_str(did_id);

        let did_result = Client::builder()
            .timeout(Duration::from_secs(2))
            .build()?
            .get(&resolver_url)
            .send()
            .await;

        match did_result {
            Ok(result) => {
                let request_result = match result.text().await {
                    Ok(text) => text,
                    Err(e) => return Ok(VadePluginResultValue::Ignored),
                };
                let resolver_result: DidResolverResult = match serde_json::from_str(&request_result)
                {
                    Ok(text) => text,
                    Err(e) => return Ok(VadePluginResultValue::Ignored),
                };
                let did_document = resolver_result.did_document.to_string();
                Ok(VadePluginResultValue::Success(Some(did_document)))
            }
            Err(e) => Ok(VadePluginResultValue::Ignored),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn enable_logging() {
        INIT.call_once(|| {
            env_logger::try_init().ok();
        });
    }

    #[tokio::test]
    async fn can_resolve_did() -> Result<(), Box<dyn std::error::Error>> {
        enable_logging();
        let mut resolver = VadeUniversalResolver::new(std::env::var("RESOLVER_URL").ok());
        let result = resolver
            .did_resolve("did:ethr:mainnet:0x3b0BC51Ab9De1e5B7B6E34E5b960285805C41736")
            .await;

        let respone = match result.as_ref() {
            Ok(VadePluginResultValue::Success(Some(value))) => value.to_string(),
            Ok(_) => "Unknown Result".to_string(),
            Err(e) => e.to_string(),
        };
        println!("did resolve result: {}", &respone);

        assert_eq!(result.is_ok(), true);
        Ok(())
    }
}
