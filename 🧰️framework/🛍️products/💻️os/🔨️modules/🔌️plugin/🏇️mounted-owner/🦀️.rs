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

impl MountedOwnerPolicyV1 {
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
    progress: RetainedCloneProgress,
    context: &'turn mut StepContext<'context>,
}

impl<'turn, 'context> MountedOwnerTurnV1<'turn, 'context> {
    /// 🛂️ Validates pinned identity and real scheduling before admitting any owner work.
    pub fn admit(instance_id: u32, expected: MountedOwnerIdentityV1, phase: MountedOwnerPhaseV1, policy: MountedOwnerPolicyV1, context: &'turn mut StepContext<'context>) -> Result<Option<Self>, ValueError> {
        let policy = policy.validate()?;
        if instance_id == 0 || expected.instance_id != instance_id || expected.operation.0 == 0 || context.operation() != expected.operation || context.generation() != expected.generation {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "mounted turn has stale instance, operation or generation"));
        }
        if context.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mounted turn was cancelled")); }
        if context.deadline_exceeded() || context.fuel_exhausted() { return Ok(None); }
        let policy_grant=policy.grant(phase);let original=context.retained_grant();
        let grant=RetainedCloneGrant {maximum_items:policy_grant.maximum_items.min(original.maximum_items),maximum_copy_bytes:policy_grant.maximum_copy_bytes.min(original.maximum_copy_bytes),maximum_capacity_bytes:policy_grant.maximum_capacity_bytes.min(original.maximum_capacity_bytes),maximum_release_bytes:policy_grant.maximum_release_bytes.min(original.maximum_release_bytes),maximum_depth:policy_grant.maximum_depth.min(original.maximum_depth)};
        if grant.maximum_items == 0 { return Ok(None); }
        Ok(Some(Self { identity: expected, phase, grant, progress: RetainedCloneProgress::default(), context }))
    }

    /// 🪪️ Returns the owner-pinned identity without consulting mutable document state.
    pub fn identity(&self) -> MountedOwnerIdentityV1 { self.identity }

    /// 🏷️ Returns the selected phase to receiving owners.
    pub fn phase(&self) -> MountedOwnerPhaseV1 { self.phase }

    /// 📬️ The next owner receives only remaining original physical authority and real scheduling fuel.
    pub fn grant(&self) -> RetainedCloneGrant {
        let original = self.context.retained_grant();
        RetainedCloneGrant {
            maximum_items: self.grant.maximum_items.saturating_sub(self.progress.copied_items).min(original.maximum_items),
            maximum_copy_bytes: self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes).min(original.maximum_copy_bytes),
            maximum_capacity_bytes: self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes).min(original.maximum_capacity_bytes),
            maximum_release_bytes: self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes).min(original.maximum_release_bytes),
            maximum_depth: self.grant.maximum_depth.min(original.maximum_depth),
        }
    }

    /// 🚦️ Descriptive demand may refuse a turn but can never increase its authority.
    pub fn admits(&self, demand: RetirementDemand) -> Result<bool, ValueError> {
        if self.context.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mounted owner was cancelled")); }
        if self.context.should_yield() { return Ok(false); }
        let grant = self.grant();
        Ok(grant.maximum_items > 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth)
    }

    /// 🧾️ Accounts the actual receipt once against both physical authority and original job fuel.
    pub fn record(&mut self, receipt: RetainedCloneProgress, complete: bool, terminal_is_empty: bool) -> Result<(), ValueError> {
        let grant = self.grant();
        let admitted=receipt.fits(grant)&&(grant.maximum_depth!=0||receipt==RetainedCloneProgress::default())&&(!complete||terminal_is_empty);
        let progress = self.progress.checked_add(receipt).map_err(|error|error.with_retained_progress(receipt))?;
        self.progress = progress;
        if receipt != RetainedCloneProgress::default() { self.context.consume_fuel(1); }
        self.context.consume_retained(receipt)?;
        if !admitted||!progress.fits(self.grant) {return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mounted owner actual receipt or terminal witness exceeded its authority").with_retained_progress(receipt));}
        Ok(())
    }

    /// ⛽️ Validates a nested job's actual scheduling debit without spending the same fuel twice.
    pub fn advance_job(&mut self, job: impl FnOnce(&mut StepContext<'context>, RetainedCloneGrant) -> Result<MountedOwnerJobReceiptV1, ValueError>) -> Result<RetainedCloneProgress, ValueError> {
        if self.context.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mounted job was cancelled before execution")); }
        if self.context.should_yield() { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "mounted job has no original scheduling authority")); }
        let grant = self.grant();
        if grant.maximum_items == 0 { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "mounted job has no original physical turn authority")); }
        let before_fuel = self.context.fuel_remaining();
        let before_physical=self.context.retained_progress();
        let result = job(self.context, grant);
        let after=self.context.retained_progress();
        let difference=|later:usize,earlier:usize|later.checked_sub(earlier).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original nested receipt recipient decreased"));
        let actual=RetainedCloneProgress {copied_items:difference(after.copied_items,before_physical.copied_items)?,copied_bytes:difference(after.copied_bytes,before_physical.copied_bytes)?,retained_capacity_bytes:difference(after.retained_capacity_bytes,before_physical.retained_capacity_bytes)?,released_bytes:difference(after.released_bytes,before_physical.released_bytes)?};
        self.progress = self.progress.checked_add(actual).map_err(|error|error.with_retained_progress(actual))?;
        let receipt = result?;
        let spent = before_fuel.saturating_sub(self.context.fuel_remaining());
        let minimum_debit = u64::from(actual != RetainedCloneProgress::default());
        if actual!=receipt.progress||!actual.fits(grant)||!self.progress.fits(self.grant)||spent<minimum_debit||spent>before_fuel||receipt.complete&&!receipt.terminal_is_empty {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mounted nested job receipt differs from original physical, scheduling or terminal authority").with_retained_progress(actual));
        }
        Ok(receipt.progress)
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
