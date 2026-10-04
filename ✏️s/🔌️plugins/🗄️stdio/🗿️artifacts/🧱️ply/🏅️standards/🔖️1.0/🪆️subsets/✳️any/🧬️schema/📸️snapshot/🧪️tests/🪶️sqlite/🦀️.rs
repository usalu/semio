use super::*;

#[test]
fn sqlite_snapshot_ply_declaration_count_patch_and_set_snapshot_retain_unsigned_metadata(){
    use store::{MutationDiff,DiffCodec};
    use dsl::DiffAlgebra;
    let base=fixture();let mut target=base.clone();target.elements[0].count=u64::MAX;
    let delta=crate::schema::diff::PlyDiff::between(&base,&target);
    assert_eq!(delta.apply(&base).unwrap(),target);
    for roundtrip in [crate::schema::diff::PlyDiff::parse_diff(&delta.print_diff()).unwrap(),crate::schema::diff::PlyDiff::decode_diff(&delta.encode_diff().unwrap()).unwrap()]{assert_eq!(roundtrip.apply(&base).unwrap(),target);}
}


use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> PlySnapshot { semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_ply_actual_declaration_typed_and_erased_io_mount(){
 use store::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot,io_route,io_run_with_snapshot_control}};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("PLY owned SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.ply".into(),standard:"1.0".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);let mut snapshot=fixture();snapshot.schema="actual registered PLY intermediate".into();snapshot.format=PlyFormat::BinaryBigEndian;snapshot.elements[0].count=u64::MAX;snapshot.elements[0].rows[0].values=vec![PlyValue::List(vec![PlyValue::Double(f64::from_bits(0xfff0000000001234)),PlyValue::Float(f32::from_bits(0xff800123))])];let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 let export=io_route(&dialect,&sqlite,1).await.unwrap().value;let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
 for encoding in[store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text]{
  let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,limits,&mut |_|true).await.unwrap().value;let restored=io_import_sqlite_snapshot::<PlySnapshot>(&dialect,&bytes,limits,&mut |_|true).await.unwrap().value;assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  let payload=match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let file=io_run_with_snapshot_control(&export,payload,limits,&mut |_|true).await.unwrap().value;let restored=io_run_with_snapshot_control(&import,file,limits,&mut |_|true).await.unwrap().value;let restored=PlySnapshot::decode_sqlite_snapshot_native(&restored,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);super::native_pack::retire(restored);
 }
}

