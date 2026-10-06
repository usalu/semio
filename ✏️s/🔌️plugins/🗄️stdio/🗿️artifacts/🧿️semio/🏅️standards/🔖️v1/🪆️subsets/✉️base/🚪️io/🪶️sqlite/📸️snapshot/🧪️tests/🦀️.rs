use crate::base::io::sqlite::snapshot::*;
use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::*};
fn subsets()->Vec<SemioSubsetSnapshot>{vec![SemioSubsetSnapshot::Brep(SemioBrepSnapshot::default()),SemioSubsetSnapshot::Mesh(SemioMeshSnapshot::default()),SemioSubsetSnapshot::Model(SemioModelSnapshot::default()),SemioSubsetSnapshot::Value(SemioValueSnapshot::default()),SemioSubsetSnapshot::Document(SemioDocumentSnapshot::default()),SemioSubsetSnapshot::Cad(SemioCadSnapshot::default()),SemioSubsetSnapshot::Drawing(SemioDrawingSnapshot::default()),SemioSubsetSnapshot::Image(SemioImageSnapshot::default()),SemioSubsetSnapshot::Video(SemioVideoSnapshot::default()),SemioSubsetSnapshot::Audio(SemioAudioSnapshot::default()),SemioSubsetSnapshot::Animation(SemioAnimationSnapshot::default()),SemioSubsetSnapshot::Presentation(SemioPresentationSnapshot::default()),SemioSubsetSnapshot::Flow(SemioFlowSnapshot::default()),SemioSubsetSnapshot::Text(SemioTextSnapshot::default()),SemioSubsetSnapshot::Table(SemioTableSnapshot::default()),SemioSubsetSnapshot::Graph(SemioGraphSnapshot::default()),SemioSubsetSnapshot::Object(SemioObjectSnapshot::default()),SemioSubsetSnapshot::Kit(SemioKitSnapshot::default())]}
fn project(s:&SemioSnapshot)->SqliteDatabase{s.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(db:&SqliteDatabase)->Result<SemioSnapshot,semio_framework_value::ValueError>{SemioSnapshot::from_sqlite_database(db,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()))}
#[test]
fn sqlite_snapshot_semio_base_all_eighteen_explicit_union_branches_and_native_state(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for(i,subset)in subsets().into_iter().enumerate(){let s=SemioSnapshot{schema:f["schema"].as_str().unwrap().into(),subset};let db=project(&s);assert_eq!(db.table("semio_base_document").unwrap().single_row().unwrap().text(2).unwrap(),f["subsets"][i].as_str().unwrap());let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let r=restore(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();assert_eq!(r,s);assert_eq!(<SemioSnapshot as store::ArtifactPack>::encode_pack(&r),<SemioSnapshot as store::ArtifactPack>::encode_pack(&s));assert_eq!(<SemioSnapshot as store::ArtifactDsl>::print_dsl(&r),<SemioSnapshot as store::ArtifactDsl>::print_dsl(&s));}}
#[test]
fn sqlite_snapshot_semio_base_rejects_second_selected_subset_wrong_discriminator_and_rows(){let original=project(&SemioSnapshot::default());for change in 0..3{let mut db=original.clone();match change{0=>db.table_mut("semio_base_document").unwrap().rows[0].values[4]=SqliteValue::Integer(1),1=>db.table_mut("semio_base_document").unwrap().rows[0].values[2]=SqliteValue::Text("unknown".into()),_=>db.table_mut("semio_mesh_document").unwrap().rows.push(SqliteRow{rowid:1,values:vec![SqliteValue::Integer(1),SqliteValue::Text("unexpected".into())]})}assert!(restore(&db).is_err());}assert!(SemioSnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());}
#[test]
fn sqlite_snapshot_semio_base_independent_schema_and_selected_typed_row_edit(){use std::io::Write;use std::process::{Command,Stdio};let s=SemioSnapshot{schema:"stdio.semio".into(),subset:SemioSubsetSnapshot::Text(SemioTextSnapshot::default())};let bytes=export_sqlite_database(&project(&s),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(db.query('SELECT subset,text_document_id FROM semio_base_document').get().subset!=='text')throw Error('typed union');db.query(\"UPDATE semio_text_document SET schema='independently edited'\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let r=restore(&import_sqlite_database(&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();if let SemioSubsetSnapshot::Text(t)=r.subset{assert_eq!(t.schema,"independently edited");}else{panic!("wrong union branch");}}

#[test]
fn sqlite_snapshot_semio_base_bare_owner_capability_native_payload_bridge(){let snapshot=SemioSnapshot::default();let payload=store::os_io::IoPayload::Binary(<SemioSnapshot as store::ArtifactPack>::encode_pack(&snapshot));let codec=store::ArtifactCodec::bare::<SemioSnapshot,crate::standards::v1::subsets::base::schema::mutations::SemioMutation>("s.stdio.semio.base");let provider=codec.snapshot_sqlite.expect("owner must publish typed SQLite capability");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<SemioSnapshot>()));let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"*".into()};let db=(provider.export)("s.stdio.semio.base",&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let db=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let actual=(provider.import)("s.stdio.semio.base",&dialect,db,SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(actual,payload);}

#[test]
fn sqlite_snapshot_semio_base_typed_exact_subset_dialect_and_document_guard(){let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let snapshot=SemioSnapshot::default();let database=project(&snapshot);let dialect=|v:&serde_json::Value|store::os_io::ArtifactDialect{artifact_kind:v["artifactKind"].as_str().unwrap().into(),standard:v["standard"].as_str().unwrap().into(),subset:v["subset"].as_str().unwrap().into()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(&f["sqliteDialect"]),&database,&mut control).unwrap().diagnostics.is_empty());for invalid in f["invalidSqliteDialects"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(invalid),&database,&mut control).is_err());}let mut malformed=database;malformed.table_mut("semio_base_document").unwrap().rows[0].values[1]=SqliteValue::Text("other schema".into());assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(&f["sqliteDialect"]),&malformed,&mut control).is_err());}

