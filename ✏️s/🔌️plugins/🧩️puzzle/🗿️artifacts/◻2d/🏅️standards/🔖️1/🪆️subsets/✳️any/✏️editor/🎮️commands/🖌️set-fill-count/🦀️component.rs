//! 🖌️ `set-fill-count` command.

use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{apply_brush_place_payload, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::Effect;
use serde_json::Value;

fn apply_fill_placements(ctx: &mut Puzzle2dActionCtx<'_>, placements: &[Value]) {
    for placement in placements {
        apply_brush_place_payload(&mut ctx.scene.fixture, placement);
    }
}

fn queue_next_step(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.effects.push(Effect::DispatchAction {
        req: semio_framework_plugin::kernel::RequestId(semio_framework_job::allocate_operation_id().0),
        action: "brushFillSessionStep".into(),
        args: semio_framework::optional_json_to_dsl(Some(serde_json::json!({ "generation": ctx.scene.runtime.fill_job_generation }))),
        delay_ms: 0,
    });
}

pub fn begin_fill_job(ctx: &mut Puzzle2dActionCtx<'_>, count: u32, seed: u64) {
    let snapshot = ctx.host.borrow().board_fill_snapshot();
    let generation = ctx.scene.runtime.fill_job_generation.saturating_add(1);
    let job = infinite_canvas::BoardFillJob::new(snapshot, count, seed, 0, generation);
    let operation = job.operation();
    ctx.scene.runtime.fill_job_checkpoint = Some(job.checkpoint_bytes());
    ctx.scene.runtime.fill_job_operation = operation.operation.0;
    ctx.scene.runtime.fill_job_generation = operation.generation.0;
    ctx.scene.runtime.fill_job_seed = seed;
    ctx.scene.runtime.fill_job_applied_count = 0;
    step_fill_job(ctx, Some(generation));
}

pub fn step_fill_job(ctx: &mut Puzzle2dActionCtx<'_>, expected_generation: Option<u64>) {
    if expected_generation.is_some_and(|generation| generation != ctx.scene.runtime.fill_job_generation) {
        return;
    }
    let Some(checkpoint) = ctx.scene.runtime.fill_job_checkpoint.clone() else { return };
    let operation = semio_framework_job::Operation::new(
        semio_framework_job::OperationId(ctx.scene.runtime.fill_job_operation),
        semio_framework_job::RevisionId(0),
        semio_framework_job::Generation(ctx.scene.runtime.fill_job_generation),
        ctx.scene.runtime.fill_job_seed,
    );
    let Ok(job) = infinite_canvas::BoardFillJob::restore(&checkpoint, operation) else {
        ctx.scene.runtime.fill_job_checkpoint = None;
        return;
    };
    let operation = job.operation();
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "puzzle2d.fill", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_ms: 7 },
        now_ms: semio_framework_job::default_now_ms,
    };
    let Ok(mut session) = semio_framework_job::BatchJobSession::try_new(job, params) else {
        ctx.scene.runtime.fill_job_checkpoint = None;
        return;
    };
    if session.step().is_err() {
        session.begin_close();
        return;
    }
    let Some(job) = session.checked_out_job_mut() else {
        session.begin_close();
        return;
    };
    let applied = ctx.scene.runtime.fill_job_applied_count.min(job.placements().len());
    apply_fill_placements(ctx, &job.placements()[applied..]);
    ctx.scene.runtime.fill_job_applied_count = job.placements().len();
    let next_checkpoint = job.checkpoint_bytes();
    let Some(mut outcome) = session.take_outcome() else {
        session.begin_close();
        return;
    };
    if let semio_framework_job::StepOutcome::PreviewReady(bytes) = &outcome {
        ctx.scene.runtime.fill_job_preview = serde_json::from_slice(bytes).ok();
    }
    match &outcome {
        semio_framework_job::StepOutcome::Complete(_) => {
            ctx.scene.runtime.fill_job_checkpoint = None;
            ctx.scene.runtime.fill_job_preview = None;
        }
        semio_framework_job::StepOutcome::Cancelled | semio_framework_job::StepOutcome::Fault(_) => {
            ctx.scene.runtime.fill_job_checkpoint = None;
            ctx.scene.runtime.fill_job_preview = None;
            ctx.scene.runtime.fill_count = 0;
        }
        semio_framework_job::StepOutcome::CheckpointReady(checkpoint) => {
            ctx.scene.runtime.fill_job_checkpoint = Some(checkpoint.state.to_vec());
            queue_next_step(ctx);
        }
        _ => {
            ctx.scene.runtime.fill_job_checkpoint = Some(next_checkpoint);
            queue_next_step(ctx);
        }
    }
    while !outcome.terminal_is_empty() {
        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    session.begin_close();
}

/// 🪣️ Activates fill and starts one persistent, generation-tagged job session.
pub fn set_fill_count(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map_or(0, |value| value.round().max(0.0) as u32).min(fill::PUZZLE2D_FILL_COUNT_MAX);
    ctx.scene.runtime.fill_count = count;
    ctx.effects.push(Effect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
    ctx.host.borrow_mut().set_active_utility(overview::utilities::brush::UTILITY_ID);
    begin_fill_job(ctx, count, 1);
}