#[test]
fn sqlite_snapshot_ply_typed_properties_scalar_cells_and_list_items_roundtrip() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("ply_property").unwrap().rows.len(), 10);
    assert_eq!(database.table("ply_cell").unwrap().rows.len(), 19);
    assert_eq!(database.table("ply_list_item").unwrap().rows.len(), 3);
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = PlySnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&restored))).unwrap();
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    assert_eq!(oracle, expected);
    assert_eq!(<PlySnapshot as store::ArtifactPack>::encode_pack(&restored), <PlySnapshot as store::ArtifactPack>::encode_pack(&snapshot));
    for alteration in 0..5 {
        let mut broken = database.clone();
        match alteration {
            0 => broken.table_mut("ply_cell").unwrap().rows[0].values[3] = SqliteValue::Integer(999),
            1 => broken.table_mut("ply_list_item").unwrap().rows[0].values[1] = SqliteValue::Integer(999),
            2 => broken.table_mut("ply_value").unwrap().rows[3].values[2] = SqliteValue::Integer(128),
            3 => broken.table_mut("ply_element").unwrap().rows[0].values[4] = SqliteValue::Integer(4294967296),
            _ => broken.table_mut("ply_list_item").unwrap().rows[1].values[2] = SqliteValue::Integer(0),
        }
        assert!(PlySnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(PlySnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_ply_independent_sql_property_queries_and_edits_restore_scalar_types() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT e.name,r.ordinal,p.name AS property,v.kind,v.integer_value,v.real_value FROM ply_element e JOIN ply_row r ON r.element_id=e.id JOIN ply_cell c ON c.row_id=r.id JOIN ply_property p ON p.element_id=e.id AND p.ordinal=c.ordinal JOIN ply_value v ON v.id=c.value_id ORDER BY e.ordinal,r.ordinal,p.ordinal').all();if(rows.length!==19||rows[8].integer_value!==4294967295||rows[0].real_value!==3.25)throw Error('typed query');db.query('UPDATE ply_value SET real_value=8.5,real_value_ieee754_bits=4620974692658839552 WHERE id=(SELECT value_id FROM ply_cell WHERE row_id=1 AND ordinal=0)').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = PlySnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    snapshot.elements[0].rows[0].values[0] = PlyValue::Double(8.5);
    assert_eq!(restored, snapshot);
}
#[test]
fn sqlite_snapshot_ieee754_native_domain_through_independent_sqlite() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
    for hex in fixture["binary64Bits"].as_array().unwrap(){
        let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);
        let snapshot=PlySnapshot{schema:"noncanonical".into(),format:PlyFormat::Ascii,comments:vec![],elements:vec![PlyElement{name:"vertex".into(),count:1,properties:vec![PlyProperty::Scalar{name:"x".into(),kind:PlyScalarType::Double}],rows:vec![PlyRow{values:vec![PlyValue::Double(value)]}]}]};
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT CAST(real_value_ieee754_bits AS TEXT) AS bits FROM ply_value').get().bits!=='{}')throw Error('IEEE bits');await Bun.write(Bun.stdout,d.serialize());d.close();",bits as i64);
        let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let restored=PlySnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        assert_eq!((match &restored.elements[0].rows[0].values[0]{PlyValue::Double(value)=>*value,_=>panic!()}).to_bits(),bits);

        let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));d.run(\"UPDATE ply_value SET real_value=NULL,real_value_ieee754_bits=0,real_value_numeric_class='nan'\");if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('malformed oracle');await Bun.write(Bun.stdout,d.serialize());d.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let malformed=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        assert!(PlySnapshot::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap_err().message.contains("IEEE"));
    }
    println!("[DEBUG] geometry full native binary64 domain survives independent SQLite");
}
#[test]
fn sqlite_snapshot_exact_owned_coordinates_and_document_identity() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
    let snapshot=PlySnapshot::default();
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let dialect=|value:&serde_json::Value|semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()};
    let accepted=dialect(&fixture["sqliteDialect"]);
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_ok());
    for value in fixture["invalidSqliteDialects"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(value),&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
    let mut wrong=database.clone();wrong.table_mut("ply_document").unwrap().rows[0].values[1]=SqliteValue::Text("different document".into());
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&wrong,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.validate_sqlite_snapshot_subset(&accepted,&database,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
    assert!(<PlySnapshot as store::ArtifactPack>::sqlite_snapshot_codec().is_some());
    println!("[DEBUG] geometry exact owned dialect, document identity and cancellation laws");
}
#[test]
fn sqlite_snapshot_ply_binary32_payloads_and_lists_survive_independent_sqlite() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
    for hex in fixture["binary32Bits"].as_array().unwrap(){
        let bits=u32::from_str_radix(hex.as_str().unwrap(),16).unwrap();let value=f32::from_bits(bits);
        let snapshot=PlySnapshot{schema:"exact".into(),format:PlyFormat::BinaryBigEndian,comments:vec![],elements:vec![PlyElement{name:"vertex".into(),count:1,properties:vec![PlyProperty::Scalar{name:"x".into(),kind:PlyScalarType::Float},PlyProperty::List{name:"samples".into(),count_kind:PlyScalarType::UChar,value_kind:PlyScalarType::Float}],rows:vec![PlyRow{values:vec![PlyValue::Float(value),PlyValue::List(vec![PlyValue::Float(value)])]}]}]};
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');for(const id of [1,3])if(d.query('SELECT real_value_ieee754_bits AS bits FROM ply_value WHERE id='+id).get().bits!=={bits})throw Error('binary32 identity');await Bun.write(Bun.stdout,d.serialize());d.close();");
        let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let restored=PlySnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let PlyValue::Float(value)=&restored.elements[0].rows[0].values[0]else{panic!()};assert_eq!(value.to_bits(),bits);
        let PlyValue::List(values)=&restored.elements[0].rows[0].values[1]else{panic!()};let PlyValue::Float(value)=&values[0]else{panic!()};assert_eq!(value.to_bits(),bits);
    }
    println!("[DEBUG] PLY exact native binary32 scalar/list NaN payload and special value laws");
}

#[test]
fn sqlite_snapshot_ply_native_encoding_preflight_admits_owned_model_and_refuses_budget_or_cancellation(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let snapshot=PlySnapshot::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=1;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}

#[test]
fn sqlite_snapshot_ply_native_encoding_preflight_bounds_escaped_text_and_cancels_borrowed_members(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️encoding.json")).unwrap();
 let mut snapshot=PlySnapshot::default();snapshot.comments=vec!["\n\\\"".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize)];snapshot.elements=vec![PlyElement{name:"x".into(),count:cases["workItems"].as_u64().unwrap(),properties:vec![PlyProperty::Scalar{name:"x".into(),kind:PlyScalarType::Double}],rows:vec![PlyRow{values:vec![PlyValue::Double(f64::from_bits(1))]};cases["workItems"].as_u64().unwrap() as usize]}];
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=cases["smallBudgetBytes"].as_u64().unwrap() as usize;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cases["cancelAfterWork"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached,"member admission walk must checkpoint before native ownership");
 }
}

