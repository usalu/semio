//! 🧪️ Contract §2.5 surface laws, now the real framework functions (`📓️w0-f-report.md` Gap 2) —
//! no local stand-ins.
use semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[test]
fn raster_live_editor_uses_its_original_registered_construction(){
 use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress,RetainedWorkOutcome};
 let fixture:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("../🧫️fixtures/native-registration.json")).unwrap();let mut program=super::plugin().unwrap();assert_eq!(program.manifest.plugin_id,fixture["pluginId"].as_str().unwrap());let app_id=&program.manifest.apps[fixture["editorIndex"].as_u64().unwrap()as usize].id;assert!(program.has_original_app_factory(app_id));assert!(!program.has_original_app_factory("absent.raster.editor"));
 let grant=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:fixture["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:fixture["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:4096};let stock=RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096};let mut caller=OriginalJobFixtureCaller::admit(OperationId(588),Generation(29),stock,||Some(1),u64::MAX,1<<26);let mut decoder=OriginalJobDecodeOwner::empty();let mut construction=None;
 assert!(caller.turn_with_decoder(grant,&mut decoder,|cx|program.admit_original_app(app_id,cx,&mut construction)).unwrap());assert!(construction.is_some());caller.cancel().cancel_now();for _ in 0..65536{if construction.as_ref().unwrap().terminal_is_empty(){break}caller.turn_with_decoder(grant,&mut decoder,|cx|cx.with_original_retirement(|cx|construction.as_mut().unwrap().close_step(cx))).unwrap();}assert!(construction.as_ref().unwrap().terminal_is_empty());let d=construction.as_ref().unwrap().boxed_release_demands().unwrap();caller.turn_with_decoder(grant,&mut decoder,|cx|cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{construction=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())})).unwrap();
 for _ in 0..65536{if program.original_definitions_terminal_is_empty(){break}caller.turn_with_decoder(grant,&mut decoder,|cx|cx.with_original_retirement(|cx|program.close_original_definitions(cx))).unwrap();}assert!(program.original_definitions_terminal_is_empty());while !decoder.terminal_is_empty(){caller.turn(grant,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}caller.retire(stock);
}

#[semio_framework_async_macros::async_test]
async fn raster_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_raster_raster_viewer::RasterViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn raster_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_raster_raster_editor::RasterPlayApp, semio_s_artifact_raster_raster_viewer::RasterViewer>().await;
}
