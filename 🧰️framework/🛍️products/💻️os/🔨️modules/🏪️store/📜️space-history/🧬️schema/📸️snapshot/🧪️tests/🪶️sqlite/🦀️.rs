//! 📣️ Explicit owner registration separates generic history construction from registry I/O.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_space_history_explicit_owner_registration_after_create_reload_and_retained() {
    const CASE: &str = "SEMIO_SQLITE_HISTORY_REGISTRATION_CASE";
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📣️registration/🔣️.json")).unwrap();
    let Ok(case) = std::env::var(CASE) else {
        for sample in corpus["cases"].as_array().unwrap() {
            let id = sample["id"].as_str().unwrap();
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .arg("--exact")
                .arg(format!("{}::sqlite_snapshot_framework_space_history_explicit_owner_registration_after_create_reload_and_retained", module_path!().split_once("::").expect("owning test module includes crate").1))
                .arg("--nocapture")
                .env(CASE, id)
                .output()
                .unwrap();
            assert!(String::from_utf8_lossy(&output.stdout).contains("running 1 test"), "isolated owner-registration selector must execute its actual test: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
            assert!(output.status.success(), "isolated {id} explicit owner registration failed: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }
        return;
    };
    let sample = corpus["cases"].as_array().unwrap().iter().find(|sample| sample["id"].as_str() == Some(case.as_str())).expect("exact isolated corpus case");
    let mode = sample["opening"].as_str().unwrap();
    let encoding = match sample["encoding"].as_str().unwrap() {
        "binary" => SnapshotEncoding::Binary,
        "text" => SnapshotEncoding::Text,
        _ => panic!("closed encoding corpus"),
    };
    let source = fixture();
    let mut envelope = store::create_document_envelope::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA, "sqlite-history-registration", source.clone(), None);
    if mode == "retained" {
        envelope.dialect = Some(dialect());
    }
    let mut live = if mode == "retained" {
        let saved = store::print_document_pack(&envelope).await.unwrap();
        envelope.retire_unadopted();
        let history = store::os_spr::decode_history(&saved.spr, &store::os_spr::DecodeOptions::default()).await.unwrap();
        let mut open = store::RetainedPersistedDocumentHydration::<SpaceHistorySnapshot, SpaceHistoryMutation>::from_initial(
            source.clone(),
            history,
            store::os_io::ArtifactRef { artifact_id: "sqlite-history-registration".into(), dialect: dialect() },
            None,
            S_SPACE_HISTORY_SCHEMA.into(),
            store::test_support::plain_document_store_owners(),
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            u64::MAX,
            store::PersistedDocumentHydrationTarget::Store { generation: 0 },
        );
        let cancellation = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut ready = None;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(1),
                semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(256, u64::MAX),
                cancellation.clone(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            match open.step(&mut context) {
                store::PersistedDocumentHydrationStep::Pending(_) => {}
                store::PersistedDocumentHydrationStep::Ready(store::PersistedDocumentHydrationOutput::Store(member)) => {
                    ready = Some(*member);
                    break;
                }
                _ => panic!("valid retained history must hand off its exact store"),
            }
        }
        ready.expect("bounded retained opening must finish")
    } else {
        let envelope = if mode == "reload" {
            let saved = store::print_document_pack(&envelope).await.unwrap();
            envelope.retire_unadopted();
            store::parse_document_pack::<SpaceHistorySnapshot, SpaceHistoryMutation>(&saved.pack, &saved.spr).await.unwrap().into_envelope()
        } else {
            envelope
        };
        let mut live = store::ArtifactStore::new(envelope).await.unwrap();
        live.install_document_store_owners_exact(store::test_support::plain_document_store_owners());
        live
    };
    let expected_envelope_dialect = if sample["envelopeDialect"].is_null() { None } else { Some(dialect()) };
    assert_eq!(live.envelope().dialect, expected_envelope_dialect);
    assert_eq!(*live.snapshot_ref(), source);
    assert_eq!(dialect().to_coordinate(), corpus["coordinate"].as_str().unwrap());
    assert_eq!(store::os_io::ArtifactDialect::from(store::space_history_sqlite::SQLITE_SNAPSHOT_DIALECT), dialect());
    let limits = SqliteDatabaseLimits::default();
    let mut independent = database();
    store::os_io::io_mechanism::attach_sqlite_snapshot_metadata(&mut independent, &dialect(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let independent_bytes = export_sqlite_database(&independent, limits, &mut |_| true).unwrap();
    let export_refusal = store::os_io::io_mechanism::io_export_sqlite_snapshot(&dialect(), live.snapshot_ref(), encoding, limits, &mut |_| true).await.unwrap_err();
    assert_eq!(export_refusal.cause.kind, semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    assert!(export_refusal.diagnostics.is_empty());
    let import_refusal = store::os_io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &independent_bytes, limits, &mut |_| true).await.unwrap_err();
    assert_eq!(import_refusal.cause.kind, semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    assert!(import_refusal.diagnostics.is_empty());
    assert_eq!(live.envelope().dialect, expected_envelope_dialect);
    assert_eq!(*live.snapshot_ref(), source);
    store::space_history_sqlite::register_sqlite_snapshot().unwrap();
    let output = store::os_io::io_mechanism::io_export_sqlite_snapshot(&dialect(), live.snapshot_ref(), encoding, limits, &mut |_| true).await.unwrap();
    assert!(output.diagnostics.is_empty());
    let bytes = output.value;
    let relational = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
    assert_eq!(relational, independent);
    assert_eq!(store::os_io::io_mechanism::sqlite_snapshot_metadata(&relational).unwrap(), (dialect(), encoding));
    let imported = store::os_io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &bytes, limits, &mut |_| true).await.unwrap();
    assert!(imported.diagnostics.is_empty());
    assert_eq!(imported.value, source);
    assert_eq!(store::os_io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &independent_bytes, limits, &mut |_| true).await.unwrap().value, source);
    let route = store::os_io::io_mechanism::io_route(&dialect(), &store::os_io::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT), 1).await.unwrap().value;
    assert_eq!(route.hops.len(), sample["routeHops"].as_u64().unwrap() as usize);
    assert_eq!(route.hops[0].from, dialect());
    assert_eq!(route.hops[0].into, store::os_io::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT));
    assert_eq!(live.envelope().dialect, expected_envelope_dialect);
    assert_eq!(*live.snapshot_ref(), source);
    store::test_support::close_plain_test_store(&mut live);
    println!("[DEBUG] explicit SpaceHistory owner registration {case}: typed UnsupportedOwner before publication; complete state and exact SQLite metadata/route retained after publication");
}
#[test]
fn sqlite_snapshot_framework_space_history_same_control_retained_materialization_budget_is_cumulative() {
    let f = f();
    let case = &f["interiorControl"];
    let mut source = fixture();
    source.checkpoints[0].message = case["textUnit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap() as usize);
    let bytes = source.checkpoints[0].message.len();
    let database = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::to_sqlite_database(&source, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: bytes * case["ceilingNumerator"].as_u64().unwrap() as usize / case["ceilingDenominator"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
    let mut progress = |_| true;
    let mut control = SqliteSnapshotControl::new(&mut progress, limits);
    let retained = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database, &mut control).unwrap();
    assert_eq!(retained, source);
    assert!(<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::from_sqlite_database(&database, &mut control).is_err(), "second live materialization must not reset the caller ownership ledger");
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_space_history_actual_typed_io_file_metadata_and_route() {
    store::space_history_sqlite::register_sqlite_snapshot().unwrap();
    let source = fixture();
    let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let bytes = store::os_io::io_mechanism::io_export_sqlite_snapshot(&dialect(), &source, encoding, limits, &mut |_| true).await.unwrap().value;
        let database = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
        assert_eq!(store::os_io::io_mechanism::sqlite_snapshot_metadata(&database).unwrap(), (dialect(), encoding));
        assert_eq!(store::os_io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &bytes, limits, &mut |_| true).await.unwrap().value, source);
    }
    let route = store::os_io::io_mechanism::io_route(&dialect(), &store::os_io::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT), 1).await.unwrap().value;
    assert_eq!(route.hops.len(), 1);
}
#[test]
fn sqlite_snapshot_framework_space_history_independent_surrogate_renumber_preserves_complete_logical_state() {
    let source = fixture();
    let codec = codec();
    let changed = oracle::renumber(&database(), f()["identityOffset"].as_i64().unwrap());
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = (codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), changed.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        assert_eq!(decode(native), source);
    }
}
use super::{S_SPACE_HISTORY_SCHEMA, SpaceHistoryMutation, SpaceHistorySnapshot};
use crate as store;
use store::sqlite_snapshot::*;
#[path = "../../../../../../🪐️space/🧪️tests/🪶️sqlite/🔬️oracle/🦀️.rs"]
mod oracle;
const SQL: &str = include_str!("../../🪶️sqlite/🗄️.sql");
fn f() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()
}
fn fixture() -> SpaceHistorySnapshot {
    {
        let f = f();
        let mut value: SpaceHistorySnapshot = semio_framework_pack_json::from_json_str(&f["nativeSnapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for (checkpoint, clock) in value.checkpoints.iter_mut().zip(f["clocks"].as_array().unwrap()) {
            checkpoint.timestamp.actor = clock["actor"].as_str().unwrap().parse().unwrap();
            checkpoint.timestamp.physical_ms = clock["physicalMs"].as_str().unwrap().parse().unwrap();
            checkpoint.timestamp.logical = clock["logical"].as_str().unwrap().parse().unwrap();
        }
        value
    }
}
fn database() -> SqliteDatabase {
    oracle::database(&f(), SQL)
}
fn dialect() -> store::os_io::ArtifactDialect {
    store::os_io::ArtifactDialect { artifact_kind: S_SPACE_HISTORY_SCHEMA.into(), standard: "1".into(), subset: "*".into() }
}
fn codec() -> store::ArtifactSqliteSnapshotCodec {
    store::ArtifactCodec::bare::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA).snapshot_sqlite.expect("actual builtin native factory requires its owned semantic SQLite capability")
}
fn payload(value: &SpaceHistorySnapshot, encoding: SnapshotEncoding) -> store::os_io::IoPayload {
    match encoding {
        SnapshotEncoding::Binary => store::os_io::IoPayload::Binary(store::ArtifactPack::encode_pack(value)),
        SnapshotEncoding::Text => store::os_io::IoPayload::Text(store::ArtifactDsl::print_dsl(value)),
    }
}
fn decode(value: store::os_io::IoPayload) -> SpaceHistorySnapshot {
    match value {
        store::os_io::IoPayload::Binary(bytes) => store::ArtifactPack::decode_pack(&bytes).unwrap(),
        store::os_io::IoPayload::Text(text) => store::ArtifactDsl::parse_dsl(&text).unwrap(),
    }
}
#[test]
fn sqlite_snapshot_framework_space_history_known_unicode_native_frontiers_cancel_inside_both_encodings() {
    let mut f = f();
    let specimen = &f["interiorControl"];
    let text = specimen["textUnit"].as_str().unwrap().repeat(specimen["repeat"].as_u64().unwrap() as usize);
    assert_eq!(text.len(), specimen["utf8Bytes"].as_u64().unwrap() as usize);
    let mut source = fixture();
    for checkpoint in &mut source.checkpoints {
        checkpoint.message = text.clone();
    }
    f["edit"]["value"] = serde_json::Value::String(text);
    let expected = oracle::edit(&database(), &f);
    let codec = codec();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = payload(&source, encoding);
        let mut decoded_interior = false;
        let export = (codec.export)(
            S_SPACE_HISTORY_SCHEMA,
            &dialect(),
            &native,
            &mut SqliteSnapshotControl::new(
                &mut |event| {
                    if event.phase == SqliteSnapshotPhase::DecodeNative && event.completed > 0 && event.completed < event.total {
                        decoded_interior = true;
                        false
                    } else {
                        true
                    }
                },
                SqliteDatabaseLimits::default(),
            ),
        );
        assert!(decoded_interior && export.is_err(), "native input must expose known interior Unicode work before typed construction");
        let mut encoded_interior = false;
        let import = (codec.import)(
            S_SPACE_HISTORY_SCHEMA,
            &dialect(),
            expected.clone(),
            encoding,
            &mut SqliteSnapshotControl::new(
                &mut |event| {
                    if event.phase == SqliteSnapshotPhase::EncodeNative && event.completed > 0 && event.completed < event.total {
                        encoded_interior = true;
                        false
                    } else {
                        true
                    }
                },
                SqliteDatabaseLimits::default(),
            ),
        );
        assert!(encoded_interior && import.is_err(), "native output must expose known interior Unicode work before final emission");
    }
}
#[test]
fn sqlite_snapshot_framework_space_history_actual_factory_capability() {
    let codec = codec();
    assert_eq!(codec.snapshot_type, Some(std::any::TypeId::of::<SpaceHistorySnapshot>()));
}
#[test]
fn sqlite_snapshot_framework_space_history_ordinary_native_complete_state() {
    let source = fixture();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        assert_eq!(decode(payload(&source, encoding)), source);
    }
}
#[test]
fn sqlite_snapshot_framework_space_history_erased_both_directions_preserve_all_semantic_cells() {
    let source = fixture();
    let codec = codec();
    let limits = SqliteDatabaseLimits::default();
    let expected = database();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = payload(&source, encoding);
        assert_eq!((codec.export)(S_SPACE_HISTORY_SCHEMA, &dialect(), &native, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value, expected);
        let native = (codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), expected.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap().value;
        assert_eq!(decode(native), source);
    }
}
#[test]
fn sqlite_snapshot_framework_space_history_independent_sql_edit_retains_literal_typed_state() {
    let codec = codec();
    let edited = oracle::edit(&database(), &f());
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = (codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), edited.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let value = decode(native);
        assert!(value.checkpoints.iter().all(|row| row.message == f()["edit"]["value"].as_str().unwrap()));
    }
}
#[test]
fn sqlite_snapshot_framework_space_history_authored_schema_and_exact_semantic_row_admission() {
    let codec = codec();
    let source = fixture();
    let expected = database();
    let rows = f()["rowsTotal"].as_u64().unwrap() as usize;
    assert_eq!(expected.tables.iter().map(|table| table.rows.len()).sum::<usize>(), rows);
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = payload(&source, encoding);
        for maximum in [rows - 1, rows] {
            let limits = SqliteDatabaseLimits { max_rows: maximum, ..SqliteDatabaseLimits::default() };
            let output = (codec.export)(S_SPACE_HISTORY_SCHEMA, &dialect(), &native, &mut SqliteSnapshotControl::new(&mut |_| true, limits));
            let input = (codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), expected.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits));
            assert_eq!(output.is_ok(), maximum == rows);
            assert_eq!(input.is_ok(), maximum == rows);
        }
        let limits = SqliteDatabaseLimits { max_schema_bytes: SQL.len() - 1, ..SqliteDatabaseLimits::default() };
        assert!((codec.export)(S_SPACE_HISTORY_SCHEMA, &dialect(), &native, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        assert!((codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), expected.clone(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    }
}
#[test]
fn sqlite_snapshot_framework_space_history_all_owned_phases_cancel_and_wrong_dialect_refuses() {
    let codec = codec();
    let source = fixture();
    let expected = database();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = payload(&source, encoding);
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::ProjectSnapshot] {
            let mut seen = false;
            let result = (codec.export)(
                S_SPACE_HISTORY_SCHEMA,
                &dialect(),
                &native,
                &mut SqliteSnapshotControl::new(
                    &mut |p| {
                        if p.phase == phase {
                            seen = true;
                            false
                        } else {
                            true
                        }
                    },
                    SqliteDatabaseLimits::default(),
                ),
            );
            assert!(result.is_err() && seen);
        }
        for phase in [SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::EncodeNative] {
            let mut seen = false;
            let result = (codec.import)(
                S_SPACE_HISTORY_SCHEMA,
                &dialect(),
                expected.clone(),
                encoding,
                &mut SqliteSnapshotControl::new(
                    &mut |p| {
                        if p.phase == phase {
                            seen = true;
                            false
                        } else {
                            true
                        }
                    },
                    SqliteDatabaseLimits::default(),
                ),
            );
            assert!(result.is_err() && seen);
        }
        let wrong = store::os_io::ArtifactDialect { subset: "invented".into(), ..dialect() };
        assert!((codec.export)(S_SPACE_HISTORY_SCHEMA, &wrong, &native, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
}

#[test]
fn sqlite_snapshot_framework_space_history_native_input_retained_materialization_uses_same_caller_ledger() {
    let corpus = f();
    let sample = &corpus["interiorControl"];
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json")).unwrap();
    let ratio = &contract["cumulative"];
    let mut source = fixture();
    source.checkpoints[0].message = sample["textUnit"].as_str().unwrap().repeat(sample["repeat"].as_u64().unwrap() as usize);
    let charged = |control: &SqliteSnapshotControl<'_>, limits: SqliteDatabaseLimits| {
        let semantic = limits.max_value_bytes - control.reconstruction_remaining_bytes().unwrap();
        let allocation = limits.max_allocation_bytes - control.allocation_remaining_bytes();
        assert_eq!(semantic, allocation, "one native producer must settle its admitted ownership into both caller ledgers");
        assert!(semantic > 0, "native parser and retained typed fields must publish their actual cumulative ownership");
        semantic
    };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = payload(&source, encoding);
        let limits = SqliteDatabaseLimits { max_allocation_bytes: contract["zeroPhysical"]["maximumBytes"].as_u64().unwrap() as usize, ..Default::default() };
        let mut progress = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control) else { panic!("zero physical allowance must refuse before native construction") };
        assert_eq!(error.kind.as_str(), contract["zeroPhysical"]["expectedKind"].as_str().unwrap());
        assert_eq!(limits.max_value_bytes - control.reconstruction_remaining_bytes().unwrap(), contract["zeroPhysical"]["expectedCharge"].as_u64().unwrap() as usize);
        assert_eq!(control.allocation_remaining_bytes(), 0);
        let limits = SqliteDatabaseLimits::default();
        let mut progress = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let first = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control).unwrap();
        assert_eq!(first, source);
        let owned = charged(&control, limits);
        let ceiling = owned.checked_mul(ratio["ceilingNumerator"].as_u64().unwrap() as usize).unwrap() / ratio["ceilingDenominator"].as_u64().unwrap() as usize;
        for physical in [false, true] {
            let limits = if physical { SqliteDatabaseLimits { max_allocation_bytes: ceiling, ..Default::default() } } else { SqliteDatabaseLimits { max_value_bytes: ceiling, ..Default::default() } };
            let mut progress = |_| true;
            let mut control = SqliteSnapshotControl::new(&mut progress, limits);
            let retained = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control).unwrap();
            assert_eq!(retained, source);
            assert_eq!(charged(&control, limits), owned);
            let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control) else { panic!("second retained native input must not reset either caller ledger") };
            assert_eq!(error.kind.as_str(), ratio["expectedKind"].as_str().unwrap());
            assert!(charged(&control, limits) >= owned);
        }
        let limits = SqliteDatabaseLimits { max_rows: contract["rowRefusal"]["maximumRows"].as_u64().unwrap() as usize, ..Default::default() };
        let mut progress = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control) else { panic!("native census must refuse after parser ownership") };
        assert_eq!(error.kind.as_str(), contract["rowRefusal"]["expectedKind"].as_str().unwrap());
        let refused_owned = charged(&control, limits);
        let mut reached = false;
        let minimum = contract["cancellation"]["minimumCompleted"].as_u64().unwrap() as usize;
        let mut progress = |event: SqliteSnapshotProgress| {
            if event.phase == SqliteSnapshotPhase::DecodeNative && event.completed >= minimum && event.completed < event.total {
                reached = true;
                false
            } else {
                true
            }
        };
        let limits = SqliteDatabaseLimits::default();
        let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control) else { panic!("known interior native frontier must cancel") };
        assert_eq!(error.kind.as_str(), contract["cancellation"]["expectedKind"].as_str().unwrap());
        let canceled_owned = charged(&control, limits);
        drop(control);
        assert!(reached);
        println!("[DEBUG] SpaceHistory {encoding:?} native ownership: admitted={owned}, row-refusal={refused_owned}, canceled={canceled_owned}; semantic and allocation ledgers agree");
    }
}

