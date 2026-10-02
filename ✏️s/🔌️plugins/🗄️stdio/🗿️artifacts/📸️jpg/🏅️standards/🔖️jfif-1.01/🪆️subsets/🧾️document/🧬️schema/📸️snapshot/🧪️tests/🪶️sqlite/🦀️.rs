use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteValue},ArtifactSqliteSnapshot};

fn fixture()->JpgSnapshot{pack::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn roundtrip(snapshot:&JpgSnapshot)->JpgSnapshot{let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();JpgSnapshot::from_sqlite_database(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}

fn intermediate_fixture()->JpgSnapshot{
 let case:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️ownership/🔣️.json")).unwrap();let mut snapshot=fixture();snapshot.width=u32::try_from(case["width"].as_u64().unwrap()).unwrap();snapshot.height=u32::try_from(case["height"].as_u64().unwrap()).unwrap();snapshot.pixels=case["pixels"].as_array().unwrap().iter().map(|value|u8::try_from(value.as_u64().unwrap()).unwrap()).collect();snapshot.jfif_thumbnail=Some(pack::json::from_json_str(&case["jfifThumbnail"].to_string()).unwrap());snapshot
}
#[test]
fn sqlite_snapshot_jpg_owned_intermediate_dimensions_preserve_partial_pixels(){let snapshot=intermediate_fixture();assert_eq!(roundtrip(&snapshot),snapshot);}

#[test]
fn sqlite_snapshot_jpg_complete_neutral_corpus_and_optional_empty_entities(){
 let snapshot=fixture();assert_eq!(roundtrip(&snapshot),snapshot);let oracle:serde_json::Value=serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&snapshot))).unwrap();assert_eq!(oracle,serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());let mut zero_id=snapshot.clone();zero_id.quant_tables[0].id=0;zero_id.frame.as_mut().unwrap().components[0].quant_table_id=0;assert_eq!(roundtrip(&zero_id),zero_id);
 for units in [JfifDensityUnits::Aspect,JfifDensityUnits::PixelsPerInch,JfifDensityUnits::PixelsPerCm]{for present in [false,true]{let snapshot=JpgSnapshot{schema:"custom 世界".into(),width:u32::MAX,height:0,pixels:Vec::new(),jfif_density_units:units,jfif_thumbnail:present.then_some(JfifThumbnail{width:0,height:255,rgb_data:Vec::new()}),frame:present.then_some(JpgFrameHeader{precision:255,width:0,height:u16::MAX,components:Vec::new()}),re_encode_quality:present.then_some(0),restart_interval:present.then_some(u16::MAX),huffman_tables:vec![JpgHuffmanTable{id:0,class:JpgHuffmanClass::Ac,bits:[0;16],values:Vec::new()}],..JpgSnapshot::default()};assert_eq!(roundtrip(&snapshot),snapshot);}}
}