#[test]
fn sqlite_snapshot_semio_base_all_owned_native_preflight_branches() {
    let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"*".into()};let codec=<SemioSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    for subset in subsets(){let snapshot=SemioSnapshot{schema:"stdio.semio".into(),subset};for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let db=project(&snapshot);let actual=(codec.import)("Semio union admission",&dialect,db,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let expected=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<SemioSnapshot as store::ArtifactPack>::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(<SemioSnapshot as store::ArtifactDsl>::print_dsl(&snapshot))};assert_eq!(actual,expected);assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:16,..SqliteDatabaseLimits::default()})).is_err());}}
}

#[test]
fn sqlite_snapshot_semio_base_all_eighteen_controlled_native_constructors(){
 use store::{ArtifactDsl,ArtifactPack};let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for subset in subsets(){let expected=SemioSnapshot{schema:fixture["schema"].as_str().unwrap().into(),subset};for payload in[store::os_io::IoPayload::Binary(expected.encode_pack()),store::os_io::IoPayload::Text(expected.print_dsl())]{let actual=SemioSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(actual,expected);assert!(SemioSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());for limits in[SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_value_bytes:1,..SqliteDatabaseLimits::default()}]{assert!(SemioSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err())}}
 }
}

#[test]
fn sqlite_snapshot_semio_base_late_reconstruction_cancellation_retires_owned_tree(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let limits=&corpus["reconstructionRetirement"];let depth=limits["depth"].as_u64().unwrap() as usize;let stack=limits["stackBytes"].as_u64().unwrap() as usize;let bytes=limits["schemaBytes"].as_u64().unwrap() as usize;let mut root=crate::standards::v1::subsets::value::schema::snapshot::SemioValue::Null;for _ in 0..depth{root=crate::standards::v1::subsets::value::schema::snapshot::SemioValue::List{items:vec![root]};}let snapshot=SemioSnapshot{schema:"x".repeat(bytes),subset:SemioSubsetSnapshot::Value(SemioValueSnapshot{root,..SemioValueSnapshot::default()})};crate::standards::v1::subsets::base::schema::snapshot::native_output_tests::assert_reconstruction(snapshot,bytes,stack);}

