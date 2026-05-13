use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use sha2::{Digest, Sha256};

pub fn generate_random_key(prefix: &str) -> String {
    let mut rng = rand::thread_rng();

    let random_bytes: Vec<u8> =
        (0..32).map(|_| rng.r#gen()).collect();

    let random_str = URL_SAFE_NO_PAD.encode(&random_bytes);

    format!("{}{}", prefix, &random_str[..40])
}

pub fn compute_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}