//! 🔬️ Complete direct domain backing is observed without a copied carrier authority.
#[path = "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observer;
use crate::standards::v1::subsets::any::io::sqlite::snapshot::*;
use store::ArtifactSqliteSnapshot;

pub(super) fn verify(snapshot: &NoteSnapshot) {
    let limits = sqlite_snapshot::SqliteDatabaseLimits::default();
    let mut callback = |_| true;
    let database = snapshot.to_sqlite_database(&mut Control::new(&mut callback, limits)).unwrap();
    observer::verify_snapshot_backing_by(snapshot, &database, |actual, expected| {
        assert_eq!(store::ArtifactPack::encode_pack(actual), store::ArtifactPack::encode_pack(expected));
        assert_eq!(actual.grid_spacing.unwrap().to_bits(), expected.grid_spacing.unwrap().to_bits());
    });
    observer::verify_snapshot_failure_backing(snapshot, &database, 65_536);
}
