// ⏲️ Interior undo, pure lane and wall deadline: history verb refusals, an interior undo over a long history, the pure lane hydration and a deferred history step that ends at the wall deadline.

//#region 🧭️AcceptanceLaws
/// 🔁️ A history verb as `actor`, admitted and settled, answering the fault code of a refusal (`None`: applied).
async fn history_verb_refusal(app: &mut ToyApp, actor: &str, action: &str, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Option<String> {
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let settled = match app.handle_action(action, None, &meta, identity).await {
        Ok(admitted) => semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(app, admitted).await,
        Err(fault) => Err(fault),
    };
    settled.err().map(|fault| fault.code.0)
}

/// 🧮️ What a history step may change: the head, the content revision, the edits, the transitions and the history rows.
async fn history_trace(app: &mut ToyApp) -> ((i32, String), [u8; 32], usize, usize, usize) {
    let rows = app.render_ctx().history_patch(true).await.expect("history patch").upserts.len();
    (head(app), app.stores.store.content_revision_now(), app.stores.store.envelope().vcs.edits.len(), app.stores.store.envelope().transitions.len(), rows)
}

/// ✏️ Publishes a foreign downstream edit per value and drains displaced owners under bounded pressure.
async fn apply_other_edits(app: &mut ToyApp, count: i32) {
    for value in 0..count {
        publish_as(app, "downstream", SetCount { value }.into());
        for _ in 0..4_096 {
            if !app.stores.store.maintenance_retirements_under_pressure(){break;}
            let grant=app.mounted_policy.maintenance;
            let step=app.stores.store.maintenance_retirements_step(grant).expect("the store retires what the edits displaced");assert!(step.progress().fits(grant));
            if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){break;}
        }
    }
}

/// ⚰️ One edit of `author` ([`publish_as`]) under `count` one-operation edits of the fixture actor
/// ([`apply_other_edits`]): the history an interior undo of `author` replays through. Proves what it built: exactly one
/// history row is `author`'s and it is not the newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(author.into()), "the buried edit belongs to the opened actor");
    publish_as(app, author, SetLabel { value: "buried".into() }.into());
    apply_other_edits(app, count).await;
    app.render_ctx().refresh_cache().await.expect("backfill");
    let authors: Vec<Option<String>> = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().map(|row| row.author).collect();
    let own = authors.iter().filter(|row| row.as_deref() == Some(author)).count();
    assert!(own == 1 && authors.first().and_then(|row| row.as_deref()) != Some(author), "one interior row is {author}'s: {own} of {} rows, newest {:?}", authors.len(), authors.first());
}

