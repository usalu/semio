//! 🧪️ The generic test-harness helpers for `ArtifactApp` implementors, with the shared imports of the former inline module. See [`super`].

//! 🧰️ Generic test-harness helpers for `ArtifactApp` implementors. Factors out the ~24x duplicated
//! `meta()`/`new_app()`/`new_app_with_registry()`/`paired_apps()` boilerplate plus the repeated
//! undo-redo / two-instance-convergence / ingest-idempotency test *bodies* (parameterized by closures
//! for the app-specific action names/snapshot shape, so only the control flow is shared). Not
//! Enabled only by this crate's tests or the explicit `artifact-app-testing` feature; see
//! `terminology_tests`/`panel_kit_tests` above for the sibling pattern of testing SDK primitives
//! themselves inline.

use semio_plugin_job::reserved_jobs::register_framework_reserved_factories;
use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_app_builder::App;
use semio_plugin_window_kits::AppActionRegistry;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_decl_app::tool_contract::{ArtifactToolFactoryRegistry, TypedOperationResultLane};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_instance_app::VcsArtifactApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};
pub use super::{OriginalFixtureCaller, OriginalFixtureScope, OriginalWorkFixture, receive_persistent_fixture, receive_view_fixture, typing_run, TypingRun};
    use semio_framework_value::ErasedSnapshotRetirement;

    /// 🧮️ Refuses a test binary whose allocator is not the heap witness: every heap-equality assertion of these laws would otherwise read `(0, 0)` and fail any non-zero retained page.  Linking the testing caller installs it.
    pub fn assert_heap_witness_installed(){
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||std::hint::black_box(Box::new([0u8;64])));
        assert!(heap.requested_bytes>=64,"fixture heap witness missing: link semio-plugin-testing-caller into this test binary");
    }

    /// 🪙️ A fixture without a job turn funds one page presentation from an explicit finite wallet of exactly its grant.
    pub fn take_fixture_result_turn<P:PluginApp>(app:&mut P,receiver:u32,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Option<semio_plugin_job::TypedOperationResultPage>,semio_framework_value::retained_clone::RetainedCloneProgress),semio_framework::Fault>{
        let mut wallet=semio_framework_value::AuthoringWallet::new(grant,1<<16);
        match wallet.run(grant,1<<16,|effect|app.take_typed_operation_result_page(receiver,effect)).expect("fixture wallet admits its finite page stock"){Some(authored)=>authored.output,None=>Ok((None,Default::default()))}
    }

    /// 📃️ Observes each production page presentation with the exact original fixture grant.

    pub fn take_fixture_result_page<P:PluginApp>(app:&mut P,receiver:u32,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<semio_plugin_job::TypedOperationResultPage>,semio_framework::Fault>{

        assert_heap_witness_installed();loop{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||take_fixture_result_turn(app,receiver,grant));let(page,p)=result?;assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));if page.is_some()||p==Default::default(){return Ok(page);}}

    }

    /// ♻️ Returns a projected page's token only after its exact physical alias closure.

    pub fn close_fixture_result_page(page:&mut semio_plugin_job::TypedOperationResultPage,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_plugin_job::TypedOperationResultToken{

        let token=page.token;for _ in 0..100000{if page.terminal_is_empty(){return token;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||page.close_step(grant).expect("original fixture receiver page closure"));assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_ne!(step.progress(),Default::default(),"original fixture close grant must fund its unchanged page authority");}panic!("original fixture receiver page did not close");

    }

    /// 🧳️ Retires caller-owned fixture custody without losing its pending cursor on refusal or pause.
    pub fn retire_fixture_owned<T:semio_framework_value::retirement::RetireOwned+'static>(pending:&mut Option<T>,active:&mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,grant:RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        for _ in 0..OriginalFixtureCaller::maximum_turns(){
            if pending.is_none()&&active.is_none(){return Ok(true)}
            let demand=if let Some(owner)=active.as_ref(){store::os_store::snapshot_read::artifact_retirement_box_demands(owner.as_ref(),grant.maximum_copy_bytes)}else{store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(pending)}?;
            let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(grant,|same|semio_framework_job::close_original_retained_step(same,demand,|issued|{
                if active.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(active,issued)}else{store::os_store::snapshot_read::artifact_retirement_admit_owned(pending,active,issued)}
            })));
            let progress=match &result{Ok(step)=>step.progress(),Err(failure)=>failure.effects};
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            result?;
            if progress==Default::default(){return Ok(pending.is_none()&&active.is_none())}
        }
        Ok(pending.is_none()&&active.is_none())
    }

    /// 📨️ Presents a real mounted page under its caller, returning when the current turn cannot advance.
    pub fn present_fixture_mounted_page<A: semio_plugin_decl_app::ArtifactApp>(mounted:&mut semio_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation<A>,grant:semio_framework_value::RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_plugin_job::TypedOperationResultPage>,semio_framework_job::OriginalTurnFailure>{
        use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
        for _ in 0..OriginalFixtureCaller::maximum_turns(){
            let mut output=None;
            let demand=mounted.result_page_presentation_demands()?;
            let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_plugin_ownership::bounded_ownership::original_lifecycle_effect_step(cx,grant,demand,|_,effect|{
                let (page,progress)=mounted.take_result_page(effect)?;
                let complete=page.is_some();output=page;
                Ok((if complete{PluginLifecycleStep::Complete(progress)}else{PluginLifecycleStep::Progress(progress)},complete))
            }));
            let step=result?;
            let progress=match step{PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)=>progress,_=>Default::default()};
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            if output.is_some()||progress==Default::default(){return Ok(output)}
        }
        Err(semio_framework_value::ValueError::invariant("mounted page presentation exceeded its original caller bound").into())
    }

    /// 📫️ Retires the acknowledged producer through its financed public port without renewing the caller's turn.
    pub fn retire_fixture_acknowledged_mounted_page<A: semio_plugin_decl_app::ArtifactApp>(mounted:&mut semio_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation<A>,grant:semio_framework_value::RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
        for _ in 0..OriginalFixtureCaller::maximum_turns(){
            if !mounted.result_page_retirement_pending(){return Ok(true)}
            let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(grant,|same|mounted.advance_acknowledged_result_page(same)));
            let step=result?;
            let progress=match step{PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)=>progress,_=>Default::default()};
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            if progress==Default::default(){return Ok(!mounted.result_page_retirement_pending())}
        }
        Err(semio_framework_value::ValueError::invariant("mounted page retirement exceeded its original caller bound").into())
    }





