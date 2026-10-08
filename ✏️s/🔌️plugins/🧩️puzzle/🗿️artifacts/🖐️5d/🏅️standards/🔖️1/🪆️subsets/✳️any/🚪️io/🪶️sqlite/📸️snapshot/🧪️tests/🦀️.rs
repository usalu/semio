//! 🧩️ Actual Puzzle5d capability and native scalar baselines precede owner mounting.
use crate::standards::v1::subsets::any::io::sqlite::snapshot::Puzzle5dSnapshot;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::*};

fn laws()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}

fn complete(word:u64)->Puzzle5dSnapshot{
 use crate::*;
 let value=f64::from_bits(word);let f=laws();let child=&f["kindCatalogChild"];let target=&child["target"];let dialect=&target["dialect"];
 Puzzle5dSnapshot{schema:String::new(),domain:String::new(),label:Some(String::new()),meta:Puzzle5dMeta{description:"literal\0😀".into()},kind_catalogs:Some(store::ArtifactChild::new(child["childId"].as_str().unwrap().into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:target["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:dialect["artifactKind"].as_str().unwrap().into(),standard:dialect["standard"].as_str().unwrap().into(),subset:dialect["subset"].as_str().unwrap().into()}})),
  kind_catalogs_extra:Some(Puzzle5dKindCatalogsExtra{
   parts:vec![Puzzle5dCatalogPartKindExtra{id:String::new(),name:String::new(),label:String::new(),description:String::new(),icon:String::new(),image:String::new(),unit:String::new(),is_abstract:false,base_kinds:vec![String::new(),String::new()],representations:vec![Puzzle5dRepresentation{id:String::new(),name:String::new(),url:String::new(),mime:String::new(),tags:vec![String::new(),String::new()],lod:None,description:String::new()}],grips:vec![Puzzle5dGripTemplate{id:String::new(),name:String::new(),label:String::new(),description:String::new(),icon:String::new(),grip_kind:None,point:[value;3],direction:[value;3],t:Some(value),mandatory:Some(false),radius:None}],attributes:vec![Puzzle5dAttribute{id:String::new(),key:String::new(),value:String::new(),definition:None}],authors:vec![Puzzle5dAuthor{id:String::new(),name:String::new(),email:String::new(),role:None,rank:Some(i32::MIN)}]}],
   grips:vec![Puzzle5dCatalogGripKindExtra{id:String::new(),code:None,label:Some(String::new()),order:Some(i32::MAX),compatible_with:vec![String::new(),String::new()],description:String::new(),icon:String::new(),color:String::new(),default_rope_kind:String::new()}],fasteners:vec![Puzzle5dCatalogFastenerKindExtra{id:String::new(),name:String::new(),label:None}],ropes:vec![Puzzle5dCatalogRopeKindExtra{id:String::new(),name:String::new(),label:String::new(),default_fastener_kind:String::new()}]}),
  kind_compatibility:vec![Puzzle5dKindCompatibility{source:"unresolved".into(),target:String::new(),bidirectional:false,important:true,specificity:Puzzle5dCompatSpecificity::Rope}],
  parts:vec![Puzzle5dPart{id:String::new(),part_kind:None,anchor:Puzzle5dPartAnchor::Derived,part_2d:Puzzle5dPart2d{x:value,y:value,shape:Some(String::new()),radius:Some(value),width:None,height:Some(value),text:None,icon_kind:Some(String::new()),hidden:None,locked:Some(false)},part_3d:Puzzle5dPart3d{origin:[value;3],mesh_url:Some(String::new()),orientation:Some([value;4]),scale:Some(Puzzle5dScale::Uniform(value)),label:None},grips:vec![Puzzle5dGrip{id:String::new(),grip_kind:Some("unresolved".into()),grip_2d:Puzzle5dGrip2d{angle:value,grip_kind:None,radius:Some(value)},grip_3d:Puzzle5dGrip3d{position:[value;3],direction:Some([value;3]),radius:None,label:Some(String::new())}}]}],
  fasteners:vec![Puzzle5dFastener{id:String::new(),source:String::new(),target:"unresolved".into(),fastener_kind:None,gap:value,shift:value,rise:value,rotation:value,turn:value,tilt:value,x:value,y:value}],
  target_volumes:vec![Puzzle5dTargetVolume{id:String::new(),origin:[value;3],orientation:None,scale:Some(Puzzle5dScale::Vec3([value;3])),hidden:false,locked:true}]}
}
fn project(s:&Puzzle5dSnapshot)->SqliteDatabase{s.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(d:&SqliteDatabase)->Result<Puzzle5dSnapshot,semio_framework_value::ValueError>{Puzzle5dSnapshot::from_sqlite_database(d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))}

#[test]
fn sqlite_snapshot_puzzle5d_all_entities_and_words_cross_real_physical_files(){
 for hex in laws()["binary64Bits"].as_array().unwrap(){let expected=project(&complete(u64::from_str_radix(hex.as_str().unwrap(),16).unwrap()));for(name,count)in laws()["tableRowCounts"].as_object().unwrap(){assert_eq!(expected.table(name).unwrap().rows.len(),count.as_u64().unwrap()as usize,"{name}");}let bytes=export_sqlite_database(&expected,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let d=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(project(&restore(&d).unwrap()),expected);}
}

#[test]
fn sqlite_snapshot_puzzle5d_independent_sqlite_queries_edits_and_surrogate_renumbering(){
 use std::io::Write;use std::process::{Command,Stdio};let source=project(&complete(0));
 let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const t=d.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all();if(t.length!==25)throw Error('tablecount');for(const{name}of t)if(d.query('SELECT COUNT(*) AS n FROM '+name).get().n<1n)throw Error('empty table');d.exec('UPDATE puzzle5_part_board SET x=2,x_ieee754_bits=4611686018427387904,x_numeric_class=\'finite\'');d.query('UPDATE puzzle5_representation SET description=?').run('edited 日本\u0000');for(const{name}of t){d.exec('UPDATE '+name+' SET id=id+1000');for(const fk of d.query('PRAGMA foreign_key_list('+name+')').all())d.exec('UPDATE '+name+' SET '+fk.from+'='+fk.from+'+1000')}if(d.query('PRAGMA foreign_key_check').all().length)throw Error('renumbered foreignkeys');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&export_sqlite_database(&source,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let actual=restore(&import_sqlite_database(&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();let mut expected=complete(0);expected.parts[0].part_2d.x=2.;expected.kind_catalogs_extra.as_mut().unwrap().parts[0].representations[0].description="edited 日本\0".into();assert_eq!(project(&actual),project(&expected));
}

#[test]
fn sqlite_snapshot_puzzle5d_rejects_malformed_graphs_presence_and_scalar_domains(){
 let source=project(&complete(0));for n in 0..15{let mut d=source.clone();match n{
  0=>d.table_mut("puzzle5_part_board").unwrap().rows[0].values[3]=SqliteValue::Integer(1),
  1=>{let row=&mut d.table_mut("puzzle5_part_world").unwrap().rows[0];for i in 15..18{row.values[i]=SqliteValue::Null;}},
  2=>d.table_mut("puzzle5_part_scale").unwrap().rows[0].values[2]=SqliteValue::Text("vec3".into()),
  3=>d.table_mut("puzzle5_grip").unwrap().rows[0].values[2]=SqliteValue::Integer(2),
  4=>{let t=d.table_mut("puzzle5_catalog_extra").unwrap();let mut r=t.rows[0].clone();r.rowid=2;r.values[0]=SqliteValue::Integer(2);t.rows.push(r);},
  5=>d.table_mut("puzzle5_author").unwrap().rows[0].values[7]=SqliteValue::Integer(i64::from(i32::MAX)+1),
  6=>d.table_mut("puzzle5_grip_board").unwrap().rows.clear(),
  7=>d.table_mut("puzzle5_part").unwrap().rows[0].values[5]=SqliteValue::Text("invalid".into()),
  8=>d.table_mut("puzzle5_attribute").unwrap().rows[0].values[1]=SqliteValue::Integer(999),
  9=>d.table_mut("puzzle5_part_kind_base").unwrap().rows[1].values[2]=SqliteValue::Integer(0),
  10=>d.table_mut("puzzle5_part").unwrap().rows[0].values[0]=SqliteValue::Integer(9),
  11=>d.table_mut("puzzle5_part_kind").unwrap().rows[0].values[10]=SqliteValue::Integer(2),
  12=>{let row=&mut d.table_mut("puzzle5_part_board").unwrap().rows[0];row.values[2]=SqliteValue::Integer(9007199254740993);row.values[3]=SqliteValue::Integer(9007199254740992f64.to_bits()as i64);},
  13=>{let t=d.table_mut("puzzle5_document").unwrap();t.sql=t.sql.replacen("TEXT NOT NULL","TEXT \"NOT\" \"NULL\"",1);},
  _=>d.table_mut("puzzle5_part_board").unwrap().rows[0].values[10]=SqliteValue::Null,
 }assert!(restore(&d).is_err(),"malformed case {n}");}
}

#[test]
fn sqlite_snapshot_puzzle5d_actual_typed_and_play_erased_io_preserves_every_field(){
 let declared=crate::standards::v1::subsets::any::schema::snapshot::native_codec();let typed=declared.snapshot_sqlite.expect("actual typed declaration");let typed_schema=declared.schema;
 let play=store::ArtifactCodec::bare::<crate::Puzzle5dPlaySnapshot,crate::Puzzle5dMutation>(crate::PUZZLE_5D_SCHEMA);let play_schema=play.schema;let play=play.snapshot_sqlite.expect("actual Play declaration");let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.puzzle.puzzle5d".into(),standard:"1".into(),subset:"*".into()};
 for hex in laws()["binary64Bits"].as_array().unwrap(){let s=complete(u64::from_str_radix(hex.as_str().unwrap(),16).unwrap());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&s)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&s))};for(provider,schema)in[(&typed,&typed_schema),(&play,&play_schema)]{let d=(provider.export)(schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(project(&restore(&d).unwrap()),project(&s));let actual=(provider.import)(schema,&dialect,d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(actual,payload);}}}
}

#[test]
fn sqlite_snapshot_puzzle5d_exact_borrowed_and_owned_row_frontiers_admit_before_copy(){
 let s=complete(0);let d=project(&s);let count=d.tables.iter().map(|t|t.rows.len()).sum::<usize>();for max_rows in[count,count-1]{let limits=SqliteDatabaseLimits{max_rows,..SqliteDatabaseLimits::default()};let mut accept=|_|true;let mut c=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(s.to_sqlite_database(&mut c).is_ok(),max_rows==count);let mut c=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(Puzzle5dSnapshot::from_sqlite_database(&d,&mut c).is_ok(),max_rows==count);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut c=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(s.encode_sqlite_snapshot_native(encoding,&mut c).is_ok(),max_rows==count);}}
 let mut s=s;s.parts[0].part_2d.text=Some("😀".repeat(100000));let limits=SqliteDatabaseLimits{max_schema_bytes:32,..SqliteDatabaseLimits::default()};assert!(s.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(restore(&project(&Puzzle5dSnapshot::default())).is_ok());
}

#[test]
fn sqlite_snapshot_puzzle5d_all_four_copy_phases_have_real_interior_cancellation(){
 let mut s=complete(0);s.kind_catalogs_extra.as_mut().unwrap().parts[0].description="日本😀".repeat(32768);let d=project(&s);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{let mut interior=false;let mut callback=|p:SqliteSnapshotProgress|{if p.phase==phase&&p.total>65536&&p.completed>=65536&&p.completed<p.total{interior=true;false}else{true}};let mut c=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let result=match phase{SqliteSnapshotPhase::ProjectSnapshot=>s.to_sqlite_database(&mut c).map(|_|()),SqliteSnapshotPhase::ReconstructSnapshot=>Puzzle5dSnapshot::from_sqlite_database(&d,&mut c).map(|_|()),SqliteSnapshotPhase::EncodeNative=>s.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut c).map(|_|()),_=>Puzzle5dSnapshot::decode_sqlite_snapshot_native(&store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&s)),&mut c).map(|_|())};assert!(result.is_err());assert!(interior,"phase {phase:?}");}
}

#[test]
fn sqlite_snapshot_puzzle5d_actual_native_capability_is_explicit(){
 assert!(store::ArtifactCodec::bare::<Puzzle5dSnapshot,crate::Puzzle5dMutation>(crate::PUZZLE_5D_SCHEMA).snapshot_sqlite.is_some(),"Puzzle5d actual native owner must declare semantic SQLite capability");
}

#[test]
fn sqlite_snapshot_puzzle5d_play_native_capability_owns_the_same_complete_parent(){
 assert!(store::ArtifactCodec::bare::<crate::Puzzle5dPlaySnapshot,crate::Puzzle5dMutation>(crate::PUZZLE_5D_SCHEMA).snapshot_sqlite.is_some(),"Puzzle5d actual Play owner must declare its complete persisted parent capability");
}

#[test]
fn sqlite_snapshot_puzzle5d_native_part_and_scale_words_survive_actual_text_and_pack(){
 for hex in laws()["binary64Bits"].as_array().unwrap(){
  let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);
  let mut expected=Puzzle5dSnapshot::default();let mut part=crate::Puzzle5dPart::default();part.id="literal\0😀".into();part.part_2d.x=value;part.part_2d.y=value;part.part_3d.origin=[value;3];part.part_3d.scale=Some(crate::Puzzle5dScale::Uniform(value));expected.parts.push(part);
  let packed=<Puzzle5dSnapshot as store::ArtifactPack>::encode_pack_with(&expected,&store::PackEncodeOptions::default()).unwrap();
  let binary=<Puzzle5dSnapshot as store::ArtifactPack>::decode_pack_with(&packed,&store::PackDecodeOptions::default()).unwrap();
  let text=<Puzzle5dSnapshot as store::ArtifactDsl>::print_dsl(&expected);let textual=<Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap();
  for actual in[binary,textual]{assert_eq!(actual.parts[0].part_2d.x.to_bits(),bits,"x {hex}");assert_eq!(actual.parts[0].part_2d.y.to_bits(),bits,"y {hex}");assert_eq!(actual.parts[0].part_3d.origin.map(f64::to_bits),[bits;3],"origin {hex}");match actual.parts[0].part_3d.scale{Some(crate::Puzzle5dScale::Uniform(value))=>assert_eq!(value.to_bits(),bits,"scale {hex}"),_=>panic!("uniform scale variant lost")};assert_eq!(actual.parts[0].id,expected.parts[0].id);}
 }
}

