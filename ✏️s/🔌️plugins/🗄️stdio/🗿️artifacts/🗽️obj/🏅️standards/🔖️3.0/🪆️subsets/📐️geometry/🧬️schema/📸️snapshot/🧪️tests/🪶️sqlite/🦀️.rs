use super::*;


use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> ObjSnapshot { semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }

#[test]
fn sqlite_snapshot_obj_controlled_output_retains_exact_ieee_and_source_words_with_interior_cancellation(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();let limits=SqliteDatabaseLimits::default();for word in cases["binary64Bits"].as_array().unwrap(){let mut snapshot=fixture();snapshot.schema="世界 🪐".repeat(30000);snapshot.vertices[0].x=f64::from_bits(u64::from_str_radix(word.as_str().unwrap(),16).unwrap());snapshot.unknown_statements[0].line_index=u64::MAX;let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=ObjSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);let mut interior=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==snapshot.schema.len()&&event.completed>=65536&&event.completed<event.total{interior=true;false}else{true}},limits)).is_err());assert!(interior);for limited in[SqliteDatabaseLimits{max_rows:1,..limits},SqliteDatabaseLimits{max_value_bytes:128,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());}}}
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_obj_actual_declaration_typed_and_erased_io_mount(){
 use store::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot,io_route,io_run_with_snapshot_control}};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("OBJ owned SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.obj".into(),standard:"3.0".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);
 let mut snapshot=fixture();snapshot.schema="actual registered OBJ intermediate".into();snapshot.vertices[0].x=f64::from_bits(0xfff0000000001234);snapshot.faces[0].vertices.clear();snapshot.groups[0].faces=vec![u64::MAX];snapshot.objects[0].faces=vec![9007199254740993];snapshot.usemtl[0].face_index_from=u64::MAX;snapshot.unknown_statements[0].line_index=u64::MAX;
 let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 let export=io_route(&dialect,&sqlite,1).await.unwrap().value;let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
 for encoding in[store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text]{
  let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,limits,&mut |_|true).await.unwrap().value;let restored=io_import_sqlite_snapshot::<ObjSnapshot>(&dialect,&bytes,limits,&mut |_|true).await.unwrap().value;assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  let payload=match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let file=io_run_with_snapshot_control(&export,payload,limits,&mut |_|true).await.unwrap().value;let restored=io_run_with_snapshot_control(&import,file,limits,&mut |_|true).await.unwrap().value;let restored=ObjSnapshot::decode_sqlite_snapshot_native(&restored,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
 }
}

#[test]
fn sqlite_snapshot_obj_unresolved_native_source_indices_are_complete_states(){
    let mut corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔗️source-indices.json")).unwrap();corpus.as_object_mut().unwrap().remove("unsigned64Indices");
    let snapshot:ObjSnapshot=semio_framework_pack_json::from_json_str(&corpus.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    use std::{process::{Command,Stdio},io::Write};
    let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    assert_eq!(ObjSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
}

#[test]
fn sqlite_snapshot_obj_unsigned_source_words_and_optional_resolved_relationships_are_independently_editable(){
 use std::{process::{Command,Stdio},io::Write};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔗️source-indices.json")).unwrap();
 let points:Vec<u64>=corpus["unsigned64Indices"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().parse().unwrap()).collect();
 let mut snapshot=fixture();snapshot.groups.truncate(1);snapshot.groups[0].faces=points.clone();snapshot.objects[0].faces=points.clone();snapshot.usemtl=points.iter().map(|&face_index_from|ObjUsemtlRange{face_index_from,material:"words".into()}).collect();snapshot.smoothing_groups=points.iter().map(|&face_index_from|ObjSmoothingRange{face_index_from,group:None}).collect();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));for(const [t,h,l]of[['obj_group_face','face_source_index_high','face_source_index_low'],['obj_object_face','face_source_index_high','face_source_index_low'],['obj_material_range','first_face_source_index_high','first_face_source_index_low'],['obj_smoothing_range','first_face_source_index_high','first_face_source_index_low']]){const r=d.query('SELECT '+h+' AS h,'+l+' AS l FROM '+t+' ORDER BY ordinal').all().slice(0,3);if(JSON.stringify(r)!==JSON.stringify([{h:0,l:0},{h:2097152,l:1},{h:4294967295,l:4294967295}]))throw Error('words');}d.run('UPDATE obj_group_face SET face_id=NULL');d.run('UPDATE obj_group_face SET face_source_index_high=4294967295,face_source_index_low=4294967294 WHERE group_id=1 AND ordinal=1');if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();snapshot.groups[0].faces[1]=u64::MAX-1;
 assert_eq!(ObjSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
}

#[test]
fn sqlite_snapshot_obj_signed_surrogate_renumbering_preserves_owned_identity_guard(){
 use std::{process::{Command,Stdio},io::Write};
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 let edits:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔗️surrogate-indices.json")).unwrap();
 let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));d.run('PRAGMA foreign_keys=OFF');for(const sql of {})d.run(sql);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();",edits["statements"]);
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 assert_eq!(ObjSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
 let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.stdio.obj".into(),standard:"3.0".into(),subset:"*".into()};
 snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
}

