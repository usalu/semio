//! 🏇️ Mounted ownership joins caller physical grants with the original job scheduling authority.

use semio_framework_job::{Generation, OperationId, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

/// 🎟️ Each phase requires its embedding owner's independent physical authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MountedOwnerPolicyV1 {
    pub preparation: RetainedCloneGrant,
    pub maintenance: RetainedCloneGrant,
    pub close: RetainedCloneGrant,
}

/// 🪪️ The instance and job identities remain pinned throughout a mounted turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MountedOwnerIdentityV1 {
    pub instance_id: u32,
    pub operation: OperationId,
    pub generation: Generation,
}

/// 🧭️ Physical authorities belong to the actual selected lifecycle phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedOwnerPhaseV1 { Preparation, Maintenance, Close }

/// 🧾️ A nested job reports physical work while its owner keeps the original outcome.
pub struct MountedOwnerJobReceiptV1 {
    pub progress: RetainedCloneProgress,
    pub complete: bool,
    pub terminal_is_empty: bool,
}

/// 🪜️ Raises copy, capacity, release and depth to the covering power of two of an indivisible demand, never past `ceiling` and never below `loan`.
pub fn covering_grant(loan: RetainedCloneGrant, demand: RetirementDemand, ceiling: RetainedCloneGrant) -> RetainedCloneGrant {
    let cover = |current: usize, needed: usize, limit: usize| if needed <= current { current } else { needed.checked_next_power_of_two().unwrap_or(needed).min(limit).max(current) };
    RetainedCloneGrant { maximum_copy_bytes: cover(loan.maximum_copy_bytes, demand.copy_bytes, ceiling.maximum_copy_bytes), maximum_capacity_bytes: cover(loan.maximum_capacity_bytes, demand.capacity_bytes, ceiling.maximum_capacity_bytes), maximum_release_bytes: cover(loan.maximum_release_bytes, demand.release_bytes, ceiling.maximum_release_bytes), maximum_depth: cover(loan.maximum_depth, demand.depth, ceiling.maximum_depth), ..loan }
}

impl MountedOwnerPolicyV1 {
    /// 🎛️ The interactive component embedding every hub composition grants per host turn: 64 one-item units batched within the turn, 16 MiB capacity so a whole-document clone fits one apply, 1 MiB release and the covering power-of-two copy of the largest indivisible step (ticket 26/09/23 BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT, `📓️2026-10-12-publication-perf.md`).
    pub const INTERACTIVE_COMPONENT: Self = {
        let grant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: Self::covering_copy_bytes(Self::LARGEST_INDIVISIBLE_STEP_COPY_BYTES), maximum_capacity_bytes: 16_777_216, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        Self { preparation: grant, maintenance: grant, close: grant }
    };

    /// 📐️ The largest indivisible per-step copy demand any retained owner posts, measured: a mounted Drawing job close (688,784 B), a store-replacement member open (377,696 B), a member record receive (277,538 B) and a history-replay preparation step (33,976 B). A host grant below it leaves archive loads, undo and time travel unfundable forever.
    pub const LARGEST_INDIVISIBLE_STEP_COPY_BYTES: usize = 688_784;

    /// 🪜️ Smallest power-of-two copy grant (never below 32 KiB) that covers one indivisible step demand.
    pub const fn covering_copy_bytes(demand: usize) -> usize {
        let covering = demand.next_power_of_two();
        if covering < 32_768 { 32_768 } else { covering }
    }

    /// 📏️ Checks the schema's portable target-width bounds without granting omitted authority.
    pub fn validate(self) -> Result<Self, ValueError> {
        for grant in [self.preparation, self.maintenance, self.close] {
            if [grant.maximum_items, grant.maximum_copy_bytes, grant.maximum_capacity_bytes, grant.maximum_release_bytes, grant.maximum_depth].into_iter().any(|amount| amount > u32::MAX as usize) {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "mounted policy exceeds portable u32 authority"));
            }
        }
        Ok(self)
    }

    /// 🏷️ Selects an explicitly supplied phase without a default grant.
    pub fn grant(self, phase: MountedOwnerPhaseV1) -> RetainedCloneGrant {
        match phase { MountedOwnerPhaseV1::Preparation => self.preparation, MountedOwnerPhaseV1::Maintenance => self.maintenance, MountedOwnerPhaseV1::Close => self.close }
    }
}

