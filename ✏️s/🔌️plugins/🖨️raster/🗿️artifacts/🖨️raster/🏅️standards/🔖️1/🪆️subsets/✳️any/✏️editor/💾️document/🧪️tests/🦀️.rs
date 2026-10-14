//! 💾️ Editable archives preserve the layer tree, embedded image owners and semantic undo history.
use crate::unit_tests::context;
use crate::RasterCommand;
use crate::commands::patch_layer;
use semio_s_artifact_raster_raster_model::testing::raster_image_test_snapshot;
use semio_s_artifact_raster_raster_model::{find_layer, layer_name};
use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn editable_archive_restores_nested_masks_assets_adjustments_and_history() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let mut document=raster_image_test_snapshot();document.title=Some(fixture["title"].as_str().unwrap().into());
    let layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();semio_s_artifact_raster_raster_model::retire_raster_layers(std::mem::replace(&mut document.layers,layers));
    let mut envelope=store::os_store::document_materialize::create_document_envelope::<semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot,semio_s_artifact_raster_raster_mutations::RasterMutation>(semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA,"raster",document,None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);envelope.dialect=Some(semio_s_artifact_raster_raster_model::RASTER_DIALECT.into());
    let files=store::os_store::text_format::print_document_pack(&envelope).await.unwrap();context::retire_raster_envelope(envelope);
    let mut source=context::app().await;semio_framework_plugin::artifact_app_laws::load_document(&mut source, &files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
    let before=source.snapshot().unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&before.layers)).unwrap(),fixture["layers"]);
    let rename=&fixture["rename"];let id=rename["id"].as_str().unwrap();
    context::dispatch(&mut source,RasterCommand::PatchLayer(patch_layer::PatchLayer {layer_id:id.into(),field:"name".into(),value:rename["after"].as_str().unwrap().into()})).await;
    let edited=source.snapshot().unwrap();let archive=source.document_archive().await.unwrap();
    assert!(!archive.parent_pack.is_empty());assert!(!archive.parent_spr.is_empty());
    let mut restored=context::app().await;restored.begin_document_archive_load(71,archive).unwrap();
    let mut ready=false;
    for _ in 0..100000 {
        let status=restored.poll_document_archive_load(71, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap();
        match status.state {
            protocol::DocumentArchiveLoadState::Ready=>{ready=true;break;},
            protocol::DocumentArchiveLoadState::Fault=>panic!("{}",semio_framework_diagnostic::decode_fault_bytes(&status.fault).describe()),
            protocol::DocumentArchiveLoadState::Cancelled=>panic!("archive unexpectedly cancelled"),
            _=>{}
        }
        restored.maintenance_step(1,store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).unwrap();semio_framework_async::yield_once().await;
    }
    assert!(ready,"editable archive did not finish within its fixture turn bound");restored.acknowledge_document_archive_load(71).unwrap();
    let loaded=restored.snapshot().unwrap();assert_eq!(loaded,edited);assert_eq!(layer_name(find_layer(&loaded.layers,id).unwrap()),rename["after"].as_str().unwrap());retire_raster_snapshot(loaded);
    context::history(&mut restored,"undo").await;let undone=restored.snapshot().unwrap();assert_eq!(undone,before);retire_raster_snapshot(undone);
    context::history(&mut restored,"redo").await;let redone=restored.snapshot().unwrap();assert_eq!(redone,edited);retire_raster_snapshot(redone);
    retire_raster_snapshot(before);retire_raster_snapshot(edited);
}