#[test]
fn sqlite_snapshot_jpg_independent_sqlite_entities_and_editable_relationships(){
 use std::{io::Write,process::{Command,Stdio}};
 let mut expected=fixture();let database=expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT c.ordinal,c.component_id,q.id AS quantizer_id,COUNT(t.id) AS definition_count FROM jpg_frame_component c JOIN jpg_quantizer q ON q.id=c.quantizer_id LEFT JOIN jpg_quantization_table t ON t.quantizer_id=q.id GROUP BY c.id ORDER BY c.ordinal').all();if(rows.length!==2||rows[0].definition_count!==2||rows[1].quantizer_id!==42||rows[1].definition_count!==0)throw Error('relationships');db.run('UPDATE jpg_rgba_pixel SET red=42 WHERE ordinal=0');db.run('UPDATE jpg_quantization_coefficient SET coefficient=12345 WHERE table_id=1 AND zigzag_ordinal=63');db.run('UPDATE jpg_huffman_symbol SET symbol=99 WHERE table_id=1 AND ordinal=1');db.run('UPDATE jpg_segment_octet SET octet=17 WHERE segment_id=1 AND ordinal=2');await Bun.write(Bun.stdout,db.serialize());db.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));expected.pixels[0]=42;expected.quant_tables[0].values[63]=12345;expected.huffman_tables[0].values[1]=99;expected.other_segments[0].data[2]=17;
 assert_eq!(JpgSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
}

#[test]
fn sqlite_snapshot_jpg_refuses_malformed_relations_and_bounds_expensive_work(){
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 for(table,row,column,value)in [("jpg_rgba_pixel",0,1,SqliteValue::Integer(99)),("jpg_rgba_pixel",0,4,SqliteValue::Null),("jpg_rgba_pixel",1,4,SqliteValue::Null),("jpg_rgba_pixel",0,2,SqliteValue::Integer(2)),("jpg_rgba_pixel",1,2,SqliteValue::Integer(0)),("jpg_frame_component",1,2,SqliteValue::Integer(0)),("jpg_quantization_coefficient",0,1,SqliteValue::Integer(99)),("jpg_huffman_table",0,4,SqliteValue::Text("unknown".into())),("jpg_document",0,4,SqliteValue::Integer(256))]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows[row].values[column]=value;assert!(JpgSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}
 for table in ["jpg_quantization_coefficient","jpg_huffman_code_length","jpg_thumbnail"]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows.remove(0);assert!(JpgSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}
 assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()})).is_err());assert!(JpgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:0,..SqliteDatabaseLimits::default()})).is_err());
 let large=JpgSnapshot{width:1000,height:1,pixels:vec![1;4000],..snapshot};let mut checkpoints=0;assert!(large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |event|{checkpoints+=1;event.completed<256},SqliteDatabaseLimits::default())).is_err());assert!(checkpoints>0);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_jpg_actual_declaration_preserves_owned_fields_and_baseline_diagnostics(){
 use semio_framework_os_kernel::{io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}},sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase}};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("JPG SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let mut snapshot=fixture();snapshot.schema="owned 世界".into();let any:ArtifactDialect=crate::JPG_ANY_DIALECT.into();let baseline:ArtifactDialect=crate::JPG_BASELINE_DIALECT.into();let mut phases=Vec::new();
 for dialect in [&any,&baseline]{let output=io_export_sqlite_snapshot(dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap();if dialect==&baseline{assert!(output.diagnostics.iter().any(|diagnostic|diagnostic.severity==dsl::Severity::Warning));}assert_eq!(io_import_sqlite_snapshot::<JpgSnapshot>(dialect,&output.value,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);}
 assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
 snapshot.sof_marker=194;let error=io_export_sqlite_snapshot(&baseline,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap_err();assert!(error.diagnostics.iter().any(|diagnostic|diagnostic.code.0=="stdio.jpg.baseline.sof-marker"));
 let mut database=fixture().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();database.table_mut("jpg_document").unwrap().rows[0].values[10]=SqliteValue::Integer(194);let restored=JpgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(restored.validate_sqlite_snapshot_subset(&baseline,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_jpg_baseline_validation_cancels_during_owned_component_scan(){
 use crate::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance_with;
 let mut snapshot=fixture();let diagnostics=check_baseline_conformance_with(&snapshot,&mut |_,_|Ok(())).unwrap();assert_eq!(diagnostics.len(),1);assert_eq!(diagnostics[0].code.0,"stdio.jpg.baseline.component-sampling");assert_eq!(diagnostics[0].severity,dsl::Severity::Warning);
 snapshot.frame.as_mut().unwrap().components=vec![snapshot.frame.as_ref().unwrap().components[0];2000];let mut reached=false;assert!(check_baseline_conformance_with(&snapshot,&mut |position,total|{if position==256&&total==2000{reached=true;Err("cancelled".into())}else{Ok(())}}).is_err());assert!(reached);
}

#[test]
fn sqlite_snapshot_jpg_actual_erased_records_retain_all_owned_fields() {
 use semio_framework_os_kernel::{io_schema::ArtifactDialect,sqlite_snapshot::SnapshotEncoding};
 let mut complete=fixture();complete.schema="owned 世界\0".into();complete.re_encode_quality=Some(0);complete.sof_marker=255;complete.arithmetic=true;complete.jfif_version=(0,255);
 let absent=JpgSnapshot{schema:"independent empty state".into(),width:u32::MAX,height:0,pixels:Vec::new(),..JpgSnapshot::default()};
 let empty=JpgSnapshot{jfif_thumbnail:Some(JfifThumbnail{width:0,height:255,rgb_data:Vec::new()}),frame:Some(JpgFrameHeader{precision:255,width:0,height:u16::MAX,components:Vec::new()}),restart_interval:Some(0),..absent.clone()};
 let codec=(crate::native_codecs()[0].codec)();let provider=codec.snapshot_sqlite.expect("JPG owner SQLite provider");let dialect:ArtifactDialect=crate::JPG_ANY_DIALECT.into();
 for snapshot in [complete,absent,empty,intermediate_fixture()]{
  let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let payload=(provider.import)(&codec.schema,&dialect,expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
   let restored=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
   assert_eq!(restored,expected,"all seventeen owned JPG fields survive the erased record bridge");
  }
 }
}

#[test]
fn sqlite_snapshot_jpg_owned_native_controls_physical_and_typed_materialization(){
 use store::{ArtifactDsl,ArtifactPack,ArtifactSqliteSnapshot};use semio_framework_os_kernel::{io_schema::IoPayload,sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase}};
 let mut snapshot=fixture();snapshot.schema="logical 世界\0".into();snapshot.other_segments=vec![JpgSegment{marker:255,data:vec![0,255,128]};2048];
 for payload in [IoPayload::Text(snapshot.print_dsl()),IoPayload::Binary(snapshot.encode_pack())]{let restored=JpgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored,snapshot);let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=128;assert!(JpgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());let mut canceled=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|event.phase!=SqliteSnapshotPhase::DecodeNative||event.completed<256||event.completed>=event.total;assert!(JpgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut canceled,SqliteDatabaseLimits::default())).is_err());}
 let record=owned_text::to_record(&snapshot);let mut interior=false;let mut callback=|event:semio_framework_os_kernel::native_decoding::NativeDecodeProgress|{if event.total==2048&&event.completed==256{interior=true;false}else{true}};let mut control=dsl::NativeDecodeControl::new(10_000_000,&mut callback);assert!(owned_text::from_record_controlled(&record,&mut control).is_err());drop(control);assert!(interior);
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=1024;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());}
 let foreign=IoPayload::Text(snapshot.print_dsl().replacen("stdio.jpg.dsl","stdio.png.dsl",1));assert!(JpgSnapshot::decode_sqlite_snapshot_native(&foreign,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_jpg_native_factory_has_declared_structural_identity(){
 let codec=(crate::native_codecs()[0].codec)();let hash=codec.pack_schema_hash.iter().map(|byte|format!("{byte:02x}")).collect::<String>();let definition:serde_json::Value=serde_json::from_str(crate::ARTIFACT_DEFINITION_SCHEMA).unwrap();assert_eq!(hash,definition["codecs"][0]["native_factory"]["pack_schema_hash"].as_str().unwrap());
}


#[test]
fn sqlite_snapshot_jpg_controlled_output_admits_owned_octets_and_cancels_inside_copy(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase as Phase,SqliteSnapshotProgress};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️encoding/🔣️.json")).unwrap();
 let mut snapshot=fixture();snapshot.schema="owned JPG 世界\0".into();snapshot.other_segments[0].data=vec![255;plan["octets"].as_u64().unwrap()as usize];
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let limits=SqliteDatabaseLimits::default();let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert!(JpgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap()==snapshot);
  let mut interior=false;let mut callback=|event:SqliteSnapshotProgress|if event.phase==Phase::EncodeNative&&event.total==plan["octets"].as_u64().unwrap()as usize&&event.completed>=plan["cancelAfter"].as_u64().unwrap()as usize&&event.completed<event.total{interior=true;false}else{true};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut callback,limits)).is_err());assert!(interior);
  let mut small=limits;small.max_value_bytes=plan["valueBudget"].as_u64().unwrap()as usize;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,small)).is_err());small=limits;small.max_rows=plan["rowBudget"].as_u64().unwrap()as usize;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,small)).is_err());
 }
 let script=r#"import Ajv from 'ajv';import {Database}from'bun:sqlite';const input=JSON.parse(process.argv[1]);if(!new Ajv({strict:true}).validate(input.schema,input.plan))throw Error('schema');const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const row=db.query('SELECT COUNT(*) AS count,MIN(octet) AS minimum,MAX(octet) AS maximum,MAX(ordinal) AS last FROM jpg_segment_octet WHERE segment_id=1').get();if(row.count!==input.plan.octets||row.last!==input.plan.octets-1||row.minimum!==255||row.maximum!==255)throw Error('owned octets');console.log('owned JPG output oracle');"#;
 let input=serde_json::json!({"plan":plan,"schema":serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🛫️encoding/🧬️schema/🔣️.json")).unwrap()});
 use std::{io::Write,process::{Command,Stdio}};let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let mut child=Command::new("bun").args(["-e",script,&input.to_string()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert!(String::from_utf8_lossy(&output.stdout).contains("owned JPG output oracle"));
}
