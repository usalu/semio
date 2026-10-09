//! 🧪️ Original Socket carriers and installed IO preserve the complete raw Unicode corpus.
use super::{NativeSocketProbeSnapshot,store};
use store::{os_store::{ArtifactSqliteSnapshot,ArtifactDsl,ArtifactPack},sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits as Limits,SqliteSnapshotControl as Control,SqliteSnapshotPhase as Phase,SnapshotEncoding,SqliteValue},io_schema::IoPayload};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,NativeDecodeProgress,NativeEncodeProgress,ValueError,ValueRefusalKind as Kind};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText;
use semio_framework_async::block_on;

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn native_socket_sqlite_snapshot_catalog_declarations_and_failed_assembly_are_owned(){
 use store::io::{ArtifactCatalogBinding as Row,ArtifactCatalogCapability as Capability,ArtifactCatalogTarget as Target,ArtifactAssemblyRegistryPlan as Plan,ArtifactAssemblyRegistryError as Error,ArtifactCatalogBindingError as CatalogError,artifact_catalog_bindings,preflight_artifact_catalog_bindings_in_assembly,commit_artifact_assembly_registry_plan,ArtifactCodecBinding};
 use semio_framework_value::FromValue;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🚪️io/🪪️bindings/🏛️catalog/🧫️fixtures/🔣️.json")).unwrap();
 let source=&fixture["cases"][2]["row"];let target=&source["target"];
 let row=Row{contributor:store::os_directory::DocumentOpenPackageV1::from_value(source["contributor"].clone().into()).unwrap(),owner:store::os_directory::DocumentOpenPackageV1::from_value(source["owner"].clone().into()).unwrap(),artifact:store::os_directory::DocumentOpenArtifactV1::from_value(source["artifact"].clone().into()).unwrap(),target:Some(Target{parent_dialect:ArtifactDialect::parse_coordinate(target["parentDialect"].as_str().unwrap()).unwrap(),surface:store::os_directory::DocumentOpenSurfaceV1::from_value(target["surface"].clone().into()).unwrap(),grant:store::os_directory::DocumentOpenGrantV1::from_value(target["grant"].clone().into()).unwrap(),browser_actor:store::os_directory::schema::DocumentOpenBrowserActorV1::from_value(target["browserActor"].clone().into()).unwrap()}),capability:Capability::UnlinkedGuest};
 let mut other=row.clone();other.target.as_mut().unwrap().surface.window_kind_id=fixture["cases"][3]["row"]["target"]["surface"]["windowKindId"].as_str().unwrap().into();
 let assembly=semio_framework_schema_registry::assembly::begin().unwrap();assert!(preflight_artifact_catalog_bindings_in_assembly(&assembly,&[row.clone(),other.clone()],&[]).is_ok());assert!(matches!(preflight_artifact_catalog_bindings_in_assembly(&assembly,&[row.clone(),row.clone()],&[]),Err(CatalogError::Duplicate{..})));commit_artifact_assembly_registry_plan(&assembly,Plan{catalog_bindings:vec![row.clone(),other.clone()],..Default::default()}).unwrap();drop(assembly);
 let before=artifact_catalog_bindings().unwrap().into_iter().filter(|entry|entry.artifact.schema==row.artifact.schema).collect::<Vec<_>>();assert_eq!(before.len(),2);assert!(before.iter().all(|entry|matches!(entry.capability,Capability::UnlinkedGuest)));assert_eq!(before[0].owner.plugin_id,"owner");assert_eq!(before[0].contributor.plugin_id,"host");
 let mut changed=row.clone();changed.owner=changed.contributor.clone();let assembly=semio_framework_schema_registry::assembly::begin().unwrap();assert!(matches!(preflight_artifact_catalog_bindings_in_assembly(&assembly,&[changed],&[]),Err(CatalogError::Conflict{..})));drop(assembly);
 let mut invalid=row.clone();invalid.artifact.pack_schema_hash="0".repeat(64);
 let mut codec=NativeSocketProbeSnapshot::native_snapshot_registration().unwrap().1;codec.schema.push_str(".catalog-refused");let schema=codec.schema.clone();assert!(block_on(store::os_store::document_codec(&schema)).unwrap().is_none());let binding=ArtifactCodecBinding::schema_only(&codec);
 let assembly=semio_framework_schema_registry::assembly::begin().unwrap();assert!(matches!(commit_artifact_assembly_registry_plan(&assembly,Plan{catalog_bindings:vec![invalid],document_codecs:vec![codec],document_bindings:vec![binding],..Default::default()}),Err(Error::Catalog(CatalogError::Invalid{..}))));drop(assembly);
 assert!(block_on(store::os_store::document_codec(&schema)).unwrap().is_none());assert_eq!(artifact_catalog_bindings().unwrap().into_iter().filter(|entry|entry.artifact.schema==row.artifact.schema).collect::<Vec<_>>(),before);
 assert_eq!(serde_json::from_str::<serde_json::Value>(&serde_json::to_string(&fixture).unwrap()).unwrap(),fixture);
 eprintln!("[DEBUG] Original catalog Native neutral rows=2 distinctWindows=true unlinkedRetained=true rejectedAssemblyPublishesNothing=true independentSerde=true");
}
#[test]
fn native_socket_sqlite_snapshot_independent_native_identity_and_provider_admission() {
 use store::os_store::{ArtifactCodec,ArtifactNativeSnapshotIdentity as Identity};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪪️identity/🔣️.json")).unwrap();
 let codec=ArtifactCodec::bare::<NativeSocketProbeSnapshot,super::NativeSocketProbeMutation>(super::native_socket_sqlite::SCHEMA);
 let Identity::Typed{snapshot_type,owner}=codec.native_identity else{panic!("actual linked Socket owner")};
 assert_eq!(snapshot_type,std::any::TypeId::of::<NativeSocketProbeSnapshot>());
 assert_eq!(owner.rsplit("::").next().unwrap(),fixture["typedOwner"].as_str().unwrap());
 let provider=codec.snapshot_sqlite.as_ref().unwrap();
 assert!(codec.native_identity.validates_provider(&codec.schema,Some(provider)));
 assert!(codec.native_identity.validates_provider(&codec.schema,None));
 assert!(fixture["sqlCapabilityIndependent"].as_bool().unwrap());
 let mut absent=codec.clone();absent.snapshot_sqlite=None;
 assert_eq!(absent.native_identity,codec.native_identity);
 let mut wrong=codec.clone();wrong.native_identity=Identity::typed::<String>();
 assert!(!wrong.native_identity.validates_provider(&wrong.schema,wrong.snapshot_sqlite.as_ref()));
 assert!(block_on(store::os_store::preflight_document_codecs(&[wrong])).is_err());
 let mut other=absent.clone();other.native_identity=Identity::typed::<String>();
 assert!(block_on(store::os_store::preflight_document_codecs(&[absent,other])).is_err());
 eprintln!("[DEBUG] Socket genuine native identity survives absent SQL; wrong typed provider and same-schema owner collision refuse");
}
fn native_grant()->semio_framework_value::RetainedCloneGrant{serde_json::from_value(fixture()["callerGrant"].clone()).expect("original independent Socket caller grant")}

