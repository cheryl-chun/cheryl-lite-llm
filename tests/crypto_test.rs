use sha2::{Sha256, Digest};

// 计算 SHA256 hash
fn compute_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[test]
fn test_sha256_hash() {
    let token = "sk_test_123456";
    let hash = compute_hash(token);

    // SHA256 输出是 64 个十六进制字符
    assert_eq!(hash.len(), 64);

    // 相同输入产生相同 hash
    assert_eq!(hash, compute_hash(token));
}

#[test]
fn test_hash_uniqueness() {
    let hash1 = compute_hash("sk_test_123456");
    let hash2 = compute_hash("sk_test_123457");

    // 不同输入产生不同 hash
    assert_ne!(hash1, hash2);
}
