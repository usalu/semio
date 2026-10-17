//! 📤️ Presence peer encoding: the peer, tool run and history edit encoders and the peer wire limits. See [`super`].

use super::*;

//#region 🔖️Presence
/// 🎯️ Binary `PresencePeer` codec: `actor str | flags varint_u64 | connected_at_ms varint |
/// fields present per bitmask, strictly in bit order`. `protocol_wire::ClientFrame::Presence`/
/// `ServerFrame::Presence` carry the resulting bytes opaquely (that crate has no dependency on this
/// one) — this is the encode/decode pair store_sync calls on either side of the wire.
/// `presence_pack` is length-prefixed bytes in flag bit 1; `drag_ghost_json` stays opaque app-owned
/// text (never re-parsed as JSON here, same as `ArtifactDiff.payload` staying opaque bytes). Bit 5
/// carries `interaction` via `encode_presence_interaction`/`decode_presence_interaction`
/// (self-delimiting varint-counted fields — see the `🔖️PresenceInteraction` region below); bit 8
/// (`views`) is set iff non-empty; bit 9 (`ui`) carries three `opt_str` fields unconditionally once
/// present. `flags` widened from a single `u8` to a varint (ticket 26/08/17/SHARED-PRESENCE-SESSION-
/// COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.1) now that bit 9 exceeds a byte's range. Bit 10 carries
/// `tool_run` as `tool_id str | state u8 | stage varint | completed varint | total bool+varint?`.
/// Bit 11 carries `principal_kind` as one declaration-order tag byte (`0` human, `1` agent); bit 12 carries `active_tool`;
/// bit 13 carries `history_edit` as `mutation_id str | stage u8 | drafts varint`; bit 14 (`typing`) is set iff non-empty and
/// carries `count varint | (window_id str | deleted str | insert str)*`; a peer
/// that leaves it unset is the pre-agent wire shape and reads back as `None`, so every encoder and
/// decoder written before the agent principal existed still round-trips byte-for-byte.
pub async fn encode_presence_peer(peer: &PresencePeer) -> Vec<u8> {
    let mut out = Vec::new();
    semio_framework_replication_wire_core::write_str(&mut out, &peer.actor);
    let mut flags = 0u64;
    if peer.label.is_some() {
        flags |= 1 << 0;
    }
    if peer.presence_pack.is_some() {
        flags |= 1 << 1;
    }
    if peer.user_id.is_some() {
        flags |= 1 << 2;
    }
    if peer.role.is_some() {
        flags |= 1 << 3;
    }
    if peer.drag_ghost_json.is_some() {
        flags |= 1 << 4;
    }
    if peer.interaction.is_some() {
        flags |= 1 << 5;
    }
    if peer.color.is_some() {
        flags |= 1 << 6;
    }
    if peer.surface.is_some() {
        flags |= 1 << 7;
    }
    if !peer.views.is_empty() {
        flags |= 1 << 8;
    }
    if peer.ui.is_some() {
        flags |= 1 << 9;
    }
    if peer.tool_run.is_some() {
        flags |= 1 << 10;
    }
    if peer.principal_kind.is_some() {
        flags |= 1 << 11;
    }
    if peer.active_tool.is_some() {
        flags |= 1 << 12;
    }
    if peer.history_edit.is_some() {
        flags |= 1 << 13;
    }
    if !peer.typing.is_empty() {
        flags |= 1 << 14;
    }
    semio_framework_replication_wire_core::write_varint_u64(&mut out, flags);
    semio_framework_replication_wire_core::write_varint_u64(&mut out, peer.connected_at_ms as u64);
    if let Some(label) = &peer.label {
        semio_framework_replication_wire_core::write_str(&mut out, label);
    }
    if let Some(presence_pack) = &peer.presence_pack {
        semio_framework_replication_wire_core::write_bytes(&mut out, presence_pack);
    }
    if let Some(user_id) = &peer.user_id {
        semio_framework_replication_wire_core::write_str(&mut out, user_id);
    }
    if let Some(role) = &peer.role {
        semio_framework_replication_wire_core::write_str(&mut out, role);
    }
    if let Some(drag_ghost_json) = &peer.drag_ghost_json {
        semio_framework_replication_wire_core::write_str(&mut out, drag_ghost_json);
    }
    if let Some(interaction) = &peer.interaction {
        encode_presence_interaction(interaction, &mut out).await;
    }
    if let Some(color) = peer.color {
        out.push(color);
    }
    if let Some(surface) = &peer.surface {
        semio_framework_replication_wire_core::write_str(&mut out, surface);
    }
    if !peer.views.is_empty() {
        write_vec_presence_window_view(&mut out, &peer.views).await;
    }
    if let Some(ui) = &peer.ui {
        encode_presence_ui(ui, &mut out).await;
    }
    if let Some(tool_run) = &peer.tool_run {
        encode_presence_tool_run(tool_run, &mut out);
    }
    if let Some(principal_kind) = peer.principal_kind {
        out.push(principal_kind as u8);
    }
    if let Some(active_tool) = &peer.active_tool {
        semio_framework_replication_wire_core::write_str(&mut out, active_tool);
    }
    if let Some(history_edit) = &peer.history_edit {
        encode_presence_history_edit(history_edit, &mut out);
    }
    if !peer.typing.is_empty() {
        semio_framework_replication_wire_core::write_varint_u64(&mut out, peer.typing.len() as u64);
        for typing in &peer.typing {
            semio_framework_replication_wire_core::write_str(&mut out, &typing.window_id);
            semio_framework_replication_wire_core::write_str(&mut out, &typing.deleted);
            semio_framework_replication_wire_core::write_str(&mut out, &typing.insert);
        }
    }
    out
}

