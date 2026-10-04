use super::*;
#[path = "💰️backing/🦀️.rs"]
mod owned_requests;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::*};
fn deep_owned_document(depth:usize)->semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlDocument{
 use semio_s_artifact_stdio_xml::schema::snapshot::retained::{RetainedXmlDocument,RetainedXmlNode,RetainedXmlNodeKind,RetainedXmlText};
 let mut document=RetainedXmlDocument::default();
 document.nodes.try_push(RetainedXmlNode{next_sibling:None,value:RetainedXmlNodeKind::Text{text:RetainedXmlText::try_from_str("leaf").unwrap()}}).unwrap();
 for child in 0..depth{document.nodes.try_push(RetainedXmlNode{next_sibling:None,value:RetainedXmlNodeKind::Element{name:RetainedXmlText::try_from_str("node").unwrap(),first_attribute:0,attribute_count:0,first_child:Some(child)}}).unwrap();}
 document.root=Some(depth);document.validate().unwrap();document
}
fn profile_fixture(plan:&serde_json::Value)->DocxSnapshot{
 use semio_s_artifact_stdio_zip::opc::{OpcPackage,OpcRelationship,OpcTargetMode};
 let mut opc=OpcPackage::default();opc.relationships.replace_owner(String::new(),vec![OpcRelationship{id:"main".into(),rel_type:format!("{}/officeDocument",plan["relationshipBase"].as_str().unwrap()),target:"word/document.xml".into(),target_mode:OpcTargetMode::Internal}]);
 let opc=semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage::try_from_package(opc).unwrap();
 DocxSnapshot{schema:"literal owned schema".into(),opc,xml_parts:crate::schema::snapshot::DocxXmlParts::try_from_iter([DocxXmlPart{path:"word/document.xml".into(),content_type:"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml".into(),document:semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlDocument::try_from_document(&semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text(plan["xml"].as_str().unwrap()).unwrap()).unwrap()}]).unwrap()}
}
#[test]
fn sqlite_snapshot_docx_exact_profile_policies_use_typed_namespace_entities(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛡️profile/🔣️.json")).unwrap();let limits=SqliteDatabaseLimits::default();
 for case in plan["cases"].as_array().unwrap(){let snapshot=profile_fixture(case);let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.stdio.docx".into(),standard:"ecma-376".into(),subset:case["subset"].as_str().unwrap().into()};let db=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let result=snapshot.validate_sqlite_snapshot_subset(&dialect,&db,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(result.is_ok(),case["codes"].as_array().unwrap().len()==case["warnings"].as_u64().unwrap()as usize,"{}",case["id"]);let diagnostics=match result{Ok(outcome)=>outcome.diagnostics,Err(error)=>error.diagnostics};assert_eq!(diagnostics.iter().map(|item|item.code.0.as_str()).collect::<Vec<_>>(),case["codes"].as_array().unwrap().iter().map(|item|item.as_str().unwrap()).collect::<Vec<_>>(),"{}",case["id"]);assert_eq!(diagnostics.iter().filter(|item|item.severity==semio_framework_diagnostic::Severity::Warning).count(),case["warnings"].as_u64().unwrap()as usize);snapshot.retire_sqlite_snapshot();}
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_docx_actual_exact_profile_declarations_preserve_owned_warnings(){
 use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("DOCX exact profiles").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛡️profile/🔣️.json")).unwrap();let limits=SqliteDatabaseLimits::default();
 for case in plan["cases"].as_array().unwrap().iter().filter(|case|case["codes"].as_array().unwrap().len()==case["warnings"].as_u64().unwrap()as usize){let snapshot=profile_fixture(case);let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.stdio.docx".into(),standard:"ecma-376".into(),subset:case["subset"].as_str().unwrap().into()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let exported=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,limits,&mut |_|true).await.unwrap();assert_eq!(exported.diagnostics.len(),case["warnings"].as_u64().unwrap()as usize);let restored=io_import_sqlite_snapshot::<DocxSnapshot>(&dialect,&exported.value,limits,&mut |_|true).await.unwrap();assert_eq!(restored.value,snapshot);assert_eq!(restored.diagnostics.len(),exported.diagnostics.len());let foreign=store::io_schema::ArtifactDialect{subset:if dialect.subset=="strict"{"transitional".into()}else{"strict".into()},..dialect.clone()};assert!(io_import_sqlite_snapshot::<DocxSnapshot>(&foreign,&exported.value,limits,&mut |_|true).await.is_err());restored.value.retire_sqlite_snapshot();}snapshot.retire_sqlite_snapshot();}
}
#[test]
fn sqlite_snapshot_docx_native_protocol_checks_complete_literal_fields(){
 let snapshot=fixture();let bytes=<DocxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);let(_,body)=store::semio_format::unwrap_binary(&bytes).unwrap();
 let protocol=semio_framework_dsl::parse_protocol(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO).unwrap();
 assert_eq!(semio_framework_dsl::walk_protocol(&protocol,&body).unwrap().consumed,body.len());
 for length in[0,1,body.len()-1]{assert!(semio_framework_dsl::walk_protocol(&protocol,&body[..length]).is_err());}
 let mut trailing=body.clone();trailing.push(0);assert!(semio_framework_dsl::walk_protocol(&protocol,&trailing).is_err());snapshot.retire_sqlite_snapshot();
}
#[test]
fn sqlite_snapshot_docx_native_grammar_describes_complete_owned_fields(){
 let snapshot=fixture();let text=<DocxSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);let(envelope,body)=store::semio_format::split_text_preamble(&text).unwrap();let grammar=semio_framework_dsl::parse_grammar(crate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO).unwrap();assert!(semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments").recognize(&format!("{}\n{body}",envelope.envelope_id())).unwrap());snapshot.retire_sqlite_snapshot();
}
fn fixture_position_u64(text:&str)->Option<u64>{
 if text.is_empty()||!text.bytes().all(|byte|byte.is_ascii_digit())||(text.len()>1&&text.starts_with('0')){return None;}
 text.parse::<u64>().ok()
}
fn fixture()->DocxSnapshot{
 fn field<'a>(value:&'a mut semio_framework_value::DslValue,name:&str)->&'a mut semio_framework_value::DslValue{
  let semio_framework_value::DslValue::Object(fields)=value else{panic!("closed fixture object required")};
  fields.iter_mut().find(|(key,_)|key==name).map(|(_,value)|value).expect("closed fixture required field")
 }
 let mut value:semio_framework_value::DslValue=semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let semio_framework_value::DslValue::Array(parts)=field(&mut value,"xmlParts") else{panic!("closed fixture XML parts required")};
 for part in parts{
  let doctype=field(field(part,"document"),"doctype");
  if matches!(doctype,semio_framework_value::DslValue::Null){continue;}
  let position=field(doctype,"prologPosition");
  let semio_framework_value::DslValue::String(decimal)=position else{panic!("closed retained fixture decimal position required")};
  let number=fixture_position_u64(decimal).expect("closed retained fixture canonical u64 decimal");
  *position=semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(number));
 }
 <DocxSnapshot as semio_framework_value::FromValue>::from_value(value).expect("complete authored native DOCX fixture")
}
#[test]
fn sqlite_snapshot_docx_closed_fixture_position_preserves_exact_unsigned_domain(){
 assert_eq!(fixture_position_u64("0"),Some(0));
 assert_eq!(fixture_position_u64("18446744073709551615"),Some(u64::MAX));
 for text in["","00","01","+1","-1"," 1","1 ","1.0","18446744073709551616","１"]{assert_eq!(fixture_position_u64(text),None,"{text}");}
 let mut snapshot=fixture();
 assert_eq!(snapshot.xml_parts[0].document.doctype.as_ref().unwrap().prolog_position,u64::MAX);
 let external=snapshot.xml_parts[0].document.doctype.as_ref().unwrap().external_id.as_ref().unwrap();
 let public_wire=semio_framework_value::ToValue::to_value(external);
 assert_eq!(public_wire.get("publicId"),Some(&semio_framework_value::DslValue::String("public".into())));assert_eq!(public_wire.get("systemId"),Some(&semio_framework_value::DslValue::String("system".into())));assert!(public_wire.get("public_id").is_none());assert!(public_wire.get("system_id").is_none());
 let semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlExternalId::Public{public_id,system_id}=external else{panic!("complete authored PUBLIC fixture required")};
 assert_eq!(semio_framework_value::ToValue::to_value(public_id),semio_framework_value::DslValue::String("public".into()));assert_eq!(semio_framework_value::ToValue::to_value(system_id),semio_framework_value::DslValue::String("system".into()));
 let system=semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlExternalId::System{system_id:semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlText::try_from_str("SYSTEM\0引用😀").unwrap()};
 let system_wire=semio_framework_value::ToValue::to_value(&system);assert_eq!(system_wire.get("systemId"),Some(&semio_framework_value::DslValue::String("SYSTEM\0引用😀".into())));assert!(system_wire.get("system_id").is_none());
 snapshot.xml_parts[0].document.doctype.as_mut().unwrap().external_id=Some(system);
 let bytes=<DocxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
 let restored=<DocxSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap();
 assert_eq!(restored,snapshot);restored.retire_sqlite_snapshot();
 let text=<DocxSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
 let restored=<DocxSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap();
 assert_eq!(restored,snapshot);restored.retire_sqlite_snapshot();
 snapshot.retire_sqlite_snapshot();
}
#[test]
fn sqlite_snapshot_docx_all_owned_fields_and_literal_package_domains(){let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=DocxSnapshot::from_sqlite_database(&db,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(snapshot,restored);assert_eq!(db.tables.len(),21);}
#[test]
fn sqlite_snapshot_docx_exact_aggregate_row_budget(){let snapshot=fixture();let defaults=SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();let count=db.tables.iter().map(|table|table.rows.len()).sum::<usize>();let exact=SqliteDatabaseLimits{max_rows:count,..defaults};assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,exact)).is_ok());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:count-1,..defaults})).is_err());}

