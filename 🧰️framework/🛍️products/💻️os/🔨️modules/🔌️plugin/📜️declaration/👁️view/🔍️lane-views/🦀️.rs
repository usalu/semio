//! 🔍️ The read views of the config, draft, presence and interaction lanes. See [`super`].

use super::*;

/// 🧮️ Read-only view of an app's config snapshot — same role as {@link ArtifactView} for the
/// config {@link ConfigStore} owned by {@link VcsArtifactApp}.
pub struct ConfigView<'a, C> {
    pub snapshot: &'a C,
    pub window: Option<&'a WindowConfigSnapshot>,
}

impl<'a, C> ConfigView<'a, C> {
    pub fn window<O: WindowConfigOwner>(&self) -> Option<&O::State> {
        self.window.and_then(WindowConfigSnapshot::get::<O>)
    }
}

/// 📝️ Read-only view of an app's volatile draft snapshot — same role as {@link ConfigView}
/// for the draft {@link store::DraftStore} (ephemeral; never checkpoints).
pub struct DraftView<'a, D> {
    pub snapshot: &'a D,
}

/// 👥️ Read-only view of the PRESENCE lane: this actor's own live shared state plus every
/// peer's, as last broadcast. Ephemeral and shared — never persisted, never undoable.
pub struct PresencePeersView<'a, P> {
    pub root: &'a store::os_store::presence::PresencePeersRoot<P>,
}

impl<'a, P> PresencePeersView<'a, P> {
    /// 👥️ Borrows the canonical immutable roster without granting mutable peer authority.
    pub fn new(root: &'a store::os_store::presence::PresencePeersRoot<P>) -> Self { Self { root } }
    /// 👥️ Iterates the already-sorted immutable peer root without projecting a roster.
    pub fn iter(&self) -> impl Iterator<Item = (&'a str, &'a P)> {
        self.root.peers()
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }
}

pub struct PresenceView<'a, P> {
    /// 👤️ This actor's own presence — the value that gets broadcast.
    pub local: &'a P,
    /// 👥️ Every other peer's presence, sorted by actor id for a stable render order.
    pub peers: PresencePeersView<'a, P>,
}

/// 🕹️ Read-only view of the framework-owned INTERACTION mechanism (hover + selection + active
/// mode/granularity per domain) — threaded into `ArtifactApp::handle`/`copy_fragment`/
/// `cut_operations` so an app reads its own current selection/hover instead of ever storing it
/// again itself. `state` is the persisted-local half (selection/mode/granularity, one
/// `VcsArtifactApp::interaction_store` `ApplyInLane{lane: HistoryLane::Interaction}` dispatch away
/// from every change); `hover` is the ephemeral-local half (`VcsArtifactApp::interaction_hover`,
/// never persisted — see that field's own doc comment). Both are combined once per dispatch by
/// `VcsArtifactApp::interaction_view`, never mutated through this read-only handle.
pub struct InteractionView<'a> {
    pub state: &'a protocol::InteractionState,
    pub hover: &'a InteractionHoverState,
    /// 👥️ Every OTHER peer's last-adopted presence, keyed by actor — see
    /// `VcsArtifactApp::peer_presence`'s own doc comment. Sourced from `BTreeMap` iteration
    /// order, which is already sorted by actor.
    pub peers: &'a PeerPresenceRoot,
    /// ⏪️ What the open time-travel draft references, per selection domain
    /// ([`crate::time_travel::TimeTravelLedger::draft_references`]) — empty while no draft editor is open.
    pub draft_references: &'a BTreeMap<String, Vec<String>>,
}

impl<'a> InteractionView<'a> {
    /// 👁️ Borrows already admitted immutable interaction owners for one dispatch.
    pub fn new(state: &'a protocol::InteractionState, hover: &'a InteractionHoverState, peers: &'a PeerPresenceRoot, draft_references: &'a BTreeMap<String, Vec<String>>) -> Self { Self { state, hover, peers, draft_references } }
    /// 🔗️ `domain`'s ids the open time-travel draft references — every id its `Reference` inputs of that domain
    /// hold, so a render highlights what the edited mutation acts on; empty while no draft editor is open.
    pub fn draft_references(&self, domain: &str) -> &[String] {
        self.draft_references.get(domain).map_or(&[], Vec::as_slice)
    }

