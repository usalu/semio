use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::{SemioDocumentDiff, diff_set_snapshot};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;

/// 🔺️ Diff helper for set-snapshot.
pub fn diff(base: &SemioDocumentSnapshot, snapshot: &SemioDocumentSnapshot) -> SemioDocumentDiff {
    diff_set_snapshot(base, snapshot)
}
