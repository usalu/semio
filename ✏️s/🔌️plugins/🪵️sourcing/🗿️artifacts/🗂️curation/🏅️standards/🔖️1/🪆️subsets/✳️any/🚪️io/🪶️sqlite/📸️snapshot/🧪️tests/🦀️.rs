use crate::{CurationSnapshot,SourcingMutation,SOURCING_CURATION_SCHEMA};

#[test]
fn sqlite_snapshot_curation_actual_io_declaration_exposes_owned_semantic_capability(){
    let codec=crate::standards::v1::subsets::any::io::io().native.codec;
    assert!(codec.snapshot_sqlite.is_some(),"the actual Curation native I/O declaration must expose SQLite");
    assert!(store::ArtifactCodec::bare::<CurationSnapshot,SourcingMutation>(SOURCING_CURATION_SCHEMA).snapshot_sqlite.is_some());
}

#[test]
fn sqlite_snapshot_curation_complete_neutral_recipes_reach_both_declared_native_directions(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let source:CurationSnapshot=semio_framework_pack_json::from_json_str(&fixture["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(source.stock_extra.len(),5);assert_eq!(source.curated[0].count,u32::MAX);
    let codec=crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("Curation semantic owner");
    let _=codec;
    let binary=<CurationSnapshot as store::ArtifactPack>::encode_pack_with(&source,&store::PackEncodeOptions::default()).unwrap();let restored=<CurationSnapshot as store::ArtifactPack>::decode_pack_with(&binary,&store::PackDecodeOptions::default()).unwrap();assert_eq!(restored,source);
    let text=<CurationSnapshot as store::ArtifactDsl>::print_dsl(&source);assert_eq!(<CurationSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),source);
}

#[test]
fn sqlite_snapshot_curation_mandatory_kit_child_admission_retains_literal_identity_strings(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for identity in fixture["literalChildIds"].as_array().unwrap(){let mut value=fixture["snapshot"].clone();value["catalog"]["childId"]=identity.clone();value["catalog"]["target"]["artifactId"]=identity.clone();let expected:CurationSnapshot=semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Kit catalog identity is a literal persisted reference");expected.validate().unwrap();for text in[false,true]{let actual=if text{<CurationSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&expected)).unwrap()}else{<CurationSnapshot as store::ArtifactPack>::decode_pack_with(&store::ArtifactPack::encode_pack_with(&expected,&store::PackEncodeOptions::default()).unwrap(),&store::PackDecodeOptions::default()).unwrap()};assert_eq!(actual,expected);}}
}

fn dimensions(snapshot:&CurationSnapshot)->Vec<u64>{snapshot.stock_extra.iter().flat_map(|extra|match extra.geometry.as_ref(){crate::GeometryRecipe::Box{width,height,depth}=>vec![width.to_bits(),height.to_bits(),depth.to_bits()],crate::GeometryRecipe::Frame{width,height,depth,profile}=>vec![width.to_bits(),height.to_bits(),depth.to_bits(),profile.to_bits()],crate::GeometryRecipe::Slab{width,depth,thickness}=>vec![width.to_bits(),depth.to_bits(),thickness.to_bits()],crate::GeometryRecipe::Glb{extent,..}=>vec![extent.to_bits()],crate::GeometryRecipe::Mesh{..}=>Vec::new()}).collect()}
fn native_roundtrip(snapshot:&CurationSnapshot,text:bool)->CurationSnapshot{if text{<CurationSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(snapshot)).unwrap()}else{<CurationSnapshot as store::ArtifactPack>::decode_pack_with(&store::ArtifactPack::encode_pack_with(snapshot,&store::PackEncodeOptions::default()).unwrap(),&store::PackDecodeOptions::default()).unwrap()}}

#[test]
fn sqlite_snapshot_curation_every_native_quantity_preserves_exact_binary64_words(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for hex in fixture["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected:CurationSnapshot=semio_framework_pack_json::from_json_str(&fixture["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let value=f64::from_bits(bits);for extra in &mut expected.stock_extra{match extra.geometry.as_mut(){crate::GeometryRecipe::Box{width,height,depth}=>{*width=value;*height=value;*depth=value},crate::GeometryRecipe::Frame{width,height,depth,profile}=>{*width=value;*height=value;*depth=value;*profile=value},crate::GeometryRecipe::Slab{width,depth,thickness}=>{*width=value;*depth=value;*thickness=value},crate::GeometryRecipe::Glb{extent,..}=>*extent=value,crate::GeometryRecipe::Mesh{..}=>{}}}for text in[false,true]{assert_eq!(dimensions(&native_roundtrip(&expected,text)),vec![bits;11],"{hex}: literal quantity Text={text}");}}
}

#[test]
fn sqlite_snapshot_curation_mesh_native_scalars_preserve_exact_binary32_words(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for hex in fixture["binary32Words"].as_array().unwrap(){let bits=u32::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected:CurationSnapshot=semio_framework_pack_json::from_json_str(&fixture["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let crate::GeometryRecipe::Mesh{positions,normals,..}=expected.stock_extra[3].geometry.as_mut()else{panic!("neutral Mesh")};positions.fill(f32::from_bits(bits));normals.fill(f32::from_bits(bits));for text in[false,true]{let actual=native_roundtrip(&expected,text);let crate::GeometryRecipe::Mesh{positions,normals,indices}=actual.stock_extra[3].geometry.as_ref()else{panic!("native Mesh")};assert_eq!(positions.iter().chain(normals).map(|value|value.to_bits()).collect::<Vec<_>>(),vec![bits;6],"{hex}: literal binary32 Text={text}");assert_eq!(indices,&vec![u32::MAX,0,0]);}}
}
#[test]
fn sqlite_snapshot_curation_local_kit_aliases_retain_distinct_literal_target_identities(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for pair in fixture["literalChildAliases"].as_array().unwrap(){let mut value=fixture["snapshot"].clone();value["catalog"]["childId"]=pair["childId"].clone();value["catalog"]["target"]["artifactId"]=pair["artifactId"].clone();let expected:CurationSnapshot=semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Kit aliases are independent literal fields");expected.validate().unwrap();for text in[false,true]{assert_eq!(native_roundtrip(&expected,text),expected);}}
}

use semio_framework_plugin::{PluginApp,VcsArtifactApp,EditorApp,ViewerApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 🗂️ The actual Curation editor and viewer declarations.
 enum SqliteApps:PluginApp{
  Editor(VcsArtifactApp<EditorApp<crate::editor::sourcing::SourcingCurationApp>,semio_s_artifact_stdio_semio::SemioMembers>),
  Viewer(VcsArtifactApp<ViewerApp<crate::viewer::sourcing::SourcingViewer>,semio_s_artifact_stdio_semio::SemioMembers>),
 }
}

use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
fn laws()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn fixture()->CurationSnapshot{semio_framework_pack_json::from_json_str(&laws()["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn database(snapshot:&CurationSnapshot)->SqliteDatabase{snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(database:&SqliteDatabase)->CurationSnapshot{CurationSnapshot::from_sqlite_database(database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn file(snapshot:&CurationSnapshot)->Vec<u8>{export_sqlite_database(&database(snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()}
fn native(snapshot:&CurationSnapshot,encoding:SnapshotEncoding)->store::io_schema::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(snapshot))}}
fn dialect()->store::io_schema::ArtifactDialect{store::io_schema::ArtifactDialect{artifact_kind:"s.sourcing.curation".into(),standard:"1".into(),subset:"*".into()}}
fn mesh_words(snapshot:&CurationSnapshot)->Vec<u32>{let crate::GeometryRecipe::Mesh{positions,normals,..}=snapshot.stock_extra[3].geometry.as_ref()else{panic!("neutral Mesh")};positions.iter().chain(normals).map(|value|value.to_bits()).collect()}
fn set_dimensions(snapshot:&mut CurationSnapshot,value:f64){for extra in &mut snapshot.stock_extra{match extra.geometry.as_mut(){crate::GeometryRecipe::Box{width,height,depth}=>{*width=value;*height=value;*depth=value},crate::GeometryRecipe::Frame{width,height,depth,profile}=>{*width=value;*height=value;*depth=value;*profile=value},crate::GeometryRecipe::Slab{width,depth,thickness}=>{*width=value;*depth=value;*thickness=value},crate::GeometryRecipe::Glb{extent,..}=>*extent=value,crate::GeometryRecipe::Mesh{..}=>{}}}}
fn erased_roundtrip(snapshot:&CurationSnapshot,encoding:SnapshotEncoding)->CurationSnapshot{
 let codec=crate::standards::v1::subsets::any::io::io().native.codec;let provider=codec.snapshot_sqlite.unwrap();let d=(provider.export)(&codec.schema,&dialect(),&native(snapshot,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let bytes=export_sqlite_database(&d,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let d=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let payload=(provider.import)(&codec.schema,&dialect(),d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;CurationSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()
}

#[test]
fn sqlite_snapshot_curation_queryable_complete_recipes_and_empty_collection_presence(){
 let expected=fixture();let d=database(&expected);assert_eq!(d.tables.len(),14);assert_eq!(d.tables.iter().map(|table|table.rows.len()).sum::<usize>(),34);assert_eq!(restore(&import_sqlite_database(&file(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
 let mut empty=expected.clone();empty.stock_extra.clear();empty.curated.clear();let d=database(&empty);assert_eq!(d.tables.iter().map(|table|table.rows.len()).sum::<usize>(),2);assert_eq!(restore(&d),empty);
}

#[test]
fn sqlite_snapshot_curation_all_quantity_words_survive_both_erased_native_directions(){
 for hex in laws()["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=fixture();set_dimensions(&mut expected,f64::from_bits(bits));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let actual=erased_roundtrip(&expected,encoding);assert_eq!(dimensions(&actual),vec![bits;11],"{hex}: {encoding:?}");assert_eq!(actual.catalog,expected.catalog);assert_eq!(actual.curated,expected.curated);}}
}

#[test]
fn sqlite_snapshot_curation_all_mesh_words_survive_both_erased_native_directions(){
 for hex in laws()["binary32Words"].as_array().unwrap(){let bits=u32::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=fixture();let crate::GeometryRecipe::Mesh{positions,normals,..}=expected.stock_extra[3].geometry.as_mut()else{panic!("neutral Mesh")};positions.fill(f32::from_bits(bits));normals.fill(f32::from_bits(bits));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let actual=erased_roundtrip(&expected,encoding);assert_eq!(mesh_words(&actual),vec![bits;6],"{hex}: {encoding:?}");let crate::GeometryRecipe::Mesh{indices,..}=actual.stock_extra[3].geometry.as_ref()else{panic!("restored Mesh")};assert_eq!(indices,&vec![u32::MAX,0,0]);}}
}

#[test]
fn sqlite_snapshot_curation_sqlite_preserves_every_literal_kit_child_identity(){
 for identity in laws()["literalChildIds"].as_array().unwrap(){let mut expected=fixture();let id=identity.as_str().unwrap();expected.catalog.child_id=id.into();expected.catalog.target.artifact_id=id.into();assert_eq!(restore(&database(&expected)),expected);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{assert_eq!(erased_roundtrip(&expected,encoding),expected);}}
}

#[test]
fn sqlite_snapshot_curation_independent_queries_and_complete_surrogate_edits_retain_semantics(){
 use std::{io::Write,process::{Command,Stdio}};let mut expected=fixture();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const stock=db.query('SELECT object_id,availability FROM curation_stock_extra ORDER BY ordinal').all();if(stock.length!==5||stock[0].object_id!==''||stock[0].availability!==4294967295n)throw Error('complete stock');const paths=db.query('SELECT segment FROM curation_typology_segment WHERE stock_extra_id=1 ORDER BY ordinal').all();if(JSON.stringify(paths.map(x=>x.segment))!=='["","A","A"]')throw Error('ordered duplicates');const positions=db.query('SELECT value_bits,value_class FROM curation_mesh_position ORDER BY ordinal').all();const word=Buffer.alloc(4);word.writeFloatBE(-2);if(positions.length!==4||positions[2].value_bits!==BigInt(word.readUInt32BE())||positions[2].value_class!=='finite')throw Error('binary32 query');if(db.query('SELECT value FROM curation_mesh_index WHERE ordinal=0').get().value!==4294967295n)throw Error('unresolved native index');db.run('PRAGMA foreign_keys=OFF');db.run('UPDATE curation_catalog SET id=id+99,child_id=?,artifact_id=?',['edited 世界','edited 世界']);db.run('UPDATE curation_stock_extra SET id=id+99,name=?',['edited name']);db.run('UPDATE curation_geometry SET id=id+99,stock_extra_id=stock_extra_id+99');db.run('UPDATE curation_typology_segment SET id=id+99,stock_extra_id=stock_extra_id+99');for(const table of['box','frame','slab','mesh','glb'])db.run('UPDATE curation_'+table+' SET id=id+99,geometry_id=geometry_id+99');for(const table of['position','normal','index'])db.run('UPDATE curation_mesh_'+table+' SET id=id+99,mesh_id=mesh_id+99');db.run('UPDATE curation_curated SET id=id+99,object_id=?',['edited selection']);if(db.query('PRAGMA foreign_key_check').all().length)throw Error('renumbered relationships');await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file(&expected)).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));expected.catalog.child_id="edited 世界".into();expected.catalog.target.artifact_id="edited 世界".into();for extra in &mut expected.stock_extra{extra.name="edited name".into()}for item in &mut expected.curated{item.object_id="edited selection".into()}assert_eq!(restore(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}

#[test]
fn sqlite_snapshot_curation_independent_malformed_sql_refuses_owned_shapes(){
 use std::{io::Write,process::{Command,Stdio}};let bytes=file(&fixture());let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(new Uint8Array(input.bytes));db.run('PRAGMA ignore_check_constraints=ON');db.run(input.sql);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 for sql in laws()["malformedSql"].as_array().unwrap(){let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"bytes":bytes,"sql":sql}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{sql}: {}",String::from_utf8_lossy(&output.stderr));assert!(import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).and_then(|d|CurationSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))).is_err(),"{sql}");}
}

#[test]
fn sqlite_snapshot_curation_exact_rows_and_native_owned_file_limits_precede_construction(){
 let snapshot=fixture();let d=database(&snapshot);let limits=SqliteDatabaseLimits::default();assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:34,..limits})).is_ok());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:33,..limits})).is_err());assert!(CurationSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for restricted in[SqliteDatabaseLimits{max_rows:33,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits},SqliteDatabaseLimits{max_file_bytes:1,..limits}]{assert!(CurationSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());}assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..limits})).is_err());}
}

