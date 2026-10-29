/// 🏗️ Media result, close destructor and builder: the explicit mime of a completed media result, the constant final destructor of an artifact close, the retained field limits and the plugin builder wiring.

    #[test]
    fn completed_media_result_uses_explicit_mime_instead_of_schema() {
        let chunks = ArtifactOutputChunks::new(4);
        assert_eq!(chunks.push(vec![1, 2, 3, 4]), Ok(4));
        assert_eq!(chunks.seal(), Ok(4));
        let result = semio_framework_plugin_decl_media::ArtifactMediaExportResult::structured(MediaType { class: MediaClass::Presentation, form: MediaForm::Sequence }, "stdio.mp4.isobmff", "video/mp4", chunks).expect("completed media");
        assert_eq!(result.mime_type, "video/mp4");
        assert_eq!(result.schema, "stdio.mp4.isobmff");
        assert_ne!(result.mime_type, result.schema);
        assert_eq!(result.chunks.bytes(), 4);
        assert!(result.chunks.is_sealed());
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_close_final_destructor_is_constant_after_every_owned_field_is_drained() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
        let mut app = contract_app_raw(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        let _ = app.render_ctx().test_history().await;
        let mut turns = 0;
        for _ in 0..4_096 {
            if app.ladder_ref().retained_fields_own_terminal_is_empty() {
                break;
            }
            let demand = app.ladder_ref().retained_fields_close_demand(page).expect("quoted retained field demand");
            let grant = fixture_close_grant(demand);
            let starved = store::RetainedCloneGrant { maximum_items: 0, ..grant };
            assert_eq!(app.ladder_ctx().retained_fields_close_step(starved).expect("unfunded retained field turn yields").progress(), store::RetainedCloneProgress::default());
            let progress = app.ladder_ctx().retained_fields_close_step(grant).expect("retained field turn").progress();
            assert!(progress.fits(grant), "retained field receipt {progress:?} exceeds its exact grant {grant:?}");
            assert_ne!(progress, store::RetainedCloneProgress::default(), "a funded retained field turn must move one owner");
            turns += 1;
        }
        assert!(turns != 0, "the warmed projection cache and history view are retained fields the rung must release");
        assert!(app.ladder_ref().retained_fields_own_terminal_is_empty());
        assert!(matches!(app.ladder_ctx().retained_fields_close_step(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close).expect("repeated terminal turn"), store::RetainedCloneStep::Complete(progress) if progress == store::RetainedCloneProgress::default()));
        artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[test]
    fn retained_field_maximum_and_maximum_plus_one_are_language_neutral() {
        use store::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        for (letter, length) in [("x", ARTIFACT_OUTPUT_CHUNK_BYTES), ("y", ARTIFACT_OUTPUT_CHUNK_BYTES + 1)] {
            let mut registry = AppActionRegistry::test_with_controller_id(letter.repeat(length));
            let detach = fixture_close_grant(registry.retirement_demands());
            assert_eq!(registry.close_step(detach).expect("row detaches"), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }), "the row detaches whole");
            let quote = registry.retirement_demands();
            assert!(quote.release_bytes >= length, "the detached identifier is quoted as one whole release");
            let exact = fixture_close_grant(quote);
            let starved = RetainedCloneGrant { maximum_release_bytes: quote.release_bytes - 1, ..exact };
            assert_eq!(registry.close_step(starved).expect("starved release yields"), RetainedCloneStep::Progress(RetainedCloneProgress::default()), "a release grant below the quote yields instead of refusing");
            assert!(!registry.terminal_is_empty(), "the unreleased allocation stays retained until its own whole grant");
            assert_eq!(registry.close_step(exact).expect("exact release"), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: quote.release_bytes, ..Default::default() }), "a grant at the quote retires the identifier in one whole release");
            assert!(registry.terminal_is_empty());
            let terminal = fixture_close_grant(registry.retirement_demands());
            assert_eq!(registry.close_step(terminal).expect("terminal"), RetainedCloneStep::Complete(RetainedCloneProgress::default()));
            assert_eq!(registry.close_step(terminal).expect("repeated terminal"), RetainedCloneStep::Complete(RetainedCloneProgress::default()), "repeated close is idempotent after interruption and resume");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn unproved_command_fails_before_an_overrun_reducer_can_start() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = unproved_contract_app(mounted_policy, &mut identity).await;
        assert!(!app.tools.test_registered_tool_keys().iter().any(|key| key.1 == "watchdogOverrun"), "the unproved fixture registers no owned factory for this verb");
        let error = app.dispatch_typed(TestCommand::WatchdogOverrun, &meta(), &mut identity, cx).await.expect_err("an unproved operation must fail before an over-budget reducer starts");
        assert_eq!(error.code.0, "interactive-job.not-ui-safe", "a verb with no proof is a verb that was never migrated, and the backstop names the cause");
        assert_eq!(app.stores.test_snapshot().await.count, 0, "the over-budget reducer never started");
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn plugin_builder_builds_bundle_from_fluent_spec() {
        let bundle = __semio_plugin_bundle().await.expect("synthetic plugin assembly");
        assert_eq!(bundle.manifest.plugin_id, "test");
        assert_eq!(bundle.manifest.label.as_str(), "Synthetic");
        assert_eq!(bundle.manifest.version, "0.0.1");
        assert!(bundle.manifest.apps.iter().any(|app| app.id == TestApp::<false>::APP_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn test_app_id_matches_its_own_dialect() {
        assert_eq!(TestApp::<false>::APP_ID, test_app_surface_id().await, "TestApp::<false>::APP_ID must not drift from TEST_APP_DIALECT (contract §1)");
    }

    #[semio_framework_async_macros::async_test]
    async fn plugin_builder_wires_app_factory_for_create_app() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let bundle = __semio_plugin_bundle().await.expect("synthetic plugin assembly");
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🏪️store/🧪️tests/🧫️fixtures/🧫️actor-genesis/🔣️.json")).expect("neutral actor/genesis fixture");
        let actor = protocol::ActorId(fixture["actors"]["opened"].as_str().expect("opened actor").into());
        let mut app = bundle.create_app(TestApp::<false>::APP_ID, actor.clone(), mounted_policy, &mut identity).expect("registered app factory births").expect("registered app");
        assert_eq!(app.app_id().await, TestApp::<false>::APP_ID);
        let TestRuntimeApps::Test(owner) = &mut app;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(owner, mounted_policy);
        assert_eq!(owner.store.local_actor_id(), &actor);
        assert_eq!(owner.config_store.local_actor_id(), &actor);
        assert_eq!(owner.draft_store.local_actor_id(), &actor);
        assert_eq!(owner.interaction_store.local_actor_id(), &actor);
        let roots = [owner.store.local_actor_id(), owner.config_store.local_actor_id(), owner.draft_store.local_actor_id(), owner.interaction_store.local_actor_id()];
        assert_eq!(serde_json::to_value(roots.map(|actor| actor.0.as_str())).expect("independent actor projection"), serde_json::json!([fixture["actors"]["opened"], fixture["actors"]["opened"], fixture["actors"]["opened"], fixture["actors"]["opened"]]));
        let pack = owner.document_pack().await.expect("stored document");
        artifact_app_laws::load_document(owner, &pack, &mut identity).await.expect("production reload");
        assert_eq!(owner.store.local_actor_id(), &actor, "reload retains the opened actor");
        assert!(bundle.create_app("unknown-app", actor, mounted_policy, &mut identity).expect("unknown app is not a fault").is_none());
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
