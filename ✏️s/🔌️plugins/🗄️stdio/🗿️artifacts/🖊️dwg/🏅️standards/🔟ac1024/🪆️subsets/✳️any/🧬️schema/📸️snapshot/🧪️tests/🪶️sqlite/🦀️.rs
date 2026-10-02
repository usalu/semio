//! 🧫️ DWG language-neutral document fidelity before any native binary encoding.
use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SqliteDatabaseLimits,SqliteSnapshotControl},ArtifactSqliteSnapshot};
use std::io::Write;
use std::process::{Command,Stdio};
#[path="🎭️bodies/🦀️.rs"]
mod bodies;

#[test]
fn sqlite_snapshot_dwg_borrowed_root_admission_precedes_typed_field_allocation(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️encoding.json")).unwrap();let mut snapshot=DwgSnapshot::default();snapshot.schema=fixture["controlledNative"]["text"].as_str().unwrap().repeat(fixture["controlledNative"]["repeat"].as_u64().unwrap() as usize);
    let limits=SqliteDatabaseLimits{max_rows:fixture["rootAdmissionMaximumRows"].as_u64().unwrap() as usize,..SqliteDatabaseLimits::default()};let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>()>limits.max_rows);
    let record=snapshot.__dsl_to_record();let mut progress=|_|true;let mut native=dsl::NativeDecodeControl::new(limits.max_value_bytes,&mut progress);
    assert!(super::sqlite::construct_native_record(&record,&mut native,limits).is_err(),"borrowed mandatory occurrence admission must reject before materializing typed fields");assert_eq!(native.owned_bytes(),0,"refused known row frontier must not allocate root strings or typed slots");
}

#[test]
fn sqlite_snapshot_dwg_intermediate_xrecord_native_and_sqlite_are_independent_of_wire_constraints(){
    use dsl::DslField;
    use semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️encoding.json")).unwrap();let row=&fixture["intermediateXRecord"];
    let code=i16::try_from(row["groupCode"].as_i64().unwrap()).unwrap();let count=row["binaryLength"].as_u64().unwrap() as usize;
    let values=vec![DwgXRecordValue::String{group_code:code,value:row["text"].as_str().unwrap().into()},DwgXRecordValue::Binary{group_code:code,octets:(0..count).map(|index|index as u8).collect()}];
    for value in &values{assert!(value.validate().is_err());assert_eq!(DwgXRecordValue::from_value(&value.to_value()).unwrap(),*value,"logical native fields preserve intermediate group codes and octets independently of DWG wire conformance");}
    let mut snapshot=DwgSnapshot::default();snapshot.drawing.objects=vec![DwgLogicalObject{body:Some(DwgLogicalObjectBody::XRecord(DwgXRecordBody{cloning_flag:u16::MAX,values,object_id_handles:vec![]})),..DwgLogicalObject::default()}];
    let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let bytes=export_sqlite_database(&expected,limits,&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(*) AS n FROM dwg_xrecord_binary_octet').get().n!==256)throw Error('octets');await Bun.write(Bun.stdout,d.serialize());d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(DwgSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
    for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let restored=DwgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);}
}

