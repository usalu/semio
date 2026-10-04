use super::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::*};
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_ifc4_actual_declaration_preserves_every_ieee_word_in_owned_encodings(){
    use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("IFC SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let snapshot=fixture();let dialect=ArtifactDialect{artifact_kind:"s.stdio.ifc".into(),standard:"4".into(),subset:"*".into()};
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let exported=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,SqliteDatabaseLimits::default(),&mut |p|{phases.push(p.phase);true}).await.unwrap();let imported=io_import_sqlite_snapshot::<IfcSnapshot>(&dialect,&exported.value,SqliteDatabaseLimits::default(),&mut |p|{phases.push(p.phase);true}).await.unwrap();assert_eq!(project(&imported.value),project(&snapshot));assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));}
}
#[test]
fn sqlite_snapshot_ifc4_native_preflight_and_erased_binary_text(){let snapshot=fixture();snapshot.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).expect("owned Native preflight retains every binary64 word");let mut finite=IfcSnapshot::default();finite.entities=vec![IfcEntity{id:1,name:"PRIMARY".into(),args:vec![IfcValue::Integer(i64::MIN),IfcValue::Real(-0.0),IfcValue::Reference(u64::MAX)],complex:vec![]},IfcEntity{id:u64::MAX,name:"TARGET".into(),args:vec![],complex:vec![]}];let provider=<IfcSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.ifc".into(),standard:"4".into(),subset:"*".into()};for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<IfcSnapshot as store::ArtifactPack>::encode_pack(&finite)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(<IfcSnapshot as store::ArtifactDsl>::print_dsl(&finite))};let database=(provider.export)(&finite.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=(provider.import)(&finite.schema,&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(restored,payload);}}
#[test]
fn sqlite_snapshot_ifc4_rejects_corrupt_ownership_reference_words_ieee_and_controls(){let original=project(&fixture());for mutation in 0..6{let mut db=original.clone();match mutation{0=>db.table_mut("ifc_argument").unwrap().rows[0].values[4]=SqliteValue::Integer(999),1=>db.table_mut("ifc_value").unwrap().rows.iter_mut().find(|row|row.text(1).unwrap()=="reference").unwrap().values[6]=SqliteValue::Text("00".into()),2=>db.table_mut("ifc_entity").unwrap().rows[1].values[3]=SqliteValue::Text("0".into()),3=>db.table_mut("ifc_argument").unwrap().rows[0].values[2]=SqliteValue::Integer(1),4=>db.table_mut("ifc_file_name_argument").unwrap().rows[0].values[2]=SqliteValue::Integer(8),_=>db.table_mut("ifc_value").unwrap().rows.iter_mut().find(|row|row.text(1).unwrap()=="real").unwrap().values[10]=SqliteValue::Text("finite".into())}assert!(restore(&db).is_err());}let limits=SqliteDatabaseLimits{max_value_bytes:1,..SqliteDatabaseLimits::default()};assert!(fixture().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(IfcSnapshot::from_sqlite_database(&original,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(fixture().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());}
fn fixture()->IfcSnapshot{let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let mut args=vec![IfcValue::Unset,IfcValue::Derived,IfcValue::Integer(i64::MIN),IfcValue::Integer(i64::MAX)];for word in f["binary64Words"].as_array().unwrap(){args.push(IfcValue::Real(f64::from_bits(u64::from_str_radix(word.as_str().unwrap(),16).unwrap())));}args.extend([IfcValue::String(f["text"].as_str().unwrap().into()),IfcValue::Enum("UNKNOWN".into()),IfcValue::Reference(u64::MAX),IfcValue::Aggregate(vec![]),IfcValue::TypedValue{name:"IFCLENGTHMEASURE".into(),items:vec![IfcValue::Integer(1),IfcValue::Aggregate(vec![IfcValue::Derived])] }]);IfcSnapshot{schema:f["schema"].as_str().unwrap().into(),header:IfcHeader{file_description:vec![IfcValue::Aggregate(vec![IfcValue::String("description".into())]),IfcValue::TypedValue{name:"HEADER_CUSTOM".into(),items:vec![]}],file_name:vec![IfcValue::Reference(0),IfcValue::Real(f64::from_bits(0x7ff0000000000001))],file_schema:vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into()),IfcValue::String("IFC4".into())])]},entities:vec![IfcEntity{id:0,name:"IFCWALL".into(),args,complex:vec![IfcComplexType{name:"IFCROOT".into(),args:vec![IfcValue::Reference(0)]}]},IfcEntity{id:u64::MAX,name:"TARGET".into(),args:vec![IfcValue::Reference(0)],complex:vec![]}]}}
fn project(snapshot:&IfcSnapshot)->SqliteDatabase{snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(database:&SqliteDatabase)->Result<IfcSnapshot,semio_framework_value::ValueError>{IfcSnapshot::from_sqlite_database(database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))}
#[test]
fn sqlite_snapshot_ifc4_every_header_value_variant_full_width_and_ieee_word(){let snapshot=fixture();let db=project(&snapshot);let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let restored=restore(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();assert_eq!(project(&restored),db);assert_eq!(project(&restore(&project(&IfcSnapshot::default())).unwrap()),project(&IfcSnapshot::default()));}
#[test]
fn sqlite_snapshot_ifc4_independent_sqlite_full_value_query_and_edit(){use std::io::Write;use std::process::{Command,Stdio};let bytes=export_sqlite_database(&project(&fixture()),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(db.query('SELECT kind FROM ifc_value GROUP BY kind').all().length!==9)throw Error('variants');const{importSqliteDatabase,exportSqliteDatabase}=await import(process.argv[1]);const{ifcSnapshotFromSqliteDatabase,ifcSnapshotToSqliteDatabase}=await import(process.argv[2]);const other=Database.deserialize(await exportSqliteDatabase(await ifcSnapshotToSqliteDatabase(await ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize())))));const canonical=value=>JSON.stringify(value,(_,v)=>typeof v==='bigint'?v.toString():v);for(const table of['ifc_document','ifc_header','ifc_file_description_argument','ifc_file_name_argument','ifc_file_schema_argument','ifc_entity','ifc_complex_type','ifc_argument','ifc_value','ifc_aggregate_element','ifc_typed_argument'])if(canonical(db.query('SELECT * FROM '+table+' ORDER BY id').safeIntegers(true).all())!==canonical(other.query('SELECT * FROM '+table+' ORDER BY id').safeIntegers(true).all()))throw Error('Rust/TypeScript IFC4 field mismatch '+table);other.close();db.query(\"UPDATE ifc_entity SET name='SQL_EDIT' WHERE instance_id='18446744073709551615'\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";let framework=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts").canonicalize().unwrap();let artifact=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🟦️.ts").canonicalize().unwrap();let mut child=Command::new("bun").args(["-e",script,framework.to_str().unwrap(),artifact.to_str().unwrap()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let restored=restore(&import_sqlite_database(&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();assert_eq!(restored.entities[1].name,"SQL_EDIT");}

fn cohort_ifc4(text:String)->IfcSnapshot{let mut snapshot=IfcSnapshot::default();snapshot.schema="literal external schema".into();snapshot.entities=vec![IfcEntity{id:1,name:"PRIMARY".into(),args:vec![IfcValue::String(text),IfcValue::Real(0.125),IfcValue::Integer(i64::MIN),IfcValue::Reference(u64::MAX)],complex:vec![]},IfcEntity{id:u64::MAX,name:"TARGET".into(),args:vec![],complex:vec![]}];snapshot}
fn cohort_ifc4_payload(snapshot:&IfcSnapshot,encoding:SnapshotEncoding)->store::os_io::IoPayload{match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<IfcSnapshot as store::ArtifactPack>::encode_pack(snapshot)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(<IfcSnapshot as store::ArtifactDsl>::print_dsl(snapshot))}}
#[test]
fn sqlite_snapshot_ifc4_cohort_controlled_input_preserves_actual_carrier_and_cancels_inside_copy(){
 let snapshot=cohort_ifc4("literal 文🌠".into());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=cohort_ifc4_payload(&snapshot,encoding);let expected=match &payload{store::os_io::IoPayload::Binary(bytes)=><IfcSnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(),store::os_io::IoPayload::Text(text)=><IfcSnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap()};
 let restored=<IfcSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(project(&restored),project(&expected));
 assert!(<IfcSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_allocation_bytes:1,..SqliteDatabaseLimits::default()})).is_err());
 let payload=cohort_ifc4_payload(&cohort_ifc4("文🌠".repeat(20000)),encoding);let mut reached=false;let mut callback=|p:SqliteSnapshotProgress|{if p.phase==SqliteSnapshotPhase::DecodeNative&&p.total>=65536&&p.completed>=65536&&p.completed<p.total{reached=true;false}else{true}};
 assert!(<IfcSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}
#[test]
fn sqlite_snapshot_ifc4_cohort_controlled_output_preserves_actual_carrier_and_cancels_inside_copy(){
 let snapshot=cohort_ifc4("literal 文🌠".into());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let expected=cohort_ifc4_payload(&snapshot,encoding);let actual=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(actual,expected);
 assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_allocation_bytes:1,..SqliteDatabaseLimits::default()})).is_err());
 let large=cohort_ifc4("文🌠".repeat(20000));let mut reached=false;let mut callback=|p:SqliteSnapshotProgress|{if p.phase==SqliteSnapshotPhase::EncodeNative&&p.total>=65536&&p.completed>=65536&&p.completed<p.total{reached=true;false}else{true}};assert!(large.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}