#[test]
fn native_socket_sqlite_snapshot_actual_installed_binding_and_duplicate_preflight(){
 use store::io::{ArtifactCodecBindingChannel as Channel,ArtifactCodecBindingError as Error,artifact_codec_bindings,preflight_artifact_codec_bindings};
 use store::os_store::ArtifactNativeSnapshotIdentity as Identity;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🚪️io/🪪️bindings/🧫️fixtures/🔣️.json")).unwrap();
 let(owner,codec)=NativeSocketProbeSnapshot::native_snapshot_registration().unwrap();let dialect=ArtifactDialect::from(owner);NativeSocketProbeSnapshot::publish_native_snapshot().unwrap();
 let rows=artifact_codec_bindings().unwrap();let matches:Vec<_>=rows.iter().filter(|row|row.channel==Channel::DirectNative&&row.dialect.as_ref()==Some(&dialect)).collect();assert_eq!(matches.len(),1);let row=matches[0];
 assert_eq!(row.artifact_kind.as_deref(),Some(owner.artifact_kind));assert_eq!(row.schema,codec.schema);assert_eq!(row.native_identity,codec.native_identity);assert_eq!(row.apps.len(),fixture["appLess"].as_array().unwrap().len());assert!(row.factory.is_none());assert_eq!(row.sqlite_schema,codec.snapshot_sqlite.as_ref().map(|provider|provider.schema.clone()));
 let Identity::Typed{owner:native_owner,..}=&row.native_identity else{panic!("actual Socket typed identity")};assert_eq!(row.contributor,*native_owner);
 assert!(preflight_artifact_codec_bindings(&[row.clone()],&[codec.clone()]).is_ok());
 assert!(matches!(preflight_artifact_codec_bindings(&[row.clone(),row.clone()],&[codec.clone()]),Err(Error::Duplicate{..})));assert_eq!(fixture["duplicateProposal"],"refuse");assert_eq!(fixture["identicalInstalled"],"idempotent");
 let mut invalid=row.clone();invalid.dialect=None;assert!(matches!(preflight_artifact_codec_bindings(&[invalid],&[codec.clone()]),Err(Error::Invalid{..})));
 let mut invalid=row.clone();invalid.apps=vec!["viewer".into(),"viewer".into()];assert!(matches!(preflight_artifact_codec_bindings(&[invalid],&[codec]),Err(Error::Invalid{..})));
 let mut different_schema=NativeSocketProbeSnapshot::native_snapshot_registration().unwrap().1;different_schema.schema.push_str("/refusal-specimen");assert!(store::io::io_mechanism::preflight_native_snapshots(&[store::io::io_mechanism::NativeSnapshotRegistration{dialect,codec:different_schema}]).is_err());
 assert_eq!(artifact_codec_bindings().unwrap(),rows);
 eprintln!("[DEBUG] original installed Socket binding exactOwner=true exactDialect=true appLess=true noGuessedFactory=true duplicatePreflight=true");
}
fn samples(f:&serde_json::Value)->Vec<(String,usize)>{let mut rows:Vec<_>=f["cases"].as_array().unwrap().iter().map(|row|(row["text"].as_str().unwrap().to_owned(),row["valueBytes"].as_u64().unwrap()as usize)).collect();rows.push((f["large"]["unit"].as_str().unwrap().repeat(f["large"]["repetitions"].as_u64().unwrap()as usize),f["large"]["valueBytes"].as_u64().unwrap()as usize));rows}
fn payload(text:&str,encoding:SnapshotEncoding)->IoPayload{match encoding{SnapshotEncoding::Text=>IoPayload::Text(text.into()),SnapshotEncoding::Binary=>IoPayload::Binary(text.as_bytes().to_vec())}}
fn decode(input:&IoPayload,control:&mut Control<'_>)->Result<NativeSocketProbeSnapshot,ValueError>{let mut cb=|_|true;NativeSocketProbeSnapshot::decode_sqlite_snapshot_native(input,control,&mut NativeDecodeControl::new(Limits::default().max_allocation_bytes,&mut cb))}
fn encode(snapshot:&NativeSocketProbeSnapshot,encoding:SnapshotEncoding,control:&mut Control<'_>)->Result<IoPayload,ValueError>{let mut cb=|_|true;snapshot.encode_sqlite_snapshot_native(encoding,control,&mut store::os_store::NativeSnapshotEncodeOwner::new(&mut NativeEncodeControl::new(Limits::default().max_allocation_bytes,&mut cb),native_grant()))}
fn expected(result:Result<(),ValueError>,kind:Kind){assert_eq!(result.unwrap_err().kind,kind);}
fn oracle(script:&str,bytes:&[u8])->Vec<u8>{use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));output.stdout}

