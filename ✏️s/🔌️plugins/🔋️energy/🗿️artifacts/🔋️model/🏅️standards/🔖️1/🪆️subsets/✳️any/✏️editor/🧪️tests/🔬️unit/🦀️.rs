use super::*;
use crate::model::{Surface, Thermostat};
use std::collections::BTreeSet;

fn model_value(snapshot:&EnergyModelSnapshot)->serde_json::Value{serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(snapshot)).unwrap()["model"].clone()}
fn plain_model(snapshot:&EnergyModelSnapshot)->crate::model::Model{serde_json::from_value(model_value(snapshot)).expect("the snapshot projects to a plain model")}
fn plain_value(model:&crate::model::Model)->serde_json::Value{serde_json::to_value(model).unwrap()}
async fn document_files(snapshot:EnergyModelSnapshot)->store::os_store::serialization::ArtifactPackFiles{
 let pack=<EnergyModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
 let envelope=store::os_store::document_materialize::create_document_envelope::<EnergyModelSnapshot,EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA,"model",snapshot,None,&::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners();
 let spr=store::os_store::text_format::print_document_spr(&envelope).await.expect("fixture spr");
 store::os_store::serialization::ArtifactPackFiles{pack,spr,ops:String::new()}
}
use semio_plugin::PluginApp as _;

include!("🧰️definition-and-vocabulary-laws/🦀️.rs");

include!("🧵️dispatch-scrub-and-run-laws/🦀️.rs");

include!("🔍️inspector-and-verb-laws/🦀️.rs");

include!("🌉️host-merge-ownership-and-recolour-laws/🦀️.rs");

