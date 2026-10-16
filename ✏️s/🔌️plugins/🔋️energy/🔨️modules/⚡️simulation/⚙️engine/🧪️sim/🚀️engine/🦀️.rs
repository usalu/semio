//! 🚀️ Headless batch engine: the job constructor, the batch run, the batch grant and the owned close driver.

use super::*;

/// ⚡️ Headless BEM simulation engine.
pub struct Engine;

impl Engine {
    /// ⚡️ Create the persistent operation used by interactive hosts and the batch adapter.
    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn job(model: Model, config: SimulationConfig) -> Result<EnergyJob, EnergyAdmissionRejected> {
        EnergyJob::new(Operation::new(allocate_operation_id(), RevisionId(0), Generation(0), 0), model, config)
    }

    /// ⚡️ Run full building energy simulation.
    pub fn run(model: Model, config: SimulationConfig) -> Result<Results, Error> {
        enum Seen {
            Yield,
            Preview,
            Checkpoint,
            Complete,
            Fault,
            Cancelled,
        }
        let mut job = Self::job(model, config).map_err(|_| Error::severe("energy numerical admission rejected"))?;
        let operation = job.operation.operation;
        let generation = job.operation.generation;
        let mut sequence = 0;
        let seen = loop {
            let mut receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation, generation, StepBudget::new(32, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut receipt);
            let seen = match InteractiveJob::step(&mut job, &mut context) {
                Ok(Some(JobOutcomeBorrow::PreviewReady { .. })) => Seen::Preview,
                Ok(Some(JobOutcomeBorrow::CheckpointReady { .. })) => Seen::Checkpoint,
                Ok(Some(JobOutcomeBorrow::Complete { .. })) => Seen::Complete,
                Ok(Some(JobOutcomeBorrow::Fault { .. })) | Err(_) => Seen::Fault,
                Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => Seen::Cancelled,
                Ok(Some(JobOutcomeBorrow::Yield { .. }) | None) => Seen::Yield,
            };
            match seen {
                Seen::Preview => {
                    if let Some(mut packet) = job.take_preview_packet(generation).map_err(|_| Error::severe("energy preview generation rejected"))? {
                        close_retained_packet(&mut packet);
                    }
                }
                Seen::Checkpoint => {
                    if let Some(mut lease) = job.take_checkpoint_packet(generation).map_err(|_| Error::severe("energy checkpoint generation rejected"))? {
                        close_retained_packet(lease.packet_mut());
                        job.ack_checkpoint_packet(lease).map_err(|_| Error::severe("energy checkpoint ACK rejected"))?;
                    }
                }
                Seen::Yield => {}
                terminal => break terminal,
            }
        };
        let result = match seen {
            Seen::Complete => job.take_results().ok_or_else(|| Error::severe("energy job completed without results")),
            Seen::Fault => Err(Error::severe("energy simulation fault")),
            Seen::Cancelled => Err(Error::severe("energy simulation cancelled")),
            Seen::Yield | Seen::Preview | Seen::Checkpoint => Err(Error::severe("energy batch adapter stopped before a terminal outcome")),
        };
        close_owned_job(&mut job);
        result
    }
}

/// 🎟️ The bounded five-currency authority the batch adapter supplies to every admitted step.
pub const ENERGY_BATCH_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🚪️ Runs the job's own close ladder to completion with grants it quotes itself.
pub fn close_owned_job(job: &mut EnergyJob) {
    InteractiveJob::begin_close(job);
    let mut idle = 0usize;
    loop {
        if InteractiveJob::terminal_is_empty(job) {
            return;
        }
        let grant = energy_self_funded(job.close_demands().expect("a locally owned energy job quotes its close demand"));
        let step = EnergyJobAuthority::close_step(job, grant).expect("a locally owned energy job close is never refused");
        assert!(step.progress().fits(grant), "an energy close receipt must fit its own quoted grant");
        assert!(!matches!(step, InteractiveJobCloseStep::Blocked), "a locally owned energy job close has no external owner");
        idle = if step.progress() == RetainedCloneProgress::default() && !matches!(step, InteractiveJobCloseStep::Complete { .. }) { idle + 1 } else { 0 };
        assert!(idle < 1_000, "the energy close ladder stalled");
    }
}