#[test]
fn sqlite_snapshot_puzzle5d_actual_play_text_retains_the_typed_word_authority(){
 for hex in laws()["binary64Bits"].as_array().unwrap(){
  let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=Puzzle5dSnapshot::default();let mut part=crate::Puzzle5dPart::default();part.id="literal\0😀".into();part.part_2d.x=f64::from_bits(bits);expected.parts.push(part);
  let packed=<Puzzle5dSnapshot as store::ArtifactPack>::encode_pack_with(&expected,&store::PackEncodeOptions::default()).unwrap();let play=<crate::Puzzle5dPlaySnapshot as store::ArtifactPack>::decode_pack_with(&packed,&store::PackDecodeOptions::default()).unwrap();
  let text=<crate::Puzzle5dPlaySnapshot as store::ArtifactDsl>::print_dsl(&play);let restored=<crate::Puzzle5dPlaySnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap();assert_eq!(restored.typed().parts.len(),1,"part ownership {hex}");assert_eq!(restored.typed().parts[0].part_2d.x.to_bits(),bits,"actual Play Text {hex}");
 }
}

#[test]
fn sqlite_snapshot_puzzle5d_controlled_native_scale_preserves_both_exact_variants(){
 for hex in laws()["binary64Bits"].as_array().unwrap(){
  let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);
  for expected in [crate::Puzzle5dScale::Uniform(value),crate::Puzzle5dScale::Vec3([value;3])]{
   let mut accepted=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(4096,&mut accepted);
   let shape=<crate::Puzzle5dScale as semio_framework_dsl_record::DslField>::shape_controlled(&mut encoding).expect("Scale requires genuine controlled metadata");assert!(matches!(shape,semio_framework_dsl_record::Shape::List(_)));
   let field=<crate::Puzzle5dScale as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut encoding).expect("Scale requires genuine controlled output");
   let mut accepted_decode=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(4096,&mut accepted_decode);let actual=<crate::Puzzle5dScale as semio_framework_dsl_record::DslField>::from_value_controlled(&field,&mut decoding).expect("Scale requires genuine controlled input");
   match(expected,actual){(crate::Puzzle5dScale::Uniform(_),crate::Puzzle5dScale::Uniform(actual))=>assert_eq!(actual.to_bits(),bits),(crate::Puzzle5dScale::Vec3(_),crate::Puzzle5dScale::Vec3(actual))=>assert_eq!(actual.map(f64::to_bits),[bits;3]),_=>panic!("Scale variant changed")}
  }
 }
 let mut cancelled=|_|false;let mut encoding=semio_framework_value::NativeEncodeControl::new(4096,&mut cancelled);assert!(<crate::Puzzle5dScale as semio_framework_dsl_record::DslField>::to_value_controlled(&crate::Puzzle5dScale::Uniform(0.0),&mut encoding).is_err());
 let mut accepted=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(0,&mut accepted);assert!(<crate::Puzzle5dScale as semio_framework_dsl_record::DslField>::from_value_controlled(&semio_framework_dsl_record::FieldValue::List(vec![semio_framework_dsl_record::FieldValue::Float(0.0)]),&mut decoding).is_err());
}