#[test]
fn sqlite_snapshot_docx_rejects_dangling_and_shared_package_owners(){let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();for(table,row,column,value)in[("docx_document",0,2,SqliteValue::Integer(2)),("docx_xml_part",1,4,SqliteValue::Integer(1)),("docx_relationship",0,1,SqliteValue::Integer(99)),("docx_default_content_type",0,2,SqliteValue::Integer(99)),("docx_relationship_owner",1,2,SqliteValue::Text("".into()))]{let mut invalid=db.clone();invalid.table_mut(table).unwrap().rows[row].values[column]=value;assert!(DocxSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"{table}");}}
#[test]
fn sqlite_snapshot_docx_literal_large_part_bytes_have_interior_controls(){let mut snapshot=fixture();let path=snapshot.opc.parts.get(0).unwrap().path.to_string_owner();let content_type=snapshot.opc.parts.get(0).unwrap().content_type.to_string_owner();snapshot.opc.set_part(&path,&content_type,vec![0x97;100000]).unwrap();let limits=SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut observed=false;let mut callback=|event:SqliteSnapshotProgress|{if event.phase==phase&&event.total==100000&&event.completed>=65536&&event.completed<event.total{observed=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,limits);let failed=if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).is_err()}else{DocxSnapshot::from_sqlite_database(&db,&mut control).is_err()};assert!(failed);assert!(observed);}}
#[test]
fn sqlite_snapshot_docx_partial_part_reconstruction_retires_deep_completed_documents(){std::thread::Builder::new().stack_size(256*1024).spawn(||{struct Owned(Option<DocxSnapshot>);impl Drop for Owned{fn drop(&mut self){if let Some(snapshot)=self.0.take(){snapshot.retire_sqlite_snapshot();}}}let mut snapshot=fixture();snapshot.xml_parts[0].document=deep_owned_document(8192);snapshot.xml_parts[1].content_type="z".repeat(100000);let snapshot=Owned(Some(snapshot));let limits=SqliteDatabaseLimits::default();let db=snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let mut observed=false;assert!(DocxSnapshot::from_sqlite_database(&db,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::ReconstructSnapshot&&event.total==100000&&event.completed>=65536&&event.completed<event.total{observed=true;false}else{true}},limits)).is_err());assert!(observed);let restored=DocxSnapshot::from_sqlite_database(&db,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();restored.retire_sqlite_snapshot();}).unwrap().join().unwrap();}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_docx_actual_declaration_exposes_complete_typed_snapshot(){use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("DOCX SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let dialect=ArtifactDialect{artifact_kind:"s.stdio.docx".into(),standard:"ecma-376".into(),subset:"*".into()};let snapshot=fixture();let file=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let restored=io_import_sqlite_snapshot::<DocxSnapshot>(&dialect,&file,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert_eq!(restored,snapshot);restored.retire_sqlite_snapshot();snapshot.retire_sqlite_snapshot();}

#[test]
fn sqlite_snapshot_docx_actual_erased_native_boundaries_retain_all_owned_fields(){let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.stdio.docx".into(),standard:"ecma-376".into(),subset:"*".into()};let codec=<DocxSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let db=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=(codec.import)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,db,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;let projected=(codec.export)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;let restored=DocxSnapshot::from_sqlite_database(&projected,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);restored.retire_sqlite_snapshot();}snapshot.retire_sqlite_snapshot();}
#[test]
fn sqlite_snapshot_docx_deep_erased_native_input_output_are_interior_cancellable(){let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️native/🔣️.json")).unwrap();std::thread::Builder::new().stack_size(plan["smallStackBytes"].as_u64().unwrap()as usize).spawn(move||{struct Owner(Option<DocxSnapshot>);impl Drop for Owner{fn drop(&mut self){if let Some(snapshot)=self.0.take(){snapshot.retire_sqlite_snapshot();}}}let mut snapshot=fixture();snapshot.xml_parts[0].document=deep_owned_document(usize::try_from(plan["depth"].as_u64().unwrap()).unwrap());snapshot.xml_parts[1].content_type="z".repeat(plan["lateTextBytes"].as_u64().unwrap()as usize);let snapshot=Owner(Some(snapshot));let limits=SqliteDatabaseLimits::default();let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.stdio.docx".into(),standard:"ecma-376".into(),subset:"*".into()};let codec=<DocxSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let db=snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let expected_database=db.clone();let payload=(codec.import)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,db,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;let projected=(codec.export)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert!(projected==expected_database,"complete deep native graph differs");assert_eq!(projected.table("docx_xml_element").unwrap().rows.iter().filter(|row|row.text(1).unwrap()=="node").count(),plan["depth"].as_u64().unwrap()as usize);assert_eq!(projected.table("docx_xml_text").unwrap().rows.iter().filter(|row|row.text(1).unwrap()=="leaf").count(),1);for phase in[SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{let mut reached=false;let mut callback=|event:SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=plan["cancelAfter"].as_u64().unwrap()as usize&&event.completed<event.total{reached=true;false}else{true}};let failed=if phase==SqliteSnapshotPhase::EncodeNative{let db=snapshot.0.as_ref().unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();(codec.import)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,db,encoding,&mut SqliteSnapshotControl::new(&mut callback,limits)).is_err()}else{(codec.export)(crate::STDIO_DOCX_DOCUMENT_SCHEMA,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut callback,limits)).is_err()};assert!(failed);assert!(reached,"actual interior native phase {phase:?}");}}}).unwrap().join().unwrap();}

