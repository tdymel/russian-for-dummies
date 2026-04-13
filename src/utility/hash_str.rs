use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

pub fn hash_to_base64(input: &str) -> String {
    let hash = Sha256::digest(input.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}
