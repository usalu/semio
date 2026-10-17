//! 🔬️ Complete Layout ownership is checked against actual System allocation requests.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::*;
use store::os_store::serialization::ArtifactSqliteSnapshotHost;
use semio_framework_io_sqlite_snapshot_testing_backing as observed;
fn literal_equal(actual:&LayoutSnapshot,expected:&LayoutSnapshot){assert_eq!(actual.data_fields,expected.data_fields,"complete typed dictionary tags, words, octets and member order");assert_eq!(pack::snapshot::ArtifactPack::encode_pack(actual),pack::snapshot::ArtifactPack::encode_pack(expected),"complete Layout native fields and raw words");}
#[test]
fn sqlite_snapshot_layout_full_concrete_requests_preserve_every_owned_field(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧾️dictionary/🔣️.json")).unwrap();assert_eq!(plan["allocation"],"fullConcreteRequests");assert_eq!(plan["retirementRefund"],false);
 let words:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for(index,hex)in words["float64Bits"].as_array().unwrap().iter().enumerate(){
  let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let color=u32::from_str_radix(words["float32Bits"][index].as_str().unwrap(),16).unwrap();
  let expected=complete_fixture(f64::from_bits(bits),f32::from_bits(color));let mut accepted=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut accepted,store::sqlite_snapshot::SqliteDatabaseLimits::default());
  let database=expected.to_sqlite_database(&mut control).unwrap();observed::verify_snapshot_backing_by(&expected,&database,literal_equal);expected.retire_sqlite_snapshot();
 }
}

