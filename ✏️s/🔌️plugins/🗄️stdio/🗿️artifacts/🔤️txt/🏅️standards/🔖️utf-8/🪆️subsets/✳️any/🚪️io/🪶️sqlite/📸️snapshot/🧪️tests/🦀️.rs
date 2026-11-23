use crate::{standards::v_utf_8::subsets::any::schema::snapshot::LineEnding, standards::v_utf_8::subsets::any::schema::snapshot::TxtSnapshot};
use semio_os_kernel::{sqlite_snapshot::artifact::Cell, sqlite_snapshot::SnapshotEncoding, sqlite_snapshot::SqliteSnapshotPhase};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::NativeEncodeControl;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_os_kernel::{sqlite_snapshot::ValueError, sqlite_snapshot::ValueRefusalKind};
#[allow(unused_imports)]
use semio_os_kernel::{sqlite_snapshot::artifact::Projection, sqlite_snapshot::artifact::Reconstruction, sqlite_snapshot::artifact::ordered_row_refs, sqlite_snapshot::validate_sqlite_database_schema, sqlite_snapshot::SqliteDatabase};
#[allow(unused_imports)]
use semio_framework_value_native_owned::native_decoding::NativeDecodeProgress;
use crate::standards::v_utf_8::subsets::any::io::sqlite::snapshot::{original_encoding};

/// 🎟️ Supplies the immutable caller policy independently of every observed frontier.
fn original_caller_grant()->semio_framework_value::RetainedCloneGrant{
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🫴️receiving.json")).unwrap();serde_json::from_value(fixture["originalCallerGrant"].clone()).unwrap()
}

/// 🛬️ Preserves this test's original SQL controller and retained native recipient through decode.
fn with_original_decode<O>(sql:&mut SqliteSnapshotControl<'_>,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>,&mut store::NativeSnapshotDecodeOwner<'_,'_>)->O)->O{
 semio_framework_test_original_snapshot_owners::with_decoding(|owner|operation(sql,owner))
}

/// 🛫️ Supplies the actual encode owner without claiming cold producer receipt conservation.
fn with_original_encode<O>(sql:&mut SqliteSnapshotControl<'_>,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>,&mut store::NativeSnapshotEncodeOwner<'_,'_>)->O)->O{
 semio_framework_test_original_snapshot_owners::with_encoding(|owner|operation(sql,owner))
}

/// 📸️ Invokes only the genuine supplied-owner typed decode definition.
fn decode_original(payload:&store::io_schema::IoPayload,sql:&mut SqliteSnapshotControl<'_>)->Result<TxtSnapshot,semio_framework_test_original_snapshot_owners::OriginalSnapshotRefusal>{semio_framework_test_original_snapshot_owners::decode_native::<TxtSnapshot>(payload,sql)}

/// 📸️ Invokes only the current supplied-owner typed encode definition.
fn encode_original(snapshot:&TxtSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,sql:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_test_original_snapshot_owners::OriginalSnapshotRefusal>{semio_framework_test_original_snapshot_owners::encode_native(snapshot,encoding,sql)}

use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v_utf_8::subsets::any::io::sqlite::snapshot::*;
use semio_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, os_store::serialization::ArtifactSqliteSnapshotHost};

fn registered_full_fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🗂️typed-file.json")).unwrap()}

/// ♻️ Runs one funded retirement turn of a controlled owner inside one original effect, observing only the owner's own heap use.
fn retire_turn<T:semio_framework_value::retirement::RetireOwned>(owner:&mut semio_framework_value::retirement::controlled::ControlledRetirement<T>,bank:&mut semio_framework_value_testing_admission::EffectBank,grant:semio_framework_value::RetainedCloneGrant)->(Result<semio_framework_value::RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,usize,usize){
 let mut heap=(0,0);
 let turn=bank.run_outcome(grant,1<<20,|effect|{let mut done=None;semio_framework_value_testing_admission::with_original_invocation(None,effect,&mut |effect|{let(poll,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant.maximum_copy_bytes,effect));heap=(observed.requested_bytes,observed.released_bytes);done=Some(match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{..}=>panic!("retirement turn stayed pending")})}).unwrap();done.expect("retirement turn ran")});
 bank.take_admitted();(turn.result,heap.0,heap.1)
}
fn retirement_bank(grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value_testing_admission::EffectBank{semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::BankWallet::new(grant,1<<40))}

