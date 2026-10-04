//! 🧪️ Export retains exact pixels, bounded output and cancellation without document edits.
use super::*;
use crate::standards::v1::subsets::any::schema::{create_pixel_layer,empty_raster_snapshot,snapshot::retire_raster_snapshot};

fn document(width:u32,height:u32,pixels:Vec<u8>)->RasterSnapshot {
    let mut encoder=PngEncodeJob::new(semio_framework_pixels::RasterImage {width,height,pixels}).unwrap();while !encoder.advance().unwrap().done {}
    let asset=crate::RasterImageAsset {mime:"image/png".into(),data:encoder.into_result().unwrap().data};
    let mut document=empty_raster_snapshot();document.assets.insert("image".into(),crate::mint_raster_asset_child("image",&asset)).unwrap();
    let mut layer=create_pixel_layer("Export",width,height);if let crate::RasterLayerNode::Pixel {image_key,..}=&mut layer {*image_key=Some("image".into());}document.layers.push(layer);document
}

#[test]
fn export_pages_match_neutral_base64_vectors() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(BASE64_INPUT_BYTES,fixture["base64InputBytesPerGrant"].as_u64().unwrap() as usize);assert_eq!(ArtifactOutputChunks::CHUNK_BYTES,fixture["outputChunkBytes"].as_u64().unwrap() as usize);
    for row in fixture["base64Cases"].as_array().unwrap() {let bytes:Vec<u8>=serde_json::from_value(row["bytes"].clone()).unwrap();assert_eq!(String::from_utf8(base64_output_page(&bytes).unwrap()).unwrap(),row["expected"].as_str().unwrap());}
    for row in fixture["boundaryLengths"].as_array().unwrap() {let bytes:Vec<u8>=(0..row.as_u64().unwrap() as usize).map(|i|i as u8).collect();let mut output=Vec::new();for part in bytes.chunks(BASE64_INPUT_BYTES){let chunk=base64_output_page(part).unwrap();assert!(chunk.len()<=ArtifactOutputChunks::CHUNK_BYTES);output.extend(chunk);}assert_eq!(String::from_utf8(output).unwrap(),base64_codec::base64_standard_encode(bytes));}
    assert!(base64_output_page(&vec![0;BASE64_INPUT_BYTES+1]).is_err());
}

#[test]
fn export_work_preserves_rgba_and_yields_each_stage() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let image=&fixture["image"];
    let pixels:Vec<u8>=serde_json::from_value(image["pixels"].clone()).unwrap();let document=document(2,2,pixels.clone());
    let mut work=ImageExportWork::default();assert!(work.advance(&document,0).unwrap().is_none());assert!(work.preparing.is_none());
    let mut stages=std::collections::BTreeSet::new();let mut output=Vec::new();
    for _ in 0..1000 {stages.insert(work.stage());if let Some(chunk)=work.advance(&document,1).unwrap(){assert!(chunk.len()<=ArtifactOutputChunks::CHUNK_BYTES);output.extend(chunk);}if work.done{break;}}
    assert!(work.done);for stage in fixture["stages"].as_array().unwrap(){assert!(stages.contains(stage.as_str().unwrap()));}
    let bytes=base64_codec::base64_standard_decode(std::str::from_utf8(&output).unwrap()).unwrap();let decoded=semio_framework_pixels::decode_png(&bytes).unwrap();assert_eq!((decoded.width,decoded.height),(2,2));assert_eq!(decoded.pixels,pixels);
    assert!(work.advance(&document,1).is_err());while !work.terminal_is_empty(){work.close_step(4096);}retire_raster_snapshot(document);
}

