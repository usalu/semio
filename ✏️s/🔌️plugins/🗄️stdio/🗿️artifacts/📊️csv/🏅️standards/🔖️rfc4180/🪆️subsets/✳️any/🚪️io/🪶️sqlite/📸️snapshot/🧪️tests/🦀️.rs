use crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::*;
use crate::standards::v_rfc4180::subsets::any::io::{text::snapshot::read_csv_source_text,binary::snapshot::read_csv_source_binary};
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

#[test]
fn sqlite_snapshot_csv_reconstruction_respects_value_budget() {
    let database = CsvSnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(CsvSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}


#[test]
fn sqlite_snapshot_csv_borrowed_native_preflight_is_exact_cancellable_and_unowned(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase,ValueRefusalKind};
 let fixture=csv_native_control_fixture();let policy=&fixture["preflight"];assert_eq!(policy["ownershipBytes"],0);assert_eq!(policy["retirementRefund"],false);
 let snapshot=csv_canonical_carrier_snapshot();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=csv_native_payload(&snapshot,encoding);let length=match &payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};assert!(length>0);
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
fn sqlite_snapshot_csv_wide_record_preflight_can_be_cancelled() {
    let snapshot = CsvSnapshot { records: vec![CsvRecord { fields: vec![CsvField { value: String::new(), quoted: false }; 1024] }], ..CsvSnapshot::default() };
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    let mut checkpoints = 0;
    let error = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |progress| { if progress.total == 1025 { checkpoints += 1; } checkpoints < 3 }, limits)).unwrap_err();
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::Canceled, "{error}");
}

#[test]
fn sqlite_snapshot_csv_records_fields_and_quoting_are_relational() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = CsvSnapshot {
        schema: fixture["schema"].as_str().unwrap().into(),
        has_header: fixture["hasHeader"].as_bool().unwrap(),
        records: fixture["records"].as_array().unwrap().iter().map(|row| CsvRecord { fields: row["fields"].as_array().unwrap().iter().map(|field| CsvField { value: field["value"].as_str().unwrap().into(), quoted: field["quoted"].as_bool().unwrap() }).collect() }).collect(),
    };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("csv_record").unwrap().rows.len(), 4);
    assert_eq!(database.table("csv_field").unwrap().rows.len(), 5);
    assert_eq!(database.table("csv_field").unwrap().rows[3].values[3], SqliteValue::Text("Grüße,\nLesesaal".into()));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(CsvSnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, fixture);
    let mut broken = reopened;
    broken.table_mut("csv_field").unwrap().rows[0].values[1] = SqliteValue::Integer(999);
    assert!(CsvSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

fn csv_native_control_fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🛬️native-control/🔣️.json")).unwrap()}
fn csv_native_control_snapshot()->CsvSnapshot{let input=csv_native_control_fixture();let count=usize::try_from(input["workItems"].as_u64().unwrap()).unwrap();let pattern=input["bytePattern"].as_array().unwrap().iter().map(|v|u8::try_from(v.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();CsvSnapshot{schema:input["ownedSchema"].as_str().unwrap().into(),has_header:false,records:vec![CsvRecord{fields:Vec::new()},CsvRecord{fields:(0..count).map(|_|CsvField{value:input["fieldText"].as_str().unwrap().into(),quoted:false}).collect()},CsvRecord{fields:vec![CsvField{value:String::new(),quoted:true}]},CsvRecord{fields:Vec::new()}]}}
fn csv_native_payload(snapshot:&CsvSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io_schema::IoPayload{match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(<CsvSnapshot as store::ArtifactDsl>::print_dsl(snapshot))}}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_csv_actual_factory_canonical_external_carrier(){
 use {semio_framework_artifact_reference::ArtifactDialect};
 use store::sqlite_snapshot::SnapshotEncoding;
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Csv full owned SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let codec=store::document_codec(crate::STDIO_CSV_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().expect("actual owning declaration publishes semantic SQLite");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<CsvSnapshot>()));
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.csv".into(),standard:"rfc4180".into(),subset:"*".into()};let snapshot=csv_canonical_carrier_snapshot();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=csv_native_payload(&snapshot,encoding);let database=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;assert_eq!(database,expected);let payload=(provider.import)(&codec.schema,&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;let restored=<CsvSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(restored,snapshot);}
}
#[test]
fn sqlite_snapshot_csv_native_input_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=csv_native_control_fixture();let snapshot=csv_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=csv_native_payload(&snapshot,encoding);assert_eq!(<CsvSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
  let mut observed=false;let result=<CsvSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native decode cancellation required");

 }
}
#[test]
fn sqlite_snapshot_csv_long_schema_projection_and_reconstruction_are_interior_controlled(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let input=csv_native_control_fixture();let mut snapshot=csv_native_control_snapshot();snapshot.schema="x".repeat(usize::try_from(input["copyBytes"].as_u64().unwrap()).unwrap());let after=usize::try_from(input["copyCancelAfter"].as_u64().unwrap()).unwrap();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for phase in [SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut observed=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=after&&event.completed<event.total{observed=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let result=if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).map(|_|())}else{CsvSnapshot::from_sqlite_database(&database,&mut control).map(|_|())};assert!(result.is_err(),"long owned schema copied without interior cancellation");assert!(observed);}
}