#[test]
fn native_socket_sqlite_snapshot_original_raw_carriers_and_limits(){
 let f=fixture();assert_eq!(NativeSocketProbeSnapshot::SQLITE_SCHEMA,f["sql"].as_str().unwrap());assert_eq!(NativeSocketProbeSnapshot::EXTENSION,f["extension"].as_str().unwrap());
 for(text,bytes)in samples(&f){let limits=Limits{max_rows:1,max_tables:1,max_columns:2,max_value_bytes:bytes,max_schema_bytes:NativeSocketProbeSnapshot::SQLITE_SCHEMA.len(),..Default::default()};
  for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let input=payload(&text,encoding);let mut callback=|_|true;let mut control=Control::new(&mut callback,limits);let snapshot=decode(&input,&mut control).unwrap();assert_eq!(snapshot.0,text);let database=snapshot.to_sqlite_database(&mut control).unwrap();assert_eq!(database.tables.len(),1);assert_eq!(database.tables[0].rows[0].values,vec![SqliteValue::Integer(1),SqliteValue::Text(text.clone())]);let restored=NativeSocketProbeSnapshot::from_sqlite_database(&database,&mut control).unwrap();assert_eq!(restored.0,text);assert_eq!(encode(&restored,encoding,&mut control).unwrap(),input);
   for short in[Limits{max_rows:0,..limits},Limits{max_tables:0,..limits},Limits{max_columns:1,..limits},Limits{max_schema_bytes:limits.max_schema_bytes-1,..limits},Limits{max_value_bytes:bytes-1,..limits},Limits{max_allocation_bytes:0,..limits}]{let mut cb=|_|true;assert!(snapshot.to_sqlite_database(&mut Control::new(&mut cb,short)).is_err());assert!(NativeSocketProbeSnapshot::from_sqlite_database(&database,&mut Control::new(&mut cb,short)).is_err());}
   let raw_limits=Limits{max_file_bytes:text.len(),..limits};let mut cb=|_|true;assert_eq!(decode(&input,&mut Control::new(&mut cb,raw_limits)).unwrap().0,text);assert_eq!(encode(&snapshot,encoding,&mut Control::new(&mut cb,raw_limits)).unwrap(),input);if !text.is_empty(){let short=Limits{max_file_bytes:text.len()-1,..limits};expected(decode(&input,&mut Control::new(&mut cb,short)).map(drop),Kind::OwnershipLimit);expected(encode(&snapshot,encoding,&mut Control::new(&mut cb,short)).map(drop),Kind::OwnershipLimit);}
  }
 }
 for bytes in f["invalidUtf8"].as_array().unwrap(){let input=IoPayload::Binary(bytes.as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect());let mut cb=|_|true;expected(decode(&input,&mut Control::new(&mut cb,Limits::default())).map(drop),Kind::InvalidValue);}
 eprintln!("[DEBUG] Socket original Native Text/Binary full semantic/control corpus");
}

