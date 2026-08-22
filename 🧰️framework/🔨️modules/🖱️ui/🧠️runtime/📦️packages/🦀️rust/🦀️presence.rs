//! @emoji 👥️ The `PresenceHub`: TTL-scoped, coalesced, never part of a document revision.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! Presence never enters a document revision. Hover and peer cursors change at input frequency, and
//! routing every one of them through `🧬️contract`'s revisioned [`ui_contract::UiPatch`] would make
//! every mouse move a patch — this hub is the separate channel that exists so it never has to. Own
//! presence ([`ui_contract::OwnPresence`]) is this session's own, authoritative and locally live, so it
//! carries no expiry — it stands until this session explicitly overwrites it. A [`ui_contract::PeerMark`]
//! is different: it describes what some OTHER session last told us, and that session can vanish without
//! ever sending a goodbye, so ITS entry carries a TTL. Without that TTL a disconnected peer would leave
//! a permanently stuck cursor on the surface — [`PresenceHub::expire`] is exactly what prevents that.
//!
//! Multiple [`PresenceHub::record_own`]/[`PresenceHub::record_peer`] calls to the same `(surface,
//! node_key)` between two [`PresenceHub::flush`] calls coalesce to one [`ui_contract::PresenceUpdate`]
//! carrying the LAST value written — a burst of pointer moves costs one update, not one per move.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use ui_contract::{OwnPresence, PeerMark, PresenceUpdate, SurfaceId};

//#region 🔖️Presence

/// 🔑️ The addressable slot one presence entry occupies — a render surface plus the node's stable
/// reconciliation key (never its [`ui_contract::UiNodeId`], which a reconciliation is free to reassign).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PresenceKey {
    surface: SurfaceId,
    node_key: String,
}

/// 👥️ One OTHER peer's currently-live mark plus the deadline at which [`PresenceHub::expire`] drops it.
struct PeerEntry {
    mark: PeerMark,
    expires_at_ms: u64,
}

/// 📇️ Everything live for one [`PresenceKey`] — own presence never expires; peers are keyed by
/// [`PeerMark::actor`] in a [`BTreeMap`] so [`PresenceHub::flush`] emits them in a deterministic order.
struct PresenceEntry {
    own: OwnPresence,
    peers: BTreeMap<String, PeerEntry>,
    ttl_ms: u32,
}

impl PresenceEntry {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn empty(ttl_ms: u32) -> Self {
        Self { own: OwnPresence::default(), peers: BTreeMap::new(), ttl_ms }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn is_empty(&self) -> bool {
        self.own == OwnPresence::default() && self.peers.is_empty()
    }
}

/// 👥️ The keyed `(SurfaceId, node_key) → entry` presence channel. Own presence is set by
/// [`Self::record_own`] and never expires; peer marks are set by [`Self::record_peer`] and age out via
/// [`Self::expire`]; [`Self::flush`] drains every key touched since the last flush into one coalesced
/// [`PresenceUpdate`] each.
#[derive(Default)]
pub struct PresenceHub {
    entries: HashMap<PresenceKey, PresenceEntry>,
    dirty: BTreeSet<PresenceKey>,
}

impl PresenceHub {
    /// 🏭️ An empty hub.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self::default()
    }

