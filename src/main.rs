use std::env;
use std::collections::BTreeMap;

fn main() {
    let mut data = BTreeMap::new();
    for envvar in env::vars() {
        data.insert(envvar.0, envvar.1);
    }

    println!("{}", serde_json::to_string(&data).unwrap());    
}