#[test]
fn sqlite_snapshot_ifc4_cohort_semantic_words_match_independent_sqlite_dataview() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../📇️registry/🧬️contract/📐️part21/🧫️fixtures/🚦️sqlite-cohort/🔣️.json")).unwrap();
    let words: Vec<String> = corpus["binary64Words"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().into()).collect();
    let bits: Vec<u64> = words.iter().map(|v|u64::from_str_radix(v,16).unwrap()).collect();
    let mut snapshot = cohort_ifc4(String::new());
    snapshot.schema = "literal owned schema".into();
    snapshot.entities[0].args = bits.iter().map(|word|IfcValue::Real(f64::from_bits(*word))).collect();
    let bytes = export_sqlite_database(&project(&snapshot), SqliteDatabaseLimits::default(), &mut |_|true).unwrap();
    let script = r#"import{Database}from'bun:sqlite';const expected=EXPECTED_WORDS;const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query("SELECT real_value,real_bits,real_class FROM ifc_value WHERE kind='real' ORDER BY id").safeIntegers(true).all();if(rows.length!==expected.length)throw Error('count');for(let i=0;i<rows.length;i++){const row=rows[i],bits=BigInt('0x'+expected[i]);if(BigInt.asUintN(64,row.real_bits)!==bits)throw Error('word');const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,bits,true);const number=view.getFloat64(0,true);const kind=Number.isNaN(number)?'nan':number===Infinity?'positiveInfinity':number===-Infinity?'negativeInfinity':'finite';if(row.real_class!==kind||(kind==='nan'?row.real_value!==null:row.real_value!==number))throw Error('query value/class');}await Bun.write(Bun.stdout,db.serialize());db.close();"#.replace("EXPECTED_WORDS", &serde_json::to_string(&words).unwrap());
    let mut child = Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let restored = restore(&import_sqlite_database(&result.stdout, SqliteDatabaseLimits::default(), &mut |_|true).unwrap()).unwrap();
    assert_eq!(restored.schema,snapshot.schema);
    assert_eq!(restored.entities[0].args.iter().map(|v|match v { IfcValue::Real(value)=>value.to_bits(),_=>panic!("real tag") }).collect::<Vec<_>>(),bits);
}