/// ⏯️ Appends one `PresenceToolRun` body — the exact bytes a `PresencePeer` carries under flag bit 10.
/// `pub` so a guest's `AppFrame::Ephemeral` can publish its run summary in the same encoding the heartbeat
/// assembly re-embeds (contract `📋️tool-run-contract.md` §3.4).
pub fn encode_presence_tool_run(tool_run: &PresenceToolRun, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &tool_run.tool_id);
    out.push(tool_run.state as u8);
    semio_framework_replication_wire_core::write_varint_u64(out, u64::from(tool_run.stage));
    semio_framework_replication_wire_core::write_varint_u64(out, tool_run.completed);
    semio_framework_replication_wire_core::write_bool(out, tool_run.total.is_some());
    if let Some(total) = tool_run.total {
        semio_framework_replication_wire_core::write_varint_u64(out, total);
    }
}

/// ⏪️ Appends one `PresenceHistoryEdit` body — the exact bytes a `PresencePeer` carries under flag bit 13, and what a
/// guest's `AppFrame::Ephemeral.history_edit` publishes.
pub fn encode_presence_history_edit(history_edit: &PresenceHistoryEdit, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &history_edit.mutation_id);
    out.push(history_edit.stage as u8);
    semio_framework_replication_wire_core::write_varint_u64(out, u64::from(history_edit.drafts));
}

/// 🛡️ Fixed hostile-input ceilings shared with the TypeScript presence decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresencePeerWireLimitsV1 {
    pub maximum_entry_bytes: usize,
    pub maximum_text_bytes: usize,
    pub maximum_presence_pack_bytes: usize,
    pub maximum_views: usize,
    pub maximum_interaction_domains: usize,
    pub maximum_domain_ids: usize,
    pub maximum_connected_at_ms: u64,
    pub maximum_tool_run_units: u64,
    pub maximum_typing_runs: usize,
}

pub const PRESENCE_PEER_WIRE_LIMITS_V1: PresencePeerWireLimitsV1 = PresencePeerWireLimitsV1 {
    maximum_entry_bytes: 4_096,
    maximum_text_bytes: 1_024,
    maximum_presence_pack_bytes: 2_048,
    maximum_views: 16,
    maximum_interaction_domains: 16,
    maximum_domain_ids: 64,
    maximum_connected_at_ms: 9_007_199_254_740_991,
    maximum_tool_run_units: 9_007_199_254_740_991,
    maximum_typing_runs: 8,
};
//#endregion 🔖️Presence
