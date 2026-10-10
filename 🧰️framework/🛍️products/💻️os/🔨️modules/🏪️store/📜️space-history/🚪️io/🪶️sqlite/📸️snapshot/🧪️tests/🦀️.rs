//! 📣️ Explicit owner registration separates generic history construction from registry I/O.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_framework_space_history_explicit_owner_registration_after_create_reload_and_retained() {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    const CASE: &str = "SEMIO_SQLITE_HISTORY_REGISTRATION_CASE";
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📣️registration/🔣️.json")).unwrap();
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
        let mut open = store::RetainedPersistedDocumentHydration::<SpaceHistorySnapshot, SpaceHistoryMutation>::from_decoded_pack(
            source.clone(),
            saved.pack.clone(),
            *semio_framework_hash::hash(&saved.pack).as_bytes(),
            history,
            semio_framework_artifact_reference::ArtifactRef { artifact_id: "sqlite-history-registration".into(), dialect: dialect() },
            None,
            S_SPACE_HISTORY_SCHEMA.into(),
            store::test_support::plain_document_store_owners(),
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            u64::MAX,
            store::PersistedDocumentHydrationTarget::Store { generation: 0 },
            store::os_spr::ActorId("actor:sqlite-retained-registration-fixture".into()),
        );
        let cancellation = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        let mut ready = None;
        for _ in 0..100_000 {
            let mut retained_progress=semio_framework_value::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(1),
                semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(256, u64::MAX,caller_grant()),
                cancellation.clone(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
                &mut retained_progress,
            );
            match open.step(&mut context, semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 }) {
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
        let mut live = store::ArtifactStore::new(envelope, store::os_spr::ActorId("actor:sqlite-retained-registration-fixture".into())).await.unwrap();
        live.install_document_store_owners_exact(store::test_support::plain_document_store_owners());
        live
    };
    let expected_envelope_dialect = if sample["envelopeDialect"].is_null() { None } else { Some(dialect()) };
    assert_eq!(live.envelope().dialect, expected_envelope_dialect);
    assert_eq!(*live.snapshot_ref(), source);
    assert_eq!(dialect().to_coordinate(), corpus["coordinate"].as_str().unwrap());
    assert_eq!(semio_framework_artifact_reference::ArtifactDialect::from(store::space_history::io::sqlite::snapshot::SQLITE_SNAPSHOT_DIALECT), dialect());
    let limits = SqliteDatabaseLimits::default();
    let mut independent = database();
    store::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut independent, &dialect(), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    let independent_bytes = export_sqlite_database(&independent, limits, &mut |_| true).unwrap();
    let export_refusal = store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(), live.snapshot_ref(), encoding, limits, &mut |_| true).await.unwrap_err();
    assert_eq!(export_refusal.cause.kind, semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    assert!(export_refusal.diagnostics.is_empty());
    let import_refusal = store::io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &independent_bytes, limits, &mut |_| true).await.unwrap_err();
    assert_eq!(import_refusal.cause.kind, semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    assert!(import_refusal.diagnostics.is_empty());
    assert_eq!(live.envelope().dialect, expected_envelope_dialect);
    assert_eq!(*live.snapshot_ref(), source);
    store::space_history::io::sqlite::snapshot::register_sqlite_snapshot().unwrap();
    let output = store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(), live.snapshot_ref(), encoding, limits, &mut |_| true).await.unwrap();
    assert!(output.diagnostics.is_empty());
    let bytes = output.value;
    let relational = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
    assert_eq!(relational, independent);
    assert_eq!(store::io::io_mechanism::sqlite_snapshot_metadata(&relational).unwrap(), (dialect(), encoding));
    let imported = store::io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &bytes, limits, &mut |_| true).await.unwrap();
    assert!(imported.diagnostics.is_empty());
    assert_eq!(imported.value, source);
    assert_eq!(store::io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &independent_bytes, limits, &mut |_| true).await.unwrap().value, source);
    let route = store::io::io_mechanism::io_route(&dialect(), &semio_framework_artifact_reference::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT), 1).await.unwrap().value;
    assert_eq!(route.hops.len(), sample["routeHops"].as_u64().unwrap() as usize);
    assert_eq!(route.hops[0].from, dialect());
    assert_eq!(route.hops[0].into, semio_framework_artifact_reference::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT));
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
    store::space_history::io::sqlite::snapshot::register_sqlite_snapshot().unwrap();
    let source = fixture();
    let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let bytes = store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(), &source, encoding, limits, &mut |_| true).await.unwrap().value;
        let database = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
        assert_eq!(store::io::io_mechanism::sqlite_snapshot_metadata(&database).unwrap(), (dialect(), encoding));
        assert_eq!(store::io::io_mechanism::io_import_sqlite_snapshot::<SpaceHistorySnapshot>(&dialect(), &bytes, limits, &mut |_| true).await.unwrap().value, source);
    }
    let route = store::io::io_mechanism::io_route(&dialect(), &semio_framework_artifact_reference::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT), 1).await.unwrap().value;
    assert_eq!(route.hops.len(), 1);
}
#[test]
fn sqlite_snapshot_framework_space_history_independent_surrogate_renumber_preserves_complete_logical_state() {
    let source = fixture();
    let codec = codec();
    let changed = oracle::renumber(&database(), f()["identityOffset"].as_i64().unwrap());
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = with_original_import(changed.clone(),|input,native_owner|(codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), input, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()),native_owner)).unwrap().value;
        assert_eq!(decode(native), source);
    }
}
use super::{S_SPACE_HISTORY_SCHEMA, SpaceHistoryMutation, SpaceHistorySnapshot};
use store::ArtifactSqliteSnapshot;
fn caller_grant()->semio_framework_value::RetainedCloneGrant{serde_json::from_str(include_str!("../🧫️fixtures/🫴️grant/🔣️.json")).expect("original authored History caller policy")}
/// 🫴️ Installs and drains one actual caller recipient for each independent decode fixture operation.
fn with_original_decode<T>(operation:impl FnOnce(&mut store::NativeSnapshotDecodeOwner<'_,'_>)->T)->T{let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{assert!(event.owned_bytes<=1<<30&&(event.total==0||event.completed<=event.total));true};let mut native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut progress);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,caller_grant());let output=operation(&mut owner);drop(owner);close_native_history_recipient(&mut native);drop(native);assert!(!recipient.has_owner());output}
/// 🫴️ Installs and drains one actual caller recipient for each independent encode fixture operation.
fn with_original_encode<T>(operation:impl FnOnce(&mut store::NativeSnapshotEncodeOwner<'_,'_>)->T)->T{let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(event.owned_bytes<=1<<30&&(event.total==0||event.completed<=event.total));true};let mut native=semio_framework_value::NativeEncodeControl::new(1<<30,&mut progress);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,caller_grant());let output=operation(&mut owner);drop(owner);close_encode_history_recipient(&mut native);drop(native);assert!(!recipient.has_owner());output}
/// ♻️ Retires actual encoder refusal custody through its unchanged original allocation control.
/// 🫴️ Keeps each original SQL fixture slot through refusal and closes it under the same caller wallet.
fn with_original_import<T>(database:SqliteDatabase,operation:impl FnOnce(&mut Option<SqliteDatabase>,&mut store::NativeSnapshotEncodeOwner<'_,'_>)->T)->T{
 with_original_encode(|native|{
  let mut input=semio_framework_value::retirement::controlled::ControlledRetirement::new(Some(database)).unwrap_or_else(|_|panic!("SQL fixture supports original retirement"));
  let result=operation(input.original_mut().unwrap(),native);
  for _ in 0..100000{if input.terminal_is_empty(){break}let grant=native.remaining_grant();let capacity=input.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap();native.native().checkpoint().unwrap();native.native().charge(capacity).unwrap();let step=input.step(grant).unwrap();let progress=step.progress();assert!(progress.fits(grant));native.record_progress(progress).unwrap();assert!(progress!=Default::default()||input.terminal_is_empty(),"original SQL fixture close made no funded progress");}
  assert!(input.terminal_is_empty());result
 })
}

