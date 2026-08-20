//! 📥️ Deserialize `stdio.step` from stdio.txt.
use crate::artifacts::step::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
pub async fn register() {}
pub async fn deserialize(from: &TxtSnapshot) -> Result<StepSnapshot, store::TextError> {
    let _ = STDIO_STEP_DOCUMENT_SCHEMA;
    let document = crate::artifacts::step::engine::part21::parse_part21(from.to_body().await.trim()).await.map_err(|e| store::TextError::new(format!("step parse: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(StepSnapshot::from_part21_document(document).await)
}
pub async fn deserialize_text(text: &str) -> Result<StepSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await
}
