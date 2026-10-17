//! 🧪️ Unit tests of the PDF 1.4 sqlite snapshot backing, limits and erased round trips.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
use semio_framework_test_fixtures::fixture_text;
use store::sqlite_snapshot::{self, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase};
#[allow(unused_imports)]
use semio_framework_value::{ValueError, ValueRefusalKind};
#[allow(unused_imports)]
use store::sqlite_snapshot::{SqliteDatabase as Db, artifact::Cell as C, artifact::Projection, artifact::FloatColumn, artifact::FloatRow, artifact::insert_ieee754};
#[path = "../💰️backing/🦀️.rs"]
mod backing;
#[test]
fn sqlite_snapshot_pdf14_full_concrete_backing_preserves_owned_pages_and_caller_limits() {
    let cases: serde_json::Value = serde_json::from_str(fixture_text!("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔢ieee.json")).unwrap();
    assert_eq!(cases["backing"]["authority"], "completeSystemAllocatorRequests");
    let mut snapshot = fixture();
    snapshot.schema = "literal PDF 世界\0".into();
    for (index, case) in cases["binary64"].as_array().unwrap().iter().enumerate() {
        let value = f64::from_bits(u64::from_str_radix(case["bits"].as_str().unwrap(), 16).unwrap());
        snapshot.pages.push(PageDoc { width: value, height: value, text: format!("page-{index}\0😀") });
    }
    snapshot.pages[0].text = "x".repeat(131_072);
    backing::verify(&snapshot);
}

