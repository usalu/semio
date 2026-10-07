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
fn sqlite_snapshot_semantic_every_supported_profile_preserves_precise_owned_values() {
 for (name,bytes) in sources() {
  let snapshot=crate::standards::v_v3::subsets::any::io::decode_bmp(bytes).unwrap();
  let database=project(&snapshot);
  assert_eq!(database.table("bmp_image").unwrap().rows.len(),1);
  assert!(database.tables.iter().all(|table|!table.name.contains("literal")&&!table.name.contains("padding")));
  let restored=restore(&database);assert_eq!(restored,snapshot,"{name}: exact native precision and metadata");
  let encoded=crate::standards::v_v3::subsets::any::io::encode_bmp(&restored).unwrap();
  let (_,_,independent)=semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::oracle_visual_rgba8(&encoded).unwrap();
  assert_eq!(crate::schema::operations::bmp_rgba8_preview(&restored).unwrap(),independent,"{name}: independent image crate");
  eprintln!("[DEBUG] bmp semantic profile={name} precise-values=exact");
 }
}

#[test]
fn sqlite_snapshot_semantic_refuses_invalid_owned_models_and_orphan_samples() {
 let fixture=fixture();let mut models=Vec::new();
 let mut wrong_schema=fixture.clone();wrong_schema.schema="undeclared".into();models.push(wrong_schema);
 let mut wrong_count=fixture.clone();wrong_count.image.width+=1;models.push(wrong_count);
 let mut wrong_precision=fixture.clone();let crate::schema::snapshot::BmpPixels::Direct {samples}=&mut wrong_precision.image.pixels else {panic!("direct fixture");};samples[0].red=256;models.push(wrong_precision);
 for snapshot in models {assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).is_err());}
 let mut database=project(&fixture);database.table_mut("bmp_pixel_sample").unwrap().rows[0].values[1]=SqliteValue::Integer(2);
 assert!(BmpSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).is_err());
 eprintln!("[DEBUG] bmp semantic invalid-owned-refusals=4");
}
