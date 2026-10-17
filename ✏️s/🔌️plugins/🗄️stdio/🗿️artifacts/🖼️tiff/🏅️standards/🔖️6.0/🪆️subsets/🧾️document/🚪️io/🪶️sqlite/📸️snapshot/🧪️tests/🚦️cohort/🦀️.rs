//! 🚦️ Exact owned sample words and contiguous relational positions.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v6_0::subsets::document::io::sqlite::snapshot::*;
use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffSnapshot,TiffWord64};
use semio_framework_os_kernel::{sqlite_snapshot::*, os_store::serialization::ArtifactSqliteSnapshotHost};
use semio_framework_test_fixtures::fixture_text;
pub(super) fn fixture()->TiffSnapshot{let neutral:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎛️semantic.json")).unwrap();let case=neutral["cases"].as_array().unwrap().iter().find(|case|case["id"]=="metadataAndExactRaster").unwrap();semio_framework_pack_json::from_json_str(&case["input"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}

pub(super) fn project(snapshot: &TiffSnapshot) -> SqliteDatabase {
    snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).expect("project")
}

pub(super) fn restore(database: &SqliteDatabase) -> Result<TiffSnapshot, ValueError> {
    TiffSnapshot::from_sqlite_database(database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default()))
}
#[test]fn owned_word_precision_and_sample_ordinals_survive_projection(){let source=fixture();let restored=restore(&project(&source)).unwrap();assert_eq!(restored.ifds[1].blocks[0].samples,vec![TiffWord64{lo:32769,hi:0},TiffWord64{lo:65535,hi:0}]);}
#[test]fn sample_ordinals_must_be_contiguous(){let mut database=project(&fixture());database.table_mut("tiff_sample").unwrap().rows[1].values[2]=SqliteValue::Integer(4);assert!(restore(&database).is_err());}