fn close_encode_history_recipient(native:&mut semio_framework_value::NativeEncodeControl<'_>){for _ in 0..100_000{if !native.has_retirement_owner(){return}let step=native.close_retirement_recipient(caller_grant()).unwrap();assert!(step.progress()!=Default::default()||!native.has_retirement_owner(),"original encoder close must advance actual custody");}assert!(!native.has_retirement_owner(),"original encoder return supervisor did not finish");}

/// ♻️ Returns only original pending snapshot custody under the unchanged independent test policy.
fn close_native_history_recipient(native:&mut semio_framework_value::NativeDecodeControl<'_>){
    for _ in 0..100_000 {
        if !native.has_retirement_owner(){return}
        let step=native.close_retirement_recipient(caller_grant()).unwrap();
        assert!(step.progress()!=Default::default()||!native.has_retirement_owner(),"original returned snapshot close must advance actual ownership");
    }
    assert!(!native.has_retirement_owner(),"original bounded snapshot return supervisor did not finish");
}

#[test]
fn sqlite_snapshot_framework_space_history_json_turns_preserve_original_ceiling_and_cumulative_charge() {
    let source = fixture();
    let limits = SqliteDatabaseLimits::default();
    let mut accepted = |_| true;
    let mut observed = semio_framework_value::NativeEncodeControl::new(1 << 30, &mut accepted);
    let mut sql_accepted = |_| true;
    let mut sql = SqliteSnapshotControl::new(&mut sql_accepted, limits);
    let output = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source, SnapshotEncoding::Text, &mut sql, &mut store::NativeSnapshotEncodeOwner::new(&mut observed,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096})).unwrap();
    let store::io::IoPayload::Text(text) = output else { panic!("declared text carrier") };
    assert_eq!(semio_framework_pack_json::from_json_str::<SpaceHistorySnapshot>(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(), source);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(),serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&source)).unwrap());
    assert_eq!(observed.maximum_bytes(), 1 << 30);
    let owned = observed.owned_bytes();
    assert!(owned > text.len());
    assert_eq!(limits.max_allocation_bytes - sql.allocation_remaining_bytes(), owned);
    for deficit in [0, 1] {
        let maximum = owned + 3 - deficit;
        let mut accepted = |_| true;
        let mut original = semio_framework_value::NativeEncodeControl::new(maximum, &mut accepted);
        original.charge(3).unwrap();
        let mut sql_accepted = |_| true;
        let mut sql = SqliteSnapshotControl::new(&mut sql_accepted, limits);
        let result = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source, SnapshotEncoding::Text, &mut sql, &mut store::NativeSnapshotEncodeOwner::new(&mut original,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096}));
        assert_eq!(original.maximum_bytes(), maximum);
        assert_eq!(limits.max_allocation_bytes - sql.allocation_remaining_bytes(), original.owned_bytes() - 3);
        if deficit == 0 {
            let store::io::IoPayload::Text(actual) = result.unwrap() else { panic!("declared text carrier") };
            assert_eq!(actual, text);
            assert_eq!(original.owned_bytes(), maximum);
            let mut fresh_sql_accepted = |_| true;
            let mut fresh_sql = SqliteSnapshotControl::new(&mut fresh_sql_accepted, limits);
            assert!(<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source, SnapshotEncoding::Text, &mut fresh_sql, &mut store::NativeSnapshotEncodeOwner::new(&mut original,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096})).is_err());
            assert_eq!(original.owned_bytes(), maximum);
        } else {
            assert_eq!(result.unwrap_err().kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
            assert!(original.owned_bytes() <= maximum);
        }
    }
    println!("[DEBUG] SpaceHistory actual JSON turns: nativeBytes={owned} outputBytes={} original ceiling restored; cumulative and one-short checked; independent=Serde", text.len());
}
use crate as store;
use store::sqlite_snapshot::*;
#[path = "../../../../../../🪐️space/🧪️tests/🪶️sqlite/🔬️oracle/🦀️.rs"]
mod oracle;
const SQL: &str = include_str!("../🗄️.sql");
fn f() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
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
fn dialect() -> semio_framework_artifact_reference::ArtifactDialect {
    semio_framework_artifact_reference::ArtifactDialect { artifact_kind: S_SPACE_HISTORY_SCHEMA.into(), standard: "1".into(), subset: "*".into() }
}
fn codec() -> store::ArtifactSqliteSnapshotCodec {
    store::ArtifactCodec::bare::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA).snapshot_sqlite.expect("actual builtin native factory requires its owned semantic SQLite capability")
}
fn payload(value: &SpaceHistorySnapshot, encoding: SnapshotEncoding) -> store::io::IoPayload {
    match encoding {
        SnapshotEncoding::Binary => store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(value)),
        SnapshotEncoding::Text => store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(value)),
    }
}
fn decode(value: store::io::IoPayload) -> SpaceHistorySnapshot {
    match value {
        store::io::IoPayload::Binary(bytes) => store::ArtifactPack::decode_pack(&bytes).unwrap(),
        store::io::IoPayload::Text(text) => store::ArtifactDsl::parse_dsl(&text).unwrap(),
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
        let export = with_original_decode(|native_owner|(codec.export)(
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
         native_owner));
        assert!(decoded_interior && export.is_err(), "native input must expose known interior Unicode work before typed construction");
        let mut encoded_interior = false;
        let import = with_original_import(expected.clone(),|input,native_owner|(codec.import)(
            S_SPACE_HISTORY_SCHEMA,
            &dialect(),
            input,
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
            ),native_owner));
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
        assert_eq!(with_original_decode(|native_owner|(codec.export)(S_SPACE_HISTORY_SCHEMA, &dialect(), &native, &mut SqliteSnapshotControl::new(&mut |_| true, limits), native_owner)).unwrap().value, expected);
        let native = with_original_import(expected.clone(),|input,native_owner|(codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), input, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits),native_owner)).unwrap().value;
        assert_eq!(decode(native), source);
    }
}
#[test]
fn sqlite_snapshot_framework_space_history_independent_sql_edit_retains_literal_typed_state() {
    let codec = codec();
    let edited = oracle::edit(&database(), &f());
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let native = with_original_import(edited.clone(),|input,native_owner|(codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), input, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()),native_owner)).unwrap().value;
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
            let output = with_original_decode(|native_owner|(codec.export)(S_SPACE_HISTORY_SCHEMA, &dialect(), &native, &mut SqliteSnapshotControl::new(&mut |_| true, limits), native_owner));
            let input = with_original_import(expected.clone(),|input,native_owner|(codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), input, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits),native_owner));
            assert_eq!(output.is_ok(), maximum == rows);
            assert_eq!(input.is_ok(), maximum == rows);
        }
        let limits = SqliteDatabaseLimits { max_schema_bytes: SQL.len() - 1, ..SqliteDatabaseLimits::default() };
        assert!(with_original_decode(|native_owner|(codec.export)(S_SPACE_HISTORY_SCHEMA, &dialect(), &native, &mut SqliteSnapshotControl::new(&mut |_| true, limits), native_owner)).is_err());
        assert!(with_original_import(expected.clone(),|input,native_owner|(codec.import)(S_SPACE_HISTORY_SCHEMA, &dialect(), input, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits),native_owner)).is_err());
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
            let result = with_original_decode(|native_owner|(codec.export)(
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
             native_owner));
            assert!(result.is_err() && seen);
        }
        for phase in [SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::EncodeNative] {
            let mut seen = false;
            let result = with_original_import(expected.clone(),|input,native_owner|(codec.import)(
                S_SPACE_HISTORY_SCHEMA,
                &dialect(),
                input,
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
                ),native_owner));
            assert!(result.is_err() && seen);
        }
        let wrong = semio_framework_artifact_reference::ArtifactDialect { subset: "invented".into(), ..dialect() };
        assert!(with_original_decode(|native_owner|(codec.export)(S_SPACE_HISTORY_SCHEMA, &wrong, &native, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()), native_owner)).is_err());
    }
}

