//! 📤️ Serialize `stdio.ifc` to stdio.txt.
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::ifc::IfcSnapshot;
pub fn register() {}
pub fn serialize(from: &IfcSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::ifc::schema::snapshot::ifc_brep_to_text(&from.brep);
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}
pub fn serialize_text(from: &IfcSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