#[test]
fn sqlite_snapshot_curation_initial_and_interior_text_cancellation_reaches_owning_phases(){
 let mut snapshot=fixture();let limits=SqliteDatabaseLimits::default();let d=database(&snapshot);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{let mut callback=|_|false;let mut control=SqliteSnapshotControl::new(&mut callback,limits);let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>CurationSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::EncodeNative=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err(),_=>CurationSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,SnapshotEncoding::Text),&mut control).is_err()};assert!(rejected,"{phase:?}");}
 snapshot.stock_extra[0].name="😀".repeat(laws()["control"]["largeTextBytes"].as_u64().unwrap()as usize);let d=database(&snapshot);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative,SqliteSnapshotPhase::DecodeNative]{let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=65536&&(event.total==0||event.total>event.completed){reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,limits);let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>CurationSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::EncodeNative=>snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err(),_=>CurationSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,SnapshotEncoding::Text),&mut control).is_err()};assert!(rejected,"{phase:?}");assert!(reached,"{phase:?}");}
}

#[test]
fn sqlite_snapshot_curation_cancel_reaches_known_stock_and_ordinal_frontiers(){
 let mut snapshot=fixture();snapshot.stock_extra=vec![snapshot.stock_extra[0].clone();laws()["control"]["collectionLength"].as_u64().unwrap()as usize];let d=database(&snapshot);let limits=SqliteDatabaseLimits::default();
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative]{let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.total==2048&&event.completed>=256&&event.completed<event.total{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,limits);let rejected=match phase{SqliteSnapshotPhase::ProjectSnapshot=>snapshot.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>CurationSnapshot::from_sqlite_database(&d,&mut control).is_err(),_=>CurationSnapshot::decode_sqlite_snapshot_native(&native(&snapshot,SnapshotEncoding::Binary),&mut control).is_err()};assert!(rejected,"{phase:?}");assert!(reached,"{phase:?}");}
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_curation_actual_native_io_declaration_routes_queryable_files(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧩️declaration/🔣️.json")).unwrap();
 let declaration=crate::artifact::<SqliteApps>();
 assert_eq!(declaration.kind.as_str(),law["dialect"]["artifactKind"].as_str().unwrap());assert_eq!(declaration.standards.len(),1);assert_eq!(declaration.standards[0].id.0,law["dialect"]["standard"].as_str().unwrap());assert_eq!(declaration.standards[0].subsets.len(),1);
 let subset=&declaration.standards[0].subsets[0];assert_eq!(subset.dialect.artifact_kind,law["dialect"]["artifactKind"].as_str().unwrap());assert_eq!(subset.dialect.standard.0,law["dialect"]["standard"].as_str().unwrap());assert_eq!(subset.dialect.subset.0,law["dialect"]["subset"].as_str().unwrap());
 let codec=subset.io.native.codec.clone();assert_eq!(codec.schema,law["schema"].as_str().unwrap());let declared=codec.snapshot_sqlite.clone().expect("actual declared Snapshot semantic provider");assert_eq!(declared.snapshot_type,Some(std::any::TypeId::of::<CurationSnapshot>()));assert!(!CurationSnapshot::SQLITE_SCHEMA.trim().is_empty());assert_eq!(declared.schema.as_ref(),CurationSnapshot::SQLITE_SCHEMA);let typed=store::ArtifactCodec::bare::<CurationSnapshot,SourcingMutation>(law["schema"].as_str().unwrap());assert!(declared.identical_to(typed.snapshot_sqlite.as_ref().unwrap()));
 use store::io::io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot};let plugin=semio_framework_plugin::Plugin::<SqliteApps>::builder("sourcing").label("Curation owned SQLite").version("0.0.1").package_id("semio:sourcing").declare_artifact(declaration).try_build().unwrap();
 let installed=store::document_codec(&codec.schema).await.unwrap().expect("actual owning builder installs document codec");assert_eq!(installed.schema,codec.schema);assert!(declared.identical_to(installed.snapshot_sqlite.as_ref().unwrap()));let native=store::io_schema::ArtifactDialect{artifact_kind:law["dialect"]["artifactKind"].as_str().unwrap().into(),standard:law["dialect"]["standard"].as_str().unwrap().into(),subset:law["dialect"]["subset"].as_str().unwrap().into()};assert_eq!(store::io::io_mechanism::native_snapshot_sqlite_schema(&native).unwrap(),CurationSnapshot::SQLITE_SCHEMA);let sqlite=store::io_schema::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);for(from,into)in[(&native,&sqlite),(&sqlite,&native)]{let route=store::io::io_mechanism::io_route(from,into,1).await.unwrap().value;assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,store::io_schema::IoFidelity::Exact);}
 eprintln!("[DEBUG] Curation actual generic declaration Snapshot={} schema={} TypeId={:?} sql_bytes={} installed_same_hooks=true direct_exact_directions=2",law["snapshot"].as_str().unwrap(),codec.schema,declared.snapshot_type,declared.schema.len());