#[test]
fn sqlite_snapshot_framework_space_history_native_input_retained_materialization_uses_same_caller_ledger() {
    let corpus = f();
    let sample = &corpus["interiorControl"];
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧮️ownership/🔣️.json")).unwrap();
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
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_decoding::NativeDecodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut native_progress);
        native.install_retirement_recipient(&mut original_recipient).unwrap();
        let mut native_owner=store::NativeSnapshotDecodeOwner::new(&mut native,caller_grant());
        let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control,&mut native_owner) else { panic!("zero physical allowance must refuse before native construction") };
        assert_eq!(error.kind.as_str(), contract["zeroPhysical"]["expectedKind"].as_str().unwrap());
        assert_eq!(limits.max_value_bytes - control.reconstruction_remaining_bytes().unwrap(), contract["zeroPhysical"]["expectedCharge"].as_u64().unwrap() as usize);
        assert_eq!(control.allocation_remaining_bytes(), 0);
        drop(native_owner);close_native_history_recipient(&mut native);drop(native);assert!(!original_recipient.has_owner());
        let limits = SqliteDatabaseLimits::default();
        let mut progress = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_decoding::NativeDecodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut native_progress);
        native.install_retirement_recipient(&mut original_recipient).unwrap();
        let mut native_owner=store::NativeSnapshotDecodeOwner::new(&mut native,caller_grant());
        let first = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control,&mut native_owner).unwrap();
        assert_eq!(first, source);
        let owned = charged(&control, limits);
        let ceiling = owned.checked_mul(ratio["ceilingNumerator"].as_u64().unwrap() as usize).unwrap() / ratio["ceilingDenominator"].as_u64().unwrap() as usize;
        drop(native_owner);close_native_history_recipient(&mut native);drop(native);assert!(!original_recipient.has_owner());
        for physical in [false, true] {
            let limits = if physical { SqliteDatabaseLimits { max_allocation_bytes: ceiling, ..Default::default() } } else { SqliteDatabaseLimits { max_value_bytes: ceiling, ..Default::default() } };
            let mut progress = |_| true;
            let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_decoding::NativeDecodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut native_progress);
        native.install_retirement_recipient(&mut original_recipient).unwrap();
        let mut native_owner=store::NativeSnapshotDecodeOwner::new(&mut native,caller_grant());
            let retained = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control,&mut native_owner).unwrap();
            assert_eq!(retained, source);
            assert_eq!(charged(&control, limits), owned);
            let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control,&mut native_owner) else { panic!("second retained native input must not reset either caller ledger") };
            assert_eq!(error.kind.as_str(), ratio["expectedKind"].as_str().unwrap());
            assert!(charged(&control, limits) >= owned);
        drop(native_owner);close_native_history_recipient(&mut native);drop(native);assert!(!original_recipient.has_owner());
        }
        let limits = SqliteDatabaseLimits { max_rows: contract["rowRefusal"]["maximumRows"].as_u64().unwrap() as usize, ..Default::default() };
        let mut progress = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut progress, limits);
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_decoding::NativeDecodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut native_progress);
        native.install_retirement_recipient(&mut original_recipient).unwrap();
        let mut native_owner=store::NativeSnapshotDecodeOwner::new(&mut native,caller_grant());
        let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control,&mut native_owner) else { panic!("native census must refuse after parser ownership") };
        assert_eq!(error.kind.as_str(), contract["rowRefusal"]["expectedKind"].as_str().unwrap());
        let refused_owned = charged(&control, limits);
        drop(native_owner);close_native_history_recipient(&mut native);drop(native);assert!(!original_recipient.has_owner());
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
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_decoding::NativeDecodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut native_progress);
        native.install_retirement_recipient(&mut original_recipient).unwrap();
        let mut native_owner=store::NativeSnapshotDecodeOwner::new(&mut native,caller_grant());
        let Err(error) = <SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload, &mut control,&mut native_owner) else { panic!("known interior native frontier must cancel") };
        assert_eq!(error.kind.as_str(), contract["cancellation"]["expectedKind"].as_str().unwrap());
        let canceled_owned = charged(&control, limits);
        drop(native_owner);close_native_history_recipient(&mut native);drop(native);assert!(!original_recipient.has_owner());
        drop(control);
        assert!(reached);
        println!("[DEBUG] SpaceHistory {encoding:?} native ownership: admitted={owned}, row-refusal={refused_owned}, canceled={canceled_owned}; semantic and allocation ledgers agree");
    }
}