#[test]
fn sqlite_snapshot_obj_source_line_positions_preserve_unsigned_words() {
    let mut snapshot = fixture();
    snapshot.unknown_statements[0].line_index = u64::MAX;
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let row = &database.table("obj_unknown_statement").unwrap().rows[0];
    let position = snapshot.unknown_statements[0].line_index;
    assert_eq!(row.integer(3).unwrap(), (position >> 32) as i64);
    assert_eq!(row.integer(4).unwrap(), (position & u32::MAX as u64) as i64);
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(ObjSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let mut broken = database;
    broken.table_mut("obj_unknown_statement").unwrap().rows[0].values[4] = SqliteValue::Integer(u32::MAX as i64 + 1);
    assert!(ObjSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_obj_geometry_membership_ranges_and_unknown_statements_roundtrip() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("obj_face_vertex").unwrap().rows.len(), 7);
    assert_eq!(database.table("obj_face_boundary").unwrap().rows.len(), 3);
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = ObjSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&restored))).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    assert_eq!(oracle, expected);
    assert_eq!(<ObjSnapshot as store::ArtifactPack>::encode_pack(&restored), <ObjSnapshot as store::ArtifactPack>::encode_pack(&snapshot));
    for alteration in 0..4 {
        let mut broken = database.clone();
        match alteration {
            0 => broken.table_mut("obj_face_vertex").unwrap().rows[0].values[3] = SqliteValue::Integer(999),
            1 => broken.table_mut("obj_group_face").unwrap().rows[0].values[1] = SqliteValue::Integer(999),
            2 => broken.table_mut("obj_material_range").unwrap().rows[0].values[3] = SqliteValue::Integer(999),
            _ => broken.table_mut("obj_face_vertex").unwrap().rows[1].values[2] = SqliteValue::Integer(0),
        }
        assert!(ObjSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(ObjSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_obj_independent_sql_joins_and_edits_reconstruct_geometry() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT f.ordinal,v.x,c.texcoord_id FROM obj_face f JOIN obj_face_vertex c ON c.face_id=f.id JOIN obj_vertex v ON v.id=c.vertex_id ORDER BY f.ordinal,c.ordinal').all();if(rows.length!==7||rows[1].x!==3.25||rows[2].texcoord_id!==null)throw Error('query');db.query('UPDATE obj_vertex SET x=8.5,x_ieee754_bits=4620974692658839552 WHERE ordinal=1').run();db.query('UPDATE obj_group SET name=? WHERE ordinal=0').run('SQLite-Gruppe');await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = ObjSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    snapshot.vertices[1].x = 8.5;
    snapshot.groups[0].name = "SQLite-Gruppe".into();
    assert_eq!(restored, snapshot);
}
#[test]
fn sqlite_snapshot_ieee754_native_domain_through_independent_sqlite() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
    for hex in fixture["binary64Bits"].as_array().unwrap(){
        let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);
        let mut snapshot=ObjSnapshot::default();snapshot.vertices=vec![ObjVertex{x:value,y:value,z:value,w:Some(value)}];snapshot.texcoords=vec![ObjTexCoord{u:value,v:value,w:None}];snapshot.normals=vec![ObjNormal{x:value,y:value,z:value}];
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT CAST(x_ieee754_bits AS TEXT) AS bits FROM obj_vertex').get().bits!=='{}')throw Error('IEEE bits');await Bun.write(Bun.stdout,d.serialize());d.close();",bits as i64);
        let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let restored=ObjSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        assert_eq!((restored.vertices[0].x).to_bits(),bits);

        let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));d.run(\"UPDATE obj_vertex SET x=NULL,x_ieee754_bits=0,x_numeric_class='nan'\");if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('malformed oracle');await Bun.write(Bun.stdout,d.serialize());d.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let malformed=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        assert!(ObjSnapshot::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap_err().message.contains("IEEE"));
    }
    println!("[DEBUG] geometry full native binary64 domain survives independent SQLite");
}
#[test]
fn sqlite_snapshot_exact_owned_coordinates_and_document_identity() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
    let snapshot=ObjSnapshot::default();
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let dialect=|value:&serde_json::Value|semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()};
    let accepted=dialect(&fixture["sqliteDialect"]);
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_ok());
    for value in fixture["invalidSqliteDialects"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(value),&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
    let mut wrong=database.clone();wrong.table_mut("obj_document").unwrap().rows[0].values[1]=SqliteValue::Text("different document".into());
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&wrong,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&database,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
    assert!(<ObjSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().is_some());
    println!("[DEBUG] geometry exact owned dialect, document identity and cancellation laws");
}

