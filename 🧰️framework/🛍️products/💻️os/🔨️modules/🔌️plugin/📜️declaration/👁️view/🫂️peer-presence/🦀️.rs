//! 🫂️ The ephemeral interaction hover map, the last-adopted presence slice of each peer, its bounded root, the mark a peer leaves on a target and the cancellation action. See [`super`].

use super::*;
use semio_framework_plugin_operation::CANCEL_TYPED_OPERATION_ACTION_ID;

/// 🐁️ The ephemeral, never-persisted half of the interaction mechanism — `VcsArtifactApp`'s hover
/// map, one `DomainHover` per domain (mirrors `protocol::InteractionState.hover`'s own shape; see
/// that field's doc for why one slot per domain, not per channel). Deliberately a plain type alias,
/// not a generic `TransientStore<P, M>` instance: hover here is applied directly by the interaction
/// dispatch interception, never through an app's own `Emit`/ephemeral lane, so the `Mutation`-trait
/// machinery `TransientStore` requires would be unused ceremony.
pub type InteractionHoverState = BTreeMap<String, protocol::DomainHover>;

/// 👥️ One OTHER peer's last-adopted artifact/app-scope presence slice — the plugin-side
/// twin of `PresencePeer`'s `color`/`surface`/`interaction` fields, keyed by actor in
/// `VcsArtifactApp::peer_presence` (contract-freeze §C7.6). `presence_pack`'s decoded app-typed
/// value is NOT carried here — it lands in `presence_store` instead (the existing peer-roster
/// mechanism, unchanged), so this struct stays generic over every `ArtifactApp`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct PeerPresence {
    pub color: Option<u8>,
    pub surface: Option<String>,
    pub interaction: Option<protocol::PresenceInteraction>,
}

pub const PEER_PRESENCE_SLOTS: usize = 64;

const PEER_PRESENCE_ID_BYTES: usize = 256;

#[derive(semio_framework_value::RetireOwned)]
pub struct PeerPresenceEntry {
    pub(super) actor: String,
    pub(super) presence: PeerPresence,
}

pub struct PeerPresenceRoot {
    pub entries: [Option<std::sync::Arc<PeerPresenceEntry>>; PEER_PRESENCE_SLOTS],
    pub len: usize,
}

impl PeerPresenceRoot {
    pub fn empty() -> Self {
        Self { entries: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn empty_shared() -> std::sync::Arc<Self> {
        static EMPTY: std::sync::OnceLock<std::sync::Arc<PeerPresenceRoot>> = std::sync::OnceLock::new();
        EMPTY.get_or_init(|| std::sync::Arc::new(Self::empty())).clone()
    }

    pub fn insert(&mut self, actor: String, presence: PeerPresence) -> Result<(), Fault> {
        if actor.is_empty() || actor.len() > PEER_PRESENCE_ID_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-presence-actor"), "peer actor identity is empty or exceeds its fixed byte authority"));
        }
        let mut index = 0usize;
        while index < self.len && self.entries[index].as_ref().is_some_and(|entry| entry.actor.as_str() < actor.as_str()) {
            index += 1;
        }
        if self.entries.get(index).and_then(Option::as_ref).is_some_and(|entry| entry.actor == actor) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-presence-duplicate"), "one presence roster contains a duplicate actor identity"));
        }
        if self.len == PEER_PRESENCE_SLOTS {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-presence-capacity"), "presence roster exceeds its fixed peer authority"));
        }
        for cursor in (index..self.len).rev() {
            self.entries[cursor + 1] = self.entries[cursor].take();
        }
        self.entries[index] = Some(std::sync::Arc::new(PeerPresenceEntry { actor, presence }));
        self.len += 1;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &PeerPresence)> {
        self.entries[..self.len].iter().filter_map(|entry| entry.as_deref().map(|entry| (entry.actor.as_str(), &entry.presence)))
    }

    pub(super) fn get(&self, actor: &str) -> Option<&PeerPresence> {
        self.iter().find_map(|(candidate, presence)| (candidate == actor).then_some(presence))
    }

    pub fn contains_key(&self, actor: &str) -> bool {
        self.get(actor).is_some()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 👥️ One peer's mark on a single interaction target — `InteractionView::peers_selecting`/
/// `peers_hovering`'s element type (contract-freeze §C7.6), sorted by actor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PeerMark<'a> {
    pub actor: &'a str,
    pub color: Option<u8>,
}

pub fn cancellation_action_definition() -> ActionDefinition {
    ActionDefinition::resumable_framework(CANCEL_TYPED_OPERATION_ACTION_ID, LocalizedLabel::native("Cancel Operation", "Vorgang abbrechen"), ActionKind::View, "square").with_in_palette(false)
}
