//! 🧺️ The instance components of `instance-stores`: ConstructorState, InstanceStores, PeerRoster, ActionIngress, LocalInteraction (WP6, plan section 7.3).
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, ArtifactFixedRegistry};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, ConfigStore, MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase, OriginalCatalogConstructor};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use semio_framework_plugin_decl_views::{InteractionHoverState, PeerPresenceRoot};
use std::mem::ManuallyDrop;
use protocol::OpText;
use semio_framework_plugin_peer::{PeerPresenceRootRetirement, PeerRosterPublication, PresenceRosterOutcome};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_plugin_child::SnapshotReadReturnPump;
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_framework_ui_contract::*;

/// 🪣️ Store-constructor state: the receipts, units and lane of the four original catalogue constructors.
pub struct ConstructorState<A: ArtifactApp> {
    pub constructor_receipts: std::mem::MaybeUninit<[Option<(ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)>; 24]>,
    pub constructor_recipient_admitted: bool,
    pub constructor_recipient_retired: bool,
    pub constructor_terminal_receipt: Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)>,
    pub constructor_document: OriginalCatalogConstructor<A::Snapshot, A::Mutation>,
    pub constructor_config: OriginalCatalogConstructor<A::Config, A::ConfigMutation>,
    pub constructor_draft: OriginalCatalogConstructor<A::Draft, A::DraftMutation>,
    pub constructor_interaction: OriginalCatalogConstructor<protocol::InteractionState, InteractionConfigMutation>,
    pub constructor_lane: usize,
}

impl<A: ArtifactApp> ConstructorState<A> {
    /// 🧼️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            constructor_receipts: std::mem::MaybeUninit::uninit(),
            constructor_recipient_admitted: false,
            constructor_recipient_retired: false,
            constructor_terminal_receipt: None,
            constructor_document: OriginalCatalogConstructor::new(),
            constructor_config: OriginalCatalogConstructor::new(),
            constructor_draft: OriginalCatalogConstructor::new(),
            constructor_interaction: OriginalCatalogConstructor::new(),
            constructor_lane: 0,
        }
    }
}

/// 🪥️ The stores of the instance: document, config, draft, presence, transient, interaction and window owner registries with their one-item publication factories.
pub struct InstanceStores<A: ArtifactApp> {
    pub store: ArtifactStore<A::Snapshot, A::Mutation>,
    pub config_store: ConfigStore<A::Config, A::ConfigMutation>,
    /// 📝️ Volatile draft lane — never checkpoints; prune via `ArtifactCommand::PruneDrafts`.
    /// Moves to host `ArtifactSession` when CHANNEL_VERSION 5 exchange lands.
    pub draft_store: store::DraftStore<A::Draft, A::DraftMutation>,
    /// 👥️ Ephemeral SHARED lane — a last-writer-wins peer roster, not an event log. Holds
    /// this actor's own presence plus whatever peers last broadcast; never persisted, never
    /// checkpointed, never undoable.
    pub presence_store: store::PresenceStore<A::Presence, A::PresenceMutation>,
    /// 🫧️ Ephemeral LOCAL-ONLY lane — typed UI state that never leaves this client and
    /// never becomes document content. The typed home for what used to live in plugin
    /// `thread_local!`s.
    pub transient_store: store::TransientStore<A::Transient, A::TransientMutation>,
    pub window_config_store: WindowConfigOwnerRegistry,
    pub window_transient_store: WindowTransientOwnerRegistry,
    pub retired_window_transient_stores: ArtifactFixedRegistry<WindowTransientOwnerRegistry>,
    pub artifact_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>>,
    pub config_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Config, A::ConfigMutation>>>,
    pub draft_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Draft, A::DraftMutation>>>,
    pub presence_one_item_factory: Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Presence, A::PresenceMutation>>>,
    pub transient_one_item_factory: Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Transient, A::TransientMutation>>>,
    pub presence_local_root_retirement_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>,
    pub transient_local_root_retirement_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Transient>>>,
    pub unsupported_publication_contracts: BTreeMap<String, &'static str>,
    /// 🕹️ The framework-owned, PERSISTED-LOCAL half of the interaction mechanism: one domain's
    /// current selection + active mode/granularity, written ONLY by the six intercepted interaction
    /// verbs (`dispatch_interaction_action`) through `ApplyInLane{lane: HistoryLane::Interaction}` —
    /// single writer, so it never drifts from what the presence broadcast reports. Persists/reloads
    /// like `config_store`; the general "undo"/"redo" actions above only ever dispatch against
    /// `self.store` (the DOCUMENT store), never this one, so a pick is never undoable by construction
    /// — the `HistoryLane` tag on its own entries additionally lets a lane-aware caller walk PAST them
    /// (`ArtifactCommand::UndoInLane`/`RedoInLane`) without needing a second store at all. A real
    /// `store::ConfigStore<InteractionState, _>` instance (not folded into `config_store: ConfigStore
    /// <A::Config, A::ConfigMutation>`, which is per-app-config-typed): keeps this zero-touch for
    /// every app's own `Config`/`ConfigMutation` types, since ~17 crates never need to know this
    /// exists — see `InteractionConfigMutation`'s own doc comment for the orphan-rule reason its
    /// `Mutation` impl couldn't target `A::Config` directly either way.
    pub interaction_store: ConfigStore<protocol::InteractionState, InteractionConfigMutation>,
}

