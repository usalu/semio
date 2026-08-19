//! 📥️ Deserialize `stdio.ifc` from stdio.txt.
use crate::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
pub async fn register() {}
pub async fn deserialize(from: &TxtSnapshot) -> Result<IfcSnapshot, store::TextError> {
    let document = crate::artifacts::step::engine::part21::parse_part21(from.to_body().trim()).map_err(|e| store::TextError::new(format!("ifc parse: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(crate::artifacts::ifc::schema::snapshot::from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
}
pub async fn deserialize_text(text: &str) -> Result<IfcSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
