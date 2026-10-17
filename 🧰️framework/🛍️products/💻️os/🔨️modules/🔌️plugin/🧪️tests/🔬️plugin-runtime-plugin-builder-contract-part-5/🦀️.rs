//! 🔬️ Part 5 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.


use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_framework_plugin_command::HISTORY_ROW_OPERATION_PREVIEW;
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_framework_plugin_decl_app::MediaConsumption;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use serde_json::Value;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_window_kits::context_menu;
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
use semio_framework_plugin_decl_app::media_type;
use semio_framework_plugin_plugin_program::plugin_runtime;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel::io::resolve;
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_os_kernel as store;
use semio_framework_plugin_abi::ArtifactAnalysis;
use semio_framework_plugin_abi::ArtifactAnalyzer;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_abi::ArtifactChildren;
use semio_framework_plugin_abi::ArtifactComposer;
use semio_framework_plugin_abi::ArtifactComposition;
use semio_framework_plugin_abi::ArtifactDecomposer;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_emission::AsyncTaskRun;
use semio_framework_plugin_decl_emission::AsyncTaskSource;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_abi::DerivedArtifactSpec;
use semio_framework_plugin_window_kits::LabelAxes;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_os_kernel::io::SubsetValidator;
use semio_framework_plugin_decl_emission::ToolCommandCatalog;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_config::WindowConfigReplaceMutation;
use semio_framework_plugin_window_kits::WindowKit;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{INGRESS_TURNS_PER_COMMAND, TEST_APP_WINDOW_BODY_KEY, TestApp, TestCommand, __semio_plugin_bundle, app_maintenance_turn, close_reserved_app, command_page_authority_budget, contract_app, contract_app_raw, contract_app_under_test, contract_app_with, contract_composed_app_raw, declare_test_app_verbs, declare_test_child, drain_and_close_composed_fixture, drain_and_close_fixture, dv, fixture_close_grant, inline_interaction_select, interaction_app_under_test, interaction_target_args, label_in, meta, new_bare_test_child, prepare_original_fixture_child, publish_presence_roster, reactor_native_lifecycle_ack, reactor_native_lifecycle_finish, reactor_native_lifecycle_open, reactor_native_lifecycle_poll, register_test_child, reserved_action, sample_presence_peer, settle_contract_app, settle_reserved, test_app_surface_id, test_child_dialect};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_plugin_interaction::interaction_topology::INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_decl_views::history_view::CommandView;
use semio_framework_plugin_decl_views::history_view::HistoryCommandFilter;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework::{MediaClass, MediaType};
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::{ClipboardFragment, Effect, PasteAnchor, UiDirtyScope};
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, MediaForm, NOTE_SHELL_COMMAND_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use semio_framework_job::InteractiveJob as _;
use serde_json::json;
use std::collections::BTreeMap;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, UiMenuRef};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;


    /// 🎡️ The maintenance rotation is fair: one call runs every stage that still has something to give
    /// and reports a blocked stage only once nothing else can progress, so a block is the rotation's
    /// EVENTUAL answer and never the answer of the one call that happens to land on that stage. An
    /// authority that was LOST instead of blocked never reports a block at all, so waiting for it
    /// weakens nothing — it is the same assertion, made where the rotation can actually answer it.
    #[doc(hidden)]
