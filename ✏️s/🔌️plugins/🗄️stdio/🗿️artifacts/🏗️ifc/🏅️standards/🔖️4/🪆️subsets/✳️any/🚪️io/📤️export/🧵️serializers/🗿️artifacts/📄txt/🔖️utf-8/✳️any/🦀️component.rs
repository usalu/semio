//! 📤️ Serialize `stdio.ifc` to stdio.txt.
use crate::artifacts::ifc::IfcSnapshot;
use crate::artifacts::txt::TxtSnapshot;
pub fn register() {}
pub fn serialize(from: &IfcSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::step::engine::part21::write_part21(&crate::artifacts::ifc::schema::snapshot::to_part21_document(from));
    Ok(TxtSnapshot::from_body(&text))
}
pub fn serialize_text(from: &IfcSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
