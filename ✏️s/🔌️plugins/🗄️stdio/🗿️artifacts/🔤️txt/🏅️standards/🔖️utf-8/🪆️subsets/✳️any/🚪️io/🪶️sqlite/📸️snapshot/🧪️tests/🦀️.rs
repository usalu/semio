
/// 🎟️ Supplies the immutable caller policy independently of every observed frontier.
fn original_caller_grant()->semio_framework_value::RetainedCloneGrant{
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🫴️receiving.json")).unwrap();serde_json::from_value(fixture["originalCallerGrant"].clone()).unwrap()
}

/// 🛬️ Preserves this test's original SQL controller and retained native recipient through decode.
fn with_original_decode<O>(sql:&mut SqliteSnapshotControl<'_>,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>,&mut store::NativeSnapshotDecodeOwner<'_,'_>)->O)->O{
 let policy=original_caller_grant();let mut accepted=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=semio_framework_value::NativeDecodeControl::new(sql.limits().max_value_bytes,&mut accepted);native.install_retirement_recipient(&mut recipient).unwrap();
 let result={let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,policy);operation(sql,&mut owner)};
 for _ in 0..4096{if !native.has_retirement_owner(){break}let step=native.close_retirement_recipient(policy).unwrap();assert!(step.progress().fits(policy));}assert!(!native.has_retirement_owner());result
}

/// 🛫️ Supplies the actual encode owner without claiming cold producer receipt conservation.
fn with_original_encode<O>(sql:&mut SqliteSnapshotControl<'_>,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>,&mut store::NativeSnapshotEncodeOwner<'_,'_>)->O)->O{
 let policy=original_caller_grant();let mut accepted=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=semio_framework_value::NativeEncodeControl::new(sql.limits().max_value_bytes,&mut accepted);native.install_retirement_recipient(&mut recipient).unwrap();
 let result={let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,policy);operation(sql,&mut owner)};
 for _ in 0..4096{if !native.has_retirement_owner(){break}let step=native.close_retirement_recipient(policy).unwrap();assert!(step.progress().fits(policy));}assert!(!native.has_retirement_owner());result
}

/// 📸️ Invokes only the genuine supplied-owner typed decode definition.
fn decode_original(payload:&store::io_schema::IoPayload,sql:&mut SqliteSnapshotControl<'_>)->Result<TxtSnapshot,semio_framework_value::ValueError>{with_original_decode(sql,|sql,owner|TxtSnapshot::decode_sqlite_snapshot_native(payload,sql,owner))}

/// 📸️ Invokes only the current supplied-owner typed encode definition.
fn encode_original(snapshot:&TxtSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,sql:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{with_original_encode(sql,|sql,owner|snapshot.encode_sqlite_snapshot_native(encoding,sql,owner))}

use crate::standards::v_utf_8::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn native_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🛂️native.json")).unwrap()
}

fn native_case(value: &serde_json::Value) -> TxtSnapshot {
    TxtSnapshot { schema: crate::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: value["lines"].as_array().unwrap().iter().map(|line| line.as_str().unwrap().into()).collect(), trailing_newline: value["trailingNewline"].as_bool().unwrap(), line_ending: if value["lineEnding"] == "crLf" { LineEnding::CrLf } else { LineEnding::Lf } }
}

fn native_payload(snapshot: &TxtSnapshot, encoding: store::sqlite_snapshot::SnapshotEncoding) -> store::io_schema::IoPayload {
    match encoding { store::sqlite_snapshot::SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(<TxtSnapshot as store::ArtifactPack>::encode_pack(snapshot)), store::sqlite_snapshot::SnapshotEncoding::Text => store::io_schema::IoPayload::Text(<TxtSnapshot as store::ArtifactDsl>::print_dsl(snapshot)) }
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
    println!("[DEBUG] TXT controlled native codecs retain the authored raw text and pack carriers");
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
            assert!(interior && error.kind == semio_framework_value::ValueRefusalKind::Canceled, "{phase:?}: {error}");
        }
    }
    println!("[DEBUG] TXT native transfer admits exact file and domain row bounds and cancels during large copies");
}