#[test]
fn sqlite_snapshot_csv_native_output_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=csv_native_control_fixture();let snapshot=csv_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut observed=false;let result=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native encode cancellation required");
  let rows=SqliteDatabaseLimits{max_rows:usize::try_from(input["lowRows"].as_u64().unwrap()).unwrap(),..Default::default()};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,rows)).is_err());assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
 }
}

fn csv_canonical_carrier_snapshot()->CsvSnapshot{let count=usize::try_from(csv_native_control_fixture()["workItems"].as_u64().unwrap()).unwrap();CsvSnapshot{schema:crate::STDIO_CSV_DOCUMENT_SCHEMA.into(),has_header:true,records:vec![CsvRecord{fields:(0..count).map(|_|CsvField{value:"cell".into(),quoted:false}).collect()}]}}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_csv_actual_typed_file_preserves_complete_literal_owned_fields(){
 use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Csv typed SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.csv".into(),standard:"rfc4180".into(),subset:"*".into()};
 let snapshot=csv_native_control_snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 assert_eq!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>(),usize::try_from(csv_native_control_fixture()["expectedRows"].as_u64().unwrap()).unwrap());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(&file[..16],b"SQLite format 3\0");let restored=io_import_sqlite_snapshot::<CsvSnapshot>(&dialect,&file,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(restored,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::DecodeNative|SqliteSnapshotPhase::EncodeNative)));}
 println!("[DEBUG] Csv actual declaration typed SQLite files preserved complete literal fields in both metadata encodings without native lowering");
}
#[test]
fn sqlite_snapshot_csv_logical_carrier_preserves_complete_owner(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let snapshot=csv_native_control_snapshot();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=csv_native_payload(&snapshot,encoding);let normal:CsvSnapshot=match &input{store::io_schema::IoPayload::Binary(bytes)=><CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(),store::io_schema::IoPayload::Text(text)=><CsvSnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap()};assert_eq!(normal,snapshot);assert_eq!(csv_native_payload(&normal,encoding),input);}
}

#[test]
fn sqlite_snapshot_csv_literal_long_field_copies_cancel_at_paid_utf8_boundary() {
    let input = csv_native_control_fixture();
    let text = input["copyTextUnit"].as_str().unwrap().repeat(usize::try_from(input["copyTextRepeat"].as_u64().unwrap()).unwrap());
    let expected_bytes = usize::try_from(input["copyTextBytes"].as_u64().unwrap()).unwrap();
    let boundary = usize::try_from(input["copyTextBoundary"].as_u64().unwrap()).unwrap();
    assert_eq!(text.len(), expected_bytes);
    assert!(text.is_char_boundary(boundary));
    let snapshot = CsvSnapshot { schema: input["ownedSchema"].as_str().unwrap().into(), has_header: false, records: vec![CsvRecord { fields: vec![CsvField { value: text, quoted: true }] }] };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
    assert_eq!(CsvSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap(),snapshot);
    for phase in [store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot] {
        let mut reached = false;
        let mut callback = |event:store::sqlite_snapshot::SqliteSnapshotProgress| {
            if event.phase == phase && event.total == expected_bytes && event.completed == boundary { reached = true; false } else { true }
        };
        let mut control = SqliteSnapshotControl::new(&mut callback,Default::default());
        let result = if phase == store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut control).map(|_|()) } else { CsvSnapshot::from_sqlite_database(&database,&mut control).map(|_|()) };
        assert!(result.is_err(),"literal field copied without paid UTF-8 interior checkpoint");
        assert!(reached,"actual field byte-copy boundary not observed");
    }
    let low = SqliteDatabaseLimits { max_value_bytes: expected_bytes-1, ..Default::default() };
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,low)).is_err());
    assert!(CsvSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,low)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_csv_complete_logical_owner_erased_factory_preserves_every_literal_field(){
 use {semio_framework_artifact_reference::ArtifactDialect};
 use store::sqlite_snapshot::SnapshotEncoding;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧠️logical-owner/🔣️.json")).unwrap();
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Csv complete logical owner").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let codec=store::document_codec(crate::STDIO_CSV_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().unwrap();assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<CsvSnapshot>()));
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.csv".into(),standard:"rfc4180".into(),subset:"*".into()};
 for input in fixture["snapshots"].as_array().unwrap(){
  let snapshot=CsvSnapshot{schema:input["schema"].as_str().unwrap().into(),has_header:input["hasHeader"].as_bool().unwrap(),records:input["records"].as_array().unwrap().iter().map(|record|CsvRecord{fields:record["fields"].as_array().unwrap().iter().map(|field|CsvField{value:field["value"].as_str().unwrap().into(),quoted:field["quoted"].as_bool().unwrap()}).collect()}).collect()};
  let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let input=csv_native_payload(&snapshot,encoding);let length=match &input{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};let exact=SqliteDatabaseLimits{max_file_bytes:length,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,exact);snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap();assert_eq!(control.allocation_remaining_bytes(),0);let admitted=CsvSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(admitted,snapshot,"native owner admission must retain every authored field before erased export");
   let projected=(provider.export)(&codec.schema,&dialect,&input,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;assert_eq!(projected,expected);
   let output=(provider.import)(&codec.schema,&dialect,projected,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;
   let restored=CsvSnapshot::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(restored,snapshot);
   let ordinary=match output{store::io_schema::IoPayload::Binary(bytes)=><CsvSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),store::io_schema::IoPayload::Text(text)=><CsvSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()};assert_eq!(ordinary,snapshot);
  }
 }
 println!("[DEBUG] Csv actual erased factory preserved every complete logical owner in both native formats");
}

