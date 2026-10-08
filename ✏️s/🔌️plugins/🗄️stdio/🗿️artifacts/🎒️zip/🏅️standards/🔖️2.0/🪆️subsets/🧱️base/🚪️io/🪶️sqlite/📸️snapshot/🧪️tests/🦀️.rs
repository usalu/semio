use crate::standards::v2_0::subsets::base::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> ZipSnapshot { semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }

#[test]
fn sqlite_snapshot_zip_large_archive_comment_can_cancel_before_owned_copy(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🚦️control.json")).unwrap();let mut snapshot=fixture();snapshot.comment="x".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize);let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut reached=false;let result=ZipSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |event|{if event.completed==cases["cancelCompleted"].as_u64().unwrap() as usize&&event.total==0{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(reached,"large archive text ownership requires a checkpoint");assert!(result.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_zip_actual_declaration_keeps_full_owned_fields_over_io(){
 use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::sqlite_snapshot::SnapshotEncoding,store::sqlite_snapshot::SqliteSnapshotPhase};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("ZIP SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.zip".into(),standard:"2.0".into(),subset:"*".into()};let mut snapshot=fixture();snapshot.schema="Owned ZIP 世界".into();let mut phases=Vec::new();let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<ZipSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut wrong=dialect.clone();wrong.subset="unknown".into();assert!(snapshot.validate_sqlite_snapshot_subset(&wrong,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
 let mut iso=dialect.clone();iso.subset="iso21320".into();let exported=io_export_sqlite_snapshot(&iso,&snapshot,SnapshotEncoding::Text,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap();assert_eq!(exported.diagnostics.len(),1);assert_eq!(exported.diagnostics[0].code.0,"stdio.zip.iso21320.data-descriptor-present");let imported=io_import_sqlite_snapshot::<ZipSnapshot>(&iso,&exported.value,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap();assert_eq!(imported.value,snapshot);assert_eq!(imported.diagnostics.len(),1);
 let mut blocked=snapshot.clone();blocked.entries[0].metadata.local.flags|=1;let refusal=io_export_sqlite_snapshot(&iso,&blocked,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap_err();assert!(refusal.diagnostics.iter().any(|d|d.code.0=="stdio.zip.iso21320.entry-encrypted"&&d.severity==semio_framework_diagnostic::Severity::Error));
 let mut edited=import_sqlite_database(&exported.value,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();edited.table_mut("zip_local_header").unwrap().rows[0].values[2]=SqliteValue::Integer(1);let edited=export_sqlite_database(&edited,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let refusal=io_import_sqlite_snapshot::<ZipSnapshot>(&iso,&edited,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap_err();assert!(refusal.diagnostics.iter().any(|d|d.code.0=="stdio.zip.iso21320.entry-encrypted"&&d.severity==semio_framework_diagnostic::Severity::Error));
}

#[test]
fn sqlite_snapshot_zip_all_member_header_policy_and_ordered_extra_fields_roundtrip() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.tables.len(), 12);
    assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = ZipSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&restored))).unwrap();
    assert_eq!(oracle, serde_json::from_str::<serde_json::Value>(include_str!("../🧫️fixtures/🔣️.json")).unwrap());
    for alteration in 0..5 {
        let mut broken = database.clone();
        match alteration {
            0 => broken.table_mut("zip_entry_byte").unwrap().rows[0].values[1] = SqliteValue::Integer(999),
            1 => broken.table_mut("zip_local_extra_field_byte").unwrap().rows[0].values[3] = SqliteValue::Integer(256),
            2 => broken.table_mut("zip_central_header").unwrap().rows[0].values[8] = SqliteValue::Integer(0),
            3 => broken.table_mut("zip_entry").unwrap().rows[1].values[2] = SqliteValue::Integer(0),
            _ => { broken.table_mut("zip_local_header").unwrap().rows.pop(); },
        }
        assert!(ZipSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(ZipSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
    let empty = ZipSnapshot::default();
    let database = empty.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(ZipSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), empty);
}

#[test]
fn sqlite_snapshot_zip_independent_archive_member_header_and_payload_edits() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT e.name,h.external_attributes,f.tag FROM zip_entry e JOIN zip_central_header h ON h.id=e.id JOIN zip_central_extra_field f ON f.header_id=h.id ORDER BY e.ordinal,f.ordinal').all();if(rows.length!==2||rows[1].tag!==51966||rows[0].external_attributes!==4294967295)throw Error('header relationships');db.query('UPDATE zip_entry_byte SET value=42 WHERE entry_id=1 AND ordinal=2').run();db.query('UPDATE zip_central_header SET comment=? WHERE id=1').run('SQLite Kommentar');db.query('UPDATE zip_local_extra_field_byte SET value=128 WHERE extra_field_id=1 AND ordinal=1').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    snapshot.entries[0].data[2] = 42;
    snapshot.entries[0].metadata.central.comment = "SQLite Kommentar".into();
    snapshot.entries[0].metadata.local.extra_fields[0].data[1] = 128;
    assert_eq!(ZipSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
}

#[test]
fn sqlite_snapshot_zip_named_guards_use_owned_header_metadata_without_wire_materialization(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🚦️subsets.json")).unwrap();
 for sample in cases["cases"].as_array().unwrap(){
  let mut snapshot=ZipSnapshot::default();let mut entry=ZipEntry::default();entry.name=cases["entryName"].as_str().unwrap().into();entry.metadata.compression_method=sample["compressionMethod"].as_u64().unwrap()as u16;entry.metadata.local.flags=sample["localFlags"].as_u64().unwrap()as u16;entry.metadata.central.flags=sample["centralFlags"].as_u64().unwrap()as u16;entry.metadata.local.version_needed=sample["localVersion"].as_u64().unwrap()as u16;entry.metadata.central.version_needed=sample["centralVersion"].as_u64().unwrap()as u16;entry.metadata.data_descriptor_signature=true;snapshot.entries.push(entry);
  let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.zip".into(),standard:"2.0".into(),subset:"iso21320".into()};
  let result=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let actual:Vec<serde_json::Value>=result.diagnostics.iter().map(|d|serde_json::json!({"code":d.code.0,"severity":format!("{:?}",d.severity)})).collect();assert_eq!(serde_json::json!(actual),sample["diagnostics"],"{}",sample["id"]);
  let logical=crate::standards::v2_0::subsets::iso21320::io::check_iso21320_conformance(&snapshot);assert_eq!(format!("{:?}",result.diagnostics),format!("{:?}",logical));
  let mut wildcard=dialect.clone();wildcard.subset="*".into();assert!(snapshot.validate_sqlite_snapshot_subset(&wildcard,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().diagnostics.is_empty());
  for row in cases["rejectedDialects"].as_array().unwrap(){let rejected=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:row["artifactKind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()};assert!(snapshot.validate_sqlite_snapshot_subset(&rejected,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
 }
}

#[test]
fn sqlite_snapshot_zip_named_guard_independent_sqlite_edits_and_bounded_cancel(){
 use std::{io::Write,process::{Command,Stdio}};
 let mut snapshot=ZipSnapshot::default();snapshot.entries.push(ZipEntry{name:"guard.bin".into(),..Default::default()});let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));db.run('UPDATE zip_local_header SET flags=1 WHERE id=1');db.run('UPDATE zip_entry SET compression_method=65535 WHERE id=1');if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,new Uint8Array(db.serialize()));";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let restored=ZipSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.zip".into(),standard:"2.0".into(),subset:"iso21320".into()};let result=restored.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored.entries[0].metadata.compression_method,65535);assert_eq!(result.diagnostics.len(),2);assert_eq!(result.diagnostics[0].code.0,"stdio.zip.iso21320.entry-encrypted");assert_eq!(result.diagnostics[0].severity,semio_framework_diagnostic::Severity::Error);assert_eq!(result.diagnostics[1].code.0,"stdio.zip.iso21320.compression-method-unsupported");assert_eq!(result.diagnostics[1].severity,semio_framework_diagnostic::Severity::Error);
 let mut wrong=restored.clone();wrong.schema="wrong".into();assert!(wrong.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
 snapshot.entries=vec![ZipEntry::default();600];let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut reached=false;let result=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |event|{if event.completed==256{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(reached);assert!(result.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_zip_named_registered_payload_validator_reads_typed_pack_and_text(){
 use semio_framework_plugin::io::SubsetValidator;
 let mut snapshot=ZipSnapshot::default();snapshot.entries.push(ZipEntry{name:"guard.bin".into(),..Default::default()});snapshot.entries[0].metadata.local.flags=1;
 let payloads=[store::io_schema::IoPayload::Binary(<ZipSnapshot as store::ArtifactPack>::encode_pack(&snapshot)),store::io_schema::IoPayload::Text(<ZipSnapshot as store::ArtifactDsl>::print_dsl(&snapshot))];
 for payload in payloads{let diagnostics=crate::standards::v2_0::subsets::iso21320::io::ZipIso21320Validator::validate(&payload).await;assert_eq!(diagnostics.len(),1,"{diagnostics:?}");assert_eq!(diagnostics[0].code.0,"stdio.zip.iso21320.entry-encrypted");assert_eq!(diagnostics[0].severity,semio_framework_diagnostic::Severity::Error);}
}

#[test]
fn sqlite_snapshot_zip_full_unsigned16_method_domain_is_distinct_from_iso_and_native_wire(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🚦️subsets.json")).unwrap();
 for method in cases["methodCodes"].as_array().unwrap(){let method=method.as_u64().unwrap()as u16;let mut snapshot=ZipSnapshot::default();snapshot.entries.push(ZipEntry{name:"guard.bin".into(),..Default::default()});snapshot.entries[0].metadata.compression_method=method;let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(database.table("zip_entry").unwrap().rows[0].integer(4).unwrap(),i64::from(method));let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let restored=ZipSnapshot::from_sqlite_database(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored,snapshot);let mut dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.zip".into(),standard:"2.0".into(),subset:"*".into()};assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().diagnostics.is_empty());dialect.subset="iso21320".into();let diagnostics=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().diagnostics;assert_eq!(diagnostics.iter().any(|d|d.code.0=="stdio.zip.iso21320.compression-method-unsupported"&&d.severity==semio_framework_diagnostic::Severity::Error),!matches!(method,0|8));assert_eq!(crate::standards::v2_0::subsets::base::io::encode_zip(&snapshot).is_ok(),matches!(method,0|8));}
}

#[test]
fn sqlite_snapshot_zip_erased_binary_and_text_keep_all_unsigned16_methods_and_noncanonical_metadata(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🚦️subsets.json")).unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.zip".into(),standard:"2.0".into(),subset:"*".into()};let codec=<ZipSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
 for method in cases["methodCodes"].as_array().unwrap(){let mut snapshot=fixture();snapshot.schema="owned noncanonical snapshot".into();snapshot.entries[0].metadata.compression_method=method.as_u64().unwrap()as u16;snapshot.entries[0].metadata.local.flags=65535;snapshot.entries[1].metadata.central.version_needed=65535;let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=(codec.import)(&snapshot.schema,&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=(codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(restored,database);assert_eq!(ZipSnapshot::from_sqlite_database(&restored,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);}
 }


}

#[test]
fn sqlite_snapshot_zip_native_encoding_admits_owned_model_and_refuses_budget_or_cancellation(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let snapshot=ZipSnapshot::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=1;
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}

#[test]
fn sqlite_snapshot_zip_native_encoding_bounds_escaped_text_and_cancels_borrowed_members(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️encoding.json")).unwrap();
 let mut snapshot=ZipSnapshot::default();snapshot.comment="\n\\\"".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize);snapshot.entries=vec![ZipEntry::default();cases["workItems"].as_u64().unwrap() as usize];
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=cases["smallBudgetBytes"].as_u64().unwrap() as usize;
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cases["cancelAfterWork"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached,"member admission walk must checkpoint before native ownership");
 }
}

fn native_cases()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🚦️native.json")).unwrap()}
fn native_payload(snapshot:&ZipSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io_schema::IoPayload{match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(snapshot))}}
#[test]
fn sqlite_snapshot_zip_declared_controlled_native_input_output_and_exact_rows(){
    use store::sqlite_snapshot::SnapshotEncoding;
    let cases=native_cases();let snapshot=fixture();let rows=cases["fixtureRows"].as_u64().unwrap() as usize;
    let limits=SqliteDatabaseLimits{max_rows:rows,..SqliteDatabaseLimits::default()};
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
        let payload=native_payload(&snapshot,encoding);
        assert_eq!(ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
        assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),payload);
        let short=SqliteDatabaseLimits{max_rows:rows-1,..limits};
        assert!(ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err());
        assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err());
        let tiny=SqliteDatabaseLimits{max_value_bytes:cases["tinyBytes"].as_u64().unwrap() as usize,..limits};
        assert!(ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,tiny)).is_err());
        assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,tiny)).is_err());
    }
}
#[test]
fn sqlite_snapshot_zip_owned_native_copy_and_collection_work_can_cancel_interior(){
    use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase,SqliteSnapshotProgress};
    let cases=native_cases();let total=cases["copyBytes"].as_u64().unwrap() as usize;let threshold=cases["copyCancelAfter"].as_u64().unwrap() as usize;
    let mut snapshot=fixture();snapshot.schema="x".repeat(total);
    let limits=SqliteDatabaseLimits{max_value_bytes:cases["maximumBytes"].as_u64().unwrap() as usize,..SqliteDatabaseLimits::default()};
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
        let payload=native_payload(&snapshot,encoding);
        assert_eq!(ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
        assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),payload);
        for phase in [SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{
            let mut interior=false;let mut callback=|event:SqliteSnapshotProgress|{if event.phase==phase&&event.total==total&&event.completed>=threshold&&event.completed<total{interior=true;false}else{true}};
            let mut control=SqliteSnapshotControl::new(&mut callback,limits);
            let canceled=match phase{SqliteSnapshotPhase::DecodeNative=>ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut control).is_err(),_=>snapshot.encode_sqlite_snapshot_native(encoding,&mut control).is_err()};
            assert!(canceled);assert!(interior,"actual owned ZIP string copy {phase:?}");
        }
    }
    let count=cases["collectionItems"].as_u64().unwrap() as usize;let threshold=cases["cancelAfter"].as_u64().unwrap() as usize;
    snapshot.schema=String::new();snapshot.entries=vec![ZipEntry::default();count];
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
        let payload=native_payload(&snapshot,encoding);
        for phase in [SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{
            let mut interior=false;let mut callback=|event:SqliteSnapshotProgress|{if event.phase==phase&&event.total==count&&event.completed>=threshold&&event.completed<count{interior=true;false}else{true}};
            let mut control=SqliteSnapshotControl::new(&mut callback,limits);
            let canceled=match phase{SqliteSnapshotPhase::DecodeNative=>ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut control).is_err(),_=>snapshot.encode_sqlite_snapshot_native(encoding,&mut control).is_err()};
            assert!(canceled);assert!(interior,"actual ZIP member construction {phase:?}");
        }
    }
}