#[test]
fn sqlite_snapshot_dwg_controlled_native_admits_all_domain_rows_and_unicode() {
    use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️encoding.json")).unwrap();
    let mut snapshot=DwgSnapshot::default();
    snapshot.schema=fixture["controlledNative"]["text"].as_str().unwrap().repeat(fixture["controlledNative"]["repeat"].as_u64().unwrap() as usize);
    snapshot.header.units.unit1_conversion=f64::from_bits(0xfff0000000000123);
    snapshot.drawing.objects=bodies::authored_bodies().into_iter().enumerate().map(|(index,body)|DwgLogicalObject{handle:index as u64,type_code:u16::MAX,class_name:String::new(),category:DwgObjectCategory::Object,owner_handle:Some(u64::MAX),reactor_handles:vec![u64::MAX,0],extension_dictionary_handle:None,referenced_handles:vec![0,u64::MAX],extended_data:vec![],body:Some(body)}).collect();
    let limits=SqliteDatabaseLimits::default();
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
    let bytes=export_sqlite_database(&database,limits,&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');let rows=0;for(const{name}of d.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all())rows+=d.query('SELECT COUNT(*) AS n FROM '+name).get().n;await Bun.write(Bun.stdout,String(rows));d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let rows:usize=String::from_utf8(output.stdout).unwrap().parse().unwrap();assert_eq!(rows,database.tables.iter().map(|table|table.rows.len()).sum::<usize>());assert!(rows>snapshot.drawing.objects.len()+1);
    let record=snapshot.__dsl_to_record();let mut progress=|_|true;let mut native=dsl::NativeDecodeControl::new(limits.max_value_bytes,&mut progress);assert_eq!(super::sqlite::forecast_native_rows(&record,&mut native,limits).unwrap(),rows,"literal borrowed body forecast matches independent SQLite exactly");assert_eq!(native.owned_bytes(),0);
    let record=snapshot.__dsl_to_record();let limited=SqliteDatabaseLimits{max_rows:rows-1,..limits};let mut progress=|_|true;let mut native=dsl::NativeDecodeControl::new(limits.max_value_bytes,&mut progress);
    assert!(super::sqlite::construct_native_record(&record,&mut native,limited).is_err(),"all 43 borrowed body domains must admit their exact descendant frontier before typed construction");assert_eq!(native.owned_bytes(),0,"an independently counted descendant frontier must refuse before any typed string or collection allocation");
    for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
        let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
        let restored=DwgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
        assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),database);
        let limited=SqliteDatabaseLimits{max_rows:rows-1,..limits};
        assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err(),"every authored DWG occurrence consumes its row admission");
        assert!(DwgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).is_err(),"native reconstruction must admit every authored DWG row");
        let mut interior=false;assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut|event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==snapshot.schema.len()&&event.completed>=fixture["controlledNative"]["cancelAfter"].as_u64().unwrap() as usize&&event.completed<event.total{interior=true;false}else{true}},limits)).is_err());assert!(interior);
    }
}