#[test]
fn sqlite_snapshot_csv_canonical_owner_assets_match_actual_producers_and_natural_sources(){
 use store::{ArtifactDsl,ArtifactPack};
 for(text,binary)in[(include_str!("../🧫️fixtures/🧠️logical-owner/🗣️0.dsl.semio"),include_bytes!("../🧫️fixtures/🧠️logical-owner/🎒️0.pack.semio").as_slice()),(include_str!("../🧫️fixtures/🧠️logical-owner/🗣️1.dsl.semio"),include_bytes!("../🧫️fixtures/🧠️logical-owner/🎒️1.pack.semio").as_slice())]{let snapshot=CsvSnapshot::parse_dsl(text).unwrap();assert_eq!(snapshot.print_dsl(),text);assert_eq!(snapshot.encode_pack_with(&Default::default()).unwrap(),binary);assert_eq!(CsvSnapshot::decode_pack(binary).unwrap(),snapshot);}
 let demo=CsvSnapshot::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap();assert_eq!(demo.print_dsl(),crate::examples::demo::PRIMARY_TEXT);assert_eq!(demo.encode_pack_with(&Default::default()).unwrap(),crate::examples::demo::PACK_BYTES);let raw=crate::examples::demo::NATIVE_BYTES;let text=std::str::from_utf8(raw).unwrap();assert_eq!(read_csv_source_text(text).unwrap(),demo);assert_eq!(read_csv_source_binary(raw).unwrap(),demo);assert_eq!(read_csv_source_binary(crate::examples::demo::PACK_BYTES).unwrap(),demo);
 println!("[DEBUG] actual CSV owner producers matched every paired asset and authored natural file");
}