#[test]
fn sqlite_snapshot_txt_projection_and_reconstruction_cancel_inside_long_literal_fields() {
    use store::sqlite_snapshot::SqliteSnapshotPhase;
    let fixture = native_fixture();
    let text = fixture["controls"]["longText"].as_str().unwrap().repeat(fixture["controls"]["longRepeat"].as_u64().unwrap() as usize);
    for schema_field in [false, true] {
        let snapshot = if schema_field { TxtSnapshot { schema: text.clone(), ..TxtSnapshot::default() } } else { TxtSnapshot { lines: vec![text.clone()], ..TxtSnapshot::default() } };
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
            let mut interior = false;
            let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { let cancel = event.phase == phase && event.completed >= fixture["controls"]["cancelAtBytes"].as_u64().unwrap() as usize && event.total > event.completed; interior |= cancel; !cancel };
            let result = if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()) } else { TxtSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()) };
            let error = result.unwrap_err();
            assert!(interior && error.kind == semio_framework_value::ValueRefusalKind::Canceled, "{phase:?}: {error}");
        }
    }
    println!("[DEBUG] TXT SQL copies cancel inside both document schema and line content");
}

#[test]
fn sqlite_snapshot_txt_surrogate_ids_and_literal_metadata_are_not_native_headers() {
    let fixture = native_fixture();
    for schema in fixture["literalSchemas"].as_array().unwrap() {
        let snapshot = TxtSnapshot { schema: schema.as_str().unwrap().into(), lines: vec!["任意\0\nembedded".into(), String::new()], ..TxtSnapshot::default() };
        let mut database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let key = fixture["controls"]["rootIdentity"].as_i64().unwrap();
        database.table_mut("text_document").unwrap().rows[0].rowid = key;
        database.table_mut("text_document").unwrap().rows[0].values[0] = SqliteValue::Integer(key);
        for row in &mut database.table_mut("text_line").unwrap().rows { row.values[1] = SqliteValue::Integer(key); }
        assert_eq!(TxtSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
        let valid = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
        assert!(snapshot.validate_sqlite_snapshot_subset(&valid, &database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_ok());
        for invalid in [semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.csv".into(), ..valid.clone() }, semio_framework_artifact_reference::ArtifactDialect { standard: "other".into(), ..valid.clone() }, semio_framework_artifact_reference::ArtifactDialect { subset: "other".into(), ..valid.clone() }] {
            assert!(snapshot.validate_sqlite_snapshot_subset(&invalid, &database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
        }
        for corruption in 0..6 {
            let mut broken = database.clone();
            match corruption { 0 => broken.table_mut("text_document").unwrap().rows[0].rowid = -1, 1 => broken.table_mut("text_document").unwrap().rows[0].values[0] = SqliteValue::Integer(key + 1), 2 => broken.table_mut("text_document").unwrap().rows[0].values.push(SqliteValue::Null), 3 => broken.table_mut("text_line").unwrap().rows[0].values[1] = SqliteValue::Integer(key + 1), 4 => broken.table_mut("text_line").unwrap().rows[0].values[2] = SqliteValue::Integer(-1), _ => broken.table_mut("text_line").unwrap().rows[0].values.push(SqliteValue::Null) }
            assert!(TxtSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
        }
    }
    println!("[DEBUG] TXT semantic SQL restores arbitrary literal metadata and positive independent surrogate identities");
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_txt_actual_typed_io_preserves_fields_without_native_phases() {
    use store::{io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}, sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("TXT SQLite Test").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let fixture = native_fixture();
    let source = TxtSnapshot { schema: fixture["literalSchemas"][2].as_str().unwrap().into(), lines: vec!["任意\0\ninside one entity".into(), String::new(), "tail".into()], trailing_newline: true, line_ending: LineEnding::CrLf };
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
    let limits = SqliteDatabaseLimits { max_rows: fixture["controls"]["finalRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let mut phases = Vec::new();
        let bytes = io_export_sqlite_snapshot(&dialect, &source, encoding, limits, &mut |event| { phases.push(event.phase); true }).await.unwrap().value;
        assert!(!phases.iter().any(|phase| matches!(phase, SqliteSnapshotPhase::DecodeNative | SqliteSnapshotPhase::EncodeNative)));
        assert_eq!(import_sqlite_database(&bytes, limits, &mut |_| true).unwrap().tables.iter().map(|table| table.rows.len()).sum::<usize>(), limits.max_rows);
        assert_eq!(io_import_sqlite_snapshot::<TxtSnapshot>(&dialect, &bytes, limits, &mut |_| true).await.unwrap().value, source);
        assert!(io_export_sqlite_snapshot(&dialect, &source, encoding, SqliteDatabaseLimits { max_rows: limits.max_rows - 1, ..limits }, &mut |_| true).await.is_err());
    }
    println!("[DEBUG] TXT actual typed I/O preserves every persisted field independently of external carrier limitations");
}

#[test]
fn sqlite_snapshot_txt_actual_bare_factory_transfers_native_carriers() {
    use store::sqlite_snapshot::SnapshotEncoding;
    let codec = store::ArtifactCodec::bare::<TxtSnapshot, crate::standards::v_utf_8::subsets::any::schema::mutations::TxtMutation>(crate::STDIO_TXT_DOCUMENT_SCHEMA);
    let capability = codec.snapshot_sqlite.expect("TXT bare owner publishes its semantic SQLite capability");
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
    for case in native_fixture()["nativeCases"].as_array().unwrap() {
        let snapshot = native_case(case);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = native_payload(&snapshot, encoding);
            let database = with_original_decode(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()),|sql,owner|(capability.export)(crate::STDIO_TXT_DOCUMENT_SCHEMA,&dialect,&payload,sql,owner)).unwrap().value;
            assert_eq!(database.table("text_line").unwrap().rows.len(), snapshot.lines.len());
            assert_eq!(with_original_encode(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()),|sql,owner|(capability.import)(crate::STDIO_TXT_DOCUMENT_SCHEMA,&dialect,&mut Some(database),encoding,sql,owner)).unwrap().value, payload);
        }
    }
    println!("[DEBUG] TXT actual erased bare codec transfers queryable entities in both native encodings");
}

#[test]
fn sqlite_snapshot_text_reconstruction_respects_value_budget() {
    let database = TxtSnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(TxtSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}

#[test]
fn sqlite_snapshot_text_value_preflight_can_be_cancelled() {
    let snapshot = TxtSnapshot { lines: vec![String::new(); 1024], ..TxtSnapshot::default() };
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    let mut checkpoints = 0;
    let error = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| { checkpoints += 1; checkpoints < 2 }, limits)).unwrap_err();
    assert!(error.kind == semio_framework_value::ValueRefusalKind::Canceled, "{error}");
}

#[test]
fn sqlite_snapshot_text_lines_preserve_order_and_line_endings() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = TxtSnapshot { schema: fixture["schema"].as_str().unwrap().into(), lines: fixture["lines"].as_array().unwrap().iter().map(|line| line.as_str().unwrap().into()).collect(), trailing_newline: true, line_ending: LineEnding::CrLf };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("text_line").unwrap().rows.len(), 3);
    assert_eq!(database.table("text_document").unwrap().rows[0].values[3], SqliteValue::Text("crlf".into()));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(TxtSnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, fixture);
    let mut broken = reopened;
    broken.table_mut("text_line").unwrap().rows[1].values[2] = SqliteValue::Integer(0);
    assert!(TxtSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}


/// 🛫️ Reads the closed language-neutral encoder policy independently of observed receipts.
fn txt_original_encoding_fixture()->serde_json::Value{serde_json::from_str(include_str!("../🛫️encoding/🧫️fixtures/🔣️.json")).unwrap()}

/// 🎟️ Borrows one explicitly authored original grant without deriving authority from demand.
fn txt_original_encoding_grant(law:&serde_json::Value,name:&str)->semio_framework_value::RetainedCloneGrant{serde_json::from_value(law[name].clone()).unwrap()}

/// ♻️ Verifies every actual funded close against System allocation and release.
fn txt_original_encoding_retire<T:semio_framework_value::retirement::RetireOwned>(output:T,law:&serde_json::Value)->(usize,usize){
 let grant=txt_original_encoding_grant(law,"closeGrant");let mut owner=semio_framework_value::retirement::ControlledRetirement::new(output).ok().unwrap();let(mut born,mut freed)=(0,0);
 for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}
 assert!(owner.terminal_is_empty());(born,freed)
}

#[test]
fn txt_original_encoding_borrowed_frontiers_and_canceled_prefix_keep_one_backing(){
 use semio_framework_value::{NativeEncodeControl,RetainedCloneProgress};use std::cell::Cell;use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let text=law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize);let source=TxtSnapshot{lines:vec![text],..Default::default()};let pointer=source.lines[0].as_ptr();let bytes=source.lines[0].len();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{
  let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let extent=bytes+prefix;let cutoff=prefix+law["source"]["cancelAfterBytes"].as_u64().unwrap()as usize;let canceled=Cell::new(false);let mut observe_native=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{let cut=event.total==extent&&event.completed>=cutoff;canceled.set(canceled.get()||cut);!cut};let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observe_native);let mut body=store::NativeSnapshotBodyWallet::new(policy);let mut slot=None;
  let(result,heap)=observe(||original_encoding::receive_into(&source,encoding,bytes,&mut slot,&mut native,&mut body));let error=result.unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message,law["nativeCause"]["message"].as_str().unwrap());assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert!(canceled.get());assert_eq!((heap.requested_bytes,heap.released_bytes),(extent,0));assert_eq!((body.progress().retained_capacity_bytes,body.progress().released_bytes),(extent,0));assert_eq!(error.retained_progress(),body.progress());assert_eq!(body.progress().copied_items,3+usize::from(prefix>0)*3);assert_eq!(body.progress().copied_bytes,std::mem::size_of::<Option<Vec<u8>>>()+std::mem::size_of::<Vec<u8>>()+cutoff);let retained=slot.as_ref().unwrap();assert_eq!(retained.len(),cutoff);assert_eq!(retained.capacity(),extent);assert_eq!(&retained[prefix..],&source.lines[0].as_bytes()[..cutoff-prefix]);if prefix>0{assert_eq!(&retained[..8],law["prefix"]["magic"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>());assert_eq!(u32::from_le_bytes(retained[8..12].try_into().unwrap())as usize,law["prefix"]["token"].as_str().unwrap().len());assert_eq!(&retained[12..prefix],law["prefix"]["token"].as_str().unwrap().as_bytes());}let capacity=retained.capacity();let(born,freed)=txt_original_encoding_retire(slot.take().unwrap(),&law);assert_eq!(capacity+born,freed);assert_eq!(source.lines[0].as_ptr(),pointer);
  for axis in ["items","copy","capacity","depth","native"]{let mut denied=policy;match axis{"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes=0,"capacity"=>denied.maximum_capacity_bytes=0,"depth"=>denied.maximum_depth=0,_=>{}}let mut yes=|_|true;let mut native=NativeEncodeControl::new(if axis=="native"{0}else{law["nativeMaximumBytes"].as_u64().unwrap()as usize},&mut yes);let mut body=store::NativeSnapshotBodyWallet::new(denied);let mut slot=None;let(result,heap)=observe(||original_encoding::receive_into(&source,encoding,bytes,&mut slot,&mut native,&mut body));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes,native.owned_bytes()),(0,0,0));assert_eq!(body.progress(),RetainedCloneProgress::default());assert!(slot.is_none());assert_eq!(source.lines[0].as_ptr(),pointer);}
 }
 eprintln!("[DEBUG] Original TXT encoding borrowed item/copy/capacity/depth/native refusals are before body birth; canceled140000-byte Unicode retains exact65536-byte body prefix and one29-byte binary envelope backing");
}

#[test]
fn txt_original_encoding_original_owner_cancellation_and_funded_close_conserve_exact_cause(){
 use semio_framework_value::{NativeEncodeControl,RetainedCloneProgress};use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let close=txt_original_encoding_grant(&law,"closeGrant");let zero=txt_original_encoding_grant(&law,"deniedCloseGrant");let text=law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize);let source=TxtSnapshot{lines:vec![text],..Default::default()};let source_pointer=source.lines[0].as_ptr();let bytes=source.lines[0].len();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{
  let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let extent=bytes+prefix;let cut=prefix+law["source"]["cancelAfterBytes"].as_u64().unwrap()as usize;let blocked=Cell::new(false);let canceled=Cell::new(false);let callback_count=Cell::new(0);let ledger=Cell::new(0);let mut original=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{callback_count.set(callback_count.get()+1);let hit=event.total==extent&&event.completed>=cut;canceled.set(canceled.get()||hit);!blocked.get()&&!hit};let mut allocation=|event:semio_framework_value::native_encoding::NativeEncodeAllocation|{assert_eq!(event.owned_bytes,ledger.get());ledger.set(event.next_owned_bytes);Ok(())};let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new_forwarded(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut original,&mut allocation);native.install_retirement_recipient(&mut recipient).unwrap();let prior=native.copy_text("original").unwrap();let before=native.owned_bytes();let maximum=native.maximum_bytes();let mut sql_yes=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_yes,SqliteDatabaseLimits::default());let sql_before=sql.allocation_remaining_bytes();let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,policy);
  let(result,heap)=observe(||source.encode_sqlite_snapshot_native(encoding,&mut sql,&mut owner));let error=result.unwrap_err();let accepted=owner.progress();assert_eq!(owner.grant(),policy);assert!(accepted.fits(policy));assert_eq!(error.retained_progress(),accepted);assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message,law["nativeCause"]["message"].as_str().unwrap());assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert_eq!(accepted.released_bytes,0);assert!(canceled.get());drop(owner);assert!(native.has_retirement_owner());assert_eq!(native.maximum_bytes(),maximum);assert_eq!(native.owned_bytes()-before,ledger.get()-before);assert_eq!(sql_before-sql.allocation_remaining_bytes(),native.owned_bytes()-before);assert_eq!(prior,"original");assert_eq!(source.lines[0].as_ptr(),source_pointer);
  let count=callback_count.get();let(zero_step,zero_heap)=observe(||native.close_retirement_recipient(zero).unwrap());assert_eq!(zero_step.progress(),RetainedCloneProgress::default());assert_eq!((zero_heap.requested_bytes,zero_heap.released_bytes),(0,0));assert_eq!(callback_count.get(),count);assert!(native.has_retirement_owner());blocked.set(true);let(result,heap)=observe(||native.close_retirement_recipient(close));let close_error=result.unwrap_err();assert_eq!(close_error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(close_error.message,law["nativeCause"]["message"].as_str().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(native.has_retirement_owner());blocked.set(false);let(mut born,mut freed)=(0,0);
  for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,heap)=observe(||native.close_retirement_recipient(close).unwrap());assert!(step.progress().fits(close));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);assert_eq!(source.lines[0].as_ptr(),source_pointer);assert_eq!(native.maximum_bytes(),maximum);drop(native);assert!(!recipient.has_owner());
 }
 eprintln!("[DEBUG] Original TXT typed encoder conserves actual System receipts, original forwarded allocation ledger, same cancellation Cells, exact borrowed cause and funded retained close in both encodings");
}

