use crate::standards::iana::subsets::any::io::sqlite::snapshot::*;
use crate::standards::iana::subsets::any::io::{binary::snapshot::read_tsv_source_binary, text::snapshot::read_tsv_source_text};
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

#[global_allocator]
static TSV_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[test]
fn sqlite_snapshot_tsv_original_typed_destination_retains_cancelled_children(){
 use semio_framework_dsl_record::{FieldValue,RecordValue};use semio_framework_value::{NativeDecodeControl,native_decoding::NativeDecodeProgress,retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let fixture=tsv_native_control_fixture();let law=&fixture["typedDestination"];let source=fixture["copyTextUnit"].as_str().unwrap().repeat(fixture["copyTextRepeat"].as_u64().unwrap()as usize);let normal:RetainedCloneGrant=serde_json::from_value(law["normalGrant"].clone()).unwrap();let close:RetainedCloneGrant=serde_json::from_value(law["closeGrant"].clone()).unwrap();
 let record=RecordValue{fields:[(0,FieldValue::Text(fixture["ownedSchema"].as_str().unwrap().into())),(1,FieldValue::List(vec![FieldValue::List(vec![]),FieldValue::List(vec![FieldValue::Text(source.clone()),FieldValue::Text("tail".into())]),FieldValue::List(vec![])])),(2,FieldValue::Bool(true)),(3,FieldValue::Enum(1))].into_iter().collect()};
 let demand=admission::binding_demands(&record).unwrap();for refused in [RetainedCloneGrant{maximum_items:demand.maximum_items-1,..normal},RetainedCloneGrant{maximum_copy_bytes:demand.maximum_copy_bytes-1,..normal},RetainedCloneGrant{maximum_capacity_bytes:demand.maximum_capacity_bytes-1,..normal},RetainedCloneGrant{maximum_depth:demand.maximum_depth-1,..normal}]{let mut callback=|_|true;let mut native=NativeDecodeControl::new(normal.maximum_capacity_bytes,&mut callback);let mut wallet=semio_framework_os_kernel::NativeSnapshotBodyWallet::new(refused);let mut output=None;let(result,heap)=observe(||admission::bind(&record,&mut output,&mut native,&mut wallet));assert!(result.is_err());assert!(output.is_none());assert_eq!(wallet.progress(),Default::default());assert_eq!(native.owned_bytes(),0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
 for cancel in law["cancelAt"].as_array().unwrap(){let cancel=cancel.as_u64().map(|n|n as usize);let mut callback=|event:NativeDecodeProgress|event.total!=source.len()||cancel.is_none_or(|at|event.completed<at);let mut native=NativeDecodeControl::new(normal.maximum_capacity_bytes,&mut callback);native.begin_stage(17).unwrap();native.advance(3).unwrap();let mut output=None;let mut wallet=semio_framework_os_kernel::NativeSnapshotBodyWallet::new(normal);
 let(result,heap)=observe(||admission::bind(&record,&mut output,&mut native,&mut wallet));let progress=wallet.progress();assert!(progress.fits(normal));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));let snapshot=output.as_ref().expect("actual destination must precede child allocation");assert_eq!(snapshot.schema,fixture["ownedSchema"].as_str().unwrap());assert!(snapshot.trailing_newline);assert_eq!(snapshot.line_ending,LineEnding::Crlf);let prefix=&snapshot.records[1][0];assert!(source.starts_with(prefix));assert!(source.is_char_boundary(prefix.len()));assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(prefix).unwrap()).unwrap(),source[..prefix.len()]);assert_eq!(heap.released_bytes,0);assert!(heap.requested_bytes<=normal.maximum_capacity_bytes);assert!(snapshot.schema.len()+prefix.len()<=normal.maximum_copy_bytes);if cancel.is_some(){assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(snapshot.records.len(),2);assert_eq!(snapshot.records[1].len(),1);}else{result.unwrap();assert_eq!(snapshot.records,vec![Vec::<String>::new(),vec![source.clone(),"tail".into()],Vec::new()]);}
 native.advance(14).unwrap();drop(native);let prefix_bytes=prefix.len();let mut owner=ControlledRetirement::new(output.take().unwrap()).map_err(|(error,_)|error).unwrap();let(mut born,mut released)=(heap.requested_bytes,0);for _ in 0..law["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break;}let(step,heap)=observe(||owner.step(close).unwrap());assert!(step.progress().fits(close));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}assert!(owner.terminal_is_empty());assert_eq!(born,released);let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,law["terminalDropBytes"].as_u64().unwrap()as usize));println!("[DEBUG] TSV original typed destination cancelAt={cancel:?} fieldPrefix={prefix_bytes} born={born} released={released} parentStage17 SerdePrefix terminalDrop0");
 }
}

