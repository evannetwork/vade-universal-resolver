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

#[cfg(feature = "sdk")]
use crate::in3_request_list::ResolveHttpRequest;

#[cfg(not(feature = "sdk"))]
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[cfg(feature = "sdk")]
use std::ffi::{CStr, CString};
#[cfg(feature = "sdk")]
use std::os::raw::c_void;
#[cfg(feature = "sdk")]
use std::os::raw::c_char;
#[cfg(not(feature = "sdk"))]
use std::time::Duration;
use async_trait::async_trait;
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
    #[cfg(feature = "sdk")]
    pub request_id: *const c_void,
    #[cfg(feature = "sdk")]
    pub resolve_http_request: ResolveHttpRequest,
}

/// Resolver for DIDs via Universal Resolver
pub struct VadeUniversalResolver {
    config: UniversalResolverConfig,
}

impl VadeUniversalResolver {
    /// Creates new instance of `VadeUniversalResolver`.
    pub fn new(
        resolver_url: Option<String>,
        #[cfg(feature = "sdk")] request_id: *const c_void,
        #[cfg(feature = "sdk")] resolve_http_request: ResolveHttpRequest,
    ) -> VadeUniversalResolver {
        // Setting default value for resolver url as universal resolver
        // If environment variable is found and it contains some value, it will replace default value
        let url = resolver_url.unwrap_or_else(|| DEFAULT_URL.to_string());

        let config = UniversalResolverConfig {
            resolver_url: url,
            #[cfg(feature = "sdk")]
            request_id,
            #[cfg(feature = "sdk")]
            resolve_http_request,
        };

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

        #[cfg(feature = "sdk")]
        let request_pointer = self.config.request_id.clone();

        #[cfg(feature = "sdk")]
        let resolve_http_request = self.config.resolve_http_request;

        cfg_if::cfg_if! {
              if #[cfg(feature = "sdk")]{
                // if compiled for sdk integration, get_http_response function will be called
                // TODO: once c library function is received from SDK team add logic here to call the function,
                // TODO: as of now the function call is assumed to be returning request pending, need more information regarding function parameters
                // structures
                let url = CString::new(resolver_url.to_string()).expect("CString::new failed for resolver_url");
                let url = url.as_ptr();

                let method = CString::new("GET").expect("CString::new failed for method");
                let method = method.as_ptr();

                let path = CString::new("").expect("CString::new failed for path");
                let path = path.as_ptr();

                let payload = CString::new("").expect("CString::new failed for payload");
                let payload = payload.as_ptr();

                let mut res: *mut c_char = std::ptr::null_mut();
                
                let error_code = (resolve_http_request)(
                    request_pointer,
                    url,
                    method,
                    path,
                    payload,
                    &mut res as *mut *mut c_char);

                println!("error_code {}",error_code);

                if error_code < 0 {
                    return Err(Box::from(format!("{}", error_code)));
                }
                
                let res = unsafe { CStr::from_ptr(res).to_string_lossy().into_owned() };
                println!("res {}",res);   
                return Ok(VadePluginResultValue::Success(Some(res.to_string())));
              } else {

                let did_result = Client::builder();
                #[cfg(not(target_arch = "wasm32"))]
                let did_result = did_result.timeout(Duration::from_secs(2));
                let did_result = did_result.build()?.get(resolver_url).send().await;
                match did_result {
                    Ok(result) => {
                        let request_result = match result.text().await {
                            Ok(text) => text.to_string(),
                            Err(_) => return Ok(VadePluginResultValue::Ignored),
                        };
                        let resolver_result: DidResolverResult = match serde_json::from_str(&request_result)
                        {
                            Ok(text) => text,
                            Err(_) => return Ok(VadePluginResultValue::Ignored),
                        };
                        let did_document = resolver_result.did_document.to_string();
                        Ok(VadePluginResultValue::Success(Some(did_document)))
                    }
                    Err(_) => Ok(VadePluginResultValue::Ignored),
                }
              }
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
