//! 🎟️ Holds a picker until its mounted document edit and Original source have settled.

use semio_framework_job::{
    Operation, OperationId, OriginalSourceBinding, OriginalTurnFailure, StepContext, close_original_effect_step,
};
use semio_framework_value::{
    OriginalRetirementFailure, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll,
    RetirementDemand, SharedUtf8, ValueError, retained_clone::RetainedCloneClose,
};
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_ownership::bounded_ownership::{
    PluginLifecycleFailure, PluginLifecycleStep, original_lifecycle_delta, original_lifecycle_step,
};
use semio_plugin_decl_media::original_media::{FormatReceipt, RawMediaInput, RawMediaPicker};

/// 📌 Remembers the exact picker, actor, and freshly mounted operation until its ownership is returned.
pub struct PickerPublication {
    pub source: OriginalSourceBinding,
    pub meta: Option<ActionMeta>,
    pub input: Option<Box<RawMediaInput>>,
    pub operation: Option<Operation>,
    pub mount_id: Option<OperationId>,
    pub committed: Option<OperationId>,
    pub receipt: Option<FormatReceipt>,
    input_donated: bool,
    input_close: RetainedCloneClose,
}

impl PickerPublication {
    /// 📥 Creates the stable action metadata and pre-funds the eventual Store result.
    pub fn admit(
        picker: &RawMediaPicker, actor: &SharedUtf8, cx: &mut StepContext<'_>,
    ) -> Result<Option<Box<Self>>, OriginalTurnFailure> {
        let Some(loan) = picker.issued_loan() else { return Ok(None); };
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>() + size_of::<Self>() + size_of::<Option<OperationId>>(),
            retained_capacity_bytes: size_of::<Self>(), ..Default::default()
        };
        let combined = progress.checked_add(RawMediaPicker::format_receipt_progress())?
            .checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < 3 || grant.maximum_depth == 0
            || !combined.fits(grant) {
            return Ok(None);
        }
        let Some(receipt) = picker.reserve_format_receipt(cx)? else { return Ok(None); };
        let Some((turn, work)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| actor.admit_clone(effect).result)? else {
            return Ok(None);
        };
        cx.consume_fuel(work as u64);
        let (attached_actor, lease) = turn.map_err(|error| OriginalTurnFailure::debit_original(error, cx))?;
        cx.consume_retained(progress.checked_add(lease)?)?;
        cx.consume_fuel(1);
        Ok(Some(Box::new(Self {
            source: OriginalSourceBinding::borrow(cx),
            meta: Some(ActionMeta { actor: attached_actor, instance_id: loan.instance_id, expected_document_revision: Some(*loan.base_revision), view_state: None }),
            input: None, operation: None, mount_id: None, committed: None, receipt: Some(receipt),
            input_donated: false, input_close: RetainedCloneClose::default(),
        })))
    }

    /// 📦 Transfers the picker pages on the same receiving source before reserved mounting starts.
    pub fn take_input(&mut self, picker: &mut RawMediaPicker, revision: &[u8; 32], current: &str, cx: &mut StepContext<'_>)
        -> Result<bool, OriginalTurnFailure>
    {
        if self.input_donated { return Ok(true); }
        let Some((operation_id, generation, instance_id)) = picker.issued_loan().map(|loan| (loan.operation_id, loan.generation, loan.instance_id)) else { return Ok(false); };
        let live = semio_plugin_decl_media::original_media::OriginalMediaLoan {
            operation_id, generation, instance_id, document_id: current, base_revision: revision,
        };
        self.input_donated = picker.take_format_input(live, &mut self.input, cx)?;
        Ok(self.input_donated)
    }

    /// 📏 Quotes the original input, metadata, or now-empty carrier's next release.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.input.is_some() {
            self.input_close.next_owner_demands::<Box<RawMediaInput>>(true, &None, body)
        } else if self.meta.is_some() || !self.input_close.is_empty() {
            self.input_close.next_owner_demands::<ActionMeta>(self.meta.is_some(), &None, body)
        } else {
            Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<Self>>>(), release_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
        }
    }

    /// ♻️ Returns pixels that were donated to this carrier but did not enter the reserved worker.
    pub fn close_input(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.input.is_none() && self.meta.is_none() && self.input_close.is_empty() { return Ok(true); }
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.close_demand(body)?;
        cx.with_original_retirement(|cx| close_original_effect_step(cx, demand, |effect| {
            if self.input_close.is_empty() {
                let turn = if self.input.is_some() {
                    self.input_close.begin_granted(&mut self.input, effect)
                } else {
                    self.input_close.begin_granted(&mut self.meta, effect)
                };
                RetainedWorkPoll::Complete(RetainedWorkOutcome {
                    work: turn.work,
                    result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default())))
                        .map_err(OriginalRetirementFailure::from),
                })
            } else {
                self.input_close.step_granted(body, effect)
            }
        }))?;
        Ok(self.input.is_none() && self.meta.is_none() && self.input_close.is_empty())
    }

    /// 🧹 Returns the input and releases the slot on the original receiving source.
    pub fn close_mounted(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>, grant: RetainedCloneGrant)
        -> Result<PluginLifecycleStep, PluginLifecycleFailure>
    {
        let Some(owner) = slot.as_ref() else {
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        };
        if !owner.source.matches(cx) {
            return Ok(PluginLifecycleStep::AwaitingInput { reason: "Original picker awaits its retained receiving source" });
        }
        let demand = owner.close_demand(grant.maximum_copy_bytes)?;
        original_lifecycle_step(cx, grant, demand, |same, _| {
            let before = same.retained_progress();
            if slot.as_mut().unwrap().close_input(same)? {
                same.with_original_retirement(|same| Self::retire(slot, same))?;
            }
            let progress = original_lifecycle_delta(before, same.retained_progress())?;
            Ok((PluginLifecycleStep::Progress(progress), false))
        })
    }

    /// 🧾 Releases this carrier only after its separately paid input and mounted job have closed.
    pub fn retire(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if slot.is_none() { return Ok(true); }
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>(),
            released_bytes: size_of::<Self>(), ..Default::default()
        };
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || grant.maximum_depth == 0 || !progress.fits(grant) {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        slot.take();
        Ok(true)
    }
}
