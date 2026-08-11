use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioMeshSnapshot, snapshot: &SemioMeshSnapshot) -> SemioMeshDiff {
    diff_set_snapshot(base, snapshot)
}