fn csv_semantic_corpus_snapshot(value:&serde_json::Value)->CsvSnapshot{CsvSnapshot{schema:value["schema"].as_str().unwrap().into(),has_header:value["hasHeader"].as_bool().unwrap(),records:value["records"].as_array().unwrap().iter().map(|r|CsvRecord{fields:r["fields"].as_array().unwrap().iter().map(|f|CsvField{value:f["value"].as_str().unwrap().into(),quoted:f["quoted"].as_bool().unwrap()}).collect()}).collect()}}
#[test]
fn sqlite_snapshot_csv_complete_native_semantic_cells_use_closed_independent_sql_authority(){
 use store::sqlite_snapshot::SnapshotEncoding;use std::io::Write;use std::process::{Command,Stdio};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();let defaults=SqliteDatabaseLimits::default();let columns=plan["tableWidths"].as_object().unwrap().values().map(|v|v.as_u64().unwrap()as usize).max().unwrap();let tables=plan["tableWidths"].as_object().unwrap().len();
 for sample in plan["cases"].as_array().unwrap(){let snapshot=csv_semantic_corpus_snapshot(&sample["snapshot"]);let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();let file=export_sqlite_database(&database,defaults,&mut |_|true).unwrap();
  let mut child=Command::new("bun").args(["-e","import Ajv from \"ajv/dist/2020\";import{Database}from\"bun:sqlite\";const plan=JSON.parse(process.argv[1]),sample=JSON.parse(process.argv[2]);const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())),quote=v=>'\"'+v.replaceAll('\"','\"\"')+'\"';try{if(db.query(\"PRAGMA integrity_check\").get().integrity_check!==\"ok\"||db.query(\"PRAGMA foreign_key_check\").all().length)throw Error(\"independent SQL integrity\");let rows=0,bytes=0;for(const[table,width]of Object.entries(plan.tableWidths)){const fields=db.query(\"PRAGMA table_info(\"+quote(table)+\")\").all().map(v=>v.name);if(fields.length!==width)throw Error(\"authored columns \"+table);const cells=fields.map(v=>{const f=quote(v);return \"CASE typeof(\"+f+\") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST(\"+f+\" AS BLOB)) WHEN 'blob' THEN length(\"+f+\") ELSE 0 END\";}).join(\"+\");const extent=db.query(\"SELECT COUNT(*) AS rows,COALESCE(SUM(\"+cells+\"),0) AS bytes FROM \"+quote(table)).get();rows+=extent.rows;bytes+=extent.bytes;}if(rows!==sample.rows||bytes!==sample.bytes)throw Error(\"independent complete cells \"+JSON.stringify({rows,bytes,expected:sample}));}finally{db.close();}"]).arg(plan.to_string()).arg(sample.to_string()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}: {}",sample["id"],String::from_utf8_lossy(&result.stderr));
  let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;let authored=CsvSnapshot::SQLITE_SCHEMA.len();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=csv_native_payload(&snapshot,encoding);let retained=CsvSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();assert_eq!(retained,snapshot);retained.retire_sqlite_snapshot();
   for(field,maximum,success)in[("columns",columns,false),("schema",authored,false),("tables",tables,false),("rows",rows,false),("bytes",bytes,false),("columns",columns,true),("schema",authored,true),("tables",tables,true),("rows",rows,true),("bytes",bytes,true)]{
    let maximum=maximum-usize::from(!success);let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:maximum,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:maximum,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:maximum,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:maximum,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:maximum,..defaults},_=>unreachable!()};
    let decoded=CsvSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(decoded.is_ok(),success,"{} direct native {} complete authored limit",sample["id"],field);if let Ok(owner)=decoded{owner.retire_sqlite_snapshot();}
    assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),success,"{} typed output {}",sample["id"],field);assert_eq!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),success,"{} borrowed preflight {}",sample["id"],field);
   }
  }snapshot.retire_sqlite_snapshot();
 }
}

#[test]
fn sqlite_snapshot_csv_complete_borrowed_gate_isolated_exact_and_one_short_before_typed_binding(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛂️semantic/🔣️.json")).unwrap();
 let defaults=SqliteDatabaseLimits::default();let columns=plan["tableWidths"].as_object().unwrap().values().map(|v|v.as_u64().unwrap()as usize).max().unwrap();let tables=plan["tableWidths"].as_object().unwrap().len();
 for sample in plan["cases"].as_array().unwrap(){let snapshot=csv_semantic_corpus_snapshot(&sample["snapshot"]);let rows=sample["rows"].as_u64().unwrap()as usize;let bytes=sample["bytes"].as_u64().unwrap()as usize;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=csv_native_payload(&snapshot,encoding);
 for(field,maximum)in[("columns",columns),("tables",tables),("schema",CsvSnapshot::SQLITE_SCHEMA.len()),("rows",rows),("bytes",bytes)]{for success in[true,false]{let max=maximum-usize::from(!success);let limits=match field{"columns"=>SqliteDatabaseLimits{max_columns:max,..defaults},"tables"=>SqliteDatabaseLimits{max_tables:max,..defaults},"schema"=>SqliteDatabaseLimits{max_schema_bytes:max,..defaults},"rows"=>SqliteDatabaseLimits{max_rows:max,..defaults},"bytes"=>SqliteDatabaseLimits{max_value_bytes:max,..defaults},_=>unreachable!()};let result=store::decode_sqlite_snapshot_record_native(&input,<CsvSnapshot as store::ArtifactDsl>::envelope_id(),CsvSnapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::admission::admit(record,native,limits)})(); *snapshot_output = Some(constructed?); Ok(()) },&mut SqliteSnapshotControl::new(&mut |_|true,defaults));assert_eq!(result.is_ok(),success,"{} isolated copied {} full semantic limit",sample["id"],field);}}
 }snapshot.retire_sqlite_snapshot();}
}