pub fn assert_maintenance_reports_block<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, stage: u8, what: &str,cx:&mut semio_framework_job::StepContext<'_>) {
        for _ in 0..MAINTENANCE_STAGES {
            app.maintenance.maintenance_stage = stage;
            match app_maintenance_turn(app,cx) {
                Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { .. }) => return,
                Ok(_) => {}
                Err(fault) => panic!("{what}: bounded maintenance faulted: {fault:?}"),
            }
        }
        panic!("{what} never reported its exact block");
    }

    #[semio_framework_async_macros::async_test]
    async fn segmented_download_remains_addressable_until_terminal_none_is_observed() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let chunks = ArtifactOutputChunks::new(4);
        assert_eq!(chunks.push(vec![1, 2, 3, 4]), Ok(4));
        assert_eq!(chunks.seal(), Ok(4));
        assert!(app.media.segmented_downloads.insert(91, ArtifactDownloadOutput::new("exact.bin", "application/octet-stream", None, chunks).expect("bounded segmented output")).is_ok());
        assert_eq!(app.take_segmented_download_chunk(91).await.expect("last chunk"), Some(vec![1, 2, 3, 4]));
        assert!(app.media.segmented_downloads.contains(91));
        assert_eq!(app.take_segmented_download_chunk(91).await.expect("terminal none"), None);
        assert!(!app.media.segmented_downloads.contains(91));
        assert_eq!(app.take_segmented_download_chunk(91).await.expect_err("terminal none removes authority").code.0, "interactive-job.unknown-segmented-download");
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    // 📄️ What ONE command costs the guest's command-ingress prologue, and what a saturated page
    // authority does instead of trapping.
    //
    // Build #29 of ticket 26/09/02/PUZZLE-3D-END-TO-END ran Fill for 184 `fillBuildTick` commands
    // (~44 s at 4 Hz) and then answered every single one with
    // `plugin.command-page-allocation: fixed command page authority could not reserve its exact 64
    // slots`, thirty-two times in a row, before the guest trapped on `unreachable`. That fault is
    // `CommandPageSet::try_new`'s `try_reserve_exact` refusing — an ALLOCATION refusal, not the
    // saturation refusal (`plugin.command-page-count`) — so the guest's one fixed 512 MiB linear memory
    // could no longer hand out the block the prologue asked for.
    //
    // The block was 262 272 B: `COMMAND_MAXIMUM_PAGES * size_of::<FixedCommandPage>()`, reserved for
    // EVERY command regardless of how many pages that command declares, four times a second, for
    // commands that are one page of a few hundred bytes. The guest allocator grows its memory in 64 KiB
    // units and never shrinks it, so a quarter-megabyte contiguous request is the first thing a
    // fragmented or nearly-full guest refuses — which is exactly the order the browser observed.
    //
    // The laws below pin both halves: the reservation is the command's DECLARED page count, and every
    // refusal on this path is a `Fault` the host can display rather than a panic.
    
    /// 🔁️ Commands driven through the real ingress — over the 184 the browser reached before the
    /// prologue started refusing, so a per-command owner the authority never gives back shows up here.
    const INGRESS_COMMANDS: u64 = 320;
    
    /// 🧮️ Retained bytes one command may leave behind. The authority is transient by construction: its
    /// pages are released as the command is assembled and its deque dies with the command.
    const INGRESS_COMMAND_RETENTION_CEILING_BYTES: isize = 512;
    
    /// 📤️ Encodes one `AppCommand` into the exact page and cursor a host hands the guest — the bytes and
    /// the header come from `encode_app_command` and `CommandBatchDriver`, production code, not from a
    /// hand-written wire literal.
    async fn command_page_authority_page(instance: u32, seq: u64, command: &protocol::AppCommand) -> (semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage) {
        let encoded = protocol::encode_app_command(command).await.expect("the fixture command encodes");
        let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().expect("fixed command batch authority");
        envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance, seq, command: encoded }).unwrap_or_else(|(fault, _)| panic!("admit fixture command: {fault:?}"));
        let batch = semio_framework::kernel::CommandBatch::try_new(seq, envelopes).unwrap_or_else(|(fault, _)| panic!("admit fixture batch: {fault:?}"));
        let mut driver = semio_framework::kernel::CommandBatchDriver::new(seq, batch);
        let page = driver.next_page().expect("the host owner produces its page").expect("a nonempty command has a first page");
        assert_eq!(page.0.page_count, 1, "the fixture command is one page, which is what the 4 Hz stream that broke build #29 was");
        page
    }
    
    /// 🚚️ Drives one command to its terminal ingress status through the real `poll_kernel`: page 0 on the
    /// first turn, then plain turns while the guest steps the owner it retained, until it answers
    /// `CommandComplete` or `Fault`. Returns whether it completed and the turns it took.
    async fn drive_one_command(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<TestRuntimeApps>, page: (semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage),cx:&mut semio_framework_job::StepContext<'_>) -> (bool, usize) {
        let mut carried = Some(page);
        let mut turns = 0;
        loop {
            let result = semio_framework_plugin_reactor_turn::poll_kernel(runtime, &mut None, carried.take(), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
            turns += 1;
            match result.command_ingress {
                semio_framework::kernel::CommandIngressStatus::CommandComplete(_) => return (true, turns),
                semio_framework::kernel::CommandIngressStatus::Fault { .. } => return (false, turns),
                ref pending if turns >= INGRESS_TURNS_PER_COMMAND => panic!("one command did not reach a terminal ingress status in {turns} turns: {pending:?}"),
                _ => {}
            }
        }
    }

    /// ⚖️ LAW: a long command stream leaves the guest's retained ingress authority empty. Every command
    /// is dispatched and closed within its own turns; nothing pins a slot for a later command to queue
    /// behind, and the run retains no per-command heap.
    ///
    /// 🚚️ The commands go through the real `poll_kernel` ingress the shard drives, with cursors produced
    /// by the real `CommandBatchDriver` — not through a test-only shortcut into the app.
    ///
    /// 🧵️ This thread's own allocator scope — see `retained_heap_bytes_on_this_thread`; a
    /// process-wide difference weighs every concurrent law in the binary as this command's own.
    #[semio_framework_async_macros::async_test]
    async fn a_long_command_stream_never_pins_the_retained_ingress_authority() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_021;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-authority".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;
        for _ in 0..32 {
            reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
        }
        assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "a settled reactor holds no ingress owner");
        let mut warmed = 0;
        let mut settled = 0;
        let mut faulted = 0;
        let mut peak_occupancy = 0;
        let mut turns = 0;
        for index in 0..INGRESS_COMMANDS {
            let seq = 64 + index;
            let page = command_page_authority_page(instance, seq, &protocol::AppCommand::CommandText { seq, line: format!("noop {seq}") }).await;
            let (completed, spent) = drive_one_command(&runtime, page,cx).await;
            turns += spent;
            if !completed {
                faulted += 1;
            }
            let occupancy = semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy();
            peak_occupancy = peak_occupancy.max(occupancy);
            assert_eq!(occupancy, 0, "command {index} left an owner in the retained ingress authority — every later command queues behind it");
            if warmed == 0 && index + 1 == INGRESS_COMMANDS / 4 {
                warmed = index + 1;
                settled = semio_framework_trace::retained_heap_bytes_on_this_thread();
            }
        }
        let retained = semio_framework_trace::retained_heap_bytes_on_this_thread() - settled;
        let measured = (INGRESS_COMMANDS - warmed) as isize;
        let per_command = retained / measured;
        let census = format!("{INGRESS_COMMANDS} commands, {turns} turns, {faulted} faulted, peak ingress occupancy {peak_occupancy}, {retained} B over the last {measured}");
        assert!(
            per_command <= INGRESS_COMMAND_RETENTION_CEILING_BYTES,
            "the command ingress retains {per_command} B per command (ceiling {INGRESS_COMMAND_RETENTION_CEILING_BYTES}) — the guest's linear memory is fixed and never shrinks: {census}"
        );
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_action_emits_kernel_op_with_true_inverse() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 1 })).unwrap());
        assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 0 })).unwrap()]));
        assert_eq!(result.inverse_group.mutations.len(), 1);
        assert_eq!(app.stores.test_snapshot().await.count, 1);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 👥️ Contract-freeze §C7.6, "zero app-side code": an app that merely DECLARES an interaction
    /// domain with `broadcast: true` (nothing app-specific) must see its live selection show up in
    /// `ephemeral_snapshot().interaction` — assembled purely from `AppDefinition.interactions` plus
    /// the framework-owned `interaction_store`/`interaction_hover` state.
    ///
    /// 🧪️ `interaction_topology`'s fixture only knows "item-1" once `doc.snapshot.label` is
    /// non-empty (see that fn's own doc comment) — without this seed, `validate_state` prunes
    /// the pick as an unknown id and the domain never shows up in `interaction_state()`.
    #[semio_framework_async_macros::async_test]
    async fn ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;

        let snapshot = app.ephemeral_snapshot().await;
        assert!(!snapshot.interaction.is_empty(), "a broadcasting domain with a live selection must not encode to empty bytes");
        let mut pos = 0usize;
        let decoded = protocol::decode_presence_interaction(&snapshot.interaction, &mut pos).await.expect("interaction bytes decode");
        assert_eq!(decoded.app_id, app.app_id().await);
        assert_eq!(decoded.domains.len(), 1);
        assert_eq!(decoded.domains[0].domain, "items");
        assert_eq!(decoded.domains[0].selected, vec!["item-1".to_string()]);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📏️ The unoptimized-build thread stack the bounded-route law runs under. The plugin guest's real shadow stack is
    /// 8 MiB (`.cargo/config.toml` `-zstack-size`) and only optimized frames run there; the law measures DEBUG frames, whose
    /// async state machines are several times larger. Measured on this build: constructing the interaction fixture needs
    /// between 4 and 8 MiB, the select dispatch on top between 8 and 16 MiB, so 16 MiB is the smallest power-of-two bound
    /// that still fails on a runaway recursion or a re-inlined generator.
    const BOUNDED_THREAD_STACK_BYTES: usize = 16 * 1024 * 1024;

    /// 🧵️ Every world/graph pick is one framework-reserved `interactionSelect` dispatch, and it must fit
    /// a bounded thread stack. `dispatch_framework_reserved_action` used to spell all nineteen reserved
    /// routes as separate `run_framework_reserved_job` awaits inside ONE generator; an unoptimized build
    /// gives each such await its own non-overlapping slot, so that single resume frame reserved 1.80 MiB
    /// and every pick aborted the process with `fatal runtime error: stack overflow` on the 2 MiB stack a
    /// plain OS thread gets (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️selection-overflow-2026-09-09.md`). 2 MiB is exactly that default thread; the whole
    /// construct-select-close lifecycle measures ~1.75 MiB against it today, and measured ≥3.3 MiB
    /// before the split.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn one_framework_reserved_route_fits_a_bounded_thread_stack() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        std::thread::Builder::new()
            .stack_size(BOUNDED_THREAD_STACK_BYTES)
            .spawn(move || {
                let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original worker fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;

                let identity_started = std::time::Instant::now();
                let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
                    progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
                };
                let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

                semio_framework_async::block_on(async {
                    let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
                    reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
                    artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
                });
            
                let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original worker fixture paid terminal retirement");
            })
            .expect("bounded-stack worker")
            .join()
            .expect("one interactionSelect must fit the bounded thread stack");
    }

    /// 🧮️ The drive bound was a hand-picked 16 calls; the retirement needs 21 (three item releases,
    /// then the peer's own bytes one at a time). The count is a property of the fixture's roster, not
    /// of the product, so it is replaced by the stronger statement it was standing in for: EVERY
    /// bounded call must release something until the authority is terminal — a stall or a block now
    /// fails at the exact call it happened on instead of hiding under a larger number.
    #[semio_framework_async_macros::async_test]
    async fn peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        let alice = sample_presence_peer("user:alice#s1", Some(3), true);
        let bob = sample_presence_peer("user:bob#s1", Some(5), false);
        assert!(publish_presence_roster(&mut app, 1, Some(9), &[alice.clone(), bob], 1000,cx).await.fault.is_none());
        let (captured, typed_captured) = app.dispatch_ctx().test_capture_presence_roots(&TestCommand::Increment, &meta()).await.expect("actual shared typed capture helper");
        assert!(std::sync::Arc::ptr_eq(&captured, &app.roster.peer_presence), "operation capture clones one peer root Arc");

        assert!(publish_presence_roster(&mut app, 2, Some(9), &[alice], 2000,cx).await.fault.is_none());
        assert_eq!(captured.len(), 2, "captured roster remains revision-stable");
        assert_eq!(app.roster.peer_presence.len(), 1, "live roster advances independently");
        assert_maintenance_reports_block(&mut app, 5, "the shared old peer root",cx);
        app.maintenance.maintenance_stage = 6;
        let _ = app_maintenance_turn(&mut app,cx).expect("app-typed retirement selects its displaced entry");
        assert_maintenance_reports_block(&mut app, 6, "the captured app-typed peer root",cx);

        drop(captured);
        drop(typed_captured);
        for _ in 0..16 {
            if app.roster.peer_presence_retirements.is_empty() {
                break;
            }
            app.maintenance.maintenance_stage = 5;
            let _ = app_maintenance_turn(&mut app,cx).expect("bounded peer root retirement progresses");
        }
        assert!(app.roster.peer_presence_retirements.is_empty(), "old peer root reaches terminal-empty without a whole-roster drop");
        for calls in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
            if app.roster.presence_peer_retirements.is_empty() {
                break;
            }
            app.maintenance.maintenance_stage = 6;
            match app_maintenance_turn(&mut app,cx).expect("bounded app-typed peer retirement progresses") {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress == store::RetainedCloneProgress::default() => panic!("the app-typed peer retirement stalled after {calls} bounded calls"),
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { reason } => panic!("the app-typed peer retirement blocked after {calls} bounded calls: {reason}"),
                _ => {}
            }
        }
        assert!(app.roster.presence_peer_retirements.is_empty(), "displaced app-typed peer reaches its domain-owned terminal witness");
        drain_and_close_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎡️ One maintenance call now runs every stage that still has something to give, so the whole
    /// call's `released_items` is not this stage's answer. What the clause prices is unchanged and is
    /// asserted exactly: a grant one byte short of the retained page releases NO bytes, and the page
    /// stays mounted (next line).
    #[semio_framework_async_macros::async_test]
    async fn peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut saturated = contract_app_raw(mounted_policy, &mut identity).await;
        for seq in 0..ARTIFACT_LIVE_OUTPUT_SLOTS as u64 {
            let admission = saturated.reserve_presence_ingress(seq).expect("fixed roster slot admits before decode");
            let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
            saturated.admit_presence_ingress(admission, cursor, 0);
        }
        let saturation = match saturated.reserve_presence_ingress(ARTIFACT_LIVE_OUTPUT_SLOTS as u64) {
            Ok(_) => panic!("maximum plus one must fail before payload decode"),
            Err(fault) => fault,
        };
        assert_eq!(saturation.code.0, "interactive-job.peer-roster-saturated");
        for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS * 16 {
            saturated.maintenance_stage = 7;
            let _ = app_maintenance_turn(&mut saturated,cx).expect("one bounded saturation cleanup turn");
            if saturated.peer_roster_publications.is_empty() {
                break;
            }
        }
        assert!(saturated.peer_roster_publications.is_empty(), "every admitted publication reaches its exact outcome slot");
        for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            assert!(saturated.take_presence_outcome().is_some(), "every admitted roster has one ordered outcome");
        }

        drain_and_close_fixture(&mut saturated, mounted_policy,cx);

        let mut cancelled = contract_app_raw(mounted_policy, &mut identity).await;
        let admission = cancelled.reserve_presence_ingress(1).expect("cancel roster admission");
        let cancel = admission.cancel.clone();
        let page = FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
        let cursor = protocol::PresenceCommandCursor::admit_page(1, None, 1, page).map_err(|(error, _)| error).expect("cancel cursor");
        let generation = admission.generation();
        cancelled.admit_presence_ingress(admission, cursor, 0);
        cancel.cancel_now();
        cancelled.maintenance_stage = 7;
        let quote = cancelled.maintenance_retirement_demands(16).expect("quoted cancelled roster demand");
        let exact = fixture_close_grant(quote);
        let short = store::RetainedCloneGrant { maximum_copy_bytes: quote.copy_bytes.saturating_sub(1), maximum_capacity_bytes: quote.capacity_bytes.saturating_sub(1), maximum_release_bytes: quote.release_bytes.saturating_sub(1), ..exact };
        let interrupted = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,short,quote,|same,grant|Ok((cancelled.maintenance_step(same,grant)?,cancelled.maintenance_terminal_is_empty()))).expect("original sub-page close grant");
        assert!(matches!(interrupted, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress.released_bytes == 0), "interrupted close released bytes under a grant one short of its retained page: {interrupted:?}");
        assert!(cancelled.peer_roster_publications.get(generation).is_some(), "cancelled publication stays mounted until its retained page closes");
        for _ in 0..64 {
            cancelled.maintenance_stage = 7;
            let _ = app_maintenance_turn(&mut cancelled,cx).expect("bounded cancelled roster close");
            if cancelled.peer_roster_publications.is_empty() {
                break;
            }
        }
        let outcome = cancelled.take_presence_outcome().expect("cancelled roster outcome");
        assert_eq!(outcome.fault.expect("cancel is observable").code.0, "interactive-job.peer-roster-cancelled");
        assert!(cancelled.peer_presence.is_empty(), "cancelled roster never publishes metadata");
        assert_eq!(cancelled.presence_store.peers_root().len(), 0, "cancelled roster never publishes app-typed presence");

        drain_and_close_fixture(&mut cancelled, mounted_policy,cx);

        let mut stale = contract_app_raw(mounted_policy, &mut identity).await;
        let admission = stale.reserve_presence_ingress(9).expect("stale roster admission");
        let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
        stale.admit_presence_ingress(admission, cursor, 0);
        stale.peer_roster_processed_generation = 1;
        for _ in 0..64 {
            stale.maintenance_stage = 7;
            let _ = app_maintenance_turn(&mut stale,cx).expect("bounded stale roster close");
            if stale.peer_roster_publications.is_empty() {
                break;
            }
        }
        let outcome = stale.take_presence_outcome().expect("stale roster outcome");
        assert_eq!(outcome.fault.expect("stale generation is observable").code.0, "interactive-job.peer-roster-publication-authority");
        assert!(stale.peer_presence.is_empty(), "stale roster never changes the live peer root");
        assert_eq!(stale.presence_store.peers_root().len(), 0, "stale roster never changes the typed peer root");
        drain_and_close_fixture(&mut stale, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔐️ Adopts the exact member owner catalog a product child carries from
    /// `create_member_store`, for a law that first drove the refusal an owner-less member owes and
    /// then has to close the app it registered that member into.
    fn install_test_member_owners(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str) {
        let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), child_id.to_string())).expect("exact child owner").member;
        store::install_funded_member_store_owners(child,&semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer).expect("actual funded member catalog");
    }

    /// 🪆️ LAW (design §20.15, S4-LOAD wave 4): a COMPOSED document crosses a media edge whole — `produce_media("artifact:out")` carries the
    /// recursive archive (parent pack + `.spr` + members) as text that survives a UTF-8 host round trip, and the consumer's stepped load
    /// adopts the members: the child comes back as its own live store at the value its history ended on.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_document_crosses_a_media_edge_with_its_children() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut source = contract_composed_app_raw(mounted_policy, &mut identity).await;
        register_test_child(&mut source, "child-1").await;
        source.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut source, meta().instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("the composite gesture settles");
        let media_fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎞️media/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let media_grant=serde_json::from_value(media_fixture["grant"].clone()).unwrap();let mut media_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let original=identity.pause().expect("same original composed media admission receipt");assert_eq!(original.maximum_bytes(),media_fixture["originalNativeMaximumBytes"].as_u64().unwrap()as usize);let original=match original.with_retirement_recipient(&mut media_recipient){Ok(original)=>original,Err((error,_original))=>panic!("{error}")};let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume_retirement(original,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let mut artifact = identity.encode(|native| { let mut owner=semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner::new(native,media_grant); ::semio_framework_async::poll::resolve_ready(PluginApp::produce_media(&mut source, "artifact:out", &mut owner)) }).expect("the composed source hands out its whole document through its original authority");
        artifact.data = String::from_utf8(artifact.data).expect("a text-only host edge carries the carrier").into_bytes();

        let mut consumer = contract_composed_app_raw(mounted_policy, &mut identity).await;
        let semio_framework_plugin_decl_app::MediaConsumption::DocumentLoad(archive) = PluginApp::consume_media(&mut consumer, "artifact:in", artifact).await.expect("consumed") else {
            panic!("a whole document is answered as a document archive load");
        };
        assert_eq!(archive.members.iter().map(|entry| (entry.owner.slot.as_str(), entry.owner.child_id.as_str())).collect::<Vec<_>>(), vec![("slot", "child-1")]);
        PluginApp::begin_document_archive_load(&mut consumer, 9, archive).expect("admit");
        let status = loop {
            let status = PluginApp::poll_document_archive_load(&mut consumer, 9, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("poll");
            if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                break status;
            }
        };
        PluginApp::acknowledge_document_archive_load(&mut consumer, 9).expect("acknowledge");
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{:?}", semio_framework_diagnostic::decode_fault_bytes(&status.fault));
        let child = consumer.child_store("slot", "child-1").await.expect("the member came along");
        let restored: TestSnapshot = <TestSnapshot as ArtifactPack>::decode_pack(&child.document_pack_bytes().await.expect("child pack")).expect("decode child");
        assert_eq!(restored.count, 7, "the child arrives at the value its own history ended on");
        drain_and_close_composed_fixture(&mut consumer, mounted_policy,cx);
        drain_and_close_composed_fixture(&mut source, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎡️ The rotation is fair: one call runs every stage that still has something to give and
    /// reports a blocked stage only once nothing else can progress, so the block is the rotation's
    /// EVENTUAL answer rather than the answer of the one call that lands on stage 4. A lost
    /// authority never reports a block at all, so nothing here is weakened.
    #[semio_framework_async_macros::async_test]
    async fn child_snapshot_retirement_rejection_preserves_exact_erased_owner() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        declare_test_child(&mut app, "child-a").await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_bare_test_child("child-a").await.expect("construct child-a"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register child-a");
        let generation = app.child_composition.admit_child_content_publication().expect("admit replacement root");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        assert_maintenance_reports_block(&mut app, 4, "the rejected child snapshot transfer",cx);
        let retirement = app.child_composition.child_content_retirements.get(2).expect("retirement remains registered after rejected transfer");
        let entry = retirement.pending.as_ref().expect("exact rejected snapshot remains pending");
        assert!(entry.snapshot.typed::<TestSnapshot>().is_some(), "rejection preserves the exact erased owner and type identity");
        install_test_member_owners(&mut app, "child-a");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⚖️ LAW (design §20.13, L4): a config-lane edit is never a history row and undo never steps over it — the history
    /// lists the document row alone (every row, no `edit_id` filter), and `undo` retracts the newest DOCUMENT edit while the
    /// config the select wrote stays.
    #[semio_framework_async_macros::async_test]
    async fn a_config_lane_edit_is_never_a_history_row_and_undo_never_steps_over_it() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("node-1".into()) }, &meta(), &mut identity,cx).await.expect("select");
        assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "the select wrote the config lane");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.iter().map(|entry| entry.action_id.as_str()).collect::<Vec<_>>(), vec!["increment"], "only the document row is listed");
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 0, "undo retracts the document edit, never the config one");
        assert_eq!(app.stores.test_config().await.selected, Some("node-1".to_string()), "undo never steps over the config lane");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🫧️ The plugin cannot touch shell-owned state itself — it bubbles the inverse out as an effect
    /// instead of replaying anything locally, and does NOT append a new log entry on its own.
    #[semio_framework_async_macros::async_test]
    async fn shell_action_with_inverse_bubbles_a_replay_effect_instead_of_replaying_locally() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme", "inverseCommandId": "os.setThemeId", "inverseArgs": { "themeId": "light" } }))), &mut identity).await;

        let history = app.render_ctx().test_history().await;
        let entry = history.commands.first().expect("one logged shell row");
        assert_eq!(entry.kind, ActionKind::Shell);
        assert!(entry.revertible, "a Shell row with a stored inverse must be revertible");
        let seq = entry.seq;

        let result = reserved_action(&mut app, REVERT_TO_COMMAND_ACTION_ID, Some(&dv(json!({ "entrySeq": seq }))), &mut identity).await;

        assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: Some(semio_framework::dsl_value!({ "themeId": "light" })) }]);
        assert_eq!(app.render_ctx().test_history().await.commands.len(), history.commands.len(), "bubbling the effect logs nothing new by itself");
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_emits_clipboard_write_effect_with_no_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        let result = reserved_action(&mut app, "copy", None, &mut identity).await;
        assert!(result.mutations.is_empty(), "copy must not record an undo entry");
        assert_eq!(result.requested_effects.len(), 1);
        let Effect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        assert_eq!(fragment.dsl_text, "hello");
        assert_eq!(fragment.source_app, TestApp::<false>::APP_ID);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_on_empty_selection_is_a_benign_no_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, "copy", None, &mut identity).await;
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ✂️ One undo restores the cut label — cut is a single coalesced edit, not two.
    #[semio_framework_async_macros::async_test]
    async fn cut_removes_label_and_emits_clipboard_write_as_one_undo_unit() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "hello".into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        let result = reserved_action(&mut app, "cut", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");
        assert_eq!(result.requested_effects.len(), 1);
        assert!(matches!(&result.requested_effects[0], Effect::ClipboardWrite { fragment } if fragment.dsl_text == "hello"));
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "hello");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_materializes_fragment_at_original_anchor() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let fragment = ClipboardFragment {
            schema: "semio.test/v1".into(),
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            dsl_text: "pasted".into(),
            pack_bytes: None,
            source_app: TestApp::<false>::APP_ID.into(),
            label: "pasted".into(),
        };
        let args = dv(json!({ "fragment": fragment, "anchor": "original" }));
        reserved_action(&mut app, "paste", Some(&args), &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "pasted");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_with_non_original_anchor_reaches_the_app_placement() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let fragment = ClipboardFragment {
            schema: "semio.test/v1".into(),
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            dsl_text: "pasted".into(),
            pack_bytes: None,
            source_app: TestApp::<false>::APP_ID.into(),
            label: "pasted".into(),
        };
        let args = dv(json!({ "fragment": fragment, "anchor": "centroid" }));
        reserved_action(&mut app, "paste", Some(&args), &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, format!("pasted-{:?}", PasteAnchor::Centroid));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, "paste", None, &mut identity).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.stores.test_snapshot().await.label, "");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🪡️ A streamed tool transaction is ONE edit (design §15): every tick joins the open edit, the commit tick closes it,
    /// and one undo reverts the whole gesture back to the empty label.
    #[semio_framework_async_macros::async_test]
    async fn streamed_ticks_and_their_commit_are_one_undo_step() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for value in ["a", "ab"] {
            app.dispatch_typed(TestCommand::StreamLabel { value: value.into(), commit: false }, &meta(), &mut identity,cx).await.expect("streamLabel tick");
        }
        app.dispatch_typed(TestCommand::StreamLabel { value: "abc".into(), commit: true }, &meta(), &mut identity,cx).await.expect("streamLabel commit");
        assert_eq!(app.stores.test_snapshot().await.label, "abc");
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧱️ Outside a tool transaction every emission is its own plain edit (design §20.1): nothing coalesces, so one undo
    /// reverts exactly the last dispatch.
    #[semio_framework_async_macros::async_test]
    async fn plain_emissions_never_coalesce() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for value in ["a", "ab", "abc"] {
            app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        }
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "ab");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn history_actions_round_trip_through_the_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc1");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc2");
        assert_eq!(app.stores.test_snapshot().await.count, 2);

        let undo = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert!(undo.mutations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 1);

        reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 2);

        let checkpoint = reserved_action(&mut app, "commitCheckpoint", None, &mut identity).await;
        assert!(checkpoint.mutations.is_empty());
        assert!(checkpoint.events.iter().any(|event| event.kind == "history-changed"));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_invocation_does_not_require_window_ownership() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{ActionAddress, ActionInvocation};
        let mut app = contract_app(mounted_policy, &mut identity).await;
        let invocation = ActionInvocation {
            address: ActionAddress {
                plugin_id: "test".into(),
                app_id: TestApp::<false>::APP_ID.into(),
                mode_id: "edit".into(),
                window_kind_id: "window-without-undo".into(),
                window_instance_id: "main-instance".into(),
                action_id: "undo".into(),
            },
            arguments: Default::default(),
        };
        let mut args=Some(DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;let admitted=artifact_app_laws::dispatch_addressed_fixture_on_original(&mut app.0,&invocation.address,&mut args,&mut fault,Some("edit"),&meta(),&mut identity,cx).await.expect("reserved undo without window ownership");
        let result = settle_reserved(&mut app, admitted).await;
        assert!(result.mutations.is_empty());
        artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.stores.test_store_mut()
            .await
            .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())
            .await
            .expect("seed document edit");
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit undo");
        assert!(admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)));
        assert!(admitted.events.iter().all(|event| event.kind != "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 1, "first turn answers without committing");
        let settled = settle_reserved(&mut app, admitted).await;
        assert!(settled.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.stores.test_snapshot().await.count, 0, "driving the spawned job must run commit_framework_history_route");
        artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    //#region 🔖️CommandLogTests
    /// 🏷️ `increment` IS declared by `migrated_contract_registry` (the fixture every law here now uses),
    /// so the row carries its DECLARED label — still `LocalizedLabel::data`, so still locale-invariant
    /// rather than untranslated English, which is the property this clause exists for.
    #[semio_framework_async_macros::async_test]
    async fn an_operation_action_appends_one_command_log_entry_linked_to_its_edit() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "increment");
        assert_eq!(label_in(&entry.label, Locale::En), "Increment");
        assert_eq!(label_in(&entry.label, Locale::De), "Increment", "a declared label is locale-invariant data, never untranslated English");
        assert_eq!(entry.kind, ActionKind::Mutation);
        assert!(entry.edit_id.is_some());
        assert!(!entry.op_lines.is_empty(), "operation entry must carry printed op-text");
        assert!(entry.applied && entry.revertible);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_streamed_gesture_appends_exactly_one_command_log_entry() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        for (value, commit) in [("a", false), ("ab", false), ("abc", true)] {
            app.dispatch_typed(TestCommand::StreamLabel { value: value.into(), commit }, &meta(), &mut identity,cx).await.expect("streamLabel");
        }
        let history = app.render_ctx().test_history().await;
        let stream_entries: Vec<&CommandView> = history.commands.iter().filter(|entry| entry.action_id == "streamLabel").collect();
        assert_eq!(stream_entries.len(), 1, "a streamed gesture must grow one entry's op_lines, not append new entries");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📜️ LAW (ticket 26/09/23 C12): a streamed gesture's history row previews only its edit's newest
    /// `HISTORY_ROW_OPERATION_PREVIEW` operations — the newest one last — however long the gesture runs, so reading the history
    /// costs the same per tick at the end of a long gesture as at its start.
    #[semio_framework_async_macros::async_test]
    async fn a_long_streamed_gesture_previews_its_newest_operations_only() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let mut value = String::new();
        for key in 0..64u8 {
            value.push(char::from(b'a' + key % 26));
            app.dispatch_typed(TestCommand::StreamLabel { value: value.clone(), commit: key == 63 }, &meta(), &mut identity,cx).await.expect("streamLabel");
            let _ = app.render_ctx().test_history().await;
        }
        let history = app.render_ctx().test_history().await;
        let row = history.commands.iter().find(|entry| entry.action_id == "streamLabel").expect("the gesture's row");
        assert_eq!(row.op_count, 64, "the row counts every operation of its edit");
        assert_eq!(row.op_lines.len(), HISTORY_ROW_OPERATION_PREVIEW, "the row previews a bounded tail of its operations");
        assert!(row.op_lines.last().is_some_and(|line| line.contains(&value)), "the newest operation closes the preview: {:?}", row.op_lines);
        let exported = app.history_snapshot().await.expect("history snapshot");
        let exported = exported.upserts.iter().find(|entry| entry.action_id == "streamLabel").expect("the gesture's exported row");
        assert_eq!((exported.op_count, exported.op_lines.len()), (64, HISTORY_ROW_OPERATION_PREVIEW), "the exported row carries the bounded preview and its edit's operation count");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_and_redo_append_entries_and_never_shrink_the_log() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        assert_eq!(app.render_ctx().test_history().await.commands.len(), 1);
        reserved_action(&mut app, "undo", None, &mut identity).await;
        let after_undo = app.render_ctx().test_history().await;
        assert_eq!(after_undo.commands.len(), 2, "undo appends, it does not remove the increment entry");
        assert!(after_undo.commands.iter().any(|entry| entry.action_id == "increment"));
        reserved_action(&mut app, "redo", None, &mut identity).await;
        let after_redo = app.render_ctx().test_history().await;
        assert_eq!(after_redo.commands.len(), 3, "redo appends a third entry");
        assert!(after_redo.commands.iter().any(|entry| entry.action_id == "undo"));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ The receiver never dispatched anything itself — any log entry it has must come from backfill.
    #[semio_framework_async_macros::async_test]
    async fn ingested_remote_edits_are_backfilled_into_the_command_log() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut sender = contract_app(mounted_policy, &mut identity).await;
        let (near, mut far) = MemoryBackbone::pair("mem://doc-history-backfill", "mem://doc-history-backfill").await;
        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
        sender.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = contract_app(mounted_policy, &mut identity).await;
        receiver.ingest_operations(&operations, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest");
        let history = receiver.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        assert!(history.commands[0].edit_id.is_some());
        assert!(!history.commands[0].op_lines.is_empty());
    
        artifact_app_laws::close_registered_fixture_app(&mut receiver.0,receiver.1,cx).expect("original contract fixture paid terminal close");
    
        artifact_app_laws::close_registered_fixture_app(&mut sender.0,sender.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn set_history_command_filter_emits_no_operations_and_updates_the_view() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&dv(json!({ "value": "onlyMutations" }))), &mut identity).await;
        assert!(result.mutations.is_empty());
        assert_eq!(app.render_ctx().test_history().await.command_filter, HistoryCommandFilter::OnlyMutations);
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn interleaved_config_only_views_leave_only_the_document_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Select { id: Some("a".into()) }, &meta(), &mut identity,cx).await.expect("select a");
        app.dispatch_typed(TestCommand::Select { id: Some("b".into()) }, &meta(), &mut identity,cx).await.expect("select b");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        app.dispatch_typed(TestCommand::Select { id: Some("c".into()) }, &meta(), &mut identity,cx).await.expect("select c");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.iter().map(|entry| (entry.action_id.as_str(), entry.count)).collect::<Vec<_>>(), vec![("increment", 1)]);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🌐️ A shell-noted row's text is the shell's own already-rendered copy, so the row stores it as
    /// locale-INVARIANT data (`LocalizedLabel::data`) — every cell of the matrix carries it. The row
    /// therefore never claims to be English, and the History panel renders it unchanged in a German
    /// shell instead of leaking a locale it does not have.
    #[semio_framework_async_macros::async_test]
    async fn a_shell_noted_row_carries_the_same_text_in_every_locale() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme" }))), &mut identity).await;
        let history = app.render_ctx().test_history().await;
        let label = &history.commands[0].label;
        for terminology in Terminology::ALL {
            for locale in Locale::ALL {
                assert_eq!(label.resolve(terminology, locale), "Set Theme", "{terminology:?}/{locale:?}");
            }
        }
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🙈️ A `View` verb that edited no store and carries no inverse is no history row, while a mutation next to it is.
    #[semio_framework_async_macros::async_test]
    async fn a_pure_view_verb_is_no_history_row() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::ViewNoScope, &meta(), &mut identity,cx).await.expect("viewNoScope");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");
        app.dispatch_typed(TestCommand::ViewPartialScope, &meta(), &mut identity,cx).await.expect("viewPartialScope");
        let history = app.render_ctx().test_history().await;
        let rows: Vec<&str> = history.commands.iter().map(|entry| entry.action_id.as_str()).collect();
        assert_eq!(rows, vec!["increment"], "pure view verbs never become rows");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let before_len = app.render_ctx().test_history().await.commands.len();
        let result = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(result.ui_scope, UiDirtyScope::None, "nothing was logged, so the scope must not be upgraded either");
        assert_eq!(app.render_ctx().test_history().await.commands.len(), before_len);
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn set_history_command_filter_is_never_logged() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let before_len = app.render_ctx().test_history().await.commands.len();
        reserved_action(&mut app, SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&dv(json!({ "value": "onlyMutations" }))), &mut identity).await;
        assert_eq!(app.render_ctx().test_history().await.commands.len(), before_len, "the filter's own chrome must not fill the list it filters");
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn rendering_the_history_body_reflects_a_log_only_change_with_no_store_generation_bump() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render before");
        reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(json!({ "commandId": "os.setThemeId", "label": "Set Theme" }))), &mut identity).await;
        let rendered = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render after");
        assert_eq!(rendered.root.children.len(), 2, "Actions + Commands");
        assert_eq!(rendered.root.children[1].children.len(), 1, "a log-only cache key change (no store generation bump) must still refresh the rendered panel");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn an_operation_kind_action_with_zero_operations_still_logs_one_entry() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::NoopMutation, &meta(), &mut identity,cx).await.expect("noopMutation");
        let history = app.render_ctx().test_history().await;
        assert_eq!(history.commands.len(), 1);
        let entry = &history.commands[0];
        assert_eq!(entry.action_id, "noopMutation");
        assert_eq!(entry.kind, ActionKind::Mutation);
        assert!(entry.edit_id.is_none(), "no operations means no VCS edit, even though the action is Mutation-kind");
        assert_eq!(entry.count, 1);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    //#endregion 🔖️CommandLogTests

    #[semio_framework_async_macros::async_test]
    async fn undo_on_empty_history_is_a_benign_no_operation() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let result = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert!(result.mutations.is_empty());
        assert!(result.events.is_empty());
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn document_round_trips_through_serialization() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("inc");
        app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta(), &mut identity,cx).await.expect("label");
        let files = app.document_pack().await.expect("document pack");

        let mut restored = contract_app(mounted_policy, &mut identity).await;
        artifact_app_laws::load_document(&mut restored, &files, &mut identity).await.expect("load document pack");
        assert_eq!(restored.test_snapshot().await, TestSnapshot { count: 1, label: "hi".into(), slot: Vec::new() });
    
        artifact_app_laws::close_registered_fixture_app(&mut restored.0,restored.1,cx).expect("original contract fixture paid terminal close");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut sender = contract_app(mounted_policy, &mut identity).await;
        let (near, mut far) = MemoryBackbone::pair("mem://doc", "mem://doc").await;
        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
        sender.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = contract_app(mounted_policy, &mut identity).await;
        receiver.ingest_operations(&operations, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest once");
        receiver.ingest_operations(&operations, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest twice");
        assert_eq!(receiver.test_snapshot().await.count, 1, "feeding the same operation twice must not double-apply");
    
        artifact_app_laws::close_registered_fixture_app(&mut receiver.0,receiver.1,cx).expect("original contract fixture paid terminal close");
    
        artifact_app_laws::close_registered_fixture_app(&mut sender.0,sender.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn attach_detach_reattach_resumes_backbone_convergence() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        assert!(app.backbone_ref().is_none(), "default is unattached");

        let (near, mut far) = MemoryBackbone::pair("mem://reattach", "mem://reattach").await;
        app.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment while attached");
        assert!(!far.receive().await.expect("receive after attach").is_empty(), "attached edits reach the peer");

        app.detach_backbone().await.expect("detach");
        assert!(app.backbone_ref().is_none());
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment while detached");
        assert_eq!(app.stores.test_snapshot().await.count, 2, "detached edits still land on the in-memory graph");
        assert!(far.receive().await.expect("receive while detached").is_empty(), "detached edits never reach the peer");

        let (near_again, mut far_again) = MemoryBackbone::pair("mem://reattach-2", "mem://reattach-2").await;
        app.attach_backbone(store::os_store::backbone::Backbones::Memory(near_again)).await.expect("re-attach");
        assert!(app.backbone_ref().is_some());
        app.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("increment after re-attach");
        assert_eq!(app.stores.test_snapshot().await.count, 3);
        assert!(!far_again.receive().await.expect("receive after re-attach").is_empty(), "re-attaching resumes outbound convergence on the new backbone");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🖱️ `PluginApp::context_menu` end-to-end through `VcsArtifactApp`: with an empty label the
    /// "selection guard" (`Menu::when`) drops the gated command; once a label is set (a stand-in for
    /// "something is selected"), both rows resolve label/icon from the declared registry entries.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_resolves_labels_from_the_registry_and_respects_guards() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

        use semio_framework::default_action_icon_id;

        let set_label_icon = default_action_icon_id(ActionKind::Mutation).as_str().to_string();
        let increment_icon = default_action_icon_id(ActionKind::Mutation).as_str().to_string();

        let empty_label = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
        assert_eq!(empty_label.len(), 1, "the gated command must be absent with no label set: {empty_label:?}");
        assert_eq!(empty_label[0], ContextMenuItemSpec { id: "setLabelRequired".into(), label: Some("Set Label".into()), icon: Some(set_label_icon), action: Some("setLabelRequired".into()), ..Default::default() });

        app.dispatch_typed(TestCommand::SetLabel { value: "hi".into() }, &meta(), &mut identity,cx).await.expect("set label");
        let with_label = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
        assert_eq!(with_label.len(), 2, "the guard must open once a label is set: {with_label:?}");
        assert_eq!(with_label[1], ContextMenuItemSpec { id: "incrementViaCommand".into(), label: Some("Increment".into()), icon: Some(increment_icon), action: Some("incrementViaCommand".into()), ..Default::default() });
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪️ A registry declaring `setLabelRequired` plus ten `flatLeaf1..10` actions (four carrying a
    /// `RIBBON_PARENT_CATEGORIES` category via `with_category`) — feeds `TestApp::context_menu`'s
    /// `"flat-menu-test"` branch below.
    async fn flat_menu_registry() -> AppActionRegistry {
        let builder = App::builder(test_app_surface_id().await, LocalizedLabel::data("FlatMenuTest"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), "flat-menu-test.main", SurfaceKind::Canvas2d, IconName::AppWindow)
                .await
                .mutation("setLabelRequired", LocalizedLabel::data("Set Label"))
                .await
                .action_args("setLabelRequired", vec![ActionArgDef::text("value", LocalizedLabel::data("Value")).required()])
                .await
                .mutation("flatLeaf1", LocalizedLabel::data("Flat Leaf 1"))
                .await
                .mutation("flatLeaf2", LocalizedLabel::data("Flat Leaf 2"))
                .await
                .mutation("flatLeaf3", LocalizedLabel::data("Flat Leaf 3"))
                .await
                .mutation("flatLeaf4", LocalizedLabel::data("Flat Leaf 4"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf5", LocalizedLabel::data("Flat Leaf 5"), ActionKind::Mutation).with_category("view"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf6", LocalizedLabel::data("Flat Leaf 6"), ActionKind::Mutation).with_category("view"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf7", LocalizedLabel::data("Flat Leaf 7"), ActionKind::Mutation).with_category("export"))
                .await
                .action_with(ActionDefinition::bounded_catalog("flatLeaf8", LocalizedLabel::data("Flat Leaf 8"), ActionKind::Mutation).with_category("export"))
                .await
                .mutation("flatLeaf9", LocalizedLabel::data("Flat Leaf 9"))
                .await
                .mutation("flatLeaf10", LocalizedLabel::data("Flat Leaf 10"))
                .await;
        let app = App::from_builder(declare_test_app_verbs(builder).interactive_jobs(InteractiveJobClassification::Migrated).await).await;
        AppActionRegistry::from_definition(&app.definition)
    }

    /// 🖱️ End to end through `VcsArtifactApp::context_menu`: a synthetic emitter's 11 flat leaves come
    /// back as 5 primaries + 3 taxonomy-sorted `menu.group.<category>` rows — proving the funnel applies
    /// `organize_context_menu` to every emitter, not just ones that call `Menu::group` themselves.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_with(flat_menu_registry().await, mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "flat-menu-test".into() }, &meta(), &mut identity,cx).await.expect("set label");
        let request = ContextMenuRequest { menu: UiMenuRef { id: "window".into(), args: None }, surface: None, window_instance_id: None, point: None };

        let organized = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
        let ids: Vec<&str> = organized.iter().map(|item| item.id.as_str()).collect();
        assert_eq!(
            ids,
            vec!["setLabelRequired", "flatLeaf1", "flatLeaf2", "flatLeaf3", "flatLeaf4", "menu.group.view", "menu.group.actions", "menu.group.export"],
            "5 primaries, then groups in RIBBON_PARENT_CATEGORIES taxonomy order (view < actions < export): {ids:?}"
        );
        let view_children: Vec<&str> = organized[5].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(view_children, vec!["flatLeaf5", "flatLeaf6"]);
        let actions_children: Vec<&str> = organized[6].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(actions_children, vec!["flatLeaf9", "flatLeaf10"], "uncategorized overflow leaves default to menu.group.actions");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_carries_its_value_directly_and_emits_no_operations() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::SetActiveUtility { utility_id: "brush".into() }, &meta(), &mut identity,cx).await.expect("setActiveUtility is a valid View command");
        assert!(result.mutations.is_empty(), "utility switching must not create history");
        let event = result.events.iter().find(|event| event.kind == "active-utility").expect("echoed active utility");
        assert_eq!(event.payload, semio_framework_value::DslValue::from(json!({ "utilityId": "brush" })));
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧷️ A streamed gesture is one undo step on the typed route too, while each plain label edit is its own edit: one undo
    /// only reverts the last one.
    #[semio_framework_async_macros::async_test]
    async fn a_streamed_gesture_is_one_undo_step_while_each_commit_is_its_own() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        for (value, commit) in [("a", false), ("ab", false), ("abc", true)] {
            app.dispatch_typed(TestCommand::StreamLabel { value: value.into(), commit }, &meta(), &mut identity,cx).await.expect("streamLabel");
        }
        assert_eq!(app.stores.test_snapshot().await.label, "abc");
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "");

        for value in ["x", "xy"] {
            app.dispatch_typed(TestCommand::SetLabel { value: value.into() }, &meta(), &mut identity,cx).await.expect("setLabel");
        }
        assert_eq!(app.stores.test_snapshot().await.label, "xy");
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "x");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn operation_command_emits_kernel_op_with_true_inverse() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let result = app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity,cx).await.expect("incrementViaCommand");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(result.mutations[0].diff.payload, ::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 1 })).unwrap());
        assert_eq!(result.mutations[0].inverse.inverse_diff.payload, protocol::encode_ops_vec(&[::protocol::OpBinary::encode_op(&TestMutation::SetCount(SetCount { value: 0 })).unwrap()]));
        assert_eq!(app.stores.test_snapshot().await.count, 1);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_command_dispatch_validates_structural_app_ownership() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let mut valid = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into() }, command_id: "incrementViaCommand".into() }, arguments: Default::default() };
        app.handle_command(&mut valid, None, Some("edit"), &meta(), &mut identity).await.expect("app-owned command");
        settle_contract_app(&mut app,&mut identity,cx).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let mut unknown = CommandInvocation { address: CommandAddress { command_id: "nope".into(), ..valid.address }, arguments: Default::default() };
        let error = app.handle_command(&mut unknown, None, Some("edit"), &meta(), &mut identity).await.expect_err("undeclared app command");
        assert!(error.message.contains("not owned by app"), "unexpected error: {}", error.message);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_mode_command_requires_the_active_structural_owner() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        let mut invocation = CommandInvocation {
            address: CommandAddress { owner: CommandOwnerAddress::Mode { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into(), mode_id: "edit".into() }, command_id: "mode.increment".into() },
            arguments: Default::default(),
        };
        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.handle_command(&mut invocation, None, Some("edit"), &meta(), &mut identity).await.expect("active mode-owned command");
        settle_contract_app(&mut app,&mut identity,cx).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);
        let error = app.handle_command(&mut invocation, None, None, &meta(), &mut identity).await.expect_err("inactive mode command");
        assert!(error.message.contains("not owned by active mode edit"), "unexpected error: {}", error.message);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::manifest::{ActionAddress, ActionInvocation};
        let invocation = ActionInvocation {
            address: ActionAddress { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into(), mode_id: "edit".into(), window_kind_id: "main".into(), window_instance_id: "main-instance-2".into(), action_id: "targetWindow".into() },
            arguments: std::collections::BTreeMap::from([("windowId".into(),DslValue::String("main-instance-2".into()))]),
        };
        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let mut args=Some(DslValue::Object(invocation.arguments.into_iter().collect()));let mut fault=None;artifact_app_laws::dispatch_addressed_fixture_on_original(&mut app.0,&invocation.address,&mut args,&mut fault,Some("edit"),&meta(),&mut identity,cx).await.expect("addressed window action");
        settle_contract_app(&mut app,&mut identity,cx).await;
        assert_eq!(app.stores.test_snapshot().await.label, "main-instance-2");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn command_op_records_history_exactly_like_an_operation_action() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity,cx).await.expect("inc");
        app.dispatch_typed(TestCommand::IncrementViaCommand, &meta(), &mut identity,cx).await.expect("inc");
        assert_eq!(app.stores.test_snapshot().await.count, 2);
        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.count, 1);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🕹️ `interaction_topology` requires a non-empty `label` for "item-1" to exist.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_replace_persists_through_the_interaction_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        let selection = app.interaction_state().await.selection.get("items").cloned().expect("items domain selected");
        assert_eq!(selection.ids, vec!["item-1".to_string()]);
        assert_eq!(selection.granularity, "item");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🕰️ The framework-injected "undo" action only ever dispatches against `self.store` (the
    /// DOCUMENT store) — with only the label-seed edit on it, one undo reverts THAT, not the pick.
    #[semio_framework_async_macros::async_test]
    async fn a_pick_is_never_undoable_the_default_undo_only_ever_walks_the_document_store() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]));

        reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(app.stores.test_snapshot().await.label, "", "undo must revert the document edit (seeding the label)");
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "the pick itself must survive an unrelated document undo — lane discipline");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🛂️ An undeclared granularity is rejected, not silently accepted — `handle_action` only ADMITS
    /// a framework-reserved verb (`FrameworkSetInteractionGranularityJob`), so the refusal rides the
    /// reserved job's commit, exactly where `reserved_action` above drives the accepted ones.
    #[semio_framework_async_macros::async_test]
    async fn set_selection_mode_and_set_interaction_granularity_persist_immediately() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        reserved_action(&mut app, SET_SELECTION_MODE_ACTION_ID, Some(&dv(json!({ "domainId": "items", "mode": "single" }))), &mut identity).await;
        assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(SelectionMode::Single));

        reserved_action(&mut app, SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&dv(json!({ "domainId": "items", "granularityId": "item" }))), &mut identity).await;
        assert_eq!(app.interaction_state().await.active_granularity.get("items").map(String::as_str), Some("item"));

        let admitted = app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(dv(json!({ "domainId": "items", "granularityId": "bogus" }))), &meta(), &mut identity).await.expect("undeclared granularity admission");
        let error = semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut *app, admitted).await.expect_err("undeclared granularity must be rejected");
        assert!(error.message.contains("bogus"), "unexpected error: {}", error.message);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn clear_selection_and_select_all_apply_across_every_declared_domain() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");

        reserved_action(&mut app, SELECT_ALL_ACTION_ID, None, &mut identity).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]), "selectAll must select every id `interaction_topology` reports for the declared granularity");

        reserved_action(&mut app, CLEAR_SELECTION_ACTION_ID, None, &mut identity).await;
        assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "clearSelection must empty every declared domain's selection");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧹️ `TestApp::interaction_topology` reports NO ids once `label` is empty again — simulates
    /// "item-1 was deleted from the document" — task 4: revalidated after EVERY artifact dispatch.
    #[semio_framework_async_macros::async_test]
    async fn validate_state_prunes_a_stale_selection_id_after_the_document_deletes_it() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        assert_eq!(app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone()), Some(vec!["item-1".to_string()]));

        app.dispatch_typed(TestCommand::SetLabel { value: "".into() }, &meta(), &mut identity,cx).await.expect("delete item-1 (empty label)");

        assert!(app.interaction_state().await.selection.get("items").is_none_or(|selection| selection.ids.is_empty()), "the deleted id must be pruned from selection automatically");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🙈️ Interaction verbs (pick, hover, clear selection) change no history, so they are never history rows:
    /// only the seeding edit stays in the log.
    #[semio_framework_async_macros::async_test]
    async fn interaction_verbs_never_become_history_rows() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        reserved_action(&mut app, CLEAR_SELECTION_ACTION_ID, None, &mut identity).await;
        let history = app.render_ctx().test_history().await;
        let kinds: Vec<(&str, ActionKind)> = history.commands.iter().map(|entry| (entry.action_id.as_str(), entry.kind)).collect();
        assert_eq!(kinds, vec![("setLabel", ActionKind::Mutation)], "only the seeding edit is a row");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 W-G3 §8.30 — leftover `interactionSelect` of an object id on a vortex-domain-shaped pick still lands in `selectedIds`.
    #[semio_framework_async_macros::async_test]
    async fn leftover_interaction_select_object_id_on_vortex_domain_lands_in_selected_ids() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(
            &mut app,
            INTERACTION_SELECT_ACTION_ID,
            Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "seed-left-001")),
         &mut identity)
        .await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let ids = view.get("selectedIds").and_then(semio_framework_value::DslValue::as_array).expect("selectedIds");
        assert!(ids.iter().any(|id| id.as_str() == Some("seed-left-001")), "leftover selected ids {ids:?}");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 W-G3 §8.21 — `interactionHover` leftover publishes the hover target on the same leftover output.
    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_job_completion_publishes_hover_target_on_leftover() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        let view = settled.output.get("interactionView").expect("leftover InteractionView");
        let hover = view.get("hoverTarget").expect("hover target");
        assert_eq!(hover.get("id").and_then(semio_framework_value::DslValue::as_str), Some("item-1"));
        assert_eq!(hover.get("domain").and_then(semio_framework_value::DslValue::as_str), Some("items"));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🐁️ Wave B34 — the plugin-host half of the app-declared interaction scope: an `interactionHover`
    /// dirties ONLY the hover lane of its own window. The framework used to answer `UiDirtyScope::Full`
    /// for all six `INTERACTION_ACTION_IDS`, hover included, so every pointer move repainted every
    /// window body, every panel body, the utilities/tools/engagements rails, the labels and the measures
    /// — measured at 24 interaction ingresses and 3 whole-shell completions per 80 s browser lane
    /// (ticket 26/09/02 `📓️2026-09-12-wave-B32-world-lane-after-completion.md` §3.2).
    #[semio_framework_async_macros::async_test]
    async fn interaction_hover_dirties_only_the_hovering_windows_own_lane() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &mut identity).await;
        let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = settled.ui_scope else {
            panic!("a declared hover scope is never Full or None: {:?}", settled.ui_scope);
        };
        assert_eq!(window_bodies, vec![TEST_APP_WINDOW_BODY_KEY.to_string()], "a hover dirties the hovering window's own body and no other");
        assert!(panel_bodies.is_empty(), "a hover cannot dirty a panel: {panel_bodies:?}");
        assert!(!utilities && !tools && !engagements && !measures && !labels, "a hover cannot dirty the shell chrome");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🖱️ Wave B34 — the same hop for a PICK: the declaring app's answer is carried through verbatim,
    /// panel bodies included, so an app can widen a select beyond a hover without widening the hover.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_carries_the_apps_declared_selection_lane() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        assert_eq!(
            settled.ui_scope,
            UiDirtyScope::Partial {
                window_bodies: vec![TEST_APP_WINDOW_BODY_KEY.to_string()],
                panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()],
                utilities: false,
                tools: false,
                engagements: false,
                measures: false,
                labels: false,
            },
            "a pick publishes the app's declared selection scope verbatim"
        );
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🈳️ The framework DEFAULT: a verb the app declares no scope for falls through to the scope
    /// DERIVED from the app's own surface declarations (`interaction_declared_refresh_scope` over
    /// `WindowKindDefinition.interactions`) — the window that accepts the domain, the Select chrome,
    /// and no panel, because `setInteractionGranularity` moves no selection. `TestApp` declares
    /// nothing for this verb precisely so this arm has a fixture, and `UiDirtyScope::Full` is now
    /// reached only when not even a window declares the touched domain (the sibling law below).
    #[semio_framework_async_macros::async_test]
    async fn an_undeclared_interaction_verb_falls_through_to_the_declared_surface_scope() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let settled = reserved_action(&mut app, SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&dv(json!({ "domainId": "items", "granularityId": "item" }))), &mut identity).await;
        assert_eq!(
            settled.ui_scope,
            UiDirtyScope::Partial { window_bodies: vec![TEST_APP_WINDOW_BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: true, labels: false },
            "an app that declares the domain on a window narrows to that window without declaring a scope hook"
        );
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ A folded verb that cannot be applied (an undeclared domain here) must not fail the app
    /// command that carried it — the pick already happened — so it lands as one diagnostic on the
    /// carrier's result under `INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE`, the effect is still
    /// consumed (the host is never asked to retry a verb the reactor refused), and the selection is
    /// untouched. A `Full` carrier scope stays `Full`.
    #[semio_framework_async_macros::async_test]
    async fn guest_emitted_interaction_select_on_an_undeclared_domain_is_a_diagnostic_not_a_fault() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let bad = Emit { effects: vec![inline_interaction_select(interaction_target_args(json!({ "domainId": "ghosts", "merge": "replace", "method": "pick" }), "item-1"))], ui_scope: UiDirtyScope::Full, ..Default::default() };
        let result = app.test_dispatch_emit("canvasPointerUp", bad, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("a refused inline verb never fails its carrier");
        assert!(result.requested_effects.is_empty(), "the refused verb is consumed, not bounced to the host: {:?}", result.requested_effects);
        assert_eq!(result.diagnostics.len(), 1, "{:?}", result.diagnostics);
        assert_eq!(result.diagnostics[0].code.0, semio_framework_plugin_interaction::interaction_topology::INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE);
        assert!(result.diagnostics[0].message.contains("ghosts"), "the diagnostic names the refused domain: {}", result.diagnostics[0].message);
        assert_eq!(result.ui_scope, UiDirtyScope::Full);
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.values().all(|selection| selection.ids.is_empty()), "a refused verb moves nothing: {:?}", snapshot.selection);
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ The fold is scoped to the six interaction verbs: an undo/redo inverse replay or an `os.*`
    /// shell relay riding the same `ReplayShellCommand` variant keeps reaching the host untouched.
    #[semio_framework_async_macros::async_test]
    async fn non_interaction_replay_shell_commands_still_reach_the_host() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let relay = Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: Some(semio_framework_value::DslValue::from(&json!({ "themeId": "light" }))) };
        let result = app.test_dispatch_emit("relayTheme", Emit { effects: vec![relay.clone()], ui_scope: UiDirtyScope::None, ..Default::default() }, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("relay lands");
        assert_eq!(result.requested_effects, vec![relay]);
        assert_eq!(result.ui_scope, UiDirtyScope::None, "no verb was folded, so no scope was widened");
        assert!(result.diagnostics.is_empty());
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ The `DispatchAction` fold is scoped to the six interaction verbs exactly like the
    /// `ReplayShellCommand` one: a self-dispatch chain (`probeChildContinuation`, a delayed
    /// staged-work tick) riding the same variant keeps reaching the host untouched, `req` and
    /// `delay_ms` intact.
    #[semio_framework_async_macros::async_test]
    async fn non_interaction_dispatch_actions_still_reach_the_host() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let chain = Effect::DispatchAction { req: RequestId(91_002), action: "probeChildContinuation".into(), args: Some(semio_framework_value::DslValue::from(&json!({ "pass": 2 }))), delay_ms: 16 };
        let result = app.test_dispatch_emit("stagedPass", Emit { effects: vec![chain.clone()], ui_scope: UiDirtyScope::None, ..Default::default() }, &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("chain lands");
        assert_eq!(result.requested_effects, vec![chain]);
        assert_eq!(result.ui_scope, UiDirtyScope::None, "no verb was folded, so no scope was widened");
        assert!(result.diagnostics.is_empty());
        let snapshot = app.interaction.test_interaction_selection_snapshot(&app.stores);
        assert!(snapshot.selection.values().all(|selection| selection.ids.is_empty()), "a non-interaction DispatchAction moves no selection: {:?}", snapshot.selection);
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    mod owned_child_dispatch_laws { include!("../🔬️plugin-runtime-plugin-builder-contract/📨️owned-child-dispatch/🦀️.rs"); }