fn txt_full_file_retire<T:semio_framework_value::retirement::RetireOwned>(output:T,backing:usize,policy:store::io::testing::SnapshotFileCaller){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;let(mut owner,heap)=observe(||semio_framework_value::retirement::controlled::ControlledRetirement::new(output).map_err(|(error,_)|error).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut bank=retirement_bank(policy.close_grant);let(zero,born,released)=retire_turn(&mut owner,&mut bank,Default::default());assert_eq!(zero.unwrap().progress(),Default::default());assert_eq!((born,released),(0,0));let(mut born,mut released)=(0,0);
 for _ in 0..policy.maximum_close_turns{if owner.terminal_is_empty(){break}let(step,requested,freed)=retire_turn(&mut owner,&mut bank,policy.close_grant);let step=step.unwrap();assert!(step.progress().fits(policy.close_grant));assert_eq!((requested,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=requested;released+=freed;}assert!(owner.terminal_is_empty());assert_eq!(backing+born,released);
}

#[semio_framework_async_macros::async_test]
async fn txt_original_receiving_registered_complete_store_and_destination(){
 use store::{io::io_mechanism::{io_export_sqlite_snapshot_read,io_import_sqlite_snapshot_into},sqlite_snapshot::SnapshotEncoding};let f=registered_full_fixture();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:f["dialect"]["artifactKind"].as_str().unwrap().into(),standard:f["dialect"]["standard"].as_str().unwrap().into(),subset:f["dialect"]["subset"].as_str().unwrap().into()};let policy=store::io::testing::SnapshotFileCaller{grant:original_caller_grant(),native_maximum_bytes:16777216,close_grant:original_caller_grant(),maximum_close_turns:4096};let limits=SqliteDatabaseLimits{max_allocation_bytes:policy.native_maximum_bytes,..Default::default()};semio_plugin::PluginBuilder::<_, semio_plugin::app::NoPluginApp>::new("stdio").label("TXT SQLite Test").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 for row in f["cases"].as_array().unwrap(){let input=&row["snapshot"];let source=TxtSnapshot{schema:input["schema"].as_str().unwrap().into(),lines:input["lines"].as_array().unwrap().iter().map(|line|line.as_str().unwrap().into()).collect(),trailing_newline:input["trailingNewline"].as_bool().unwrap(),line_ending:if input["lineEnding"]=="crLf"{LineEnding::CrLf}else{LineEnding::Lf}};assert_eq!(source.to_body(),row["raw"].as_str().unwrap());assert_ne!(TxtSnapshot::from_body(row["raw"].as_str().unwrap()),source);let envelope=store::os_store::document_materialize::create_document_envelope::<TxtSnapshot,crate::TxtMutation>(crate::STDIO_TXT_DOCUMENT_SCHEMA,row["id"].as_str().unwrap(),source.clone(),None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);let mut original_store=store::os_store::test_support::plain_test_store(envelope).await;let read=original_store.snapshot_read().unwrap().into_erased();let pointer=read.get::<TxtSnapshot>().unwrap()as*const _;assert_eq!(read.get::<TxtSnapshot>().unwrap(),&source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=store::io::testing::with_snapshot_file_decoding(policy,limits,&mut |_|true,async|sql,native|io_export_sqlite_snapshot_read(&dialect,&read,encoding,sql,native).await).await.unwrap().value;assert_eq!(read.get::<TxtSnapshot>().unwrap()as*const _,pointer);assert_eq!(read.get::<TxtSnapshot>().unwrap(),&source);let physical=import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();assert_eq!(store::io::io_mechanism::sqlite_snapshot_metadata(&physical).unwrap(),(dialect.clone(),encoding));assert_eq!(physical.table("text_line").unwrap().rows.len(),source.lines.len());assert_eq!(physical.table("text_document").unwrap().rows[0].values[1],SqliteValue::Text(source.schema.clone()));let mut destination=None::<TxtSnapshot>;let result=store::io::testing::with_snapshot_file_encoding(policy,limits,&mut |_|true,async|sql,native|io_import_sqlite_snapshot_into(&dialect,&bytes,&mut destination,sql,native).await).await.unwrap();assert!(result.diagnostics.is_empty());assert_eq!(destination.as_ref(),Some(&source));let output=destination.take().unwrap();let backing=output.schema.capacity()+output.lines.capacity()*std::mem::size_of::<String>()+output.lines.iter().map(|line|line.capacity()).sum::<usize>();txt_full_file_retire(output,backing,policy);let backing=bytes.capacity();txt_full_file_retire(bytes,backing,policy);}
  let typed=read.try_into_typed::<TxtSnapshot>().unwrap_or_else(|_|panic!("original TXT Store read retains its exact registered type"));let witness=typed.try_return_to_registry_witness().unwrap_or_else(|_|panic!("original TXT read returns to its real issuer"));drop(witness);store::os_store::test_support::close_plain_test_store(&mut original_store);
 }
}

fn native_fixture() -> serde_json::Value {
    serde_json::from_str(fixture_text!("../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🛂️native.json")).unwrap()
}

fn native_case(value: &serde_json::Value) -> TxtSnapshot {
    TxtSnapshot { schema: crate::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: value["lines"].as_array().unwrap().iter().map(|line| line.as_str().unwrap().into()).collect(), trailing_newline: value["trailingNewline"].as_bool().unwrap(), line_ending: if value["lineEnding"] == "crLf" { LineEnding::CrLf } else { LineEnding::Lf } }
}

fn native_payload(snapshot: &TxtSnapshot, encoding: store::sqlite_snapshot::SnapshotEncoding) -> store::io_schema::IoPayload {
    match encoding { store::sqlite_snapshot::SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(<TxtSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot)), store::sqlite_snapshot::SnapshotEncoding::Text => store::io_schema::IoPayload::Text(<TxtSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(snapshot)) }
}

#[test]
fn sqlite_snapshot_txt_controlled_native_matches_the_real_external_carrier() {
    use store::sqlite_snapshot::SnapshotEncoding;
    for case in native_fixture()["nativeCases"].as_array().unwrap() {
        let snapshot = native_case(case);
        assert_eq!(snapshot.to_body(), case["body"].as_str().unwrap());
        assert_eq!(TxtSnapshot::from_body(case["body"].as_str().unwrap()), snapshot);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let ordinary = native_payload(&snapshot, encoding);
            assert_eq!(encode_original(&snapshot,encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), ordinary);
            assert_eq!(decode_original(&ordinary, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
        }
    }
}

#[test]
fn sqlite_snapshot_txt_native_controls_refuse_inside_copy_and_at_exact_limits() {
    use store::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    let fixture = native_fixture();
    let text = fixture["controls"]["longText"].as_str().unwrap().repeat(fixture["controls"]["longRepeat"].as_u64().unwrap() as usize);
    let snapshot = TxtSnapshot { lines: vec![text, String::new(), "끝\0".into()], trailing_newline: true, line_ending: LineEnding::CrLf, ..TxtSnapshot::default() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = native_payload(&snapshot, encoding);
        let size = match &payload { store::io_schema::IoPayload::Binary(bytes) => bytes.len(), store::io_schema::IoPayload::Text(text) => text.len() };
        let exact = SqliteDatabaseLimits { max_file_bytes: size, max_rows: fixture["controls"]["domainRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
        assert_eq!(encode_original(&snapshot,encoding, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap(), payload);
        assert_eq!(decode_original(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap(), snapshot);
        for limits in [SqliteDatabaseLimits { max_file_bytes: size - 1, ..exact }, SqliteDatabaseLimits { max_rows: exact.max_rows - 1, ..exact }, SqliteDatabaseLimits { max_value_bytes: fixture["controls"]["smallValueBytes"].as_u64().unwrap() as usize, ..exact }] {
            assert!(encode_original(&snapshot,encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
            assert!(decode_original(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        }
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
            let mut interior = false;
            let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { let cancel = event.phase == phase && event.completed >= fixture["controls"]["cancelAtBytes"].as_u64().unwrap() as usize && event.total > event.completed; interior |= cancel; !cancel };
            let result = if phase == SqliteSnapshotPhase::DecodeNative { decode_original(&payload, &mut SqliteSnapshotControl::new(&mut callback, exact)).map(|_| ()) } else { encode_original(&snapshot,encoding, &mut SqliteSnapshotControl::new(&mut callback, exact)).map(|_| ()) };
            let error = result.unwrap_err();
            assert!(interior && format!("{error:?}").contains("Canceled"), "{phase:?}: {error:?}");
        }
    }
}

#[test]
fn sqlite_snapshot_txt_projection_and_reconstruction_cancel_inside_long_literal_fields() {
    use store::sqlite_snapshot::SqliteSnapshotPhase;
    let fixture = native_fixture();
    let text = fixture["controls"]["longText"].as_str().unwrap().repeat(fixture["controls"]["longRepeat"].as_u64().unwrap() as usize);
    for schema_field in [false, true] {
        let snapshot = if schema_field { TxtSnapshot { schema: text.clone(), ..TxtSnapshot::default() } } else { TxtSnapshot { lines: vec![text.clone()], ..TxtSnapshot::default() } };
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();
        for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
            let mut interior = false;
            let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { let cancel = event.phase == phase && event.completed >= fixture["controls"]["cancelAtBytes"].as_u64().unwrap() as usize && event.total > event.completed; interior |= cancel; !cancel };
            let result = if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).map(|_| ()) } else { TxtSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).map(|_| ()) };
            let error = result.unwrap_err();
            assert!(interior && format!("{error:?}").contains("Canceled"), "{phase:?}: {error:?}");
        }
    }
}

#[test]
fn sqlite_snapshot_txt_surrogate_ids_and_literal_metadata_are_not_native_headers() {
    let fixture = native_fixture();
    for schema in fixture["literalSchemas"].as_array().unwrap() {
        let snapshot = TxtSnapshot { schema: schema.as_str().unwrap().into(), lines: vec!["任意\0\nembedded".into(), String::new()], ..TxtSnapshot::default() };
        let mut database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();
        let key = fixture["controls"]["rootIdentity"].as_i64().unwrap();
        database.table_mut("text_document").unwrap().rows[0].rowid = key;
        database.table_mut("text_document").unwrap().rows[0].values[0] = SqliteValue::Integer(key);
        for row in &mut database.table_mut("text_line").unwrap().rows { row.values[1] = SqliteValue::Integer(key); }
        assert_eq!(TxtSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(), snapshot);
        let valid = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
        assert!(snapshot.validate_sqlite_snapshot_subset(&valid, &database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_ok());
        for invalid in [semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.csv".into(), ..valid.clone() }, semio_framework_artifact_reference::ArtifactDialect { standard: "other".into(), ..valid.clone() }, semio_framework_artifact_reference::ArtifactDialect { subset: "other".into(), ..valid.clone() }] {
            assert!(snapshot.validate_sqlite_snapshot_subset(&invalid, &database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
        }
        for corruption in 0..6 {
            let mut broken = database.clone();
            match corruption { 0 => broken.table_mut("text_document").unwrap().rows[0].rowid = -1, 1 => broken.table_mut("text_document").unwrap().rows[0].values[0] = SqliteValue::Integer(key + 1), 2 => broken.table_mut("text_document").unwrap().rows[0].values.push(SqliteValue::Null), 3 => broken.table_mut("text_line").unwrap().rows[0].values[1] = SqliteValue::Integer(key + 1), 4 => broken.table_mut("text_line").unwrap().rows[0].values[2] = SqliteValue::Integer(-1), _ => broken.table_mut("text_line").unwrap().rows[0].values.push(SqliteValue::Null) }
            assert!(TxtSnapshot::from_sqlite_database(&broken,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).is_err());
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_txt_actual_typed_io_preserves_fields_without_native_phases() {
    use store::{io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}, sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase}};
    semio_plugin::PluginBuilder::<_, semio_plugin::app::NoPluginApp>::new("stdio").label("TXT SQLite Test").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let fixture = native_fixture();
    let source = TxtSnapshot { schema: fixture["literalSchemas"][2].as_str().unwrap().into(), lines: vec!["任意\0\ninside one entity".into(), String::new(), "tail".into()], trailing_newline: true, line_ending: LineEnding::CrLf };
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
    let limits = SqliteDatabaseLimits { max_rows: fixture["controls"]["finalRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let mut phases = Vec::new();
        let bytes = async{let original_file_dialect=&dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |event| { phases.push(event.phase); true };store::io::testing::with_snapshot_file_decoding(store::io::testing::SnapshotFileCaller{grant:original_caller_grant(),native_maximum_bytes:16777216,close_grant:original_caller_grant(),maximum_close_turns:4096},original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;
        assert!(!phases.iter().any(|phase| matches!(phase, SqliteSnapshotPhase::DecodeNative | SqliteSnapshotPhase::EncodeNative)));
        assert_eq!(import_sqlite_database(&bytes, limits, &mut |_| true).unwrap().tables.iter().map(|table| table.rows.len()).sum::<usize>(), limits.max_rows);
        assert_eq!(async{let original_file_dialect=&dialect;let original_file_input=&bytes;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;store::io::testing::with_snapshot_file_encoding(store::io::testing::SnapshotFileCaller{grant:original_caller_grant(),native_maximum_bytes:16777216,close_grant:original_caller_grant(),maximum_close_turns:4096},original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<TxtSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value, source);
        assert!(async{let original_file_dialect=&dialect;let original_file_input=&source;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits { max_rows: limits.max_rows - 1, ..limits };let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_| true;store::io::testing::with_snapshot_file_decoding(store::io::testing::SnapshotFileCaller{grant:original_caller_grant(),native_maximum_bytes:16777216,close_grant:original_caller_grant(),maximum_close_turns:4096},original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.is_err());
    }
}

#[test]
fn sqlite_snapshot_txt_actual_bare_factory_transfers_native_carriers() {
    use store::sqlite_snapshot::SnapshotEncoding;
    let codec = store::bare_artifact_codec::<TxtSnapshot, crate::standards::v_utf_8::subsets::any::schema::mutations::TxtMutation>(crate::STDIO_TXT_DOCUMENT_SCHEMA);
    let capability = codec.snapshot_sqlite.expect("TXT bare owner publishes its semantic SQLite capability");
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
    for case in native_fixture()["nativeCases"].as_array().unwrap() {
        let snapshot = native_case(case);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = native_payload(&snapshot, encoding);
            let database = semio_framework_test_original_snapshot_owners::export_sqlite(&capability,crate::STDIO_TXT_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.table("text_line").unwrap().rows.len(), snapshot.lines.len());
            assert_eq!(semio_framework_test_original_snapshot_owners::import_sqlite(&capability,crate::STDIO_TXT_DOCUMENT_SCHEMA,&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value, payload);
        }
    }
}

#[test]
fn sqlite_snapshot_text_reconstruction_respects_value_budget() {
    let database = TxtSnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(TxtSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,limits)).is_err());
}

#[test]
fn sqlite_snapshot_text_value_preflight_can_be_cancelled() {
    let snapshot = TxtSnapshot { lines: vec![String::new(); 1024], ..TxtSnapshot::default() };
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    let mut checkpoints = 0;
    let error = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| { checkpoints += 1; checkpoints < 2 },limits)).unwrap_err();
    assert!(error.kind == semio_framework_value::ValueRefusalKind::Canceled, "{error}");
}

#[test]
fn sqlite_snapshot_text_lines_preserve_order_and_line_endings() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = TxtSnapshot { schema: fixture["schema"].as_str().unwrap().into(), lines: fixture["lines"].as_array().unwrap().iter().map(|line| line.as_str().unwrap().into()).collect(), trailing_newline: true, line_ending: LineEnding::CrLf };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("text_line").unwrap().rows.len(), 3);
    assert_eq!(database.table("text_document").unwrap().rows[0].values[3], SqliteValue::Text("crlf".into()));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(TxtSnapshot::from_sqlite_database(&reopened,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, fixture);
    let mut broken = reopened;
    broken.table_mut("text_line").unwrap().rows[1].values[2] = SqliteValue::Integer(0);
    assert!(TxtSnapshot::from_sqlite_database(&broken,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false,SqliteDatabaseLimits::default())).is_err());
}


/// 🛫️ Reads the closed language-neutral encoder policy independently of observed receipts.
fn txt_original_encoding_fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛫️encoding/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}

/// 🎟️ Borrows one explicitly authored original grant without deriving authority from demand.
fn txt_original_encoding_grant(law:&serde_json::Value,name:&str)->semio_framework_value::RetainedCloneGrant{serde_json::from_value(law[name].clone()).unwrap()}

/// ♻️ Verifies every actual funded close against System allocation and release.
fn txt_original_encoding_retire<T:semio_framework_value::retirement::RetireOwned>(output:T,law:&serde_json::Value)->(usize,usize){
 let grant=txt_original_encoding_grant(law,"closeGrant");let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(output).map_err(|(error,_)|error).unwrap();let(mut born,mut freed)=(0,0);let mut bank=retirement_bank(grant);
 for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let(step,requested,released)=retire_turn(&mut owner,&mut bank,grant);let step=step.unwrap();assert!(step.progress().fits(grant));assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=requested;freed+=released;}
 assert!(owner.terminal_is_empty());(born,freed)
}

/// 🪢️ Lends the original Work, construction and allocation authorities every genuine encode or decode frame requires.
mod lent{
 use semio_framework_value::{NativeEncodeControl,OriginalRetirementFailure,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
 use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
 use semio_framework_value_testing_admission::{BankWallet,EffectBank,NativeAllocationEvent,NativeAllocationAuthorityPort,NativeAllocationReply,NativeAllocationReplyComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalSourceTurnIdentity,bank_run,driven,finite_work};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 pub const WORK:u64=1<<20;
 pub struct Open(pub RetainedCloneGrant,pub RetainedCloneProgress);
 impl OriginalConstructionRecipient for Open{
  type Context<'t,'s>=() where 's:'t;
  fn validate_context<'t,'s>(&self,_:&())->Result<(),ValueError>where 's:'t{Ok(())}
  fn source_turn<'t,'s>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 's:'t{None}
  fn submitted<'t,'s>(&self,_:&())->RetainedCloneProgress where 's:'t{self.1}
  fn grant<'a,'t,'s>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 's:'t{quote.complete(Ok(self.0))}
  fn accept<'a,'t,'s>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 's:'t{let progress=receipt.progress();let result=self.1.checked_add(progress).map(|total|self.1=total);receipt.complete(result)}
 }
 pub struct Port<F>(pub F);
 impl<F:FnMut(NativeAllocationEvent)->Result<(),ValueError>+Send> NativeAllocationAuthorityPort for Port<F>{
  type Context<'turn,'source>=() where 'source:'turn;
  fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
  fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
  fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{Ok(reply.complete((self.0)(event).map_err(Into::into)))}
  fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{Ok(reply.pending())}
 }
 macro_rules! lend{($name:ident,$control:ident)=>{
  pub fn $name<R>(native:&mut $control<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut $control<'_>)->R)->R{
   let mut work=finite_work(u64::MAX,None);let mut physical=OriginalConstructionAuthority::new(Open(grant,Default::default()));
   work.with_loan(&mut (),|wl|physical.with_loan(&mut (),|cl|native.with_work_recipient(wl,|native|native.with_construction_recipient(cl,|native|Ok::<_,ValueError>(run(native)))))).unwrap().unwrap().unwrap()
  }
 }}
 lend!(encode,NativeEncodeControl);
 lend!(decode,NativeDecodeControl);
 fn invoke<T>(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,call:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->T)->T{let mut call=Some(call);let mut out=None;semio_framework_value_testing_admission::with_original_invocation(None,effect,&mut |effect|out=Some(call.take().expect("fixture invocation ran once")(effect))).unwrap();out.expect("fixture invocation ran")}
 fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("fixture turn stayed pending")}}
 pub type Closed=(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,usize,usize);
 pub fn close_encode(native:&mut NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Closed{
  let mut bank=EffectBank::new(BankWallet::new(grant,WORK));let mut heap=(0,0);
  let turn=settled(driven::<OriginalRetirementFailure>(&mut bank,grant,WORK,|effect,reply,recover|{let(call,observed)=observe(||if recover{native.recover_close_retirement_recipient_into(grant.maximum_copy_bytes,effect,reply)}else{native.close_retirement_recipient_into(grant.maximum_copy_bytes,effect,reply)});heap=(observed.requested_bytes,observed.released_bytes);call}).0);
  (turn,heap.0,heap.1)
 }
 pub fn close_decode(native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Closed{
  let(birth,observed)=observe(||native.with_retirement::<_,ValueError>(grant,|original|Ok(original.requires_original_birth().then(||settled(original.close_original_birth_step(grant))))));
  if let Ok(Some(outcome))=birth{return(outcome,observed.requested_bytes,observed.released_bytes)}
  let mut bank=EffectBank::new(BankWallet::new(grant,WORK));let mut heap=(0,0);
  let turn=settled(bank_run(&mut bank,grant,WORK,|effect|RetainedWorkPoll::Complete(invoke(effect,|effect|{let(poll,observed)=observe(||native.close_retirement_recipient(grant.maximum_copy_bytes,effect));heap=(observed.requested_bytes,observed.released_bytes);settled(poll)}))).0);
  (turn,heap.0,heap.1)
 }
}
/// 🧾️ Takes the original native failure out of one settled stage outcome.
fn stage_failure<T>(stage:store::sqlite_snapshot::SqliteStageFailure<T,store::NativeSnapshotFailure>)->store::NativeSnapshotFailure{
 use store::sqlite_snapshot::SqliteStageFailure as Stage;
 match stage{Stage::Operation(failure)=>failure,Stage::NotStarted(error)=>error.into(),Stage::Settlement{operation,..}|Stage::WorkSettlement{operation,..}=>operation.err().expect("original stage failure retains its operation cause")}
}
/// 📸️ Encodes one snapshot through the supplied original owner and returns its payload or exact original failure.
fn encoded(source:&TxtSnapshot,encoding:SnapshotEncoding,sql:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<store::io_schema::IoPayload,store::NativeSnapshotFailure>{
 let mut output=None;let outcome=source.encode_sqlite_snapshot_native(encoding,sql,owner,&mut output);
 match(output,outcome.result){(Some(Ok(value)),Ok(()))=>Ok(value),(Some(Err(stage)),_)=>Err(stage_failure(stage)),(_,Err(stage))=>Err(stage_failure(stage)),(None,Ok(()))=>panic!("original encode produced no output")}
}
/// 👁️ Reports that an original cause carries a borrowed message, or cannot be inspected.
fn borrowed_message(failure:&store::NativeSnapshotFailure)->bool{match failure.cause(){Some(store::NativeSnapshotCause::Value(value))=>matches!(value.message,std::borrow::Cow::Borrowed(_)),_=>true}}

#[test]
fn txt_original_encoding_borrowed_frontiers_and_canceled_prefix_keep_one_backing(){
 use semio_framework_value::{NativeEncodeControl,RetainedCloneProgress};use std::cell::Cell;use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let text=law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize);let source=TxtSnapshot{lines:vec![text],..Default::default()};let pointer=source.lines[0].as_ptr();let bytes=source.lines[0].len();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{
  let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let extent=bytes+prefix;let cutoff=prefix+law["source"]["cancelAfterBytes"].as_u64().unwrap()as usize;let canceled=Cell::new(false);let mut observe_native=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{let cut=event.total==extent&&event.completed>=cutoff;canceled.set(canceled.get()||cut);!cut};let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observe_native);let mut body=store::NativeSnapshotBodyWallet::new(policy);let mut slot=None;
  let(result,heap)=lent::encode(&mut native,policy,|native|observe(||original_encoding::receive_into(&source,encoding,bytes,&mut slot,native,&mut body)));let error=result.unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message,law["nativeCause"]["message"].as_str().unwrap());assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert!(canceled.get());assert_eq!((heap.requested_bytes,heap.released_bytes),(extent,0));assert_eq!((body.progress().retained_capacity_bytes,body.progress().released_bytes),(extent,0));assert_eq!(error.retained_progress(),body.progress());assert_eq!(body.progress().copied_items,3+usize::from(prefix>0)*3);assert_eq!(body.progress().copied_bytes,std::mem::size_of::<Option<Vec<u8>>>()+std::mem::size_of::<Vec<u8>>()+cutoff);let retained=slot.as_ref().unwrap();assert_eq!(retained.len(),cutoff);assert_eq!(retained.capacity(),extent);assert_eq!(&retained[prefix..],&source.lines[0].as_bytes()[..cutoff-prefix]);if prefix>0{assert_eq!(&retained[..8],law["prefix"]["magic"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>());assert_eq!(u32::from_le_bytes(retained[8..12].try_into().unwrap())as usize,law["prefix"]["token"].as_str().unwrap().len());assert_eq!(&retained[12..prefix],law["prefix"]["token"].as_str().unwrap().as_bytes());}let capacity=retained.capacity();let(born,freed)=txt_original_encoding_retire(slot.take().unwrap(),&law);assert_eq!(capacity+born,freed);assert_eq!(source.lines[0].as_ptr(),pointer);
  for axis in ["items","copy","capacity","depth","native"]{let mut denied=policy;match axis{"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes=0,"capacity"=>denied.maximum_capacity_bytes=0,"depth"=>denied.maximum_depth=0,_=>{}}let mut yes=|_|true;let mut native=NativeEncodeControl::new(if axis=="native"{0}else{law["nativeMaximumBytes"].as_u64().unwrap()as usize},&mut yes);let mut body=store::NativeSnapshotBodyWallet::new(denied);let mut slot=None;let(result,heap)=lent::encode(&mut native,policy,|native|observe(||original_encoding::receive_into(&source,encoding,bytes,&mut slot,native,&mut body)));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes,native.owned_bytes()),(0,0,0));assert_eq!(body.progress(),RetainedCloneProgress::default());assert!(slot.is_none());assert_eq!(source.lines[0].as_ptr(),pointer);}
 }
}

#[test]
fn txt_original_encoding_original_owner_cancellation_and_funded_close_conserve_exact_cause(){
 use semio_framework_value::{NativeEncodeControl,RetainedCloneProgress};use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;use std::sync::atomic::{AtomicUsize,Ordering};
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let close=txt_original_encoding_grant(&law,"closeGrant");let zero=txt_original_encoding_grant(&law,"deniedCloseGrant");let text=law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize);let source=TxtSnapshot{lines:vec![text],..Default::default()};let source_pointer=source.lines[0].as_ptr();let bytes=source.lines[0].len();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{
  let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let extent=bytes+prefix;let cut=prefix+law["source"]["cancelAfterBytes"].as_u64().unwrap()as usize;let blocked=Cell::new(false);let canceled=Cell::new(false);let callback_count=Cell::new(0);let ledger=AtomicUsize::new(0);let mut original=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{callback_count.set(callback_count.get()+1);let hit=event.total==extent&&event.completed>=cut;canceled.set(canceled.get()||hit);!blocked.get()&&!hit};
  let port=|event:semio_framework_value_testing_admission::NativeAllocationEvent|->Result<(),semio_framework_value::ValueError>{use semio_framework_value_testing_admission::NativeAllocationEvent as Event;match event{Event::Admit(request)=>{assert_eq!(request.owned_bytes,ledger.load(Ordering::Relaxed));ledger.store(request.next_owned_bytes,Ordering::Relaxed)},Event::Quote(request)=>assert_eq!(request.owned_bytes,ledger.load(Ordering::Relaxed)),Event::Post{request,actual}=>ledger.store(request.owned_bytes+actual,Ordering::Relaxed),Event::Cancel(_)=>{}}Ok(())};
  let mut authority=semio_framework_value_testing_admission::NativeAllocationAuthority::new(lent::Port(port));let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
  authority.with_send_loan(&mut (),|loan|{
   let mut native=NativeEncodeControl::new_forwarded(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut original,loan);native.install_retirement_recipient(&mut recipient).unwrap();
   let(prior,before,maximum,accepted,error)=lent::encode(&mut native,policy,|native|{
    let prior=native.copy_text("original").unwrap();let before=native.owned_bytes();let maximum=native.maximum_bytes();let mut sql_yes=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_yes,SqliteDatabaseLimits::default());let sql_before=sql.allocation_remaining_bytes();let mut owner=store::NativeSnapshotEncodeOwner::new(native,policy);
    let(result,heap)=observe(||encoded(&source,encoding,&mut sql,&mut owner));let error=result.unwrap_err();let accepted=owner.progress();assert_eq!(owner.grant(),policy);assert!(accepted.fits(policy));assert_eq!(error.retained_progress(),accepted);assert_eq!(error.kind(),semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message(),law["nativeCause"]["message"].as_str().unwrap());assert!(borrowed_message(&error));assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert_eq!(accepted.released_bytes,0);assert!(canceled.get());drop(owner);
    assert!(native.has_retirement_owner());assert_eq!(native.maximum_bytes(),maximum);assert_eq!(native.owned_bytes()-before,ledger.load(Ordering::Relaxed)-before);assert_eq!(sql_before-sql.allocation_remaining_bytes(),native.owned_bytes()-before);
    (prior,before,maximum,accepted,error)
   });
   assert_eq!(prior,"original");assert_eq!(source.lines[0].as_ptr(),source_pointer);let _=before;
   let count=callback_count.get();let(zero_step,zero_born,zero_released)=lent::close_encode(&mut native,zero);let zero_step=zero_step.result.unwrap_or_else(|error|semio_framework_value::RetainedCloneStep::Progress(error.retained_progress()));assert_eq!(zero_step.progress(),RetainedCloneProgress::default());assert_eq!((zero_born,zero_released),(0,0));assert_eq!(callback_count.get(),count);assert!(native.has_retirement_owner());
   blocked.set(true);let(blocked_close,born,released)=lent::close_encode(&mut native,close);let close_error=blocked_close.result.unwrap_err();let primary=close_error.primary().expect("canceled close retains its original cause");assert_eq!(primary.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(primary.message,law["nativeCause"]["message"].as_str().unwrap());assert_eq!((born,released),(0,0));assert!(native.has_retirement_owner());blocked.set(false);let(mut born,mut freed)=(0,0);
   for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,requested,released)=lent::close_encode(&mut native,close);let step=step.result.unwrap();assert!(step.progress().fits(close));assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=requested;freed+=released;}
   assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);assert_eq!(source.lines[0].as_ptr(),source_pointer);assert_eq!(native.maximum_bytes(),maximum);let _=txt_original_encoding_retire(error,&law);drop(native);
  }).unwrap();
  authority.into_port().ok().expect("original allocation authority settled");assert!(!recipient.has_owner());
 }
}

#[test]
fn txt_original_encoding_seven_shapes_publish_exact_bytes_and_sql_cause_with_paid_receipts(){
 use semio_framework_value::NativeEncodeControl;use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");
 for row in law["cases"].as_array().unwrap(){let source=native_case(row);let pointers:Vec<_>=source.lines.iter().map(|line|line.as_ptr()).collect();
  for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let expected=native_payload(&source,encoding);let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut yes=|_|true;let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();
   lent::encode(&mut native,policy,|native|{let mut owner=store::NativeSnapshotEncodeOwner::new(native,policy);let mut sql_yes=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_yes,SqliteDatabaseLimits::default());let sql_before=sql.allocation_remaining_bytes();let(result,heap)=observe(||encoded(&source,encoding,&mut sql,&mut owner));let output=result.unwrap();assert_eq!(output,expected);let accepted=owner.progress();assert!(accepted.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));drop(owner);assert_eq!(sql_before-sql.allocation_remaining_bytes(),native.owned_bytes());let(mut closed_born,mut closed_freed)=(0,0);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,requested,released)=lent::close_encode(native,txt_original_encoding_grant(&law,"closeGrant"));let step=step.result.unwrap();assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));closed_born+=requested;closed_freed+=released;}assert!(!native.has_retirement_owner());let(born,freed)=txt_original_encoding_retire(output,&law);assert_eq!(accepted.retained_capacity_bytes+born+closed_born,accepted.released_bytes+freed+closed_freed);});
   assert_eq!(source.lines.iter().map(|line|line.as_ptr()).collect::<Vec<_>>(),pointers);}
 }
 let source=TxtSnapshot{lines:vec![law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize)],..Default::default()};
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let total=source.lines[0].len()+prefix;let stopped=Cell::new(false);let mut sql_callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{let cut=event.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&event.total==total&&event.completed>=prefix+65536;stopped.set(stopped.get()||cut);!cut};let mut sql=SqliteSnapshotControl::new(&mut sql_callback,SqliteDatabaseLimits::default());let mut yes=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();
  let(accepted,error)=lent::encode(&mut native,policy,|native|{let mut owner=store::NativeSnapshotEncodeOwner::new(native,policy);let(result,heap)=observe(||encoded(&source,encoding,&mut sql,&mut owner));let error=result.unwrap_err();let accepted=owner.progress();assert!(stopped.get());assert_eq!(error.kind(),semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message(),law["sqlCause"]["message"].as_str().unwrap());assert!(borrowed_message(&error));assert_eq!(error.retained_progress(),accepted);assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));drop(owner);(accepted,error)});
  let(mut born,mut freed)=(0,0);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,requested,released)=lent::close_encode(&mut native,txt_original_encoding_grant(&law,"closeGrant"));let step=step.result.unwrap();born+=requested;freed+=released;assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);let _=txt_original_encoding_retire(error,&law);}
}


