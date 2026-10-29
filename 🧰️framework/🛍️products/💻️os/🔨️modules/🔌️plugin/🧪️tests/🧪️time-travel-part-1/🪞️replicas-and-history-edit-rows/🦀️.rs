// 🪞️ Replicas and history edit rows: the replica relays, a history edit as its own row across replicas and reload, undo and redo of a finalize and an alternative history edit.

//#region 🪞️Replicas
/// 🪞️ A registered toy replica on its own memory backbone, authoring as `actor`; `seed` dispatches the fixture seed
/// through its store, so every seed edit leaves in its outbox.
async fn replica(fixture: &Value, uri: &str, actor: &str, seed: bool, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (ToyApp, MemoryBackbone) {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.into()), mounted_policy, identity).await;
    let (backbone, probe) = MemoryBackbone::pair(uri, uri).await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("attach the replica backbone");
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(actor.into()));
    if seed {
        for op in fixture["seed"].as_array().expect("seed") {
            app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![seed_op(op)], transaction: None }, identity).await.expect("seed edit applies");
        }
    }
    app.render_ctx().refresh_cache().await.expect("the command log backfills");
    (app, probe)
}

/// 🔀️ Delivers every event batch `from` published since the last relay into `to` (through its probe `into`) and ingests it.
#[doc(hidden)]
pub async fn relay(from: &mut MemoryBackbone, into: &mut MemoryBackbone, to: &mut ToyApp, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    for message in from.receive().await.expect("replica outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        into.send(message).await.expect("relay an event batch");
    }
    to.tick_backbone(identity).await.expect("ingest the relayed events");
    to.render_ctx().refresh_cache().await.expect("the command log backfills the relayed events");
}