#[test]
fn sqlite_snapshot_puzzle5d_borrowed_scale_matches_neutral_list_shape() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(corpus["scaleDsl"]["shape"], "list");
    assert_eq!(corpus["scaleDsl"]["item"], "float");
    let semio_framework_dsl_record::BorrowedShape::List(item) = <crate::Puzzle5dScale as semio_framework_dsl_record::BorrowedDslField>::SHAPE else { panic!("Scale metadata lost its self-delimiting list") };
    assert!(matches!(item(), semio_framework_dsl_record::BorrowedShape::Float));
    for row in corpus["scaleDsl"]["invalidSamples"].as_array().unwrap() {
        let sample: Vec<f64> = serde_json::from_value(row.clone()).unwrap();
        let value = <Vec<f64> as semio_framework_dsl_record::DslField>::to_value(&sample);
        assert!(<crate::Puzzle5dScale as semio_framework_dsl_record::DslField>::from_value(&value).is_err());
    }
    for row in corpus["scaleDsl"]["samples"].as_array().unwrap() {
        let sample: Vec<f64> = serde_json::from_value(row.clone()).unwrap();
        let scale = if sample.len() == 1 { crate::Puzzle5dScale::Uniform(sample[0]) } else { crate::Puzzle5dScale::Vec3([sample[0],sample[1],sample[2]]) };
        let value = <crate::Puzzle5dScale as semio_framework_dsl_record::DslField>::to_value(&scale);
        assert_eq!(value, <Vec<f64> as semio_framework_dsl_record::DslField>::to_value(&sample));
    }
}
