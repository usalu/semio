//! 🧾️ Typed operation results of the mounted artifact instance: the public tool contracts, the acknowledgement of a result page and the terminal completion with its history patch. See [`super`].

use super::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    pub(super) async fn tool_public_contracts(&self) -> Vec<ArtifactToolPublicContract> {
        self.tools.bounded_tool_contracts
            .iter()
            .cloned()
            .chain(self.tools.framework_tool_registrations.values().map(|registration| ArtifactToolPublicContract {
                owner: registration.owner,
                controller_id: registration.key.controller_id.clone(),
                tool_id: registration.key.tool_id.clone(),
                schema_id: registration.schema_id.clone(),
                max_raw_wire_bytes: registration.contract.max_raw_wire_bytes,
                publication_lanes: registration.publication_lanes,
            }))
            .chain(self.tools.app_tool_registrations.values().map(|registration| ArtifactToolPublicContract {
                owner: registration.owner,
                controller_id: registration.key.controller_id.clone(),
                tool_id: registration.key.tool_id.clone(),
                schema_id: registration.schema_id.clone(),
                max_raw_wire_bytes: registration.contract.max_raw_wire_bytes,
                publication_lanes: registration.publication_lanes,
            }))
            .collect()
    }

    pub(super) fn acknowledge_typed_operation_result(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault> {
        if token.receiver != self.tools.live_runtime_instance_id.unwrap_or(u32::MAX) {
            return Ok(false);
        }
        let download_ack = self.tools.tool_operations.get(token.operation).and_then(|operation| operation.result_page.as_ref().map(|page| (operation, page))).is_some_and(|(operation, page)| {
            page.token == token
                && page.lane == TypedOperationResultLane::Download
                && operation.stage == MountedTypedCommandFullOperationStage::AwaitingAck
                && matches!(operation.publication.as_ref(), Some(ArtifactToolCompletionValue::Download(Ok(_), _)))
        });
        if download_ack && (!self.media.segmented_downloads.can_insert(token.operation) || !self.media.segmented_closures.can_insert(token.operation)) {
            return Ok(false);
        }
        let Some(operation) = self.tools.tool_operations.get_mut(token.operation) else { return Ok(false) };
        let accepted = operation.acknowledge_result_page(token)?;
        if !accepted || !download_ack {
            return Ok(accepted);
        }
        let publication = operation.publication.take().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-owner"), "download ACK lost its exact retained publication owner"))?;
        let ArtifactToolCompletionValue::Download(Ok(download), ephemeral) = publication else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-owner"), "download ACK resolved a non-download retained publication owner"));
        };
        if !ephemeral.presence.is_empty() || !ephemeral.transient.is_empty() {
            operation.publication = Some(ArtifactToolCompletionValue::Download(Ok(download), ephemeral));
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-ephemeral"), "download ACK preceded retained ephemeral publication"));
        }
        self.media.segmented_downloads.insert_admitted(token.operation, download);
        Ok(true)
    }

    /// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: the dirty set is only authoritative AFTER the
    /// backfill. A typed operation publishes its edit through
    /// [`Self::publish_mounted_typed_operation_unit`], which stages a batched `Edit` and never calls
    /// `record_command` — that edit reaches the command log exclusively through
    /// [`Self::backfill_command_log`], which runs inside [`Self::refresh_cache`], which
    /// [`Self::history_patch`] runs FIRST. Reading `history_dirty_sequences` before that backfill
    /// therefore answered "nothing dirty" on the very call whose publication had just landed, so the
    /// terminal completion carried `history_patch: None` and the row surfaced on whatever later call
    /// happened to refresh the cache — the one-command lag measured in the browser on wasm #47
    /// (`📓️2026-09-12-wave-B19-mutation-lane-regression.md` §3).
    ///
    /// 🧾️ Lane-less rows (a View, a Mutation that emitted nothing) join this patch. Retirement
    /// still records them, but only after this function has already read `history_dirty_sequences`.
    pub(super) async fn take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault> {
        let Some(witness) = self.tools.typed_completion_outbox.pop() else { return Ok(None) };
        self.tools.record_settled_typed_operation_command(&mut self.history, &mut self.host_events, witness.operation);
        self.render_ctx().refresh_cache().await?;
        let history_patch = if self.history.history_dirty_sequences.is_empty() { None } else { Some(self.render_ctx().history_patch(false).await?) };
        let revision = self.stores.store.content_revision_now();
        Ok(Some(TypedOperationCompletion { operation: witness.operation, revision: u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")), ui_scope: witness.ui_scope, history_patch }))
    }
}