assert!(store::document_codec(SOURCING_CURATION_SCHEMA).await.unwrap().unwrap().snapshot_sqlite.is_some());let expected=public_owner::Owned::new(fixture());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=io_export_sqlite_snapshot(&dialect(),&*expected,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));let restored=public_owner::Owned::new(io_import_sqlite_snapshot::<CurationSnapshot>(&dialect(),&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);assert_eq!(*restored,*expected);let edited=public_owner::verify_and_edit(&bytes,encoding);let restored=public_owner::Owned::new(io_import_sqlite_snapshot::<CurationSnapshot>(&dialect(),&edited,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);let mut literal=public_owner::Owned::new(fixture());let public=public_owner::corpus();let value=public["independentEdit"]["value"].as_str().unwrap();literal.stock_extra[0].name=value.into();;assert_eq!(*restored,*literal);eprintln!("[DEBUG] curation actual public six-field metadata / entity ownership / independent SQL edit / explicit retirement encoding={encoding:?}");}
 drop(plugin);
}

#[test]
fn sqlite_snapshot_curation_distinct_logical_and_addressed_children_have_full_physical_integrity(){
 use std::{io::Write,process::{Command,Stdio}};
 let input:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪆️distinct/🔣️.json")).unwrap();
 let mut source=laws()["snapshot"].clone();source["catalog"]["childId"]=input["childId"].clone();source["catalog"]["target"]["artifactId"]=input["artifactId"].clone();source["catalog"]["target"]["dialect"]=input["dialect"].clone();
 let expected:CurationSnapshot=semio_framework_pack_json::from_json_str(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();expected.validate().unwrap();assert_ne!(expected.catalog.child_id,expected.catalog.target.artifact_id);
 let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(new Uint8Array(input.bytes));try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT child_id,artifact_id,artifact_kind,standard,subset FROM curation_catalog').all(),i=input.identity;if(JSON.stringify(rows)!==JSON.stringify([{child_id:i.childId,artifact_id:i.artifactId,artifact_kind:i.dialect.artifactKind,standard:i.dialect.standard,subset:i.dialect.subset}]))throw Error('full child ownership');await Bun.write(Bun.stdout,db.serialize());}finally{db.close();}"#;
 
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"identity":input,"bytes":file(&expected)}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(restore(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(CurationSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);}
}

#[test]
fn sqlite_snapshot_curation_full_semantic_cell_limit_is_exact_for_all_ieee_branches() {
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️semantic-cells/🔣️.json")).unwrap();assert_eq!(law["cases"].as_array().unwrap().len(),19);
 for case in law["cases"].as_array().unwrap(){let mut expected=fixture();let width=case["width"].as_u64().unwrap();if width!=0{let word=u64::from_str_radix(case["hex"].as_str().unwrap(),16).unwrap();for extra in &mut expected.stock_extra{match extra.geometry.as_mut(){crate::GeometryRecipe::Box{width:w,height,depth} if width==64=>{*w=f64::from_bits(word);*height=*w;*depth=*w},crate::GeometryRecipe::Frame{width:w,height,depth,profile} if width==64=>{*w=f64::from_bits(word);*height=*w;*depth=*w;*profile=*w},crate::GeometryRecipe::Slab{width:w,depth,thickness} if width==64=>{*w=f64::from_bits(word);*depth=*w;*thickness=*w},crate::GeometryRecipe::Glb{extent,..} if width==64=>*extent=f64::from_bits(word),crate::GeometryRecipe::Mesh{positions,normals,..} if width==32=>{positions.fill(f32::from_bits(word as u32));normals.fill(f32::from_bits(word as u32))},_=>{}}}}
 let maximum=usize::try_from(case["bytes"].as_u64().unwrap()).unwrap();let limits=SqliteDatabaseLimits{max_value_bytes:maximum,..SqliteDatabaseLimits::default()};let d=database(&expected);assert_eq!(d.tables.len(),14);assert_eq!(d.tables.iter().map(|table|table.rows.len()).sum::<usize>(),34);
 assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok());assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:maximum-1,..limits})).is_err());assert!(CurationSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok());assert!(CurationSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:maximum-1,..limits})).is_err());
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=native(&expected,encoding);assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok());assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:maximum-1,..limits})).is_err());assert!(CurationSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok());assert!(CurationSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:maximum-1,..limits})).is_err());}
 println!("[DEBUG] Curation complete Native semantic cell boundary case={} bytes={} all14tables=true original_backing_authority_separate=true",case["id"],maximum);
 }
}

