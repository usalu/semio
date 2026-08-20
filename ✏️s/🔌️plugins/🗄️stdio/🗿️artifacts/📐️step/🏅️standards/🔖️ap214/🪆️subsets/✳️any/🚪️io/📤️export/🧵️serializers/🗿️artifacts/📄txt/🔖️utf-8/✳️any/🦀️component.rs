//! 📤️ Serialize `stdio.step` to stdio.txt.
use crate::artifacts::step::StepSnapshot;
use crate::artifacts::txt::TxtSnapshot;
pub async fn register() {}
pub async fn serialize(from: &StepSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::step::engine::part21::write_part21(&from.to_part21_document());
    Ok(TxtSnapshot::from_body(&text).await)
}
pub async fn serialize_text(from: &StepSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from).await?).await)
}
