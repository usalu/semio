/// 🪜️ The browser's mounted per-turn grant: a real document archive load with its store replacement and the typed operation behind it complete through the instance maintenance ladder within a bounded number of turns, never stalling on a unit whose demand exceeds the per-unit loan. Rows are language-agnostic data in `🧫️fixtures/🔣️.json`.

    fn mounted_grant_policy(row: &Value) -> semio_plugin_mounted_owner::MountedOwnerPolicyV1 {
        let field = |name: &str| usize::try_from(row[name].as_u64().unwrap()).unwrap();
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: field("items"), maximum_copy_bytes: field("copy"), maximum_capacity_bytes: field("capacity"), maximum_release_bytes: field("release"), maximum_depth: field("depth") };
        semio_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant }
    }

    #[semio_framework_async_macros::async_test]
    async fn a_document_archive_load_and_its_typed_publication_complete_under_the_browser_mounted_grant() {
        let mut original_caller=semio_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let rows: Vec<Value> = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        let source_policy = semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut source = contract_composed_app_raw(source_policy, &mut identity).await;
        register_test_child(&mut source, "child-1").await;
        semio_plugin_testing_laws::artifact_app_laws::dispatch_original_fixture_command(&mut source, &TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), source_policy, &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut source, meta().instance_id, source_policy, &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("the source gesture settles before it is archived");
        let archive = PluginApp::document_archive(&source).await.expect("the composed document archives its closure");

        for row in &rows {
            let policy = mounted_grant_policy(row);
            let fuel = row["fuel"].as_u64().unwrap();
            let (fresh_grant, _) = cx.original_turn_ceiling();
            let mut reloaded = contract_composed_app_raw(policy, &mut identity).await;
            artifact_app_laws::admit_document_archive_load(&mut reloaded, 31, archive.clone(), cx).expect("the runtime admits the archive");
            let (mut turns, mut widest_copy, mut status) = (0usize, 0usize, None);
            while turns < row["maximumLoadTurns"].as_u64().unwrap() as usize {
                turns += 1;
                artifact_app_laws::fixture_renew_turn(cx, fuel, fresh_grant).expect("renewed mounted turn");
                if let Ok(demand) = reloaded.maintenance_retirement_demands(policy.maintenance.maximum_copy_bytes, cx.fuel_remaining()) { widest_copy = widest_copy.max(demand.copy_bytes); }
                let phase = reloaded.mounted_owner_maintenance_phase().expect("mounted phase");
                let caller = semio_plugin_mounted_owner::MountedOwnerIdentityV1 { instance_id: meta().instance_id, operation: cx.operation(), generation: cx.generation() };
                if let Some(mut turn) = semio_plugin_mounted_owner::MountedOwnerTurnV1::admit(meta().instance_id, caller, phase, policy, cx).expect("mounted admission") {
                    reloaded.mounted_owner_maintenance_step(&mut turn).unwrap_or_else(|failure| panic!("{}: mounted maintenance turn {turns} refused: {failure:?}", row["id"]));
                }
                artifact_app_laws::fixture_renew_turn(cx, fuel, fresh_grant).expect("renewed mounted turn");
                let polled = PluginApp::poll_document_archive_load(&mut reloaded, 31, &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("poll");
                if !matches!(polled.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) { status = Some(polled); break; }
            }
            let status = status.unwrap_or_else(|| panic!("{}: the archive load made no terminal progress in {turns} mounted turns (widest quoted copy demand {widest_copy} B against a {} B unit loan)", row["id"], policy.maintenance.maximum_copy_bytes));
            assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}: {:?}", row["id"], semio_framework_diagnostic::decode_fault_bytes(&status.fault));
            PluginApp::acknowledge_document_archive_load(&mut reloaded, 31).expect("acknowledge");
            assert_eq!(PluginApp::document_archive(&reloaded).await.expect("reloaded archive").members.len(), 1, "{}: the reloaded parent carries its member", row["id"]);

            artifact_app_laws::dispatch_original_fixture_command(&mut reloaded, &TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 9 }, &meta(), policy, &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap_or_else(|failure| panic!("{}: typed operation refused: {failure:?}", row["id"]));
            artifact_app_laws::settle_registered_typed_operation(&mut reloaded, meta().instance_id, policy, &mut semio_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap_or_else(|failure| panic!("{}: the typed operation did not publish after the load: {failure:?}", row["id"]));
            assert_eq!(PluginApp::document_archive(&reloaded).await.expect("published archive").members.len(), 1, "{}: the typed operation published over the loaded member", row["id"]);
            drain_and_close_composed_fixture(&mut reloaded, source_policy, cx);
        }

        drain_and_close_composed_fixture(&mut source, source_policy, cx);

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
