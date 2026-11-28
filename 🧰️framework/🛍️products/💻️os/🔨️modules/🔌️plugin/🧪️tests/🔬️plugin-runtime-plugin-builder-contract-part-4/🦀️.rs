//! 🔬️ Part 4 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.

use semio_plugin_decl_media::ARTIFACT_OUTPUT_CHUNK_BYTES;
use semio_plugin_decl_views::AppOperationContext;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_plugin_decl_media::ArtifactMediaExportPoll;
use semio_plugin_decl_media::ArtifactMediaExportResult;
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobContext;
use semio_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_plugin_decl_emission::HostEvent;
use semio_plugin_decl_emission::InteractionWrite;
use protocol::LOCAL_ACTOR_ID;
use semio_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::RetirementDemand;
use semio_plugin_decl_app::tool_contract::ToolOwnerWitness;
use semio_plugin_decl_app::tool_contract::TypedOperationResultLane;
use serde_json::Value;
use semio_framework_value::ValueError;
use semio_plugin_reactor_app::addressed_action_view;
use semio_plugin_reactor_app::admit_addressed_action_view;
use semio_plugin_testing_laws::artifact_app_laws;
use semio_plugin_plugin_program::plugin_runtime;
use semio_os_kernel as protocol;
use semio_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_os_kernel as store;
use semio_plugin_tool_run as tool_run;
use semio_plugin_abi::ArtifactAnalysis;
use semio_plugin_abi::ArtifactAnalyzer;
use semio_plugin_abi::ArtifactBuilder;
use semio_plugin_abi::ArtifactChildren;
use semio_plugin_abi::ArtifactComposer;
use semio_plugin_abi::ArtifactComposition;
use semio_plugin_abi::ArtifactDecomposer;
use semio_plugin_decl_surface::ArtifactEditor;
use semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_plugin_decl_surface::ArtifactViewer;
use semio_plugin_decl_emission::AsyncTaskRun;
use semio_plugin_decl_emission::AsyncTaskSource;
use semio_plugin_decl_views::child_view::ChildContentRetainers;
use semio_plugin_abi::DerivedArtifactSpec;
use semio_plugin_window_kits::LabelAxes;
use semio_plugin_decl_app::NaturalFileDecodeCursor;
use semio_os_kernel::io::SubsetValidator;
use semio_plugin_decl_emission::ToolCommandCatalog;
use semio_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_plugin_window_config::WindowConfigOwner;
use semio_plugin_window_config::WindowConfigReplaceMutation;
use semio_plugin_window_kits::WindowKit;
use semio_plugin_window_transient::window_transient::WindowTransientOwner;
use store::ReferenceSpaceMember as _;
use semio_plugin_ownership::bounded_ownership::{plugin_demand_grant, ARTIFACT_LIVE_OUTPUT_SLOTS};
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{TestRuntimeApps, ContractApp, SingleDocumentApp, TEST_APP_WINDOW_BODY_KEY, TEST_COMMAND_RETAINED_GRANT, TEST_HOST_EVENTS, TEST_RESTART_SCHEMA, TEST_RESTART_TOOL, TEST_RETAINED_COMMAND_RAW_BYTES, TEST_RETAINED_COMMAND_TOOL, TestApp, TestCommand, app_maintenance_turn, archive_fault_text, close_member_admission_app, close_reserved_app, contract_app, contract_app_under_test, contract_composed_app, contract_composed_app_raw, drain_and_close_composed_fixture, drive_job_close, drive_single_document_archive, dv, inline_interaction_select, interaction_app_under_test, interaction_target_args, job_close_grant, label_in, meta, reads_child_count, register_test_child, reserved_action, settle_contract_app, settle_reserved, test_child_emit, test_retained_command_id, test_retained_command_payload, test_task};
use semio_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_ui_locale::Locale;
use semio_ui_locale::LocalizedLabel;
use semio_ui_locale::Terminology;
use semio_plugin_decl_emission::{ActionMeta, Emit, TaskCtx, TaskResolution};
use semio_plugin_app_builder::App;
use semio_plugin_decl_app::{ArtifactApp, EphemeralSnapshot};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_window_kits::AppActionRegistry;
use semio_plugin_decl_media::ArtifactDownloadOutput;
use semio_plugin_decl_media::ArtifactMediaExportHandle;
use semio_plugin_decl_media::ArtifactOutputChunks;
use semio_plugin_decl_views::{InteractionHoverState, NoDraftMutation};
use semio_plugin_decl_views::history_view::HistoryView;
use semio_plugin_instance_app::VcsArtifactApp;
use semio_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework::{MediaClass, MediaType};
use semio_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework::{ActionKind, InteractionVerb, MediaForm, NOTE_SHELL_COMMAND_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID};
use semio_framework_job::InteractiveJob as _;
use semio_ui_contract::{ActionId, SurfaceId, Trigger, UiIntent, UiNodeId, UiRevision};
use serde_json::json;
use pack::snapshot::ArtifactPack;
use semio_os_kernel::os_store::backbone::Backbone;
use semio_os_kernel::{MemberFactory, SpaceMember};
use semio_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;

