use crate::standards::v1_2::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{
    sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue},
    ArtifactSqliteSnapshot,
};

fn fixture() -> PngSnapshot {
    crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-multi-idat-private.png")).unwrap()
}

#[test]
fn sqlite_snapshot_owns_one_exact_canonical_byte_carrier() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let table = database.table("png_document").unwrap();
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].text(1).unwrap(), snapshot.schema);
    assert_eq!(table.rows[0].text(2).unwrap(), "native");
    assert_eq!(database.tables.len(),24);
    assert!(database.table("png_literal_octet").unwrap().rows.is_empty());
    assert_eq!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).filter(|value| matches!(value, SqliteValue::Blob(_))).count(), 0);
    let file = export_sqlite_database(&database, Default::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&file, Default::default(), &mut |_| true).unwrap();
    let restored = PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    assert_eq!(restored, snapshot);
    assert_eq!(crate::standards::v1_2::subsets::any::io::encode_png(&restored).unwrap(), snapshot.bytes);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_png_public_file_round_trip_preserves_chunk_order_and_idat_boundaries() {
    use semio_framework_os_kernel::io::{io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}, ArtifactDialect};
    use store::sqlite_snapshot::SnapshotEncoding;
    let factories=crate::native_codecs();let factory=&factories[0];let codec=(factory.codec)();let kind=(factory.kind)();
    let declared:serde_json::Value=serde_json::from_str(crate::ARTIFACT_DEFINITION_SCHEMA).unwrap();let binding=&declared["codecs"][0]["native_factory"];
    let hash:String=codec.pack_schema_hash.iter().map(|byte|format!("{byte:02x}")).collect();
    eprintln!("[DEBUG] PNG actual native factory={} artifact={} kind={} schema={} extension={} pack_hash={} declared_binding={}",factory.id,factory.artifact,kind.id,codec.schema,codec.extension,hash,binding);
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
fn sqlite_snapshot_png_reconstruction_refuses_invalid_canonical_bytes_atomically() {
    let snapshot = fixture();
    let mut database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    database.table_mut("png_document").unwrap().rows[0].values[2] = SqliteValue::Text("literal".into());
    assert!(PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).is_err());
}

#[test]
fn sqlite_snapshot_png_projection_and_reconstruction_obey_cancellation_and_ownership_limits() {
    let snapshot = fixture();
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, Default::default())).is_err());
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_allocation_bytes: snapshot.bytes.len() - 1, ..Default::default() };
    assert!(PngSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}