#[test]
fn sqlite_snapshot_tsv_original_binding_quote_cancels_before_destination_birth(){
 use semio_framework_dsl_record::{FieldValue,RecordValue};use semio_framework_value::NativeDecodeControl;
 let fixture=tsv_native_control_fixture();let law=&fixture["typedDestination"];let grant:semio_framework_value::RetainedCloneGrant=serde_json::from_value(law["normalGrant"].clone()).unwrap();
 let record=RecordValue{fields:[(0,FieldValue::Text("original".into())),(1,FieldValue::List(vec![FieldValue::List(vec![FieldValue::Text("first".into()),FieldValue::Text("last".into())])])),(2,FieldValue::Bool(true)),(3,FieldValue::Enum(0))].into_iter().collect()};
 let source=match record.fields.get(&1).unwrap(){FieldValue::List(rows)=>rows.as_ptr(),_=>unreachable!()};let calls=std::cell::Cell::new(0);let mut callback=|_|{calls.set(calls.get()+1);calls.get()<law["quoteCancelCheckpoint"].as_u64().unwrap()as usize};let mut native=NativeDecodeControl::new(grant.maximum_capacity_bytes,&mut callback);let mut wallet=semio_framework_os_kernel::NativeSnapshotBodyWallet::new(grant);let mut output=None;
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admission::bind(&record,&mut output,&mut native,&mut wallet));assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert!(output.is_none());assert_eq!(wallet.progress(),Default::default());assert_eq!(native.owned_bytes(),0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(native);assert_eq!(calls.get(),law["quoteCancelCheckpoint"].as_u64().unwrap()as usize);match record.fields.get(&1).unwrap(){FieldValue::List(rows)=>assert_eq!(rows.as_ptr(),source),_=>unreachable!()};assert_eq!(serde_json::from_str::<String>(&serde_json::to_string("first").unwrap()).unwrap(),"first");eprintln!("[DEBUG] TSV original bounded quote cancels before typed birth, original source pointer unchanged, physical0 wallet0 native0");
}

#[test]
fn sqlite_snapshot_tsv_reconstruction_respects_value_budget() {
    let database = TsvSnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(TsvSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}


#[test]
fn sqlite_snapshot_tsv_borrowed_native_preflight_is_exact_cancellable_and_unowned(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase,ValueRefusalKind};
 let fixture=tsv_native_control_fixture();let policy=&fixture["preflight"];assert_eq!(policy["ownershipBytes"],0);assert_eq!(policy["retirementRefund"],false);
 let snapshot=tsv_canonical_carrier_snapshot();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=tsv_native_payload(&snapshot,encoding);let length=match &payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};assert!(length>0);
  let exact=SqliteDatabaseLimits{max_file_bytes:length,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,exact);
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).expect("actual owner preflight must admit its exact literal output with zero owned admission");assert_eq!(control.allocation_remaining_bytes(),0);
  let short=SqliteDatabaseLimits{max_file_bytes:length-1,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,short);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let rows=SqliteDatabaseLimits{max_rows:0,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,rows);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let initial=usize::try_from(fixture["maxAllocationBytes"].as_u64().unwrap()).unwrap();let limits=SqliteDatabaseLimits{max_allocation_bytes:initial,..Default::default()};let mut reached=false;
  let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,limits);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!(control.allocation_remaining_bytes(),initial);drop(control);assert!(reached,"actual borrowed preflight cancellation checkpoint required");
 }
}

