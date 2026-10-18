/// 🧹️ Child root maintenance and rates: terminal-empty child root reclaim, child publication rate waits, the maximum child dispatch and created children.

    /// 🎡️ The property is RECLAMATION, not the step word. `maintenance_step` rotates: one call
    /// walks every stage that still has something to give, so a later idle stage legitimately answers
    /// `Complete` and that is what the call returns — the previous form of this law read that as the
    /// LYING owner reaching a terminal step and failed on a stage it never visited. What must never
    /// happen is the lying owner's authority leaving the registry: every turn until the fault arrives
    /// is asserted to leave `child_content_retirements` populated, which is strictly what the old
    /// clause was a proxy for, and the fault must still arrive by name before any reclaim.
    ///
    /// 🪢️ The registered root captured the member's CURRENT snapshot, so while the member still
    /// holds that exact `Arc` the returned-read retirement is only an alias: `Arc::into_inner`
    /// answers `None`, the alias is released truthfully and the member's own owned-value factory
    /// — where the lie lives — is never consulted. Advancing the member past that snapshot and
    /// draining the alias its displacement retired makes the retiring root the snapshot's LAST
    /// owner, which is the only state in which a child-content retirement meets a lying disposer
    /// at all.
    ///
    /// 🪪️ Either named guard is an acceptable answer — the plugin ladder's own terminal-witness
    /// refusal, or the nested disposer refusal it now attributes by name instead of as the
    /// anonymous `plugin.internal` — but an unnamed internal error is not.
    ///
    /// 🧹️ The lie is spent (see `TestSnapshotRetirement::close_step`), so the very disposer that
    /// was caught now answers honestly and the fixture closes through the ordinary ladder — the
    /// law proves the refusal, not a leak.
    #[semio_framework_async_macros::async_test]
    async fn child_root_maintenance_requires_terminal_empty_before_reclaim() {
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
        install_test_snapshot_retirement(&mut app, "child-a", true);
        {
            let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), "child-a".to_string())).expect("exact child owner").member;
            for value in 1..=2 {
                apply_test_mutations(child, vec![TestMutation::SetCount(SetCount { value })]).await.expect("member advances past the captured snapshot");
            }
            for _ in 0..1_024 {
                if child.maintenance_retirements_terminal_is_empty() {
                    break;
                }
                let demand = child.maintenance_retirements_demands(4096).expect("quoted displaced snapshot demand");
                let _ = child.maintenance_retirements_step(fixture_close_grant(demand)).expect("member drains its displaced snapshot aliases");
            }
            assert!(child.maintenance_retirements_terminal_is_empty(), "the member released every snapshot alias it displaced");
        }
        let generation = app.child_composition.admit_child_content_publication().expect("admit replacement root");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        app.maintenance.maintenance_stage = 4;
        assert!(matches!(app_maintenance_turn(&mut app,cx).expect("transfer retirement authority"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(_)));
        let mut faulted = None;
        for turn in 0..64 * usize::from(MAINTENANCE_STAGES) {
            app.maintenance.maintenance_stage = 4;
            match app_maintenance_turn(&mut app,cx) {
                Ok(step) => assert!(
                    !app.child_composition.child_content_retirements.is_empty(),
                    "the lying owner's authority must never be reclaimed without its terminal witness: turn {turn} answered {step:?} from stage {} and left the registry empty",
                    (app.maintenance.maintenance_stage + MAINTENANCE_STAGES - 1) % MAINTENANCE_STAGES
                ),
                Err(fault) => {
                    faulted = Some(fault);
                    break;
                }
            }
        }
        let fault = faulted.expect("lying Complete must fail before registry removal");
        assert!(
            matches!(fault.code.0.as_str(), "interactive-job.child-snapshot-terminal-not-empty" | "interactive-job.child-snapshot-disposer-refused"),
            "the lie must be caught BY NAME: {fault:?}"
        );
        assert!(!app.child_composition.child_content_retirements.is_empty(), "terminal witness failure retains the registry authority");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn child_root_maintenance_reclaims_completed_owner_for_later_publication() {
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
        install_test_snapshot_retirement(&mut app, "child-a", false);
        let generation = app.child_composition.admit_child_content_publication().expect("admit replacement root");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        for _ in 0..64 {
            if app.child_composition.child_content_retirements.is_empty() {
                break;
            }
            app.maintenance.maintenance_stage = 4;
            let _ = app_maintenance_turn(&mut app,cx).expect("bounded retirement progress");
        }
        assert!(app.child_composition.child_content_retirements.is_empty(), "terminal-empty retirement is reclaimed while the app remains live");
        let generation = app.child_composition.admit_child_content_publication().expect("later publication can reuse the fixed retirement registry");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("later publication after bounded reclaim");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): child-content publications at the maximum rate — 10 000 with no
    /// maintenance turn between them — never fault. Every admission waits on the retirement ahead of it (bounded synchronous
    /// reclaim) instead of answering `interactive-job.child-root-retirement-saturated`, every publication is admitted in
    /// order (exactly the next generation), the registry never holds more than its fixed slots, and the saturated registry
    /// reports maintenance pressure until maintenance returned it below its quarter-occupancy bound.
    #[semio_framework_async_macros::async_test]
    async fn child_publications_at_the_maximum_rate_wait_on_retirement_instead_of_faulting() {
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
        install_test_snapshot_retirement(&mut app, "child-a", false);
        let start = app.child_composition.child_content_generation;
        for index in 1..=10_000u64 {
            let generation = app.child_composition.admit_child_content_publication().unwrap_or_else(|fault| panic!("publication {index} faulted instead of waiting: {fault:?}"));
            assert_eq!(generation, start + index, "publication {index} is admitted in order");
            app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.unwrap_or_else(|fault| panic!("publication {index} did not land: {fault:?}"));
            assert!(app.child_composition.child_content_retirements.len() <= ARTIFACT_LIVE_OUTPUT_SLOTS, "the child-root retirement registry stays within its fixed slots");
        }
        assert!(PluginApp::maintenance_under_pressure(&app), "a saturated child-root retirement registry reports maintenance pressure");
        semio_framework_plugin_testing_laws::artifact_app_laws::drain_maintenance_pressure(&mut app,cx);
        assert!(app.child_composition.child_content_retirements.len() < MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY, "pressure maintenance returns the child roots below their quarter-occupancy bound");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⏱️ This clause used to be an 8 ms wall clock. The clock was a PROXY for one property — the
    /// maximum child's bytes never cross the public dispatch boundary, the dispatch hands back a
    /// continuation instead — and on a loaded machine the proxy fails while the property holds. The
    /// property itself is asserted directly and deterministically: the dispatch requests its
    /// continuation, its own answer carries no child bytes, and the two materialization counters
    /// below stay at zero. Nothing the clock could catch is lost: a dispatch that walked the
    /// 4 MiB snapshot would have to clone it or pack-encode it, and both are counted.
    #[semio_framework_async_macros::async_test]
    async fn maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-maximum").await;
        let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), "child-maximum".to_string())).expect("maximum child").member;
        apply_test_mutations(child, vec![TestMutation::SetLabel(SetLabel { value: "x".repeat(MAXIMUM_CHILD_PROBE_BYTES) })]).await.expect("seed maximum child");
        let publication_generation = app.child_composition.admit_child_content_publication().expect("admit maximum child publication");
        app.child_composition.publish_child_content_member(publication_generation, "slot", "child-maximum").await.expect("publish maximum child root");
        MAXIMUM_CHILD_CLONES.store(0, std::sync::atomic::Ordering::Release);
        MAXIMUM_CHILD_ENCODINGS.store(0, std::sync::atomic::Ordering::Release);

        let result = app.dispatch_action("probeChild", Some(&dv(serde_json::json!({ "slot": "slot", "childId": "child-maximum" }))), &meta(), &mut identity,cx).await.expect("public maximum-child probe");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "probeChildContinuation")));
        let answer = format!("{:?}", result.output);
        assert!(answer.len() < 4096, "the public dispatch's own answer carried {} bytes of the maximum child instead of a continuation", answer.len());
        assert!(result.mutations.is_empty(), "the public dispatch published mutations instead of deferring to its continuation");
        assert_eq!(MAXIMUM_CHILD_CLONES.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must clone only the existing Arc, never the maximum snapshot graph");
        assert_eq!(MAXIMUM_CHILD_ENCODINGS.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must not pack-encode maximum child content during public dispatch");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧒️ The parent declares the created child first: absorb admits a member only under the
    /// identity the published parent declares, like every other admission.
    ///
    /// 🌱️ Proves `VcsArtifactApp::absorb_created_children` — the mechanism a
    /// `ChildGenesis`-authoring `Emit` constructor (a later wave) will rely on to make a
    /// freshly-minted child reachable at all; per B2's own `GroupReceipt::created_children`
    /// doc comment, skipping this step would make `ChildGenesis` pointless.
    #[semio_framework_async_macros::async_test]
    async fn created_children_survive_absorb_into_the_child_store_map() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        declare_test_child(&mut app, "genesis-child").await;
        let parent_id = app.stores.store.envelope().id.clone();
        app.child_composition.composition.graph_mut().await.insert_owns(&parent_id, "slot", "genesis-child").await.expect("seed ownership so absorb's slot_of lookup resolves");
        let target = ArtifactRef { artifact_id: "genesis-child".into(), dialect: test_child_dialect().await };
        let created: Vec<(ArtifactRef, TestMembers)> = vec![(target, new_test_child("genesis-child").await.expect("construct genesis child"))];

        app.child_composition.absorb_created_children(&mut app.stores, created).await.expect("absorb child and publish immutable root");

        let entry = app.child_composition.children.get_mut(&("slot".to_string(), "genesis-child".to_string())).expect("genesis child absorbed into the live map under the slot its parent declares it in");
        assert_eq!(entry.reference.dialect.artifact_kind, "s.test.child");
        assert_eq!(entry.member.document_id().await, "genesis-child");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
