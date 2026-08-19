use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation;
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &HtmlSnapshot, mutation: &HtmlMutation) -> Vec<HtmlMutation> {
    <HtmlMutation as Mutation<HtmlSnapshot>>::inverse(mutation, base)
}