#[test]
fn sqlite_snapshot_tsv_wide_record_preflight_can_be_cancelled() {
    let snapshot = TsvSnapshot { records: vec![vec![String::new(); 1024]], ..TsvSnapshot::default() };
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    let mut checkpoints = 0;
    let error = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |progress| { if progress.total == 1025 { checkpoints += 1; } checkpoints < 3 }, limits)).unwrap_err();
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::Canceled, "{error}");
}

#[test]
fn sqlite_snapshot_tsv_records_fields_and_line_endings_are_relational() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = TsvSnapshot { schema: fixture["schema"].as_str().unwrap().into(), records: fixture["records"].as_array().unwrap().iter().map(|record| record.as_array().unwrap().iter().map(|field| field.as_str().unwrap().into()).collect()).collect(), trailing_newline: true, line_ending: LineEnding::Crlf };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("tsv_record").unwrap().rows.len(), 3);
    assert_eq!(database.table("tsv_field").unwrap().rows.len(), 6);
    assert_eq!(database.table("tsv_field").unwrap().rows[2].values[3], SqliteValue::Text("Bibliothek 🌠".into()));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(TsvSnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, fixture);
    let mut broken = reopened;
    broken.table_mut("tsv_field").unwrap().rows[0].values[1] = SqliteValue::Integer(999);
    assert!(TsvSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

fn tsv_native_control_fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🛬️native-control/🔣️.json")).unwrap()}
fn tsv_native_control_snapshot()->TsvSnapshot{let input=tsv_native_control_fixture();let count=usize::try_from(input["workItems"].as_u64().unwrap()).unwrap();let pattern=input["bytePattern"].as_array().unwrap().iter().map(|v|u8::try_from(v.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();TsvSnapshot{schema:input["ownedSchema"].as_str().unwrap().into(),records:vec![Vec::new(),(0..count).map(|_|input["fieldText"].as_str().unwrap().into()).collect(),vec![String::new()],Vec::new()],trailing_newline:true,line_ending:LineEnding::Crlf}}
fn tsv_native_payload(snapshot:&TsvSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io_schema::IoPayload{match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(<TsvSnapshot as store::ArtifactPack>::encode_pack(snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(<TsvSnapshot as store::ArtifactDsl>::print_dsl(snapshot))}}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_tsv_actual_factory_canonical_external_carrier(){
 use {semio_framework_artifact_reference::ArtifactDialect};
 use store::sqlite_snapshot::SnapshotEncoding;
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Tsv full owned SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let codec=store::document_codec(crate::STDIO_TSV_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().expect("actual owning declaration publishes semantic SQLite");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<TsvSnapshot>()));
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.tsv".into(),standard:"iana".into(),subset:"*".into()};let snapshot=tsv_canonical_carrier_snapshot();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=tsv_native_payload(&snapshot,encoding);let database=with_original_decode(&native_caller_policy(),Default::default(),&mut |_|true,&mut |_|true,|_|{},|sql,native|(provider.export)(&codec.schema,&dialect,&payload,sql,native)).unwrap().value;assert_eq!(database,expected);let payload={let original_database=database;let pointer=original_database.tables.as_ptr();let mut original_input=Some(original_database);let result=with_original_encode(&native_caller_policy(),Default::default(),&mut |_|true,&mut |_|true,|_|{},|sql,native|(provider.import)(&codec.schema,&dialect,&mut original_input,encoding,sql,native));if result.is_ok(){assert!(original_input.is_none());}else if let Some(retained)=&original_input{assert_eq!(retained.tables.as_ptr(),pointer);}result}.unwrap().value;let restored=with_original_decode(&native_caller_policy(),Default::default(),&mut |_|true,&mut |_|true,|_|{},|sql,native|<TsvSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,sql,native)).unwrap();assert_eq!(restored,snapshot);retire_original_output(restored,&native_caller_policy());retire_original_output(payload,&native_caller_policy());}
}
#[test]
fn sqlite_snapshot_tsv_native_input_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=tsv_native_control_fixture();let snapshot=tsv_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=tsv_native_payload(&snapshot,encoding);assert_eq!(with_original_decode(&native_caller_policy(),low,&mut |_|true,&mut |_|true,|_|{},|sql,native|<TsvSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,sql,native)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
  let mut observed=false;let result=with_original_decode(&native_caller_policy(),Default::default(),&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},&mut |_|true,|_|{},|sql,native|<TsvSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,sql,native));assert!(result.is_err());assert!(observed,"actual interior native decode cancellation required");

 }
}
#[test]
fn sqlite_snapshot_tsv_long_schema_projection_and_reconstruction_are_interior_controlled(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let input=tsv_native_control_fixture();let mut snapshot=tsv_native_control_snapshot();snapshot.schema="x".repeat(usize::try_from(input["copyBytes"].as_u64().unwrap()).unwrap());let after=usize::try_from(input["copyCancelAfter"].as_u64().unwrap()).unwrap();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for phase in [SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut observed=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=after&&event.completed<event.total{observed=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let result=if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).map(|_|())}else{TsvSnapshot::from_sqlite_database(&database,&mut control).map(|_|())};assert!(result.is_err(),"long owned schema copied without interior cancellation");assert!(observed);}
}

