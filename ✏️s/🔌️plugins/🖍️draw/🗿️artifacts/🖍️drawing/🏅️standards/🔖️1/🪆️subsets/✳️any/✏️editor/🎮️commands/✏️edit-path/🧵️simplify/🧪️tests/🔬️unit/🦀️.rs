//! 🧪️ Shared simplification cases publish only after cooperative work and retire at exact grants.
use super::*;
fn close(work:&mut Work) {
    use semio_framework_job::InteractiveJobCloseStep;
    work.begin_close();
    let zero=work.close_step(RetainedCloneGrant::default());
    assert!(matches!(zero,InteractiveJobCloseStep::Pending {progress} if progress==Default::default()));
    for _ in 0..20000 {
        let copy=work.next_close_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:work.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:work.next_close_release_byte_demand().unwrap(),maximum_depth:work.next_close_depth_demand().unwrap()};
        match work.close_step(grant) {InteractiveJobCloseStep::Pending {progress}=>assert!(progress.fits(grant)),InteractiveJobCloseStep::Complete {progress}=>{assert!(progress.fits(grant));assert!(work.terminal_is_empty());return;},other=>panic!("Unexpected path close: {other:?}")}
    }
    panic!("Path close did not finish");
}
#[test]
fn retained_path_simplification_shared_cases() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧬️schema/🧮️geometry/✏️editing/🧫️fixtures/🎛️algorithms/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap().iter().filter(|case|case["operation"]["kind"]=="simplify"&&case["error"]!=true) {
        let source:Vec<crate::PathSegment>=serde_json::from_value(case["before"].clone()).unwrap();
        let layer=crate::schema::create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Simplify")).to_string().into()).expect("nonempty authored identity"), "Simplify",source.into());let id=crate::schema::layer_base(&layer).id.clone();
        let snapshot=DrawingSnapshot {layers:vec![layer].into(),..Default::default()};let saved=snapshot.clone();
        let command=DrawingCommand::EditPath(edit_path::EditPath {layer_id:id,edit:Box::new(serde_json::from_value(case["operation"].clone()).unwrap())});
        let config=NoConfig {};let history=semio_framework_plugin::HistoryView::empty();let interaction=protocol::InteractionState::default();let hover=semio_framework_plugin::app::InteractionHoverState::new();
        let operation=semio_framework_plugin::AppOperationContext {app_instance_id:1,parent_document_id:"simplify".into(),operation_id:1,generation:1,canonical_base_revision:[0;32],authoring_seed:"simplify".into()};
        let input=ArtifactCommandInputs::<App> {command:&command,snapshot:&snapshot,snapshot_owner:None,config:&config,history:&history,interaction:&interaction,hover:&hover,context:None,operation:&operation};
        let mut work=Work::new();let cancel=semio_framework_job::root_cancel_token();let mut sequence=0;let mut progressed=false;let mut completed=false;
        for _ in 0..20000 {
            let mut cx=semio_framework_job::StepContext::new(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),cancel.clone(),semio_framework_job::default_now_us,&mut sequence);
            match work.step(&input,&mut cx).unwrap() {
                ArtifactCommandWorkStep::Progress {..}=>progressed=true,
                ArtifactCommandWorkStep::Complete(emit)=>{
                    let mut after=snapshot.clone();for mutation in emit.mutations {crate::mutations::apply_drawing_mutation(&mut after,&mutation).unwrap();}
                    let DrawingLayerNode::Path(path)=after.layers.get(0).unwrap() else {unreachable!()};
                    let expected:Vec<crate::PathSegment>=serde_json::from_value(case["after"].clone()).unwrap();assert_eq!(path.segments.iter().cloned().collect::<Vec<_>>(),expected);completed=true;break;
                },_=>panic!("Unexpected retained path result"),
            }
            assert_eq!(snapshot,saved);
        }
        assert!(progressed&&completed);assert_eq!(snapshot,saved);close(&mut work);
        let mut interrupted=Work::new();let mut cx=semio_framework_job::StepContext::new(semio_framework_job::OperationId(2),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),cancel.clone(),semio_framework_job::default_now_us,&mut sequence);
        assert!(matches!(interrupted.step(&input,&mut cx).unwrap(),ArtifactCommandWorkStep::Progress {..}));close(&mut interrupted);assert_eq!(snapshot,saved);
    }
    eprintln!("[DEBUG] Retained path simplification fixture, deferred publication and exact retirement laws executed");
}
