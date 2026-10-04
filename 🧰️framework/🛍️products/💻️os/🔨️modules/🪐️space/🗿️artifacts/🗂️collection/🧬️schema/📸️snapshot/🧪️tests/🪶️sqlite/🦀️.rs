//! 🧪️ Actual builtin native factories and erased semantic snapshot boundaries.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_collection_builtin_creation_and_reload_publish_io_without_manual_registration() {
    const MODE: &str = "SEMIO_SQLITE_COLLECTION_BUILTIN_MODE";
    let Ok(mode) = std::env::var(MODE) else {
        for mode in ["create", "reload", "retained"] {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .arg("--exact")
                .arg(format!("{}::sqlite_snapshot_framework_collection_builtin_creation_and_reload_publish_io_without_manual_registration",module_path!().split_once("::").expect("owning test module includes crate").1))
                .arg("--nocapture")
                .env(MODE, mode).output().unwrap();
            assert!(String::from_utf8_lossy(&output.stdout).contains("running 1 test"), "isolated builtin selector must execute its actual test: {}\n{}",String::from_utf8_lossy(&output.stdout),String::from_utf8_lossy(&output.stderr));
            assert!(output.status.success(), "isolated {mode} builtin I/O failed: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }
        return;
    };
    let source = fixture();
    let mut envelope = store::create_document_envelope::<CollectionSnapshot, CollectionMutation>(S_COLLECTION_SCHEMA, "sqlite-builtin", source.clone(), None);
    if mode == "retained" { envelope.dialect = Some(dialect()); }
    let mut live = if mode == "retained" {
        let saved = store::print_document_pack(&envelope).await.unwrap();
        envelope.retire_unadopted();
        let history = store::os_spr::decode_history(&saved.spr, &store::os_spr::DecodeOptions::default()).await.unwrap();
        let mut open = store::RetainedPersistedDocumentHydration::<CollectionSnapshot, CollectionMutation>::from_initial(
            source.clone(), history,
            store::os_io::ArtifactRef { artifact_id: "sqlite-builtin".into(), dialect: dialect() }, None, S_COLLECTION_SCHEMA.into(),
            store::test_support::plain_document_store_owners(), semio_framework_job::OperationId(1), semio_framework_job::Generation(1),
            u64::MAX, store::PersistedDocumentHydrationTarget::Store { generation: 0 },
        );
        let cancellation = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut ready = None;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(1), semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(256, u64::MAX), cancellation.clone(),
                semio_framework_job::default_now_us, &mut preview_sequence,
            );
            match open.step(&mut context) {
                store::PersistedDocumentHydrationStep::Pending(_) => {},
                store::PersistedDocumentHydrationStep::Ready(store::PersistedDocumentHydrationOutput::Store(member)) => { ready = Some(*member); break; },
                _ => panic!("valid builtin retained history must hand off its exact store"),
            }
        }
        ready.expect("bounded retained opening must finish")
    } else {
        let envelope = if mode == "reload" {
        let saved = store::print_document_pack(&envelope).await.unwrap();
        envelope.retire_unadopted();
        store::parse_document_pack::<CollectionSnapshot, CollectionMutation>(&saved.pack, &saved.spr).await.unwrap().into_envelope()
        } else { envelope };
        let mut live = store::ArtifactStore::new(envelope).await.unwrap();
        live.install_document_store_owners_exact(store::test_support::plain_document_store_owners());
        live
    };
    assert_eq!(live.envelope().dialect, if mode == "retained" { Some(dialect()) } else { None });
    let output = store::os_io::io_mechanism::io_export_sqlite_snapshot(&dialect(), live.snapshot_ref(), SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |_|true).await;
    store::test_support::close_plain_test_store(&mut live);
    let bytes = output.expect("actual builtin construction publishes the authored SQLite coordinate atomically").value;
    assert_eq!(store::os_io::io_mechanism::io_import_sqlite_snapshot::<CollectionSnapshot>(&dialect(), &bytes, SqliteDatabaseLimits::default(), &mut |_|true).await.unwrap().value, source);
}