/// ⚖️ LAW (gap N17): an interior undo over a long downstream history — the author's own edit under 600 edits of another
/// author — is a local history step the runtime replays over driver turns:
/// - the undo answers at once, the head, the edits and the history rows unchanged;
/// - the wire carries `reprojection.kind = step`, the body reads "Replaying history" / "Verlauf wird neu angewendet";
/// - each turn replays at most `TIME_TRAVEL_REPLAY_OPERATIONS`;
/// - meanwhile undo and redo answer `history.replaying` and `historyEditBegin` answers `timeTravel.busy`;
/// - the adoption equals the undo of a store that replays inside the dispatch, and records the undo row.
/// Cancel replay instead drops the step with zero trace: head, revision, edits, transitions and history rows as before.
#[semio_framework_async_macros::async_test]
async fn an_interior_undo_over_a_long_history_replays_over_turns_and_cancel_leaves_zero_trace() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = "author".to_string();
    let mut undeferred = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
    undeferred.store.defer_local_replays(None);
    bury_edit_of(&mut undeferred, &actor, 600).await;
    history_verb(&mut undeferred, &actor, "undo", None, mounted_policy, &mut identity).await;
    let undone = undeferred.store.snapshot().expect("the undeferred undo");
    for cancel in [true, false] {
        let mut app = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
        bury_edit_of(&mut app, &actor, 600).await;
        let before = history_trace(&mut app).await;
        history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
        assert!(app.stores.store.local_step_pending(), "the interior undo waits for its replay");
        assert_eq!(history_trace(&mut app).await, before, "nothing of the step shows before its adoption");
        let status = app.render_ctx().history_patch(true).await.expect("history patch").reprojection.expect("the wire carries the waiting step");
        assert!(status.kind == semio_framework::kernel::HistoryReprojectionKind::Step && status.total > 0 && !status.paused && status.fault.is_none(), "{status:?}");
        for (locale, line) in [(Locale::En, "Replaying history"), (Locale::De, "Verlauf wird neu angewendet")] {
            let body = render_history(&mut app, locale).await;
            assert!(find_node(&body, "framework.history.reprojection.status").is_some_and(|row| row.to_string().contains(line)), "{locale:?}: {body}");
        }
        assert_eq!(history_verb_refusal(&mut app, &actor, "undo", mounted_policy, &mut identity).await.as_deref(), Some("history.replaying"), "a further history step waits");
        assert_eq!(history_verb_refusal(&mut app, &actor, "redo", mounted_policy, &mut identity).await.as_deref(), Some("history.replaying"));
        let first = seeded_mutation(&app, 0);
        assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))], mounted_policy, &mut identity).await), Some("timeTravel.busy"), "history editing waits for the step");
        if cancel {
            let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
            assert_eq!(rejected(&cancelled), None, "{:?}", cancelled.output);
            assert!(!app.stores.store.local_step_pending() && app.render_ref().reprojection_status().is_none(), "the step is gone");
            for _ in 0..4 {
                app.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
                collect_original_history_command_receipt(&mut app);
            }
            assert_eq!(history_trace(&mut app).await, before, "a cancelled step leaves zero trace");
        } else {
            let mut turns = 0;
            while app.stores.store.local_step_pending() {
                let done = app.stores.store.reprojection_progress().map_or(0, |progress| progress.done);
                app.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
                collect_original_history_command_receipt(&mut app);
                while app.take_typed_operation_ui_progress().is_some() {}
                let after = app.stores.store.reprojection_progress().map_or(u32::MAX, |progress| progress.done);
                assert!(after == u32::MAX || after.saturating_sub(done) as usize <= time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, "one turn replays at most one budget: {done} → {after}");
                turns += 1;
                assert!(turns < 64, "the step is adopted");
            }
            assert!(turns >= 2, "a 600-operation downstream spans several turns ({turns})");
            assert_eq!(app.stores.store.snapshot().expect("the adopted undo"), undone, "the adoption equals the undeferred undo");
            let (_, _, edits, _, rows) = history_trace(&mut app).await;
            assert_eq!((edits, rows), (before.2, before.4 + 1), "the adoption records the undo row");
        }
        close(&mut app,cx);
    }
    close(&mut undeferred,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (decision §4 of `📓️api-stepped-document-load.md`): a pure command's document lane hydrates the HEAD snapshot of
/// a 240-edit document without its history (`PluginApp::hydrate_pure_head` takes the head pack alone, so no history can
/// ride it) — the store holds no edit, the head is the source head, nothing folds; in that head-only mode undo, a history
/// edit and the history query refuse `pure.history-unavailable`, and the whole-document archive load of the history (the
/// only load path) lifts it over real polls, landing the source head and all 240 edits.
#[semio_framework_async_macros::async_test]
async fn a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut source = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
    for value in 0..240 {
        source.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], transaction: None }, &mut identity).await.expect("a source edit");
    }
    let head = source.stores.store.snapshot().expect("the source head");
    let files = source.document_pack().await.expect("the source pair");
    let mut pure = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into()), mounted_policy, &mut identity).await;
    let code = |fault: Fault| fault.code.0;
    PluginApp::hydrate_pure_head(&mut pure, &head.encode_pack()).await.expect("the head-only lane hydrates");
    assert_eq!(pure.stores.store.snapshot().expect("the pure head"), head, "the pure document is the source head");
    assert!(pure.stores.store.envelope().vcs.edits.is_empty() && pure.time_travel.history_unavailable(), "no history was folded or kept");
    assert_eq!(history_verb_refusal(&mut pure, &actor, "undo", mounted_policy, &mut identity).await.as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE));
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(&actor) };
    let begin = pure.handle_action("historyEditBegin", Some(&DslValue::object([("mutationId".to_string(), DslValue::String("any".into()))])), &meta, &mut identity).await;
    assert_eq!(begin.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "a history edit needs the history");
    assert_eq!(pure.history_snapshot().await.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "so does the history query");
    let envelope = pure.stores.store.envelope();
    let dialect: ArtifactDialect = <ToyHistoryApp as ArtifactApp>::DIALECT.into();
    let parent_spr = store::os_store::text_format::stamp_document_spr_identity(&files.spr, &envelope.id, <ToyHistoryApp as ArtifactApp>::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref()).await.expect("the load identity stamp");
    let operation = 0x51;
    PluginApp::begin_document_archive_load(&mut pure, operation, protocol::DocumentArchivePack { parent_pack: files.pack.clone(), parent_spr, members: Vec::new() }).expect("the archive load is admitted");
    let mut polls = 0;
    let status = loop {
        let status = PluginApp::poll_document_archive_load(&mut pure, operation, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the archive load status");
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status;
        }
        polls += 1;
        assert!(polls < 1_000_000, "the archive load reaches a terminal state");
    };
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "the archive load lands: {}", if status.fault.is_empty() { String::new() } else { semio_framework_diagnostic::decode_fault_bytes(&status.fault).describe() });
    PluginApp::acknowledge_document_archive_load(&mut pure, operation).expect("the terminal load is released");
    assert!(!pure.time_travel.history_unavailable() && pure.history_snapshot().await.is_ok(), "the archive load with history lifts head-only mode");
    assert_eq!((pure.stores.store.snapshot().expect("the loaded head"), pure.stores.store.envelope().vcs.edits.len()), (head, source.stores.store.envelope().vcs.edits.len()), "the archive load lands the source head and its whole history (seed + 240 edits)");
    close(&mut pure,cx);
    close(&mut source,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🩹️HistoryViewPatch

//#region ⏱️WallDeadline
thread_local! {
    /// ⏱️ A clock on this test thread that advances one millisecond per read: every replayed operation "costs" 1 ms.
    static COUNTED_CLOCK_US: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// ⏱️ Reads [`COUNTED_CLOCK_US`], advancing it by one millisecond.
fn counted_clock() -> Option<u64> {
    Some(COUNTED_CLOCK_US.with(|now| {
        now.set(now.get() + 1_000);
        now.get()
    }))
}

/// ⚖️ LAW (design §20.14, audit W2A-3): a deferred history step ends its driver turn at the turn's wall deadline, never at
/// the operation cap — with every clock read costing 1 ms, an interior undo over 600 downstream edits replays a handful of
/// them per 4 ms turn (well under the 256-operation cap), over many turns, and still lands.
#[semio_framework_async_macros::async_test]
async fn a_deferred_history_step_ends_its_turn_at_the_wall_deadline_not_an_operation_count() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = "author".to_string();
    let mut app = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
    app.time_travel.set_turn_clock(counted_clock);
    app.stores.store.defer_local_replays(Some(store::os_store::store_schema::ReplayTurnBudget { wall_us: TIME_TRAVEL_TURN_WALL_US, operations: time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, now_us: counted_clock }));
    bury_edit_of(&mut app, &actor, 600).await;
    history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert!(app.stores.store.local_step_pending(), "the interior undo waits for its replay");
    let mut turns = 0usize;
    while app.stores.store.local_step_pending() {
        let done = app.stores.store.reprojection_progress().map_or(0, |progress| progress.done);
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
        collect_original_history_command_receipt(&mut app);
        while app.take_typed_operation_ui_progress().is_some() {}
        let after = app.stores.store.reprojection_progress().map_or(u32::MAX, |progress| progress.done);
        assert!(after == u32::MAX || after.saturating_sub(done) <= 8, "a 4 ms turn of 1 ms operations replays a handful: {done} → {after}");
        turns += 1;
        assert!(turns < 1_000, "the step is adopted");
    }
    assert!(turns >= 600 / 8, "600 operations of 1 ms span many 4 ms turns ({turns})");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion ⏱️WallDeadline