fn norm_complete_source(case:&serde_json::Value)->ZipSnapshot{let mut source=fixture();if case["id"]=="emptyEntries"{source.entries.clear();}else if case["id"]=="absentEntryMetadata"{for entry in &mut source.entries{entry.metadata=ZipEntryMetadata::default();}}source}
fn norm_independent_complete_extent(source:&ZipSnapshot)->serde_json::Value{
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nawait Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(new Uint8Array(await Bun.stdin.arrayBuffer()))));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_zip_complete_independent_native_semantic_limits(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎛️semantic.json")).unwrap();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);let limits=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:12,max_columns:11,..SqliteDatabaseLimits::default()};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(ZipSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),source);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:11,..limits},SqliteDatabaseLimits{max_columns:10,..limits}]{assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"relational copied limits {short:?}");assert!(ZipSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"reconstruct copied limits {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight copied limits {encoding:?} {short:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"encoder copied limits {encoding:?} {short:?}");assert!(ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"decoder copied limits {encoding:?} {short:?}");}
  }
 }eprintln!("[DEBUG] ZIP independently measured complete copied native semantic limits");
}
#[test]
fn sqlite_snapshot_zip_complete_independent_copied_columns_admission(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let source=fixture();let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_columns:10,..SqliteDatabaseLimits::default()};assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(ZipSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"preflight copied columns {encoding:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"encoder copied columns {encoding:?}");assert!(ZipSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"decoder copied columns {encoding:?}");}
}