impl<A: ArtifactApp> InstanceStores<A> {
    /// 🪒️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new(store: ArtifactStore<A::Snapshot, A::Mutation>, config_store: ConfigStore<A::Config, A::ConfigMutation>, draft_store: store::DraftStore<A::Draft, A::DraftMutation>, presence_store: store::PresenceStore<A::Presence, A::PresenceMutation>, window_config_store: WindowConfigOwnerRegistry, window_transient_store: WindowTransientOwnerRegistry, artifact_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>>, config_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Config, A::ConfigMutation>>>, draft_one_item_factory: Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Draft, A::DraftMutation>>>, presence_one_item_factory: Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Presence, A::PresenceMutation>>>, transient_one_item_factory: Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Transient, A::TransientMutation>>>, presence_local_root_retirement_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>, transient_local_root_retirement_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Transient>>>, unsupported_publication_contracts: BTreeMap<String, &'static str>, interaction_store: ConfigStore<protocol::InteractionState, InteractionConfigMutation>) -> Self {
        Self {
            store,
            config_store,
            draft_store,
            presence_store,
            transient_store: store::TransientStore::new(A::Transient::default()),
            window_config_store,
            window_transient_store,
            retired_window_transient_stores: ArtifactFixedRegistry::new(),
            artifact_one_item_factory,
            config_one_item_factory,
            draft_one_item_factory,
            presence_one_item_factory,
            transient_one_item_factory,
            presence_local_root_retirement_factory,
            transient_local_root_retirement_factory,
            unsupported_publication_contracts,
            interaction_store,
        }
    }
}

/// 🧻️ The peer roster: this actor's colour, the adopted peer presence and the roster publication admission state.
pub struct PeerRoster<A: ArtifactApp> {
    /// 🎨️ This actor's hub-assigned palette index, adopted from the last
    /// `AppCommand::Presence` (contract-freeze §C7.6) — `None` for a folder-only session with no
    /// hub, or before the first presence push.
    pub own_color: Option<u8>,
    /// 👥️ Every OTHER peer's last-adopted color/surface/interaction, keyed by actor —
    /// the ARTIFACT+APP-scope roster `InteractionView::peers_selecting`/`peers_hovering` and the
    /// UI-tree stamping pass read from; upserted/pruned atomically with `presence_store`'s own
    /// peer map by `adopt_presence`.
    pub peer_presence: ManuallyDrop<std::sync::Arc<PeerPresenceRoot>>,
    pub peer_presence_generation: u64,
    pub peer_roster_admission_generation: u64,
    pub peer_roster_processed_generation: u64,
    pub peer_roster_outcome_generation: u64,
    pub peer_roster_reservations: [Option<(u64, u64)>; ARTIFACT_LIVE_OUTPUT_SLOTS],
    pub peer_roster_scope: semio_framework_job::CancelToken,
    pub peer_roster_publications: ArtifactFixedRegistry<PeerRosterPublication<A>>,
    pub peer_roster_outcomes: ArtifactFixedRegistry<PresenceRosterOutcome>,
    pub peer_presence_retirements: ArtifactFixedRegistry<PeerPresenceRootRetirement>,
    pub presence_peer_retirements: ArtifactFixedRegistry<store::os_store::presence::PresencePeersRetirement<A::Presence>>,
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): the render-plane presence outbox —
    /// `stamp_and_cache_interaction_ui` pushes one `PresenceUpdate` per `(surface, node_key)` whose
    /// own/peer selection or hover it derived off `protocol::InteractionState`/`peer_presence` this
    /// render; `plugin_runtime::plugin_render_surface` drains it within the same surface render. NEVER a document
    /// revision — this lane has no op log, no undo group, no command-log row, mirroring
    /// `presence_store`/`transient_store`'s own ephemeral treatment above.
    pub pending_presence: Vec<PresenceUpdate>,
    /// 🧹️ Node keys this body's LAST render marked, per body key. `derive_node_presence` skips an
    /// idle node entirely, and `PresenceHub::record_own` keeps own presence until it is explicitly
    /// overwritten — so without this ledger a node that loses the selection is never written back to
    /// idle and every renderer keeps painting it selected for the life of the session. Each render
    /// diffs the keys it marked against this set and pushes one cleared `PresenceUpdate` per key it
    /// dropped, which is the ONLY thing that retires a selection on the render plane.
    pub presence_marked_keys: BTreeMap<String, BTreeSet<String>>,
}

