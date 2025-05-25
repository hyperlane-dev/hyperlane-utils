use crate::*;

/// Parses a JSON value from a byte slice.
///
/// # Arguments
/// - `data` - A byte slice containing JSON data
///
/// # Returns
/// - `Ok(T)` if parsing succeeds, where T is the deserialized value
/// - `Err(SerdeJsonError)` if parsing fails
pub fn json_parse_from_slice<T>(data: &[u8]) -> ResultSerdeJsonError<T>
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
pub fn json_parse_from_str<T>(data: &str) -> ResultSerdeJsonError<T>
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
pub fn json_stringify<T>(data: &T) -> ResultSerdeJsonError<String>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(data)
}