/// 🔀️ [`relay`], then drives `to`'s turns until the relayed change is adopted — its replay may span turns (gap N17).
async fn relay_adopted(from: &mut MemoryBackbone, into: &mut MemoryBackbone, to: &mut ToyApp, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    relay(from, into, to, mounted_policy, identity).await;
    pump_until(to, "the relayed change is adopted", |app| app.stores.store.reprojection_progress().is_none(), mounted_policy, identity,cx).await;
    to.render_ctx().refresh_cache().await.expect("the command log backfills the adopted change");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ↩️ Runs a framework history verb (`undo`, `redo`, `revertToCommand`) as `actor` through its reserved admission.
async fn history_verb(app: &mut ToyApp, actor: &str, action: &str, args: Option<DslValue>, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let admitted = app.handle_action(action, args.as_ref(), &meta, identity).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(app, admitted, cx).await.unwrap_or_else(|fault| panic!("{action} settles: {fault:?}"));
    app.render_ctx().refresh_cache().await.expect("history refresh");
}

/// 🧾️ The history-transition rows of `app`, newest first.
async fn history_edit_rows(app: &mut ToyApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().filter(|row| row.transition_id.is_some()).collect()
}

fn english(rows: &[semio_framework::kernel::HistoryEntry]) -> Vec<&str> {
    rows.iter().map(|row| row.label.resolve(Terminology::Native, Locale::En)).collect()
}

/// ✏️ The history mutation row of seeded op `index`.
async fn seeded_mutation_row(app: &mut ToyApp, index: usize) -> semio_framework::kernel::HistoryMutationEntry {
    let target = seeded_mutation(app, index);
    app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().flat_map(|row| row.mutations).find(|mutation| mutation.mutation_id == target).expect("the seeded mutation row")
}
//#endregion 🪞️Replicas

/// ⚖️ LAW: a finalized history edit is its own history row, derived from the store's `Supersede` transition on the
/// authoring replica, on a replica that ingested it and after a text or pack reload — labelled in every locale with its
/// scope and mutation count, its author and time — and the edited mutation reads superseded from the store alike.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_is_its_own_row_locally_remotely_and_after_reload() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "history-edit-row-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "history-edit-row-remote", "remote", false, mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (5, "a".to_string()), "the remote replica holds the seed");
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone().into()), mounted_policy, &mut identity).await;
    assert_eq!(reloaded.stores.store.local_actor_id(), &protocol::ActorId(actor.clone().into()));
    artifact_app_laws::load_document_text(&mut reloaded, &local.document_text().await.expect("document text"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("text reload");
    reloaded.render_ctx().refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into()), mounted_policy, &mut identity).await;
    assert_eq!(repacked.stores.store.local_actor_id(), &protocol::ActorId("reader".into()));
    artifact_app_laws::load_document(&mut repacked, &local.document_pack().await.expect("document pack"), &mut identity).await.expect("pack reload");
    repacked.render_ctx().refresh_cache().await.expect("the repacked log backfills");
    for (who, app, revertible) in [("local", &mut local, true), ("remote", &mut remote, false), ("text reload", &mut reloaded, true), ("pack reload", &mut repacked, false)] {
        let rows = history_edit_rows(app).await;
        assert_eq!(english(&rows), ["History edited — overwrite: 1 mutation"], "{who}: one row per history edit, never a runtime duplicate");
        let row = &rows[0];
        assert_eq!(row.label.resolve(Terminology::Native, Locale::De), "Verlauf bearbeitet — überschrieben: 1 Mutation", "{who}");
        assert_eq!(row.author.as_deref(), Some(actor.as_str()), "{who}: the author");
        assert!(row.timestamp.parse::<u64>().is_ok_and(|ms| ms > 0), "{who}: the time, {}", row.timestamp);
        assert_eq!((row.kind.as_str(), row.action_id.as_str(), row.applied, row.revertible), ("history", semio_framework::history_edit::HISTORY_EDIT_COMMIT_ACTION_ID, true, revertible), "{who}");
        assert_eq!(row.key(), format!("transition:{}", row.transition_id.as_deref().expect("transition id")), "{who}: the row folds under its transition");
        let mutation = seeded_mutation_row(app, 1).await;
        assert!(mutation.superseded && !mutation.withdrawn, "{who}: the edited mutation reads superseded from the store");
        assert_eq!(head(app), (5, "b".to_string()), "{who}: the edited head");
        let history = render_history(app, Locale::De).await;
        assert!(find_node(&history, &format!("framework.history.entry.{}", row.seq)).is_some(), "{who}: the panel shows the row");
    }
    drop((local_probe, remote_probe));
    for app in [&mut local, &mut remote, &mut reloaded, &mut repacked] {
        close(app,cx);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: undo of a finalize is a new `Supersede` restoring the previous effective input (design §2), redo re-authors
/// the edit, both through the store's one supersede path and ingested alike by another replica; a newer own document
/// edit is undone first and redone last; the row's Backwards takes the edit back without touching document edits; and
/// another author can neither undo nor revert it.
#[semio_framework_async_macros::async_test]
async fn undo_and_redo_of_a_finalize_author_restoring_supersedes_on_both_replicas() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "history-edit-undo-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "history-edit-undo-remote", "remote", false, mounted_policy, &mut identity).await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let edits = local.stores.store.envelope().vcs.edits.len();

    history_verb(&mut local, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "a".to_string()), "the undo restores the original input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[0].revertible && !rows[1].applied && !rows[1].revertible, "the undone history edit reads undone");
    assert!(!seeded_mutation_row(&mut local, 1).await.superseded, "an input restored to its original is not superseded");
    assert_eq!(local.stores.store.envelope().vcs.edits.len(), edits, "the undo is a history transition, never a document edit");
    assert!(local.render_ctx().history_patch(true).await.expect("patch").can_redo, "the undone history edit can be redone");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (5, "a".to_string()), "the remote replica folds the restoring supersede");
    assert_eq!(english(&history_edit_rows(&mut remote).await), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);

    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the redo re-authors the edited input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit redone — overwrite: 1 mutation", "History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[1].applied && rows[2].applied && rows[2].revertible, "the redone history edit is in effect again");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (5, "b".to_string()));

    local.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 7 }.into()], transaction: None }, &mut identity).await.expect("a newer document edit");
    local.render_ctx().refresh_cache().await.expect("the newer edit is logged");
    history_verb(&mut local, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the newer own document edit is undone first");
    history_verb(&mut local, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "a".to_string()), "then the history edit");
    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the history edit undone last is redone first");
    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (7, "b".to_string()), "then the document edit");

    let edited = history_edit_rows(&mut local).await.into_iter().rev().find(|row| row.revertible).expect("the revertible history edit row");
    history_verb(&mut local, &actor, REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::object([("entrySeq".to_string(), DslValue::float(edited.seq as f64))])), mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (7, "a".to_string()), "Backwards takes the history edit back and leaves the document edits");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (7, "a".to_string()), "the replicas converge");

    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!((head(&local), head(&remote)), ((7, "b".to_string()), (7, "b".to_string())));
    let foreign = history_edit_rows(&mut remote).await;
    assert!(foreign.iter().all(|row| !row.revertible && row.author.as_deref() == Some(actor.as_str())), "another author's history edits are never revertible here");
    history_verb(&mut remote, "remote", "undo", None, mounted_policy, &mut identity).await;
    let original = foreign.iter().rev().find(|row| row.label.resolve(Terminology::Native, Locale::En).starts_with("History edited")).expect("the history edit row");
    history_verb(&mut remote, "remote", REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::object([("entrySeq".to_string(), DslValue::float(original.seq as f64))])), mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (7, "b".to_string()), "another author can neither undo nor revert the history edit");
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a history edit committed as a new alternative is undone and redone within that alternative: its rows name
/// the alternative, the restoring supersede is scoped to it, and the alternative stays active.
#[semio_framework_async_macros::async_test]
async fn an_alternative_history_edit_is_undone_within_its_alternative() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = replica(&fixture, "history-edit-alternative", &actor, true, mounted_policy, &mut identity).await;
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original", mounted_policy, &mut identity).await;
    let variant = app.stores.store.envelope().active_alternative_id.clone().expect("the new alternative is active");
    assert_eq!(english(&history_edit_rows(&mut app).await), ["History edited — alternative Variant: 1 mutation"]);
    history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
    pump_until(&mut app, "the alternative undo lands", |app| !app.time_travel.authoring_pending(), mounted_policy, &mut identity,cx).await;
    assert_eq!(head(&app), (5, "a".to_string()), "the undo restores the original within the alternative");
    let rows = history_edit_rows(&mut app).await;
    assert_eq!(english(&rows), ["History edit undone — alternative Variant: 1 mutation", "History edited — alternative Variant: 1 mutation"]);
    assert_eq!(rows[1].label.resolve(Terminology::Native, Locale::De), "Verlauf bearbeitet — Alternative Variant: 1 Mutation");
    assert!(
        app.stores.store.supersessions().values().all(|supersession| supersession.scope.as_deref() == Some(variant.as_str()) && Some(supersession.transition_id.as_str()) == rows[0].transition_id.as_deref()),
        "the restoring supersede is scoped to the alternative"
    );
    assert_eq!(app.stores.store.envelope().active_alternative_id.as_deref(), Some(variant.as_str()), "the alternative stays active");
    history_verb(&mut app, &actor, "redo", None, mounted_policy, &mut identity).await;
    pump_until(&mut app, "the alternative redo lands", |app| !app.time_travel.authoring_pending(), mounted_policy, &mut identity,cx).await;
    assert_eq!(head(&app), (5, "b".to_string()));
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
