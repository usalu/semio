//! 🌐 Exposes the mounted import route to the natural publication driver.

use super::*;
use semio_framework_job::Operation;
use semio_plugin_decl_media::original_media::natural_source::{NaturalFilePagedSource, NaturalFileSourceGuest};
use semio_plugin_instance_natural_publication::{NaturalFilePublication, NaturalMount, NaturalPublicationHost};

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> NaturalPublicationHost for VcsArtifactApp<A, M> {
    fn natural_slot(&mut self) -> &mut Option<Box<NaturalFilePublication>> { &mut self.media.natural_source }
    fn natural_live_instance(&self) -> Option<u32> { self.tools.live_runtime_instance_id }
    fn natural_actor(&self) -> &semio_framework_value::SharedUtf8 { &self.stores.store.local_actor_id().0 }
    fn natural_receipt_operation(&self, cx: &mut StepContext<'_>) -> Result<Option<Operation>, OriginalTurnFailure> {
        self.media.reserved_receipt.map(|receipt| receipt.operation(cx)).transpose().map_err(Into::into)
    }
    fn natural_mount(&mut self, operation: Operation) -> Option<NaturalMount<'_>> {
        self.tools.tool_operations.get_mut(operation.operation.0).map(|mounted| NaturalMount {
            operation: mounted.operation.operation, published: mounted.published_artifact, cancel_requested: &mut mounted.user_cancel_requested,
        })
    }
    fn natural_maintenance_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.maintenance_source_terminal_is_empty(cx) { return Ok(false); }
        let grant = cx.retained_grant();
        self.maintenance_source_close_step(cx, grant).map_err(OriginalTurnFailure::from)?;
        Ok(true)
    }
    fn natural_dispatch<'a>(
        &'a mut self, input: &'a mut Option<NaturalFilePagedSource>, meta: &'a ActionMeta, operation: &'a mut Option<Operation>,
        cx: &'a mut StepContext<'_>,
    ) -> impl Future<Output = Result<(), OriginalTurnFailure>> + 'a {
        async move { self.dispatch_ctx().advance_original_reserved_natural_source(input, meta, operation, cx).await.map(drop) }
    }
}

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    pub(super) fn natural_source_can_close(&self) -> bool {
        self.tools.tool_operations.is_empty() && self.pending.reserved_preparation.is_none()
            && self.pending.reserved_birth.as_ref().is_none_or(semio_framework_job::OriginalConstructionAdmission::terminal_is_empty) && self.media.reserved_mount.is_none()
            && self.media.reserved_receipt.is_none() && self.media.reserved_mount_construction.terminal_is_empty()
            && self.media.pending_reserved_producer_permit.is_none()
    }

    pub(super) async fn advance_natural_publication(&mut self, source: &mut NaturalFileSourceGuest, cx: &mut StepContext<'_>)
        -> Result<(), OriginalTurnFailure>
    {
        semio_plugin_instance_natural_publication::advance_natural_publication(self, source, cx).await
    }
}
