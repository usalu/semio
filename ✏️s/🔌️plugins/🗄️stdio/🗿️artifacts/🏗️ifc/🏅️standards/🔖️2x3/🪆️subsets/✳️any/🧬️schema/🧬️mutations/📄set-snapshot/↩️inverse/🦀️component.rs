use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::Ifc2x3Mutation;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) -> Vec<Ifc2x3Mutation> {
    <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(mutation, base)
}