#[test]
fn txt_original_encoding_seven_shapes_publish_exact_bytes_and_sql_cause_with_paid_receipts(){
 use semio_framework_value::NativeEncodeControl;use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");
 for row in law["cases"].as_array().unwrap(){let source=native_case(row);let pointers:Vec<_>=source.lines.iter().map(|line|line.as_ptr()).collect();
  for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let expected=native_payload(&source,encoding);let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut yes=|_|true;let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,policy);let mut sql_yes=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_yes,SqliteDatabaseLimits::default());let sql_before=sql.allocation_remaining_bytes();let(result,heap)=observe(||source.encode_sqlite_snapshot_native(encoding,&mut sql,&mut owner));let output=result.unwrap();assert_eq!(output,expected);let accepted=owner.progress();assert!(accepted.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));drop(owner);assert_eq!(sql_before-sql.allocation_remaining_bytes(),native.owned_bytes());assert!(!native.has_retirement_owner());let(born,freed)=txt_original_encoding_retire(output,&law);assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);assert_eq!(source.lines.iter().map(|line|line.as_ptr()).collect::<Vec<_>>(),pointers);}
 }
 let source=TxtSnapshot{lines:vec![law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize)],..Default::default()};
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let total=source.lines[0].len()+prefix;let stopped=Cell::new(false);let mut sql_callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{let cut=event.phase==store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative&&event.total==total&&event.completed>=prefix+65536;stopped.set(stopped.get()||cut);!cut};let mut sql=SqliteSnapshotControl::new(&mut sql_callback,SqliteDatabaseLimits::default());let mut yes=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,policy);let(result,heap)=observe(||source.encode_sqlite_snapshot_native(encoding,&mut sql,&mut owner));let error=result.unwrap_err();let accepted=owner.progress();assert!(stopped.get());assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message,law["sqlCause"]["message"].as_str().unwrap());assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!(error.retained_progress(),accepted);assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));drop(owner);let(mut born,mut freed)=(0,0);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,heap)=observe(||native.close_retirement_recipient(txt_original_encoding_grant(&law,"closeGrant")).unwrap());born+=heap.requested_bytes;freed+=heap.released_bytes;assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);}
 eprintln!("[DEBUG] Original TXT seven raw-line edge shapes match independent authored bytes and ordinary Semio carriers; SQL cancellation preserves its exact typed cause and full receiving receipts");
}


