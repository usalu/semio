//! 🪶️ Held real Hub authority law; include only from the owning binary test module.
use super::*;
fn independent_count_file(root:&std::path::Path,bytes:&[u8],count:i32,encoding:&str){
 let path=root.join(format!("count-{count}-{encoding}.sqlite"));std::fs::write(&path,bytes).unwrap();
 let source=r#"import{Database}from"bun:sqlite";import assert from"node:assert/strict";const[path,count,encoding]=process.argv.slice(1);const d=new Database(path,{readonly:true});try{assert.deepEqual(d.query("SELECT id,count,typeof(count) AS storage FROM fixture_counter").all(),[{id:1,count:Number(count),storage:"integer"}]);assert.deepEqual(d.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all(),[{name:"fixture_counter"},{name:"semio_snapshot"}]);assert.deepEqual(d.query("PRAGMA table_info(fixture_counter)").all().map(x=>x.name),["id","count"]);assert.deepEqual(d.query("SELECT artifact_kind,standard,subset,schema_version,native_encoding FROM semio_snapshot").all(),[{artifact_kind:"fixture.neutral-host-fixture.counter",standard:"1",subset:"*",schema_version:1,native_encoding:encoding}]);assert.deepEqual(d.query("PRAGMA integrity_check").get(),{integrity_check:"ok"});assert.deepEqual(d.query("PRAGMA foreign_key_check").all(),[])}finally{d.close()}"#;
 let result=std::process::Command::new("bun").args(["-e",source]).arg(&path).arg(count.to_string()).arg(encoding).output().unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));
}
#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn real_count_component_lease_uses_authenticated_hub_authority_and_erased_sqlite_io(){
 use semio_hub::artifact_authority::trusted_catalog::count_trusted_catalog_fixture;
 use semio_framework_os_mcp::HeadlessWorkspace;
 use directory::os_directory::client::LocalHubCredential;
 use std::path::PathBuf;
 use semio_framework::io_schema::{ArtifactDialect,IoPayload,SQLITE_SNAPSHOT};
 use directory::io::io_mechanism::{io_route,io_run_with_snapshot_control};
 use semio_framework::sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotPhase,SqliteValue};
 use semio_framework_value::ValueRefusalKind;
 let output=crate::test_artifact_root::test_artifact_root().join("real-count-workspace-lease");std::fs::create_dir_all(&output).unwrap();
 let mut repo=PathBuf::from(env!("CARGO_MANIFEST_DIR"));while !repo.join("nx.json").is_file(){assert!(repo.pop())}
 let component=std::fs::read(repo.join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/📦️packages/🦀️rust/dist/component-dev/semio_framework_plugin_host_test_component.wasm")).expect("actual Count component prerequisite");
 let document_id=artifact_document_id_for_test("real-count-workspace-lease");
 let profile=count_trusted_catalog_fixture::verified_count_profile(&output.join("verified-profile"),&component,&document_id).await.expect("production trusted loader accepts actual Count descriptor/component");
 let selection=profile.selection().clone();let mut state=test_state().await;
 let email="count-author@example.test";let author=issue_test_session(&state,email).await;let owner=issue_test_session(&state,"count-owner@example.test").await;
 let space_id=create_space_for_test(&state,&owner.user_id,"Real Count Lease",os_directory::DirectorySpaceKind::Studio,DirectorySpaceVisibility::Private).await;
 upsert_member_for_test(&state,&space_id,email,DirectorySpaceRole::Author).await;
 let scope=DocumentScope::new(space_id.clone(),document_id.clone());
 let descriptor=DocumentDescriptor{space_id:scope.space_id.clone(),document_id:scope.document_id.clone(),artifact_kind:selection.artifact.kind.clone(),artifact_schema:selection.artifact.schema.clone(),owner:os_directory::DocumentOwner{plugin_id:selection.package.plugin_id.clone(),package_id:selection.package.package_id.clone(),version:selection.package.version.clone(),package_hash:selection.package.component_sha256.clone()},pack_schema_hash:selection.artifact.pack_schema_hash.clone(),bootstrap_version:1,bootstrap_frontier:os_directory::DocumentFrontier{head_seq:0,commit_seq:0,epoch:0},bootstrap_snapshot_hash:os_directory::hex_lower(&Sha256::digest(&profile.pack))};
 state.artifact_authority=Some(Arc::new(ValidatingCanonicalArtifactAuthority::new(profile.catalog().clone())));state.verified_catalog=Some(profile.catalog().clone());state.openable_catalog=Some(profile.catalog().clone());
 let session=state.directory.authenticate_session(&SessionCapability::parse(&author.token).unwrap()).await.unwrap().unwrap();
 let actor=ArtifactCreationActorV1{user_id:author.user_id.clone(),session_id:session.id,authorization_generation:session.authorization_generation};
 publish_genesis_checkpoint_for_test(&state,actor,profile.catalog().generation_id().into(),selection.parent_dialect.clone(),descriptor,&profile.pack,&profile.spr).await;
 let(addr,shutdown,server)=spawn_restartable_server(state).await;
 let origin=format!("http://{addr}");let scope_for_worker=scope.clone();let pack=profile.pack.clone();let spr=profile.spr.clone();let component_hash=os_directory::hex_lower(&Sha256::digest(&component));let component_blake3=os_directory::hex_lower(semio_framework_hash::hash(&component).as_bytes());let component_length=component.len() as u64;
 let codec_runtime=profile.codec_runtime.clone();let compiled=profile.compiled.clone();
 let worker=tokio::task::spawn_blocking(move||{
  let credential=Arc::new(LocalHubCredential::from_minted_session(&origin,&author.token).expect("actual issued session capability"));
  let workspace=HeadlessWorkspace::open_hub(origin.clone(),space_id,credential,author.user_id,vec![]).expect("actual authenticated directory/catalog connection");
  let catalog=workspace.verified_hub_catalog_selections().unwrap();let admitted=catalog.selections.iter().find(|row|row.scope==scope_for_worker).expect("real admitted Count document");
  admitted.lease.validate().expect("actual complete issued lease");assert_eq!(admitted.lease.package.component_sha256,component_hash);assert_eq!(admitted.lease.artifact.schema,"fixture.neutral-host-fixture.counter");assert_eq!(admitted.lease.parent_dialect.artifact_kind,"fixture.neutral-host-fixture.counter");assert_eq!(admitted.lease.parent_dialect.standard,"1");assert_eq!(admitted.lease.parent_dialect.subset,"*");
  assert_eq!(admitted.lease.scope,scope_for_worker);assert_eq!(admitted.lease.package,selection.package);assert_eq!(admitted.lease.artifact,selection.artifact);assert_eq!(admitted.lease.surface,selection.surface);assert_eq!(admitted.lease.grant,selection.grant);assert_eq!(admitted.lease.component.sha256,component_hash);assert_eq!(admitted.lease.component.blake3,component_blake3);assert_eq!(admitted.lease.component.byte_length,component_length);assert_eq!(admitted.lease.descriptor.sha256,selection.package.descriptor_byte_sha256);
  assert!(workspace.bind_hub_session_document(&document_id,&pack,&spr).expect("real lease publication and actual guest codec registration"));
  let binding=workspace.plugin_artifact_binding(&document_id).expect("actual retained document binding");assert_eq!(binding.plugin_id,"neutral-host-fixture");
  let codec=semio_framework_async::block_on(directory::os_store::document_codec("fixture.neutral-host-fixture.counter")).unwrap().unwrap();assert!(codec.snapshot_sqlite.as_ref().unwrap().snapshot_type.is_none(),"real guest erasure must not use a local Count TypeId");
  semio_framework_async::block_on(async{
   let dialect=ArtifactDialect{artifact_kind:"fixture.neutral-host-fixture.counter".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);
   let export=io_route(&dialect,&sqlite,1).await.expect("actual guest public export").value;let import=io_route(&sqlite,&dialect,1).await.expect("actual guest public import").value;
   for count in [i32::MIN,-1,0,1,i32::MAX]{
    let payload=IoPayload::Text(format!("{{\"count\":{count}}}"));let file=io_run_with_snapshot_control(&export,payload.clone(),SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;
    let IoPayload::Binary(bytes)=&file else{panic!("physical SQLite file required")};
    independent_count_file(&output,bytes,count,"text");
    let mut database=semio_framework::sqlite_snapshot::import_sqlite_database(bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(database.table("fixture_counter").unwrap().rows[0].values,vec![SqliteValue::Integer(1),SqliteValue::Integer(i64::from(count))]);
    assert_eq!(directory::io::io_mechanism::sqlite_snapshot_metadata(&database).unwrap().0,dialect);
    let returned=io_run_with_snapshot_control(&import,file.clone(),SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let IoPayload::Text(text)=returned else{panic!("text encoding preserved")};assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(),serde_json::json!({"count":count}));
    database.table_mut("semio_snapshot").unwrap().rows[0].values[5]=SqliteValue::Text("binary".into());let binary_file=semio_framework::sqlite_snapshot::export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let binary=io_run_with_snapshot_control(&import,IoPayload::Binary(binary_file),SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;let IoPayload::Binary(native_pack)=&binary else{panic!("native Binary required")};
    let mirror=codec_runtime.codec_print_mirror(&compiled,"fixture.neutral-host-fixture.counter",native_pack,&spr,semio_framework_plugin_host::Budget{fuel:8_000_000_000,deadline_ms:120000,max_effects:0,max_patch_bytes:0,max_frames:0}).await.expect("actual compiled owner decodes the native Count record");assert_eq!(serde_json::from_str::<serde_json::Value>(&mirror.dsl).unwrap(),serde_json::json!({"count":count}));
    let physical=io_run_with_snapshot_control(&export,binary.clone(),SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;
    let IoPayload::Binary(binary_sqlite)=&physical else{panic!("binary native export still produces physical SQLite")};independent_count_file(&output,binary_sqlite,count,"binary");
    let binary_domain=semio_framework::sqlite_snapshot::import_sqlite_database(binary_sqlite,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(binary_domain.table("fixture_counter").unwrap().rows[0].values,vec![SqliteValue::Integer(1),SqliteValue::Integer(i64::from(count))]);
    let restored=io_run_with_snapshot_control(&import,physical,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert_eq!(restored,binary);
    for (route,input,phase)in[(&export,payload,SqliteSnapshotPhase::ProjectSnapshot),(&import,file,SqliteSnapshotPhase::ReconstructSnapshot)]{
     let mut observed=0;let error=io_run_with_snapshot_control(route,input,SqliteDatabaseLimits::default(),&mut|event|{if event.phase==phase&&event.total==0&&event.completed==0{observed+=1;return observed<2}true}).await.expect_err("interior real guest progress cancellation");assert_eq!(observed,2);assert_eq!(error.cause.kind,ValueRefusalKind::Canceled);
    }
   }
   let missing=ArtifactDialect{artifact_kind:dialect.artifact_kind.clone(),standard:"1".into(),subset:"undeclared-lease-owner".into()};assert!(io_route(&missing,&sqlite,1).await.is_err(),"unpublished exact owner cannot borrow the Count lease");
  });
  drop(workspace);
 });
 let result=worker.await;let _=shutdown.send(());server.await.expect("authenticated Hub server closes");drop(profile);result.expect("real Count workspace owner law");
}