    /// 🙋️ Records this session's own hover/selection/preview/color state for `(surface, node_key)`,
    /// overwriting whatever was there. Own presence never expires by itself — it stands until the next
    /// `record_own` call for the same key.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn record_own(&mut self, surface: SurfaceId, node_key: impl Into<String>, own: OwnPresence, ttl_ms: u32) {
        let key = PresenceKey { surface, node_key: node_key.into() };
        let entry = self.entries.entry(key.clone()).or_insert_with(|| PresenceEntry::empty(ttl_ms));
        entry.own = own;
        entry.ttl_ms = ttl_ms;
        self.dirty.insert(key);
    }

    /// 👥️ Records or refreshes one OTHER peer's mark for `(surface, node_key)`, keyed by
    /// [`PeerMark::actor`] — a second call for the same actor overwrites the first and resets its
    /// expiry to `now_ms + ttl_ms`, which is how a peer's live cursor keeps renewing itself as long as
    /// that peer keeps sending updates.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn record_peer(&mut self, surface: SurfaceId, node_key: impl Into<String>, mark: PeerMark, ttl_ms: u32, now_ms: u64) {
        let key = PresenceKey { surface, node_key: node_key.into() };
        let entry = self.entries.entry(key.clone()).or_insert_with(|| PresenceEntry::empty(ttl_ms));
        entry.ttl_ms = ttl_ms;
        entry.peers.insert(mark.actor.clone(), PeerEntry { mark, expires_at_ms: now_ms.saturating_add(ttl_ms as u64) });
        self.dirty.insert(key);
    }

    /// ⏳️ Drops every peer mark whose expiry is at or before `now_ms` — exactly at a mark's own TTL
    /// boundary, it is gone. Own presence is never touched here; it has no expiry. A key whose peer set
    /// shrinks is marked dirty so the next [`Self::flush`] reports the loss.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn expire(&mut self, now_ms: u64) {
        for (key, entry) in self.entries.iter_mut() {
            let before = entry.peers.len();
            entry.peers.retain(|_, peer| peer.expires_at_ms > now_ms);
            if entry.peers.len() != before {
                self.dirty.insert(key.clone());
            }
        }
    }

    /// 📡️ Drains every key touched since the last flush into one coalesced [`PresenceUpdate`] each,
    /// reflecting the LAST write for that key — a burst of same-key `record_own`/`record_peer` calls
    /// between two flushes costs exactly one update. A key whose entry has gone fully empty (no own
    /// presence set, no peers left) is garbage-collected after being reported once, so a flush after
    /// every peer on a key has expired still reports the now-empty `peers` list before the slot is
    /// forgotten entirely.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn flush(&mut self) -> Vec<PresenceUpdate> {
        let dirty = std::mem::take(&mut self.dirty);
        let mut updates = Vec::with_capacity(dirty.len());
        for key in dirty {
            let Some(entry) = self.entries.get(&key) else { continue };
            let peers: Vec<PeerMark> = entry.peers.values().map(|peer| peer.mark.clone()).collect();
            updates.push(PresenceUpdate { surface: key.surface.clone(), node_key: key.node_key.clone(), own: entry.own, peers, ttl_ms: entry.ttl_ms });
            if entry.is_empty() {
                self.entries.remove(&key);
            }
        }
        updates
    }
}
//#endregion 🔖️Presence

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn mark(actor: &str) -> PeerMark {
        PeerMark { actor: actor.into(), color: Some(1), hovered: true, selected: false, label: actor.to_uppercase() }
    }

    #[test]
    fn presence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them() {
        let mut hub = PresenceHub::new();
        let surface = SurfaceId::from("note.play.navigator");
        hub.record_peer(surface, "row-1", mark("a"), 1_000, 0);

        let first = hub.flush();
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].peers, vec![mark("a")]);

        hub.expire(999);
        assert!(hub.flush().is_empty(), "not yet at the TTL boundary, nothing became dirty");

        hub.expire(1_000);
        let after_expiry = hub.flush();
        assert_eq!(after_expiry.len(), 1, "expiry at the exact TTL boundary must surface as one more update");
        assert!(after_expiry[0].peers.is_empty(), "the flush after expiry must omit the expired peer");

        assert!(hub.flush().is_empty(), "the now-empty slot was garbage-collected, nothing left to report");
    }

    #[test]
    fn a_burst_of_same_key_peer_writes_coalesces_to_one_update() {
        let mut hub = PresenceHub::new();
        let surface = SurfaceId::from("note.play.navigator");
        hub.record_peer(surface.clone(), "row-1", PeerMark { hovered: true, ..mark("a") }, 4_000, 0);
        hub.record_peer(surface.clone(), "row-1", PeerMark { hovered: true, selected: true, ..mark("a") }, 4_000, 10);
        hub.record_peer(surface, "row-1", PeerMark { hovered: false, selected: true, ..mark("a") }, 4_000, 20);

        let updates = hub.flush();
        assert_eq!(updates.len(), 1, "a burst of pointer moves on one key must cost exactly one update");
        assert_eq!(updates[0].peers, vec![PeerMark { hovered: false, selected: true, ..mark("a") }]);
    }

    #[test]
    fn a_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value() {
        let mut hub = PresenceHub::new();
        let surface = SurfaceId::from("note.play.navigator");
        hub.record_own(surface.clone(), "row-1", OwnPresence { hovered: true, ..Default::default() }, 1_000);
        hub.record_own(surface.clone(), "row-1", OwnPresence { hovered: true, selected: true, ..Default::default() }, 1_000);
        let newest = OwnPresence { hovered: false, selected: true, previewed: true, color: Some(2) };
        hub.record_own(surface, "row-1", newest, 1_000);

        let updates = hub.flush();
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].own, newest);
    }

    #[test]
    fn own_presence_never_expires() {
        let mut hub = PresenceHub::new();
        let surface = SurfaceId::from("s");
        hub.record_own(surface, "row-1", OwnPresence { hovered: true, ..Default::default() }, 1_000);
        hub.flush();

        hub.expire(1_000_000);
        assert!(hub.flush().is_empty(), "expire() must never touch own presence, so nothing became dirty");
    }

    #[test]
    fn distinct_peers_on_one_key_are_all_reported_and_expire_independently() {
        let mut hub = PresenceHub::new();
        let surface = SurfaceId::from("s");
        hub.record_peer(surface.clone(), "row-1", mark("a"), 1_000, 0);
        hub.record_peer(surface, "row-1", mark("b"), 5_000, 0);
        let first = hub.flush();
        assert_eq!(first[0].peers.len(), 2);

        hub.expire(1_000);
        let after = hub.flush();
        assert_eq!(after[0].peers, vec![mark("b")], "only the expired peer drops off");
    }
}
//#endregion 🧪️Tests