#[test]
fn sqlite_snapshot_tsv_native_output_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=tsv_native_control_fixture();let snapshot=tsv_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut observed=false;let result=with_original_encode(&native_caller_policy(),Default::default(),&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},&mut |_|true,|_|{},|sql,native|snapshot.encode_sqlite_snapshot_native(encoding,sql,native));assert!(result.is_err());assert!(observed,"actual interior native encode cancellation required");
  let rows=SqliteDatabaseLimits{max_rows:usize::try_from(input["lowRows"].as_u64().unwrap()).unwrap(),..Default::default()};assert!(with_original_encode(&native_caller_policy(),rows,&mut |_|true,&mut |_|true,|_|{},|sql,native|snapshot.encode_sqlite_snapshot_native(encoding,sql,native)).is_err());assert_eq!(with_original_encode(&native_caller_policy(),low,&mut |_|true,&mut |_|true,|_|{},|sql,native|snapshot.encode_sqlite_snapshot_native(encoding,sql,native)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
 }
}

fn tsv_canonical_carrier_snapshot()->TsvSnapshot{let count=usize::try_from(tsv_native_control_fixture()["workItems"].as_u64().unwrap()).unwrap();TsvSnapshot{schema:crate::STDIO_TSV_DOCUMENT_SCHEMA.into(),records:vec![vec!["cell".into();count]],trailing_newline:false,line_ending:LineEnding::Lf}}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_tsv_actual_typed_file_preserves_complete_literal_owned_fields(){
 use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Tsv typed SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.tsv".into(),standard:"iana".into(),subset:"*".into()};
 let snapshot=tsv_native_control_snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 assert_eq!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>(),usize::try_from(tsv_native_control_fixture()["expectedRows"].as_u64().unwrap()).unwrap());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(&file[..16],b"SQLite format 3\0");let restored=io_import_sqlite_snapshot::<TsvSnapshot>(&dialect,&file,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(restored,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::DecodeNative|SqliteSnapshotPhase::EncodeNative)));}
 println!("[DEBUG] Tsv actual declaration typed SQLite files preserved complete literal fields in both metadata encodings without native lowering");
}
#[test]
fn sqlite_snapshot_tsv_logical_carrier_preserves_complete_owner(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let snapshot=tsv_native_control_snapshot();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=tsv_native_payload(&snapshot,encoding);let normal:TsvSnapshot=match &input{store::io_schema::IoPayload::Binary(bytes)=><TsvSnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(),store::io_schema::IoPayload::Text(text)=><TsvSnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap()};assert_eq!(normal,snapshot);assert_eq!(tsv_native_payload(&normal,encoding),input);}
}

