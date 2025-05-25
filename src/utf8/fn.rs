/// Checks if the given byte slice is valid UTF-8.
///
/// - `data`: The byte slice to validate.
/// - Returns: `true` if the byte slice is valid UTF-8, otherwise `false`.
pub fn is_valid_utf8(data: &[u8]) -> bool {
    std::str::from_utf8(data).is_ok()
}
