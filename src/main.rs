use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct DIDResolveResponse {}





fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ToDo: replace did , replace host with config value
    let args: Vec<String> = env::args().collect();
    let did = &args[1];
    let mut resolver_url = env::var("RESOLVER_URL").ok().unwrap();
    resolver_url.push_str(did);


    let resp = reqwest::blocking::get(&resolver_url)?
        .text()?;
    println!("{:#?}", resp);
    println!("{:?}", resolver_url);

    Ok(())
}