#[test]
fn txt_original_encoding_frame_refusals_and_borrowed_census_preserve_original_source(){
 use semio_framework_value::{NativeEncodeControl,RetainedCloneProgress};use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let source=native_case(&law["cases"][4]);let pointers:Vec<_>=source.lines.iter().map(|line|line.as_ptr()).collect();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{for axis in law["ownerRefusals"].as_array().unwrap(){let axis=axis.as_str().unwrap();let mut grant=policy;match axis{"items"=>grant.maximum_items=0,"copy"=>grant.maximum_copy_bytes=0,"capacity"=>grant.maximum_capacity_bytes=0,"release"=>grant.maximum_release_bytes=0,"depth"=>grant.maximum_depth=0,_=>{}}let mut yes=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(if axis=="native"{0}else{law["nativeMaximumBytes"].as_u64().unwrap()as usize},&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();let mut sql_yes=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_yes,SqliteDatabaseLimits{max_allocation_bytes:if axis=="sqlAllocation"{0}else{SqliteDatabaseLimits::default().max_allocation_bytes},..Default::default()});
  let(accepted,error,heap)=lent::encode(&mut native,policy,|native|{let mut owner=store::NativeSnapshotEncodeOwner::new(native,grant);let(result,heap)=observe(||encoded(&source,encoding,&mut sql,&mut owner));let error=result.unwrap_err();let accepted=owner.progress();assert!(accepted.fits(grant));assert_eq!(error.retained_progress(),accepted);assert!(borrowed_message(&error));assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert_eq!(accepted.released_bytes,0);drop(owner);(accepted,error,heap)});
  if ["items","capacity","depth","native","sqlAllocation"].contains(&axis){assert_eq!(accepted,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes,native.owned_bytes()),(0,0,0));assert!(!native.has_retirement_owner());}else{assert!(native.has_retirement_owner());let(mut born,mut freed)=(0,0);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,requested,released)=lent::close_encode(&mut native,txt_original_encoding_grant(&law,"closeGrant"));let step=step.result.unwrap();assert!(step.progress().fits(txt_original_encoding_grant(&law,"closeGrant")));assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=requested;freed+=released;}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);}let _=txt_original_encoding_retire(error,&law);assert_eq!(source.lines.iter().map(|line|line.as_ptr()).collect::<Vec<_>>(),pointers);
 }}
 let source=TxtSnapshot{lines:vec![String::new();law["census"]["lines"].as_u64().unwrap()as usize],..Default::default()};let pointer=source.lines.as_ptr();
 for original_cancel in [false,true]{let sql_calls=Cell::new(0);let mut sql_callback=|_|{sql_calls.set(sql_calls.get()+1);true};let mut sql=SqliteSnapshotControl::new(&mut sql_callback,SqliteDatabaseLimits::default());let mut callback=|_|!original_cancel;let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);let mut denied=policy;if !original_cancel{denied.maximum_items=1;}
  lent::encode(&mut native,policy,|native|{let mut owner=store::NativeSnapshotEncodeOwner::new(native,denied);let(result,heap)=observe(||encoded(&source,SnapshotEncoding::Text,&mut sql,&mut owner));let error=result.unwrap_err();assert_eq!(sql_calls.get(),0);assert_eq!(owner.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(owner);let _=txt_original_encoding_retire(error,&law);});assert_eq!(native.owned_bytes(),0);assert_eq!(source.lines.as_ptr(),pointer);}
 let count=Cell::new(0);let mut callback=|_|{count.set(count.get()+1);count.get()<=law["census"]["cancelAfterCheckpoints"].as_u64().unwrap()as usize};let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);let mut body=store::NativeSnapshotBodyWallet::new(policy);let mut slot=None;let(result,heap)=lent::encode(&mut native,policy,|native|observe(||original_encoding::receive_into(&source,SnapshotEncoding::Text,source.lines.len()-1,&mut slot,native,&mut body)));assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(count.get(),law["census"]["cancelAfterCheckpoints"].as_u64().unwrap()as usize+1);assert_eq!(body.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes,native.owned_bytes()),(0,0,0));assert!(slot.is_none());assert_eq!(source.lines.as_ptr(),pointer);
}

