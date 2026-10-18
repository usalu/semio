// 🔄️ Opened instance, edit drafts and withdraw: the opened instance actor, a remote edit while editing, the session edge scope and the mutation row withdraw and restore.

//#region ⏱️WallDeadline
/// ⚖️ LAW (design §22.6, runtime half of §22.34): an opened instance acts as its admitted actor — at its open, across a
/// reload and on the revert route.
/// - construction binds the admitted actor before genesis; [`PluginApp::bind_actor`] verifies that same authority;
/// - a whole-document reload keeps the instance's actor although the loaded history ends in another author's edit, so an
///   undo there takes nothing back; a head-only pure hydrate keeps it too;
/// - the revert route acts as the reverting actor: the store acts as that actor afterwards and another actor's edit stays
///   applied; an undo takes back the acting actor's own edit.
#[semio_framework_async_macros::async_test]
async fn an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("ada".into()), mounted_policy, &mut identity).await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId("ada".into()), "construction binds the admitted actor");
    PluginApp::bind_actor(&mut app, "ada").await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId("ada".into()), "the admitted actor is the instance's");
    publish_as(&mut app, "ada", SetCount { value: 1 }.into());
    app.render_ctx().refresh_cache().await.expect("the log backfills the instance's own edit");
    publish_as(&mut app, "grace", SetCount { value: 2 }.into());
    app.render_ctx().refresh_cache().await.expect("the log backfills the other actor's edit");
    assert_eq!(edit_authors(&app), [Some("ada".to_string()), Some("grace".to_string())], "each edit names the actor that published it");
    let other = app.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).expect("the other actor's edit");

    let mut reader = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into()), mounted_policy, &mut identity).await;
    PluginApp::bind_actor(&mut reader, "reader").await;
    artifact_app_laws::load_document(&mut reader, &app.document_pack().await.expect("document pack"), &mut identity).await.expect("pack reload");
    reader.render_ctx().refresh_cache().await.expect("the reloaded log backfills");
    assert_eq!((reader.store.local_actor_id(), head(&reader).0), (&protocol::ActorId("reader".into()), 2), "a reload keeps the instance's actor although the history ends in another author's edit");
    history_verb(&mut reader, "reader", "undo", None, mounted_policy, &mut identity).await;
    assert_eq!((head(&reader).0, reader.store.applied_edit_ids().len()), (2, 2), "an undo never takes back another author's edit");
    let pure_head = reader.store.snapshot().expect("the reloaded head").encode_pack();
    PluginApp::hydrate_pure_head(&mut reader, &pure_head).await.expect("the head-only lane hydrates");
    assert_eq!(reader.store.local_actor_id(), &protocol::ActorId("reader".into()), "a head-only hydrate keeps the instance's actor");

    let own = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().find(|row| row.author.as_deref() == Some("ada")).expect("the instance's own row").seq;
    history_verb(&mut app, "ada", "revertToCommand", Some(DslValue::object([("entrySeq".to_string(), DslValue::uint(own))])), mounted_policy, &mut identity).await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId("ada".into()), "the revert route acts as the reverting actor");
    assert!(app.stores.store.applied_edit_ids().iter().any(|id| *id == other), "a revert never takes back another actor's edit");
    history_verb(&mut app, "grace", "undo", None, mounted_policy, &mut identity).await;
    pump_until(&mut app, "the undo lands", |app| !app.stores.store.local_step_pending(), mounted_policy, &mut identity,cx).await;
    assert!(!app.stores.store.applied_edit_ids().iter().any(|id| *id == other), "an undo takes back the acting actor's own edit");
    close(&mut reader,cx);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion ☑️LongOptionRows