#[test]
fn sqlite_snapshot_complete_typescript_native_relational_interoperability() {
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(7).unwrap();
    let base=root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
    let fixture=base.join("🧫️fixtures/🪶️sqlite/🟦️.ts");let provider=base.join("🪶️sqlite/🟦️.ts");
    let physical=root.join("🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts");
    let export_script=format!("import{{dwgSnapshotFixture as snapshot}}from{fixture:?};import{{dwgSnapshotToSqliteDatabase}}from{provider:?};import{{exportSqliteDatabase}}from{physical:?};import{{Database}}from'bun:sqlite';const n=Database.deserialize(await exportSqliteDatabase(await dwgSnapshotToSqliteDatabase(snapshot)));if(n.query('PRAGMA integrity_check').get().integrity_check!=='ok'||n.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,n.serialize());n.close();");
    let output=Command::new("bun").args(["-e",&export_script]).current_dir(root).output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let snapshot=DwgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let limits=SqliteDatabaseLimits::default();let rows=database.tables.iter().map(|table|table.rows.len()).sum::<usize>();let record=snapshot.__dsl_to_record();let mut progress=|_|true;let mut native=dsl::NativeDecodeControl::new(limits.max_value_bytes,&mut progress);assert_eq!(super::sqlite::forecast_native_rows(&record,&mut native,limits).unwrap(),rows,"borrowed forecast covers the independent all-entity, all-record and all-constraint SQLite corpus exactly");assert_eq!(native.owned_bytes(),0);
    let mut progress=|_|true;let mut native=dsl::NativeDecodeControl::new(limits.max_value_bytes,&mut progress);assert!(super::sqlite::construct_native_record(&record,&mut native,SqliteDatabaseLimits{max_rows:rows-1,..limits}).is_err());assert_eq!(native.owned_bytes(),0);
    let projected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(projected.table("dwg_entity").unwrap().rows.len(),19);assert_eq!(projected.table("dwg_table_record").unwrap().rows.len(),7);assert_eq!(projected.table("dwg_constraint_node").unwrap().rows.len(),14);
    let bytes=export_sqlite_database(&projected,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let import_script=format!("import{{dwgSnapshotFixture as snapshot}}from{fixture:?};import{{dwgSnapshotFromSqliteDatabase,dwgSnapshotToSqliteDatabase}}from{provider:?};import{{importSqliteDatabase}}from{physical:?};import assert from'node:assert/strict';import{{expect}}from'bun:test';import{{Database}}from'bun:sqlite';const n=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));assert.equal(n.query('PRAGMA integrity_check').get().integrity_check,'ok');assert.deepEqual(n.query('PRAGMA foreign_key_check').all(),[]);const restored=await dwgSnapshotFromSqliteDatabase(await importSqliteDatabase(n.serialize()));expect(restored).toEqual(snapshot);assert.deepEqual(await dwgSnapshotToSqliteDatabase(restored),await dwgSnapshotToSqliteDatabase(snapshot));n.close();");
    let mut child=Command::new("bun").args(["-e",&import_script]).current_dir(root).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    println!("[DEBUG] DWG full TypeScript-to-SQLite-to-native-to-SQLite-to-TypeScript exact typed interoperability verified across all 43 body, 19 entity, 7 record and 14 constraint variants");
}

#[test]
fn sqlite_snapshot_document_typed_metadata_full_unsigned_and_independent_edits() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📄️document/🔣️.json")).unwrap();
    let mut snapshot=DwgSnapshot::default();
    snapshot.schema=fixture["schema"].as_str().unwrap().into();
    snapshot.version=fixture["version"].as_str().unwrap().into();
    snapshot.maintenance_version=fixture["maintenanceVersion"].as_u64().unwrap() as u8;
    snapshot.codepage=fixture["codepage"].as_u64().unwrap() as u16;
    snapshot.summary.title=fixture["summaryTitle"].as_str().unwrap().into();
    snapshot.summary.total_editing_time=fixture["unsignedMaximum"].as_str().unwrap().parse().unwrap();
    snapshot.auxiliary_header.handle_seed=u64::MAX;
    snapshot.summary.custom_properties=fixture["customProperties"].as_array().unwrap().iter().map(|pair|DwgCustomProperty{key:pair[0].as_str().unwrap().into(),value:pair[1].as_str().unwrap().into()}).collect();
    snapshot.application.name=fixture["applicationName"].as_str().unwrap().into();
    snapshot.summary.subject="typed subject".into();snapshot.summary.author="typed author".into();snapshot.summary.keywords="one,two".into();snapshot.summary.comments="preserved comment".into();snapshot.summary.last_saved_by="last".into();snapshot.summary.revision_number="R17".into();snapshot.summary.hyperlink_base="relative/base".into();
    snapshot.summary.created_at=DwgJulianDate{days:u32::MAX,milliseconds:17};snapshot.summary.modified_at=DwgJulianDate{days:23,milliseconds:29};
    snapshot.application.version_checksum="version digest".into();snapshot.application.version="noncanonical version".into();snapshot.application.comment_checksum="comment digest".into();snapshot.application.comment="typed comment".into();snapshot.application.product_checksum="product digest".into();snapshot.application.product="typed product".into();snapshot.application.application_version="app version".into();
    snapshot.template=DwgTemplate{description:"typed template".into(),measurement:DwgMeasurement::Metric};
    snapshot.auxiliary_header.total_saves=u32::MAX;snapshot.auxiliary_header.save_partition_one=u16::MAX;snapshot.auxiliary_header.save_partition_two=13;snapshot.auxiliary_header.save_generation=17;
    snapshot.auxiliary_header.legacy_stamp_one=DwgVersionStamp{version:19,maintenance:23};snapshot.auxiliary_header.legacy_stamp_two=DwgVersionStamp{version:29,maintenance:31};
    snapshot.auxiliary_header.created_at=DwgJulianDate{days:37,milliseconds:41};snapshot.auxiliary_header.updated_at=DwgJulianDate{days:43,milliseconds:47};snapshot.auxiliary_header.terminal_save_generation=53;
    snapshot.classes=vec![DwgClass{number:u16::MAX,proxy_flags:u32::MAX,application_name:"class app".into(),cpp_class_name:"CppClass".into(),dxf_name:"DXF_CLASS".into(),was_zombie:true,item_class_id:61,object_count:67,dwg_version:71,maintenance_version:73,reserved_values:vec![79,u32::MAX]}];
    snapshot.dependencies=vec![DwgDependency{feature:"reference".into(),full_path:"/typed/🌠".into(),relative_path:"relative".into(),fingerprint:"fingerprint".into(),version:"version".into(),timestamp:u32::MAX,file_size:u32::MAX,affects_graphics:true,reference_count:83}];
    snapshot.application_history=DwgApplicationHistory{history_identifier_one:"id1".into(),history_identifier_two:"id2".into(),class_version:u32::MAX,application_version_digest:"av digest".into(),application_version:"av".into(),trust_comment_digest:"tc digest".into(),trust_comment:"tc".into(),property_set_digest:"ps digest".into(),property_format_identifier:"format".into(),properties:vec![DwgApplicationProperty{id:u32::MAX,kind:DwgApplicationPropertyKind::DateTime,value:"noncanonical timestamp".into()},DwgApplicationProperty{id:u32::MAX,kind:DwgApplicationPropertyKind::String,value:"duplicate property id".into()}],product_digest:"product digest".into(),product:DwgProductInformation{name:"product name".into(),build_version:"build".into(),registry_version:"registry".into(),install_id:"install".into(),locale_id:"locale".into()}};
    snapshot.revision_history.revisions=fixture["revisionValues"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u32).collect();
    snapshot.revision_history.format_major=89;snapshot.revision_history.format_minor=97;
    snapshot.preview.width=2;snapshot.preview.height=1;
    snapshot.preview.palette=vec![DwgRgba{red:1,green:2,blue:3,alpha:17},DwgRgba{red:255,green:0,blue:128,alpha:255}];
    snapshot.preview.pixel_indices=vec![1,0];
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let restored=DwgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored,snapshot);
    let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const r=d.query('SELECT schema,version FROM dwg_document').get();if(r.schema!=='DWG.SQLite.semantic'||r.version!=='AC1018')throw Error('typed metadata');const w=d.query('SELECT total_editing_time_high AS high,total_editing_time_low AS low FROM dwg_summary').get();if(w.high!==4294967295||w.low!==4294967295)throw Error('unsigned64');d.run(\"UPDATE dwg_summary SET title='Independent SQLite',total_editing_time_low=4294967294\");await Bun.write(Bun.stdout,d.serialize());d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output=child.wait_with_output().unwrap();
    assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let edited=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let restored=DwgSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored.summary.title,"Independent SQLite");
    assert_eq!(restored.summary.total_editing_time,u64::MAX-1);
    assert_eq!(restored.auxiliary_header.handle_seed,u64::MAX);
    assert_eq!(restored.summary.custom_properties,snapshot.summary.custom_properties);
    println!("[DEBUG] DWG typed document SQLite oracle preserved full unsigned metadata and duplicate properties");
}

