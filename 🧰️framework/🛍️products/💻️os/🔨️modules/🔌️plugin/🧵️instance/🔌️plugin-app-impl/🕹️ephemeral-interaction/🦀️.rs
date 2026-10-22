//! 🕹️ Ephemeral interaction of the mounted artifact instance: the local interaction query lane and the ephemeral snapshot of the interaction and presence roots. See [`super`].

use super::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    pub(super) fn begin_local_interaction_query(&mut self, request_id: u64, query_generation: u64) -> Option<protocol::LocalInteractionQueryReply> {
        if self.close.close_started || self.tools.live_runtime_instance_id.is_none() {
            return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Closed });
        }
        if self.interaction.local_interaction_query.is_some() {
            return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Busy });
        }
        let document_revision = self.stores.store.content_revision_now();
        let config_revision = self.stores.config_store.content_revision_now();
        let topology_revision = match self.interaction.local_interaction_authority.revision(document_revision, config_revision) {
            Ok(revision) => revision,
            Err(_) => return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Closed }),
        };
        let identity = protocol::LocalInteractionIdentity {
            app_instance_id: self.tools.live_runtime_instance_id.expect("live instance checked"),
            generation: self.stores.interaction_store.generation_now(),
            revision: self.stores.interaction_store.content_revision_now(),
            document_revision,
            topology_revision,
        };
        self.interaction.local_interaction_query =
            Some(semio_framework_plugin_interaction::live::LocalInteractionLiveQuery::new(request_id, query_generation, identity, self.stores.store.snapshot_read().ok(), self.stores.config_store.snapshot_read().ok(), self.stores.interaction_store.snapshot_read().ok()));
        self.interaction.local_interaction_emitted_bytes = 0;
        self.interaction.local_interaction_retired_bytes = 0;
        None
    }

    pub(super) fn publish_local_interaction_query_reply(&mut self, frames: &mut Vec<Vec<u8>>, maximum_frames: usize) -> bool {
        let Some(query) = self.interaction.local_interaction_query.as_mut() else {
            return false;
        };
        if !query.reply_ready() || frames.len() >= maximum_frames {
            return false;
        }
        let mut encoded = Vec::new();
        if frames.try_reserve_exact(1).is_err() || encoded.try_reserve_exact(512).is_err() {
            return false;
        }
        let mut encoded = Some(encoded);
        let delivered = query
            .take_reply_admitted(|reply| {
                if protocol::encode_local_interaction_query_frame_into(reply, encoded.as_mut().expect("one admitted reply buffer")).is_err() {
                    return false;
                }
                frames.push(encoded.take().expect("one admitted reply buffer"));
                true
            })
            .is_some();
        if query.terminal_is_empty() {
            self.interaction.local_interaction_query = None;
        }
        delivered
    }

    /// 🎯️ Empty domains ⇒ empty bytes (contract-freeze §C7.6) — `encode_presence_interaction`
    /// itself always writes at least `app_id` + a domain count, so an explicit short-circuit
    /// is required rather than trusting the codec's own "nothing to say" shape.
    ///
    /// 👤️ Generation 0 ⇒ empty bytes, the presence twin of the interaction short-circuit above: a
    /// local presence root that was never published has nothing to say, and `encode_pack` always
    /// writes a record header, so an untouched lane would otherwise ship a pack on every frame.
    pub(super) async fn ephemeral_snapshot(&self) -> EphemeralSnapshot {
        let hover_specs: BTreeMap<String, protocol::HoverSpec> = self.tools.registry.interactions().await.map(|def| (def.id.clone(), def.hover.clone())).collect();
        let selection_specs: BTreeMap<String, protocol::SelectionSpec> = self.tools.registry.interactions().await.map(|def| (def.id.clone(), def.selection.clone())).collect();
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction = protocol::assemble_presence_interaction(self.app_id().await, &interaction_state, &hover_specs, &selection_specs).await;
        let interaction_bytes = if interaction.domains.is_empty() {
            Vec::new()
        } else {
            let mut bytes = Vec::new();
            protocol::encode_presence_interaction(&interaction, &mut bytes).await;
            bytes
        };
        let presence_generation = self.stores.presence_store.generation().await;
        EphemeralSnapshot {
            presence: if presence_generation == 0 { Vec::new() } else { self.stores.presence_store.local().encode_pack() },
            presence_generation,
            transient_generation: self.stores.transient_store.generation().await,
            interaction: interaction_bytes,
            tool_run: self.tool_runs.presence(),
            history_edit: self.time_travel.presence(),
        }
    }
}
