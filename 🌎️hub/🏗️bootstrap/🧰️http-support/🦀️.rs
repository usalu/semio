//! 🧰️ `support` — HTTP support shared by every route family.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use directory::{DslValue, FromValue, ToValue};
use futures::StreamExt;
use semio_hub_artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub_directory_contract::error::DirectoryError;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;

/// 📦️ Axum JSON boundary for first-party `ToValue`/`FromValue` directory contracts.
pub struct DirectoryJson<T>(pub T);

impl<'de, T: FromValue> Deserialize<'de> for DirectoryJson<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        T::from_value(DslValue::from(value)).map(Self).map_err(serde::de::Error::custom)
    }
}

impl<T: ToValue> IntoResponse for DirectoryJson<T> {
    fn into_response(self) -> Response {
        ([(axum::http::header::CONTENT_TYPE, "application/json")], semio_framework_pack_json::to_json_string(&self.0)).into_response()
    }
}

pub fn now_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

//#region 🔖️State
pub fn bearer(headers: &HeaderMap) -> Option<String> {
    headers.get(axum::http::header::AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).map(|value| value.to_string())
}

pub fn db_error_status(error: &db::DbError) -> StatusCode {
    match error {
        db::DbError::NotFound(_) => StatusCode::NOT_FOUND,
        db::DbError::AlreadyExists(_) | db::DbError::Conflict(_) => StatusCode::CONFLICT,
        db::DbError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        db::DbError::InvalidArgument(_) | db::DbError::LimitExceeded(_) => StatusCode::BAD_REQUEST,
        db::DbError::Unavailable(_) | db::DbError::Timeout(_) => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// #⃣ Decodes a 64-hex-char blob URL path segment into a `db::ContentHash` — the inverse of
/// `ContentHash`'s `Display` (see `pack_core::ContentHash`), never trusted as-is (a malformed path
/// is `BAD_REQUEST`, not a panic).
pub fn parse_content_hash(hex: &str) -> Option<db::ContentHash> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    let raw = hex.as_bytes();
    for (index, slot) in bytes.iter_mut().enumerate() {
        let byte_str = std::str::from_utf8(&raw[index * 2..index * 2 + 2]).ok()?;
        *slot = u8::from_str_radix(byte_str, 16).ok()?;
    }
    Some(db::ContentHash(bytes))
}
//#endregion 🔖️State

//#region 🔖️Directory
pub fn directory_error_status(error: DirectoryError) -> StatusCode {
    match error {
        DirectoryError::NotFound(_) => StatusCode::NOT_FOUND,
        DirectoryError::Conflict(_) => StatusCode::CONFLICT,
        DirectoryError::Unauthorized => StatusCode::UNAUTHORIZED,
        DirectoryError::Backend(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
//#endregion 🔖️Directory
