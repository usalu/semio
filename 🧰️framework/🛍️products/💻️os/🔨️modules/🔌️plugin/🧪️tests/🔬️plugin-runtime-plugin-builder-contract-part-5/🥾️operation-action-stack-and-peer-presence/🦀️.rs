/// 🥾️ Operation action, stack and peer presence: the kernel op with its true inverse, the ephemeral interaction snapshot, the bounded thread stack, peer presence capture and roster saturation.

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
