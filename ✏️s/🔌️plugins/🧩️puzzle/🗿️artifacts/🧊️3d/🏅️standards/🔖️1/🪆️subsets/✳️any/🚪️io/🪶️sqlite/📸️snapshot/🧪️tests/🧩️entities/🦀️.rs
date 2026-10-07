//! 🧩️ Every native entity crosses independent SQLite and bounded owned bridges.
use super::{specimen,words,Puzzle3dSnapshot};
use crate::*;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::*};
fn project(s:&Puzzle3dSnapshot)->SqliteDatabase{s.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(d:&SqliteDatabase)->Result<Puzzle3dSnapshot,String>{Puzzle3dSnapshot::from_sqlite_database(d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))}
fn laws()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()}

#[test]
fn sqlite_snapshot_puzzle3d_all_entities_and_words_cross_real_physical_files(){for word in words(){let expected=project(&specimen(word));for(name,count)in laws()["nativeTableRowCounts"].as_object().unwrap(){assert_eq!(expected.table(name).unwrap().rows.len(),count.as_u64().unwrap()as usize,"{name}");}let bytes=export_sqlite_database(&expected,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let d=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(project(&restore(&d).unwrap()),expected);}}

#[test]
fn sqlite_snapshot_puzzle3d_independent_sqlite_queries_edits_and_surrogate_renumbering(){
 use std::io::Write;use std::process::{Command,Stdio};let source=project(&specimen(0));
 let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const counts=JSON.parse(process.argv[1]);const tables=d.query('SELECT name FROM sqlite_schema WHERE type=\'table\'').all();if(tables.length!==23)throw Error('tablecount');for(const{name}of tables)if(d.query('SELECT COUNT(*) AS n FROM '+name).get().n!==BigInt(counts[name]))throw Error('entity '+name);d.exec('UPDATE puzzle3_object SET origin_x=2,origin_x_ieee754_bits=4611686018427387904,origin_x_numeric_class=\'finite\'');d.query('UPDATE puzzle3_reference_source SET url=?').run('edited 日本\u0000');for(const{name}of tables){d.exec('UPDATE '+name+' SET id=id+1000');for(const fk of d.query('PRAGMA foreign_key_list('+name+')').all())d.exec('UPDATE '+name+' SET '+fk.from+'='+fk.from+'+1000')}if(d.query('PRAGMA foreign_key_check').all().length)throw Error('renumbered foreignkeys');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
 let counts=laws()["nativeTableRowCounts"].to_string();let mut child=Command::new("bun").args(["-e",script,&counts]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&export_sqlite_database(&source,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let actual=restore(&import_sqlite_database(&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();let mut expected=specimen(0);expected.objects[0].origin[0]=2.;expected.references[0].source.url="edited 日本\0".into();assert_eq!(project(&actual),project(&expected));
}

#[test]
fn sqlite_snapshot_puzzle3d_rejects_malformed_graphs_presence_and_scalar_domains(){let source=project(&specimen(0));for n in 0..16{let mut d=source.clone();match n{
 0=>d.table_mut("puzzle3_object").unwrap().rows[0].values[8]=SqliteValue::Integer(1),
 1=>{let row=&mut d.table_mut("puzzle3_object").unwrap().rows[0];for i in 19..22{row.values[i]=SqliteValue::Null;}},
 2=>d.table_mut("puzzle3_object_scale").unwrap().rows[0].values[2]=SqliteValue::Text("vec3".into()),
 3=>d.table_mut("puzzle3_vortex").unwrap().rows[0].values[2]=SqliteValue::Integer(2),
 4=>{let t=d.table_mut("puzzle3_catalog").unwrap();let mut r=t.rows[0].clone();r.rowid=2;r.values[0]=SqliteValue::Integer(2);t.rows.push(r);},
 5=>d.table_mut("puzzle3_author").unwrap().rows[0].values[7]=SqliteValue::Integer(i64::from(i32::MAX)+1),
 6=>d.table_mut("puzzle3_reference_source").unwrap().rows.clear(),
 7=>d.table_mut("puzzle3_object").unwrap().rows[0].values[6]=SqliteValue::Text("invalid".into()),
 8=>d.table_mut("puzzle3_attribute").unwrap().rows[0].values[1]=SqliteValue::Integer(999),
 9=>d.table_mut("puzzle3_object_kind_base").unwrap().rows[1].values[2]=SqliteValue::Integer(0),
 10=>d.table_mut("puzzle3_object").unwrap().rows[0].values[0]=SqliteValue::Integer(9),
 11=>d.table_mut("puzzle3_object_kind").unwrap().rows[0].values[10]=SqliteValue::Integer(2),
 12=>{let row=&mut d.table_mut("puzzle3_object").unwrap().rows[0];row.values[7]=SqliteValue::Integer(9007199254740993);row.values[8]=SqliteValue::Integer(9007199254740992f64.to_bits()as i64);},
 13=>{let t=d.table_mut("puzzle3_document").unwrap();t.sql=t.sql.replacen("TEXT NOT NULL","TEXT \"NOT\" \"NULL\"",1);},
 14=>d.table_mut("puzzle3_meta").unwrap().rows.clear(),
 _=>d.table_mut("puzzle3_compatibility").unwrap().rows[0].values[7]=SqliteValue::Text("unknown".into()),
 }assert!(restore(&d).is_err(),"malformed case {n}");}}

#[test]
fn sqlite_snapshot_puzzle3d_distinct_duplicate_ids_and_every_optional_presence_survive(){
 let mut s=specimen(0);s.objects.push(s.objects[0].clone());s.objects[1].label=None;s.objects[1].object_kind=None;s.objects[1].anchor=Puzzle3dObjectAnchor::Fixed;s.objects[1].orientation=None;s.objects[1].scale=None;s.objects[1].mesh_url=None;s.objects[1].vortices[0].label=None;s.objects[1].vortices[0].vortex_kind=Some(String::new());s.objects[1].vortices[0].direction=None;s.objects[1].vortices[0].radius=None;
 s.references.push(s.references[0].clone());s.references[1].source.media_kind=None;s.target_volumes.push(s.target_volumes[0].clone());s.target_volumes[1].scale=None;s.target_volumes[1].orientation=None;
 let k=&mut s.meta.kind_catalogs.as_mut().unwrap().objects[0];k.representations.push(k.representations[0].clone());k.representations[1].lod=None;k.attributes.push(k.attributes[0].clone());k.attributes[1].definition=None;k.authors.push(k.authors[0].clone());k.authors[1].rank=None;k.authors[1].role=None;k.vortices.push(k.vortices[0].clone());k.vortices[1].vortex_kind=None;k.vortices[1].t=None;k.vortices[1].mandatory=None;k.vortices[1].radius=None;
 let catalog=s.meta.kind_catalogs.as_mut().unwrap();catalog.vortices.push(catalog.vortices[0].clone());catalog.vortices[1].code=None;catalog.vortices[1].label=None;catalog.vortices[1].order=None;
 let d=project(&s);assert_eq!(project(&restore(&d).unwrap()),d);s.meta.kind_catalogs=None;let d=project(&s);assert_eq!(project(&restore(&d).unwrap()),d);
}

#[test]
fn sqlite_snapshot_puzzle3d_actual_typed_and_play_erased_io_preserves_every_field(){
 let declared=super::super::native_codec();let typed=declared.snapshot_sqlite.expect("actual typed declaration");let typed_schema=declared.schema;
 let play=store::ArtifactCodec::bare::<Puzzle3dPlaySnapshot,Puzzle3dMutation>(PUZZLE_3D_SCHEMA);let play_schema=play.schema;let play=play.snapshot_sqlite.expect("actual Play declaration");let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.puzzle.puzzle3d".into(),standard:"1".into(),subset:"*".into()};
 for word in words(){let s=specimen(word);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&s)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&s))};for(provider,schema)in[(&typed,&typed_schema),(&play,&play_schema)]{let d=(provider.export)(schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(project(&restore(&d).unwrap()),project(&s));let actual=(provider.import)(schema,&dialect,d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(actual,payload);}}}
}

#[test]
fn sqlite_snapshot_puzzle3d_exact_borrowed_parsed_and_owned_rows_admit_before_copy(){let s=specimen(0);let d=project(&s);let count=d.tables.iter().map(|t|t.rows.len()).sum::<usize>();for max_rows in[count,count-1]{let limits=SqliteDatabaseLimits{max_rows,..SqliteDatabaseLimits::default()};let mut accept=|_|true;let mut c=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(s.to_sqlite_database(&mut c).is_ok(),max_rows==count);let mut c=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(Puzzle3dSnapshot::from_sqlite_database(&d,&mut c).is_ok(),max_rows==count);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut c=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(s.encode_sqlite_snapshot_native(encoding,&mut c).is_ok(),max_rows==count);let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&s)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&s))};let mut c=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(Puzzle3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut c).is_ok(),max_rows==count);}}
 let limits=SqliteDatabaseLimits{max_schema_bytes:32,..SqliteDatabaseLimits::default()};assert!(s.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(restore(&project(&Puzzle3dSnapshot::default())).is_ok());
}

#[test]
fn sqlite_snapshot_puzzle3d_all_four_copy_phases_have_real_interior_cancellation(){let mut s=specimen(0);s.meta.kind_catalogs.as_mut().unwrap().objects[0].description="日本😀".repeat(32768);let d=project(&s);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{let mut interior=false;let mut callback=|p:SqliteSnapshotProgress|{if p.phase==phase&&p.total>65536&&p.completed>=65536&&p.completed<p.total{interior=true;false}else{true}};let mut c=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let result=match phase{SqliteSnapshotPhase::ProjectSnapshot=>s.to_sqlite_database(&mut c).map(|_|()),SqliteSnapshotPhase::ReconstructSnapshot=>Puzzle3dSnapshot::from_sqlite_database(&d,&mut c).map(|_|()),SqliteSnapshotPhase::EncodeNative=>s.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut c).map(|_|()),_=>Puzzle3dSnapshot::decode_sqlite_snapshot_native(&store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&s)),&mut c).map(|_|())};assert!(result.is_err());assert!(interior,"phase {phase:?}");}}