#[test]
fn export_cancels_every_stage_and_refuses_an_empty_document() {
    let document=document(128,128,(0..128*128*4).map(|i|((i*73+i/31)%256) as u8).collect());
    for stage in ["prepare","composite","encode","output"] {
        let mut work=ImageExportWork::default();for _ in 0..10000 {if work.stage()==stage{break;}work.advance(&document,1).unwrap();}assert_eq!(work.stage(),stage);
        work.closing=true;assert!(work.advance(&document,1).is_err());for _ in 0..10000 {if work.terminal_is_empty(){break;}work.close_step(4096);}assert!(work.terminal_is_empty());
    }
    retire_raster_snapshot(document);
    let empty=empty_raster_snapshot();let mut work=ImageExportWork::default();let mut refused=false;for _ in 0..100 {if work.advance(&empty,1).is_err(){refused=true;break;}}assert!(refused);while !work.terminal_is_empty(){work.close_step(4096);}retire_raster_snapshot(empty);
}

#[semio_framework_async_macros::async_test]
async fn retained_image_export_completes_or_cancels_without_mutating_history() {
    use semio_framework_plugin::{PluginApp,ArtifactMediaExportPoll};
    use crate::editor::raster::unit_tests::context;
    let mut app=context::app().await;
    context::dispatch(&mut app,crate::editor::raster::RasterCommand::AddLayer(crate::editor::raster::commands::add_layer::AddLayer {kind:"pixel".into()})).await;
    let before=app.snapshot().unwrap();
    let handle=app.submit_media_export("image:out").await.expect("registered retained export");let mut running=0;let mut progress=0;let mut result=None;
    for _ in 0..100000 {semio_framework_async::yield_once().await;match app.poll_media_export(&handle).await.unwrap(){ArtifactMediaExportPoll::Running {applied_progress,..}=>{assert!(applied_progress>=progress);progress=applied_progress;running+=1;},ArtifactMediaExportPoll::Complete(media)=>{result=Some(media);break;},other=>panic!("export did not complete: {other:?}")}}
    assert!(running>1);assert!(progress>0);let media=result.expect("complete export");assert_eq!(media.schema,"2d.image");let mut output=Vec::new();while let Some(chunk)=media.chunks.take_chunk().unwrap(){assert!(chunk.len()<=ArtifactOutputChunks::CHUNK_BYTES);output.extend(chunk);}
    let bytes=base64_codec::base64_standard_decode(std::str::from_utf8(&output).unwrap()).unwrap();let image=semio_framework_pixels::decode_png(&bytes).unwrap();assert_eq!((image.width,image.height),(512,512));assert!(image.pixels.iter().all(|value|*value==0));
    let handle=app.submit_media_export("image:out").await.unwrap();app.cancel_media_export(&handle).await.unwrap();
    assert!(app.poll_media_export(&handle).await.is_err(),"cancelled media handles transfer to the close lane immediately");
    let after=app.snapshot().unwrap();assert_eq!(before,after);retire_raster_snapshot(before);retire_raster_snapshot(after);
    context::history(&mut app,"undo").await;let undone=app.snapshot().unwrap();assert!(undone.layers.is_empty());retire_raster_snapshot(undone);
    drop(media);
    for _ in 0..100000 {if app.close_terminal_is_empty(){break;}app.close_step(1,16384).unwrap();semio_framework_async::yield_once().await;}
    assert!(app.close_terminal_is_empty(),"completed and cancelled export owners must retire");
}

#[test]
fn export_progress_uses_neutral_localized_stage_labels() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for stage in fixture["stages"].as_array().unwrap() {let mut publication=Publication::progress(stage.as_str().unwrap(),7);let payload:serde_json::Value=serde_json::from_slice(&publication.bytes).unwrap();assert_eq!(payload["stage"],*stage);assert_eq!(payload["completedUnits"],7);for locale in ["en","de"]{assert_eq!(payload[locale],fixture["labels"][stage.as_str().unwrap()][locale]);}let mut writer=publication.writer.take().unwrap();writer.begin_close();while !writer.terminal_is_empty(){writer.close_step(1,4096);}}
}