#[test]
fn sqlite_snapshot_header_special_number_classes_survive_independent_sqlite() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️numbers/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let class=case["class"].as_str().unwrap();
        let value=match class{"finite"=>case["value"].as_f64().unwrap(),"negative_zero"=>-0.0,"nan"=>f64::NAN,"positive_infinity"=>f64::INFINITY,"negative_infinity"=>f64::NEG_INFINITY,_=>unreachable!()};
        let mut snapshot=DwgSnapshot::default();snapshot.header.units.unit1_conversion=value;
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const r=d.query('SELECT unit1_conversion_class AS kind,unit1_conversion AS value FROM dwg_header_units').get();if(r.kind!=='{class}'||('{}'!=='finite'&&r.value!==null))throw Error('numeric class');d.run(\"UPDATE dwg_header_units SET unit2_conversion_class='negative_zero',unit2_conversion_ieee754_bits=-9223372036854775808,unit2_conversion=NULL\");await Bun.write(Bun.stdout,d.serialize());d.close();",class);
        let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let edited=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let result=DwgSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let actual=result.header.units.unit1_conversion;
        assert_eq!(actual.to_bits(),value.to_bits());
        assert_eq!(result.header.units.unit2_conversion.to_bits(),(-0.0f64).to_bits());
    }
    println!("[DEBUG] DWG header all five numeric classes and independent signed-zero edit preserved");
}

