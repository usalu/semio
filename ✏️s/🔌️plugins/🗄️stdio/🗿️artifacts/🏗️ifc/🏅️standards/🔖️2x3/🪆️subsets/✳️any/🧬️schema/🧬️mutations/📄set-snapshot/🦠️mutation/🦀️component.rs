use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{Ifc2x3Mutation, apply_ifc2x3_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) {
    apply_ifc2x3_mutation(projection, mutation);
}
