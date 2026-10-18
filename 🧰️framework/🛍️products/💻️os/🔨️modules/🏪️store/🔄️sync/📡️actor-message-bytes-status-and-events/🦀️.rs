//! 📡️ Actor message bytes, status and events: the exact byte measure of an actor message, the remote state, the sync status and the artifact event set. See [`super`].

use super::*;

//#region 🔖️Protocol
pub fn artifact_actor_message_bytes(message: &ArtifactActorMsg) -> Option<usize> {
    fn field(bytes: usize) -> Option<usize> {
        4usize.checked_add(bytes)
    }
    fn add(total: &mut usize, bytes: usize) -> Option<()> {
        *total = total.checked_add(bytes)?;
        (*total <= ARTIFACT_MAILBOX_BYTES).then_some(())
    }
    fn text(total: &mut usize, value: &str) -> Option<()> {
        add(total, field(value.len())?)
    }
    fn optional_text(total: &mut usize, value: Option<&String>) -> Option<()> {
        add(total, 1)?;
        if let Some(value) = value {
            text(total, value)?;
        }
        Some(())
    }
    fn string_vec(total: &mut usize, values: &[String]) -> Option<()> {
        add(total, 4)?;
        for value in values {
            text(total, value)?;
        }
        Some(())
    }
    let mut bytes = 1usize;
    match message {
        ArtifactActorMsg::LocalMutations { envelopes } => {
            add(&mut bytes, 4)?;
            for envelope in envelopes {
                text(&mut bytes, &envelope.mutation_id.0)?;
                text(&mut bytes, &envelope.document_id.0)?;
                text(&mut bytes, &envelope.actor.0)?;
                add(&mut bytes, 4)?;
                for dependency in &envelope.dependencies {
                    text(&mut bytes, &dependency.0)?;
                }
                add(&mut bytes, 1)?;
                if let Some(observed) = &envelope.observed {
                    text(&mut bytes, &observed.0)?;
                }
                add(&mut bytes, 4)?;
                for segment in &envelope.target {
                    text(&mut bytes, segment)?;
                }
                text(&mut bytes, &envelope.diff.schema.0)?;
                add(&mut bytes, field(envelope.diff.payload.len())?)?;
                text(&mut bytes, &envelope.inverse.schema.0)?;
                add(&mut bytes, field(envelope.inverse.payload.len())?)?;
                add(&mut bytes, 24)?;
            }
        }
        ArtifactActorMsg::DocumentBackbone { message } => {
            if message.len() > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || decode_document_backbone_message_exact(message).is_err() {
                return None;
            }
            add(&mut bytes, field(message.len())?)?;
        }
        ArtifactActorMsg::LocalDocumentArchive { archive } => {
            if archive.is_empty() || archive.len() > semio_framework_os_spr::DOCUMENT_ARCHIVE_MAXIMUM_BYTES || archive[0] != 1 {
                return None;
            }
            add(&mut bytes, field(archive.len())?)?;
        }
        ArtifactActorMsg::PresenceHeartbeat { peer } => {
            text(&mut bytes, &peer.actor)?;
            add(&mut bytes, 8)?;
            optional_text(&mut bytes, peer.label.as_ref())?;
            add(&mut bytes, 1)?;
            if let Some(pack) = &peer.presence_pack {
                add(&mut bytes, field(pack.len())?)?;
            }
            optional_text(&mut bytes, peer.user_id.as_ref())?;
            optional_text(&mut bytes, peer.role.as_ref())?;
            optional_text(&mut bytes, peer.drag_ghost_json.as_ref())?;
            add(&mut bytes, 1)?;
            if let Some(interaction) = &peer.interaction {
                text(&mut bytes, &interaction.app_id)?;
                add(&mut bytes, 4)?;
                for domain in &interaction.domains {
                    text(&mut bytes, &domain.domain)?;
                    text(&mut bytes, &domain.granularity)?;
                    string_vec(&mut bytes, &domain.selected)?;
                    string_vec(&mut bytes, &domain.hovered)?;
                }
            }
            add(&mut bytes, 1)?;
            if peer.color.is_some() {
                add(&mut bytes, 1)?;
            }
            optional_text(&mut bytes, peer.surface.as_ref())?;
            add(&mut bytes, 4)?;
            for view in &peer.views {
                text(&mut bytes, &view.window_id)?;
                text(&mut bytes, &view.space)?;
                add(&mut bytes, 1)?;
                add(
                    &mut bytes,
                    match &view.kind {
                        semio_framework_os_spr::PresenceViewKind::Canvas { .. } => 24,
                        semio_framework_os_spr::PresenceViewKind::Orbit { .. } => 80,
                        semio_framework_os_spr::PresenceViewKind::Geo { .. } => 40,
                    },
                )?;
                add(&mut bytes, 17)?;
                if view.pointer.is_some() {
                    add(&mut bytes, 24)?;
                }
            }
            add(&mut bytes, 1)?;
            if let Some(ui) = &peer.ui {
                optional_text(&mut bytes, ui.hovered_path.as_ref())?;
                optional_text(&mut bytes, ui.focused_path.as_ref())?;
                optional_text(&mut bytes, ui.pressed_path.as_ref())?;
            }
            add(&mut bytes, 1)?;
            if let Some(tool_run) = &peer.tool_run {
                text(&mut bytes, &tool_run.tool_id)?;
                add(&mut bytes, 1 + 2 + 8 + 1)?;
                if tool_run.total.is_some() {
                    add(&mut bytes, 8)?;
                }
            }
        }
        ArtifactActorMsg::PublishPreview { key, seq: _, payload } => {
            text(&mut bytes, key)?;
            add(&mut bytes, 8)?;
            add(&mut bytes, field(payload.len())?)?;
        }
        ArtifactActorMsg::Reseed { pack, spr, baseline } => {
            add(&mut bytes, field(pack.len())?)?;
            add(&mut bytes, field(spr.len())?)?;
            text(&mut bytes, &baseline.document_id)?;
            text(&mut bytes, &baseline.head_edit_id)?;
            add(&mut bytes, 48)?;
        }
        ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
    }
    (bytes <= ARTIFACT_MAILBOX_BYTES).then_some(bytes)
}