#[test]
fn native_socket_sqlite_snapshot_original_native_ledgers_remain_cumulative(){
 let input=IoPayload::Text("λ🙂".into());let length=6;let mut cb=|_|true;let mut progress=|_|true;let mut c=Control::new(&mut progress,Limits::default());let mut native=NativeDecodeControl::new(length+3,&mut cb);native.charge(3).unwrap();let snapshot=NativeSocketProbeSnapshot::decode_sqlite_snapshot_native(&input,&mut c,&mut native).unwrap();assert_eq!(native.owned_bytes(),length+3);assert_eq!(c.allocation_remaining_bytes(),Limits::default().max_allocation_bytes-length);expected(NativeSocketProbeSnapshot::decode_sqlite_snapshot_native(&input,&mut c,&mut native).map(drop),Kind::OwnershipLimit);assert_eq!(native.owned_bytes(),length+3);
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let mut cb=|_|true;let mut c=Control::new(&mut progress,Limits::default());let mut native=NativeEncodeControl::new(length+3,&mut cb);native.charge(3).unwrap();assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut c,&mut store::os_store::NativeSnapshotEncodeOwner::new(&mut native,native_grant())).unwrap(),payload(&snapshot.0,encoding));assert_eq!(native.owned_bytes(),length+3);expected(snapshot.encode_sqlite_snapshot_native(encoding,&mut c,&mut store::os_store::NativeSnapshotEncodeOwner::new(&mut native,native_grant())).map(drop),Kind::OwnershipLimit);
  let mut cb=|_|true;let mut native=NativeEncodeControl::new(length-1,&mut cb);expected(snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut progress,Limits::default()),&mut store::os_store::NativeSnapshotEncodeOwner::new(&mut native,native_grant())).map(drop),Kind::OwnershipLimit);
 }
 let mut cb=|_|true;let mut native=NativeDecodeControl::new(length-1,&mut cb);expected(NativeSocketProbeSnapshot::decode_sqlite_snapshot_native(&input,&mut Control::new(&mut progress,Limits::default()),&mut native).map(drop),Kind::OwnershipLimit);
 let exact=Limits{max_allocation_bytes:length,..Default::default()};let mut c=Control::new(&mut progress,exact);assert_eq!(decode(&input,&mut c).unwrap().0,snapshot.0);assert_eq!(c.allocation_remaining_bytes(),0);expected(encode(&snapshot,SnapshotEncoding::Text,&mut c).map(drop),Kind::OwnershipLimit);let short=Limits{max_allocation_bytes:length-1,..Default::default()};expected(decode(&input,&mut Control::new(&mut progress,short)).map(drop),Kind::OwnershipLimit);expected(encode(&snapshot,SnapshotEncoding::Binary,&mut Control::new(&mut progress,short)).map(drop),Kind::OwnershipLimit);
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let mut c=Control::new(&mut progress,exact);assert_eq!(encode(&snapshot,encoding,&mut c).unwrap(),payload(&snapshot.0,encoding));assert_eq!(c.allocation_remaining_bytes(),0);}
 eprintln!("[DEBUG] Socket exact original cumulative native ledgers and positive one-short refusals");
}