#[test]
fn sqlite_snapshot_framework_space_history_native_output_full_requests_and_cumulative_owner_are_admitted(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json")).unwrap();
 let output=&contract["output"];
 let mut source=fixture();source.checkpoints[0].message=output["literal"]["unit"].as_str().unwrap().repeat(usize::try_from(output["literal"]["repeat"].as_u64().unwrap()).unwrap());
 assert_eq!(source.checkpoints[0].message.len(),usize::try_from(output["literal"]["utf8Bytes"].as_u64().unwrap()).unwrap());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let limits=SqliteDatabaseLimits::default();
  let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);
  let (result,requested)=crate::test_allocation::observe(||<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control));
  let first=result.expect("valid full History native output");
  let debit=limits.max_allocation_bytes-control.allocation_remaining_bytes();
  assert!(requested>0,"real History native output must own concrete requested backing");
  assert!(debit>=requested,"History {encoding:?} admitted {debit} bytes but made {requested} concrete allocator requests");
  assert_eq!(decode(first),source);
  let zero=usize::try_from(output["zeroPhysical"]["maximumBytes"].as_u64().unwrap()).unwrap();
  let limits=SqliteDatabaseLimits{max_allocation_bytes:zero,..Default::default()};
  let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);
  let error=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control).unwrap_err();
  assert_eq!(error.kind.as_str(),output["zeroPhysical"]["expectedKind"].as_str().unwrap());
  assert_eq!(zero-control.allocation_remaining_bytes(),usize::try_from(output["zeroPhysical"]["expectedCharge"].as_u64().unwrap()).unwrap());
  let numerator=usize::try_from(output["cumulative"]["ceilingNumerator"].as_u64().unwrap()).unwrap();let denominator=usize::try_from(output["cumulative"]["ceilingDenominator"].as_u64().unwrap()).unwrap();
  for allowance in [debit,debit-1,debit.checked_mul(numerator).unwrap()/denominator]{
   let limits=SqliteDatabaseLimits{max_allocation_bytes:allowance,..Default::default()};
   let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);
   let result=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control);
   if allowance<debit{let error=result.unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);continue;}
   let retained=result.expect("exact admitted History native output");
   assert_eq!(allowance-control.allocation_remaining_bytes(),debit);
   let error=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control).unwrap_err();
   assert_eq!(error.kind.as_str(),output["cumulative"]["expectedKind"].as_str().unwrap());
   assert!(allowance-control.allocation_remaining_bytes()>=debit);
   assert_eq!(decode(retained),source);
  }
  let minimum=usize::try_from(output["cancellation"]["minimumCompleted"].as_u64().unwrap()).unwrap();
  let mut reached=false;let mut cancel=|progress:SqliteSnapshotProgress|{if progress.phase==SqliteSnapshotPhase::EncodeNative&&progress.completed>=minimum&&progress.completed<progress.total{reached=true;false}else{true}};
  let limits=SqliteDatabaseLimits::default();let mut control=SqliteSnapshotControl::new(&mut cancel,limits);
  let error=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control).unwrap_err();
  assert_eq!(error.kind.as_str(),output["cancellation"]["expectedKind"].as_str().unwrap());
  assert!(limits.max_allocation_bytes-control.allocation_remaining_bytes()>0,"admitted canceled output must retain its caller debit");
  drop(control);assert!(reached);
 }
}

#[path="../../../../../../🪐️space/🧪️tests/🪶️sqlite/📏️preflight/🦀️.rs"]
mod public_preflight;
#[test]
fn sqlite_snapshot_framework_space_history_public_borrowed_native_preflight_admits_actual_owner(){
 let facet:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️preflight/🔣️.json")).unwrap();
 let mut short=fixture();short.checkpoints[0].message=facet["textUnit"].as_str().unwrap().into();
 let mut long=short.clone();long.checkpoints[0].message=facet["textUnit"].as_str().unwrap().repeat(usize::try_from(facet["repeat"].as_u64().unwrap()).unwrap());
 public_preflight::verify(&short,&long,SQL,usize::try_from(facet["rows"].as_u64().unwrap()).unwrap(),long.checkpoints[0].message.len(),usize::try_from(facet["cancelAt"].as_u64().unwrap()).unwrap(),|operation|crate::test_allocation::observe(operation));
}