#[test]
fn sqlite_snapshot_framework_space_history_native_output_full_requests_and_cumulative_owner_are_admitted(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧮️ownership/🔣️.json")).unwrap();
 let output=&contract["output"];
 let mut source=fixture();source.checkpoints[0].message=output["literal"]["unit"].as_str().unwrap().repeat(usize::try_from(output["literal"]["repeat"].as_u64().unwrap()).unwrap());
 assert_eq!(source.checkpoints[0].message.len(),usize::try_from(output["literal"]["utf8Bytes"].as_u64().unwrap()).unwrap());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let limits=SqliteDatabaseLimits::default();
  let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_encoding::NativeEncodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut native=semio_framework_value::NativeEncodeControl::new(1<<30,&mut native_progress);
  let (result,requested)=crate::test_allocation::observe(||<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control,&mut store::NativeSnapshotEncodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096})));
  let first=result.expect("valid full History native output");
  let debit=limits.max_allocation_bytes-control.allocation_remaining_bytes();
  assert!(requested>0,"real History native output must own concrete requested backing");
  assert!(debit>=requested,"History {encoding:?} admitted {debit} bytes but made {requested} concrete allocator requests");
  assert_eq!(decode(first),source);
  let zero=usize::try_from(output["zeroPhysical"]["maximumBytes"].as_u64().unwrap()).unwrap();
  let limits=SqliteDatabaseLimits{max_allocation_bytes:zero,..Default::default()};
  let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_encoding::NativeEncodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut native=semio_framework_value::NativeEncodeControl::new(1<<30,&mut native_progress);
  let error=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control,&mut store::NativeSnapshotEncodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096})).unwrap_err();
  assert_eq!(error.kind.as_str(),output["zeroPhysical"]["expectedKind"].as_str().unwrap());
  assert_eq!(zero-control.allocation_remaining_bytes(),usize::try_from(output["zeroPhysical"]["expectedCharge"].as_u64().unwrap()).unwrap());
  let numerator=usize::try_from(output["cumulative"]["ceilingNumerator"].as_u64().unwrap()).unwrap();let denominator=usize::try_from(output["cumulative"]["ceilingDenominator"].as_u64().unwrap()).unwrap();
  for allowance in [debit,debit-1,debit.checked_mul(numerator).unwrap()/denominator]{
   let limits=SqliteDatabaseLimits{max_allocation_bytes:allowance,..Default::default()};
   let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_encoding::NativeEncodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut native=semio_framework_value::NativeEncodeControl::new(1<<30,&mut native_progress);
   let result=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control,&mut store::NativeSnapshotEncodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096}));
   if allowance<debit{let error=result.unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);continue;}
   let retained=result.expect("exact admitted History native output");
   assert_eq!(allowance-control.allocation_remaining_bytes(),debit);
   let error=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control,&mut store::NativeSnapshotEncodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096})).unwrap_err();
   assert_eq!(error.kind.as_str(),output["cumulative"]["expectedKind"].as_str().unwrap());
   assert!(allowance-control.allocation_remaining_bytes()>=debit);
   assert_eq!(decode(retained),source);
  }
  let minimum=usize::try_from(output["cancellation"]["minimumCompleted"].as_u64().unwrap()).unwrap();
  let mut reached=false;let mut cancel=|progress:SqliteSnapshotProgress|{if progress.phase==SqliteSnapshotPhase::EncodeNative&&progress.completed>=minimum&&progress.completed<progress.total{reached=true;false}else{true}};
  let limits=SqliteDatabaseLimits::default();let mut control=SqliteSnapshotControl::new(&mut cancel,limits);
        let native_deadline=std::time::Instant::now()+std::time::Duration::from_secs(60);
        let mut native_progress=|event|{let semio_framework_value::native_encoding::NativeEncodeProgress{completed,total,owned_bytes}=event;assert!(owned_bytes<=1<<30&& (total==0||completed<=total));std::time::Instant::now()<native_deadline};
        let mut native=semio_framework_value::NativeEncodeControl::new(1<<30,&mut native_progress);
  let error=<SpaceHistorySnapshot as store::ArtifactSqliteSnapshot>::encode_sqlite_snapshot_native(&source,encoding,&mut control,&mut store::NativeSnapshotEncodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096})).unwrap_err();
  assert_eq!(error.kind.as_str(),output["cancellation"]["expectedKind"].as_str().unwrap());
  assert!(limits.max_allocation_bytes-control.allocation_remaining_bytes()>0,"admitted canceled output must retain its caller debit");
  drop(control);assert!(reached);
 }
}