/// 🧵️ A borrowed original job context governs every physical subturn and cumulative receipt.
pub struct MountedOwnerTurnV1<'turn, 'context> {
    identity: MountedOwnerIdentityV1,
    phase: MountedOwnerPhaseV1,
    grant: RetainedCloneGrant,
    ceiling: RetainedCloneGrant,
    progress: RetainedCloneProgress,
    current_progress: RetainedCloneProgress,
    context: &'turn mut StepContext<'context>,
}

fn receipt_delta(before: RetainedCloneProgress, after: RetainedCloneProgress) -> Result<RetainedCloneProgress, ValueError> {
    let delta = |earlier: usize, later: usize| later.checked_sub(earlier)
        .ok_or_else(|| ValueError::invariant("mounted receipt decreased"));
    Ok(RetainedCloneProgress {
        copied_items: delta(before.copied_items, after.copied_items)?,
        copied_bytes: delta(before.copied_bytes, after.copied_bytes)?,
        retained_capacity_bytes: delta(before.retained_capacity_bytes, after.retained_capacity_bytes)?,
        released_bytes: delta(before.released_bytes, after.released_bytes)?,
    })
}

impl<'turn, 'context> MountedOwnerTurnV1<'turn, 'context> {
    /// 🛂️ Validates pinned identity and real scheduling before admitting any owner work.
    pub fn admit(instance_id: u32, expected: MountedOwnerIdentityV1, phase: MountedOwnerPhaseV1, policy: MountedOwnerPolicyV1, context: &'turn mut StepContext<'context>) -> Result<Option<Self>, ValueError> {
        let policy = policy.validate()?;
        if instance_id == 0 || expected.instance_id != instance_id || expected.operation.0 == 0 || context.operation() != expected.operation || context.generation() != expected.generation {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "mounted turn has stale instance, operation or generation"));
        }
        if context.is_cancelled() && phase != MountedOwnerPhaseV1::Close { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mounted turn was cancelled")); }
        if if phase==MountedOwnerPhaseV1::Close {context.retirement_should_yield()}else{context.should_yield()} { return Ok(None); }
        let policy_grant=policy.grant(phase);let original=context.retained_grant();
        let grant=RetainedCloneGrant {maximum_items:policy_grant.maximum_items.min(original.maximum_items),maximum_copy_bytes:policy_grant.maximum_copy_bytes.min(original.maximum_copy_bytes),maximum_capacity_bytes:policy_grant.maximum_capacity_bytes.min(original.maximum_capacity_bytes),maximum_release_bytes:policy_grant.maximum_release_bytes.min(original.maximum_release_bytes),maximum_depth:policy_grant.maximum_depth.min(original.maximum_depth)};
        if grant.maximum_items == 0 { return Ok(None); }
        let ceiling=RetainedCloneGrant{maximum_items:grant.maximum_items,..original};
        Ok(Some(Self { identity: expected, phase, grant, ceiling, progress: RetainedCloneProgress::default(), current_progress: RetainedCloneProgress::default(), context }))
    }

    fn scheduling_should_yield(&self)->bool{if self.phase==MountedOwnerPhaseV1::Close{self.context.retirement_should_yield()}else{self.context.should_yield()}}

    /// 🪪️ Returns the owner-pinned identity without consulting mutable document state.
    pub fn identity(&self) -> MountedOwnerIdentityV1 { self.identity }

    /// 🏷️ Returns the selected phase to receiving owners.
    pub fn phase(&self) -> MountedOwnerPhaseV1 { self.phase }

    /// 📬️ The next owner receives only remaining original physical authority and real scheduling fuel.
    pub fn grant(&self) -> RetainedCloneGrant {
        let original = self.context.retained_grant();
        RetainedCloneGrant {
            maximum_items: self.grant.maximum_items.saturating_sub(self.current_progress.copied_items).min(original.maximum_items),
            maximum_copy_bytes: self.grant.maximum_copy_bytes.saturating_sub(self.current_progress.copied_bytes).min(original.maximum_copy_bytes),
            maximum_capacity_bytes: self.grant.maximum_capacity_bytes.saturating_sub(self.current_progress.retained_capacity_bytes).min(original.maximum_capacity_bytes),
            maximum_release_bytes: self.grant.maximum_release_bytes.saturating_sub(self.current_progress.released_bytes).min(original.maximum_release_bytes),
            maximum_depth: self.grant.maximum_depth.min(original.maximum_depth),
        }
    }

    /// 🚦️ Descriptive demand may refuse a turn but can never increase its authority.
    pub fn admits(&self, demand: RetirementDemand) -> Result<bool, ValueError> {
        if self.context.is_cancelled()&&self.phase!=MountedOwnerPhaseV1::Close { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mounted owner was cancelled")); }
        if self.scheduling_should_yield() { return Ok(false); }
        let grant = self.grant();
        Ok(grant.maximum_items > 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth)
    }

    /// 🧾️ Accounts the actual receipt once against both physical authority and original job fuel.
    pub fn record(&mut self, receipt: RetainedCloneProgress, complete: bool, terminal_is_empty: bool) -> Result<(), ValueError> {
        let grant = self.grant();
        let admitted=receipt.fits(grant)&&(grant.maximum_depth!=0||receipt==RetainedCloneProgress::default())&&(!complete||terminal_is_empty);
        let progress = self.progress.checked_add(receipt).map_err(|error|error.with_retained_progress(receipt))?;
        self.progress = progress;
        self.current_progress = self.current_progress.checked_add(receipt)?;
        if receipt != RetainedCloneProgress::default() { self.context.consume_fuel(1); }
        self.context.consume_retained(receipt)?;
        if !admitted||!self.current_progress.fits(self.grant) {return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mounted owner actual receipt or terminal witness exceeded its authority").with_retained_progress(receipt));}
        Ok(())
    }

    /// ⛽️ Validates a nested job's actual scheduling debit without spending the same fuel twice.
    pub fn advance_job(&mut self, job: impl FnOnce(&mut StepContext<'context>, RetainedCloneGrant) -> Result<MountedOwnerJobReceiptV1, ValueError>) -> Result<RetainedCloneProgress, ValueError> {
        if self.context.is_cancelled()&&self.phase!=MountedOwnerPhaseV1::Close { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mounted job was cancelled before execution")); }
        if self.scheduling_should_yield() { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "mounted job has no original scheduling authority")); }
        let grant = self.grant();
        if grant.maximum_items == 0 { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "mounted job has no original physical turn authority")); }
        let before_fuel = self.context.original_turn_fuel_remaining();
        let before = self.context.retained_progress();
        let reserved_before = self.context.reserved_return_progress();
        let result = job(self.context, grant);
        let (actual, current) = self.receive_posted(before, reserved_before)?;
        let receipt = result?;
        let spent = before_fuel.saturating_sub(self.context.original_turn_fuel_remaining());
        let minimum_debit = u64::from(current != RetainedCloneProgress::default());
        if actual!=receipt.progress||!current.fits(grant)||!self.current_progress.fits(self.grant)||spent<minimum_debit||spent>before_fuel||receipt.complete&&!receipt.terminal_is_empty {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mounted nested job receipt differs from original physical, scheduling or terminal authority").with_retained_progress(actual));
        }
        Ok(receipt.progress)
    }

    /// 🏇️ Adopts a producer's exact fresh or gross receipt and publishes the caller's full posted effects.
    pub fn advance_lifecycle<S: semio_framework_value::RetirementReceipt, E: From<ValueError>>(
        &mut self,
        with_progress: impl FnOnce(S, RetainedCloneProgress) -> S,
        producer: impl FnOnce(&mut StepContext<'context>, RetainedCloneGrant) -> Result<S, E>,
    ) -> Result<S, E> {
        let (call, total) = (self.grant(), self.grant);
        self.lifecycle(call, total, with_progress, producer)
    }

    /// 📐️ The physical authority an indivisible step may be covered up to: the mounted item batch over the original turn's remaining copy, capacity and release stock. The maintenance ladder widens each unit grant with [`covering_grant`] under exactly this ceiling.
    pub fn covering_ceiling(&self) -> RetainedCloneGrant {
        let original = self.context.retained_grant();
        RetainedCloneGrant { maximum_items: self.grant().maximum_items, ..original }
    }

    /// 🪜️ [`Self::advance_lifecycle`] for a producer that covers an indivisible step larger than the mounted loan: its receipt is accepted against [`Self::covering_ceiling`], the same authority the producer widened its unit grant under, never beyond it.
    pub fn advance_covering_lifecycle<S: semio_framework_value::RetirementReceipt, E: From<ValueError>>(
        &mut self,
        with_progress: impl FnOnce(S, RetainedCloneProgress) -> S,
        producer: impl FnOnce(&mut StepContext<'context>, RetainedCloneGrant) -> Result<S, E>,
    ) -> Result<S, E> {
        let call = self.covering_ceiling();
        let total = RetainedCloneGrant { maximum_items: self.grant.maximum_items, ..self.ceiling };
        self.lifecycle(call, total, with_progress, producer)
    }

    fn lifecycle<S: semio_framework_value::RetirementReceipt, E: From<ValueError>>(
        &mut self,
        bound: RetainedCloneGrant,
        total: RetainedCloneGrant,
        with_progress: impl FnOnce(S, RetainedCloneProgress) -> S,
        producer: impl FnOnce(&mut StepContext<'context>, RetainedCloneGrant) -> Result<S, E>,
    ) -> Result<S, E> {
        if self.scheduling_should_yield() || self.grant().maximum_items == 0 {
            return Ok(S::retirement_yield());
        }
        let grant = self.grant();
        let before = self.context.retained_progress();
        let before_fuel = self.context.original_turn_fuel_remaining();
        let reserved_before = self.context.reserved_return_progress();
        let result = producer(self.context, grant);
        let (actual, current) = self.receive_posted(before, reserved_before)?;
        let step = result?;
        let reported = step.retirement_progress().unwrap_or_default();
        if (reported != actual && reported != current) || !current.fits(bound) || !self.current_progress.fits(total) || current != RetainedCloneProgress::default() && self.context.original_turn_fuel_remaining() == before_fuel {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "original lifecycle receipt differs from its caller authority").with_retained_progress(actual).into());
        }
        Ok(if reported == actual { step } else { with_progress(step, actual) })
    }


    /// 🛞️ Advances funded close actions within this same caller's physical authority and deadline.
    pub fn advance_close_batch<S: semio_framework_value::RetirementReceipt, E: From<ValueError>>(
        &mut self,
        with_progress: impl Fn(S, RetainedCloneProgress) -> S,
        mut producer: impl FnMut(&mut StepContext<'context>, RetainedCloneGrant) -> Result<S, E>,
    ) -> Result<S, E> {
        if self.phase != MountedOwnerPhaseV1::Close {
            return Err(ValueError::invariant("mounted close batch requires the close phase").into());
        }
        let before = self.progress;
        let mut step = S::retirement_yield();
        for _ in 0..self.grant().maximum_items {
            step = self.advance_lifecycle(&with_progress, &mut producer)?;
            if step.retirement_complete()
                || step.retirement_progress().unwrap_or_default() == RetainedCloneProgress::default()
                || self.grant().maximum_items == 0
                || self.context.deadline_exceeded()
            {
                break;
            }
        }
        let progress = receipt_delta(before, self.progress)?;
        Ok(if step.retirement_progress().unwrap_or_default() == progress { step } else { with_progress(step, progress) })
    }

    fn receive_posted(&mut self, before: RetainedCloneProgress, reserved_before: RetainedCloneProgress) -> Result<(RetainedCloneProgress, RetainedCloneProgress), ValueError> {
        let actual = receipt_delta(before, self.context.retained_progress())?;
        let reserved = receipt_delta(reserved_before, self.context.reserved_return_progress())?;
        let current = receipt_delta(reserved, actual)?;
        self.progress = self.progress.checked_add(actual)?;
        self.current_progress = self.current_progress.checked_add(current)?;
        Ok((actual, current))
    }

    /// 📈️ Exposes all four physical receipt currencies to the actual runtime caller.
    pub fn progress(&self) -> RetainedCloneProgress { self.progress }

    /// 🔗️ Nested jobs borrow the same cancellation, deadline, generation and payload authority.
    pub fn context(&mut self) -> &mut StepContext<'context> { self.context }
}

/// 🎭️ Original bounded actor capture under explicit mounted publication authority.
#[path="🎭️actor/🦀️.rs"]
pub mod actor_capture;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🔗️ Physical alias closure preserves the original final mutable owner and dynamic box shell.
#[path="🔗️alias/🦀️.rs"]
pub mod alias_retirement;
