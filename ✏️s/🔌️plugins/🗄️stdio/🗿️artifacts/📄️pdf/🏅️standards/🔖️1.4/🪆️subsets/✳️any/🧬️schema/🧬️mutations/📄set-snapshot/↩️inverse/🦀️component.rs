use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::PdfMutation;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &PdfSnapshot, mutation: &PdfMutation) -> Vec<PdfMutation> {
    <PdfMutation as Mutation<PdfSnapshot>>::inverse(mutation, base).await
}
