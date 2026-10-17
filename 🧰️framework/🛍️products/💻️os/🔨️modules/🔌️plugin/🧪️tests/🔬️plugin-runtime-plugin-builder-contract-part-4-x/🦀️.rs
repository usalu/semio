/// 🔬️ Items of Part 4 of 7 of the plugin builder contract laws that came from included fixture files.


async fn test_restart_registry() -> AppActionRegistry {
    let manifest = App::from_builder(
        App::builder(TestApp::<true>::APP_ID, LocalizedLabel::data("Restart Fixture"))
            .await.document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil").await
            .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow).await
            .app_command(TEST_RESTART_TOOL, LocalizedLabel::data("Apply Count From Task"), "task", ActionKind::Mutation).await
            .interactive_jobs(InteractiveJobClassification::Migrated).await,
    ).await;
    AppActionRegistry::from_definition(&manifest.definition)
}

async fn test_restart_publish_and_close(command: TestCommand, meta: &ActionMeta, fixture: &Value) {
    let mut original_caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original restart full-body caller");let mut original_context=original_caller.context();let cx=&mut original_context;
    let law = &fixture["restartAuthority"];
    let items = law["closeItems"].as_u64().unwrap() as usize;
    let body = law["closeBodyBytes"].as_u64().unwrap() as usize;
    let mut app = VcsArtifactApp::<TestApp<true>>::with_registry(TestApp::<true>::default(), test_restart_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    let outcome: Result<(u64, u64, u64, u64, u64, i32), semio_framework_job::OriginalTurnFailure> = async {
        app.bind_instance_id(meta.instance_id).await;
        let contracts = app.tool_public_contracts().await;
        let exact = contracts.iter().filter(|contract| contract.tool_id == TEST_RESTART_TOOL && contract.owner == ToolOwnerWitness::of::<TestApp<true>>() && contract.controller_id == TestApp::<true>::APP_ID && contract.schema_id == TEST_RESTART_SCHEMA).count();
        if exact as u64 != law["retainedProofs"].as_u64().unwrap() { return Err(Fault::from("restart app lost its exact registered tool contract").into()); }
        let admitted = app.dispatch_typed(command, meta, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await?;
        if !admitted.mutations.is_empty() { return Err(Fault::from("restart command bypassed retained publication").into()); }
        let (mut artifact, mut ui, mut scopes, mut terminal, mut completions) = (0, 0, 0, 0, 0);
        for _ in 0..100_000 {
            let demand = app.maintenance_retirement_demands(body)?;
            let grant = RetainedCloneGrant { maximum_items: items, ..plugin_demand_grant(demand) };
            if let Some(progress) = app.maintenance_step(cx,grant)?.progress() {
                if !progress.fits(grant) { return Err(Fault::from("restart maintenance exceeded the exact grant").into()); }
            }
            app.advance_typed_operation_publication(&mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await?;
            if let Some(page) = app.take_typed_operation_result_page(meta.instance_id) {
                let fault = match page.lane {
                    TypedOperationResultLane::Artifact => { artifact += 1; None }
                    TypedOperationResultLane::Ui => { ui += 1; None }
                    TypedOperationResultLane::Terminal => { terminal += 1; None }
                    TypedOperationResultLane::Fault => Some(Fault::from(format!("restart publication fault: {}", String::from_utf8_lossy(page.bytes())))),
                    _ => Some(Fault::from("restart publication produced an undeclared lane")),
                };
                if !app.acknowledge_typed_operation_result(page.token)? { return Err(Fault::from("restart publication rejected its exact result ACK").into()); }
                if let Some(fault) = fault { return Err(fault.into()); }
            }
            if let Some(scope) = app.take_typed_operation_ui_scope() {
                if !matches!(scope, semio_framework::kernel::UiDirtyScope::Full) { return Err(Fault::from("restart publication changed its declared full UI scope").into()); }
                scopes += 1;
            }
            if let Some(completion) = app.take_typed_operation_completion().await? {
                if !matches!(completion.ui_scope, semio_framework::kernel::UiDirtyScope::Full) { return Err(Fault::from("restart completion changed its declared full UI scope").into()); }
                if completion.operation == 0 { return Err(Fault::from("restart completion lost its exact operation id").into()); }
                completions += 1;
            }
            if !app.has_pending_typed_operations() { return Ok((artifact, ui, scopes, terminal, completions, app.snapshot()?.count)); }
            std::thread::yield_now();
        }
        Err(Fault::from("restart publication did not retire within the existing fixture turn bound").into())
    }.await;
    let mut close_fault:Option<semio_framework_job::OriginalTurnFailure> = None;
    for _ in 0..100_000 {
        if app.close_terminal_is_empty() { break; }
        let grant = match app.close_retirement_demands(body) {
            Ok(demand) => RetainedCloneGrant { maximum_items: items, ..plugin_demand_grant(demand) },
            Err(error) => { close_fault = Some(error.into()); break; }
        };
        match app.close_step(cx,grant) {
            Ok(step) if step.progress().is_some_and(|progress| !progress.fits(grant)) => {
                close_fault = Some(Fault::from("restart close exceeded the exact grant").into());
                break;
            }
            Err(fault) => { close_fault = Some(fault.into()); break; }
            _ => {}
        }
        std::thread::yield_now();
    }
    let closed = app.close_terminal_is_empty();
    assert!(closed, "original restart app must retire before the collected publication result is asserted");
    drop(app);
    let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original restart full-body source close");
    assert!(close_fault.is_none(), "{close_fault:?}");
    let (artifact, ui, scopes, terminal, completions, count) = outcome.expect("actual registered restart publication");
    assert_eq!(artifact, law["artifactPublications"].as_u64().unwrap());
    assert_eq!(ui, law["uiPublications"].as_u64().unwrap());
    assert_eq!(scopes, law["uiScopes"].as_u64().unwrap());
    assert_eq!(terminal, law["terminalReceipts"].as_u64().unwrap());
    assert_eq!(completions, law["operationCompletions"].as_u64().unwrap(), "one terminal typed operation publishes exactly one AppFrame::OperationCompleted witness");
    assert_eq!(i64::from(count), fixture["checkpoint"]["restartValue"].as_i64().unwrap());
}

/// 🧹️ Closes a source store built beside the app under its bounded owners.
fn close_bounded_source_store(source: &mut store::ArtifactStore<TestSnapshot, TestMutation>) {
    source.close_owned_unscheduled().expect("the source store closes under its own quoted demands");
    assert!(source.close_owned_terminal_is_empty());
}

/// ✏️ A whole-document archive whose history supersedes inputs — a replacement, a withdrawal and a garbage replacement a
/// remote replica authored — loads through parent hydration and the bounded retained initializer to exactly the state
/// the source store folds: the garbage input folds as a no-op with its `Fatal` outcome instead of refusing the load
/// (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, audit F-C1 and F-M1).
#[semio_framework_async_macros::async_test]
async fn a_whole_document_archive_with_supersessions_loads_its_superseded_state() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
    use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    let live_id = app.stores.store.envelope().id.clone();
    let genesis = store::os_store::document_materialize::create_document_envelope::<TestSnapshot, TestMutation>(SingleDocumentApp::DOCUMENT_SCHEMA, &live_id, TestSnapshot { count: 0, label: "initial".into(), slot: Vec::new() }, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut source = Box::pin(store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(genesis, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).await.expect("source store");
    source.install_operation_wire(store::ArtifactOperationWire::canonical());
    store::install_unscheduled_catalog(&mut source, store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<TestSnapshot, TestMutation>()).expect("source owners");
    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: "edited".into() }), TestMutation::SetCount(SetCount { value: 2 })] {
        semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![operation], transaction: None }, &mut identity)).await).expect("source edit");
    }
    let ids: Vec<protocol::MutationId> = source.mutation_ops().expect("source operations").into_iter().map(|operation| operation.mutation_id).collect();
    let inputs = vec![store::os_store::store_schema::SupersedeInput { target: ids[0].clone(), replacement: Some(TestMutation::SetCount(SetCount { value: 5 })) }, store::os_store::store_schema::SupersedeInput { target: ids[1].clone(), replacement: None }];
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.dispatch(store::os_store::store_schema::ArtifactCommand::Supersede { scope: None, inputs }, &mut identity)).await).expect("a clean supersession");
    let garbage = protocol::HistoryTransition::Supersede(protocol::TransitionSupersede {
        scope: None,
        inputs: vec![protocol::SupersededInput { target: ids[2].clone(), replacement: protocol::InputReplacement::Input { schema: SingleDocumentApp::DOCUMENT_SCHEMA.into(), payload: vec![0xff, 0x13, 0x37] } }],
    });
    let remote = protocol::history_transition_envelope(&garbage, &protocol::ArtifactId(live_id.clone()), &protocol::ActorId("remote-editor".into()), vec![ids[2].clone()], protocol::HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical: 0 });
    assert!(Box::pin(source.ingest_remote(remote)).await.expect("the garbage supersession ingests").accepted);
    let expected = source.snapshot().expect("source projection");
    assert_eq!((expected.count, expected.label.as_str()), (5, "initial"));
    let files = Box::pin(store::os_store::text_format::print_document_pack(source.envelope())).await.expect("source pair prints");
    let dialect: ArtifactDialect = SingleDocumentApp::DIALECT.into();
    let parent_spr = Box::pin(store::os_store::text_format::stamp_document_spr_identity(&files.spr, &live_id, SingleDocumentApp::DOCUMENT_SCHEMA, &dialect, app.stores.store.envelope().owner.as_ref())).await.expect("load-document identity stamp");
    PluginApp::begin_document_archive_load(&mut app, 95, protocol::DocumentArchivePack { parent_pack: files.pack, parent_spr, members: Vec::new() }).expect("superseded archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 95, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "a superseded archive failed to load: {}", archive_fault_text(&status));
    PluginApp::acknowledge_document_archive_load(&mut app, 95).expect("superseded archive acknowledgement");
    assert_eq!(app.snapshot().expect("replaced projection"), expected);
    assert_eq!(app.stores.store.supersessions(), source.supersessions());
    let fatal = app.stores.store.mutation_outcomes().expect("loaded outcomes").into_iter().find(|outcome| outcome.mutation_id == ids[2]).expect("the garbage-superseded operation");
    assert_eq!((fatal.worst, fatal.superseded, fatal.withdrawn), (Some(semio_framework_diagnostic::Severity::Fatal), true, false));
    close_bounded_source_store(&mut source);
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

/// 🗂️ One history row as a human reads it: what it names (the edit or the history transition) and its English label.
fn archive_history_rows(history: &semio_framework::kernel::HistoryPatch) -> Vec<(Option<String>, Option<String>, String)> {
    let mut rows: Vec<(Option<String>, Option<String>, String)> = history.upserts.iter().map(|entry| (entry.edit_id.clone(), entry.transition_id.clone(), label_in(&entry.label, Locale::En).to_string())).collect();
    rows.sort();
    rows
}

/// 🗂️ LAW (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up 3, e2e R2-2): a document archive carries its whole
/// history. The source's edits, its history edit (`Supersede`) and an undo and redo of it (`Revert`, `Reinstate`) print into
/// the archive, and so do two history edits finalized as new alternatives in a row with edits on the second; a fresh instance that loads
/// it — holding a document of its own first, as a reloaded playground holds its example — lists exactly the rows the source
/// lists, and none of the document it replaced.
#[semio_framework_async_macros::async_test]
async fn a_document_archive_round_trip_lists_every_history_row_of_its_source() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
    use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
    let mut source = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut source, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: "edited".into() }), TestMutation::SetCount(SetCount { value: 2 })] {
        semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![operation], transaction: None }, &mut identity)).await).expect("source edit");
    }
    let ids: Vec<protocol::MutationId> = source.store.mutation_ops().expect("source operations").into_iter().map(|operation| operation.mutation_id).collect();
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::Supersede { scope: None, inputs: vec![store::os_store::store_schema::SupersedeInput { target: ids[0].clone(), replacement: Some(TestMutation::SetCount(SetCount { value: 5 })) }] }, &mut identity)).await).expect("history edit");
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::Undo, &mut identity)).await).expect("revert");
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::Redo, &mut identity)).await).expect("reinstate");
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::CreateAlternativeWithSupersede { name: "c".into(), inputs: vec![store::os_store::store_schema::SupersedeInput { target: ids[2].clone(), replacement: Some(TestMutation::SetCount(SetCount { value: 7 })) }] }, &mut identity)).await).expect("history edit as a new alternative");
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::CreateAlternativeWithSupersede { name: "b".into(), inputs: vec![store::os_store::store_schema::SupersedeInput { target: ids[1].clone(), replacement: Some(TestMutation::SetLabel(SetLabel { value: "alternative".into() })) }] }, &mut identity)).await).expect("a second history edit as a new alternative");
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetLabel(SetLabel { value: "branched".into() })], transaction: None }, &mut identity)).await).expect("edit on the alternative");
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(source.store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 9 })], transaction: None }, &mut identity)).await).expect("a second edit on the alternative");
    let expected = source.snapshot().expect("source projection");
    let before = archive_history_rows(&Box::pin(source.history_snapshot()).await.expect("source history"));
    let archive = Box::pin(PluginApp::document_archive(&source)).await.expect("source archive");
    let mut target = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut target, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(target.store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 42 })], transaction: None }, &mut identity)).await).expect("target example");
    let replaced = archive_history_rows(&Box::pin(target.history_snapshot()).await.expect("target history before the load"));
    let dialect: ArtifactDialect = SingleDocumentApp::DIALECT.into();
    let target_id = target.store.envelope().id.clone();
    let parent_spr = Box::pin(store::os_store::text_format::stamp_document_spr_identity(&archive.parent_spr, &target_id, SingleDocumentApp::DOCUMENT_SCHEMA, &dialect, target.store.envelope().owner.as_ref())).await.expect("archive identity stamp");
    PluginApp::begin_document_archive_load(&mut target, 96, protocol::DocumentArchivePack { parent_spr, ..archive }).expect("archive admission");
    let status = Box::pin(drive_single_document_archive(&mut target, 96, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "the archive failed to load: {}", archive_fault_text(&status));
    PluginApp::acknowledge_document_archive_load(&mut target, 96).expect("archive acknowledgement");
    assert_eq!(target.snapshot().expect("loaded projection"), expected);
    assert_eq!(target.store.envelope().transitions.len(), source.store.envelope().transitions.len(), "every history transition survives the round trip");
    let after = archive_history_rows(&Box::pin(target.history_snapshot()).await.expect("target history after the load"));
    for label in ["Set count to 1", "Set label to edited", "Set count to 2", "Set label to branched", "Set count to 9"] {
        assert!(before.iter().any(|(_, _, row)| row == label), "the source lists {label}: {before:?}");
    }
    assert_eq!(after, before, "the loaded instance lists exactly the source's rows (it listed {replaced:?} before the load)");
    close_member_admission_app(&mut source, &mut cx);
    close_member_admission_app(&mut target, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}