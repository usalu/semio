//! 🔬️ WAV complete SQL backing is checked against actual system allocator requests.
use super::*;
#[path = "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observed;
#[test]
fn sqlite_snapshot_wav_relational_owned_requests_match_actual_allocator(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🔣️.json")).unwrap();
 assert_eq!(plan["phases"],serde_json::json!(["projectSnapshot","reconstructSnapshot"]));
 let words:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🔢️float32.json")).unwrap();
 let floats=words["ieee754Binary32Bits"].as_array().unwrap().iter().map(|value|f32::from_bits(value.as_u64().unwrap()as u32)).collect();
 for snapshot in[fixture(),WavSnapshot{data:WavData::Float32(floats),..fixture()}]{
  let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
  assert_eq!(database.tables.len(),plan["tableCount"].as_u64().unwrap()as usize);
  observed::verify_snapshot_backing::<WavSnapshot>(&snapshot,&database);
  snapshot.retire_sqlite_snapshot();
 }
}