#[test]
fn sqlite_snapshot_named_objects_associative_state_and_evaluation_tags() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📦️objects/🔣️.json")).unwrap();
    let expression=|value|DwgEvaluationExpression{parent_id:i32::MIN,major_version:u32::MAX,minor_version:17,node_id:u32::MAX,value};
    let mut snapshot=DwgSnapshot::default();
    let mut bodies=vec![
      DwgLogicalObjectBody::Placeholder(DwgPlaceholder{}),
      DwgLogicalObjectBody::DictionaryVariable(DwgDictionaryVariable{value:String::new()}),
      DwgLogicalObjectBody::AnnotationScale(DwgAnnotationScale{name:fixture["annotationScale"]["name"].as_str().unwrap().into(),paper_units:2.5,drawing_units:7.5,is_unit_scale:false}),
      DwgLogicalObjectBody::SortEntitiesTable(DwgSortEntitiesTable{block_header_handle:u64::MAX,entries:vec![DwgDrawOrderEntry{entity_handle:9007199254740993,sort_handle:u64::MAX}]}),
      DwgLogicalObjectBody::EvaluationGraph(DwgEvaluationGraph{nodes:vec![DwgEvaluationGraphNode{id:u32::MAX,expression_handle:u64::MAX},DwgEvaluationGraphNode{id:7,expression_handle:9007199254740993}],edges:vec![DwgEvaluationGraphEdge{from_node_id:u32::MAX,to_node_id:7,reference_count:u32::MAX,invertible:true,suppressed:false}]}),
      DwgLogicalObjectBody::AssociativeVariable(DwgAssociativeVariable{action:DwgAssociativeAction{status:DwgAssociativeActionStatus::UpToDate,owning_network_handle:Some(u64::MAX),action_body_handle:None,action_index:i32::MIN,maximum_dependency_index:i32::MAX,dependencies:vec![DwgAssociativeActionDependency{owned:true,dependency_handle:9007199254740993}]},name:fixture["variable"]["name"].as_str().unwrap().into(),expression:"2+3".into(),evaluator_id:"native".into(),description:String::new(),evaluated_value:DwgEvaluationVariant::Integer32(i32::MIN),mergeable:true,mergeable_variable_name:Some(String::new()),must_merge:false,referenced_value_dependency_handles:vec![u64::MAX,0]})
    ];
    for value in [DwgEvaluationExpressionValue::Empty,DwgEvaluationExpressionValue::Double(-0.0),DwgEvaluationExpressionValue::PointGroup10(vec![]),DwgEvaluationExpressionValue::PointGroup11(vec![1.25,-0.0]),DwgEvaluationExpressionValue::String(String::new()),DwgEvaluationExpressionValue::Integer32(i32::MIN),DwgEvaluationExpressionValue::ObjectReference(u64::MAX),DwgEvaluationExpressionValue::Integer16(i16::MIN)]{
      bodies.push(DwgLogicalObjectBody::DynamicBlockProxyNode(DwgDynamicBlockProxyNode{evaluation_expression:expression(value)}));
    }
    snapshot.drawing.objects=bodies.into_iter().enumerate().map(|(index,body)|DwgLogicalObject{handle:index as u64+1,type_code:u16::MAX,class_name:"explicit owned test".into(),category:DwgObjectCategory::Object,owner_handle:None,reactor_handles:vec![],extension_dictionary_handle:None,referenced_handles:vec![],extended_data:vec![],body:Some(body)}).collect();
    let db=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let restored=DwgSnapshot::from_sqlite_database(&db,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored,snapshot);
    let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const r=d.query('SELECT s.name,s.paper_units,g.node_identifier FROM dwg_annotation_scale s JOIN dwg_evaluation_graph_node g ON g.ordinal=0').get();if(r.name!=='尺度 ☃'||r.paper_units!==2.5||r.node_identifier!==4294967295)throw Error('typed join');const variants=d.query('SELECT value_kind FROM dwg_evaluation_expression ORDER BY id').all();if(variants.length!==8)throw Error('all evaluation kinds');d.run(\"UPDATE dwg_annotation_scale SET name='Independent scale'; UPDATE dwg_associative_variable SET evaluated_integer32=2147483647\");await Bun.write(Bun.stdout,d.serialize());d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let db=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let restored=DwgSnapshot::from_sqlite_database(&db,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let Some(DwgLogicalObjectBody::AnnotationScale(scale))=&restored.drawing.objects[2].body else{panic!()};assert_eq!(scale.name,"Independent scale");
    let Some(DwgLogicalObjectBody::AssociativeVariable(variable))=&restored.drawing.objects[5].body else{panic!()};assert_eq!(variable.evaluated_value,DwgEvaluationVariant::Integer32(i32::MAX));assert_eq!(variable.mergeable_variable_name,Some(String::new()));
    println!("[DEBUG] DWG explicit object/evaluation tables support independent joins and SQL edits");
}