/// 📜️ The energy document contract distinguishes fresh model imports from archived event histories.
#[semio_framework_async_macros::async_test]
async fn energy_document_history_archive_reload_example_switch_and_snapshot_import_follow_the_shared_store(){
 use semio_plugin::{PluginApp as _,DslValue,kernel::Effect};
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📜️document-history/🔣️.json")).unwrap();
 let mut app=dispatchable_app().await;let before=app.snapshot().unwrap().model.read().name.clone();rename(&mut app,law["documents"]["archive"]["modelName"].as_str().unwrap()).await;
 let rows=transaction_rows(&mut app).await;assert!(!rows.is_empty());let ids=rows.iter().flat_map(|row|row.mutations.iter().map(|mutation|mutation.mutation_id.clone())).collect::<Vec<_>>();let wanted=model_value(&app.snapshot().unwrap());let files=app.document_pack().await.unwrap();
 let mut reloaded=dispatchable_app().await;semio_plugin::artifact_app_laws::load_document(&mut reloaded.0,&files,&mut semio_plugin::artifact_app_laws::fixture_identity(),reloaded.1.cx()).await.unwrap();assert_eq!(model_value(&reloaded.snapshot().unwrap()),wanted);assert_eq!(transaction_rows(&mut reloaded).await.iter().flat_map(|row|row.mutations.iter().map(|mutation|mutation.mutation_id.clone())).collect::<Vec<_>>(),ids);
 semio_plugin::artifact_app_laws::settle_history_verb(&mut reloaded.0,"undo",semio_plugin::artifact_app_laws::meta("local").instance_id,semio_plugin::artifact_app_laws::fixture_mounted_policy(),&mut semio_plugin::artifact_app_laws::fixture_identity(),reloaded.1.cx()).await;assert_eq!(reloaded.snapshot().unwrap().model.read().name.clone(),before);
 semio_plugin::artifact_app_laws::settle_history_verb(&mut reloaded.0,"redo",semio_plugin::artifact_app_laws::meta("local").instance_id,semio_plugin::artifact_app_laws::fixture_mounted_policy(),&mut semio_plugin::artifact_app_laws::fixture_identity(),reloaded.1.cx()).await;assert_eq!(model_value(&reloaded.snapshot().unwrap()),wanted);
 let (id,_,example)=example_rows().into_iter().next().unwrap();let args=DslValue::Object(vec![("exampleId".into(),DslValue::String(id.into()))]);semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0,SET_ACTIVE_EXAMPLE_ACTION_ID,Some(args),&semio_plugin::artifact_app_laws::meta("local"),&mut semio_plugin::artifact_app_laws::fixture_identity(),app.1.cx(),semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();let receipt=semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0,semio_plugin::artifact_app_laws::meta("local").instance_id,semio_plugin::artifact_app_laws::fixture_mounted_policy(),&mut semio_plugin::artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap();let mut loads=receipt.effects.into_iter().filter_map(|effect|match effect{Effect::LoadDocument{pack,spr}=>Some(store::os_store::serialization::ArtifactPackFiles{pack,spr,ops:String::new()}),_=>None}).collect::<Vec<_>>();assert_eq!(loads.len(),1);semio_plugin::artifact_app_laws::load_document(&mut app.0,&loads.remove(0),&mut semio_plugin::artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap();assert_eq!(model_value(&app.snapshot().unwrap()),plain_value(&example));assert!(transaction_rows(&mut app).await.is_empty());
 let mut imported=example;imported.name="Energy imported snapshot".into();let original=EnergyModelSnapshot::from_model(&imported).expect("admissible imported model");let json=semio_framework_pack_json::to_json_string(&original);let decoded=semio_s_artifact_energy_model_import::json::deserialize_bytes(json.as_bytes()).unwrap();assert_eq!(model_value(&decoded),plain_value(&imported));assert_eq!(serde_json::from_str::<serde_json::Value>(&json).unwrap()["model"],serde_json::to_value(&imported).unwrap());assert!(semio_s_artifact_energy_model_import::json::deserialize_bytes(&[255]).is_err());let files=document_files(decoded).await;semio_plugin::artifact_app_laws::load_document(&mut app.0,&files,&mut semio_plugin::artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap();assert_eq!(model_value(&app.snapshot().unwrap()),plain_value(&imported));assert!(transaction_rows(&mut app).await.is_empty());
 close(app);close(reloaded);
}

/// 🪞️ A registered energy editor keeps the shared Store and original composed member factories on each peer.
async fn energy_history_replica(actor:&str,uri:&str,files:Option<&store::os_store::serialization::ArtifactPackFiles>)->(EnergyEditorApp,store::os_store::backbone::MemoryBackbone){
 use semio_plugin::PluginApp as _;
 let scope=semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();let mut app=semio_plugin::artifact_app_laws::new_app_with_registry_and_members::<EditorApp<EnergyModelEditor>,semio_s_artifact_stdio_semio::SemioMembers>(energy_model_manifest_for_tests,semio_os_kernel::ActorId(actor.into()),semio_plugin::artifact_app_laws::fixture_mounted_policy(),&mut semio_plugin::artifact_app_laws::fixture_identity()).await;app.bind_instance_id(semio_plugin::artifact_app_laws::meta("local").instance_id).await;let mut app=EnergyEditorApp(app,scope);if let Some(files)=files{semio_plugin::artifact_app_laws::load_document(&mut app.0,files,&mut semio_plugin::artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap();}let(backbone,probe)=store::os_store::backbone::MemoryBackbone::pair(uri,uri).await;app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.unwrap();(app,probe)
}

/// 🔀️ The actual shared memory backbone delivers retained event batches and drives its Store fold.
async fn energy_history_relay(from:&mut store::os_store::backbone::MemoryBackbone,into:&mut store::os_store::backbone::MemoryBackbone,to:&mut EnergyEditorApp){
 use store::os_store::backbone::Backbone as _;use semio_plugin::PluginApp as _;
 for message in from.receive().await.unwrap(){into.send(message).await.unwrap();}to.0.tick_backbone(&mut semio_plugin::artifact_app_laws::fixture_identity(),to.1.cx()).await.unwrap();settle(to).await;to.history_snapshot().await.unwrap();
}

/// 📡️ Short connection gaps keep local work responsive; shared inverse/history events preserve another author's fields.
#[semio_framework_async_macros::async_test]
async fn energy_document_history_two_peers_reconnect_undo_redo_and_time_travel_preserve_independent_fields(){
 use semio_plugin::{PluginApp as _,DslValue};
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📜️document-history/🔣️.json")).unwrap();let peer=&law["peer"];
 let(mut local,mut local_probe)=energy_history_replica(peer["actors"][0].as_str().unwrap(),"energy-history-local",None).await;let initial=local.document_pack().await.unwrap();let(mut remote,mut remote_probe)=energy_history_replica(peer["actors"][1].as_str().unwrap(),"energy-history-remote",Some(&initial)).await;
 slide_site_field(&mut local,"latitudeDeg",press(peer["localLatitude"].as_f64().unwrap(),"energy-peer.latitude",true)).await;let edited=transaction_rows(&mut local).await.into_iter().max_by_key(|row|row.seq).and_then(|row|row.mutations.first().map(|mutation|mutation.mutation_id.clone())).unwrap();rename(&mut local,"Energy seed").await;energy_history_relay(&mut local_probe,&mut remote_probe,&mut remote).await;
 local.detach_backbone().await.unwrap();rename(&mut local,peer["localName"].as_str().unwrap()).await;slide_site_field(&mut remote,"longitudeDeg",press(peer["remoteLongitude"].as_f64().unwrap(),"energy-peer.longitude",true)).await;assert_eq!(local.snapshot().unwrap().model.read().name.clone(),peer["localName"].as_str().unwrap());assert_eq!(remote.snapshot().unwrap().model.read().site.longitude_deg,peer["remoteLongitude"].as_f64().unwrap());
 let(backbone,mut resumed_probe)=store::os_store::backbone::MemoryBackbone::pair("energy-history-resumed","energy-history-resumed").await;local.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.unwrap();energy_history_relay(&mut remote_probe,&mut resumed_probe,&mut local).await;energy_history_relay(&mut resumed_probe,&mut remote_probe,&mut remote).await;assert_eq!(model_value(&local.snapshot().unwrap()),model_value(&remote.snapshot().unwrap()));
 semio_plugin::artifact_app_laws::settle_history_verb(&mut local.0,"undo",semio_plugin::artifact_app_laws::meta("local").instance_id,semio_plugin::artifact_app_laws::fixture_mounted_policy(),&mut semio_plugin::artifact_app_laws::fixture_identity(),local.1.cx()).await;assert_eq!(local.snapshot().unwrap().model.read().name.clone(),"Energy seed");assert_eq!(local.snapshot().unwrap().model.read().site.longitude_deg,peer["remoteLongitude"].as_f64().unwrap());energy_history_relay(&mut resumed_probe,&mut remote_probe,&mut remote).await;
 semio_plugin::artifact_app_laws::settle_history_verb(&mut local.0,"redo",semio_plugin::artifact_app_laws::meta("local").instance_id,semio_plugin::artifact_app_laws::fixture_mounted_policy(),&mut semio_plugin::artifact_app_laws::fixture_identity(),local.1.cx()).await;energy_history_relay(&mut resumed_probe,&mut remote_probe,&mut remote).await;assert_eq!(model_value(&local.snapshot().unwrap()),model_value(&remote.snapshot().unwrap()));
 history_edit(&mut local,"historyEditBegin",vec![("mutationId",DslValue::String(edited))]).await;history_edit(&mut local,"historyEditInput",vec![("path",DslValue::String("/newLatitudeDeg".into())),("value",DslValue::float(peer["revisedLatitude"].as_f64().unwrap()))]).await;history_edit(&mut local,"historyEditAccept",Vec::new()).await;pump_time_travel(&mut local,|stage|stage!=Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;assert_eq!(local.snapshot().unwrap().model.read().site.latitude_deg,peer["localLatitude"].as_f64().unwrap());history_edit(&mut local,"historyEditFinalize",Vec::new()).await;history_edit(&mut local,"historyEditCommit",vec![("choice",DslValue::String("overwrite".into()))]).await;pump_time_travel(&mut local,|stage|stage.is_none()).await;energy_history_relay(&mut resumed_probe,&mut remote_probe,&mut remote).await;
 let head_snapshot=local.snapshot().unwrap();assert_eq!(model_value(&head_snapshot),model_value(&remote.snapshot().unwrap()));let head=head_snapshot.model.read();assert_eq!((head.name.as_str(),head.site.longitude_deg,head.site.latitude_deg),(peer["expected"]["name"].as_str().unwrap(),peer["expected"]["longitude"].as_f64().unwrap(),peer["expected"]["latitude"].as_f64().unwrap()));
 drop((local_probe,resumed_probe,remote_probe));close(local);close(remote);
}

/// 🧪️ The load effect is a genesis document, not an edit: its pack decodes back to exactly the
/// snapshot that went in, so the host's `ArtifactStore::reset` installs the loaded model itself
/// rather than a projection of it.
#[semio_framework_async_macros::async_test]
async fn the_load_document_effect_carries_the_model_it_was_given() {
    let mut model = crate::model::Model { name: "BESTEST 900".into(), version: "1".into(), ..crate::model::Model::default() };
    model.zones.push(crate::model::Zone { id: crate::model::EntityId(1), name: "Zone 1".into(), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true });
    let model_snapshot = EnergyModelSnapshot::from_model(&model).expect("admissible model");
    let files = document_files(model_snapshot).await;
    let (pack, spr) = (files.pack, files.spr);
    assert!(!spr.is_empty(), "the genesis envelope must still print an spr");
    let loaded = <EnergyModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).expect("the emitted pack decodes");
    assert_eq!(model_value(&loaded), plain_value(&model));
    assert_eq!(loaded.schema.get().as_str(), ENERGY_MODEL_DOCUMENT_SCHEMA);
}

#[test]
fn natural_idf_file_opens_in_the_energy_editor_and_saves_back_identically() {
    let model_json = include_str!("../../../🧪️tests/🧫️fixtures/🏛️bestest-600/🔋️model.json");
    let idf = semio_s_artifact_energy_model_export::idf::idf_from_model_json(model_json).expect("the BESTEST 600 model exports to IDF");
    let codec = <EnergyModelEditor as ArtifactEditor>::natural_file_codec().expect("the energy editor declares the IDF representation");
    assert_eq!((codec.format_kind, codec.extension), ("s.stdio.idf@25.2", ".idf"));
    let opened = <EnergyModelEditor as ArtifactEditor>::decode_natural_file(idf.as_bytes()).expect("a real IDF opens");
    let saved = <EnergyModelEditor as ArtifactEditor>::encode_natural_file(&opened).expect("save");
    assert_eq!(String::from_utf8(saved).expect("IDF is text"), idf);
}

#[test]
fn natural_idf_file_with_diagnostics_is_refused_with_the_diagnostic_named() {
    let error = <EnergyModelEditor as ArtifactEditor>::decode_natural_file(b"Version,25.2;UnknownObject,Named;").expect_err("an unsupported object is not dropped silently");
    assert!(error.to_string().contains("UnknownObject"), "{error}");
    let error = <EnergyModelEditor as ArtifactEditor>::decode_natural_file(&[0xff, 0xfe]).expect_err("binary is refused");
    assert!(error.to_string().contains("UTF-8"), "{error}");
}
