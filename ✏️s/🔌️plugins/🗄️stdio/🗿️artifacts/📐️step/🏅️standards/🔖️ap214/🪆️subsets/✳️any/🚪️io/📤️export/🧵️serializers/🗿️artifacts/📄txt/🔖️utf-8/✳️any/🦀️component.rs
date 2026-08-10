//! 📤️ Serialize `stdio.step` to stdio.txt.
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::step::StepSnapshot;
pub fn register() {}
pub fn serialize(from: &StepSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::step::schema::snapshot::step_brep_to_text(&from.brep);
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}
pub fn serialize_text(from: &StepSnapshot) -> Result<String, store::PackError> {
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}