#[test]
fn txt_original_encoding_frame_refusals_and_borrowed_census_preserve_original_source(){
 use semio_framework_value::{NativeEncodeControl,RetainedCloneProgress};use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let source=native_case(&law["cases"][4]);let pointers:Vec<_>=source.lines.iter().map(|line|line.as_ptr()).collect();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{for axis in law["ownerRefusals"].as_array().unwrap(){let axis=axis.as_str().unwrap();let mut grant=policy;match axis{"items"=>grant.maximum_items=0,"copy"=>grant.maximum_copy_bytes=0,"capacity"=>grant.maximum_capacity_bytes=0,"release"=>grant.maximum_release_bytes=0,"depth"=>grant.maximum_depth=0,_=>{}}let mut yes=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(if axis=="native"{0}else{law["nativeMaximumBytes"].as_u64().unwrap()as usize},&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,grant);let mut sql_yes=|_|true;let mut sql=SqliteSnapshotControl::new(&mut sql_yes,SqliteDatabaseLimits{max_allocation_bytes:if axis=="sqlAllocation"{0}else{SqliteDatabaseLimits::default().max_allocation_bytes},..Default::default()});let(result,heap)=observe(||source.encode_sqlite_snapshot_native(encoding,&mut sql,&mut owner));let error=result.unwrap_err();let accepted=owner.progress();assert!(accepted.fits(grant));assert_eq!(error.retained_progress(),accepted);assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert_eq!(accepted.released_bytes,0);drop(owner);
  if ["items","capacity","depth","native","sqlAllocation"].contains(&axis){assert_eq!(accepted,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes,native.owned_bytes()),(0,0,0));assert!(!native.has_retirement_owner());}else{assert!(native.has_retirement_owner());let(mut born,mut freed)=(0,0);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,heap)=observe(||native.close_retirement_recipient(txt_original_encoding_grant(&law,"closeGrant")).unwrap());assert!(step.progress().fits(txt_original_encoding_grant(&law,"closeGrant")));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);}assert_eq!(source.lines.iter().map(|line|line.as_ptr()).collect::<Vec<_>>(),pointers);
 }}
 let source=TxtSnapshot{lines:vec![String::new();law["census"]["lines"].as_u64().unwrap()as usize],..Default::default()};let pointer=source.lines.as_ptr();
 for original_cancel in [false,true]{let sql_calls=Cell::new(0);let mut sql_callback=|_|{sql_calls.set(sql_calls.get()+1);true};let mut sql=SqliteSnapshotControl::new(&mut sql_callback,SqliteDatabaseLimits::default());let mut callback=|_|!original_cancel;let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);let mut denied=policy;if !original_cancel{denied.maximum_items=1;}let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,denied);let(result,heap)=observe(||source.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut sql,&mut owner));assert!(result.is_err());assert_eq!(sql_calls.get(),0);assert_eq!(owner.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(owner);assert_eq!(native.owned_bytes(),0);assert_eq!(source.lines.as_ptr(),pointer);}
 let count=Cell::new(0);let mut callback=|_|{count.set(count.get()+1);count.get()<=law["census"]["cancelAfterCheckpoints"].as_u64().unwrap()as usize};let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);let mut body=store::NativeSnapshotBodyWallet::new(policy);let mut slot=None;let(result,heap)=observe(||original_encoding::receive_into(&source,SnapshotEncoding::Text,source.lines.len()-1,&mut slot,&mut native,&mut body));assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(count.get(),law["census"]["cancelAfterCheckpoints"].as_u64().unwrap()as usize+1);assert_eq!(body.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes,native.owned_bytes()),(0,0,0));assert!(slot.is_none());assert_eq!(source.lines.as_ptr(),pointer);
 eprintln!("[DEBUG] Original TXT full five-field frame and zero native/SQL allowances distinguish prebirth from retained copy/publication refusals;8192empty-line census honors same callback before any backing");
}