#[test]
fn sqlite_snapshot_obj_native_encoding_preflight_admits_owned_model_and_refuses_budget_or_cancellation(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let snapshot=ObjSnapshot::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=1;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}

#[test]
fn sqlite_snapshot_obj_native_encoding_preflight_bounds_escaped_text_and_cancels_borrowed_members(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️encoding.json")).unwrap();
 let mut snapshot=ObjSnapshot::default();snapshot.unknown_statements=vec![ObjUnknownStatement{line_index:u64::MAX,raw:"\n\\\"".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize)}];snapshot.vertices=vec![ObjVertex::default();cases["workItems"].as_u64().unwrap() as usize];
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=cases["smallBudgetBytes"].as_u64().unwrap() as usize;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cases["cancelAfterWork"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached,"member admission walk must checkpoint before native ownership");
 }
}

#[test]
fn sqlite_snapshot_obj_erased_binary_and_text_keep_owned_schema_exact_ieee_and_intermediate_state(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
 let coordinate=&cases["sqliteDialect"];let dialect=store::io_schema::ArtifactDialect{artifact_kind:coordinate["artifactKind"].as_str().unwrap().into(),standard:coordinate["standard"].as_str().unwrap().into(),subset:coordinate["subset"].as_str().unwrap().into()};let codec=<ObjSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
 for(index,word)in cases["binary64Bits"].as_array().unwrap().iter().enumerate(){let word=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();let mut snapshot=fixture();snapshot.schema="owned OBJ state".into();snapshot.vertices[0].x=f64::from_bits(word);snapshot.vertices[0].w=Some(f64::from_bits(word));snapshot.unknown_statements[0].line_index=u64::MAX;
  let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=(codec.import)(&snapshot.schema,&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=(codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(restored,database,"native erased snapshot must retain every owned field and IEEE word");}
 }
}

#[test]
fn sqlite_snapshot_obj_controlled_native_admits_unsigned_source_indices_and_bounds_rows(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let mut snapshot=fixture();snapshot.groups[0].faces=vec![0,9007199254740993,u64::MAX];snapshot.objects[0].faces=vec![u64::MAX];snapshot.usemtl[0].face_index_from=u64::MAX;snapshot.smoothing_groups[0].face_index_from=9007199254740993;snapshot.faces[0].vertices.clear();snapshot.vertices[0].x=f64::from_bits(0xfff0000000001234);
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(<ObjSnapshot as store::ArtifactPack>::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(<ObjSnapshot as store::ArtifactDsl>::print_dsl(&snapshot))};
  let restored=ObjSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),database);
  let mut limits=SqliteDatabaseLimits::default();limits.max_rows=1;assert!(ObjSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut cancelled=false;assert!(ObjSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>0{cancelled=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(cancelled);
 }
}