#[test]
fn sqlite_snapshot_ply_erased_binary_and_text_keep_owned_schema_exact_ieee_and_intermediate_state(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json")).unwrap();
 let coordinate=&cases["sqliteDialect"];let dialect=store::io_schema::ArtifactDialect{artifact_kind:coordinate["artifactKind"].as_str().unwrap().into(),standard:coordinate["standard"].as_str().unwrap().into(),subset:coordinate["subset"].as_str().unwrap().into()};let codec=<PlySnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
 for(index,word)in cases["binary64Bits"].as_array().unwrap().iter().enumerate(){let word=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();let mut snapshot=fixture();snapshot.schema="owned PLY state".into();snapshot.elements[0].rows[0].values[0]=PlyValue::Double(f64::from_bits(word));snapshot.elements[0].rows[0].values[1]=PlyValue::Float(f32::from_bits(u32::from_str_radix(cases["binary32Bits"][index].as_str().unwrap(),16).unwrap()));
  let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=(codec.import)(&snapshot.schema,&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=(codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(restored,database,"native erased snapshot must retain every owned field and IEEE word");}
 }
}

#[test]
fn sqlite_snapshot_ply_declaration_counts_and_count_scalar_kinds_are_independent_from_rows(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📋️declaration-state.json")).unwrap();let kinds=[PlyScalarType::Char,PlyScalarType::UChar,PlyScalarType::Short,PlyScalarType::UShort,PlyScalarType::Int,PlyScalarType::UInt,PlyScalarType::Float,PlyScalarType::Double];
 for count in cases["declaredCounts"].as_array().unwrap(){let count=count.as_str().unwrap().parse::<u64>().unwrap();for count_kind in kinds{let snapshot=PlySnapshot{schema:"owned state".into(),format:PlyFormat::Ascii,comments:vec![],elements:vec![PlyElement{name:"face".into(),count,properties:vec![PlyProperty::List{name:"indices".into(),count_kind,value_kind:PlyScalarType::Int}],rows:vec![PlyRow{values:vec![PlyValue::List(vec![PlyValue::Int(1);cases["listLength"].as_u64().unwrap() as usize])]}]}]};let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let row=&database.table("ply_element").unwrap().rows[0];assert_eq!(row.integer(4).unwrap(),(count>>32) as i64);assert_eq!(row.integer(5).unwrap(),(count&0xffffffff) as i64);let restored=PlySnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored,snapshot);}}
}