#[semio_framework_async_macros::async_test]
async fn retained_export_refuses_a_superseded_document_and_retires_its_output() {
    use semio_framework_plugin::{PluginApp,ArtifactMediaExportPoll};
    use crate::editor::raster::{unit_tests::context,RasterCommand,commands::{add_layer,patch_layer}};
    let mut app=context::app().await;context::dispatch(&mut app,RasterCommand::AddLayer(add_layer::AddLayer {kind:"pixel".into()})).await;
    let document=app.snapshot().unwrap();let id=crate::standards::v1::subsets::any::schema::layer_node_id(&document.layers[0]).to_owned();retire_raster_snapshot(document);
    let handle=app.submit_media_export("image:out").await.unwrap();
    context::dispatch(&mut app,RasterCommand::PatchLayer(patch_layer::PatchLayer {layer_id:id,field:"name".into(),value:"Changed during export".into()})).await;
    let mut refused=false;for _ in 0..100000 {semio_framework_async::yield_once().await;match app.poll_media_export(&handle).await.unwrap(){ArtifactMediaExportPoll::Running {..}=>{},ArtifactMediaExportPoll::Failed(detail)=>{assert!(detail.contains("stale revision or generation"),"{detail}");refused=true;break;},other=>panic!("superseded export escaped: {other:?}")}}
    assert!(refused);let document=app.snapshot().unwrap();assert_eq!(crate::standards::v1::subsets::any::schema::layer_name(&document.layers[0]),"Changed during export");retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn png_command_download_survives_operation_retirement_and_preserves_history() {
    use semio_framework_plugin::{PluginApp,artifact_app_laws,app::TypedOperationResultLane};
    use crate::editor::raster::{unit_tests::context,RasterCommand,commands::{add_layer,export_png}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let expected=&fixture["download"];
    let command=<RasterPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action(expected["action"].as_str().unwrap(),None).unwrap();assert!(matches!(command,RasterCommand::ExportPng(_)));
    store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    let mut app=context::app().await;context::dispatch(&mut app,RasterCommand::AddLayer(add_layer::AddLayer {kind:"pixel".into()})).await;
    let before=app.snapshot().unwrap();let mut meta=artifact_app_laws::meta("local");meta.view_state=Some(context::raster_view_state());let instance=meta.instance_id;
    let admission=app.dispatch_typed(RasterCommand::ExportPng(export_png::ExportPng {}),&meta).await.unwrap();assert!(admission.mutations.is_empty());
    let mut download=None;let mut turns=0;
    while app.has_pending_typed_operations() {
        turns+=1;assert!(turns<100000);semio_framework_async::yield_once().await;
        app.maintenance_step(1,16384).unwrap();app.advance_typed_operation_publication().await.unwrap();
        if let Some(page)=app.take_typed_operation_result_page(instance) {
            assert_ne!(page.lane,TypedOperationResultLane::Fault,"{}",String::from_utf8_lossy(page.bytes()));
            if page.lane==TypedOperationResultLane::Download {assert!(download.is_none());let row:serde_json::Value=serde_json::from_slice(page.bytes()).unwrap();download=Some((page.token.operation,row));}
            assert!(app.acknowledge_typed_operation_result(page.token).unwrap());
        }
        let _=app.take_typed_operation_effect();let _=app.take_typed_operation_event();let _=app.take_typed_operation_completion().await.unwrap();let _=app.take_typed_operation_ui_scope();
    }
    let (operation,row)=download.expect("PNG command publishes a download");assert_eq!(row[0],expected["filename"]);assert_eq!(row[1],expected["mime"]);assert_eq!(row[2],expected["encoding"]);
    let mut bytes=Vec::new();while let Some(chunk)=app.take_segmented_download_chunk(operation).await.unwrap(){assert!(chunk.len()<=ArtifactOutputChunks::CHUNK_BYTES);bytes.extend(chunk);}
    assert_eq!(bytes.len(),row[3].as_u64().unwrap() as usize);assert!(!bytes.is_empty());
    let png=base64_codec::base64_standard_decode(std::str::from_utf8(&bytes).unwrap()).unwrap();let image=semio_framework_pixels::decode_png(&png).unwrap();assert_eq!((image.width,image.height),(512,512));assert!(image.pixels.iter().all(|value|*value==0));
    let after=app.snapshot().unwrap();assert_eq!(before,after);retire_raster_snapshot(before);retire_raster_snapshot(after);
    context::history(&mut app,"undo").await;let undone=app.snapshot().unwrap();assert!(undone.layers.is_empty());retire_raster_snapshot(undone);
}

#[semio_framework_async_macros::async_test]
async fn cancelled_png_command_never_publishes_a_download_or_changes_history() {
    use semio_framework_plugin::{PluginApp,artifact_app_laws,app::{TypedOperationResultLane,ArtifactDocumentAuthority}};
    use crate::editor::raster::{unit_tests::context,RasterCommand,commands::{add_layer,export_png}};
    for cancel_after in [0,8] {
        let mut app=context::app().await;context::dispatch(&mut app,RasterCommand::AddLayer(add_layer::AddLayer {kind:"pixel".into()})).await;
        let before=app.snapshot().unwrap();let mut meta=artifact_app_laws::meta("local");meta.view_state=Some(context::raster_view_state());
        app.dispatch_typed(RasterCommand::ExportPng(export_png::ExportPng {}),&meta).await.unwrap();let cancellation=app.tool_cancellation_handle();let mut cancelled=false;let mut turns=0;
        while app.has_pending_typed_operations() {
            if turns==cancel_after {assert!(cancellation.cancel_document(ArtifactDocumentAuthority(meta.instance_id)).unwrap());cancelled=true;}
            turns+=1;assert!(turns<100000);semio_framework_async::yield_once().await;
            app.maintenance_step(1,16384).unwrap();app.advance_typed_operation_publication().await.unwrap();
            if let Some(page)=app.take_typed_operation_result_page(meta.instance_id) {assert_ne!(page.lane,TypedOperationResultLane::Download);assert!(app.acknowledge_typed_operation_result(page.token).unwrap());}
            let _=app.take_typed_operation_effect();let _=app.take_typed_operation_event();let _=app.take_typed_operation_completion().await.unwrap();let _=app.take_typed_operation_ui_scope();
        }
        assert!(cancelled);let after=app.snapshot().unwrap();assert_eq!(before,after);retire_raster_snapshot(before);retire_raster_snapshot(after);
        context::history(&mut app,"undo").await;let undone=app.snapshot().unwrap();assert!(undone.layers.is_empty());retire_raster_snapshot(undone);
    }
}

#[semio_framework_async_macros::async_test]
async fn png_progress_control_cancels_its_exact_visible_operation() {
    use semio_framework_plugin::PluginApp;
    use semio_framework_plugin::artifact_app_laws;
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_ui_locale::Locale;
    use crate::editor::raster::{unit_tests::context,RasterCommand,commands::{add_layer,export_png},panels::document::RASTER_PLAY_BODY_LAYERS};
    fn find_identity(value:&serde_json::Value)->Option<serde_json::Value> {match value{serde_json::Value::Object(map)=>{if map.get("operationId").is_some_and(serde_json::Value::is_string)&&map.get("generation").is_some_and(serde_json::Value::is_string){return Some(value.clone());}map.values().find_map(find_identity)},serde_json::Value::Array(items)=>items.iter().find_map(find_identity),_=>None}}
    let mut app=context::app().await;context::dispatch(&mut app,RasterCommand::AddLayer(add_layer::AddLayer {kind:"pixel".into()})).await;
    let mut meta=artifact_app_laws::meta("local");meta.view_state=Some(context::raster_view_state());app.dispatch_typed(RasterCommand::ExportPng(export_png::ExportPng {}),&meta).await.unwrap();
    let mut identity=None;
    for _ in 0..100000 {semio_framework_async::yield_once().await;app.maintenance_step(1,16384).unwrap();app.advance_typed_operation_publication().await.unwrap();if app.take_typed_operation_ui_scope().is_some(){let view=context::raster_view_state();let json=context::render_with_view(&mut app,RASTER_PLAY_BODY_LAYERS,&view).await;if json.contains("cancelTypedOperation"){assert!(json.contains("Cancel"));identity=find_identity(&serde_json::from_str(&json).unwrap());let mut german=view;german.locale=Locale::De;let json=context::render_with_view(&mut app,RASTER_PLAY_BODY_LAYERS,&german).await;assert!(json.contains("Abbrechen"));assert!(json.contains("Abgeschlossene Schritte"));break;}}}
    let identity=identity.expect("visible progress carries exact cancellation arguments");let mut wrong=identity.clone();wrong["generation"]=serde_json::Value::String("ffffffffffffffff".into());let wrong=semio_framework_pack_json::from_json_str(&wrong.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert!(app.handle_action("cancelTypedOperation",Some(&wrong),&meta).await.is_err());
    let args=semio_framework_pack_json::from_json_str(&identity.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut foreign=meta.clone();foreign.actor="foreign".into();assert!(app.handle_action("cancelTypedOperation",Some(&args),&foreign).await.is_err());
    foreign=meta.clone();foreign.instance_id+=1;assert!(app.handle_action("cancelTypedOperation",Some(&args),&foreign).await.is_err());
    app.handle_action("cancelTypedOperation",Some(&args),&meta).await.unwrap();app.handle_action("cancelTypedOperation",Some(&args),&meta).await.unwrap();
    let cancelling=context::render_with_view(&mut app,RASTER_PLAY_BODY_LAYERS,&context::raster_view_state()).await;assert!(cancelling.contains("Cancelling"));
    let mut terminal=false;let mut retired_refresh=false;let mut turns=0;while app.has_pending_typed_operations(){turns+=1;assert!(turns<100000);semio_framework_async::yield_once().await;app.maintenance_step(1,16384).unwrap();app.advance_typed_operation_publication().await.unwrap();if let Some(page)=app.take_typed_operation_result_page(meta.instance_id){assert_ne!(page.lane,TypedOperationResultLane::Download);assert_ne!(page.lane,TypedOperationResultLane::Fault,"an explicit Cancel is a normal terminal outcome");terminal|=page.lane==TypedOperationResultLane::Terminal;assert!(app.acknowledge_typed_operation_result(page.token).unwrap());}let _=app.take_typed_operation_effect();let _=app.take_typed_operation_event();let _=app.take_typed_operation_completion().await.unwrap();if app.take_typed_operation_ui_scope().is_some()&&app.live_typed_operation_slots()==0{retired_refresh=true;}}
    assert!(terminal);assert!(retired_refresh,"retiring the final operation must still publish a UI refresh");
    let json=context::render_with_view(&mut app,RASTER_PLAY_BODY_LAYERS,&context::raster_view_state()).await;assert!(!json.contains("cancelTypedOperation"));context::history(&mut app,"undo").await;let document=app.snapshot().unwrap();assert!(document.layers.is_empty());retire_raster_snapshot(document);
}
#[test]
fn export_snapshot_disposal_obeys_grants_and_retires_the_last_shared_owner() {
    use semio_framework_plugin::{ArtifactEditor,PluginCloseStep};
    let snapshot=std::sync::Arc::new(crate::standards::v1::subsets::any::schema::semio_fixture_snapshot());
    let mut aliases=[Some(snapshot.clone()),Some(snapshot)];
    let mut disposers=[RasterPlayApp::build_snapshot_disposer().unwrap(),RasterPlayApp::build_snapshot_disposer().unwrap()];
    for index in 0..2 {
        assert_eq!(disposers[index].close_step(&mut aliases[index],0,16384).unwrap(),PluginCloseStep::Pending {released_items:0,released_bytes:0});
        assert!(aliases[index].is_some());
        let mut complete=false;
        for _ in 0..100000 {
            match disposers[index].close_step(&mut aliases[index],1,16384).unwrap() {
                PluginCloseStep::Complete=>{complete=true;break;},
                PluginCloseStep::Pending {released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=16384);},
                other=>panic!("export snapshot failed to retire its owned alias: {other:?}"),
            }
        }
        assert!(complete);assert!(disposers[index].terminal_is_empty(&aliases[index]));
    }
}
