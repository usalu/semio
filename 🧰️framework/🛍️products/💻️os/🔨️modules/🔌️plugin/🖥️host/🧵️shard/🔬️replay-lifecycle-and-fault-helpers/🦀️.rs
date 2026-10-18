//! 🔬️ Replay lifecycle and fault helpers: the replay lifecycle projection and trace exerciser and the turn fault, stage and start job fault helpers. See [`super`].

use super::*;

pub(crate) struct ReplayLifecycleProjection {
    pub state: &'static str,
    pub first_reason: Option<String>,
    pub release_opportunities: usize,
}

pub fn exercise_replay_lifecycle_trace(events: &[String]) -> Result<ReplayLifecycleProjection, String> {
    let pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let actor = 1;
    let job = 1;
    let kind = "relay.lifecycle.fixture".to_string();
    let input = Vec::new();
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let authority = JobTurn { job, operation: JobOperation { operation: 1, base_revision: 0, generation: 1, preview_sequence: 0, seed: 1 }, step_sequence: 0 };
    let mut replay_seeds = vec![Some(MountedReplaySeed::new(actor, job, authority, request, JobPlacement::Inline, kind, input).map_err(|_| "replay lifecycle fixture admission refused".to_string())?)];
    let mut running_jobs = BTreeSet::new();
    let mut job_turns = HashMap::new();
    let mut job_authorities = HashMap::new();
    let mut job_placement = HashMap::new();
    let mut first_reason = None;
    let mut release_opportunities = 0;
    for event in events {
        if let Some(stage) = event.strip_prefix("fault:") {
            let stage = match stage {
                "capture-kind" => "capture-kind",
                "capture-input" => "capture-input",
                "checkpoint" => "checkpoint",
                "capture-checkpoint" => "capture-checkpoint",
                "live-start" => "live-start",
                "materialize-kind" => "materialize-kind",
                "materialize-input" => "materialize-input",
                "materialize-checkpoint" => "materialize-checkpoint",
                "restore" => "restore",
                "prepare-kind" => "prepare-kind",
                "restart" => "restart",
                other => return Err(format!("unknown replay lifecycle fault stage {other:?}")),
            };
            ShardLoop::begin_replay_seed_close_owned(&mut replay_seeds, &mut running_jobs, &mut job_turns, &mut job_authorities, &mut job_placement, 0, ReplaySeedCloseReason::Fault { stage, detail: event.clone() });
        } else if event == "cancel" {
            ShardLoop::begin_replay_seed_close_owned(&mut replay_seeds, &mut running_jobs, &mut job_turns, &mut job_authorities, &mut job_placement, 0, ReplaySeedCloseReason::Cancelled);
        } else if event == "release" && replay_seeds[0].is_some() {
            ShardLoop::close_replay_seed_one(&mut replay_seeds, 0).map_err(|error| error.to_string())?;
            release_opportunities += 1;
        }
        if first_reason.is_none() {
            first_reason = replay_seeds[0].as_ref().and_then(|seed| seed.close_reason.as_ref()).map(|reason| match reason {
                ReplaySeedCloseReason::Completed => "completed".to_string(),
                ReplaySeedCloseReason::Cancelled => "cancelled".to_string(),
                ReplaySeedCloseReason::ActorLost => "actor-lost".to_string(),
                ReplaySeedCloseReason::Fault { stage, .. } => format!("fault:{stage}"),
            });
        }
    }
    let state = replay_seeds[0].as_ref().map_or("Empty", |seed| match seed.phase {
        ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell => "Closing",
        ReplaySeedPhase::Retained => "Retained",
        _ => "CaptureKind",
    });
    drop(replay_seeds);
    if JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire) != pages_before || JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire) != abi_before {
        return Err("replay lifecycle fixture accounting did not return to baseline".to_string());
    }
    Ok(ReplayLifecycleProjection { state, first_reason, release_opportunities })
}

pub(super) fn turn_fault_message(fault: &TurnFault) -> String {
    fault.to_string()
}

/// 🐕️ P1c: maps an actor's [`semio_framework_actor::Lane`] onto the [`InteractiveStage`] family
/// [`Watchdog::start`] reports overruns under. `Interactive`/`UserVisible` map onto their own
/// dedicated stages (this crate has no separate "UI event" vs "UI present" distinction — a shard's
/// interactive turn IS the step); `Background`/`Maintenance` share `BackgroundStep` since neither has
/// a tighter soft target than the other in `semio_framework_trace`'s own vocabulary.
pub(super) fn interactive_stage_for(lane: semio_framework_actor::Lane) -> InteractiveStage {
    match lane {
        semio_framework_actor::Lane::Interactive => InteractiveStage::InteractiveStep,
        semio_framework_actor::Lane::UserVisible => InteractiveStage::UserVisibleSimStep,
        semio_framework_actor::Lane::Background | semio_framework_actor::Lane::Maintenance => InteractiveStage::BackgroundStep,
    }
}

/// 🧯️ Encodes a host-side `TurnFault` (a `start-job` admission failure, or a `step_job` runtime
/// fault) into the same `pack` wire shape `Event::JobCompleted{result: Err(bytes), ..}`'s bytes
/// always carry — every other fault-bearing `RequestOutcome::Err` in this crate writes a fault
/// `pack`, so the guest's `outcome_to_result` decodes it exactly like a normal `Event::Completed`
/// failure, with no special-casing for jobs.
pub(super) fn start_job_fault_bytes(fault: &TurnFault) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("job.host-fault"), fault.to_string())))
}
