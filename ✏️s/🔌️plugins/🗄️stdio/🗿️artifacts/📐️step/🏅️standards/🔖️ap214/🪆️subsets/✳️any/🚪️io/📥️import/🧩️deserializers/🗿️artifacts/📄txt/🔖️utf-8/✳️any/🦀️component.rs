//! 📥️ Deserialize `stdio.step` from stdio.txt.
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::step::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &TxtSnapshot) -> Result<StepSnapshot, store::TextError> {
    let _ = STDIO_STEP_DOCUMENT_SCHEMA;
    let document = crate::artifacts::step::engine::part21::parse_part21(from.to_body().trim()).map_err(|e| {
        store::TextError::new(format!("step parse: {e}"), dsl::TextSpan::at(1, 1))
    })?;
    Ok(StepSnapshot::from_part21_document(document))
}
pub fn deserialize_text(text: &str) -> Result<StepSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