#[path = "🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs"]
mod fixture_dispatch_steps_and_document_loading;
pub use fixture_dispatch_steps_and_document_loading::{
    assert_editor_genesis_children_declared, assert_viewer_genesis_children_declared,
    arm_host_lifetime, fixture_renew_turn, HostLifetimeGuard, HostLifetimeStock,
    dispatch_fixture_action, dispatch_original_fixture_command, ensure_fixture_cancellation_issuer, FixtureOriginalJob, dispatch_fixture_command, dispatch_fixture_response_action, poll_fixture_media_export, produce_fixture_media, retain_fixture_media_export_result, submit_fixture_media_export, take_fixture_media_export_chunk, drain_maintenance_pressure, export_fixture_media,
    fixture_identity, fixture_job_outcome, fixture_mounted_policy, fixture_payload_bytes, fixture_pending_effects,
    fixture_progress, fixture_retained_turn, fixture_step, fixture_step_context,
    fixture_step_context_for, load_document, load_document_text, plugin_load_document,
    plugin_load_document_text, plugin_settle_store_constructor, with_document_view,
    FixtureStepOutcome, LAW_DOCUMENT_LOAD_OPERATION,
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
    capture_fixture_window_config, close_registered_fixture_app, consume_media, consume_media_refused_then_funded, drain_store_displaced_retirements, import_media,
    fixture_close_grant, meta, new_app, new_app_with_registry, new_app_with_registry_and_members,
    new_registered_app, new_registered_app_with_members,
    publish_framework_schemas,
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
    paired_registered_apps_with_members, settle_history_verb, settle_history_verb_as,
};

#[path = "👥️presence-round-trip-law/🦀️.rs"]
mod presence_round_trip_law;
pub use presence_round_trip_law::{assert_presence_round_trip, publish_fixture_presence_roster};

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

pub use semio_plugin_declared_verdict::{
    DeclaredArgumentProbe, DeclaredVerbEffect, DeclaredVerbFinding, DeclaredVerbOutcome, DeclaredVerbProbe, DeclaredVerbWindowProbe, declared_argument_alternatives, declared_bridge_arguments, declared_verb_agent_divergences, declared_verb_ever_wrote_document, declared_verb_findings, declared_verb_probe, declared_verb_staged_wrote_document, declared_verb_wrote_document
};

// 🎫️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — lane 0-F. New
// subregion, appended after the peer `🧫️fixtures` subregion above without touching it.
use semio_plugin_decl_surface::{ArtifactEditor, ArtifactViewer, ViewerApp};