#[test]
fn native_socket_sqlite_snapshot_direct_reconstruction_and_interior_cancellation(){
 let f=fixture();let mut callback=|_|true;let original=NativeSocketProbeSnapshot("probe".into()).to_sqlite_database(&mut Control::new(&mut callback,Limits::default())).unwrap();
 for label in f["malformed"].as_array().unwrap(){let mut database=original.clone();let rows=&mut database.tables[0].rows;match label.as_str().unwrap(){"missingRow"=>rows.clear(),"multipleRows"=>rows.push(rows[0].clone()),"wrongRowid"=>rows[0].rowid=2,"wrongIdentity"=>rows[0].values[0]=SqliteValue::Integer(2),"nullText"=>rows[0].values[1]=SqliteValue::Null,"extraCell"=>rows[0].values.push(SqliteValue::Text("extra".into())),_=>panic!("closed malformed corpus")};assert!(NativeSocketProbeSnapshot::from_sqlite_database(&database,&mut Control::new(&mut callback,Limits::default())).is_err());}
 let large=f["large"]["unit"].as_str().unwrap().repeat(f["large"]["repetitions"].as_u64().unwrap()as usize);let snapshot=NativeSocketProbeSnapshot(large.clone());let database=snapshot.to_sqlite_database(&mut Control::new(&mut callback,Limits::default())).unwrap();
 for phase in[Phase::ProjectSnapshot,Phase::ReconstructSnapshot]{let mut interior=false;let mut cb=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{let stop=event.phase==phase&&event.total==large.len()&&event.completed>0&&event.completed<event.total;interior|=stop;!stop};let mut c=Control::new(&mut cb,Limits::default());let result=if phase==Phase::ProjectSnapshot{snapshot.to_sqlite_database(&mut c).map(drop)}else{NativeSocketProbeSnapshot::from_sqlite_database(&database,&mut c).map(drop)};expected(result,Kind::Canceled);assert!(interior);}
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{
  let mut interior=false;let mut cb=|event:NativeDecodeProgress|{let stop=event.total==large.len()&&event.completed>0&&event.completed<event.total&&event.owned_bytes>0;interior|=stop;!stop};let mut native=NativeDecodeControl::new(Limits::default().max_allocation_bytes,&mut cb);let mut c=Control::new(&mut callback,Limits::default());expected(NativeSocketProbeSnapshot::decode_sqlite_snapshot_native(&payload(&large,encoding),&mut c,&mut native).map(drop),Kind::Canceled);assert!(interior);assert_eq!(native.owned_bytes(),large.len());assert_eq!(c.allocation_remaining_bytes(),Limits::default().max_allocation_bytes-large.len());
  let mut interior=false;let mut cb=|event:NativeEncodeProgress|{let stop=event.total==large.len()&&event.completed>0&&event.completed<event.total&&event.owned_bytes>0;interior|=stop;!stop};let mut native=NativeEncodeControl::new(Limits::default().max_allocation_bytes,&mut cb);let mut c=Control::new(&mut callback,Limits::default());expected(snapshot.encode_sqlite_snapshot_native(encoding,&mut c,&mut store::os_store::NativeSnapshotEncodeOwner::new(&mut native,native_grant())).map(drop),Kind::Canceled);assert!(interior);assert_eq!(native.owned_bytes(),large.len());assert_eq!(c.allocation_remaining_bytes(),Limits::default().max_allocation_bytes-large.len());
 }
 eprintln!("[DEBUG] Socket original malformed entities and four actual inside-field cancellation controls");
}

