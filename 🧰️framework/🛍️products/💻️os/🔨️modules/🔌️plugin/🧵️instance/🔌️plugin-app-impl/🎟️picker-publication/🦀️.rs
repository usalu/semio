//! 🪝️ Connects the picker source to reserved mounting, typed publication, and original close.

use super::*;
use semio_plugin_decl_media::original_media::{FormatResult, RawMediaPicker};
use semio_plugin_instance_natural_publication::picker::PickerPublication;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    pub(super) fn observe_picker_commit(&mut self, picker: &mut RawMediaPicker, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        let Some(work) = self.media.original_picker.as_mut() else { return Ok(()); };
        if work.committed.is_some() {
            if let Some(receipt) = work.receipt.take() {
                if !picker.publish_format_receipt(receipt, FormatResult::Published, cx)? {
                    return Err(ValueError::invariant("picker Store receipt lost its Original destination").into());
                }
            }
        }
        Ok(())
    }

    /// 💰 Quotes the picker close once the mounted typed operation has released its loan.
    pub(super) fn picker_close_demands(&self, body: usize) -> Option<Result<RetirementDemand, ValueError>> {
        self.media.original_picker.as_ref().filter(|_| self.tools.tool_operations.is_empty())
            .map(|owner| owner.close_demand(body))
    }

    pub(super) fn close_picker(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant)
        -> Option<Result<PluginLifecycleStep, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>>
    {
        (self.tools.tool_operations.is_empty() && self.media.original_picker.is_some())
            .then(|| PickerPublication::close_mounted(&mut self.media.original_picker, cx, grant))
    }

    /// 🎞️ Advances the exact typed picker request and keeps its source until the terminal result is returned.
    pub(super) async fn advance_picker_publication(&mut self, picker: &mut RawMediaPicker, cx: &mut StepContext<'_>)
        -> Result<(), OriginalTurnFailure>
    {
        self.observe_picker_commit(picker, cx)?;
        if self.media.original_picker.is_none() {
            if picker.format_input().is_none() { return Ok(()); }
            self.media.original_picker = PickerPublication::admit(picker, &self.stores.store.local_actor_id().0, cx)?;
            return Ok(());
        }
        let mut owner = self.media.original_picker.take();
        let result = self.advance_picker_owner(picker, &mut owner, cx).await;
        self.media.original_picker = owner;
        result
    }

    async fn advance_picker_owner(
        &mut self, picker: &mut RawMediaPicker, slot: &mut Option<Box<PickerPublication>>, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        let owner = slot.as_mut().unwrap();
        if !owner.source.matches(cx) { return Err(ValueError::invariant("picker changed its Original publication source").into()); }
        let cancelled = picker.cancellation_pending();
        let stale = owner.operation.is_none() && owner.committed.is_none() && picker.issued_loan().is_some_and(|loan|
            loan.document_id != self.stores.store.envelope().id || loan.base_revision != &self.stores.store.content_revision());
        let cancelling = cancelled || stale;
        if owner.operation.is_none() && cancelling {
            if let Some(receipt) = self.media.reserved_receipt {
                owner.operation = Some(receipt.operation(cx)?);
            }
        }
        if let Some(operation) = owner.operation {
            let id = operation.operation.0;
            if let Some(mounted) = self.tools.tool_operations.get_mut(id) {
                if owner.mount_id != Some(mounted.operation.operation) { return Err(ValueError::invariant("picker mounted operation changed its route").into()); }
                if cancelling && !mounted.published_artifact && !mounted.user_cancel_requested {
                    let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..Default::default() };
                    if !cx.retirement_should_yield() && cx.retained_grant().maximum_depth > 0
                        && progress.fits(cx.retained_grant()) && cx.fuel_remaining() != 0 {
                        cx.consume_retained(progress)?;
                        cx.consume_fuel(1);
                        mounted.user_cancel_requested = true;
                    }
                }
                return Ok(());
            }
        } else if !cancelling {
            let revision = self.stores.store.content_revision();
            if !owner.take_input(picker, &revision, &self.stores.store.envelope().id, cx)? { return Ok(()); }
            self.dispatch_ctx().advance_original_reserved_picker(&mut owner.input, owner.meta.as_ref().expect("picker mount owns retained action metadata"), &mut owner.operation, cx).await?;
            if owner.mount_id.is_none() {
                if let Some(operation) = owner.operation {
                    owner.mount_id = Some(operation.operation);
                } else if let Some(receipt) = self.media.reserved_receipt {
                    owner.mount_id = Some(receipt.operation(cx)?.operation);
                }
            }
            return Ok(());
        }
        if !self.maintenance_source_terminal_is_empty(cx) {
            let grant = cx.retained_grant();
            self.maintenance_source_close_step(cx, grant).map_err(semio_framework_job::OriginalTurnFailure::from)?;
            return Ok(());
        }
        if !owner.close_input(cx)? { return Ok(()); }
        if !cancelled {
            if let Some(receipt) = owner.receipt.take() {
                if !picker.publish_format_receipt(receipt, FormatResult::Refused, cx)? {
                    return Err(ValueError::invariant("picker refusal lost its Original destination").into());
                }
            }
        }
        cx.with_original_retirement(|cx| PickerPublication::retire(slot, cx))?;
        Ok(())
    }
}