impl<A: ArtifactApp> PeerRoster<A> {
    /// 🪤️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            own_color: None,
            peer_presence: ManuallyDrop::new(PeerPresenceRoot::empty_shared()),
            peer_presence_generation: 0,
            peer_roster_admission_generation: 0,
            peer_roster_processed_generation: 0,
            peer_roster_outcome_generation: 0,
            peer_roster_reservations: std::array::from_fn(|_| None),
            peer_roster_scope: semio_framework_job::CancelToken::root_now(),
            peer_roster_publications: ArtifactFixedRegistry::new(),
            peer_roster_outcomes: ArtifactFixedRegistry::new(),
            peer_presence_retirements: ArtifactFixedRegistry::new(),
            presence_peer_retirements: ArtifactFixedRegistry::new(),
            pending_presence: Vec::new(),
            presence_marked_keys: BTreeMap::new(),
        }
    }

    pub fn admit_presence_ingress(&mut self, admission: semio_framework_plugin_peer::PresenceRosterAdmission, command: protocol::PresenceCommandCursor, now_ms: i64) {
        let slot = crate::stores_components::InstanceStores::<A>::peer_roster_slot(admission.generation);
        assert_eq!(command.seq(), admission.seq, "Presence cursor sequence is bound by its unforgeable reservation");
        assert_eq!(self.peer_roster_reservations[slot], Some((admission.generation, admission.seq)), "Presence reservation remains live under the app's exclusive owner");
        assert!(self.peer_roster_publications.can_insert(admission.generation), "Presence publication slot was reserved before cursor construction");
        let generation = admission.generation;
        self.peer_roster_publications.insert_admitted(generation, semio_framework_plugin_peer::PeerRosterPublication::<A>::new(admission, now_ms, command));
    }

    pub fn reject_presence_ingress(&mut self, admission: semio_framework_plugin_peer::PresenceRosterAdmission, command: semio_framework::kernel::FixedCommandPage, fault: semio_framework::Fault) {
        let slot = crate::stores_components::InstanceStores::<A>::peer_roster_slot(admission.generation);
        assert_eq!(self.peer_roster_reservations[slot], Some((admission.generation, admission.seq)), "malformed Presence owner retains its exact reservation");
        assert!(self.peer_roster_publications.can_insert(admission.generation), "malformed Presence cleanup uses its pre-reserved publication slot");
        let generation = admission.generation;
        self.peer_roster_publications.insert_admitted(generation, semio_framework_plugin_peer::PeerRosterPublication::<A>::rejected(admission, command, fault));
    }
}

/// 🥁️ Action ingress: the parser slots that retain unused original arguments and faults until their paid close.
pub struct ActionIngress {
    pub action_ingress_sources: ArtifactFixedRegistry<crate::action_ingress::ActionIngressRecord>,
    pub pending_action_ingress: ManuallyDrop<Option<crate::action_ingress::ActionIngressSource>>,
    pub pending_action_ingress_caller: std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub pending_action_ingress_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub pending_action_ingress_retirement: ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<Option<crate::action_ingress::ActionIngressSource>>>>,
}