#[test]
fn txt_original_encoding_receiving_seam_retains_exact_cause_pointer_and_accepted_output(){
 use semio_framework_value::NativeEncodeControl;use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let close=txt_original_encoding_grant(&law,"closeGrant");let source=TxtSnapshot{lines:vec![law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize)],..Default::default()};let pointer=source.lines[0].as_ptr();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{for after_body in [false,true]{let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let extent=source.lines[0].len()+prefix;let armed=Cell::new(true);let publication=Cell::new(false);let original_cause=Cell::new(0usize);let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{!armed.get()||(!publication.get()&&(after_body||event.total!=extent||event.completed<prefix+65536))};let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();
  let(accepted,error)=lent::encode(&mut native,policy,|native|{let mut owner=store::NativeSnapshotEncodeOwner::new(native,policy);let mut output=None;
   let(result,heap)=observe(||owner.receive::<Vec<u8>,store::io_schema::IoPayload,semio_framework_value::ValueError>(&mut output,&mut |slot,out,_marker,_unwind,native,body|{let result=original_encoding::receive_into(&source,encoding,source.lines[0].len(),slot,native,body);match result{Err(error)=>{original_cause.set(error.message.as_ptr()as usize);semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0}},Ok(payload)=>{publication.set(true);*out=Some(payload);semio_framework_value::RetainedWorkOutcome{result:Ok(()),work:0}}}}));let error=result.result.unwrap_err();let accepted=owner.progress();assert_eq!(error.kind(),semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message(),law["nativeCause"]["message"].as_str().unwrap());assert!(borrowed_message(&error));assert_eq!(error.retained_progress(),accepted);assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert_eq!(accepted.released_bytes,0);if !after_body{assert_ne!(original_cause.get(),0);assert_eq!(error.message().as_ptr()as usize,original_cause.get());}else{assert!(publication.get());assert!(accepted.copied_bytes>=extent);}drop(owner);assert!(native.has_retirement_owner());assert!(output.is_none());(accepted,error)});
  armed.set(false);let(mut born,mut freed)=(0,0);
  for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,requested,released)=lent::close_encode(&mut native,close);let step=step.result.unwrap();assert!(step.progress().fits(close));assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=requested;freed+=released;}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);assert_eq!(source.lines[0].as_ptr(),pointer);let _=txt_original_encoding_retire(error,&law);
 }}
}


