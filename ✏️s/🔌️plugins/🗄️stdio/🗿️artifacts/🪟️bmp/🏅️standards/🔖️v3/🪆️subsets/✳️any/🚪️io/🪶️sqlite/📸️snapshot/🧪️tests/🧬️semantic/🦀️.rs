//! 🧫️ Full current byte-owner SQL laws and independent image meaning.
use crate::standards::v_v3::subsets::any::io::sqlite::snapshot::tests::*;
fn sources() -> [(&'static str, &'static [u8]); 9] {
 [
  ("direct-rgb24-padding-gap-trailer",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")),
  ("indexed-rgb1-duplicate-palette",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb1-duplicate-palette.bmp")),
  ("indexed-rgb4-duplicate-palette",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb4-duplicate-palette.bmp")),
  ("indexed-rgb8-duplicate-palette",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp")),
  ("direct-rgb16-555",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb16-555.bmp")),
  ("direct-rgb32-reserved-sample",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb32-reserved-sample.bmp")),
  ("direct-bitfields16-565",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields16-565.bmp")),
  ("direct-bitfields32",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields32.bmp")),
  ("direct-rgb24-top-down",include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-top-down.bmp"))
 ]
}
fn project(snapshot: &BmpSnapshot) -> semio_framework_os_kernel::sqlite_snapshot::SqliteDatabase {
 snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,Default::default())).unwrap()
}
fn restore(database: &semio_framework_os_kernel::sqlite_snapshot::SqliteDatabase) -> BmpSnapshot {
 BmpSnapshot::from_sqlite_database(database,&mut SqliteSnapshotControl::new(&mut |_| true,Default::default())).unwrap()
}
#[test]
fn sqlite_snapshot_semantic_every_supported_profile_preserves_each_declared_native_octet() {
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️semantic/🔣️.json")).unwrap();
 assert_eq!(neutral["accepted"].as_array().unwrap().len(),sources().len());
 for (id,bytes) in sources() {
  assert!(neutral["accepted"].as_array().unwrap().iter().any(|value|value==id));
  let snapshot=crate::standards::v_v3::subsets::any::io::decode_bmp(bytes).unwrap();
  let database=project(&snapshot);
  assert_eq!(database.tables.len(),12,"{id}");
  assert_eq!(database.table("bmp_document").unwrap().rows[0].text(2).unwrap(),"valid_layout","{id}");
  assert!(database.tables.iter().flat_map(|table|&table.rows).flat_map(|row|&row.values).all(|value|!matches!(value,SqliteValue::Blob(_))));
  let restored=restore(&database);
  assert_eq!(restored,snapshot,"{id}");
  assert_eq!(crate::standards::v_v3::subsets::any::io::encode_bmp(&restored).unwrap(),bytes,"{id}");
  let (width,height,rgba)=semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::oracle_visual_rgba8(bytes).unwrap();
  let layout=crate::standards::v_v3::subsets::any::io::bmp_layout(&restored).unwrap();
  assert_eq!((width,height),(layout.width,layout.height),"{id}");
  assert_eq!(crate::standards::v_v3::subsets::any::io::bmp_rgba8_preview(&restored).unwrap(),rgba,"{id}");
  let file=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();
  let database=import_sqlite_database(&file,Default::default(),&mut |_|true).unwrap();
  assert_eq!(restore(&database),snapshot,"{id}");
 }
}
#[test]
fn sqlite_snapshot_semantic_literal_occurrences_preserve_empty_truncated_and_refused_native_owners() {
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️semantic/🔣️.json")).unwrap();
 let mut cases=neutral["literalCases"].as_array().unwrap().iter().map(|case|BmpSnapshot{schema:case["schema"].as_str().unwrap().into(),bytes:case["bytes"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap() as u8).collect()}).collect::<Vec<_>>();
 for bytes in [
  include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-12.bmp").as_slice(),include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-52.bmp").as_slice(),include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-56.bmp").as_slice(),include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-108.bmp").as_slice(),include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-124.bmp").as_slice()
 ] {cases.push(BmpSnapshot{schema:"literal.\0引用😀".into(),bytes:bytes.to_vec()});}
 let mut negative=fixture();negative.bytes[18..22].copy_from_slice(&(-1i32).to_le_bytes());cases.push(negative);
 for snapshot in cases {
  let diagnostic=crate::standards::v_v3::subsets::any::io::bmp_layout_bytes(&snapshot.bytes).unwrap_err();
  let database=project(&snapshot);
  let document=&database.table("bmp_document").unwrap().rows[0];
  assert_eq!(document.text(1).unwrap(),snapshot.schema);
  assert_eq!(document.text(2).unwrap(),"literal_octets");
  assert_eq!(document.text(3).unwrap(),diagnostic);
  let rows=&database.table("bmp_literal_octet").unwrap().rows;assert_eq!(rows.len(),snapshot.bytes.len());
  for (ordinal,row) in rows.iter().enumerate(){assert_eq!(row.integer(2).unwrap(),ordinal as i64);assert_eq!(row.integer(3).unwrap(),snapshot.bytes[ordinal] as i64);}
  assert!(database.tables.iter().filter(|table|table.name!="bmp_document"&&table.name!="bmp_literal_octet").all(|table|table.rows.is_empty()));
  assert_eq!(restore(&database),snapshot);
 }
 let mut arbitrary=fixture();arbitrary.schema="native-free.\0引用😀".into();assert_eq!(restore(&project(&arbitrary)),arbitrary);
}

#[path="⏱️row-tail/🦀️.rs"]
mod row_tail;
