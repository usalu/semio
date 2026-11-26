#[semio_framework_async_macros::async_test]
async fn draw_viewer_never_mutates() {
    semio_plugin::artifact_app_laws::assert_viewer_never_mutates::<semio_s_artifact_draw_drawing_viewer::DrawingViewer>(super::DRAW_MOUNTED_OWNER_POLICY, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn draw_editor_and_viewer_share_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<semio_s_artifact_draw_drawing_editor::DrawingPlayApp, semio_s_artifact_draw_drawing_viewer::DrawingViewer>().await;
}

#[test]
fn draw_mounted_release_retires_atomic_serializer_within_original_grant() {
    use semio_framework_job::{CancelToken, Generation, OperationId, StepBudget, StepContext};
    use semio_framework_value::{RetainedCloneProgress, RetirementDemand};
    use semio_plugin::{MountedOwnerIdentityV1, MountedOwnerPhaseV1, MountedOwnerPolicyV1, MountedOwnerTurnV1};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("release-policy.json")).unwrap();
    assert_eq!(fixture["version"], 1);
    let baseline = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let policy = super::DRAW_MOUNTED_OWNER_POLICY.validate().unwrap();
    let mut close = baseline.close;
    close.maximum_release_bytes = fixture["maximumExportBytes"].as_u64().unwrap() as usize;
    let mut preparation = baseline.preparation;
    preparation.maximum_capacity_bytes = close.maximum_release_bytes;
    assert_eq!(policy, MountedOwnerPolicyV1 { preparation, close, ..baseline });
    let identity = MountedOwnerIdentityV1 { instance_id: 1, operation: OperationId(2), generation: Generation(3) };
    for vector in fixture["vectors"].as_array().unwrap() {
        let phase = match vector["phase"].as_str().unwrap() {
            "close" => MountedOwnerPhaseV1::Close,
            "preparation" => MountedOwnerPhaseV1::Preparation,
            "maintenance" => MountedOwnerPhaseV1::Maintenance,
            _ => unreachable!(),
        };
        let mut caller_grant = baseline.close;
        let caller_bytes = vector["callerBytes"].as_u64().unwrap() as usize;
        caller_grant.maximum_capacity_bytes = caller_bytes;
        caller_grant.maximum_release_bytes = caller_bytes;
        let mut sequence = 0;
        let mut spent = RetainedCloneProgress::default();
        let mut context = StepContext::new(identity.operation, identity.generation, StepBudget::new(8, 100, caller_grant), CancelToken::root_now(), || Some(1), &mut sequence, &mut spent);
        let turn = MountedOwnerTurnV1::admit(identity.instance_id, identity, phase, policy, &mut context).unwrap().unwrap();
        let requested = vector["requestedBytes"].as_u64().unwrap() as usize;
        let mut demand = RetirementDemand { depth: 1, ..Default::default() };
        if phase == MountedOwnerPhaseV1::Preparation {
            demand.capacity_bytes = requested;
        } else {
            demand.release_bytes = requested;
        }
        assert_eq!(turn.admits(demand).unwrap(), vector["admitted"].as_bool().unwrap(), "{vector}");
    }
}