#[test]
fn sqlite_snapshot_tsv_literal_long_field_copies_cancel_at_paid_utf8_boundary() {
    let input = tsv_native_control_fixture();
    let text = input["copyTextUnit"].as_str().unwrap().repeat(usize::try_from(input["copyTextRepeat"].as_u64().unwrap()).unwrap());
    let expected_bytes = usize::try_from(input["copyTextBytes"].as_u64().unwrap()).unwrap();
    let boundary = usize::try_from(input["copyTextBoundary"].as_u64().unwrap()).unwrap();
    assert_eq!(text.len(), expected_bytes);
    assert!(text.is_char_boundary(boundary));
    let snapshot = TsvSnapshot { schema: input["ownedSchema"].as_str().unwrap().into(), records: vec![vec![text]], trailing_newline: true, line_ending: LineEnding::Crlf };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
    assert_eq!(TsvSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap(),snapshot);
    for phase in [store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot] {
        let mut reached = false;
        let mut callback = |event:store::sqlite_snapshot::SqliteSnapshotProgress| {
            if event.phase == phase && event.total == expected_bytes && event.completed == boundary { reached = true; false } else { true }
        };
        let mut control = SqliteSnapshotControl::new(&mut callback,Default::default());
        let result = if phase == store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut control).map(|_|()) } else { TsvSnapshot::from_sqlite_database(&database,&mut control).map(|_|()) };
        assert!(result.is_err(),"literal field copied without paid UTF-8 interior checkpoint");
        assert!(reached,"actual field byte-copy boundary not observed");
    }
    let low = SqliteDatabaseLimits { max_value_bytes: expected_bytes-1, ..Default::default() };
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,low)).is_err());
    assert!(TsvSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,low)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_tsv_complete_logical_owner_erased_factory_preserves_every_literal_field(){
 use {semio_framework_artifact_reference::ArtifactDialect};
 use store::sqlite_snapshot::SnapshotEncoding;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧠️logical-owner/🔣️.json")).unwrap();
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Tsv complete logical owner").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let codec=store::document_codec(crate::STDIO_TSV_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().unwrap();assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<TsvSnapshot>()));
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.tsv".into(),standard:"iana".into(),subset:"*".into()};
 for input in fixture["snapshots"].as_array().unwrap(){
  let snapshot=TsvSnapshot{schema:input["schema"].as_str().unwrap().into(),records:input["records"].as_array().unwrap().iter().map(|row|row.as_array().unwrap().iter().map(|field|field.as_str().unwrap().into()).collect()).collect(),trailing_newline:input["trailingNewline"].as_bool().unwrap(),line_ending:match input["lineEnding"].as_str().unwrap(){"lf"=>LineEnding::Lf,"crlf"=>LineEnding::Crlf,_=>panic!("closed fixture ending")}};
  let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let input=tsv_native_payload(&snapshot,encoding);let length=match &input{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};let exact=SqliteDatabaseLimits{max_file_bytes:length,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,exact);snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap();assert_eq!(control.allocation_remaining_bytes(),0);
   let admitted=with_original_decode(&native_caller_policy(),Default::default(),&mut |_|true,&mut |_|true,|_|{},|sql,native|TsvSnapshot::decode_sqlite_snapshot_native(&input,sql,native)).unwrap();assert_eq!(admitted,snapshot,"native admission must retain every logical owner before erased export");
   let projected=with_original_decode(&native_caller_policy(),Default::default(),&mut |_|true,&mut |_|true,|_|{},|sql,native|(provider.export)(&codec.schema,&dialect,&input,sql,native)).unwrap().value;assert_eq!(projected,expected);
   let output={let original_database=projected;let pointer=original_database.tables.as_ptr();let mut original_input=Some(original_database);let result=with_original_encode(&native_caller_policy(),Default::default(),&mut |_|true,&mut |_|true,|_|{},|sql,native|(provider.import)(&codec.schema,&dialect,&mut original_input,encoding,sql,native));if result.is_ok(){assert!(original_input.is_none());}else if let Some(retained)=&original_input{assert_eq!(retained.tables.as_ptr(),pointer);}result}.unwrap().value;
   let restored=with_original_decode(&native_caller_policy(),Default::default(),&mut |_|true,&mut |_|true,|_|{},|sql,native|TsvSnapshot::decode_sqlite_snapshot_native(&output,sql,native)).unwrap();assert_eq!(restored,snapshot);retire_original_output(restored,&native_caller_policy());
   let ordinary=match &output{store::io_schema::IoPayload::Binary(bytes)=><TsvSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),store::io_schema::IoPayload::Text(text)=><TsvSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()};assert_eq!(ordinary,snapshot);retire_original_output(output,&native_caller_policy());
  }
 }
 println!("[DEBUG] Tsv actual erased factory preserved every complete logical owner in both native formats");
}