#[test]
fn txt_original_encoding_receiving_seam_retains_exact_cause_pointer_and_accepted_output(){
 use semio_framework_value::NativeEncodeControl;use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use std::cell::Cell;
 let law=txt_original_encoding_fixture();let policy=txt_original_encoding_grant(&law,"bodyGrant");let close=txt_original_encoding_grant(&law,"closeGrant");let source=TxtSnapshot{lines:vec![law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize)],..Default::default()};let pointer=source.lines[0].as_ptr();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{for after_body in [false,true]{let prefix=if encoding==SnapshotEncoding::Binary{law["prefix"]["bytes"].as_u64().unwrap()as usize}else{0};let extent=source.lines[0].len()+prefix;let armed=Cell::new(true);let publication=Cell::new(false);let original_cause=Cell::new(0usize);let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{!armed.get()||(!publication.get()&&(after_body||event.total!=extent||event.completed<prefix+65536))};let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,policy);
  let(result,heap)=observe(||owner.receive::<Vec<u8>,store::io_schema::IoPayload>(|slot,native,body|{let result=original_encoding::receive_into(&source,encoding,source.lines[0].len(),slot,native,body);match &result{Err(error)=>original_cause.set(error.message.as_ptr()as usize),Ok(_)=>publication.set(true)}result}));let error=result.unwrap_err();let accepted=owner.progress();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(error.message,law["nativeCause"]["message"].as_str().unwrap());assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!(error.retained_progress(),accepted);assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert_eq!(accepted.released_bytes,0);if !after_body{assert_ne!(original_cause.get(),0);assert_eq!(error.message.as_ptr()as usize,original_cause.get());}else{assert!(publication.get());assert!(accepted.copied_bytes>=extent);}drop(owner);assert!(native.has_retirement_owner());armed.set(false);let(mut born,mut freed)=(0,0);
  for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,heap)=observe(||native.close_retirement_recipient(close).unwrap());assert!(step.progress().fits(close));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);assert_eq!(source.lines[0].as_ptr(),pointer);
 }}
 eprintln!("[DEBUG] Original TXT genuine binder-to-owner seam preserves exact borrowed cause pointer through receipt promotion and retains already accepted IoPayload before original close, then rearms the same Cell");
}