#[test]
fn sqlite_snapshot_docx_native_exact_file_and_entity_admission(){
 let snapshot=fixture();let defaults=SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,defaults)).unwrap();let bytes=match &payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};let exact=SqliteDatabaseLimits{max_file_bytes:bytes,max_rows:rows,..defaults};assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap(),payload);let restored=DocxSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();assert_eq!(restored,snapshot);restored.retire_sqlite_snapshot();
 for limited in[SqliteDatabaseLimits{max_file_bytes:bytes-1,..exact},SqliteDatabaseLimits{max_rows:rows-1,..exact}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());assert!(DocxSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());}
 }snapshot.retire_sqlite_snapshot();
}

#[test]
fn sqlite_snapshot_docx_borrowed_preflight_pays_frontiers_without_materializing_output(){
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,limits);
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).expect("actual complete owner must expose borrowed preflight");
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_allocation_bytes:1,..limits})).unwrap_err();
  assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:0,..limits})).unwrap_err();
  assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::WorkLimit);
  let mut reached=false;let mut callback=|event:SqliteSnapshotProgress|{let cancel=event.phase==SqliteSnapshotPhase::EncodeNative;reached|=cancel;!cancel};
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut callback,limits)).unwrap_err();
  assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert!(reached);
  let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
  let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_allocation_bytes:1,..limits})).unwrap_err();
  assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
  let error=<DocxSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_allocation_bytes:1,..limits})).unwrap_err();
  assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
 }
 snapshot.retire_sqlite_snapshot();
}
