use crate::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

#[test]
fn sqlite_snapshot_binary_reconstruction_respects_value_budget() {
    let database = BinarySnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(BinarySnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}


#[test]
fn sqlite_snapshot_binary_borrowed_native_preflight_is_exact_cancellable_and_unowned(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase,ValueRefusalKind};
 let fixture=binary_native_control_fixture();let policy=&fixture["preflight"];assert_eq!(policy["ownershipBytes"],0);assert_eq!(policy["retirementRefund"],false);
 let snapshot=binary_canonical_carrier_snapshot();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=binary_native_payload(&snapshot,encoding);let length=match &payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};assert!(length>0);
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
fn sqlite_snapshot_binary_bytes_are_queryable_ordered_integers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = BinarySnapshot { schema: fixture["schema"].as_str().unwrap().into(), bytes: fixture["bytes"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect() };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("binary_byte").unwrap().rows.len(), 6);
    assert_eq!(database.table("binary_byte").unwrap().rows[4].values[3], SqliteValue::Integer(255));
    assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(BinarySnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    assert_eq!(serde_json::to_value(&snapshot.bytes).unwrap(), fixture["bytes"]);
    let mut broken = reopened;
    broken.table_mut("binary_byte").unwrap().rows[0].values[3] = SqliteValue::Integer(256);
    assert!(BinarySnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

fn binary_native_control_fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🛬️native-control/🔣️.json")).unwrap()}
fn binary_native_control_snapshot()->BinarySnapshot{let input=binary_native_control_fixture();let count=usize::try_from(input["workItems"].as_u64().unwrap()).unwrap();let pattern=input["bytePattern"].as_array().unwrap().iter().map(|v|u8::try_from(v.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();BinarySnapshot{schema:input["ownedSchema"].as_str().unwrap().into(),bytes:(0..count).map(|i|pattern[i%4]).collect()}}
fn binary_native_payload(snapshot:&BinarySnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io_schema::IoPayload{match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(<BinarySnapshot as store::ArtifactPack>::encode_pack(snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(<BinarySnapshot as store::ArtifactDsl>::print_dsl(snapshot))}}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_binary_actual_factory_canonical_external_carrier(){
 use {semio_framework_artifact_reference::ArtifactDialect};
 use store::sqlite_snapshot::SnapshotEncoding;
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Binary full owned SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let codec=store::document_codec(crate::STDIO_BINARY_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().expect("actual owning declaration publishes semantic SQLite");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<BinarySnapshot>()));
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.binary".into(),standard:"raw".into(),subset:"*".into()};let snapshot=binary_canonical_carrier_snapshot();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=binary_native_payload(&snapshot,encoding);let database=with_binary_decoding(|owner|(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default()),owner)).unwrap().value;assert_eq!(database,expected);let mut input=Some(database);let payload=with_binary_encoding(|owner|(provider.import)(&codec.schema,&dialect,&mut input,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default()),owner)).unwrap().value;let restored=binary_decode(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(restored,snapshot);}
}
#[test]
fn sqlite_snapshot_binary_native_input_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=binary_native_control_fixture();let snapshot=binary_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=binary_native_payload(&snapshot,encoding);assert_eq!(binary_decode(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
  let mut observed=false;let result=binary_decode(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native decode cancellation required");

 }
}
#[test]
fn sqlite_snapshot_binary_long_schema_projection_and_reconstruction_are_interior_controlled(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let input=binary_native_control_fixture();let mut snapshot=binary_native_control_snapshot();snapshot.schema="x".repeat(usize::try_from(input["copyBytes"].as_u64().unwrap()).unwrap());let after=usize::try_from(input["copyCancelAfter"].as_u64().unwrap()).unwrap();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for phase in [SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut observed=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=after&&event.completed<event.total{observed=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let result=if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).map(|_|())}else{BinarySnapshot::from_sqlite_database(&database,&mut control).map(|_|())};assert!(result.is_err(),"long owned schema copied without interior cancellation");assert!(observed);}
}

#[test]
fn sqlite_snapshot_binary_native_output_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=binary_native_control_fixture();let snapshot=binary_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut observed=false;let result=binary_encode(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native encode cancellation required");
  let rows=SqliteDatabaseLimits{max_rows:usize::try_from(input["lowRows"].as_u64().unwrap()).unwrap(),..Default::default()};assert!(binary_encode(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,rows)).is_err());assert_eq!(binary_encode(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
 }
}

fn binary_canonical_carrier_snapshot()->BinarySnapshot{let mut snapshot=binary_native_control_snapshot();snapshot.schema=crate::STDIO_BINARY_DOCUMENT_SCHEMA.into();snapshot}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_binary_actual_typed_file_preserves_complete_literal_owned_fields(){
 use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Binary typed SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.binary".into(),standard:"raw".into(),subset:"*".into()};
 let snapshot=binary_native_control_snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 assert_eq!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>(),usize::try_from(binary_native_control_fixture()["expectedRows"].as_u64().unwrap()).unwrap());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(&file[..16],b"SQLite format 3\0");let restored=io_import_sqlite_snapshot::<BinarySnapshot>(&dialect,&file,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(restored,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::DecodeNative|SqliteSnapshotPhase::EncodeNative)));}
 println!("[DEBUG] Binary actual declaration typed SQLite files preserved complete literal fields in both metadata encodings without native lowering");
}
#[test]
fn sqlite_snapshot_binary_external_carrier_normalizes_unrepresented_schema(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let snapshot=binary_native_control_snapshot();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=binary_native_payload(&snapshot,encoding);let normal:BinarySnapshot=match &input{store::io_schema::IoPayload::Binary(bytes)=><BinarySnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(),store::io_schema::IoPayload::Text(text)=><BinarySnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap()};assert_eq!(normal.schema,crate::STDIO_BINARY_DOCUMENT_SCHEMA);assert_ne!(normal.schema,snapshot.schema);assert_eq!(binary_native_payload(&normal,encoding),input);}
}

fn binary_semantic_corpus_snapshot(value:&serde_json::Value)->BinarySnapshot{BinarySnapshot{schema:value["schema"].as_str().unwrap().into(),bytes:value["bytes"].as_array().unwrap().iter().map(|v|u8::try_from(v.as_u64().unwrap()).unwrap()).collect()}}
#[test]
fn sqlite_snapshot_binary_complete_native_semantic_cells_use_closed_independent_sql_authority(){
 use store::sqlite_snapshot::SnapshotEncoding;use std::io::Write;use std::process::{Command,Stdio};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();let defaults=SqliteDatabaseLimits::default();let columns=plan["tableWidths"].as_object().unwrap().values().map(|v|v.as_u64().unwrap()as usize).max().unwrap();let tables=plan["tableWidths"].as_object().unwrap().len();
 for sample in plan["cases"].as_array().unwrap(){let snapshot=binary_semantic_corpus_snapshot(&sample["snapshot"]);let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();let file=export_sqlite_database(&database,defaults,&mut |_|true).unwrap();
  let mut child=Command::new("bun").args(["-e","import Ajv from \"ajv/dist/2020\";import{Database}from\"bun:sqlite\";const plan=JSON.parse(process.argv[1]),sample=JSON.parse(process.argv[2]);const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())),quote=v=>'\"'+v.replaceAll('\"','\"\"')+'\"';try{if(db.query(\"PRAGMA integrity_check\").get().integrity_check!==\"ok\"||db.query(\"PRAGMA foreign_key_check\").all().length)throw Error(\"independent SQL integrity\");let rows=0,bytes=0;for(const[table,width]of Object.entries(plan.tableWidths)){const fields=db.query(\"PRAGMA table_info(\"+quote(table)+\")\").all().map(v=>v.name);if(fields.length!==width)throw Error(\"authored columns \"+table);const cells=fields.map(v=>{const f=quote(v);return \"CASE typeof(\"+f+\") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST(\"+f+\" AS BLOB)) WHEN 'blob' THEN length(\"+f+\") ELSE 0 END\";}).join(\"+\");const extent=db.query(\"SELECT COUNT(*) AS rows,COALESCE(SUM(\"+cells+\"),0) AS bytes FROM \"+quote(table)).get();rows+=extent.rows;bytes+=extent.bytes;}if(rows!==sample.rows||bytes!==sample.bytes)throw Error(\"independent complete cells \"+JSON.stringify({rows,bytes,expected:sample}));}finally{db.close();}"]).arg(plan.to_string()).arg(sample.to_string()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}: {}",sample["id"],String::from_utf8_lossy(&result.stderr));
  let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;let authored=BinarySnapshot::SQLITE_SCHEMA.len();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=binary_native_payload(&snapshot,encoding);let retained=binary_decode(&input,&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();assert_eq!(retained,snapshot);retained.retire_sqlite_snapshot();
   for(field,maximum,success)in[("columns",columns,false),("schema",authored,false),("tables",tables,false),("rows",rows,false),("bytes",bytes,false),("columns",columns,true),("schema",authored,true),("tables",tables,true),("rows",rows,true),("bytes",bytes,true)]{
    let maximum=maximum-usize::from(!success);let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:maximum,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},_=>unreachable!()};
    let decoded=binary_decode(&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(decoded.is_ok(),success,"{} direct native {} complete authored limit",sample["id"],field);if let Ok(owner)=decoded{owner.retire_sqlite_snapshot();}
    assert_eq!(binary_encode(&snapshot,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),success,"{} typed output {}",sample["id"],field);assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),success,"{} borrowed preflight {}",sample["id"],field);
   }
  }snapshot.retire_sqlite_snapshot();
 }
}