#[test]
fn sqlite_snapshot_semio_base_controlled_refusals_preserve_owner_categories(){
 use semio_framework_value::native_decoding::NativeDecodeControl;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let cases=&fixture["controlledRefusals"];
 let malformed=crate::base::io::sqlite::snapshot::native_decoding::record::<2>(cases["malformedRecord"]["source"].as_str().unwrap(),&mut NativeDecodeControl::new(usize::MAX,&mut |_|true)).unwrap_err();assert_eq!(malformed.kind.as_str(),cases["malformedRecord"]["kind"].as_str().unwrap());
 let canceled=crate::base::io::sqlite::snapshot::native_decoding::float(cases["cancellation"]["source"].as_str().unwrap(),&mut NativeDecodeControl::new(usize::MAX,&mut |_|false)).unwrap_err();assert_eq!(canceled.kind.as_str(),cases["cancellation"]["kind"].as_str().unwrap());
 let limited=crate::base::io::sqlite::snapshot::native_decoding::entities(&mut 0,cases["rowAdmission"]["entities"].as_u64().unwrap()as usize,SqliteDatabaseLimits{max_rows:cases["rowAdmission"]["maxRows"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()}).unwrap_err();assert_eq!(limited.kind.as_str(),cases["rowAdmission"]["kind"].as_str().unwrap());
 let mut maximum=usize::MAX;let overflow=crate::base::io::sqlite::snapshot::native_decoding::entities(&mut maximum,1,SqliteDatabaseLimits::default()).unwrap_err();assert_eq!(overflow.kind.as_str(),cases["arithmeticOverflow"]["kind"].as_str().unwrap());
 let mut value=String::from("Z");for _ in 0..cases["nestedValue"]["depth"].as_u64().unwrap(){value=format!("L[{value}]")}
 let deep=crate::standards::v1::subsets::value::io::sqlite::snapshot::native_decoding::value_text(&value,&mut NativeDecodeControl::new(usize::MAX,&mut |_|true),SqliteDatabaseLimits::default(),&mut 0).err().expect("depth refusal");assert_eq!(deep.kind.as_str(),cases["nestedValue"]["kind"].as_str().unwrap());
 let snapshot=SemioSnapshot::default();
 let projection_canceled=snapshot.project_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).unwrap_err();assert_eq!(projection_canceled.kind.as_str(),cases["cancellation"]["kind"].as_str().unwrap());
 let projection_limited=snapshot.project_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:cases["projectionRowAdmission"]["maxRows"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()})).unwrap_err();assert_eq!(projection_limited.kind.as_str(),cases["projectionRowAdmission"]["kind"].as_str().unwrap());
 let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut bound=crate::base::io::sqlite::snapshot::sqlite::native::Bound::new("",&mut control).unwrap();let bound_overflow=bound.entities(usize::MAX).unwrap_err();assert_eq!(bound_overflow.kind.as_str(),cases["arithmeticOverflow"]["kind"].as_str().unwrap());
 let mesh=crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot::default();let database=mesh.project_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let reconstruct_canceled=crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot::reconstruct_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default()),<crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA).err().expect("typed reconstruction cancellation");assert_eq!(reconstruct_canceled.kind.as_str(),cases["cancellation"]["kind"].as_str().unwrap());
 eprintln!("[DEBUG] Semio typed SQLite union projection canceled={} rows={} bound={} reconstruction={}",projection_canceled.kind.as_str(),projection_limited.kind.as_str(),bound_overflow.kind.as_str(),reconstruct_canceled.kind.as_str());
 eprintln!("[DEBUG] Semio native controlled refusal categories malformed={} canceled={} rows={} arithmetic={} depth={}",malformed.kind.as_str(),canceled.kind.as_str(),limited.kind.as_str(),overflow.kind.as_str(),deep.kind.as_str());
}


#[test]
fn sqlite_snapshot_semio_base_native_backing_is_cumulative_for_every_branch() {
 use store::{ArtifactDsl,ArtifactPack};
 use semio_framework_value::ValueRefusalKind;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let demand=&fixture["nativeAdmission"];
 for subset in subsets() {
  let snapshot=SemioSnapshot{schema:demand["schema"].as_str().unwrap().into(),subset};
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
   let expected=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(snapshot.encode_pack()),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(snapshot.print_dsl())};
   let length=match &expected{store::os_io::IoPayload::Binary(bytes)=>bytes.len(),store::os_io::IoPayload::Text(text)=>text.len()};
   let mut accept=|_|true;
   let mut output=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:length,..SqliteDatabaseLimits::default()});
   assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut output).unwrap(),expected);
   assert_eq!(output.allocation_remaining_bytes(),0,"actual output backing must settle in the caller ledger");
   let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut output).unwrap_err();
   assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);
   assert_eq!(output.allocation_remaining_bytes(),0,"retiring output never refunds cumulative admission");
   let mut accept=|_|true;let mut input=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits::default());
   let before=input.allocation_remaining_bytes();
   let actual=SemioSnapshot::decode_sqlite_snapshot_native(&expected,&mut input).unwrap();
   assert_eq!(actual,snapshot);actual.retire_sqlite_snapshot();
   let owned=before-input.allocation_remaining_bytes();assert!(owned>0,"actual native fields must settle in the caller ledger");
   let mut accept=|_|true;
   let mut exact=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:owned.checked_mul(demand["repeatedOperations"].as_u64().unwrap() as usize).unwrap(),..SqliteDatabaseLimits::default()});
   for _ in 0..demand["repeatedOperations"].as_u64().unwrap(){let actual=SemioSnapshot::decode_sqlite_snapshot_native(&expected,&mut exact).unwrap();assert_eq!(actual,snapshot);actual.retire_sqlite_snapshot();}
   assert_eq!(exact.allocation_remaining_bytes(),0);
   let error=SemioSnapshot::decode_sqlite_snapshot_native(&expected,&mut exact).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(exact.allocation_remaining_bytes(),0);
   for encode in[true,false]{let mut accept=|_|true;let mut zero=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:0,..SqliteDatabaseLimits::default()});let error=if encode{snapshot.encode_sqlite_snapshot_native(encoding,&mut zero).unwrap_err()}else{SemioSnapshot::decode_sqlite_snapshot_native(&expected,&mut zero).unwrap_err()};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(zero.allocation_remaining_bytes(),0);}
   eprintln!("[DEBUG] Semio native branch={} encoding={} emitted={} admittedInput={} cumulativeOperations={}",crate::standards::v1::subsets::base::io::text::snapshot::subset_tag(&snapshot.subset),encoding.as_str(),length,owned,demand["repeatedOperations"]);
  }
 }
}

