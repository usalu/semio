//! 🪂 Owns the paid natural input and drives its publication through any host that exposes the mounted route.

use semio_framework_job::{Operation, OperationId, OriginalSourceBinding, OriginalTurnFailure, StepContext, close_original_effect_step};
use semio_framework_value::{
    OriginalRetirementFailure, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
    RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, SharedUtf8, ValueError, retained_clone::RetainedCloneClose,
};
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_decl_media::original_media::natural_source::{NaturalFilePagedSource, NaturalFileSourceGuest};
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_media::original_media::natural_source_gate::NaturalSourceDeclaration;
use std::future::Future;
use semio_plugin_ownership::bounded_ownership::{PluginLifecycleFailure, PluginLifecycleStep, original_lifecycle_delta, original_lifecycle_step};

#[path = "🎟️picker/🦀️.rs"]
pub mod picker;

pub struct NaturalFilePublication {
    pub source: OriginalSourceBinding,
    pub meta: Option<ActionMeta>,
    pub input: Option<NaturalFilePagedSource>,
    pub operation: Option<Operation>,
    pub mount_id: Option<OperationId>,
    pub committed: bool,
    input_donated: bool,
    close: RetainedCloneClose,
}

impl NaturalFilePublication {
    /// 📥 Retains the actual local actor and reserves the terminal Store witness before mounting.
    pub fn admit(instance: u32, actor: &SharedUtf8, cx: &mut StepContext<'_>) -> Result<Option<Box<Self>>, OriginalTurnFailure> {
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>() + size_of::<Self>() + size_of::<bool>(),
            retained_capacity_bytes: size_of::<Self>(), ..Default::default()
        };
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < 2 || grant.maximum_depth == 0
            || grant.maximum_items < 2 || !progress.fits(grant) {
            return Ok(None);
        }
        let mut attached = None;
        cx.with_retained_loan(RetainedCloneGrant {
            maximum_items: grant.maximum_items - progress.copied_items,
            maximum_copy_bytes: grant.maximum_copy_bytes - progress.copied_bytes,
            maximum_capacity_bytes: grant.maximum_capacity_bytes - progress.retained_capacity_bytes,
            ..grant
        }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - 1, u64::MAX, |cx| {
            let Some((turn, work)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| actor.admit_clone(effect).result)? else {
                return Ok(());
            };
            cx.consume_fuel(work as u64);
            let (actor, lease) = turn.map_err(|error| OriginalTurnFailure::debit_original(error, cx))?;
            cx.consume_retained(lease)?;
            attached = Some(actor);
            Ok(())
        }))?;
        let Some(actor) = attached else { return Ok(None); };
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(Some(Box::new(Self {
            source: OriginalSourceBinding::borrow(cx),
            meta: Some(ActionMeta { actor, instance_id: instance, expected_document_revision: None, view_state: None }),
            input: None, operation: None, mount_id: None, committed: false, input_donated: false, close: Default::default(),
        })))
    }

    /// 🎟️ Takes the sealed original without copying any of its bytes.
    pub fn take_source(&mut self, source: &mut NaturalFileSourceGuest, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.input_donated { return Ok(true); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..Default::default() };
        let grant = cx.retained_grant();
        if grant.maximum_items < 2 || cx.fuel_remaining() < 2 || !progress.fits(grant) { return Ok(false); }
        let mut donated = false;
        cx.with_retained_loan(RetainedCloneGrant {
            maximum_items: grant.maximum_items - progress.copied_items,
            maximum_copy_bytes: grant.maximum_copy_bytes - progress.copied_bytes,
            ..grant
        }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - 1, u64::MAX, |cx| {
            donated = source.take_source(source.loan().operation, true, &mut self.input, cx)?;
            Ok(())
        }))?;
        if donated {
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            self.input_donated = true;
        }
        Ok(donated)
    }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(input) = self.input.as_ref() {
            return if input.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<NaturalFilePagedSource>>(), depth: 1, ..Default::default() })
            } else { input.next_demand(body) };
        }
        if self.meta.is_some() || !self.close.is_empty() {
            return self.close.next_owner_demands::<ActionMeta>(self.meta.is_some(), &None, body);
        }
        Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<Self>>>(), release_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }

    /// ♻️ Returns undonated bytes one backing at a time and closes the retained action metadata.
    pub fn close_input(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.input.is_none() && self.meta.is_none() && self.close.is_empty() { return Ok(true); }
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.close_demands(body)?;
        if self.input.as_ref().is_some_and(|input| input.terminal_is_empty()) {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
            if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0
                || !progress.fits(cx.retained_grant()) { return Ok(false); }
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            self.input.take();
            return Ok(false);
        }
        cx.with_original_retirement(|cx| close_original_effect_step(cx, demand, |effect| {
            if let Some(input) = self.input.as_mut() { return input.step(body, effect); }
            if !self.close.is_empty() { return self.close.step_granted(body, effect); }
            let turn = self.close.begin_granted(&mut self.meta, effect);
            RetainedWorkPoll::Complete(RetainedWorkOutcome {
                work: turn.work,
                result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default())))
                    .map_err(OriginalRetirementFailure::from),
            })
        }))?;
        Ok(self.input.is_none() && self.meta.is_none() && self.close.is_empty())
    }

    pub fn close_mounted(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>, grant: RetainedCloneGrant)
        -> Result<PluginLifecycleStep, PluginLifecycleFailure>
    {
        let Some(owner) = slot.as_ref() else { return Ok(PluginLifecycleStep::Complete(Default::default())); };
        if !owner.source.matches(cx) {
            return Ok(PluginLifecycleStep::AwaitingInput { reason: "natural source awaits its retained receiving source" });
        }
        let demand = owner.close_demands(grant.maximum_copy_bytes)?;
        original_lifecycle_step(cx, grant, demand, |same, _| {
            let before = same.retained_progress();
            if slot.as_mut().expect("natural source owner is mounted").close_input(same)? {
                same.with_original_retirement(|same| Self::retire(slot, same))?;
            }
            let progress = original_lifecycle_delta(before, same.retained_progress())?;
            Ok((PluginLifecycleStep::Progress(progress), false))
        })
    }

    pub fn retire(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>(), released_bytes: size_of::<Self>(), ..Default::default()
        };
        if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0
            || !progress.fits(cx.retained_grant()) { return Ok(false); }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        slot.take();
        Ok(true)
    }
}

