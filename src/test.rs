use super::*;

#[tokio::test]
async fn get_hases_with_zeros_success() {
    let result = get_hash_with_zeros(4163, 3).await;
    let _answer = HashResult::Result((
        4163,
        "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000".to_string(),
    ));
    assert!(matches!(result, _answer));
}

#[tokio::test]
async fn get_hases_with_zeros_fail() {
    let result = get_hash_with_zeros(4163, 3).await;
    let _answer = HashResult::Error;
    assert!(matches!(result, _answer));
}

#[tokio::test]
async fn generate_sha256_hash_success() {
    let result = generate_sha256_hash(4163).await;
    let _answer = String::from("95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000");
    assert_eq!(result, _answer);
}

#[tokio::test]
async fn validate_sha256_hash_success() {
    let result = validate_sha256_hash(
        "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000",
        3,
    )
    .await;
    let _answer = true;
    assert_eq!(result, _answer);
}

#[tokio::test]
async fn validate_sha256_hash_fail() {
    let result = validate_sha256_hash(
        "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000",
        4,
    )
    .await;
    let _answer = false;
    assert_eq!(result, _answer);
}
