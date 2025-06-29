use hyperlane_utils::*;

#[test]
fn test_is_valid_utf8() {
    let valid_utf8: &[u8] = "Hello, world!".as_bytes();
    assert!(is_valid_utf8(valid_utf8));

    let invalid_utf8: &[u8; 3] = &[0xFF, 0xFE, 0xFD];
    assert!(!is_valid_utf8(invalid_utf8));
}
