use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &AviSnapshot, mutation: &AviMutation) -> Vec<AviMutation> {
    <AviMutation as Mutation<AviSnapshot>>::inverse(mutation, base)
}