#[test]
fn native_socket_sqlite_snapshot_original_registered_typed_erased_io_and_independent_file_edit(){
 use store::io::io_mechanism::{io_route,io_export_sqlite_snapshot,io_import_sqlite_snapshot,io_run_with_snapshot_control,IoRunControl};
 let f=fixture();let(owner,declared)=NativeSocketProbeSnapshot::native_snapshot_registration().expect("normal Socket owner registration");let dialect=ArtifactDialect::from(owner);assert_eq!(dialect.to_coordinate(),f["dialect"].as_str().unwrap());assert_eq!(declared.schema,f["artifactSchema"].as_str().unwrap());NativeSocketProbeSnapshot::publish_native_snapshot().unwrap();let installed=block_on(store::os_store::document_codec(&declared.schema)).unwrap().unwrap();let provider=installed.snapshot_sqlite.as_ref().unwrap();assert!(provider.identical_to(declared.snapshot_sqlite.as_ref().unwrap()));assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<NativeSocketProbeSnapshot>()));
 let sqlite=ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);let export=block_on(io_route(&dialect,&sqlite,1)).unwrap().value;let import=block_on(io_route(&sqlite,&dialect,1)).unwrap().value;assert_eq!(export.fidelity,store::io_schema::IoFidelity::Exact);assert_eq!(import.fidelity,store::io_schema::IoFidelity::Exact);
 for(text,_)in samples(&f){let snapshot=NativeSocketProbeSnapshot(text);
  for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let input=payload(&snapshot.0,encoding);let mut cb=|_|true;let mut native_cb=|_|true;let mut native=NativeDecodeControl::new(Limits::default().max_allocation_bytes,&mut native_cb);let projected=(provider.export)(&installed.schema,&dialect,&input,&mut Control::new(&mut cb,Limits::default()),&mut store::os_store::NativeSnapshotDecodeOwner::new(&mut native,native_grant())).unwrap().value;let mut native_cb=|_|true;let mut native=NativeEncodeControl::new(Limits::default().max_allocation_bytes,&mut native_cb);assert_eq!((provider.import)(&installed.schema,&dialect,projected,encoding,&mut Control::new(&mut cb,Limits::default()),&mut store::os_store::NativeSnapshotEncodeOwner::new(&mut native,native_grant())).unwrap().value,input);
   let file=block_on(io_export_sqlite_snapshot(&dialect,&snapshot,encoding,Limits::default(),&mut cb)).unwrap().value;assert!(file.starts_with(b"SQLite format 3\0"));assert_eq!(block_on(io_import_sqlite_snapshot::<NativeSocketProbeSnapshot>(&dialect,&file,Limits::default(),&mut cb)).unwrap().value,snapshot);
   let script=format!(r#"import{{Database}}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{{safeIntegers:true}});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query("SELECT COUNT(*) AS n FROM sqlite_schema WHERE type='table'").get().n!==2n)throw Error('complete entities');if(d.query('SELECT text FROM socket_probe').get().text!=={})throw Error('raw text');const m=d.query('SELECT artifact_kind,standard,subset FROM semio_snapshot').get();if(m.artifact_kind!=='native.socket-grant.probe'||m.standard!=='1'||m.subset!=='*')throw Error('declared metadata');d.query('UPDATE socket_probe SET text=? WHERE id=1').run({});await Bun.write(Bun.stdout,d.serialize());d.close();"#,serde_json::to_string(&snapshot.0).unwrap(),serde_json::to_string(f["edit"].as_str().unwrap()).unwrap());let edited=oracle(&script,&file);assert_eq!(block_on(io_import_sqlite_snapshot::<NativeSocketProbeSnapshot>(&dialect,&edited,Limits::default(),&mut cb)).unwrap().value.0,f["edit"].as_str().unwrap());
   let mut decode_cb=|_|true;let mut encode_cb=|_|true;let mut decode=NativeDecodeControl::new(Limits::default().max_allocation_bytes,&mut decode_cb);let mut encode=NativeEncodeControl::new(Limits::default().max_allocation_bytes,&mut encode_cb);let mut native=IoRunControl::new(&mut decode,&mut encode,native_grant());let mut control=Control::new(&mut cb,Limits::default());let physical=block_on(io_run_with_snapshot_control(&export,input.clone(),&mut native,&mut control)).unwrap().value;assert_eq!(block_on(io_run_with_snapshot_control(&import,physical,&mut native,&mut control)).unwrap().value,input);
   let mut decode_cb=|_|true;let mut encode_cb=|_|true;let mut decode=NativeDecodeControl::new(Limits::default().max_allocation_bytes,&mut decode_cb);let mut encode=NativeEncodeControl::new(Limits::default().max_allocation_bytes,&mut encode_cb);assert_eq!(block_on(io_run_with_snapshot_control(&import,IoPayload::Binary(edited),&mut IoRunControl::new(&mut decode,&mut encode,native_grant()),&mut Control::new(&mut cb,Limits::default()))).unwrap().value,payload(f["edit"].as_str().unwrap(),encoding));
  }
 }
 eprintln!("[DEBUG] Socket normal production registration, installed provider, public typed/erased Text/Binary SQLite and independent physical edits for every neutral case");
}

#[test]
fn native_socket_snapshot_encode_borrows_the_original_grant_and_native_receipt() {
 let grant=native_grant();
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary] {
  let snapshot=NativeSocketProbeSnapshot("original λ🙂\0".into());
  let mut observe=|_|true;
  let mut native=NativeEncodeControl::new(1024,&mut observe);
  native.charge(3).unwrap();
  let pointer=&native as *const _;
  let mut owner=store::os_store::NativeSnapshotEncodeOwner::new(&mut native,grant);
  assert_eq!(owner.native() as *const _,pointer);
  let mut progress=|_|true;
  let mut control=Control::new(&mut progress,Limits::default());
  assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut control,&mut owner).unwrap(),payload(&snapshot.0,encoding));
  assert_eq!(owner.grant(),grant);
  assert_eq!(owner.native() as *const _,pointer);
  assert_eq!(owner.native().owned_bytes(),3+snapshot.0.len());
  assert_eq!(owner.native().maximum_bytes(),1024);
  println!("[DEBUG] original Socket snapshot encode unchangedGrant=true sameNative=true cumulativeNativeOwned={}",owner.native().owned_bytes());
 }
}