#[path="\u{1f4b0}\u{FE0F}backing/\u{1f980}\u{FE0F}.rs"]mod allocation;
#[test]
fn sqlite_snapshot_curation_reconstruction_pays_actual_system_requests_cumulatively(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️semantic-cells/🔣️.json")).unwrap();let backing=&law["nativeBacking"];assert_eq!(backing["requestIncludesReallocFullSize"],true);let maximum=usize::try_from(backing["maximumBytes"].as_u64().unwrap()).unwrap();let expected=fixture();let database=database(&expected);let defaults=SqliteDatabaseLimits{max_allocation_bytes:maximum,..SqliteDatabaseLimits::default()};let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,defaults);
 let(result,requested)=allocation::observe(||CurationSnapshot::from_sqlite_database(&database,&mut control));assert_eq!(result.unwrap(),expected);let admitted=maximum-control.allocation_remaining_bytes();println!("[DEBUG] Curation actual system backing requests={} caller_admitted={} semantic_cells_separate=true",requested,admitted);assert!(admitted>0,"real Curation indexes and typed allocations must settle in the caller ledger");assert_eq!(admitted,requested,"every complete source/typed backing request must be admitted");
 for allowance in[admitted,admitted-1,0,1]{let limits=SqliteDatabaseLimits{max_allocation_bytes:allowance,..defaults};let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);let(result,requested)=allocation::observe(||CurationSnapshot::from_sqlite_database(&database,&mut control));if allowance==admitted{assert_eq!(result.unwrap(),expected);assert_eq!(requested,admitted);assert_eq!(control.allocation_remaining_bytes(),0);}else{assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert!(control.allocation_remaining_bytes()<=allowance);}}
 let repetitions=usize::try_from(backing["repeatedOwnerships"].as_u64().unwrap()).unwrap();assert_eq!(repetitions,2);let total=admitted.checked_mul(repetitions).unwrap();let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:total,..defaults});for _ in 0..repetitions{let(result,requested)=allocation::observe(||CurationSnapshot::from_sqlite_database(&database,&mut control));assert_eq!(result.unwrap(),expected);assert_eq!(requested,admitted);}assert_eq!(control.allocation_remaining_bytes(),0);assert_eq!(CurationSnapshot::from_sqlite_database(&database,&mut control).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
}

#[path="🚦️public/🦀️.rs"]
mod public_owner;
