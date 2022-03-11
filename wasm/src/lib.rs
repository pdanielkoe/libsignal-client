use libsignal_protocol::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() -> String {
    "Hell o World!!!".to_string()
}

#[wasm_bindgen]
pub fn ecprivate_key_generate() -> PrivateKey {
    let mut rng = rand::rngs::OsRng;
    let keypair = KeyPair::generate(&mut rng);
    keypair.private_key
}