/// 📶️ Connection state of a document's remote (semio_hub) transport.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RemoteState {
    Detached,
    Connecting,
    Live { peer_count: usize },
    Backoff { retry_in_ms: u64 },
}

/// 🚦️ Snapshot of a document's sync health for status badges.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactSyncStatus {
    pub persisted: bool,
    pub pending_mutations: usize,
    pub remote: RemoteState,
    /// 📌️ The edited head the hub acknowledged, present only while nothing local is unacknowledged —
    /// the head a hub Check In names. `None` before the first edit and while mutations are in flight.
    pub acknowledged_head: Option<semio_framework_os_directory::EditedArtifactFrontierV1>,
}

impl Default for ArtifactSyncStatus {
    fn default() -> Self {
        Self { persisted: false, pending_mutations: 0, remote: RemoteState::Detached, acknowledged_head: None }
    }
}

/// 📬️ Actor → subscriber events, delivered on the broadcast channel from {@link ArtifactHost::subscribe}.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactEvent {
    /// 🕸️ Remote operations (semio_hub fan-out or appended external edits) — also pushed into the store's
    /// inbound queue so `store.tick()` materializes them.
    RemoteMutations {
        #[value(serialize_with = "envelope_serde::to_value", deserialize_with = "envelope_serde::from_value")]
        envelopes: Vec<MutationEnvelope>,
    },
    /// 🪢️ One canonical Hub mutation batch for an exact mounted guest document port.
    DocumentBackbone { message: Vec<u8> },
    /// 🗃️ The whole root plus recursive owned-member closure was replaced atomically.
    DocumentArchiveReplaced { archive: Vec<u8> },
    /// 🫙️ The bound folder holds no archive of this document: answered once per open, by the first answered folder read alone,
    /// and never after an archive was read or written.
    DocumentArchiveAbsent,
    /// 📈️ Monotonic, bounded progress for one descriptor-bound artifact bootstrap.
    /// A new transfer starts at zero after reconnect; no progress event implies a committed frontier.
    BootstrapProgress { received_bytes: u64, total_bytes: u64, received_chunks: u32, total_chunks: u32 },
    /// 🚦️ Sync status changed.
    Status(ArtifactSyncStatus),
    /// 📡️ The presence roster changed.
    Presence { peers: Vec<PresencePeer> },
    /// 🛟️ The hub made this document rebuild from its canonical checkpoint (fan-out lag, an approval checkpoint): the actor
    /// dropped its projection and socket, keeps its unacked work and waits for the host's [`ArtifactActorMsg::Reseed`] with the
    /// pair `control` names — the pair only the host's directory client fetches and only the host can load into the guest.
    RebootstrapRequired { control: semio_framework_os_directory::RebootstrapRequired },
    /// 🎨️ The hub assigned (or re-confirmed, on reconnect) this connection's session color —
    /// `crate::os_spr::wire::ServerFrame::Session`, sent once per connection after `Welcome`. The
    /// actor stores it and stamps it onto every outbound `PresenceHeartbeat` via {@link stamp_session}.
    Session { actor: String, color: u8 },
    /// 👻️ A peer published an ephemeral preview blob (`crate::os_spr::wire::ServerFrame::Preview`)
    /// on the uncredited, loss-tolerant preview lane — the counterpart of
    /// {@link ArtifactActorMsg::PublishPreview}.
    Preview { actor: semio_framework_value::SharedUtf8, key: String, seq: u64, payload: Vec<u8> },
    /// 📮️ The semio_hub's terminal disposition for one outbound `Commands` batch
    /// (`crate::os_spr::wire::ServerFrame::Ack`'s `Applied` stage) — accepted as-is, transformed against
    /// concurrent history (the transformed envelope is already delivered as a
    /// {@link ArtifactEvent::RemoteMutations} replacing the speculative local one), or rejected
    /// (the speculative local head is rolled back via {@link rollback_envelope} before this fires).
    CommandOutcome {
        #[value(rename = "batchId")]
        batch_id: u64,
        outcome: CommandAckOutcome,
    },
    /// ⚠️ A structural conflict (external divergence with local pending operations / semio_hub
    /// protocol-level reject), on the frozen diagnostic-bag vocabulary (contract freeze `26/08/16/
    /// MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10) rather than the deleted
    /// per-domain conflict-bag type this used to wrap: `level` is `Error` (this actor has no finer
    /// severity signal from either source), `target` carries the diverged folder uri / hub base url.
    /// This is a transport-level diagnostic, never a first-class `crate::os_spr::Conflict` (no
    /// `id`/`status`/`actors`/`timestamp` exists for either source event).
    Conflict(MutationMessage),
}
//#endregion 🔖️Protocol
