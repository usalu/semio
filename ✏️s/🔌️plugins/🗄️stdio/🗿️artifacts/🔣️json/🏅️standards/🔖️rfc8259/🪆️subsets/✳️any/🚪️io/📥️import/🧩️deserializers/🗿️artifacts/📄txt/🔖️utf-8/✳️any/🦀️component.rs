//! deser json via txt
use crate::artifacts::json::schema::snapshot::parse_json_text;
use crate::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
pub async fn register() {}
pub async fn deserialize(from: &TxtSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let value = parse_json_text(from.to_body().await.trim()).await?;
    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
pub async fn deserialize_text(text: &str) -> Result<JsonSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await
}