//#region 📡️RemoteEditWhileEditing
/// ⚖️ LAW (live fault F4, session half; design §7: remote ingests keep arriving → `BaseMoved`): a remote edit ingested
/// while a draft is being edited moves the base and nothing else. The session stays `Editing` with its draft, its editor
/// and its generation, so an input stamped with the generation the editor's controls carried before the base move is
/// not stale; the remote edit is listed downstream of the edited mutation as not applied while editing, and nothing
/// waits for a replay (a tail edit is adopted at ingest); Accept replays the remote edit too — the reviewed head holds
/// the draft and the remote edit — and the overwrite converges on the other replica.
#[semio_framework_async_macros::async_test]
async fn a_remote_edit_while_editing_keeps_the_draft_and_accept_replays_it_too() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "remote-while-editing-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "remote-while-editing-remote", "remote", false, mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut local, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let (target, generation) = (seeded_mutation(&local, 1), local.time_travel.session().generation);
    let drafted = local.time_travel.session().pending.clone().expect("a pending draft");

    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }, &mut identity).await.expect("the remote edit");
    relay_adopted(&mut remote_probe, &mut local_probe, &mut local, mounted_policy, &mut identity).await;
    pump_until(&mut local, "the base move reaches the session", |app| app.time_travel.session().base.content_revision == app.stores.store.content_revision(), mounted_policy, &mut identity,cx).await;
    let arrived = local.stores.store.mutation_ops().expect("applied operations").last().map(|op| op.mutation_id.0.clone()).expect("the remote operation");
    assert!(arrived != seeded_mutation(&local, 3), "the remote edit is applied in this replica's store");

    let session = local.time_travel.session();
    assert_eq!((session.stage, session.generation, session.pending.as_ref()), (TimeTravelStage::Editing, generation, Some(&drafted)), "the base move keeps the stage, the generation and the draft");
    assert_eq!(local.time_travel.editor().map(|editor| (editor.target.0.clone(), editor.value.get("value").and_then(DslValue::as_str).map(str::to_string))), Some((target.clone(), Some("b".to_string()))), "the editor keeps its drafted value");
    assert!(local.render_ref().reprojection_status().is_none(), "a tail edit is adopted at ingest: nothing waits for a replay");
    let wire = local.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().flat_map(|row| row.mutations).find(|mutation| mutation.mutation_id == arrived).expect("the remote edit is a history row");
    assert!(wire.pending && wire.label.resolve(Terminology::Native, Locale::En) == "Set count to 9", "the remote edit is downstream and not applied in the preview: {wire:?}");
    let seq = local.render_ctx().history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == arrived)).map(|entry| entry.seq).expect("the history row of the remote edit");
    let body = render_history_window(&mut local, seq, 0, 8).await;
    let row = find_node(&body, &format!("framework.history.mutation.{arrived}")).unwrap_or_else(|| panic!("the mutation row of the remote edit: {body}"));
    assert!(row.to_string().contains("Not applied while editing"), "the row says so in words: {row}");
    assert!(render_body(&mut local).await.contains("count=1 label=b"), "the preview stays the state before the edited mutation with the draft");

    let typed = verb(&mut local, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(generation)))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&typed), None, "the user's next input, stamped with the generation from before the base move, is not stale");

    let accepted = verb(&mut local, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&accepted), None, "{:?}", accepted.output);
    pump_until(&mut local, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    let status = local.time_travel.status().expect("a reviewing session");
    assert_eq!((status.review, status.blocking, status.accepted_count), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, 1), "the replay over the remote edit is clean");
    assert!(render_body(&mut local).await.contains("count=9 label=c"), "the reviewed head holds the draft and the remote edit");

    for (action, args) in [("historyEditFinalize", Vec::new()), ("historyEditCommit", vec![("choice".to_string(), DslValue::String("overwrite".into()))])] {
        let result = verb(&mut local, &fixture, action, args, mounted_policy, &mut identity).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    assert_eq!(head(&local), (9, "c".to_string()), "the overwrite lands on the remote edit");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), head(&local), "the other replica adopts the history edit");
    pump_until(&mut local, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 📡️RemoteEditWhileEditing

//#region 🎞️SessionEdgeScope
/// ⚖️ LAW (live probe: the `choosing` edge; design §20.14, per-render work is O(change)): a history-edit verb re-publishes
/// every window body only when the document a window shows was swapped — opening the session (committed → preview), a
/// draft (a new preview), a finished replay (preview → head), leaving (→ committed). The edges that swap nothing —
/// an accepted draft starting its replay, the finalize prompt opening and closing, a replay run again — re-publish the
/// history body alone; and a replay run again from a review keeps showing the head reviewed last, never the committed
/// document.
#[semio_framework_async_macros::async_test]
async fn a_session_edge_republishes_every_window_only_when_the_shown_document_swaps() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let full = |result: &InvocationResult| matches!(result.ui_scope, UiDirtyScope::Full);
    let history_only = |result: &InvocationResult| matches!(&result.ui_scope, UiDirtyScope::Partial { window_bodies, panel_bodies, .. } if window_bodies.is_empty() && panel_bodies.len() == 1 && panel_bodies[0] == FRAMEWORK_HISTORY_BODY_KEY);
    let mutation = seeded_mutation(&app, 1);
    let begun = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))], mounted_policy, &mut identity).await;
    assert!(rejected(&begun).is_none() && full(&begun), "opening shows the preview in every window: {:?}", begun.ui_scope);
    let drafted = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("b".into()))], mounted_policy, &mut identity).await;
    assert!(rejected(&drafted).is_none() && full(&drafted), "a draft is a new preview: {:?}", drafted.ui_scope);
    let accepted = verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&accepted).is_none() && history_only(&accepted), "an accepted draft keeps its preview while it replays: {:?}", accepted.ui_scope);
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the preview stays while the replay runs");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let prompted = verb(&mut app, &fixture, "historyEditFinalize", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&prompted).is_none() && history_only(&prompted), "the finalize prompt swaps no document: {:?}", prompted.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Choosing);
    let back = verb(&mut app, &fixture, "historyEditBack", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&back).is_none() && history_only(&back), "leaving the prompt swaps no document: {:?}", back.ui_scope);

    let first = seeded_mutation(&app, 0);
    for (action, args) in [("historyEditBegin", vec![("mutationId".to_string(), DslValue::String(first))]), ("historyEditInput", vec![("path".to_string(), DslValue::String("/value".into())), ("value".to_string(), DslValue::uint(7))]), ("historyEditAccept", Vec::new())] {
        let result = verb(&mut app, &fixture, action, args, mounted_policy, &mut identity).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&cancelled).is_none() && full(&cancelled), "a cancelled replay leaves its preview for the head reviewed last: {:?}", cancelled.ui_scope);
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the cancelled review shows the head reviewed last");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&rerun).is_none() && history_only(&rerun), "a replay run again keeps the head reviewed last: {:?}", rerun.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = render_body(&mut app).await;
    assert!(replaying.contains("count=5 label=b") && !replaying.contains(text(&fixture["committedBody"])), "no window falls back to the committed document while it replays again: {replaying}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let exited = verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&exited).is_none() && full(&exited), "leaving shows the committed document in every window: {:?}", exited.ui_scope);
    assert!(render_body(&mut app).await.contains(text(&fixture["committedBody"])));
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🎞️SessionEdgeScope

//#region 🚫️RowWithdraw
/// 📍️ The rendered mutation row of seeded edit `index`, its history row opened as a host window shows it.
async fn seeded_mutation_node(app: &mut ToyApp, index: usize) -> Value {
    let id = seeded_mutation(app, index);
    let seq = app.render_ctx().history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == id)).map(|entry| entry.seq).expect("the history row of the seeded edit");
    let body = render_history_window(app, seq, 0, 8).await;
    find_node(&body, &format!("framework.history.mutation.{id}")).cloned().unwrap_or_else(|| panic!("the mutation row of seeded edit {index}: {body}"))
}