#[test]
fn sqlite_snapshot_all_43_body_domains_and_14_constraint_tags_independent_sqlite() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎭️bodies/🔣️.json")).unwrap();
    let mut snapshot=DwgSnapshot::default();
    snapshot.drawing=DwgLogicalDrawing::from_native(&dwg_engine::DwgDrawing::default()).unwrap();
    for (index,body) in bodies::authored_bodies().into_iter().enumerate(){
        snapshot.drawing.objects.push(DwgLogicalObject{handle:10000+index as u64,type_code:u16::MAX,class_name:"authored body vector".into(),category:DwgObjectCategory::Object,owner_handle:Some(u64::MAX),reactor_handles:vec![0,u64::MAX],extension_dictionary_handle:Some(0),referenced_handles:vec![9007199254740993],extended_data:vec![],body:Some(body)});
    }
    snapshot.drawing.objects[0].extended_data=vec![DwgExtendedEntityData{application_handle:u64::MAX,values:vec![DwgXRecordValue::String{group_code:1000,value:"extended".into()}]}];
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let kinds:std::collections::BTreeSet<_>=database.table("dwg_object").unwrap().rows.iter().map(|row|row.text(12).unwrap()).collect();
    let expected:std::collections::BTreeSet<_>=fixture["bodyKinds"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();assert_eq!(kinds,expected);
    let restored=DwgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored,snapshot);
    let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const kinds=d.query('SELECT DISTINCT body_kind FROM dwg_object').all();if(kinds.length!==43)throw Error('body coverage');if(d.query('SELECT DISTINCT kind FROM dwg_constraint_node').all().length!==14)throw Error('constraint coverage');const rows=d.query('SELECT c.role,b.edge,m.horizontal_spacing FROM dwg_cell_style c JOIN dwg_cell_border b ON b.cell_style_id=c.id JOIN dwg_cell_margins m ON m.id=c.id ORDER BY c.ordinal,b.ordinal').all();if(rows.length!==8||rows.some(r=>r.horizontal_spacing!==6.25))throw Error('cell border join');if(d.query('SELECT channel FROM dwg_material_map ORDER BY ordinal').all().map(r=>r.channel).join(',')!=='diffuse,specular,reflection,opacity,bump,refraction')throw Error('typed maps');d.run('UPDATE dwg_block_stretch_action SET distance_multiplier=9.25,distance_multiplier_ieee754_bits=4621396905123905536; UPDATE dwg_constrained_implicit_point SET point_present=0');await Bun.write(Bun.stdout,d.serialize());d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let restored=DwgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let stretch=restored.drawing.objects.iter().find_map(|object|match &object.body{Some(DwgLogicalObjectBody::BlockStretchAction(value))=>Some(value),_=>None}).unwrap();assert_eq!(stretch.distance_multiplier,fixture["editedStretchDistance"].as_f64().unwrap());
    let group=restored.drawing.objects.iter().find_map(|object|match &object.body{Some(DwgLogicalObjectBody::Assoc2dConstraintGroup(value))=>Some(value),_=>None}).unwrap();
    let DwgConstraintNode::ConstrainedImplicitPoint(point)=&group.nodes[0]else{panic!()};assert_eq!(point.point,None);assert_eq!(group.nodes.len(),14);
    assert_eq!(restored.drawing.objects.len(),snapshot.drawing.objects.len());
    println!("[DEBUG] DWG all 43 object bodies and 14 constraint tags preserve typed state and independent SQLite edits");
}