use crate::standards::v1_4::subsets::base::io::sqlite::snapshot::*;
use store::os_store::serialization::ArtifactSqliteSnapshotHost;
use semio_framework_value::FromValue;
fn fixture()->PdfSnapshot{PdfSnapshot::from_value(serde_json::from_str(fixture_text!("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()).unwrap()}
async fn erased_roundtrip(snapshot: &PdfSnapshot, encoding: sqlite_snapshot::SnapshotEncoding) -> PdfSnapshot {
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::IoPayload,semio_framework_os_kernel::io::io_mechanism::io_route};
    crate::root_tests::register_sqlite_test_declaration();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.pdf".into(), standard: "1.4".into(), subset: "*".into() };
    let sqlite = semio_framework_os_kernel::io_schema::SQLITE_SNAPSHOT.into();
    let payload = match encoding { sqlite_snapshot::SnapshotEncoding::Binary => IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(snapshot)), sqlite_snapshot::SnapshotEncoding::Text => IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(snapshot)) };
    let bytes = semio_s_artifact_stdio_contract::testing::io_run(&io_route(&dialect, &sqlite, 1).await.unwrap().value, payload, Default::default(), &mut |_| true).await.unwrap().value;
    let payload = semio_s_artifact_stdio_contract::testing::io_run(&io_route(&sqlite, &dialect, 1).await.unwrap().value, bytes, Default::default(), &mut |_| true).await.unwrap().value;
    match payload { IoPayload::Binary(bytes) => pack::snapshot::ArtifactPack::decode_pack(&bytes).unwrap(), IoPayload::Text(text) => pack::snapshot::ArtifactDsl::parse_dsl(&text).unwrap() }
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_pdf14_actual_erased_binary_preserves_owned_schema_pages_and_ieee() {
    let snapshot = fixture(); assert_eq!(erased_roundtrip(&snapshot, sqlite_snapshot::SnapshotEncoding::Binary).await, snapshot);
    erased_ieee(sqlite_snapshot::SnapshotEncoding::Binary).await;
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_pdf14_actual_erased_text_preserves_owned_schema_pages_and_ieee() {
    let snapshot = fixture(); assert_eq!(erased_roundtrip(&snapshot, sqlite_snapshot::SnapshotEncoding::Text).await, snapshot);
    erased_ieee(sqlite_snapshot::SnapshotEncoding::Text).await;
}
async fn erased_ieee(encoding: sqlite_snapshot::SnapshotEncoding) {
    let cases: serde_json::Value = serde_json::from_str(fixture_text!("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔢ieee.json")).unwrap();
    let pages = cases["binary64"].as_array().unwrap().iter().map(|case| { let bits = u64::from_str_radix(case["bits"].as_str().unwrap(), 16).unwrap(); PageDoc { width: f64::from_bits(bits), height: f64::from_bits(bits), text: case["class"].as_str().unwrap().into() } }).collect();
    let snapshot = PdfSnapshot { schema: "every-owned-pdf14-word".into(), pages };
    let restored = erased_roundtrip(&snapshot, encoding).await;
    assert_eq!(restored.schema, snapshot.schema); assert_eq!(restored.pages.len(), snapshot.pages.len());
    for (expected, actual) in snapshot.pages.iter().zip(restored.pages) { assert_eq!(actual.width.to_bits(), expected.width.to_bits()); assert_eq!(actual.height.to_bits(), expected.height.to_bits()); assert_eq!(actual.text, expected.text); }
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_pdf14_owned_io_preserves_arbitrary_schema_and_text(){
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_os_kernel::io::register_native_snapshot_codec,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};let native=Dialect{artifact_kind:"s.stdio.pdf",standard:StandardId("1.4"),subset:SubsetId("*")};register_native_snapshot_codec(native,store::artifact_codec::<PdfSnapshot,crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation>(crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA)).unwrap();let dialect:ArtifactDialect=native.into();let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut phases=Vec::new();let file=async{let original_file_dialect=&dialect;let original_file_input=&snapshot;let original_file_encoding=sqlite_snapshot::SnapshotEncoding::Binary;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |event|{phases.push(event.phase);true};semio_framework_os_kernel::io::testing::with_snapshot_file_decoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;assert_eq!(async{let original_file_dialect=&dialect;let original_file_input=&file;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<PdfSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,Phase::DecodeNative|Phase::EncodeNative)));
}
#[test]
fn sqlite_snapshot_pdf14_independent_sql_page_query_and_edit(){
    use std::{io::Write,process::{Command,Stdio}};let mut snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT p.ordinal,p.width,p.height,p.text FROM pdf14_document d JOIN pdf14_page p ON p.document_id=d.id ORDER BY p.ordinal').all();if(rows.length!==3)throw Error('page query');db.query(\"UPDATE pdf14_page SET text='independent edit' WHERE ordinal=1\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let loaded=sqlite_snapshot::import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();snapshot.pages[1].text="independent edit".into();assert_eq!(PdfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
}
#[test]
fn sqlite_snapshot_pdf14_preserves_ieee_width_and_height(){
    let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔢ieee.json")).unwrap();let pages=cases["binary64"].as_array().unwrap().iter().map(|value|{let bits=u64::from_str_radix(value["bits"].as_str().unwrap(),16).unwrap();PageDoc{width:f64::from_bits(bits),height:f64::from_bits(bits),text:value["class"].as_str().unwrap().into()}}).collect();let snapshot=PdfSnapshot{schema:"every-ieee-page".into(),pages};let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();for(row,case)in db.table("pdf14_page").unwrap().rows.iter().zip(cases["binary64"].as_array().unwrap()){assert_eq!(row.text(7).unwrap(),case["class"].as_str().unwrap());assert_eq!(row.integer(6).unwrap() as u64,u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap());}let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
    use std::{io::Write,process::{Command,Stdio}};let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT CAST(width_bits AS TEXT) AS bits,width_class AS class,width IS NULL AS nullQuery FROM pdf14_page ORDER BY ordinal').all()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let independent:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();for(row,case)in independent.as_array().unwrap().iter().zip(cases["binary64"].as_array().unwrap()){assert_eq!(row["class"],case["class"]);let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();assert_eq!(row["bits"].as_str().unwrap(),(bits as i64).to_string());assert_eq!(row["nullQuery"].as_i64().unwrap(),i64::from(case["class"]=="nan"));}
    let loaded=sqlite_snapshot::import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();let restored=PdfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap();for(before,after)in snapshot.pages.iter().zip(restored.pages){assert_eq!(before.width.to_bits(),after.width.to_bits());assert_eq!(before.height.to_bits(),after.height.to_bits());}
    let mut bad=loaded;bad.table_mut("pdf14_page").unwrap().rows[0].values[7]=sqlite_snapshot::SqliteValue::Text("finite".into());assert!(PdfSnapshot::from_sqlite_database(&bad,&mut Control::new(&mut |_|true,limits)).is_err());
}
#[test]
fn sqlite_snapshot_pdf14_preserves_all_pages_and_empty_documents(){
    let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=Control::new(&mut progress,limits);
    for snapshot in [fixture(),PdfSnapshot{schema:"empty-schema".into(),pages:Vec::new()}]{
        let db=snapshot.to_sqlite_database(&mut control).unwrap();assert_eq!(db.table("pdf14_page").unwrap().rows.len(),snapshot.pages.len());
        assert_eq!(PdfSnapshot::from_sqlite_database(&db,&mut control).unwrap(),snapshot);
        let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();assert_eq!(PdfSnapshot::from_sqlite_database(&loaded,&mut control).unwrap(),snapshot);
    }
    let mut db=fixture().to_sqlite_database(&mut control).unwrap();db.table_mut("pdf14_page").unwrap().rows[0].values[5]=sqlite_snapshot::SqliteValue::Text("SQL edited".into());assert_eq!(PdfSnapshot::from_sqlite_database(&db,&mut control).unwrap().pages[0].text,"SQL edited");
}
#[test]
fn sqlite_snapshot_pdf14_rejects_bad_relations_and_bounds_work(){
    let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|false,limits)).is_err());
    let mut small=limits;small.max_rows=2;assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());small=limits;small.max_value_bytes=1;assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());
    let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();assert!(PdfSnapshot::from_sqlite_database(&db,&mut Control::new(&mut |_|false,limits)).is_err());
    for column in [1,2]{let mut bad=db.clone();bad.table_mut("pdf14_page").unwrap().rows[0].values[column]=sqlite_snapshot::SqliteValue::Integer(999);assert!(PdfSnapshot::from_sqlite_database(&bad,&mut Control::new(&mut |_|true,limits)).is_err());}
    let mut large=snapshot;large.pages=vec![large.pages[0].clone();1000];let mut calls=0;assert!(large.to_sqlite_database(&mut Control::new(&mut |_|{calls+=1;calls<4},limits)).is_err());
}
