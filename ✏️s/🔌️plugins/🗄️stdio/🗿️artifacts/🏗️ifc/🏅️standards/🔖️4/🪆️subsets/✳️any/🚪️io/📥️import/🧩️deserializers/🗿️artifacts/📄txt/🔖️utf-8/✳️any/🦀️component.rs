//! 📥️ Deserialize `stdio.ifc` from stdio.txt.
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &TxtSnapshot) -> Result<IfcSnapshot, store::TextError> {
    let document = crate::artifacts::step::engine::part21::parse_part21(from.text.trim()).map_err(|e| {
        store::TextError::new(format!("ifc parse: {e}"), dsl::TextSpan::at(1, 1))
    })?;
    Ok(IfcSnapshot { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), document })
}
pub fn deserialize_text(text: &str) -> Result<IfcSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