    /// 🖱️ `domain`'s current selection — an empty `DomainSelection` (not `None`) for an undeclared
    /// or never-selected domain, so call sites never have to unwrap.
    pub fn selection(&self, domain: &str) -> &protocol::DomainSelection {
        self.state.selection.get(domain).unwrap_or_else(|| empty_domain_selection())
    }

    /// 🕹️ Leftover.ids for Inspection: every domain's selected ids as a SET (see
    /// {@link leftover_selected_ids_of}). A pick can land on leftover `selectedIds` while
    /// `selection(vortex)` is still the empty persist snapshot.
    pub fn leftover_selected_ids(&self) -> Vec<String> {
        leftover_selected_ids_of(self.state)
    }

    /// 🐁️ `domain`'s current hover on `channel` — empty when nothing is hovered on that channel
    /// right now. `InteractionState.hover` holds exactly one live channel per domain at a time (the
    /// most recently hovered — see `next_hover`'s doc), so a `channel` mismatch also reads empty
    /// rather than returning a stale different-channel hover.
    pub fn hover(&self, domain: &str, channel: &str) -> &protocol::DomainHover {
        match self.hover.get(domain) {
            Some(hover) if hover.channel == channel => hover,
            _ => empty_domain_hover(),
        }
    }

    /// 🪜️ `domain`'s active granularity id, `None` for an undeclared domain (never dispatched
    /// through `validate_state` yet — e.g. a registry-less test construction).
    pub fn active_granularity(&self, domain: &str) -> Option<&str> {
        self.state.active_granularity.get(domain).map(String::as_str)
    }

    /// 🔀️ `domain`'s active `SelectionMode`, `None` for an undeclared domain.
    pub fn active_mode(&self, domain: &str) -> Option<protocol::SelectionMode> {
        self.state.active_mode.get(domain).copied()
    }

    /// 👥️ Every OTHER peer currently selecting `id` within `domain`, sorted by actor
    /// (contract-freeze §C7.6) — `[]` for a peer with no adopted interaction, an interaction with
    /// no matching domain, or a domain where `id` is not in `selected`.
    /// 🚫️async: E1 pure in-memory filter, no suspension point — see R9.
    ///
    /// ⚠️ Gap (SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY packet `sdk-helpers` — see
    /// `📓️terra-sdk-helpers-report.md`): the sync `marks_for` closure this used to feed
    /// (`stamp_and_cache_interaction_ui`'s old presence-stamping half) is gone — that destination no
    /// longer exists on a built node. This method itself is unaffected and still tested directly; a
    /// future packet publishing real `PresenceUpdate`s is the next intended caller.
    pub fn peers_selecting(&self, domain: &str, id: &str) -> Vec<PeerMark<'a>> {
        self.peers
            .iter()
            .filter_map(|(actor, presence)| {
                let interaction = presence.interaction.as_ref()?;
                let matched = interaction.domains.iter().find(|candidate| candidate.domain == domain)?;
                matched.selected.iter().any(|selected_id| selected_id == id).then_some(PeerMark { actor, color: presence.color })
            })
            .collect()
    }

    /// 👥️ `peers_selecting`'s hover twin — every OTHER peer currently hovering `id` within
    /// `domain`, sorted by actor.
    /// 🚫️async: E1 pure in-memory filter — see `peers_selecting`'s doc for why this must be sync.
    pub fn peers_hovering(&self, domain: &str, id: &str) -> Vec<PeerMark<'a>> {
        self.peers
            .iter()
            .filter_map(|(actor, presence)| {
                let interaction = presence.interaction.as_ref()?;
                let matched = interaction.domains.iter().find(|candidate| candidate.domain == domain)?;
                matched.hovered.iter().any(|hovered_id| hovered_id == id).then_some(PeerMark { actor, color: presence.color })
            })
            .collect()
    }
}