#[test]
fn sqlite_snapshot_txt_original_paid_receiving_keeps_every_cancelled_backing_and_actual_line_receipt(){
 use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, retirement::controlled::ControlledRetirement};
 use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 use std::cell::Cell;
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🫴️receiving.json")).unwrap();let policy:RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();let mut successes=0;
 for row in fixture["cases"].as_array().unwrap(){
  let expected=TxtSnapshot::from_body(row["body"].as_str().unwrap());let wire=<TxtSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&expected);let pointer=wire.as_ptr();
  for cut in (0..24).chain(std::iter::once(usize::MAX)){
   let events=Cell::new(0);let allowance=Cell::new(cut);let mut callback=|_|{events.set(events.get()+1);events.get()<=allowance.get()};let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();
   lent::decode(&mut native,policy,|native|{
    let mut owner=store::NativeSnapshotDecodeOwner::new(native,policy);let mut output=None;
    let(outcome,heap)=observe(||<TxtSnapshot as store::ArtifactPackReceiving>::receive_pack(&wire,&mut output,&mut owner));let accepted=owner.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert!(accepted.fits(policy));drop(owner);assert_eq!(wire.as_ptr(),pointer);allowance.set(usize::MAX);let mut born=0;let mut freed=0;
    let failure=match outcome.result{Ok(())=>{successes+=1;assert_eq!(output.as_ref().expect("receiving published its snapshot"),&expected);None},Err(error)=>{assert_eq!(error.retained_progress(),accepted);Some(error)}};
    if let Some(output)=output.take(){let mut retired=ControlledRetirement::new(output).ok().unwrap();let mut bank=retirement_bank(policy);for _ in 0..4096{if retired.terminal_is_empty(){break}let(step,requested,released)=retire_turn(&mut retired,&mut bank,policy);let step=step.unwrap();assert!(step.progress().fits(policy));assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=requested;freed+=released;}assert!(retired.terminal_is_empty());}
    for _ in 0..4096{if !native.has_retirement_owner(){break}let(step,requested,released)=lent::close_decode(native,policy);let step=step.result.unwrap();assert!(step.progress().fits(policy));assert_eq!((requested,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=requested;freed+=released;}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);
    if let Some(failure)=failure{let mut retired=ControlledRetirement::new(failure).map_err(|(error,_)|error).unwrap();let mut bank=retirement_bank(policy);for _ in 0..4096{if retired.terminal_is_empty(){break}let(step,_,_)=retire_turn(&mut retired,&mut bank,policy);step.unwrap();}assert!(retired.terminal_is_empty());}
   });
  }
 }

 for axis in ["items","capacity","depth"]{
  let expected=TxtSnapshot::from_body("a\nb");let wire=<TxtSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&expected);let pointer=wire.as_ptr();let mut denied=policy;match axis{"items"=>denied.maximum_items=0,"capacity"=>denied.maximum_capacity_bytes=0,_=>denied.maximum_depth=0};let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut accept=|_|true;let mut native=NativeDecodeControl::new(1048576,&mut accept);native.install_retirement_recipient(&mut recipient).unwrap();
  lent::decode(&mut native,policy,|native|{let mut owner=store::NativeSnapshotDecodeOwner::new(native,denied);let mut output=None;let(outcome,heap)=observe(||<TxtSnapshot as store::ArtifactPackReceiving>::receive_pack(&wire,&mut output,&mut owner));let error=outcome.result.unwrap_err();assert!(output.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes,owner.progress()),(0,0,RetainedCloneProgress::default()));drop(owner);let mut retired=ControlledRetirement::new(error).map_err(|(error,_)|error).unwrap();let mut bank=retirement_bank(policy);for _ in 0..4096{if retired.terminal_is_empty(){break}retire_turn(&mut retired,&mut bank,policy).0.unwrap();}});assert!(!native.has_retirement_owner());assert_eq!(wire.as_ptr(),pointer);
 }
 assert!(successes>=fixture["cases"].as_array().unwrap().len());
}
