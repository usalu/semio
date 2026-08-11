use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::{HtmlMutation, apply_html_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut HtmlSnapshot, mutation: &HtmlMutation) {
    let _ = apply_html_mutation(projection, mutation);
}