#[path="../../../../../../🪐️space/🧪️tests/🪶️sqlite/📏️preflight/🦀️.rs"]
mod public_preflight;
#[test]
fn sqlite_snapshot_framework_space_history_public_borrowed_native_preflight_admits_actual_owner(){
 let facet:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️preflight/🔣️.json")).unwrap();
 let mut short=fixture();short.checkpoints[0].message=facet["textUnit"].as_str().unwrap().into();
 let mut long=short.clone();long.checkpoints[0].message=facet["textUnit"].as_str().unwrap().repeat(usize::try_from(facet["repeat"].as_u64().unwrap()).unwrap());
 let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(facet["callerGrant"].clone()).unwrap();let maximum=facet["nativeMaximumBytes"].as_u64().unwrap()as usize;let mut receive=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{assert!(event.owned_bytes<=maximum);true};let mut emit=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(event.owned_bytes<=maximum);true};let mut receive_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut emit_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut decode=semio_framework_value::NativeDecodeControl::new(maximum,&mut receive);let mut encode=semio_framework_value::NativeEncodeControl::new(maximum,&mut emit);decode.install_retirement_recipient(&mut receive_recipient).unwrap();encode.install_retirement_recipient(&mut emit_recipient).unwrap();let mut original_io=store::io::io_mechanism::IoRunControl::new(&mut decode,&mut encode,grant);
 public_preflight::verify(&short,&long,SQL,usize::try_from(facet["rows"].as_u64().unwrap()).unwrap(),long.checkpoints[0].message.len(),usize::try_from(facet["cancelAt"].as_u64().unwrap()).unwrap(),|operation|crate::test_allocation::observe(operation),&mut original_io);
}