impl ActionIngress {
    /// 🪘️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            action_ingress_sources: ArtifactFixedRegistry::unallocated(16),
            pending_action_ingress: ManuallyDrop::new(None),
            pending_action_ingress_caller: std::mem::ManuallyDrop::new(None),
            pending_action_ingress_identity: None,
            pending_action_ingress_retirement: ManuallyDrop::new(None),
        }
    }
}

/// 🎷️ Local interaction: the framework-owned live query, topology authority, hover and snapshot read returns.
pub struct LocalInteraction<A: ArtifactApp> {
    pub local_interaction_query: Option<semio_framework_plugin_interaction::live::LocalInteractionLiveQuery<A::Snapshot, A::Config>>,
    pub local_interaction_query_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub local_interaction_authority: semio_framework_plugin_interaction::authority::LocalInteractionTopologyAuthority,
    pub snapshot_read_return_stage: u8,
    pub local_interaction_query_turn: bool,
    pub local_interaction_emitted_bytes: u128,
    pub local_interaction_retired_bytes: u128,
    pub config_snapshot_read_returns: SnapshotReadReturnPump,
    pub interaction_snapshot_read_returns: SnapshotReadReturnPump,
    /// 🐁️ The EPHEMERAL-local half — see `InteractionHoverState`'s own doc comment. Never persisted,
    /// never undoable, mirrored into the ephemeral-shared `PresenceInteraction` broadcast
    /// (`interaction_state`) exactly like `presence_store`'s own local half.
    pub interaction_hover: InteractionHoverState,
    /// 🕹️ Wave B9: the just-computed selection `dispatch_interaction_action` published on leftover,
    /// retained for the guest's own render. `protocol::validate_state` prunes every id an empty
    /// topology does not contain, so the store snapshot a first pick reads can be empty while this
    /// overlay still names the picked id — Inspection reads this when the store does not.
    pub interaction_leftover_selection: Option<protocol::InteractionState>,
    pub interaction_leftover_ids: Vec<String>,
    /// 🌳️ One `DomainTopology` per `HierarchyProvider::UiTree` domain, derived from the LAST
    /// rendered `UiNode::Tree` carrying that `interaction_domain` (see `render`'s post-processing
    /// pass) — `resolve_domain_topology`'s cache for domains the app itself never supplies a
    /// topology for. Empty (safely degrading to Flat-like single-target behavior) until the first
    /// render of that domain's tree.
    pub interaction_ui_topology: HashMap<String, protocol::DomainTopology>,
    /// 🗺️ `A::interaction_topology`'s whole answer for one document generation.
    ///
    /// The app builds EVERY declared domain in one pass off a fresh decode of the document, and one
    /// pick asked for it at least twice — once in [`Self::dispatch_interaction_action`]'s own verb arm
    /// and once more inside [`Self::revalidate_and_persist_interaction_state`]'s
    /// `build_full_interaction_topology`, with nothing between them that can move the document. On the
    /// 180-object puzzle3d flagship that is two full fixture decodes plus ~900 `TopologyNode`
    /// allocations per pick and is the reason `interactionSelect` cost 20× there
    /// (26/09/02/PUZZLE-3D-END-TO-END wave B44 §2.2). Keyed on `(store.generation(),
    /// config_store.generation())` — the pair every input of `A::interaction_topology` is derived
    /// from, `live_render_operation` included — so a mutation invalidates it in the same turn it lands.
    pub interaction_topology_memo: Option<((u64, u64), protocol::InteractionTopology)>,
}

impl<A: ArtifactApp> LocalInteraction<A> {
    /// 🎺️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new() -> Self {
        Self {
            local_interaction_query: None,
            local_interaction_query_receipt: None,
            local_interaction_authority: Default::default(),
            snapshot_read_return_stage: 0,
            local_interaction_query_turn: false,
            local_interaction_emitted_bytes: 0,
            local_interaction_retired_bytes: 0,
            config_snapshot_read_returns: SnapshotReadReturnPump::new(),
            interaction_snapshot_read_returns: SnapshotReadReturnPump::new(),
            interaction_hover: InteractionHoverState::new(),
            interaction_leftover_selection: None,
            interaction_leftover_ids: Vec::new(),
            interaction_ui_topology: HashMap::new(),
            interaction_topology_memo: None,
        }
    }
}
