use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};

fn fixture() -> SemioTextSnapshot {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    SemioTextSnapshot { schema: fixture["schema"].as_str().unwrap().into(), runs: fixture["runs"].as_array().unwrap().iter().map(|run| SemioTextRun { language: run["language"].as_str().unwrap().into(), content: run["content"].as_str().unwrap().into(), marks: run["marks"].as_array().unwrap().iter().map(|mark| SemioTextMark { kind: match mark["kind"].as_str().unwrap() { "bold" => SemioTextMarkKind::Bold, "italic" => SemioTextMarkKind::Italic, "code" => SemioTextMarkKind::Code, _ => SemioTextMarkKind::Link }, href: mark["href"].as_str().unwrap().into() }).collect() }).collect() }
}
fn project(snapshot: &SemioTextSnapshot) -> SqliteDatabase { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }
fn restore(database: &SqliteDatabase) -> Result<SemioTextSnapshot, String> { SemioTextSnapshot::from_sqlite_database(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) }

#[test]
fn sqlite_snapshot_semio_text_native_fields_and_ordinals_roundtrip() {
    let snapshot = fixture(); let database = project(&snapshot); assert_eq!(database.table("semio_text_run").unwrap().rows.len(), 3); assert_eq!(database.table("semio_text_mark").unwrap().rows.len(), 4);
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(); let restored = restore(&import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()).unwrap(); assert_eq!(restored, snapshot);
    assert_eq!(<SemioTextSnapshot as store::ArtifactPack>::encode_pack(&restored), <SemioTextSnapshot as store::ArtifactPack>::encode_pack(&snapshot)); assert_eq!(<SemioTextSnapshot as store::ArtifactDsl>::print_dsl(&restored), <SemioTextSnapshot as store::ArtifactDsl>::print_dsl(&snapshot));
    assert_eq!(restore(&project(&SemioTextSnapshot::default())).unwrap(), SemioTextSnapshot::default());
}

#[test]
fn sqlite_snapshot_semio_text_rejects_invalid_owners_order_kinds_and_budgets() {
    let original = project(&fixture());
    for alteration in 0..4 { let mut database = original.clone(); match alteration { 0 => database.table_mut("semio_text_mark").unwrap().rows[0].values[1] = SqliteValue::Integer(999), 1 => database.table_mut("semio_text_mark").unwrap().rows[0].values[2] = SqliteValue::Integer(99), 2 => database.table_mut("semio_text_mark").unwrap().rows[0].values[3] = SqliteValue::Text("unknown".into()), _ => database.table_mut("semio_text_run").unwrap().rows[0].values[1] = SqliteValue::Integer(2) } assert!(restore(&database).is_err()); }
    let limits = SqliteDatabaseLimits { max_rows: 1, max_value_bytes: 1, ..SqliteDatabaseLimits::default() }; assert!(fixture().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err()); assert!(SemioTextSnapshot::from_sqlite_database(&original, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err()); assert!(fixture().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_semio_text_independent_join_query_and_edit() {
    use std::io::Write; use std::process::{Command,Stdio};
    let snapshot=fixture();let bytes=export_sqlite_database(&project(&snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const r=db.query('SELECT language,kind,href FROM semio_text_run JOIN semio_text_mark ON run_id=semio_text_run.id ORDER BY semio_text_run.ordinal,semio_text_mark.ordinal').all();if(r.length!==4||r[0].language!=='en'||r[3].kind!=='link')throw Error('run/mark relationships');db.query('UPDATE semio_text_run SET content=? WHERE ordinal=0').run('independently edited');await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let restored=restore(&import_sqlite_database(&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();let mut expected=snapshot;expected.runs[0].content="independently edited".into();assert_eq!(restored,expected);
}

#[test]
fn sqlite_snapshot_semio_text_bare_owner_capability_native_payload_bridge(){let snapshot=fixture();let payload=store::os_io::IoPayload::Binary(<SemioTextSnapshot as store::ArtifactPack>::encode_pack(&snapshot));let codec=store::ArtifactCodec::bare::<SemioTextSnapshot,crate::standards::v1::subsets::text::schema::mutations::SemioTextMutation>("s.stdio.semio.text");let provider=codec.snapshot_sqlite.expect("owner must publish typed SQLite capability");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<SemioTextSnapshot>()));let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"text".into()};let db=(provider.export)("s.stdio.semio.text",&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let db=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let actual=(provider.import)("s.stdio.semio.text",&dialect,db,SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(actual,payload);}

#[test]
fn sqlite_snapshot_semio_text_typed_exact_subset_dialect_and_document_guard(){let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let snapshot=fixture();let database=project(&snapshot);let dialect=|v:&serde_json::Value|store::os_io::ArtifactDialect{artifact_kind:v["artifactKind"].as_str().unwrap().into(),standard:v["standard"].as_str().unwrap().into(),subset:v["subset"].as_str().unwrap().into()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(&f["sqliteDialect"]),&database,&mut control).unwrap().diagnostics.is_empty());for invalid in f["invalidSqliteDialects"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(invalid),&database,&mut control).is_err());}let mut malformed=database;malformed.table_mut("semio_text_document").unwrap().rows[0].values[1]=SqliteValue::Text("other schema".into());assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(&f["sqliteDialect"]),&malformed,&mut control).is_err());}

#[test]
fn sqlite_snapshot_semio_text_bounded_native_text_binary_admission() {
    let snapshot=fixture();let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"text".into()};
    let provider=<SemioTextSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
        snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let actual=(provider.import)("Semio native admission",&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
        let expected=match encoding{SnapshotEncoding::Text=>store::os_io::IoPayload::Text(<SemioTextSnapshot as store::ArtifactDsl>::print_dsl(&snapshot)),SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<SemioTextSnapshot as store::ArtifactPack>::encode_pack(&snapshot))};assert_eq!(actual,expected);
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:16,..SqliteDatabaseLimits::default()})).is_err());
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
    }
    let large= SemioTextSnapshot{schema:"x".repeat(100000),..snapshot};
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:4096,..SqliteDatabaseLimits::default()})).is_err());
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |p|p.phase!=SqliteSnapshotPhase::EncodeNative||p.completed==0,SqliteDatabaseLimits::default())).is_err());
}