fn semantic_role_source(index:usize,text:&str)->SpaceHistorySnapshot{let mut source=fixture();match index{0=>{source.active_alternative_id=Some(text.into());},1=>{source.checkpoints[0].id=text.into();},2=>{source.checkpoints[0].parent_id=Some(text.into());},3=>{source.checkpoints[0].message=text.into();},4=>{source.checkpoints[0].authors[0].id=text.into();},5=>{source.checkpoints[0].authors[0].name=text.into();},6=>{source.checkpoints[0].authors[1].avatar=Some(text.into());},7=>{source.checkpoints[0].members[0].document_id=text.into();},8=>{source.checkpoints[0].members[0].checkpoint_id=text.into();},9=>{source.checkpoints[0].members[0].alternative_id=text.into();},10=>{source.alternatives[0].id=text.into();},11=>{source.alternatives[0].name=text.into();},12=>{source.alternatives[0].checkpoint_ids[0]=text.into();},_=>panic!("closed thirteen authored text roles")}source}

#[test]
fn sqlite_snapshot_framework_space_history_complete_native_semantic_text_roles_and_columns(){
 use store::ArtifactSqliteSnapshot;use std::io::Write;use std::process::{Command,Stdio};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 
 let defaults=SqliteDatabaseLimits::default();let text=plan["text"].as_str().unwrap();assert_eq!(text.len(),plan["textBytes"].as_u64().unwrap()as usize);
 for(index,sample)in plan["cases"].as_array().unwrap().iter().enumerate(){
  let source=semantic_role_source(index,text);let bytes=sample["semanticBytes"].as_u64().unwrap()as usize;
  let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();
  let file=export_sqlite_database(&database,defaults,&mut |_|true).unwrap();
  let script=r###"import{Database}from "bun:sqlite";const plan=JSON.parse(process.argv[1]),sample=JSON.parse(process.argv[2]);if(Buffer.byteLength(plan.text,"utf8")!==plan.textBytes)throw Error("independent UTF8 extent");const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const quote=v=>'"'+v.replaceAll('"','""')+'"';try{if(db.query("PRAGMA integrity_check").get().integrity_check!=="ok"||db.query("PRAGMA foreign_key_check").all().length)throw Error("independent physical integrity");let rows=0,bytes=0;for(const[table,count]of Object.entries(plan.tableRows)){const fields=db.query("PRAGMA table_info("+quote(table)+")").all().map(v=>v.name);const cells=fields.map(v=>{const f=quote(v);return "CASE typeof("+f+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+f+" AS BLOB)) WHEN 'blob' THEN length("+f+") ELSE 0 END";}).join("+");const actual=db.query("SELECT COUNT(*) AS rows,COALESCE(SUM("+cells+"),0) AS bytes FROM "+quote(table)).get();if(actual.rows!==count)throw Error("authored row extent "+table);rows+=actual.rows;bytes+=actual.bytes;}if(rows!==plan.rows||bytes!==sample.semanticBytes)throw Error("independent complete cell extent "+JSON.stringify({rows,bytes}));}finally{db.close();}"###;
  let mut child=Command::new("bun").args(["-e",script]).arg(plan.to_string()).arg(sample.to_string()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
  child.stdin.take().unwrap().write_all(&file).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}: {}",sample["id"],String::from_utf8_lossy(&result.stderr));
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let input=payload(&source,encoding);
   let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
   let mut original_progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{assert!(event.owned_bytes<=1<<30&&(event.total==0||event.completed<=event.total));true};
   let mut original_native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut original_progress);
   original_native.install_retirement_recipient(&mut original_recipient).unwrap();
   let mut original_owner=store::NativeSnapshotDecodeOwner::new(&mut original_native,caller_grant());
   let retained=SpaceHistorySnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,defaults), &mut original_owner).unwrap();assert_eq!(retained,source);retained.retire_sqlite_snapshot();
   let short=SqliteDatabaseLimits{max_columns:7-1,..defaults};
   assert!(SpaceHistorySnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,short), &mut original_owner).is_err(),"{} borrowed complete native columns must refuse before typed materialization",sample["id"]);
   drop(original_owner);close_native_history_recipient(&mut original_native);drop(original_native);assert!(!original_recipient.has_owner());
   for maximum in[bytes-1,bytes]{
    let limits=SqliteDatabaseLimits{max_value_bytes:maximum,..defaults};
    let output=with_original_encode(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits),native_owner));
    assert_eq!(output.is_ok(),maximum==bytes,"{} complete typed semantic output cells",sample["id"]);
    let preflight=source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));
    assert_eq!(preflight.is_ok(),maximum==bytes,"{} complete borrowed semantic preflight cells",sample["id"]);
   }
  }
  source.retire_sqlite_snapshot();
 }
}