#[test]fn sqlite_snapshot_framework_collection_same_control_retained_materialization_budget_is_cumulative(){let f=f();let case=&f["reconstructionBudget"];let mut source=fixture();source.name=case["textUnit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap()as usize);let bytes=source.name.len();let database=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::to_sqlite_database(&source,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_value_bytes:bytes*case["ceilingNumerator"].as_u64().unwrap()as usize/case["ceilingDenominator"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()};let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);let retained=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut control).unwrap();assert_eq!(retained.name,source.name);assert!(<CollectionSnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database,&mut control).is_err(),"second live materialization must not reset the caller ownership ledger");}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_collection_actual_typed_io_file_metadata_and_route(){crate::register_sqlite_snapshot().unwrap();let source=fixture();let limits=SqliteDatabaseLimits::default();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=store::os_io::io_mechanism::io_export_sqlite_snapshot(&dialect(),&source,encoding,limits,&mut |_|true).await.unwrap().value;let database=import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();assert_eq!(store::os_io::io_mechanism::sqlite_snapshot_metadata(&database).unwrap(),(dialect(),encoding));assert_eq!(store::os_io::io_mechanism::io_import_sqlite_snapshot::<CollectionSnapshot>(&dialect(),&bytes,limits,&mut |_|true).await.unwrap().value,source);}let route=store::os_io::io_mechanism::io_route(&dialect(),&store::os_io::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT),1).await.unwrap().value;assert_eq!(route.hops.len(),1);}
#[test]fn sqlite_snapshot_framework_collection_independent_surrogate_renumber_preserves_complete_logical_state(){let source=fixture();let codec=codec();let changed=oracle::renumber(&database(),f()["identityOffset"].as_i64().unwrap());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),changed.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(decode(native),source);}}
use crate::*;
use semio_framework_os_kernel as store;
use store::sqlite_snapshot::*;
#[path="../../../../../../🧪️tests/🪶️sqlite/🔬️oracle/🦀️.rs"]mod oracle;
const SQL:&str=include_str!("../../🪶️sqlite/🗄️.sql");
fn f()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn fixture()->CollectionSnapshot{{let f=f();let mut value:CollectionSnapshot=serde_json::from_value(f["nativeSnapshot"].clone()).unwrap();let mut sizes=f["blobSizes"].as_array().unwrap().iter();for entry in&mut value.entries{if let ArtifactBody::Blob{blob}=entry.body.as_mut(){blob.size=sizes.next().unwrap().as_str().unwrap().parse().unwrap();}}value}}
fn database()->SqliteDatabase{oracle::database(&f(),SQL)}
fn dialect()->store::os_io::ArtifactDialect{store::os_io::ArtifactDialect{artifact_kind:S_COLLECTION_SCHEMA.into(),standard:"1".into(),subset:"*".into()}}
fn codec()->store::ArtifactSqliteSnapshotCodec{store::ArtifactCodec::bare::<CollectionSnapshot,CollectionMutation>(S_COLLECTION_SCHEMA).snapshot_sqlite.expect("actual builtin native factory requires its owned semantic SQLite capability")}
fn payload(value:&CollectionSnapshot,encoding:SnapshotEncoding)->store::os_io::IoPayload{match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(value)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(value))}}
fn decode(value:store::os_io::IoPayload)->CollectionSnapshot{match value{store::os_io::IoPayload::Binary(bytes)=>store::ArtifactPack::decode_pack(&bytes).unwrap(),store::os_io::IoPayload::Text(text)=>store::ArtifactDsl::parse_dsl(&text).unwrap()}}
#[test]fn sqlite_snapshot_framework_collection_actual_factory_capability(){let codec=codec();assert_eq!(codec.snapshot_type,Some(std::any::TypeId::of::<CollectionSnapshot>()));}
#[test]fn sqlite_snapshot_framework_collection_ordinary_native_complete_state(){let source=fixture();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{assert_eq!(decode(payload(&source,encoding)),source);}}
#[test]fn sqlite_snapshot_framework_collection_erased_both_directions_preserve_all_semantic_cells(){let source=fixture();let codec=codec();let limits=SqliteDatabaseLimits::default();let expected=database();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);assert_eq!((codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,expected);let native=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert_eq!(decode(native),source);}}
#[test]fn sqlite_snapshot_framework_collection_independent_sql_edit_retains_literal_typed_state(){let codec=codec();let edited=oracle::edit(&database(),&f());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),edited.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let value=decode(native);assert!(matches!(value.entries[0].body.as_ref(),ArtifactBody::Document{document_id,..}if document_id==f()["edit"]["value"].as_str().unwrap()));}}
#[test]fn sqlite_snapshot_framework_collection_authored_schema_and_exact_semantic_row_admission(){let codec=codec();let source=fixture();let expected=database();let rows=f()["rowsTotal"].as_u64().unwrap()as usize;assert_eq!(expected.tables.iter().map(|table|table.rows.len()).sum::<usize>(),rows);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);for maximum in[rows-1,rows]{let limits=SqliteDatabaseLimits{max_rows:maximum,..SqliteDatabaseLimits::default()};let output=(codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |_|true,limits));let input=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(output.is_ok(),maximum==rows);assert_eq!(input.is_ok(),maximum==rows);}let limits=SqliteDatabaseLimits{max_schema_bytes:SQL.len()-1,..SqliteDatabaseLimits::default()};assert!((codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!((codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}}
#[test]fn sqlite_snapshot_framework_collection_all_owned_phases_cancel_and_wrong_dialect_refuses(){let codec=codec();let source=fixture();let expected=database();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let native=payload(&source,encoding);for phase in[SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::ProjectSnapshot]{let mut seen=false;let result=(codec.export)(S_COLLECTION_SCHEMA,&dialect(),&native,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==phase{seen=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(result.is_err()&&seen);}for phase in[SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative]{let mut seen=false;let result=(codec.import)(S_COLLECTION_SCHEMA,&dialect(),expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |p|{if p.phase==phase{seen=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(result.is_err()&&seen);}let wrong=store::os_io::ArtifactDialect{subset:"invented".into(),..dialect()};assert!((codec.export)(S_COLLECTION_SCHEMA,&wrong,&native,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}}