#[test]
fn sqlite_snapshot_ifc4_surrogate_root_header_ids_are_relationships_not_fixed_ones() {
    use std::{io::Write, process::{Command, Stdio}};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../📇️registry/🧬️contract/📐️part21/🧫️fixtures/🚦️sqlite-cohort/🔣️.json")).unwrap();
    let snapshot = fixture(); let original = project(&snapshot); let limits = SqliteDatabaseLimits::default();
    let offset = corpus["nativeOwned"]["surrogateOffset"].as_i64().unwrap();
    let script = format!("import{{Database}}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));db.exec('PRAGMA foreign_keys=OFF');for(const{{name}}of db.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all()){{const key=db.query('PRAGMA table_info('+name+')').all().find(column=>column.pk===1).name;const links=db.query('PRAGMA foreign_key_list('+name+')').all();db.exec('UPDATE '+name+' SET '+[key+'='+key+'+{offset}',...links.filter(link=>link.from!==key).map(link=>link.from+'='+link.from+'+{offset}')].join(','));}}if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('renumber integrity');await Bun.write(Bun.stdout,db.serialize());db.close();");
    let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&export_sqlite_database(&original,limits,&mut |_|true).unwrap()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let database=import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();let restored=restore(&database).expect("normalized surrogate relationships must retain exact root/header ownership");assert_eq!(project(&restored),original);
}