#[test]
fn sqlite_snapshot_framework_space_history_borrowed_semantic_gate_uses_closed_exact_cells_before_typed_construction(){
 use store::ArtifactSqliteSnapshot;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();let defaults=SqliteDatabaseLimits::default();
 for(index,sample)in plan["cases"].as_array().unwrap().iter().enumerate(){
  let source=semantic_role_source(index,plan["text"].as_str().unwrap());let bytes=sample["semanticBytes"].as_u64().unwrap()as usize;let rows=plan["rows"].as_u64().unwrap()as usize;let schema=SpaceHistorySnapshot::SQLITE_SCHEMA.len();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=payload(&source,encoding);
   for(field,maximum,success)in[("bytes",bytes,true),("bytes",bytes-1,false),("rows",rows,true),("rows",rows-1,false),("columns",7,true),("columns",6,false),("schema",schema,true),("schema",schema-1,false)]{
    let limits=match field{"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},_=>unreachable!()};
    let mut callback=|_|true;let mut caller=SqliteSnapshotControl::new(&mut callback,defaults);let mut original_progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{assert!(event.owned_bytes<=1<<30&&(event.total==0||event.completed<=event.total));true};let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut original_native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut original_progress);original_native.install_retirement_recipient(&mut original_recipient).unwrap();let mut original_owner=store::NativeSnapshotDecodeOwner::new(&mut original_native,caller_grant());let admitted=store::space_history::io::sqlite::snapshot::decode_native_cst(&input,&mut caller,&mut original_owner,|value,output,native,body|{store::space_history::io::sqlite::snapshot::admission::intrinsic(value,native,limits)?;body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Option<()>>(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1})?;*output=Some(());body.record_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<()>>(),..Default::default()})});drop(original_owner);close_native_history_recipient(&mut original_native);assert_eq!(admitted.is_ok(),success,"{} isolated borrowed {} grant before typed construction",sample["id"],field);
   }
  }source.retire_sqlite_snapshot();
 }
}

