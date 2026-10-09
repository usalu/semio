//! 🧪️ Retained image decoding defers publication and closes every child at its real grant.
use super::*;
fn close(work:&mut Work) {
    use semio_framework_job::InteractiveJobCloseStep;
    work.begin_close();
    let zero=work.close_step(RetainedCloneGrant::default());
    assert!(matches!(zero,InteractiveJobCloseStep::Pending {progress}|InteractiveJobCloseStep::Complete {progress} if progress==Default::default()));
    for _ in 0..20000 {
        let copy=work.next_close_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:work.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:work.next_close_release_byte_demand().unwrap(),maximum_depth:work.next_close_depth_demand().unwrap()};
        match work.close_step(grant) {InteractiveJobCloseStep::Pending {progress}=>assert!(progress.fits(grant)),InteractiveJobCloseStep::Complete {progress}=>{assert!(progress.fits(grant));assert!(work.terminal_is_empty());return;},other=>panic!("Unexpected image close: {other:?}")}
    }
    panic!("Image close did not finish");
}
#[test]
fn retained_image_import_shared_cases() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {
  let snapshot=DrawingSnapshot::default();let saved=snapshot.clone();let command=DrawingCommand::ImportImage(import_image::ImportImage {payload:row["input"]["payload"].as_str().unwrap().into(),name:Some("Artwork.png".into()),parent_id:None,index:None});
  let config=NoConfig {};let history=semio_framework_plugin::HistoryView::empty();let interaction=protocol::InteractionState::default();let hover=semio_framework_plugin::app::InteractionHoverState::new();
  let operation=semio_framework_plugin::AppOperationContext {app_instance_id:1,parent_document_id:"images".into(),operation_id:1,generation:1,canonical_base_revision:[0;32],authoring_seed:"images".into()};
  let input=ArtifactCommandInputs::<App> {command:&command,snapshot:&snapshot,snapshot_owner:None,config:&config,history:&history,interaction:&interaction,hover:&hover,context:None,operation:&operation};
  let cancel=semio_framework_job::root_cancel_token();let mut sequence=0;
  for stop in [0,1,25,100,usize::MAX] {
   let mut work=Work::new();let mut completed=false;
   for at in 0..100000 {if at==stop {break;}
    let mut cx=semio_framework_job::StepContext::new(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),cancel.clone(),semio_framework_job::default_now_us,&mut sequence);
    match work.step(&input,&mut cx).unwrap() {ArtifactCommandWorkStep::Progress {..}=>{},ArtifactCommandWorkStep::Complete(emit)=>{assert_eq!(emit.mutations.len(),2);let mut after=snapshot.clone();for mutation in emit.mutations {crate::mutations::apply_drawing_mutation(&mut after,&mutation).unwrap();}let asset=after.assets.values().next().unwrap();assert_eq!(serde_json::to_value(asset).unwrap(),row["expected"]);completed=true;break;},_=>panic!("Unexpected image import result")}
    assert_eq!(snapshot,saved);
   }
   if stop==usize::MAX {assert!(completed);}assert_eq!(snapshot,saved);close(&mut work);
  }
 }
 eprintln!("[DEBUG] Retained image import deferred publication, original source and exact child retirement laws executed");
}