/// 🪙️ Runs one operation inside the allocation effect of a funded original retirement pump turn.
fn run_retirement_effect<R>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> R {
    try_retirement_effect(grant, operation).expect("the retirement effect is funded")
}

/// 🪙️ Like [`run_retirement_effect`], answering `None` when the grant funds no effect at all.
fn try_retirement_effect<R>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> Option<R> {
    semio_framework_job::OriginalRetirementPump::new()
        .turn(1 << 16, u64::MAX, semio_framework_job::logical_now_us, grant)
        .and_then(|mut turn| turn.with_effect(operation))
        .expect("the retirement effect has a finite original Source turn")
        .map(|(output, _)| output)
}


const PART_4_JOB_POLICY: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
const PART_4_ORIGIN_POLICY: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1_048_576, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
/// 🧹️ The job return origins of one test thread. A test that failed before closing them has already reported its own failure; its unclosed origins are leaked on thread exit instead of panicking inside the thread-local destructor, which would abort the whole test binary.
#[derive(Default)]
struct Part4Origins(std::collections::HashMap<usize, semio_framework_job::OriginalJobReturnOrigin>);
impl Drop for Part4Origins {
    fn drop(&mut self) {
        for (_, origin) in self.0.drain() { if !origin.terminal_is_empty() { std::mem::forget(origin) } }
    }
}
thread_local! { static PART_4_ORIGINS: std::cell::RefCell<Part4Origins> = std::cell::RefCell::new(Part4Origins::default()); }

/// 🎟️ Admits a job's return origin until its body is ready, as the host does at job birth.
fn prepare_job_origin(origin: &mut semio_framework_job::OriginalJobReturnOrigin, cx: &mut semio_framework_job::StepContext<'_>) {
    for _ in 0..semio_plugin_testing_caller::OriginalFixtureCaller::maximum_turns() {
        if origin.body_ready() { return }
        cx.with_retained_loan(PART_4_ORIGIN_POLICY, |same| origin.admit(same).map(|_| ())).expect("original job return origin admission");
    }
    assert!(origin.body_ready(), "the job return origin admits within its turn bound");
}

/// ▶️ One fixture job step whose outcome travels the job's own paid return origin.
fn job_step<J: semio_framework_job::InteractiveJob + ?Sized>(job: &mut J, cx: &mut semio_framework_job::StepContext<'_>) -> artifact_app_laws::FixtureStepOutcome {
    let key = (job as *mut J).cast::<()>() as usize;
    PART_4_ORIGINS.with(|origins| {
        let mut origins = origins.borrow_mut();
        let origin = origins.0.entry(key).or_insert_with(semio_framework_job::OriginalJobReturnOrigin::empty);
        prepare_job_origin(origin, cx);
        let mut outcome = None;
        cx.with_retained_loan(PART_4_JOB_POLICY, |same| { outcome = Some(artifact_app_laws::fixture_step(job, same, origin.receiver()?)); Ok(()) }).expect("original job step");
        let outcome = outcome.expect("original job step returned");
        if !cx.is_cancelled() { prepare_job_origin(origin, cx) }
        outcome
    })
}

/// 🧹️ Returns every job return origin this thread admitted.
fn close_job_origins(cx: &mut semio_framework_job::StepContext<'_>) {
    PART_4_ORIGINS.with(|origins| {
        for origin in origins.borrow_mut().0.values_mut() {
            for _ in 0..semio_plugin_testing_caller::OriginalFixtureCaller::maximum_turns() {
                if origin.terminal_is_empty() { break }
                cx.with_retained_loan(PART_4_ORIGIN_POLICY, |same| origin.close_step(same).map(|_| ())).expect("original job return origin close");
            }
            assert!(origin.terminal_is_empty(), "the job return origin closes within its turn bound");
        }
        origins.borrow_mut().0.clear();
    })
}

include!("../🔬️plugin-runtime-plugin-builder-contract-part-4-x/🦀️.rs");

include!("🎞️host-events-media-cancel-and-download-seam/🦀️.rs");
include!("👶️child-emit-prepublication-and-ephemeral-lanes/🦀️.rs");
include!("📚️archive-load-reload-and-revert/🦀️.rs");
include!("↩️reserved-undo-and-note-shell-command/🦀️.rs");
include!("📡️host-events-intents-and-streamed-tick/🦀️.rs");
include!("🖱️interaction-hover-and-select-laws/🦀️.rs");
include!("🔀️selected-ids-merge-vocabulary-and-checkpoint/🦀️.rs");
