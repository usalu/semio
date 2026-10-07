use crate::standards::v1_2::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{
    sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue},
    ArtifactSqliteSnapshot,
};

fn fixture() -> PngSnapshot {
    crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-multi-idat-private.png")).unwrap()
}

#[test]
fn sqlite_snapshot_owns_precise_samples_and_semantic_metadata() {
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(database.tables.len(),13);assert_eq!(database.table("png_image").unwrap().single_row().unwrap().text(1).unwrap(),snapshot.schema);assert_eq!(database.table("png_sample").unwrap().rows.len(),snapshot.image.samples.len());assert!(database.table("png_literal_octet").is_err());assert!(database.table("png_deflate_block").is_err());
 let file=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();let imported=import_sqlite_database(&file,Default::default(),&mut |_|true).unwrap();let restored=PngSnapshot::from_sqlite_database(&imported,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(restored,snapshot);assert_eq!(crate::standards::v1_2::subsets::any::io::decode_png(&crate::standards::v1_2::subsets::any::io::encode_png(&restored).unwrap()).unwrap(),snapshot);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_png_public_file_round_trip_preserves_owned_sample_metadata() {
    use {semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot,semio_framework_artifact_reference::ArtifactDialect};
    use store::sqlite_snapshot::SnapshotEncoding;
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")
        .label("PNG canonical SQLite").version("0.0.1").package_id("semio:stdio")
        .artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.png".into(), standard: "1.2".into(), subset: "*".into() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let snapshot = fixture();
        let file = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, Default::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(&file[..16], b"SQLite format 3\0");
        let restored = io_import_sqlite_snapshot::<PngSnapshot>(&dialect, &file, Default::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(restored, snapshot);
    }
}

#[test]
fn sqlite_snapshot_png_reconstruction_refuses_invalid_owned_samples_atomically() {
 let snapshot=fixture();let mut database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();database.table_mut("png_sample").unwrap().rows[0].values[3]=SqliteValue::Integer(65535);assert!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).is_err());assert_eq!(snapshot,fixture());
}
#[test]
fn sqlite_snapshot_png_projection_and_reconstruction_obey_cancellation_and_ownership_limits() {
 let snapshot=fixture();assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,Default::default())).is_err());let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|false,Default::default())).is_err());let limits=SqliteDatabaseLimits{max_allocation_bytes:1,..Default::default()};assert!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
}
#[test]
fn sqlite_snapshot_png_independent_client_edits_one_native_sample() {
 use std::{io::Write,process::{Command,Stdio}};
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();let file=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();let script=r#"import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));db.run('UPDATE png_sample SET value=7 WHERE ordinal=0');if(db.query('PRAGMA foreign_key_check').all().length)throw Error('foreign keys');await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let edited=import_sqlite_database(&output.stdout,Default::default(),&mut |_|true).unwrap();let restored=PngSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();let mut expected=snapshot;expected.image.samples[0]=7;assert_eq!(restored,expected);let bytes=crate::standards::v1_2::subsets::any::io::encode_png(&restored).unwrap();let mut reader=png::Decoder::new(std::io::Cursor::new(bytes)).read_info().unwrap();let mut pixels=vec![0;reader.output_buffer_size().unwrap()];reader.next_frame(&mut pixels).unwrap();assert_eq!(pixels[0],7);println!("[DEBUG] PNG independent SQLite client edited one exact owned sample; png oracle confirmed published pixel");
}
fn independent_extent(snapshot:&PngSnapshot)->serde_json::Value {
 use std::{io::Write,process::{Command,Stdio}};
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();let file=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nconst file=new Uint8Array(await Bun.stdin.arrayBuffer());await Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(file)));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_png_complete_independent_owned_semantic_limits() {
 use store::sqlite_snapshot::SnapshotEncoding;
 let source=fixture();let extent=independent_extent(&source);let rows=extent["rows"].as_u64().unwrap()as usize;let value=extent["valueBytes"].as_u64().unwrap()as usize;let schema=extent["schemaBytes"].as_u64().unwrap()as usize;let limits=SqliteDatabaseLimits{max_rows:rows,max_value_bytes:value,max_schema_bytes:schema.max(PngSnapshot::SQLITE_SCHEMA.len()),max_tables:13,max_columns:9,..Default::default()};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(PngSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
  for short in [SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:value-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:12,..limits},SqliteDatabaseLimits{max_columns:8,..limits}] {assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"projection {short:?}");assert!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"reconstruction {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight {short:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"encode {short:?}");assert!(PngSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"decode {short:?}");}
 }println!("[DEBUG] PNG exact relational cell limits matched independent SQLite census rows={rows} values={value} schema={schema}");
}
#[test]
fn sqlite_snapshot_png_complete_independent_copied_columns_admission() {
 let source=fixture();let limits=SqliteDatabaseLimits{max_columns:8,..Default::default()};assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in [store::sqlite_snapshot::SnapshotEncoding::Text,store::sqlite_snapshot::SnapshotEncoding::Binary] {assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}
}