#[test]
fn sqlite_snapshot_ifc4_controlled_native_owned_frame_preserves_literal_schema_and_all_variants() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../📇️registry/🧬️contract/📐️part21/🧫️fixtures/🚦️sqlite-cohort/🔣️.json")).unwrap();
    let mut snapshot = fixture(); snapshot.schema = corpus["nativeOwned"]["schema"].as_str().unwrap().into();
    let expected = project(&snapshot); let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
        let payload = snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).expect("actual owner must produce full literal Native state");
        let restored = <IfcSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).expect("actual owner must construct full literal Native state");
        assert_eq!(project(&restored),expected);
        let codec = <IfcSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
        let dialect = store::os_io::ArtifactDialect{artifact_kind:"s.stdio.ifc".into(),standard:"4".into(),subset:"*".into()};
        assert_eq!((codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,expected);
        let reconstructed = (codec.import)(&snapshot.schema,&dialect,expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;
        let roundtrip = <IfcSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&reconstructed,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(); assert_eq!(project(&roundtrip),expected);
    }
}

#[test]
fn sqlite_snapshot_ifc4_actual_removal_keeps_literal_references_through_erased_native_and_sqlite_file() {
 use crate::schema::mutations::{apply_ifc_mutation,IfcMutation,remove_entity::RemoveEntity};
 use std::{io::Write,process::{Command,Stdio}};
 fn edit(value:&mut IfcValue,from:u64,to:u64){match value{IfcValue::Reference(word)if *word==from=>*word=to,IfcValue::Aggregate(items)|IfcValue::TypedValue{items,..}=>for item in items{edit(item,from,to)},_=>{}}}
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧩️intermediate/🔣️.json")).unwrap();
 let limits=SqliteDatabaseLimits::default();let provider=<IfcSnapshot as ArtifactSqliteSnapshot>::sqlite_codec();
 let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.ifc".into(),standard:"4".into(),subset:"*".into()};
 for case_ in corpus["cases"].as_array().unwrap(){
  let removed=case_["removedId"].as_str().unwrap().parse::<u64>().unwrap();let replacement=corpus["editReference"].as_str().unwrap().parse::<u64>().unwrap();
  let mut snapshot=fixture();snapshot.schema=corpus["schema"].as_str().unwrap().into();let mut expected=snapshot.clone();expected.entities.retain(|entity|entity.id!=removed);
  let outcome=apply_ifc_mutation(&mut snapshot,&IfcMutation::RemoveEntity(RemoveEntity{id:removed}));assert!(outcome.messages().is_empty());
  assert_eq!(<IfcSnapshot as store::ArtifactPack>::encode_pack(&snapshot),<IfcSnapshot as store::ArtifactPack>::encode_pack(&expected));
  let semantic=project(&snapshot);assert_eq!(semantic,project(&expected));
  for value in expected.header.file_description.iter_mut().chain(&mut expected.header.file_name).chain(&mut expected.header.file_schema){edit(value,removed,replacement);}
  for entity in &mut expected.entities{for value in &mut entity.args{edit(value,removed,replacement);}for part in &mut entity.complex{for value in &mut part.args{edit(value,removed,replacement);}}}
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
   let decoded=IfcSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(project(&decoded),semantic);
   let database=(provider.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert_eq!(database,semantic);
   let bytes=export_sqlite_database(&database,limits,&mut |_|true).unwrap();
   let oracle=r#"import{Database}from'bun:sqlite';const expected=EXPECTED_REFERENCES;const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query("SELECT v.reference_instance_id AS literal,e.instance_id AS resolved FROM ifc_value AS v LEFT JOIN ifc_entity AS e ON e.id=v.reference_entity_id WHERE v.kind='reference' ORDER BY v.id").all();if(JSON.stringify(rows)!==JSON.stringify(expected))throw Error('literal/resolved reference state');db.query("UPDATE ifc_value SET reference_instance_id=? WHERE kind='reference' AND reference_entity_id IS NULL").run(EDIT_LITERAL);await Bun.write(Bun.stdout,db.serialize());db.close();"#.replace("EXPECTED_REFERENCES",&serde_json::to_string(&case_["references"]).unwrap()).replace("EDIT_LITERAL",&serde_json::to_string(&corpus["editReference"]).unwrap());
   let mut child=Command::new("bun").args(["-e",&oracle]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
   let edited=import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();let imported=(provider.import)(&snapshot.schema,&dialect,edited,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;
   let actual=IfcSnapshot::decode_sqlite_snapshot_native(&imported,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(project(&actual),project(&expected));
   eprintln!("[DEBUG] IFC4 actual removal complete erased/file owner {removed} {encoding:?}");
  }
 }
}
