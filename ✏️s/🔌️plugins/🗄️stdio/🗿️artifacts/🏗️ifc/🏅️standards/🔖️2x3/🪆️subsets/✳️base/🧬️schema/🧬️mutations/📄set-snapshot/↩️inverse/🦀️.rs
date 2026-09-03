use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation;
use crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) -> Vec<Ifc2x3Mutation> {
    <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(mutation, base)
}
