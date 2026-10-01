use super::*;


use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> (serde_json::Value, StlSnapshot) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let snapshot = StlSnapshot { schema: fixture["schema"].as_str().unwrap().into(), solid_name: fixture["solidName"].as_str().unwrap().into(), triangles: fixture["triangles"].as_array().unwrap().iter().map(|triangle| StlTriangle { normal: std::array::from_fn(|axis| triangle["normal"][axis].as_f64().unwrap()), vertices: std::array::from_fn(|vertex| std::array::from_fn(|axis| triangle["vertices"][vertex][axis].as_f64().unwrap())) }).collect() };
    (fixture, snapshot)
}

#[test]
fn sqlite_snapshot_stl_facets_own_exactly_three_vertices_and_preserve_normals() {
    let (fixture, snapshot) = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("stl_facet").unwrap().rows.len(), 2);
    assert_eq!(database.table("stl_vertex").unwrap().rows.len(), 6);
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = StlSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&restored))).unwrap();
    assert_eq!(oracle, fixture);
    for alteration in 0..3 {
        let mut broken = database.clone();
        match alteration {
            0 => { broken.table_mut("stl_vertex").unwrap().rows.pop(); },
            1 => broken.table_mut("stl_vertex").unwrap().rows[0].values[1] = SqliteValue::Integer(999),
            _ => broken.table_mut("stl_facet").unwrap().rows[1].values[2] = SqliteValue::Integer(0),
        }
        assert!(StlSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(StlSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_stl_sql_edits_reconstruct_native_facet_coordinates() {
    use std::{io::Write, process::{Command, Stdio}};
    let (_, mut snapshot) = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite'; const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())); if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT s.name,f.ordinal,v.ordinal AS vertex,v.x FROM stl_solid s JOIN stl_facet f ON f.solid_id=s.id JOIN stl_vertex v ON v.facet_id=f.id ORDER BY f.ordinal,v.ordinal').all();if(rows.length!==6||rows[1].x!==3.25)throw Error('facet query');db.query('UPDATE stl_vertex SET x=7.5,x_ieee754_bits=4620130267728707584 WHERE facet_id=1 AND ordinal=1').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = StlSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    snapshot.triangles[0].vertices[1][0] = 7.5;
    assert_eq!(restored, snapshot);
    let native = <StlSnapshot as store::ArtifactPack>::decode_pack(&<StlSnapshot as store::ArtifactPack>::encode_pack(&restored)).unwrap();
    assert_eq!(native.triangles, restored.triangles);
}
#[test]
fn sqlite_snapshot_ieee754_native_domain_through_independent_sqlite() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
    for hex in fixture["binary64Bits"].as_array().unwrap(){
        let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);
        let snapshot=StlSnapshot{schema:"noncanonical".into(),solid_name:String::new(),triangles:vec![StlTriangle{normal:[value;3],vertices:[[value;3];3]}]};
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT CAST(normal_x_ieee754_bits AS TEXT) AS bits FROM stl_facet').get().bits!=='{}')throw Error('IEEE bits');await Bun.write(Bun.stdout,d.serialize());d.close();",bits as i64);
        let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let restored=StlSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        assert_eq!((restored.triangles[0].normal[0]).to_bits(),bits);

        let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));d.run(\"UPDATE stl_facet SET normal_x=NULL,normal_x_ieee754_bits=0,normal_x_numeric_class='nan'\");if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('malformed oracle');await Bun.write(Bun.stdout,d.serialize());d.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let malformed=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        assert!(StlSnapshot::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap_err().contains("IEEE"));
    }
    println!("[DEBUG] geometry full native binary64 domain survives independent SQLite");
}
#[test]
fn sqlite_snapshot_exact_owned_coordinates_and_document_identity() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
    let snapshot=StlSnapshot::default();
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let dialect=|value:&serde_json::Value|semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()};
    let accepted=dialect(&fixture["sqliteDialect"]);
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_ok());
    for value in fixture["invalidSqliteDialects"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(value),&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
    let mut wrong=database.clone();wrong.table_mut("stl_solid").unwrap().rows[0].values[1]=SqliteValue::Text("different document".into());
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&wrong,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&database,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
    assert!(<StlSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().is_some());
    println!("[DEBUG] geometry exact owned dialect, document identity and cancellation laws");
}

#[test]
fn sqlite_snapshot_stl_native_encoding_preflight_admits_owned_model_and_refuses_budget_or_cancellation(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let snapshot=StlSnapshot::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=1;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}

#[test]
fn sqlite_snapshot_stl_native_encoding_preflight_bounds_escaped_text_and_cancels_borrowed_members(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️encoding.json")).unwrap();
 let mut snapshot=StlSnapshot::default();snapshot.solid_name="\n\\\"".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize);snapshot.triangles=vec![StlTriangle::default();cases["workItems"].as_u64().unwrap() as usize];
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=cases["smallBudgetBytes"].as_u64().unwrap() as usize;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cases["cancelAfterWork"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached,"member admission walk must checkpoint before native ownership");
 }
}

#[test]
fn sqlite_snapshot_stl_erased_binary_and_text_keep_owned_schema_exact_ieee_and_intermediate_state(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
 let coordinate=&cases["sqliteDialect"];let dialect=store::io_schema::ArtifactDialect{artifact_kind:coordinate["artifactKind"].as_str().unwrap().into(),standard:coordinate["standard"].as_str().unwrap().into(),subset:coordinate["subset"].as_str().unwrap().into()};let codec=<StlSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
 for(index,word)in cases["binary64Bits"].as_array().unwrap().iter().enumerate(){let word=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();let(_,mut snapshot)=fixture();snapshot.schema="owned STL state".into();snapshot.triangles[0].normal[0]=f64::from_bits(word);snapshot.triangles[0].vertices[2][2]=f64::from_bits(word);
  let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=(codec.import)(&snapshot.schema,&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=(codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(restored,database,"native erased snapshot must retain every owned field and IEEE word");}
 }
}
