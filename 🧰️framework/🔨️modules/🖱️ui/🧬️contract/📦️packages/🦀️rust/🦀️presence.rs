//! @emoji 👥️ The presence channel: `PresenceUpdate`/`PeerMark`, TTL-scoped and coalesced.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! [`Activity`] is the one presence-shaped field that DOES live on [`crate::UiNodeRecord`] — a node
//! genuinely *is* loading, waiting, or finished, and that is document state every subscriber must
//! receive with the node. Everything else here (hover, selection, peer cursors, own color) travels on
//! this separate, coalesced, TTL-scoped channel and never touches the document: those signals change
//! at input frequency, and forcing a document revision for every mouse-move would defeat the whole
//! point of a revisioned patch protocol. A peer whose session drops does not need an explicit
//! "goodbye" message either — its marks simply age out once `ttl_ms` elapses without a refresh.

use serde::{Deserialize, Serialize};

//#region 🔖️Presence

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_false(value: &bool) -> bool {
    !*value
}

/// 🧭️ The activity lifecycle of a node, orthogonal to `disabled`/`transition` — was `UiStatus` on the
/// old wgpu target's `UiPresence`. Lives on the document (`crate::UiNodeRecord::activity`) because it
/// is genuinely part of what the node IS this revision, not an ephemeral input-frequency signal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Activity {
    Waiting,
    Loading,
    #[default]
    Idle,
    Finished,
}

/// 👥️ One OTHER peer's mark on a node — hover/selection dot plus initials chip. Ported faithfully
/// from the old wgpu target's `UiPeerMark` (contract-freeze §C7.6, ticket 26/08/17/SHARED-PRESENCE-
/// SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION) — `label` is still the actor id's display form, not
/// a free-text caption.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerMark {
    pub actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub hovered: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub selected: bool,
    pub label: String,
}

/// 🙋️ This session's own hover/selection/preview state and palette color on a node — the local half
/// of the presence channel; every OTHER session's equivalent arrives as a [`PeerMark`] in `peers`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnPresence {
    #[serde(default, skip_serializing_if = "is_false")]
    pub hovered: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub selected: bool,
    /// 👁️ Mid-drag or mid-hover-preview emphasis distinct from `hovered` — e.g. previewing a drop
    /// target before release, or a `Trigger::HoverPreview` binding's target while armed.
    #[serde(default, skip_serializing_if = "is_false")]
    pub previewed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
}

/// 📡️ One coalesced, TTL-scoped update on the presence channel, keyed `(surface, node_key)` — never
/// carried by [`crate::UiSnapshot`]/[`crate::UiPatch`] themselves. A receiver clears a peer's mark once
/// `ttl_ms` has elapsed without a fresh `PresenceUpdate` for that key, so a disconnected peer fades out
/// on a timer instead of leaving a stuck mark. Replaces the old `ui_tree_stamp_presence`, which
/// mutated hover/selection/color/peers directly onto tree nodes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceUpdate {
    pub surface: crate::SurfaceId,
    /// 🔑️ [`crate::UiNodeRecord::key`], not [`crate::UiNodeId`] — presence must still land on the
    /// right element across a reconciliation that reassigns ids but keeps keys stable.
    pub node_key: String,
    pub own: OwnPresence,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peers: Vec<PeerMark>,
    pub ttl_ms: u32,
}
//#endregion 🔖️Presence

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_defaults_to_idle_and_round_trips() {
        assert_eq!(Activity::default(), Activity::Idle);
        for activity in [Activity::Waiting, Activity::Loading, Activity::Idle, Activity::Finished] {
            let json = serde_json::to_string(&activity).expect("serialize");
            let back: Activity = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(activity, back);
        }
    }

    #[test]
    fn peer_mark_round_trips_and_omits_false_flags() {
        let mark = PeerMark { actor: "actor-1".into(), color: Some(3), hovered: true, selected: false, label: "AS".into() };
        let json = serde_json::to_value(&mark).expect("serialize");
        assert_eq!(json.get("hovered").and_then(|v| v.as_bool()), Some(true));
        assert!(json.get("selected").is_none());
        let back: PeerMark = serde_json::from_value(json).expect("deserialize");
        assert_eq!(mark, back);
    }

    #[test]
    fn own_presence_default_serializes_to_empty_object() {
        let json = serde_json::to_value(OwnPresence::default()).expect("serialize");
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn presence_update_round_trips_with_peers() {
        let update = PresenceUpdate {
            surface: crate::SurfaceId::from("note.play.navigator"),
            node_key: "row-9".into(),
            own: OwnPresence { hovered: true, selected: true, previewed: false, color: Some(2) },
            peers: vec![PeerMark { actor: "a".into(), color: Some(1), hovered: true, selected: false, label: "A".into() }, PeerMark { actor: "b".into(), color: None, hovered: false, selected: true, label: "B".into() }],
            ttl_ms: 4_000,
        };
        let first = serde_json::to_string(&update).expect("serialize");
        let deserialized: PresenceUpdate = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(update, deserialized);
    }

    #[test]
    fn presence_update_omits_empty_peers() {
        let update = PresenceUpdate { surface: crate::SurfaceId::from("s"), node_key: "k".into(), own: OwnPresence::default(), peers: Vec::new(), ttl_ms: 1_000 };
        let json = serde_json::to_value(&update).expect("serialize");
        assert!(json.get("peers").is_none());
    }
}
//#endregion 🧪️Tests
