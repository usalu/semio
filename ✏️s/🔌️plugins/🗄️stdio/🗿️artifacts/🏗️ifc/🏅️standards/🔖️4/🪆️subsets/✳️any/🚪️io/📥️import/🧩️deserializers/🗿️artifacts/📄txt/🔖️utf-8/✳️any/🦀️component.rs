//! 📥️ Deserialize `stdio.ifc` from stdio.txt.
use crate::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<IfcSnapshot, store::TextError> {
    let document = crate::artifacts::step::engine::part21::parse_part21(from.to_body().trim()).map_err(|e| store::TextError::new(format!("ifc parse: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(crate::artifacts::ifc::schema::snapshot::from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<IfcSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
