use std::env;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let param:Value = serde_json::from_str(&args[1]).unwrap();
    let resp = ureq::get(param["url"].as_str().unwrap()).call().unwrap();

    println!("{}", resp.into_string().unwrap().as_str());
}
