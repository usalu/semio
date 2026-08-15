use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::{apply_html_mutation, HtmlMutation};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut HtmlSnapshot, mutation: &HtmlMutation) {
    let _ = apply_html_mutation(projection, mutation);
}
