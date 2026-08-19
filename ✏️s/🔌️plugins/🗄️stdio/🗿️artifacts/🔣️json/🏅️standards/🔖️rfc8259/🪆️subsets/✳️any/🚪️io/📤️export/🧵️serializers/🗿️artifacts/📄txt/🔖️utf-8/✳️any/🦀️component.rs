//! ser json to txt
use crate::artifacts::json::schema::snapshot::write_json_pretty;
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::txt::TxtSnapshot;
pub async fn register() {}
pub async fn serialize(from: &JsonSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = write_json_pretty(&from.value);
    Ok(TxtSnapshot::from_body(&text))
}
pub async fn serialize_text(from: &JsonSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
