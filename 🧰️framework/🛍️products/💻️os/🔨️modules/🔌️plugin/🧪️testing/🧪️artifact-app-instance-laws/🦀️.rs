//! 🧪️ Instance laws of the artifact app test harness: declared verb probing, paired instance history and convergence, pane smoke and fixture tree projection, layered over the fixture dispatch of [`semio_plugin_testing_laws`].

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;

use semio_plugin_testing_laws::artifact_app_laws::*;
use pack::snapshot::ArtifactPack;
use protocol::LOCAL_ACTOR_ID;
use protocol::{OpBinary, OpText};
use semio_framework::AppRole;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::kernel::{ActorId, Effect, InvocationResult};
use semio_framework::manifest::ActionInvocation as ManifestActionInvocation;
use semio_framework::{ActionKind, AppIo, ConfigSpec, Fault, FaultCode, FaultOrigin, MediaPortSpec, ViewModel, ViewWindowInstance, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID};
use semio_framework::{Media, MediaError, MediaFingerprint};
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
use semio_framework_value::DslValue;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_plugin_app_builder::App;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_app::tool_contract::ArtifactToolFactoryRegistry;
use semio_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_decl_emission::Emit;
use semio_plugin_decl_emission::surface_contract::ViewEmit;
use semio_plugin_decl_registry::artifact_registration::surface_dependency_breaches;
use semio_plugin_decl_surface::ArtifactEditor;
use semio_plugin_decl_surface::ArtifactViewer;
use semio_plugin_decl_surface::ViewerApp;
use semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_plugin_decl_views::AppRenderOperationContext;
use semio_plugin_decl_views::ArtifactView;
use semio_plugin_decl_views::ConfigView;
use semio_plugin_decl_views::DraftView;
use semio_plugin_decl_views::InteractionHoverState;
use semio_plugin_decl_views::InteractionView;
use semio_plugin_decl_views::NoDraft;
use semio_plugin_decl_views::NoPresence;
use semio_plugin_decl_views::NoPresenceMutation;
use semio_plugin_decl_views::NoTransient;
use semio_plugin_decl_views::NoTransientMutation;
use semio_plugin_decl_views::PeerPresenceRoot;
use semio_plugin_decl_views::PresenceView;
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_plugin_decl_views::history_view::HistoryView;
use semio_plugin_instance_app::VcsArtifactApp;
use semio_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_plugin_instance_children::children_instance_root::genesis_member_schema;
use semio_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_plugin_interaction::UiAssemblyResult;
use semio_plugin_interaction::interaction_topology::empty_draft_references;
use semio_plugin_job::TransactionProposalDraft;
use semio_plugin_job::reserved_jobs::is_framework_reserved_action_id;
use semio_plugin_job::reserved_jobs::register_framework_reserved_factories;
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_store_lifecycle::store_lifecycle::ArtifactDisposal;
use semio_plugin_store_lifecycle::store_lifecycle::ArtifactOwnedDisposer;
use semio_plugin_store_lifecycle::store_lifecycle::bounded_config_store_disposer;
use semio_plugin_store_lifecycle::store_lifecycle::bounded_document_store_disposer;
use semio_plugin_plugin_program::Plugin;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_window_config::{WindowConfigOwner, WindowConfigOwnerRegistry};
use semio_plugin_window_kits::AppActionRegistry;
use semio_plugin_window_transient::window_transient::TransientView;
use semio_plugin_window_transient::window_transient::{WindowTransientOwner, WindowTransientOwnerRegistry};
use semio_ui_contract::*;
use semio_ui_contract::BuiltNode;
use semio_ui_layout::layout::ContextMenuItemSpec;
use semio_ui_layout::layout::ContextMenuRequest;
use semio_ui_layout::layout::WindowEngagement;
use semio_ui_layout::layout::WindowMeasure;
use semio_ui_locale::Locale;
use semio_ui_locale::LocalizedLabel;
use semio_ui_locale::Terminology;
use semio_ui_runtime::ComponentTree;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::future::Future;
use store::ArtifactStoreInitializationAuthority;
use store::os_store::document_materialize::create_document_envelope;
use store::os_store::store_schema::ArtifactCommand;
use store::{ArtifactStore, ConfigStore, MemberFactory, Mutation, NoMembers, ReferenceSpaceMember, SpaceMember};
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};

#[path = "🌳️fixture-tree-projection-and-scenes/🦀️.rs"]
mod fixture_tree_projection_and_scenes;
pub use fixture_tree_projection_and_scenes::{
    built_carrier_text, built_surface_scene, decode_fixture_scene, decode_fixture_scene_with_lanes,
    document_input_commit_findings, document_input_commit_lint, fixture_carrier_text,
    fixture_scene_node, observe_and_retire_fixture_tree, project_and_retire_fixture_tree,
    FIXTURE_TREE_MAX_DEPTH, FIXTURE_TREE_MAX_NODES, FIXTURE_TREE_RETIRE_STEPS,
};
use fixture_tree_projection_and_scenes::project_fixture_node;

#[path = "👯️paired-apps-history-and-convergence-laws/🦀️.rs"]
mod paired_apps_history_and_convergence_laws;
pub use paired_apps_history_and_convergence_laws::{
    assert_declared_actions_bridge_to_commands, assert_ingest_idempotent,
    assert_proposes_transaction, assert_registered_ingest_idempotent,
    assert_registered_ingest_idempotent_with_members, assert_transaction_commits_as_one_edit,
    assert_transaction_rollback_leaves_state_untouched, assert_two_instances_converge,
    assert_two_registered_instances_converge, assert_two_registered_instances_converge_with_members,
    assert_undo_redo_round_trip, paired_apps, paired_registered_apps,
    paired_registered_apps_boxed_with_members, paired_registered_apps_with_members, settle_history_verb, settle_history_verb_as, settle_reserved_action_as,
};

#[path = "🪧️pane-smoke-law/🦀️.rs"]
mod pane_smoke_law;
pub use pane_smoke_law::{assert_pane_smoke_by_example, PaneSmokeLanguage, PANE_SMOKE_LOCALES};

#[path = "🔎️declared-verb-probing/🦀️.rs"]
mod declared_verb_probing;
pub use declared_verb_probing::{
    agent_preview, assert_declared_verbs_honour_their_declarations,
    dispatch_addressed_fixture_on_original, probe_agent_lane, probe_declared_verbs,
    observe_registered_fixture_app, render_registered_fixture_app, render_registered_fixture_app_over, DeclaredVerbBoot,
};
