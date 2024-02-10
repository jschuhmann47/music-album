use sha2::{Sha256, Digest};

pub fn sha256(input: String) -> String {
    let hash = Sha256::digest(input);
    base16ct::lower::encode_string(&hash)
}