use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteValue},ArtifactSqliteSnapshot};

fn fixture()->JpgSnapshot{pack::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn roundtrip(snapshot:&JpgSnapshot)->JpgSnapshot{let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();JpgSnapshot::from_sqlite_database(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}

#[test]
fn sqlite_snapshot_jpg_complete_neutral_corpus_and_optional_empty_entities(){
 let snapshot=fixture();assert_eq!(roundtrip(&snapshot),snapshot);let oracle:serde_json::Value=serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&snapshot))).unwrap();assert_eq!(oracle,serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());let mut zero_id=snapshot.clone();zero_id.quant_tables[0].id=0;zero_id.frame.as_mut().unwrap().components[0].quant_table_id=0;assert_eq!(roundtrip(&zero_id),zero_id);
 for units in [JfifDensityUnits::Aspect,JfifDensityUnits::PixelsPerInch,JfifDensityUnits::PixelsPerCm]{for present in [false,true]{let snapshot=JpgSnapshot{schema:"custom 世界".into(),width:u32::MAX,height:0,pixels:Vec::new(),jfif_density_units:units,jfif_thumbnail:present.then_some(JfifThumbnail{width:0,height:255,rgb_data:Vec::new()}),frame:present.then_some(JpgFrameHeader{precision:255,width:0,height:u16::MAX,components:Vec::new()}),re_encode_quality:present.then_some(0),restart_interval:present.then_some(u16::MAX),huffman_tables:vec![JpgHuffmanTable{id:0,class:JpgHuffmanClass::Ac,bits:[0;16],values:Vec::new()}],..JpgSnapshot::default()};assert_eq!(roundtrip(&snapshot),snapshot);}}
}

#[test]
fn sqlite_snapshot_jpg_independent_sqlite_entities_and_editable_relationships(){
 use std::{io::Write,process::{Command,Stdio}};
 let mut expected=fixture();let database=expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT c.ordinal,c.component_id,q.id AS quantizer_id,COUNT(t.id) AS definition_count FROM jpg_frame_component c JOIN jpg_quantizer q ON q.id=c.quantizer_id LEFT JOIN jpg_quantization_table t ON t.quantizer_id=q.id GROUP BY c.id ORDER BY c.ordinal').all();if(rows.length!==2||rows[0].definition_count!==2||rows[1].quantizer_id!==42||rows[1].definition_count!==0)throw Error('relationships');db.run('UPDATE jpg_rgba_pixel SET red=42 WHERE x=0 AND y=0');db.run('UPDATE jpg_quantization_coefficient SET coefficient=12345 WHERE table_id=1 AND zigzag_ordinal=63');db.run('UPDATE jpg_huffman_symbol SET symbol=99 WHERE table_id=1 AND ordinal=1');db.run('UPDATE jpg_segment_octet SET octet=17 WHERE segment_id=1 AND ordinal=2');await Bun.write(Bun.stdout,db.serialize());db.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));expected.pixels[0]=42;expected.quant_tables[0].values[63]=12345;expected.huffman_tables[0].values[1]=99;expected.other_segments[0].data[2]=17;
 assert_eq!(JpgSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
}

#[test]
fn sqlite_snapshot_jpg_refuses_malformed_relations_and_bounds_expensive_work(){
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 for(table,row,column,value)in [("jpg_rgba_pixel",0,1,SqliteValue::Integer(99)),("jpg_rgba_pixel",0,2,SqliteValue::Integer(2)),("jpg_rgba_pixel",1,2,SqliteValue::Integer(0)),("jpg_frame_component",1,2,SqliteValue::Integer(0)),("jpg_quantization_coefficient",0,1,SqliteValue::Integer(99)),("jpg_huffman_table",0,4,SqliteValue::Text("unknown".into())),("jpg_document",0,4,SqliteValue::Integer(256))]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows[row].values[column]=value;assert!(JpgSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}
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