#[global_allocator]
static TXT_ORIGINAL_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[test]
fn sqlite_snapshot_txt_original_paid_receiving_keeps_every_cancelled_backing_and_actual_line_receipt(){
 use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,retirement::ControlledRetirement};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 use std::cell::Cell;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🫴️receiving.json")).unwrap();let policy:RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();let mut successes=0;
 for row in fixture["cases"].as_array().unwrap(){
  let expected=TxtSnapshot::from_body(row["body"].as_str().unwrap());let wire=<TxtSnapshot as store::ArtifactPack>::encode_pack(&expected);let pointer=wire.as_ptr();
  for cut in (0..24).chain(std::iter::once(usize::MAX)){
   let events=Cell::new(0);let allowance=Cell::new(cut);let mut callback=|_|{events.set(events.get()+1);events.get()<=allowance.get()};let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,policy);
   let(result,heap)=observe(||<TxtSnapshot as store::ArtifactPackReceiving>::receive_pack(&wire,&mut owner));let accepted=owner.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(accepted.retained_capacity_bytes,accepted.released_bytes));assert!(accepted.fits(policy));drop(owner);assert_eq!(wire.as_ptr(),pointer);allowance.set(usize::MAX);let mut born=0;let mut freed=0;
   match result{Ok(output)=>{successes+=1;assert_eq!(output,expected);let mut retired=ControlledRetirement::new(output).ok().unwrap();for _ in 0..4096{if retired.terminal_is_empty(){break}let(step,heap)=observe(||retired.step(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(retired.terminal_is_empty());},Err(error)=>assert_eq!(error.retained_progress(),accepted)}
   for _ in 0..4096{if !native.has_retirement_owner(){break}let(step,heap)=observe(||native.close_retirement_recipient(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(!native.has_retirement_owner());assert_eq!(accepted.retained_capacity_bytes+born,accepted.released_bytes+freed);
  }
 }

 for axis in ["items","capacity","depth"]{
  let expected=TxtSnapshot::from_body("a\nb");let wire=<TxtSnapshot as store::ArtifactPack>::encode_pack(&expected);let pointer=wire.as_ptr();let mut denied=policy;match axis{"items"=>denied.maximum_items=0,"capacity"=>denied.maximum_capacity_bytes=0,_=>denied.maximum_depth=0};let mut accept=|_|true;let mut native=NativeDecodeControl::new(1048576,&mut accept);let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,denied);let(result,heap)=observe(||<TxtSnapshot as store::ArtifactPackReceiving>::receive_pack(&wire,&mut owner));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes,owner.progress()),(0,0,RetainedCloneProgress::default()));drop(owner);assert!(!native.has_retirement_owner());assert_eq!(wire.as_ptr(),pointer);
 }
 assert!(successes>=fixture["cases"].as_array().unwrap().len());
 eprintln!("[DEBUG] Original Txt receiving fixed plain grant conserves System allocation/release and caller wire through24 cancellation cuts and actual LF/CRLF/empty/Unicode lines");
}
