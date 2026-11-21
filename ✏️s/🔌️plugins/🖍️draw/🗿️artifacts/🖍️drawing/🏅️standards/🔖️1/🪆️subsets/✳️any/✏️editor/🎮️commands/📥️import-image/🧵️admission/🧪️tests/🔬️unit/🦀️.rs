//! 🧪️ Retained image decoding defers publication and closes every child at its real grant.
use semio_framework_test_fixtures::fixture_text;
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, OriginalJobFixtureCaller};
use semio_plugin::NoConfig;

#[allow(unused_imports)] use semio_s_artifact_draw_drawing_editor::{DRAWING_BOUNDED_PAYLOAD_SCHEMA, DRAWING_IMAGE_PUBLICATION_CONTRACTS, DRAWING_IMAGE_RAW_BYTES, DRAWING_IMAGE_TOOL_IDS, DrawingCommand, DrawingPlayApp};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_editor_commands::import_image;
#[allow(unused_imports)] use semio_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep};
#[allow(unused_imports)] use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep},ValueError};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_editor::image_job::*;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1, maximum_copy_bytes: 1 << 18, maximum_capacity_bytes: 1 << 29,
    maximum_release_bytes: 1 << 29, maximum_depth: 128,
};

fn job_caller(operation: &semio_plugin::AppOperationContext) -> OriginalJobFixtureCaller {
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 29, ..TURN };
    OriginalJobFixtureCaller::admit(
        OperationId(operation.operation_id), Generation(operation.generation),
        stock, || Some(0), u64::MAX, 1 << 24,
    )
}

fn close(work: &mut Work, mut caller: OriginalJobFixtureCaller) {
    work.begin_close();
    let zero=caller.turn(RetainedCloneGrant::default(), |cx| cx.with_original_retirement(|cx| work.close_step(cx))).expect("original fixture close turn");
    assert!(matches!(zero,InteractiveJobCloseStep::Pending {progress}|InteractiveJobCloseStep::Complete {progress} if progress==Default::default()));
    for _ in 0..20000 {
        let copy=work.next_close_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:work.next_close_capacity_byte_demand(if copy>0{copy}else{work.next_close_release_byte_demand().unwrap()}).unwrap(),maximum_release_bytes:work.next_close_release_byte_demand().unwrap(),maximum_depth:work.next_close_depth_demand().unwrap()};
        match caller.turn(grant, |cx| cx.with_original_retirement(|cx| work.close_step(cx))).expect("original fixture close turn") {InteractiveJobCloseStep::Pending {progress}=>assert!(progress.fits(grant)),InteractiveJobCloseStep::Complete {progress}=>{assert!(progress.fits(grant));assert!(work.terminal_is_empty());caller.retire_exact();return;},other=>panic!("Unexpected image close: {other:?}")}
    }
    panic!("Image close did not finish");
}
#[test]
fn retained_image_import_shared_cases() {
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🎮️commands/📥️import-image/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {
  let snapshot=DrawingSnapshot::default();let saved=snapshot.clone();let command=DrawingCommand::ImportImage(import_image::ImportImage {payload:row["input"]["payload"].as_str().unwrap().into(),name:Some("Artwork.png".into()),parent_id:None,index:None});
  let config=NoConfig {};let history=semio_plugin::HistoryView::empty();let interaction=protocol::InteractionState::default();let hover=semio_plugin::app::InteractionHoverState::new();
  let operation=semio_plugin::AppOperationContext {app_instance_id:1,parent_document_id:"images".into(),operation_id:1,generation:1,canonical_base_revision:[0;32],retained:semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation,authoring_seed:"images".into()};
  let input=ArtifactCommandInputs::<App> {command:&command,snapshot:&snapshot,snapshot_owner:None,snapshot_read:None,config:&config,history:&history,interaction:&interaction,hover:&hover,context:None,operation:&operation};
  for stop in [0,1,25,100,usize::MAX] {
   let mut work=Work::new();let mut completed=false;
   let mut caller = job_caller(&operation);
   for at in 0..100000 {if at==stop {break;}
    match caller.turn(TURN, |cx| work.step(&input, cx)).unwrap() {ArtifactCommandWorkStep::Progress {..}=>{},ArtifactCommandWorkStep::Complete(emit)=>{assert_eq!(emit.artifact_mutations.len(),2);let mut after=snapshot.clone();for mutation in emit.artifact_mutations {semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,&mutation).unwrap();}let asset=after.assets.values().next().unwrap();assert_eq!(serde_json::to_value(asset).unwrap(),row["expected"]);completed=true;break;},_=>panic!("Unexpected image import result")}
    assert_eq!(snapshot,saved);
   }
   if stop==usize::MAX {assert!(completed);}assert_eq!(snapshot,saved);close(&mut work, caller);
  }
 }
}
