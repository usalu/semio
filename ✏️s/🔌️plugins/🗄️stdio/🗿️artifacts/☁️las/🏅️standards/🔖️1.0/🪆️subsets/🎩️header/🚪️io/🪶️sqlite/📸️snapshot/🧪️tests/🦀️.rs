//! 🧫️ LAS semantic SQLite laws share typed vectors and independent SQLite queries with TypeScript.
use crate::standards::v1_0::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,export_sqlite_database,import_sqlite_database}};
use std::{io::Write,process::{Command,Stdio}};

fn snapshot()->LasSnapshot{
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let f=|index:usize|f64::from_bits(u64::from_str_radix(fixture["float64Bits"][index%9].as_str().unwrap(),16).unwrap());
    LasSnapshot{schema:fixture["schema"].as_str().unwrap().into(),header:LasHeader{version_major:255,version_minor:255,system_identifier:"owned 🌠".into(),generating_software:"SQL".into(),creation_day_of_year:u16::MAX,creation_year:u16::MAX,header_size:u16::MAX,offset_to_point_data:u32::MAX,number_of_vlrs:u32::MAX,point_data_format_id:255,point_data_record_length:u16::MAX,number_of_point_records:u32::MAX,points_by_return:[u32::MAX,0,17,17,1],x_scale:f(6),y_scale:f(7),z_scale:f(8),x_offset:f(1),y_offset:f(4),z_offset:f(5),max_x:f(3),min_x:f(2),max_y:f(0),min_y:f(6),max_z:f(7),min_z:f(8)},vlrs:fixture["vlrs"].as_array().unwrap().iter().map(|v|LasVlr{user_id:v["userId"].as_str().unwrap().into(),record_id:v["recordId"].as_u64().unwrap() as u16,description:v["description"].as_str().unwrap().into(),data:v["data"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect()}).collect(),points:vec![
      LasPoint{x:f(6),y:f(1),z:f(4),intensity:u16::MAX,return_number:255,number_of_returns:255,scan_direction_flag:true,edge_of_flight_line:false,classification:255,scan_angle_rank:-128,user_data:255,point_source_id:u16::MAX,gps_time:None,rgb:None},
      LasPoint{x:f(7),y:f(2),z:f(5),intensity:0,return_number:0,number_of_returns:0,scan_direction_flag:false,edge_of_flight_line:true,classification:0,scan_angle_rank:127,user_data:0,point_source_id:0,gps_time:Some(f(6)),rgb:None},
      LasPoint{x:f(8),y:f(3),z:f(0),intensity:17,return_number:1,number_of_returns:2,scan_direction_flag:false,edge_of_flight_line:false,classification:17,scan_angle_rank:0,user_data:17,point_source_id:17,gps_time:None,rgb:Some((0,u16::MAX,17))},
      LasPoint{x:f(1),y:f(4),z:f(5),intensity:17,return_number:7,number_of_returns:7,scan_direction_flag:true,edge_of_flight_line:true,classification:255,scan_angle_rank:-1,user_data:255,point_source_id:u16::MAX,gps_time:Some(f(1)),rgb:Some((u16::MAX,0,u16::MAX))}
    ]}
}
fn sqlite(bytes:&[u8],sql:&str)->Vec<u8>{let sql=serde_json::to_string(sql).unwrap();let script=format!("import{{Database}}from'bun:sqlite';const n=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if({sql}.trim())n.run({sql});if(n.query('PRAGMA integrity_check').get().integrity_check!=='ok'||n.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,n.serialize());n.close();");let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));output.stdout}

#[test]
fn sqlite_snapshot_full_header_vlr_points_ieee_and_edited_sqlite(){
    let snapshot=snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let native=import_sqlite_database(&sqlite(&bytes,""),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let restored=LasSnapshot::from_sqlite_database(&native,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),database);
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let edited=import_sqlite_database(&sqlite(&bytes,fixture["independentEdit"].as_str().unwrap()),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let restored=LasSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored.points[1].intensity,23);assert_eq!(restored.vlrs[0].data,vec![0,42,17,0]);assert!(restored.points[3].gps_time.is_none());assert_eq!(restored.header.number_of_point_records,u32::MAX);
    println!("[DEBUG] LAS full typed fields, stale structural metadata, exact IEEE words and independent SQLite edits verified");
}