/// ⚖️ LAW (design §22.1, coordinator decision "Restore"): every applied mutation row offers Withdraw beside Edit on the one
/// row target. A row's Withdraw opens the session on a withdrawn draft (the preview is the state before the mutation, the
/// store untouched); on the mutation being edited it stays the editor's own while every other row names why it is blocked;
/// a running replay refuses it naming why; a mutation holding an accepted draft offers Restore in Withdraw's place and is
/// no longer withdrawable on the wire; restoring the only accepted draft leaves history editing with zero trace.
#[semio_framework_async_macros::async_test]
async fn a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let generation = app.stores.store.generation();
    let (first, target) = (seeded_mutation(&app, 0), seeded_mutation(&app, 3));
    let enabled = |verb: &str, label: &str| (verb.to_string(), label.to_string(), false, None);
    let idle = seeded_mutation_node(&mut app, 3).await;
    assert_eq!(row_actions(&idle), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "Edit stays first, Withdraw follows: {idle}");
    assert_eq!((idle["component"]["target"]["args"]["mutationId"].as_str(), idle["component"]["target"]["activation"].as_str()), (Some(target.as_str()), Some("historyEditBegin")), "one row target for both actions: {idle}");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.editable && wire.withdrawable && !wire.withdrawn, "the wire row is withdrawable");

    let withdrawn = app.handle_action("historyEditWithdraw", Some(&dsl(&idle["component"]["target"]["args"])), &meta(&fixture), &mut identity).await.expect("the row's Withdraw dispatches");
    assert_eq!(rejected(&withdrawn), None, "{:?}", withdrawn.output);
    let session = app.time_travel.session();
    assert_eq!((session.stage, session.pending.as_ref().map(|pending| (pending.target.mutation.0.clone(), pending.replacement.clone()))), (TimeTravelStage::Editing, Some((target.clone(), protocol::InputReplacement::Withdrawn))), "the session opens on a withdrawn draft");
    assert_eq!((render_body(&mut app).await.contains("count=1 label=a"), app.stores.store.generation()), (true, generation), "the preview is the state before the mutation; the store is untouched");
    let editing = row_actions(&seeded_mutation_node(&mut app, 3).await);
    assert_eq!((editing[0].2, editing[0].3.as_deref().is_some_and(|reason| reason.starts_with("Blocked")), &editing[1]), (true, true, &enabled("historyEditWithdraw", "Withdraw")), "the withdrawn draft blocks Edit; Withdraw stays the editor's own: {editing:?}");
    let other = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(other.iter().all(|action| action.2 && action.3.as_deref().is_some_and(|reason| reason.starts_with("Blocked"))), "another row names the open draft: {other:?}");
    let blocked = verb(&mut app, &fixture, "historyEditWithdraw", vec![("mutationId".into(), DslValue::String(first.clone()))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&blocked), Some("timeTravel.blocked"), "the verb refuses what the row disables");

    verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(replaying.iter().all(|action| action.2 && action.3.as_deref() == Some("Not possible right now")), "a running replay disables Edit and Withdraw, naming why: {replaying:?}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    let status = app.time_travel.status().expect("a session is open");
    assert_eq!((status.accepted_count, status.blocking, status.next_problem.is_none()), (1, false, true), "the withdrawal replays clean");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 3).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditRestore", "Restore")], "an accepted draft offers Restore in Withdraw's place");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 0).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "the review withdraws other rows again");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.withdrawn && !wire.withdrawable && wire.edited, "the replayed row reads withdrawn and is no longer withdrawable");
    let unaccepted = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(first.clone()))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&unaccepted), Some("timeTravel.illegal"), "a mutation without an accepted draft has nothing to restore");

    let restored = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(target.clone()))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&restored), None, "{:?}", restored.output);
    assert!(app.time_travel.status().is_none(), "restoring the only accepted draft leaves history editing");
    assert_eq!((render_body(&mut app).await.contains(text(&fixture["committedBody"])), app.stores.store.generation()), (true, generation), "zero trace: the committed document, the store untouched");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🚫️RowWithdraw