#[test]
fn sqlite_snapshot_tsv_canonical_owner_assets_match_actual_producers_and_natural_sources(){
 use store::{ArtifactDsl,ArtifactPack};
 for(text,binary)in[(include_str!("../🧫️fixtures/🧠️logical-owner/🗣️0.dsl.semio"),include_bytes!("../🧫️fixtures/🧠️logical-owner/🎒️0.pack.semio").as_slice()),(include_str!("../🧫️fixtures/🧠️logical-owner/🗣️1.dsl.semio"),include_bytes!("../🧫️fixtures/🧠️logical-owner/🎒️1.pack.semio").as_slice()),(include_str!("../🧫️fixtures/🧠️logical-owner/🗣️2.dsl.semio"),include_bytes!("../🧫️fixtures/🧠️logical-owner/🎒️2.pack.semio").as_slice()),(include_str!("../🧫️fixtures/🧠️logical-owner/🗣️3.dsl.semio"),include_bytes!("../🧫️fixtures/🧠️logical-owner/🎒️3.pack.semio").as_slice())]{let snapshot=TsvSnapshot::parse_dsl(text).unwrap();assert_eq!(snapshot.print_dsl(),text);assert_eq!(snapshot.encode_pack_with(&Default::default()).unwrap(),binary);assert_eq!(TsvSnapshot::decode_pack(binary).unwrap(),snapshot);}
 let demo=TsvSnapshot::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap();assert_eq!(demo.print_dsl(),crate::examples::demo::PRIMARY_TEXT);assert_eq!(demo.encode_pack_with(&Default::default()).unwrap(),crate::examples::demo::PACK_BYTES);let raw=crate::examples::demo::RAW_TSV.as_bytes();let text=std::str::from_utf8(raw).unwrap();assert_eq!(read_tsv_source_text(text).unwrap(),demo);assert_eq!(read_tsv_source_binary(raw).unwrap(),demo);assert_eq!(read_tsv_source_binary(crate::examples::demo::PACK_BYTES).unwrap(),demo);
 println!("[DEBUG] actual TSV owner producers matched every paired asset and authored natural file");
}

