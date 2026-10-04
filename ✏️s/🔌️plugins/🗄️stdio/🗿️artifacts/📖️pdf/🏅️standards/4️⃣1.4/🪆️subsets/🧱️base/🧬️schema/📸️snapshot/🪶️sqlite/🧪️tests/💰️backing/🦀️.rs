//! 🔬️ Complete direct domain backing is observed without a copied carrier authority.
#[path = "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observer;
use super::*;
use store::ArtifactSqliteSnapshot;

pub(super) fn verify(snapshot: &PdfSnapshot) {
    let limits = sqlite_snapshot::SqliteDatabaseLimits::default();
    let mut callback = |_| true;
    let database = snapshot.to_sqlite_database(&mut Control::new(&mut callback, limits)).unwrap();
    observer::verify_snapshot_backing_by(snapshot, &database, |actual, expected| {
        assert_eq!(actual.schema, expected.schema);
        assert_eq!(actual.pages.len(), expected.pages.len());
        for (actual, expected) in actual.pages.iter().zip(&expected.pages) {
            assert_eq!(actual.width.to_bits(), expected.width.to_bits());
            assert_eq!(actual.height.to_bits(), expected.height.to_bits());
            assert_eq!(actual.text, expected.text);
        }
    });
    observer::verify_snapshot_failure_backing(snapshot, &database, 65_536);
}