#[test]
fn sqlite_snapshot_png_semantic_chunks_scanlines_and_exact_native_owner() {
    use std::{io::Write,process::{Command,Stdio}};
    let neutral:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧩️semantic/🔣️.json")).unwrap();
    let snapshot=crate::standards::v1_2::subsets::any::io::decode_png(include_bytes!("../../../../../../../../🧫️fixtures/🧬️canonical-source/indexed-2bit-duplicate-palette.png")).unwrap();
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
    let header=database.table("png_ihdr").expect("real native PNG must expose all seven interpreted IHDR fields");
    assert_eq!(header.rows.len(),1);
    assert_eq!(header.rows[0].integer(1).unwrap(),neutral["ihdr"]["width"].as_i64().unwrap());
    assert!(database.table("png_scanline").expect("native PNG must expose actual filtered scanlines").rows.len()>0);
    assert!(database.table("png_sample").expect("native PNG must expose packed source samples").rows.len()>0);
    assert!(database.table("png_deflate_block").expect("native PNG must expose complete compression blocks").rows.len()>0);
    let file=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();
    let script=r#"import {Database} from 'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));console.log(JSON.stringify({header:d.query('SELECT width,height,bit_depth AS bitDepth,color_type AS colorType,compression,filter,interlace FROM png_ihdr').get(),chunks:d.query('SELECT kind FROM png_chunk ORDER BY ordinal').all().map(r=>r.kind),fk:d.query('PRAGMA foreign_key_check').all(),blobs:d.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table' AND sql LIKE '%BLOB%'").get()}));d.close();"#;
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let write=child.stdin.take().unwrap().write_all(&file);let output=child.wait_with_output().unwrap();
    assert!(write.is_ok()&&output.status.success(),"independent SQL oracle: write={write:?}; stderr={}",String::from_utf8_lossy(&output.stderr));
    let observed:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(observed["header"],neutral["ihdr"]);assert_eq!(observed["chunks"],neutral["chunkKinds"]);assert_eq!(observed["fk"],serde_json::json!([]));assert_eq!(observed["blobs"]["count"],0);
    assert_eq!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap(),snapshot);
    for encoding in [store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text]{let native=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(PngSnapshot::decode_sqlite_snapshot_native(&native,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap(),snapshot);}
    let mut edited=database;edited.table_mut("png_ihdr").unwrap().rows[0].values[1]=SqliteValue::Integer(neutral["edit"]["value"].as_i64().unwrap());
    let restored=PngSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
    assert_ne!(restored.bytes,snapshot.bytes);
    let script=r#"import {PNG} from 'pngjs';const image=PNG.sync.read(Buffer.from(await Bun.stdin.arrayBuffer()));console.log(JSON.stringify({width:image.width,height:image.height,rgba:[...image.data]}));"#;
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();let write=child.stdin.take().unwrap().write_all(&restored.bytes);let output=child.wait_with_output().unwrap();assert!(write.is_ok()&&output.status.success(),"independent pixel oracle: write={write:?}; stderr={}",String::from_utf8_lossy(&output.stderr));let pixels:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(pixels["width"],neutral["edit"]["value"]);assert_eq!(pixels["rgba"],neutral["editedRgba"]);
}

fn norm_complete_source(case:&serde_json::Value)->PngSnapshot{if case["id"]=="nativeCanonicalAsset"{return fixture()}let json:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let bytes=if case["id"]=="emptyLiteral"{Vec::new()}else if case["id"]=="invalidRawLiteral"{vec![0,255,1]}else{json["bytes"].as_array().unwrap().iter().map(|value|u8::try_from(value.as_u64().unwrap()).unwrap()).collect()};PngSnapshot{schema:"stdio.png".into(),bytes}}

fn norm_independent_complete_extent(source:&PngSnapshot)->serde_json::Value{
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nconst file=new Uint8Array(await Bun.stdin.arrayBuffer());const db=Database.deserialize(file);const role=db.query(\"SELECT role FROM png_document\").get().role;db.close();const carrier=new Uint8Array(JSON.parse(process.argv[1]));await Bun.write(Bun.stdout,JSON.stringify({...independentSqliteExtent(file),role,sha256:new Bun.CryptoHasher(\"sha256\").update(carrier).digest(\"hex\")}));");let carrier=serde_json::to_string(&source.bytes).unwrap();let mut child=Command::new("bun").args(["-e",script,&carrier]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_png_complete_independent_native_semantic_limits(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎛️semantic.json")).unwrap();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);assert_eq!(extent["role"],case["role"]);assert_eq!(extent["sha256"],case["sha256"]);assert_eq!(source.bytes.len(),case["rawBytes"].as_u64().unwrap()as usize);let limits=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:24,max_columns:9,..SqliteDatabaseLimits::default()};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(PngSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:23,..limits},SqliteDatabaseLimits{max_columns:8,..limits}]{assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"relational copied limits {short:?}");assert!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"reconstruct copied limits {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight copied limits {encoding:?} {short:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"encoder copied limits {encoding:?} {short:?}");assert!(PngSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"decoder copied limits {encoding:?} {short:?}");}
  }
 }eprintln!("[DEBUG] PNG independently measured complete copied native semantic limits");
}
#[test]
fn sqlite_snapshot_png_complete_independent_copied_columns_admission(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let source=fixture();let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_columns:8,..SqliteDatabaseLimits::default()};assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(PngSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"preflight copied columns {encoding:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"encoder copied columns {encoding:?}");assert!(PngSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"decoder copied columns {encoding:?}");}
}