#[test]
fn sqlite_snapshot_bounds_cancellation_and_malformed_typed_ownership() {
    let mut snapshot=DwgSnapshot::default();
    snapshot.drawing.objects=vec![DwgLogicalObject{handle:u64::MAX,type_code:0,class_name:String::new(),category:DwgObjectCategory::Object,owner_handle:None,reactor_handles:vec![0,u64::MAX],extension_dictionary_handle:None,referenced_handles:vec![],extended_data:vec![],body:Some(DwgLogicalObjectBody::AnnotationScale(DwgAnnotationScale{name:String::new(),paper_units:-0.0,drawing_units:f64::NAN,is_unit_scale:true}))}];
    let limits=SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()};
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap_err().contains("row"));
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).unwrap_err().contains("cancel"));
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    assert!(DwgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).unwrap_err().contains("cancel"));
    let mut wrong=database.clone();wrong.table_mut("dwg_object_reactor_handle").unwrap().rows[0].values[2]=semio_framework_os_kernel::sqlite_snapshot::SqliteValue::Integer(7);
    assert!(DwgSnapshot::from_sqlite_database(&wrong,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap_err().contains("ordinal"));
    let mut wrong=database.clone();wrong.table_mut("dwg_annotation_scale").unwrap().rows[0].values[2]=semio_framework_os_kernel::sqlite_snapshot::SqliteValue::Text("finite".into());
    assert!(DwgSnapshot::from_sqlite_database(&wrong,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
    let mut wrong=database.clone();wrong.table_mut("dwg_object").unwrap().rows[0].values[12]=semio_framework_os_kernel::sqlite_snapshot::SqliteValue::Null;
    assert!(DwgSnapshot::from_sqlite_database(&wrong,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap_err().contains("unowned"));
    let limits=SqliteDatabaseLimits{max_value_bytes:1,..SqliteDatabaseLimits::default()};
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
    println!("[DEBUG] DWG resource and cancellation controls reject invalid typed ownership and numeric presence");
}

#[test]
fn sqlite_snapshot_exact_nan_payload_identity_and_exact_owned_dialects() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️numbers/🔣️.json")).unwrap();
    let dialects:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎭️bodies/🔣️.json")).unwrap();
    for bits in fixture["float64Bits"].as_array().unwrap().iter().map(|value|u64::from_str_radix(value.as_str().unwrap(),16).unwrap()){
        let mut snapshot=DwgSnapshot::default();snapshot.version="noncanonical".into();snapshot.header.units.unit1_conversion=f64::from_bits(bits);
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        for row in dialects["sqliteDialects"].as_array().unwrap(){
            let dialect=semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:row["artifactKind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()};
            assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_ok());
        }
        for row in dialects["invalidSqliteDialects"].as_array().unwrap(){
            let dialect=semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:row["artifactKind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()};
            assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
        }
        assert!(<DwgSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().is_some());
        let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let signed=bits as i64;
        let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const r=d.query('SELECT CAST(unit1_conversion_ieee754_bits AS TEXT) AS bits,unit1_conversion AS value FROM dwg_header_units').get();if(r.bits!=='{signed}'||r.value!==null)throw Error('exact IEEE identity');await Bun.write(Bun.stdout,d.serialize());d.close();");
        let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let restored=DwgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored.header.units.unit1_conversion.to_bits(),bits);
    }
    println!("[DEBUG] DWG exact quiet/signaling/signed NaN payloads and negative zero survive independent SQLite");
}

#[test]
fn sqlite_snapshot_independent_well_formed_sqlite_rejects_invalid_domain_edits() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📦️objects/🔣️.json")).unwrap();
    let mut snapshot=DwgSnapshot::default();
    snapshot.drawing.objects=vec![DwgLogicalObject{handle:1,type_code:0,class_name:String::new(),category:DwgObjectCategory::Object,owner_handle:None,reactor_handles:vec![0,u64::MAX],extension_dictionary_handle:None,referenced_handles:vec![],extended_data:vec![],body:Some(DwgLogicalObjectBody::AnnotationScale(DwgAnnotationScale{name:String::new(),paper_units:2.5,drawing_units:7.5,is_unit_scale:true}))}];
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
    let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    for case in fixture["invalidSqliteEdits"].as_array().unwrap(){
        let sql=serde_json::to_string(case["sql"].as_str().unwrap()).unwrap();
        let script=format!("import{{Database}}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));d.run({sql});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('invalid SQLite fixture');await Bun.write(Bun.stdout,d.serialize());d.close();");
        let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
        let error=DwgSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap_err();
        assert!(error.contains(case["error"].as_str().unwrap()),"{error}");
    }
    println!("[DEBUG] DWG rejects independently edited valid SQLite with ordinal, IEEE identity, presence and body ownership violations");
}

fn erased_nan_words(encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️numbers/🔣️.json")).unwrap();
    let dialect=semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:"s.stdio.dwg".into(),standard:"ac1024".into(),subset:"*".into()};
    let codec=<DwgSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    for value in fixture["nativeEncodingFloat64Bits"].as_array().unwrap(){
        let bits=u64::from_str_radix(value.as_str().unwrap(),16).unwrap();let mut snapshot=DwgSnapshot::default();snapshot.version="noncanonical".into();snapshot.header.units.unit1_conversion=f64::from_bits(bits);
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let payload=(codec.import)(&snapshot.schema,&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
        let restored=(codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
        assert_eq!(DwgSnapshot::from_sqlite_database(&restored,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().header.units.unit1_conversion.to_bits(),bits,"native erased snapshot encoding must preserve every IEEE word");
        assert_eq!(restored,database);
    }
}
#[test]
fn sqlite_snapshot_erased_binary_preserves_exact_nan_payload_words(){erased_nan_words(semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary);}
#[test]
fn sqlite_snapshot_erased_text_preserves_exact_nan_payload_words(){erased_nan_words(semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text);}

#[test]
fn sqlite_snapshot_dwg_native_encoding_preflight_admits_owned_model_and_refuses_budget_or_cancellation(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let snapshot=DwgSnapshot::default();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=1;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}

#[test]
fn sqlite_snapshot_dwg_native_encoding_preflight_bounds_escaped_text_and_cancels_borrowed_members(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️encoding.json")).unwrap();
 let mut snapshot=DwgSnapshot::default();snapshot.summary.custom_properties=vec![DwgCustomProperty{key:"x".into(),value:"\n\\\"".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize)}];snapshot.drawing.objects=vec![DwgLogicalObject::default();cases["workItems"].as_u64().unwrap() as usize];
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut limits=SqliteDatabaseLimits::default();limits.max_value_bytes=cases["smallBudgetBytes"].as_u64().unwrap() as usize;
  assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
  let mut reached=false;assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=cases["cancelAfterWork"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached,"member admission walk must checkpoint before native ownership");
 }
}

#[test]
fn sqlite_snapshot_dwg_owned_guard_rejects_independent_non_document_edits() {
    let mut snapshot=DwgSnapshot::default();snapshot.summary.title="before SQL edit".into();
    let limits=SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let bytes=export_sqlite_database(&database,limits,&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));d.run(\"UPDATE dwg_summary SET title='independently edited state'\");if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,d.serialize());d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let edited=import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎭️bodies/🔣️.json")).unwrap();
    for row in fixture["sqliteDialects"].as_array().unwrap(){let dialect=store::io_schema::ArtifactDialect{artifact_kind:row["artifactKind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()};assert!(snapshot.validate_sqlite_snapshot_subset(&dialect,&edited,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err(),"guard must compare complete typed state");let restored=DwgSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();restored.validate_sqlite_snapshot_subset(&dialect,&edited,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();}
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_dwg_actual_declared_ac1018_ac1024_typed_and_erased_io() {
    use store::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot,io_route,io_run_with_snapshot_control}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("DWG actual owned SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let mut snapshot=DwgSnapshot::default();snapshot.version="independent intermediate version".into();snapshot.summary.total_editing_time=u64::MAX;snapshot.summary.title="世界 🖊️".into();snapshot.header.units.unit1_conversion=f64::from_bits(0xfff0000000000123);
    let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎭️bodies/🔣️.json")).unwrap();
    for row in fixture["sqliteDialects"].as_array().unwrap(){let dialect=ArtifactDialect{artifact_kind:row["artifactKind"].as_str().unwrap().into(),standard:row["standard"].as_str().unwrap().into(),subset:row["subset"].as_str().unwrap().into()};let export=io_route(&dialect,&sqlite,1).await.unwrap().value;let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
        for encoding in[store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text]{let file=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,limits,&mut |_|true).await.unwrap().value;let restored=io_import_sqlite_snapshot::<DwgSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value;assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let file=io_run_with_snapshot_control(&export,payload,limits,&mut |_|true).await.unwrap().value;let payload=io_run_with_snapshot_control(&import,file,limits,&mut |_|true).await.unwrap().value;let restored=DwgSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);}
    }
}