#[test]
fn sqlite_snapshot_framework_space_history_original_native_owner_cancels_and_keeps_its_complete_receipt(){
 let corpus=f();let specimen=&corpus["interiorControl"];let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧮️ownership/🔣️.json")).unwrap();let minimum=contract["cancellation"]["minimumCompleted"].as_u64().unwrap()as usize;
 let mut source=fixture();source.checkpoints[0].message=specimen["textUnit"].as_str().unwrap().repeat(specimen["repeat"].as_u64().unwrap()as usize);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for interior in[false,true]{
  let limits=SqliteDatabaseLimits::default();let input=payload(&source,encoding);let mut sql_progress=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_progress,limits);let armed=std::cell::Cell::new(false);let mut reached=false;
  let mut observe=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if armed.get()&&(!interior||(event.completed>=minimum&&event.completed<event.total)){reached=true;false}else{true}};
  let mut original_recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
  let mut native=semio_framework_value::NativeDecodeControl::new(1<<30,&mut observe);native.install_retirement_recipient(&mut original_recipient).unwrap();native.charge(7).unwrap();armed.set(true);
  let mut native_owner=store::NativeSnapshotDecodeOwner::new(&mut native,caller_grant());
  let error=SpaceHistorySnapshot::decode_sqlite_snapshot_native(&input,&mut sql,&mut native_owner).unwrap_err();drop(native_owner);assert_eq!(error.kind.as_str(),contract["cancellation"]["expectedKind"].as_str().unwrap());let debit=native.owned_bytes()-7;assert_eq!(native.maximum_bytes(),1<<30);assert_eq!(debit,limits.max_allocation_bytes-sql.allocation_remaining_bytes());assert_eq!(debit,limits.max_value_bytes-sql.reconstruction_remaining_bytes().unwrap());armed.set(false);close_native_history_recipient(&mut native);drop(native);drop(observe);assert!(!original_recipient.has_owner());assert!(reached);
  let mut sql_progress=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_progress,limits);let armed=std::cell::Cell::new(false);let mut reached=false;let mut observe=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if armed.get()&&(!interior||(event.completed>=minimum&&event.completed<event.total)){reached=true;false}else{true}};
  let mut native=semio_framework_value::NativeEncodeControl::new(1<<30,&mut observe);native.charge(7).unwrap();armed.set(true);
  let error=source.encode_sqlite_snapshot_native(encoding,&mut sql,&mut store::NativeSnapshotEncodeOwner::new(&mut native,semio_framework_value::RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096})).unwrap_err();assert_eq!(error.kind.as_str(),contract["cancellation"]["expectedKind"].as_str().unwrap());assert_eq!(native.maximum_bytes(),1<<30);assert_eq!(native.owned_bytes()-7,limits.max_allocation_bytes-sql.allocation_remaining_bytes());drop(native);drop(observe);assert!(reached);
  println!("[DEBUG] SpaceHistory {encoding:?} original native owner interior={interior} canceled and retained complete cumulative receipt");
 }}
}
