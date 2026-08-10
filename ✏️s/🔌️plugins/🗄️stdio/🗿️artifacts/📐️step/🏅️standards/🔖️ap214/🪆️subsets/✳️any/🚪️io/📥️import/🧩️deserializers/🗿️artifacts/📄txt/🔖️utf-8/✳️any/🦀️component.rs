//! 📥️ Deserialize `stdio.step` from stdio.txt.
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::step::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &TxtSnapshot) -> Result<StepSnapshot, store::TextError> {
    let brep = crate::artifacts::step::schema::snapshot::step_brep_from_text(from.text.trim()).map_err(|e| {
        store::TextError::new(format!("step parse: {e}"), dsl::TextSpan::at(1, 1))
    })?;
    Ok(StepSnapshot { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), brep })
}
pub fn deserialize_text(text: &str) -> Result<StepSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