fn tsv_semantic_corpus_snapshot(value:&serde_json::Value)->TsvSnapshot{TsvSnapshot{schema:value["schema"].as_str().unwrap().into(),records:value["records"].as_array().unwrap().iter().map(|r|r.as_array().unwrap().iter().map(|f|f.as_str().unwrap().into()).collect()).collect(),trailing_newline:value["trailingNewline"].as_bool().unwrap(),line_ending:match value["lineEnding"].as_str().unwrap(){"lf"=>LineEnding::Lf,"crlf"=>LineEnding::Crlf,_=>panic!("closed ending")}}}
#[test]
fn sqlite_snapshot_tsv_complete_native_semantic_cells_use_closed_independent_sql_authority(){
 use store::sqlite_snapshot::SnapshotEncoding;use std::io::Write;use std::process::{Command,Stdio};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();let defaults=SqliteDatabaseLimits::default();let columns=plan["tableWidths"].as_object().unwrap().values().map(|v|v.as_u64().unwrap()as usize).max().unwrap();let tables=plan["tableWidths"].as_object().unwrap().len();
 for sample in plan["cases"].as_array().unwrap(){let snapshot=tsv_semantic_corpus_snapshot(&sample["snapshot"]);let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();let file=export_sqlite_database(&database,defaults,&mut |_|true).unwrap();
  let mut child=Command::new("bun").args(["-e","import Ajv from \"ajv/dist/2020\";import{Database}from\"bun:sqlite\";const plan=JSON.parse(process.argv[1]),sample=JSON.parse(process.argv[2]);const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())),quote=v=>'\"'+v.replaceAll('\"','\"\"')+'\"';try{if(db.query(\"PRAGMA integrity_check\").get().integrity_check!==\"ok\"||db.query(\"PRAGMA foreign_key_check\").all().length)throw Error(\"independent SQL integrity\");let rows=0,bytes=0;for(const[table,width]of Object.entries(plan.tableWidths)){const fields=db.query(\"PRAGMA table_info(\"+quote(table)+\")\").all().map(v=>v.name);if(fields.length!==width)throw Error(\"authored columns \"+table);const cells=fields.map(v=>{const f=quote(v);return \"CASE typeof(\"+f+\") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST(\"+f+\" AS BLOB)) WHEN 'blob' THEN length(\"+f+\") ELSE 0 END\";}).join(\"+\");const extent=db.query(\"SELECT COUNT(*) AS rows,COALESCE(SUM(\"+cells+\"),0) AS bytes FROM \"+quote(table)).get();rows+=extent.rows;bytes+=extent.bytes;}if(rows!==sample.rows||bytes!==sample.bytes)throw Error(\"independent complete cells \"+JSON.stringify({rows,bytes,expected:sample}));}finally{db.close();}"]).arg(plan.to_string()).arg(sample.to_string()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}: {}",sample["id"],String::from_utf8_lossy(&result.stderr));
  let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;let authored=TsvSnapshot::SQLITE_SCHEMA.len();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=tsv_native_payload(&snapshot,encoding);let retained=with_original_decode(&native_caller_policy(),defaults,&mut |_|true,&mut |_|true,|_|{},|sql,native|TsvSnapshot::decode_sqlite_snapshot_native(&input,sql,native)).unwrap();assert_eq!(retained,snapshot);retire_original_output(retained,&native_caller_policy());
   for(field,maximum,success)in[("columns",columns,false),("schema",authored,false),("tables",tables,false),("rows",rows,false),("bytes",bytes,false),("columns",columns,true),("schema",authored,true),("tables",tables,true),("rows",rows,true),("bytes",bytes,true)]{
    let maximum=maximum-usize::from(!success);let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:maximum,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},_=>unreachable!()};
    let decoded=with_original_decode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|sql,native|TsvSnapshot::decode_sqlite_snapshot_native(&input,sql,native));assert_eq!(decoded.is_ok(),success,"{} direct native {} complete authored limit",sample["id"],field);if let Ok(owner)=decoded{retire_original_output(owner,&native_caller_policy());}
    {let original_output=with_original_encode(&native_caller_policy(),limits,&mut |_|true,&mut |_|true,|_|{},|sql,native|snapshot.encode_sqlite_snapshot_native(encoding,sql,native));assert_eq!(original_output.is_ok(),success,"{} typed output {}",sample["id"],field);if let Ok(payload)=original_output{retire_original_output(payload,&native_caller_policy());}};assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),success,"{} borrowed preflight {}",sample["id"],field);
   }
  }retire_original_output(snapshot,&native_caller_policy());
 }
}

#[test]
fn sqlite_snapshot_tsv_complete_borrowed_gate_isolated_exact_and_one_short_before_typed_binding(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 let defaults=SqliteDatabaseLimits::default();let columns=plan["tableWidths"].as_object().unwrap().values().map(|v|v.as_u64().unwrap()as usize).max().unwrap();let tables=plan["tableWidths"].as_object().unwrap().len();
 for sample in plan["cases"].as_array().unwrap(){let snapshot=tsv_semantic_corpus_snapshot(&sample["snapshot"]);let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=tsv_native_payload(&snapshot,encoding);
 for(field,maximum)in[("columns",columns),("tables",tables),("schema",TsvSnapshot::SQLITE_SCHEMA.len()),("rows",rows),("bytes",bytes)]{for success in[true,false]{let max=maximum-usize::from(!success);let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:max,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:max,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:max,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:max,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:max,..defaults},_=>unreachable!()};let result=with_original_decode(&native_caller_policy(),defaults,&mut |_|true,&mut |_|true,|_|{},|sql,native|store::decode_sqlite_snapshot_record_native(&input,<TsvSnapshot as store::ArtifactDsl>::envelope_id(),TsvSnapshot::__dsl_spec_producer(),|record,snapshot_output,native,body|{crate::standards::iana::subsets::any::io::sqlite::snapshot::admission::admit(record,native,limits)?;crate::standards::iana::subsets::any::io::sqlite::snapshot::admission::bind(record,snapshot_output,native,body)},sql,native));assert_eq!(result.is_ok(),success,"{} isolated copied {} full semantic limit",sample["id"],field);if let Ok(snapshot)=result{retire_original_output(snapshot,&native_caller_policy());}}}
 }retire_original_output(snapshot,&native_caller_policy());}
}

