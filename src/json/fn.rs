use crate::*;

/// Parses a JSON value from a byte slice.
///
/// # Arguments
/// - `data` - A byte slice containing JSON data
///
/// # Returns
/// - `Ok(T)` if parsing succeeds, where T is the deserialized value
/// - `Err(SerdeJsonError)` if parsing fails
pub fn json_parse_from_slice<T>(data: &[u8]) -> Result<T, SerdeJsonError>
where
    T: DeserializeOwned,
{
    serde_json::from_slice(data)
}

/// Parses a JSON value from a string slice.
///
/// # Arguments
/// - `data` - A string slice containing JSON data
///
/// # Returns
/// - `Ok(T)` if parsing succeeds, where T is the deserialized value
/// - `Err(SerdeJsonError)` if parsing fails
pub fn json_parse_from_str<T>(data: &str) -> Result<T, SerdeJsonError>
where
    T: DeserializeOwned,
{
    serde_json::from_str(data)
}

/// Serializes a value to a JSON string.
///
/// # Arguments
/// - `data` - The value to serialize
///
/// # Returns
/// - `Ok(String)` containing the JSON text if serialization succeeds
/// - `Err(SerdeJsonError)` if serialization fails
pub fn json_stringify_string<T>(data: &T) -> Result<String, SerdeJsonError>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(data)
}

/// Serializes a value to a JSON byte vector.
///
/// # Arguments
/// - `data` - The value to serialize
///
/// # Returns
/// - `Ok(Vec<u8>)` containing the JSON bytes if serialization succeeds
/// - `Err(SerdeJsonError)` if serialization fails
pub fn json_stringify_binary<T>(data: &T) -> Result<Vec<u8>, SerdeJsonError>
where
    T: ?Sized + Serialize,
{
    serde_json::to_vec(data)
}
