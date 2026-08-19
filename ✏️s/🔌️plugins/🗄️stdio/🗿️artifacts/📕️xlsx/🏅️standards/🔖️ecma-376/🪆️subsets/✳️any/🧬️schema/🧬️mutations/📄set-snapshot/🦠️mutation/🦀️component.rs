use crate::artifacts::xlsx::schema::mutations::{apply_xlsx_mutation, XlsxMutation};
use crate::artifacts::xlsx::XlsxSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut XlsxSnapshot, mutation: &XlsxMutation) {
    apply_xlsx_mutation(projection, mutation);
}
