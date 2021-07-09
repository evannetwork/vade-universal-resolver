use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DIDResolveResponse {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://dev.uniresolver.io/1.0/identifiers/did:ethr:mainnet:0x3b0BC51Ab9De1e5B7B6E34E5b960285805C41736")?
        .text()?;
    println!("{:#?}", resp);
    Ok(())
}
