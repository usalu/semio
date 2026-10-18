/// 🎟️ Typed operation slots: registered dispatch rebase, exact slot pre-admission, slot release by the host call and the microsecond registered factory dispatch.

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_retained_keyed_dispatch::<KeyedTestApp>(
            keyed_test_registry().await,
            |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value },
            |value| TestMutation::SetCount(SetCount { value }),
            |snapshot| snapshot.count,
            None,
            mounted_policy,
            &mut identity,
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_slot_preadmission::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_slot_retirement_under_storm::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_slot_release_on_every_outcome::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn a_status_only_host_call_finishes_every_typed_operation_it_admitted() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_completes_under_a_status_only_host_call::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn an_admitting_host_call_hands_back_the_terminal_lane_it_earned() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_lands_its_edit_inside_the_admitting_call::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn microsecond_registered_factory_dispatch_preserves_exact_half_ms_fake_clock() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        fn clock() -> Option<u64> {
            Some(1_000)
        }
        test_retained_keyed_dispatch::<KeyedTestApp>(
            keyed_test_registry().await,
            |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value },
            |value| TestMutation::SetCount(SetCount { value }),
            |snapshot| snapshot.count,
            Some(clock),
            mounted_policy,
            &mut identity,
        )
        .await;
    }
