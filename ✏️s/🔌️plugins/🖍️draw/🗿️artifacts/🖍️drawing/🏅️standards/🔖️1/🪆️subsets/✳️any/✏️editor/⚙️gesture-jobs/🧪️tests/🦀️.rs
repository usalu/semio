//! ♻️ Original completion rejection closes before the gesture wire decoder.

use super::*;
use semio_framework::ToolJobFactory as _;
use semio_framework_job::{Generation, InteractiveJob as _, InteractiveJobCloseStep, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin::{Emit, NoConfigMutation, NoDraftMutation};

fn grant(items: usize, bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 32 }
}

fn released(items: usize, bytes: usize) -> InteractiveJobCloseStep {
    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() } }
}

#[test]
fn drawing_completion_rejection_retires_child_before_decoder_without_redispatch() {
    let mut emit: Emit<DrawingMutation, NoConfigMutation, NoDraftMutation> = Emit::default();
    emit.child_emits.push(semio_plugin::app::ChildEmit::open("member","drawing-child",0));
    let rejected = semio_plugin::app::ArtifactToolCompletionRejection::<semio_plugin::EditorApp<DrawingPlayApp>> {
        emit: Ok(emit),
        ephemeral: semio_plugin::EphemeralEmit::default(),
        fault: Fault::from("injected completion rejection"),
    };
    let mut caller = OriginalJobFixtureCaller::admit(
        OperationId(1), Generation(1), grant(1 << 20, 1 << 30), || Some(0), u64::MAX, 1 << 20,
    );
    let mut job = DrawingGestureOperationJob {
        payload: None,
        retirement: None,
        pending_completion_rejection: Some(rejected),
        raw_input: None,
        raw_page_cursor: 0,
        raw_byte_cursor: 0,
        decoder: Some(DrawingRetainedCommandDecoder::new("canvasEscape")),
        raw_validated: true,
        completed: false,
        closing: false,
        outcomes: semio_s_artifact_draw_drawing_host::outcome::DrawingJobOutcomes::new(),
    };
    job.begin_close();
    assert_eq!(caller.turn(grant(0, 1), |cx| job.close_step(cx)).expect("original fixture close turn"), released(0, 0));
    assert!(job.pending_completion_rejection.is_some());
    assert!(job.decoder.is_some());
    for _ in 0..128 {
        if job.pending_completion_rejection.is_none() {
            break;
        }
        let step = caller.turn(grant(1, 4_096), |cx| job.close_step(cx)).expect("original fixture close turn");
        if let InteractiveJobCloseStep::Pending { progress } = step {
            assert!(progress.copied_items <= 1 && progress.released_bytes <= 4_096);
        }
    }
    assert!(job.pending_completion_rejection.is_none());
    assert!(job.decoder.is_some(), "normal decoder owner stays retained until the rejected output is terminal");
    assert_eq!(caller.turn(grant(1, 4_096), |cx| job.close_step(cx)).expect("original fixture close turn"), released(1, 0));
    assert_eq!(caller.turn(grant(1, 4_096), |cx| job.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Complete { progress: Default::default() });
    assert!(job.terminal_is_empty());
    caller.retire_exact();
}

/// 🧪️ A wire page costs one work unit, so a multi-byte page fits the contract's one-unit step loan.
#[test]
fn drawing_gesture_wire_page_costs_one_work_unit_inside_its_step_loan() {
    let wire = [7u8; 64];
    let (mut input, _) = semio_framework_action_bus::RetainedToolWireInput::admit_original(wire.len(), DRAWING_GESTURE_RAW_BYTES, grant(4, 1 << 20)).expect("wire admission").expect("wire admitted");
    input.admit_page(semio_framework_action_bus::ToolWirePage::try_copy_from(&wire).expect("page")).map_err(|(error, _)| error).expect("page admitted");
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), grant(1 << 20, 1 << 30), || Some(0), u64::MAX, 1 << 20);
    let mut job = DrawingGestureOperationJob::from_wire(DrawingGestureOperationPayload { source_identity: SceneIdentity { instance: 1, base: 1, generation: 1, revision: [1; 32] }, original: None }, input);
    let contract = DrawingGestureOperationJobFactory::new("drawing").execution_contract();
    let turn = caller.turn(grant(1, 4_096), |cx| cx.with_original_work_loan(contract.max_work_units_per_step, u64::MAX, |cx| Ok(job.turn(cx)))).expect("one-unit step loan is not overspent by a wire page");
    assert_eq!(turn, semio_s_artifact_draw_drawing_host::outcome::DrawingJobTurn::Yield);
    assert_eq!(job.raw_page_cursor, 1);
}
