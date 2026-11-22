//! 🧪️ Shared simplification cases publish only after cooperative work and retire at exact grants.
use semio_framework_test_fixtures::fixture_text;
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, OriginalJobFixtureCaller};
use semio_plugin::NoConfig;

#[allow(unused_imports)] use semio_s_artifact_draw_drawing_editor::{DRAWING_BOUNDED_PAYLOAD_SCHEMA, DRAWING_PATH_PUBLICATION_CONTRACTS, DRAWING_PATH_TOOL_IDS, DrawingCommand, DrawingPlayApp, DrawingWindowCommandWork};
#[allow(unused_imports)] #[cfg(test)] use semio_s_artifact_draw_drawing_editor_commands::edit_path;
#[allow(unused_imports)] use semio_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep};
#[allow(unused_imports)] use semio_framework_value::{retirement::controlled::ControlledRetirement, retained_clone::{RetainedCloneGrant, RetainedCloneStep}, ValueError};
#[allow(unused_imports)] use semio_framework_value_persistent::list::PagedList;
use semio_s_artifact_draw_drawing_model::{snapshot::DrawingSnapshot, DrawingLayerNode};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_editor::path_job::*;

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
    assert!(matches!(zero,InteractiveJobCloseStep::Pending {progress} if progress==Default::default()));
    for _ in 0..20000 {
        let copy=work.next_close_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:work.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:work.next_close_release_byte_demand().unwrap(),maximum_depth:work.next_close_depth_demand().unwrap()};
        match caller.turn(grant, |cx| cx.with_original_retirement(|cx| work.close_step(cx))).expect("original fixture close turn") {InteractiveJobCloseStep::Pending {progress}=>assert!(progress.fits(grant)),InteractiveJobCloseStep::Complete {progress}=>{assert!(progress.fits(grant));assert!(work.terminal_is_empty());caller.retire_exact();return;},other=>panic!("Unexpected path close: {other:?}")}
    }
    panic!("Path close did not finish");
}
#[test]
fn retained_path_simplification_shared_cases() {
    let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🧮️geometry/✏️editing/🎮️operations/🎛️algorithms/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap().iter().filter(|case|case["operation"]["kind"]=="simplify"&&case["error"]!=true) {
        let source:Vec<semio_s_artifact_draw_drawing_model::PathSegment>=serde_json::from_value(case["before"].clone()).unwrap();
        let layer=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Simplify")).to_string().into()).expect("nonempty authored identity"), "Simplify",source.into());let id=semio_s_artifact_draw_drawing_model::layer_base(&layer).id.clone();
        let snapshot=DrawingSnapshot {layers:vec![layer].into(),..Default::default()};let saved=snapshot.clone();
        let command=DrawingCommand::EditPath(edit_path::EditPath {layer_id:id.to_string_owner(),edit:Box::new(serde_json::from_value(case["operation"].clone()).unwrap())});
        let config=NoConfig {};let history=semio_plugin::HistoryView::empty();let interaction=protocol::InteractionState::default();let hover=semio_plugin::app::InteractionHoverState::new();
        let operation=semio_plugin::AppOperationContext {app_instance_id:1,parent_document_id:"simplify".into(),operation_id:1,generation:1,canonical_base_revision:[0;32],retained:semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation,authoring_seed:"simplify".into()};
        let input=ArtifactCommandInputs::<App> {command:&command,snapshot:&snapshot,snapshot_owner:None,snapshot_read:semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation),config:&config,history:&history,interaction:&interaction,hover:&hover,context:None,operation:&operation};
        let mut work=Work::new();let mut progressed=false;let mut completed=false;
        let mut caller = job_caller(&operation);
        for _ in 0..20000 {
            match caller.turn(TURN, |cx| work.step(&input, cx)).unwrap() {
                ArtifactCommandWorkStep::Progress {..}=>progressed=true,
                ArtifactCommandWorkStep::Complete(emit)=>{
                    let mut after=snapshot.clone();for mutation in emit.artifact_mutations {semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,&mutation).unwrap();}
                    let DrawingLayerNode::Path(path)=after.layers.get(0).unwrap() else {unreachable!()};
                    let expected:Vec<semio_s_artifact_draw_drawing_model::PathSegment>=serde_json::from_value(case["after"].clone()).unwrap();assert_eq!(path.segments.iter().cloned().collect::<Vec<_>>(),expected);completed=true;break;
                },_=>panic!("Unexpected retained path result"),
            }
            assert_eq!(snapshot,saved);
        }
        assert!(progressed&&completed);assert_eq!(snapshot,saved);close(&mut work, caller);
        let mut interrupted = Work::new();
        let mut caller = job_caller(&operation);
        assert!(matches!(caller.turn(TURN, |cx| interrupted.step(&input, cx)).unwrap(), ArtifactCommandWorkStep::Progress {..}));
        close(&mut interrupted, caller);assert_eq!(snapshot,saved);
    }
}
