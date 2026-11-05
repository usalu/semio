use super::*;
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan;
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;

fn rows(count:usize)->Vec<RasterSelectionSpan>{(0..count).map(|start|RasterSelectionSpan{start:start as u32,length:1,coverage:128}).collect()}
fn selected(rows:Vec<RasterSelectionSpan>)->RasterPixelSelection{let mut stock=OriginalWorkFixture::stock();stock.maximum_items=8*1024*1024;let mut caller=OriginalWorkFixture::with_source(semio_framework_job::OperationId(9697),semio_framework_job::Generation(17),stock,||Some(1),u64::MAX,stock.maximum_items as u64*16);let spans=caller.receive_native(rows,stock);caller.finish();RasterPixelSelection{layer_id:"p".into(),target:"pixels".into(),width:256,height:256,spans}}
fn selection(count:usize)->RasterPixelSelection{selected(rows(count))}
fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T){let mut fixture=OriginalWorkFixture::new();fixture.retire_owned(value,OriginalWorkFixture::stock());fixture.finish();}

#[test]
fn large_selection_uses_its_construction_proof_before_root_exchange(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../fixture.json")).unwrap();
    let mutation=RasterConfigMutation::SetPixelSelection(SetPixelSelectionEdit{selection:Some(selection(fixture["spans"].as_u64().unwrap()as usize))});
    let post=RasterConfig::default();let before=post.clone();let mut cursor=0;let body=HEADER;
    let demand=mutation.validation_demands(&post,cursor,body).unwrap();assert_eq!(demand.copy_bytes,body);assert_eq!(demand.depth,1);let bound=mutation.validation_work_demand(&post,cursor,body).unwrap();assert!(bound<=fixture["maximumValidationWork"].as_u64().unwrap());let turn=mutation.validate_step(&post,&mut cursor,body);assert!(turn.work>0&&turn.work<=bound);let(done,progress)=turn.result.unwrap();assert!(done);assert_eq!(cursor,65536);assert_eq!(progress.copied_bytes,HEADER);assert_eq!(post,before);
    let mut candidate=before.clone();let inverse=mutation.exchange(&mut candidate).unwrap();assert_eq!(candidate.pixel_selection.as_ref().unwrap().spans.len(),65536);retire(inverse.exchange(&mut candidate).unwrap());assert_eq!(candidate,before);retire(candidate);retire(before);retire(post);
}

#[test]
fn malformed_late_rows_refuse_during_original_construction(){
    for (index,coverage) in [(4096,128),(8192,256)]{
        let mut payload=rows(16384);if coverage==256{payload[index].coverage=coverage;}else{payload[index].start-=1;}
        let post=RasterConfig::default();let mut caller=OriginalWorkFixture::new();let error=caller.try_receive_native::<_,crate::selection::RasterSelectionSpans>(payload,OriginalWorkFixture::stock()).err().expect("malformed coverage cannot become an immutable root");caller.retire_owned(error,OriginalWorkFixture::stock());caller.finish();assert!(post.pixel_selection.is_none());retire(post);
    }
}

#[test]
fn target_change_keeps_two_inverse_rows_by_moving_the_original_coverage(){
    let mut post=RasterConfig{pixel_selection:Some(selection(4096)),..Default::default()};let before=post.clone();let pointer=post.pixel_selection.as_ref().unwrap().spans.get(0).unwrap()as*const RasterSelectionSpan;
    let inverse=RasterConfigMutation::SetPaintTarget(SetPaintTargetEdit{value:"mask".into()}).exchange(&mut post).unwrap();let followup=inverse.take_followup_inverse(&mut post).unwrap();assert!(post.pixel_selection.is_none());
    let RasterConfigMutation::SetPixelSelection(edit)=&followup else{panic!("selection inverse")};assert_eq!(edit.selection.as_ref().unwrap().spans.get(0).unwrap()as*const RasterSelectionSpan,pointer);
    retire(inverse.exchange(&mut post).unwrap());retire(followup.exchange(&mut post).unwrap());assert_eq!(post,before);retire(post);retire(before);
}

#[test]
fn cancelled_config_cursor_retires_without_changing_the_committed_owner(){
    use store::snapshot_clone_preparation::{ConfigApplyEdit,RetainedCloneEdit,RetainedCloneEditCursor};
    let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
    let edit=ConfigApplyEdit::<RasterConfig,RasterConfigMutation>::new();let(mut cursor,_)=edit.begin(grant).unwrap();let post=RasterConfig{pixel_selection:Some(selection(4096)),..Default::default()};let before=post.clone();cursor.cancel();cursor.begin_close();for _ in 0..10000{if cursor.terminal_is_empty(){break;}cursor.close_step(grant).unwrap();}assert!(cursor.terminal_is_empty());assert_eq!(post,before);retire(post);retire(before);
}