#[test]
fn sqlite_snapshot_refuses_valid_sqlite_incoherent_identity_and_child_order(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let database=snapshot().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();for sql in fixture["invalidEdits"].as_array().unwrap(){let database=import_sqlite_database(&sqlite(&bytes,sql.as_str().unwrap()),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert!(LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
}

#[test]
fn sqlite_snapshot_owned_coordinates_controls_and_actual_declarations(){
    let snapshot=snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for (key,accepted) in [("acceptedDialects",true),("rejectedDialects",false)]{for row in fixture[key].as_array().unwrap(){let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:row["artifactKind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()};assert_eq!(snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_ok(),accepted);}}
    let limits=SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()};assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(<LasSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().is_some());
    let declaration=crate::declaration(crate::definition().unwrap()).unwrap();assert_eq!(declaration.hosted_kinds().unwrap().len(),1);assert_eq!(declaration.definition().identity().as_str(),"s.stdio.las");
}

#[test]
fn sqlite_snapshot_complete_typescript_native_relational_interoperability() {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(7).unwrap();
    let base=root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot");
    let fixture=base.join("🧪️tests/🧰️support/🟦️.ts");let provider=base.join("🟦️.ts");
    let physical=root.join("🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts");
    let export_script=format!("import{{lasSnapshotFixture as snapshot}}from{fixture:?};import{{lasSnapshotToSqliteDatabase}}from{provider:?};import{{exportSqliteDatabase}}from{physical:?};import{{Database}}from'bun:sqlite';const n=Database.deserialize(await exportSqliteDatabase(await lasSnapshotToSqliteDatabase(snapshot)));if(n.query('PRAGMA integrity_check').get().integrity_check!=='ok'||n.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,n.serialize());n.close();");
    let output=Command::new("bun").args(["-e",&export_script]).current_dir(root).output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let snapshot=LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let projected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(projected.table("las_point").unwrap().rows.len(),4);assert_eq!(projected.table("las_vlr").unwrap().rows.len(),2);assert_eq!(projected.table("las_return_histogram").unwrap().rows.len(),5);
    let bytes=export_sqlite_database(&projected,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let import_script=format!("import{{lasSnapshotFixture as snapshot}}from{fixture:?};import{{lasSnapshotFromSqliteDatabase,lasSnapshotToSqliteDatabase}}from{provider:?};import{{importSqliteDatabase}}from{physical:?};import assert from'node:assert/strict';import{{expect}}from'bun:test';import{{Database}}from'bun:sqlite';const n=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));assert.equal(n.query('PRAGMA integrity_check').get().integrity_check,'ok');assert.deepEqual(n.query('PRAGMA foreign_key_check').all(),[]);const restored=await lasSnapshotFromSqliteDatabase(await importSqliteDatabase(n.serialize()));expect(restored).toEqual(snapshot);assert.deepEqual(await lasSnapshotToSqliteDatabase(restored),await lasSnapshotToSqliteDatabase(snapshot));n.close();");
    let mut child=Command::new("bun").args(["-e",&import_script]).current_dir(root).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    println!("[DEBUG] LAS full TypeScript-to-SQLite-to-native-to-SQLite-to-TypeScript exact typed interoperability verified across header, histogram, duplicate VLRs, points and IEEE optional components");
}


#[test]
fn sqlite_snapshot_large_semantic_loops_cancel_before_completion(){
    let mut snapshot=snapshot();snapshot.vlrs[0].data=vec![17;1200];snapshot.points=vec![snapshot.points[0].clone();600];
    let mut reached=false;let result=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |p|{if p.completed>=256{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(result.is_err());assert!(reached);
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let mut reached=false;let result=LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |p|{if p.completed>=256{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(result.is_err());assert!(reached);
}

#[test]
fn sqlite_snapshot_erased_binary_and_text_capabilities_preserve_every_native_field(){
    let snapshot=snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let codec=<LasSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.las".into(),standard:"1.0".into(),subset:"*".into()};
    for encoding in [semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text]{
        let payload=(codec.import)(&snapshot.schema,&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
        let restored=(codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
        assert_eq!(restored,database,"actual erased native snapshot bridge must preserve typed fields independently of LAS wire encoding");
    }
    println!("[DEBUG] LAS erased native snapshot capability preserves all typed fields in binary and text encoding");
}

#[test]
fn sqlite_snapshot_las_native_encoding_preflight_admits_owned_model_and_refuses_budget_or_cancellation(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let snapshot=LasSnapshot::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=1;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}

#[test]
fn sqlite_snapshot_las_native_encoding_preflight_bounds_escaped_text_and_cancels_borrowed_members(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️encoding.json")).unwrap();
 let mut snapshot=LasSnapshot::default();snapshot.header.generating_software="\n\\\"".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize);snapshot.points=vec![LasPoint::default();cases["workItems"].as_u64().unwrap() as usize];
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=cases["smallBudgetBytes"].as_u64().unwrap() as usize;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cases["cancelAfterWork"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached,"member admission walk must checkpoint before native ownership");
 }
}

#[path="🚦️cohort/🦀️.rs"]
mod cohort;

fn norm_complete_source(case:&serde_json::Value)->LasSnapshot{let mut source=snapshot();if case["id"]=="metadataRetainingEmpty"{source.vlrs.clear();source.points.clear();}source}
fn norm_independent_complete_extent(source:&LasSnapshot)->serde_json::Value{
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=concat!(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts"),"\nawait Bun.write(Bun.stdout,JSON.stringify(independentSqliteExtent(new Uint8Array(await Bun.stdin.arrayBuffer()))));");let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn sqlite_snapshot_las_complete_independent_native_semantic_limits(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let contract:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎛️semantic.json")).unwrap();
 for case in contract["cases"].as_array().unwrap(){let source=norm_complete_source(case);let extent=norm_independent_complete_extent(&source);assert_eq!(extent["rows"],case["rows"]);assert_eq!(extent["valueBytes"],case["valueBytes"]);assert_eq!(extent["schemaBytes"],contract["schemaBytes"]);assert_eq!(extent["tableWidths"],contract["tableWidths"]);let limits=SqliteDatabaseLimits{max_rows:case["rows"].as_u64().unwrap()as usize,max_value_bytes:case["valueBytes"].as_u64().unwrap()as usize,max_schema_bytes:contract["schemaBytes"].as_u64().unwrap()as usize,max_tables:8,max_columns:50,..SqliteDatabaseLimits::default()};let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),database);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(LasSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),database);
   for short in[SqliteDatabaseLimits{max_rows:limits.max_rows-1,..limits},SqliteDatabaseLimits{max_value_bytes:limits.max_value_bytes-1,..limits},SqliteDatabaseLimits{max_schema_bytes:limits.max_schema_bytes-1,..limits},SqliteDatabaseLimits{max_tables:7,..limits},SqliteDatabaseLimits{max_columns:49,..limits}]{assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"relational copied limits {short:?}");assert!(LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"reconstruct copied limits {short:?}");assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"preflight copied limits {encoding:?} {short:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"encoder copied limits {encoding:?} {short:?}");assert!(LasSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"decoder copied limits {encoding:?} {short:?}");}
  }
 }eprintln!("[DEBUG] LAS independently measured complete copied native semantic limits");
}
#[test]
fn sqlite_snapshot_las_complete_independent_copied_columns_admission(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::*};let source=snapshot();let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let limits=SqliteDatabaseLimits{max_columns:49,..SqliteDatabaseLimits::default()};assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(LasSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"preflight copied columns {encoding:?}");assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"encoder copied columns {encoding:?}");assert!(LasSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"decoder copied columns {encoding:?}");}
}
