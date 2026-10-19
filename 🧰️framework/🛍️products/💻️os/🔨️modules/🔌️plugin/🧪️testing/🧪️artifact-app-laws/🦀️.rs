//! 🧪️ The generic test-harness helpers for `ArtifactApp` implementors, with the shared imports of the former inline module. See [`super`].

//! 🧪️ Generic test-harness helpers for `ArtifactApp` implementors. Factors out the ~24x duplicated
//! `meta()`/`new_app()`/`new_app_with_registry()`/`paired_apps()` boilerplate plus the repeated
//! undo-redo / two-instance-convergence / ingest-idempotency test *bodies* (parameterized by closures
//! for the app-specific action names/snapshot shape, so only the control flow is shared). Not
//! Enabled only by this crate's tests or the explicit `artifact-app-testing` feature; see
//! `terminology_tests`/`panel_kit_tests` above for the sibling pattern of testing SDK primitives
//! themselves inline.

use semio_framework_plugin_job::reserved_jobs::register_framework_reserved_factories;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactToolFactoryRegistry, TypedOperationResultLane};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};
pub use super::{OriginalFixtureCaller, OriginalFixtureScope};

#[path = "🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs"]
mod fixture_dispatch_steps_and_document_loading;
pub use fixture_dispatch_steps_and_document_loading::{
    assert_editor_genesis_children_declared, assert_viewer_genesis_children_declared,
    dispatch_fixture_action, dispatch_fixture_command, dispatch_fixture_response_action, poll_fixture_media_export, produce_fixture_media, retain_fixture_media_export_result, submit_fixture_media_export, take_fixture_media_export_chunk, drain_maintenance_pressure, export_fixture_media,
    fixture_identity, fixture_job_outcome, fixture_mounted_policy, fixture_payload_bytes,
    fixture_progress, fixture_retained_turn, fixture_step, fixture_step_context,
    fixture_step_context_for, load_document, load_document_text, plugin_load_document,
    plugin_load_document_text, plugin_settle_store_constructor, typing_run, with_document_view,
    FixtureStepOutcome, TypingRun, LAW_DOCUMENT_LOAD_OPERATION,
};

#[path = "🌳️fixture-tree-projection-and-scenes/🦀️.rs"]
mod fixture_tree_projection_and_scenes;
pub use fixture_tree_projection_and_scenes::{
    built_carrier_text, built_surface_scene, decode_fixture_scene, decode_fixture_scene_with_lanes,
    document_input_commit_findings, document_input_commit_lint, fixture_carrier_text,
    fixture_scene_node, observe_and_retire_fixture_tree, project_and_retire_fixture_tree,
    FIXTURE_TREE_MAX_DEPTH, FIXTURE_TREE_MAX_NODES, FIXTURE_TREE_RETIRE_STEPS,
};
use fixture_tree_projection_and_scenes::project_fixture_node;

#[path = "🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs"]
mod registered_app_construction_and_typed_operation_settlement;
pub use registered_app_construction_and_typed_operation_settlement::{
    capture_fixture_window_config, close_registered_fixture_app, drain_store_displaced_retirements,
    fixture_close_grant, meta, new_app, new_app_with_registry, new_app_with_registry_and_members,
    new_registered_app, new_registered_app_with_members,
    register_framework_reserved_action_factories, retain_original_ownership_receipts,
    settle_extension_invocations, settle_registered_typed_operation, settle_store_constructor,
    settle_store_constructor_close, PendingExtensionInvocation, SettledExtensionInvocations,
    TypedOperationFixtureOwnershipReceipt, TypedOperationFixtureReceipt,
};

#[path = "👯️paired-apps-history-and-convergence-laws/🦀️.rs"]
mod paired_apps_history_and_convergence_laws;
pub use paired_apps_history_and_convergence_laws::{
    assert_declared_actions_bridge_to_commands, assert_ingest_idempotent,
    assert_proposes_transaction, assert_registered_ingest_idempotent,
    assert_registered_ingest_idempotent_with_members, assert_transaction_commits_as_one_edit,
    assert_transaction_rollback_leaves_state_untouched, assert_two_instances_converge,
    assert_two_registered_instances_converge, assert_two_registered_instances_converge_with_members,
    assert_undo_redo_round_trip, paired_apps, paired_registered_apps,
    paired_registered_apps_with_members, settle_history_verb,
};

#[path = "🔎️declared-verb-probing/🦀️.rs"]
mod declared_verb_probing;
pub use declared_verb_probing::{
    agent_preview, assert_declared_verbs_honour_their_declarations,
    dispatch_addressed_fixture_on_original, probe_agent_lane, probe_declared_verbs,
    observe_registered_fixture_app, render_registered_fixture_app, render_registered_fixture_app_over, DeclaredVerbBoot,
};

#[path = "🔭️bounded-viewer-fixture-and-viewer-laws/🦀️.rs"]
mod bounded_viewer_fixture_and_viewer_laws;
pub use bounded_viewer_fixture_and_viewer_laws::{
    assert_editor_and_viewer_share_dialect, assert_surface_dependencies_declared,
    assert_viewer_never_mutates, new_viewer, reduce_editor_command, BoundedViewerFixture,
};

#[path = "🧭️declaration-registration-laws/🦀️.rs"]
mod declaration_registration_laws;
pub use declaration_registration_laws::{
    assert_declaration_registration_is_atomic, assert_declaration_tree_registers_all,
    assert_subset_declaration_ids_are_derived,
};

pub use semio_framework_plugin_declared_verdict::{
    DeclaredArgumentProbe, DeclaredVerbEffect, DeclaredVerbFinding, DeclaredVerbOutcome, DeclaredVerbProbe, DeclaredVerbWindowProbe, declared_argument_alternatives, declared_bridge_arguments, declared_verb_agent_divergences, declared_verb_ever_wrote_document, declared_verb_findings, declared_verb_probe, declared_verb_staged_wrote_document, declared_verb_wrote_document
};

// 🎫️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — lane 0-F. New
// subregion, appended after the peer `🧫️fixtures` subregion above without touching it.
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer, ViewerApp};