/// 🔌 Borrowed view of the mounted import operation that the natural owner routes through.
pub struct NaturalMount<'a> {
    pub operation: OperationId,
    pub published: bool,
    pub cancel_requested: &'a mut bool,
}

/// 🏠 Narrow host surface the natural turn driver needs from the mounted app.
pub trait NaturalPublicationHost {
    fn natural_slot(&mut self) -> &mut Option<Box<NaturalFilePublication>>;
    fn natural_live_instance(&self) -> Option<u32>;
    fn natural_actor(&self) -> &SharedUtf8;
    fn natural_receipt_operation(&self, cx: &mut StepContext<'_>) -> Result<Option<Operation>, OriginalTurnFailure>;
    fn natural_mount(&mut self, operation: Operation) -> Option<NaturalMount<'_>>;
    fn natural_maintenance_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>;
    fn natural_dispatch<'a>(
        &'a mut self, input: &'a mut Option<NaturalFilePagedSource>, meta: &'a ActionMeta,
        operation: &'a mut Option<Operation>, cx: &'a mut StepContext<'_>,
    ) -> impl Future<Output = Result<(), OriginalTurnFailure>> + 'a;
}

/// 🧭 Advances the staged natural input through admission, mounting, cancellation and format retirement.
pub async fn advance_natural_publication<H: NaturalPublicationHost>(
    host: &mut H, source: &mut NaturalFileSourceGuest, cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    if host.natural_slot().is_none() {
        if !source.input_ready() { return Ok(()); }
        let instance = source.loan().operation.instance;
        if host.natural_live_instance() != Some(instance) {
            return Err(ValueError::invariant("natural source named another mounted instance").into());
        }
        let admitted = NaturalFilePublication::admit(instance, host.natural_actor(), cx)?;
        *host.natural_slot() = admitted;
        return Ok(());
    }
    let mut owner = host.natural_slot().take();
    let result = advance_owner(host, source, &mut owner, cx).await;
    *host.natural_slot() = owner;
    result
}

async fn advance_owner<H: NaturalPublicationHost>(
    host: &mut H, source: &mut NaturalFileSourceGuest, slot: &mut Option<Box<NaturalFilePublication>>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    let owner = slot.as_mut().expect("natural original publication owner");
    if !owner.source.matches(cx) {
        return Err(ValueError::invariant("natural source changed its Original publication authority").into());
    }
    let cancelled = source.cancellation_pending();
    if owner.operation.is_none() && cancelled {
        owner.operation = host.natural_receipt_operation(cx)?;
    }
    if let Some(operation) = owner.operation {
        if let Some(mounted) = host.natural_mount(operation) {
            if owner.mount_id != Some(mounted.operation) {
                return Err(ValueError::invariant("natural source mounted operation changed its route").into());
            }
            if cancelled && !mounted.published && !*mounted.cancel_requested {
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..Default::default() };
                if !cx.retirement_should_yield() && cx.retained_grant().maximum_depth > 0
                    && progress.fits(cx.retained_grant()) && cx.fuel_remaining() != 0 {
                    cx.consume_retained(progress)?;
                    cx.consume_fuel(1);
                    *mounted.cancel_requested = true;
                }
            }
            return Ok(());
        }
    } else if !cancelled {
        if !owner.take_source(source, cx)? { return Ok(()); }
        let meta = owner.meta.as_ref().expect("natural source mount owns action metadata");
        host.natural_dispatch(&mut owner.input, meta, &mut owner.operation, cx).await?;
        if owner.mount_id.is_none() {
            let route = match owner.operation { Some(operation) => Some(operation), None => host.natural_receipt_operation(cx)? };
            owner.mount_id = route.map(|operation| operation.operation);
        }
        return Ok(());
    }
    if host.natural_maintenance_step(cx)? { return Ok(()); }
    if !owner.close_input(cx)? { return Ok(()); }
    if !cancelled && !source.finish_format(source.loan().operation, owner.committed, cx)? { return Ok(()); }
    cx.with_original_retirement(|cx| NaturalFilePublication::retire(slot, cx))?;
    Ok(())
}

/// 📐 Projects the mounted app's paged decoder declaration and byte budget as the natural source capability.
pub fn natural_source_capability<A: ArtifactApp>() -> Option<(NaturalSourceDeclaration<'static>, u32)> {
    let maximum = A::natural_file_paged_maximum_bytes();
    if maximum == 0 { return None; }
    let codec = semio_framework_async::poll::resolve_ready(A::natural_file_codec())?;
    if !codec.binary { return None; }
    Some((NaturalSourceDeclaration { format_kind: codec.format_kind, extension: codec.extension, media_type: codec.media_type, binary: codec.binary }, maximum))
}