/** 🫴️ Constructs one actual native caller from independently authored body and close authority. */
fn native_caller_policy()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🛬️native-control/🎟️original.json")).unwrap()}
trait OriginalNativeRefusalReceipt{fn original_native_receipt(&self)->semio_framework_value::retained_clone::RetainedCloneProgress;}
impl OriginalNativeRefusalReceipt for semio_framework_value::ValueError{fn original_native_receipt(&self)->semio_framework_value::retained_clone::RetainedCloneProgress{self.retained_progress()}}
impl OriginalNativeRefusalReceipt for store::io_schema::IoError{fn original_native_receipt(&self)->semio_framework_value::retained_clone::RetainedCloneProgress{self.cause.retained_progress()}}
macro_rules! original_native_caller{
 ($name:ident,$module:ident,$control:ident,$owner:ident,$recipient:ident,$allocation:ident,$progress:ident)=>{
  fn $name<T,E:OriginalNativeRefusalReceipt>(law:&serde_json::Value,limits:SqliteDatabaseLimits,callback:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool,original_observer:&mut dyn FnMut(semio_framework_value::$module::$progress)->bool,original_continuation:impl FnOnce(&mut semio_framework_value::$control<'_>),operation:impl FnOnce(&mut SqliteSnapshotControl<'_>,&mut store::$owner<'_,'_>)->Result<T,E>)->Result<T,E>{
   use semio_framework_value::$module::{$recipient,$allocation};
   use std::sync::atomic::{AtomicUsize,Ordering};
   let grant=serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["bodyGrant"].clone()).unwrap();let close=serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["closeGrant"].clone()).unwrap();
   let accepted=AtomicUsize::new(0usize);let mut original_allocation=|request:$allocation|{assert_eq!(request.owned_bytes,accepted.load(Ordering::Relaxed));accepted.store(request.next_owned_bytes,Ordering::Relaxed);Ok(())};let mut recipient=$recipient::new();
   let mut native=semio_framework_value::$control::new_forwarded(law["nativeMaximumBytes"].as_u64().unwrap()as usize,original_observer,&mut original_allocation);native.install_retirement_recipient(&mut recipient).unwrap();
   let mut sql=SqliteSnapshotControl::new(callback,limits);let mut owner=store::$owner::new(&mut native,grant);let result=operation(&mut sql,&mut owner);assert!(owner.progress().fits(grant));if let Err(error)=&result{assert_eq!(error.original_native_receipt(),owner.progress());}drop(owner);assert_eq!(native.owned_bytes(),accepted.load(Ordering::Relaxed));
   let held=native.has_retirement_owner();let denied=serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["deniedCloseGrant"].clone()).unwrap();assert_eq!(native.close_retirement_recipient(denied).unwrap().progress(),Default::default());assert_eq!(native.has_retirement_owner(),held);original_continuation(&mut native);
   for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){return result}assert!(native.close_retirement_recipient(close).unwrap().progress().fits(close));}
   assert!(!native.has_retirement_owner(),"original native caller did not reach funded terminal");result
  }
 };
}
original_native_caller!(with_original_decode,native_decoding,NativeDecodeControl,NativeSnapshotDecodeOwner,NativeDecodeRetirementRecipient,NativeDecodeAllocation,NativeDecodeProgress);
original_native_caller!(with_original_encode,native_encoding,NativeEncodeControl,NativeSnapshotEncodeOwner,NativeEncodeRetirementRecipient,NativeEncodeAllocation,NativeEncodeProgress);
/** ♻️ Settles successful caller-owned native output under its separate authored close turns. */
fn retire_original_output<T:semio_framework_value::retirement::RetireOwned>(source:T,law:&serde_json::Value){
 let close=serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["closeGrant"].clone()).unwrap();let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(source).map_err(|(error,_)|error).unwrap();
 for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){return}assert!(owner.step(close).unwrap().progress().fits(close));}assert!(owner.terminal_is_empty(),"original caller output did not reach funded terminal");
}