#[test]
fn sqlite_snapshot_framework_collection_native_input_retained_materialization_uses_same_caller_ledger(){
 let corpus=f();let sample=&corpus["reconstructionBudget"];let ratio=&corpus["nativeRetention"];let mut source=fixture();source.name=sample["textUnit"].as_str().unwrap().repeat(sample["repeat"].as_u64().unwrap()as usize);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=payload(&source,encoding);let limits=SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);let first=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut control).unwrap();assert_eq!(first,source);let charged=limits.max_value_bytes-control.reconstruction_remaining_bytes().unwrap();assert!(charged>0,"native parser and retained typed fields must publish their actual cumulative ownership");
  let ceiling=charged.checked_mul(ratio["ceilingNumerator"].as_u64().unwrap()as usize).unwrap()/ratio["ceilingDenominator"].as_u64().unwrap()as usize;let limits=SqliteDatabaseLimits{max_value_bytes:ceiling,..limits};let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);let retained=<CollectionSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut control).unwrap();assert_eq!(retained,source);assert!(<CollectionSnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut control).is_err(),"second retained native input must not reset the same caller ledger");
 }
}

#[path="../../../../../../🧪️tests/🪶️sqlite/📏️preflight/🦀️.rs"]
mod public_preflight;
#[test]
fn sqlite_snapshot_framework_collection_public_borrowed_native_preflight_admits_actual_owner(){
 let facet:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️preflight/🔣️.json")).unwrap();
 let mut short=fixture();short.name=facet["textUnit"].as_str().unwrap().into();
 let mut long=short.clone();long.name=facet["textUnit"].as_str().unwrap().repeat(usize::try_from(facet["repeat"].as_u64().unwrap()).unwrap());
 public_preflight::verify(&short,&long,SQL,usize::try_from(facet["rows"].as_u64().unwrap()).unwrap(),long.name.len(),usize::try_from(facet["cancelAt"].as_u64().unwrap()).unwrap(),|operation|crate::test_allocation::observe(operation));
}
