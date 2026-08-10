use crate::artifacts::xlsx::{XlsxSnapshot};
use crate::artifacts::xlsx::schema::mutations::{XlsxMutation, apply_xlsx_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut XlsxSnapshot, mutation: &XlsxMutation) {
    apply_xlsx_mutation(projection, mutation);
}