#[test]
fn sqlite_snapshot_binary_complete_borrowed_gate_isolated_exact_and_one_short_before_typed_binding(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 let defaults=SqliteDatabaseLimits::default();let columns=plan["tableWidths"].as_object().unwrap().values().map(|v|v.as_u64().unwrap()as usize).max().unwrap();let tables=plan["tableWidths"].as_object().unwrap().len();
 for sample in plan["cases"].as_array().unwrap(){let snapshot=binary_semantic_corpus_snapshot(&sample["snapshot"]);let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=binary_native_payload(&snapshot,encoding);
 for(field,maximum)in[("columns",columns),("tables",tables),("schema",BinarySnapshot::SQLITE_SCHEMA.len()),("rows",rows),("bytes",bytes)]{for success in[true,false]{let max=maximum-usize::from(!success);let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:max,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:max,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:max,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:max,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:max,..defaults},_=>unreachable!()};let source=snapshot.bytes.as_slice();let result=crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native_cells(&snapshot.schema,source.len(),source.iter().copied().map(Ok),&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(result.is_ok(),success,"{} isolated copied {} full semantic limit",sample["id"],field);}}
 }snapshot.retire_sqlite_snapshot();}
}

fn binary_original_grant()->semio_framework_value::RetainedCloneGrant{let f:serde_json::Value=serde_json::from_str(include_str!("../🫴️receiving/🧫️fixtures/🔣️.json")).unwrap();let g=&f["original"]["grant"];semio_framework_value::RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize}}
fn with_binary_decoding<T>(operation:impl FnOnce(&mut store::NativeSnapshotDecodeOwner<'_,'_>)->T)->T{let fixture:serde_json::Value=serde_json::from_str(include_str!("../🫴️receiving/🧫️fixtures/🔣️.json")).unwrap();let grant=binary_original_grant();let mut callback=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=semio_framework_value::NativeDecodeControl::new(fixture["original"]["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,grant);let result=operation(&mut owner);drop(owner);while native.has_retirement_owner(){native.close_retirement_recipient(grant).unwrap();}result}
fn with_binary_encoding<T>(operation:impl FnOnce(&mut store::NativeSnapshotEncodeOwner<'_,'_>)->T)->T{let fixture:serde_json::Value=serde_json::from_str(include_str!("../🫴️receiving/🧫️fixtures/🔣️.json")).unwrap();let grant=binary_original_grant();let mut callback=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=semio_framework_value::NativeEncodeControl::new(fixture["original"]["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,grant);let result=operation(&mut owner);drop(owner);while native.has_retirement_owner(){native.close_retirement_recipient(grant).unwrap();}result}
fn binary_decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<BinarySnapshot,semio_framework_value::ValueError>{with_binary_decoding(|owner|BinarySnapshot::decode_sqlite_snapshot_native(payload,control,owner))}
fn binary_encode(snapshot:&BinarySnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{with_binary_encoding(|owner|snapshot.encode_sqlite_snapshot_native(encoding,control,owner))}