#[test]
fn sqlite_snapshot_ply_independent_vectors_and_recursive_heterogeneous_values(){
 use std::{io::Write,process::{Command,Stdio}};
 let corpus=include_str!("../../🧫️fixtures/🪶️sqlite/🧩️independent-values.json").replace("\"18446744073709551615\"","18446744073709551615").replace("\"0\"","0");
 let snapshot:PlySnapshot=semio_framework_pack_json::from_json_str(&corpus, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert!(snapshot.elements[0].rows[0].values.is_empty());assert_eq!(snapshot.elements[1].properties.len(),0);
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(*) AS count FROM ply_value WHERE kind=\"list\"').get().count!==3)throw Error('recursive values');await Bun.write(Bun.stdout,d.serialize());d.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let restored=PlySnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored,snapshot);
}

#[test]
fn sqlite_snapshot_ply_whole_native_controls_deep_values_and_exact_payload_retirement(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧱️topology-laws.json")).unwrap();let depth=cases["depth"].as_u64().unwrap()as usize;let bits=u64::from_str_radix(cases["binary64Bits"].as_str().unwrap(),16).unwrap();let mut value=PlyValue::Double(f64::from_bits(bits));for _ in 0..depth{value=PlyValue::List(vec![value]);}
 let snapshot=PlySnapshot{schema:"deep owned state 世界".repeat(10000),format:PlyFormat::Ascii,comments:vec![],elements:vec![PlyElement{name:"independent".into(),count:u64::MAX,properties:vec![],rows:vec![PlyRow{values:vec![value]}]}]};
 for payload in[store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))]{
  let restored=PlySnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut value=&restored.elements[0].rows[0].values[0];let mut count=0;while let PlyValue::List(values)=value{count+=1;value=&values[0];}assert_eq!(count,depth);let PlyValue::Double(value)=value else{panic!()};assert_eq!(value.to_bits(),bits);super::native_pack::retire(restored);
  for limits in[SqliteDatabaseLimits{max_rows:cases["maxRows"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_value_bytes:cases["maxValueBytes"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()}]{assert!(PlySnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}
  let mut interior=false;assert!(PlySnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=cases["cancelAfter"].as_u64().unwrap()as usize&&event.completed<event.total{interior=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(interior);
 }
 super::native_pack::retire(snapshot);
}

#[test]
fn sqlite_snapshot_ply_unreachable_owned_cycle_rejects_independent_valid_sqlite(){
 use std::{io::Write,process::{Command,Stdio}};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧱️topology-laws.json")).unwrap();let a=corpus["unreachableCycle"][0].as_u64().unwrap();let b=corpus["unreachableCycle"][1].as_u64().unwrap();let database=PlySnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));d.run(\"INSERT INTO ply_value(id,kind) VALUES ({a},'list'),({b},'list')\");d.run('INSERT INTO ply_list_item(id,list_id,ordinal,value_id) VALUES (1,{a},0,{b}),(2,{b},0,{a})');if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();");let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert!(PlySnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap_err().message.contains("unreachable"));
}

#[test]
fn sqlite_snapshot_ply_controlled_native_output_retains_deep_values_and_cancels_inside_unicode(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧱️topology-laws.json")).unwrap();let depth=f["depth"].as_u64().unwrap()as usize;let bits=u64::from_str_radix(f["binary64Bits"].as_str().unwrap(),16).unwrap();let mut value=PlyValue::Double(f64::from_bits(bits));for _ in 0..depth{value=PlyValue::List(vec![value]);}
 let mut snapshot=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(PlySnapshot{schema:"世界 🪐".repeat(30000),format:PlyFormat::BinaryBigEndian,comments:vec!["independent metadata".into()],elements:vec![PlyElement{name:"independent".into(),count:u64::MAX,properties:vec![PlyProperty::List{name:"unrelated declaration".into(),count_kind:PlyScalarType::Double,value_kind:PlyScalarType::Float}],rows:vec![PlyRow{values:vec![value,PlyValue::Float(f32::from_bits(0xff800123))]}]}]},super::native_pack::retire);let schema_length=snapshot.as_mut().schema.len();let limits=SqliteDatabaseLimits::default();let expected=snapshot.as_mut().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=snapshot.as_mut().encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).expect("PLY actual controlled native output owner");let mut restored=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(PlySnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),super::native_pack::retire);assert_eq!(restored.as_mut().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);let mut interior=false;assert!(snapshot.as_mut().encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==schema_length&&event.completed>=65536&&event.completed<event.total{interior=true;false}else{true}},limits)).is_err());assert!(interior);for limited in[SqliteDatabaseLimits{max_rows:30,..limits},SqliteDatabaseLimits{max_value_bytes:128,..limits}]{assert!(snapshot.as_mut().encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err());}}
}

#[test]
fn sqlite_snapshot_ply_controlled_native_retains_refusal_categories(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧱️topology-laws.json")).unwrap();
 let cases=&fixture["controlledRefusals"];let source=PlySnapshot::default();
 let canceled=super::native_pack::record_controlled(&source,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|false),usize::MAX).err().expect("canceled projection");
 assert_eq!(canceled.kind.as_str(),cases["cancellation"].as_str().unwrap());
 let limited=super::native_pack::record_controlled(&source,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|true),0).err().expect("row admission");
 assert_eq!(limited.kind.as_str(),cases["rowAdmission"].as_str().unwrap());
 let mut record=super::native_pack::record_controlled(&source,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|true),usize::MAX).unwrap();record.fields.remove(&3);
 let malformed=super::native_pack::reconstruct_record_controlled(&record,&mut semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut |_|true),usize::MAX).err().expect("malformed record");
 assert_eq!(malformed.kind.as_str(),cases["malformedRecord"].as_str().unwrap());
 let source_case=&fixture["controlledTextSource"];let expected_span=semio_framework_diagnostic::TextSpan{line:source_case["span"]["line"].as_u64().unwrap()as u32,column:source_case["span"]["column"].as_u64().unwrap()as u32,length:source_case["span"]["length"].as_u64().unwrap()as u32};
 let spec=super::native_pack::spec();
 let text_error=semio_framework_dsl_record::parse_exact_controlled(source_case["source"].as_str().unwrap(),&spec,&semio_framework_dsl_record::ParseOptions::default(),&mut semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut |_|true)).err().expect("actual malformed Text source");
 assert_eq!(text_error.kind.as_str(),cases["malformedRecord"].as_str().unwrap());
 assert_eq!(text_error.span,expected_span);
 let io_error=store::io_schema::IoError::from_text_error_controlled(text_error,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|true)).unwrap();
 assert_eq!(io_error.cause.kind.as_str(),cases["malformedRecord"].as_str().unwrap());
 assert_eq!(io_error.diagnostics.len(),1);
 assert_eq!(io_error.diagnostics[0].span,expected_span);
 eprintln!("[DEBUG] PLY controlled refusal categories cancellation={} admission={} malformed={}",canceled.kind.as_str(),limited.kind.as_str(),malformed.kind.as_str());
}
