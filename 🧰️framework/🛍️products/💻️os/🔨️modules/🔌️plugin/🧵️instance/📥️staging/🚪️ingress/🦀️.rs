//! 📒️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-app` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use semio_framework_plugin_instance_stores::stores_components::{ActionIngress, ConstructorState, InstanceStores, LocalInteraction, PeerRoster};
use semio_framework::kernel::ActorId;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_app::ArtifactApp;
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, ConfigStore, create_config_envelope, MemberFactory, Mutation, ReferenceSpaceMember};
use semio_framework_plugin_decl_app::tool_contract::{ArtifactToolFactoryRegistry, ArtifactToolPublicContract, ArtifactToolPublicationLane};
use std::collections::BTreeMap;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_instance_ladders::ladders_components::{CloseCursors, MaintenanceCursors};
use semio_framework_plugin_instance_history::history_components::{CommandHistory, HostEvents, Transactions};
use store::os_store::composition_coordinator::CompositionCoordinator;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_instance_docs::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use protocol::OpText;
use semio_framework_plugin_instance_jobs::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_registry::artifact_registration::PluginRuntimeRegistry;
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_plugin_job::lifted_full_operation_job::TypedCommandFullOperationJobFactory;
use crate::VcsArtifactApp;
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use store::os_store::document_materialize::create_document_envelope;
use semio_framework_plugin_job::reserved_jobs::register_framework_reserved_factories;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel as time_travel;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧬️ Constructs a store-only wrapper with an empty registry. Every action and typed
    /// command fails closed before decoding until a registry-backed wrapper is constructed.
    pub async fn new(app: A, actor: ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        Self::with_registry(app, AppActionRegistry::default(), actor, mounted_policy, identity).await
    }

    /// 🧬️ Constructs a wrapper carrying the app's {@link AppActionRegistry} so `handle_action`
    /// enforces default materialization, required-arg validation, and kind discipline.

    /// 🧱 Runs `initial_snapshot` on its own frame so fixture sync cannot nest inside envelope construction.
    #[inline(never)]
    async fn isolate_artifact_app_initial_snapshot() -> A::Snapshot {
        A::initial_snapshot().await
    }

    pub async fn with_registry(app: A, registry: AppActionRegistry, actor: ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        Self::with_registry_on_bus(app, registry, semio_framework::ActionBus::new(), actor, mounted_policy, identity).await
    }

    /// 🫂️ Activates an app controller against Platform's injected shared production registry.
    pub async fn with_registry_on_bus(app: A, registry: AppActionRegistry, tool_jobs: semio_framework::ActionBus, actor: ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        let mounted_policy = mounted_policy.validate().expect("explicit app embedding policy");
        let app_id = app.instance_id().await.to_string();
        PluginRuntimeRegistry::publish_framework_schema_documents().expect("the OS's own schema documents never conflict");
        for documents in <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMA_DOCUMENTS {
            semio_framework_schema_registry::register_referenced_schema_documents(documents);
        }
        let snapshot = Self::isolate_artifact_app_initial_snapshot().await;
        let mut envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &app_id, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        envelope.dialect = Some(A::DIALECT.into());
        let config_id = format!("{}-config", app_id);
        let config_envelope = create_config_envelope::<A::Config, A::ConfigMutation>(A::config_schema().await, &config_id, A::initial_config().await, None);
        let draft_id = format!("{}-draft", app_id);
        let draft_envelope = create_document_envelope::<A::Draft, A::DraftMutation>("draft.empty", &draft_id, A::initial_draft().await, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let interaction_id = format!("{}-interaction", app_id);
        let interaction_envelope = create_document_envelope::<protocol::InteractionState, InteractionConfigMutation>("framework.interaction", &interaction_id, protocol::InteractionState::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor.clone()).await.expect("failed to create document store");
        store.install_operation_wire(store::ArtifactOperationWire::semantic::<A::Snapshot>().with_batch_digest());
        store.enable_convergence_early_exit();
        store.defer_remote_replays(Some(time_travel::time_travel_replay_turn_budget()));
        store.defer_local_replays(Some(time_travel::time_travel_replay_turn_budget()));
        let config_store = ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(config_envelope.await, actor.clone()).await.expect("failed to create config store");
        let draft_store = store::DraftStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(draft_envelope, actor.clone()).await.expect("failed to create draft store");
        let interaction_store = ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(interaction_envelope, actor.clone()).await.expect("failed to create interaction store");
        let genesis_mutations = A::genesis().await;
        if !genesis_mutations.is_empty() {
            store.dispatch(ArtifactCommand::Apply { mutations: genesis_mutations, transaction: None }, identity).await.expect("ArtifactApp::genesis mutations must apply cleanly onto a freshly constructed store");
        }
        let dispatch_report = protocol::DispatchReport { policy: store.merge_policy(), worst: None, messages: Vec::new() };
        let mut framework_tool_registry = ArtifactToolFactoryRegistry::<A>::new(&tool_jobs, &app_id);
        register_framework_reserved_factories(&mut framework_tool_registry).expect("framework-owned reserved factories must preserve exact owner/controller/schema/tool authority");
        let framework_tool_registrations = framework_tool_registry.finish();
        let mut app_tool_registry = ArtifactToolFactoryRegistry::<A>::new(&tool_jobs, &app_id);
        A::register_tool_job_factories(&mut app_tool_registry).expect("app-owned tool factories must preserve exact owner/controller/schema/tool authority");
        let app_tool_registrations = app_tool_registry.finish();
        let (tool_job_controller_id, tool_job_registrations) = semio_framework_plugin_job::reserved_jobs::tool_job_registration::<A>(&registry, &app_id, A::DOCUMENT_SCHEMA, <A::Command as ::protocol::OpBinary>::TOOL_JOB_IDS, &tool_jobs, &app_tool_registrations)
            .expect("tool proof catalog must exactly join migrated generated declarations to live concrete factories");
        let artifact_one_item_factory = A::build_artifact_store_one_item_preparation_factory();
        let config_one_item_factory = A::build_config_store_one_item_preparation_factory();
        let draft_one_item_factory = A::build_draft_store_one_item_preparation_factory();
        let presence_one_item_factory = A::build_presence_store_one_item_preparation_factory();
        let transient_one_item_factory = A::build_transient_store_one_item_preparation_factory();
        let presence_local_root_retirement_factory = A::build_presence_local_root_retirement_factory();
        let transient_local_root_retirement_factory = A::build_transient_local_root_retirement_factory();
        let mut window_config_store = WindowConfigOwnerRegistry::new(actor);
        A::register_window_config_owners(&mut window_config_store).expect("window config owners must be uniquely registered by exact window kind");
        let mut window_transient_store = WindowTransientOwnerRegistry::default();
        A::register_window_transient_owners(&mut window_transient_store).expect("window transient owners must be uniquely registered by exact window kind");
        let mut unsupported_publication_contracts = BTreeMap::new();
        for registration in framework_tool_registrations.values().chain(app_tool_registrations.values()) {
            let unsupported = registration.publication_lanes.iter().copied().find_map(|lane| match lane {
                ArtifactToolPublicationLane::Artifact if artifact_one_item_factory.is_none() => Some("artifact"),
                ArtifactToolPublicationLane::Config if config_one_item_factory.is_none() => Some("config"),
                ArtifactToolPublicationLane::Draft if draft_one_item_factory.is_none() => Some("draft"),
                ArtifactToolPublicationLane::Presence if presence_one_item_factory.is_none() || presence_local_root_retirement_factory.is_none() => Some("presence"),
                ArtifactToolPublicationLane::Transient if transient_one_item_factory.is_none() || transient_local_root_retirement_factory.is_none() => Some("transient"),
                ArtifactToolPublicationLane::WindowConfig if window_config_store.is_empty() => Some("window-config"),
                ArtifactToolPublicationLane::WindowTransient if window_transient_store.is_empty() => Some("window-transient"),
                ArtifactToolPublicationLane::Child => None,
                ArtifactToolPublicationLane::Interaction => None,
                ArtifactToolPublicationLane::HostOnly
                | ArtifactToolPublicationLane::Artifact
                | ArtifactToolPublicationLane::Config
                | ArtifactToolPublicationLane::Draft
                | ArtifactToolPublicationLane::Presence
                | ArtifactToolPublicationLane::Transient => None,
                ArtifactToolPublicationLane::WindowConfig => None,
                ArtifactToolPublicationLane::WindowTransient => None,
            });
            if let Some(lane) = unsupported {
                unsupported_publication_contracts.insert(registration.key.tool_id.clone(), lane);
            }
        }
        let instance_operation_owner = ArtifactInstanceOperationOwnerHandle::new(A::build_instance_operation_owner());
        let mut bounded_tool_proofs = BTreeMap::new();
        let mut bounded_tool_contracts = Vec::with_capacity(tool_job_registrations.len());
        for proof in tool_job_registrations {
            let tool_id = proof.row.tool_id.to_string();
            if app_tool_registrations.contains_key(&tool_id) || framework_tool_registrations.contains_key(&tool_id) {
                continue;
            }
            bounded_tool_contracts.push(ArtifactToolPublicContract {
                owner: proof.owner,
                controller_id: proof.row.controller_id.to_string(),
                tool_id: tool_id.clone(),
                schema_id: proof.schema_id(),
                max_raw_wire_bytes: proof.contract().max_raw_wire_bytes,
                publication_lanes: &[ArtifactToolPublicationLane::HostOnly],
            });
            let factory = TypedCommandFullOperationJobFactory::<A>::from_proof(proof.clone()).expect("validated bounded reducer proof must retain its concrete owner authority");
            tool_jobs.register_once(factory).expect("validated bounded reducer factory must register exactly once");
            assert!(bounded_tool_proofs.insert(tool_id, proof).is_none(), "validated bounded reducer proofs must remain unique by tool id");
        }
        let mut presence_store = store::PresenceStore::new(A::Presence::default());
        if let Some(factory) = presence_local_root_retirement_factory.as_ref() {
            presence_store.install_local_retirement_factory(factory.clone()).expect("presence local retirement factory installs exactly once during app construction");
        }
        if let Some(factory) = A::build_presence_peer_retirement_factory() {
            presence_store.install_peer_retirement_factory(factory).expect("presence peer retirement factory installs exactly once during app construction");
        }
        let composition = CompositionCoordinator::new().await;
        let mut this = Self {
            app,
            mounted_policy,
            constructor: ConstructorState::new(),
            stores: InstanceStores::new(store, config_store, draft_store, presence_store, window_config_store, window_transient_store, artifact_one_item_factory, config_one_item_factory, draft_one_item_factory, presence_one_item_factory, transient_one_item_factory, presence_local_root_retirement_factory, transient_local_root_retirement_factory, unsupported_publication_contracts, interaction_store),
            roster: PeerRoster::new(),
            ingress: ActionIngress::new(),
            interaction: LocalInteraction::new(),
            history: CommandHistory::new(),
            host_events: HostEvents::new(),
            txn: Transactions::new(),
            child_composition: ChildComposition::new(composition),
            pending: PendingReserved::new(dispatch_report),
            tools: ToolOperations::new(registry, tool_job_controller_id, tool_jobs, bounded_tool_proofs, bounded_tool_contracts, framework_tool_registrations, app_tool_registrations, instance_operation_owner),
            media: MediaTransfers::new(),
            ingest: EnvelopeJobs::new(),
            maintenance: MaintenanceCursors::new(),
            close: CloseCursors::new(),
            tool_runs: ToolRunLedger::default(),
            time_travel: TimeTravelLedger::default(),
            tool_machines: ToolMachineRuntime::default(),
        };
        this.child_composition.seed_genesis_children(&mut this.stores, identity).await.expect("ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store");
        this.child_composition.followed_parent_generation = this.stores.store.generation_now();
        this
    }
}