#[test]
fn sqlite_snapshot_semio_base_native_cancelled_backing_remains_admitted() {
 use store::{ArtifactDsl,ArtifactPack};
 use semio_framework_value::ValueRefusalKind;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let demand=&fixture["nativeAdmission"];
 let snapshot=SemioSnapshot{schema:demand["largeSchemaUnit"].as_str().unwrap().repeat(demand["largeSchemaRepeats"].as_u64().unwrap() as usize),..SemioSnapshot::default()};
 let schema_bytes=snapshot.schema.len();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text] {
  let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(snapshot.encode_pack()),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(snapshot.print_dsl())};
  let length=match &payload{store::os_io::IoPayload::Binary(bytes)=>bytes.len(),store::os_io::IoPayload::Text(text)=>text.len()};
  let mut reached=false;let mut cancel=|event:SqliteSnapshotProgress|{let stop=event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==length&&event.completed>=65536&&event.completed<event.total;reached|=stop;!stop};
  let mut output=SqliteSnapshotControl::new(&mut cancel,SqliteDatabaseLimits::default());let before=output.allocation_remaining_bytes();
  let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut output).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);
  assert_eq!(before-output.allocation_remaining_bytes(),length,"allocated output stays charged after interior cancellation");drop(output);assert!(reached);
  let mut reached=false;let mut validation_complete=false;let mut cancel=|event:SqliteSnapshotProgress|{let schema_stage=event.phase==SqliteSnapshotPhase::DecodeNative&&event.total==schema_bytes;if schema_stage&&event.completed==event.total{validation_complete=true;}let paid_stage=encoding==SnapshotEncoding::Text||validation_complete;let stop=schema_stage&&paid_stage&&event.completed>=65536&&event.completed<event.total;reached|=stop;!stop};
  let mut input=SqliteSnapshotControl::new(&mut cancel,SqliteDatabaseLimits::default());let before=input.allocation_remaining_bytes();
  let error=SemioSnapshot::decode_sqlite_snapshot_native(&payload,&mut input).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);
  let admitted=before-input.allocation_remaining_bytes();assert!(admitted>=schema_bytes,"allocated owner fields stay charged after cancellation");drop(input);assert!(reached);if encoding==SnapshotEncoding::Binary{assert!(validation_complete,"actual borrowed UTF8 validation must finish before paid copy cancellation");}
  eprintln!("[DEBUG] Semio interior native cancellation encoding={} outputBacking={} inputBacking={}",encoding.as_str(),length,admitted);
 }
}

#[test]
fn sqlite_snapshot_semio_base_native_refused_backing_remains_admitted() {
 use semio_framework_value::{ValueError,ValueRefusalKind};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let demand=&fixture["nativeAdmission"];
 let text=demand["schema"].as_str().unwrap();let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:text.len(),..SqliteDatabaseLimits::default()});
 let payload=store::os_io::IoPayload::Text("semio stdio.semio.dsl v1\n".into());
 let error=crate::base::io::sqlite::snapshot::native_decoding::decode::<String>(&payload,crate::base::io::sqlite::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA,&mut control,|_,native,_|{let _owned=native.copy_text(text)?;Err(ValueError::new(ValueRefusalKind::InvalidValue,"authored refusal after field allocation"))},|_,native,_|{let _owned=native.copy_text(text)?;Err(ValueError::new(ValueRefusalKind::InvalidValue,"authored refusal after field allocation"))}).unwrap_err();
 assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(error.message,"authored refusal after field allocation");
 assert_eq!(control.allocation_remaining_bytes(),0,"a failed native construction settles its actual admitted field backing");
 eprintln!("[DEBUG] Semio native typed refusal settled backing={} kind={}",text.len(),error.kind.as_str());
}

