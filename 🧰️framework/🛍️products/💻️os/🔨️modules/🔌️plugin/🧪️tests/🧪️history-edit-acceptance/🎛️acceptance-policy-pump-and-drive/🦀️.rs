//! 🎛️ Acceptance policy, pump and drive: the policy, meta and verb helpers, the pump, close, seeded document, head and the drive of one leaf. See [`super`].

use super::*;

/// 🎟️ The explicit mounted owner policy every acceptance instance runs under: the production interactive component grant.
pub(super) fn acceptance_policy() -> semio_plugin_mounted_owner::MountedOwnerPolicyV1 {
    semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT
}

/// 🎟️ Establishes the original decoder receipt required to enter mounted command dispatch.
pub(super) fn acceptance_context(caller: &mut artifact_app_laws::OriginalFixtureCaller) -> semio_framework_job::StepContext<'_> {
    let mut cx = caller.context();
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        let mut scope = None;
        if cx.with_native_receiving(&mut scope, |_, _| Ok(())).expect("original acceptance decoder receiving scope") {
            let outcome = scope.expect("original acceptance decoder receipt");
            assert!(outcome.source_refusal.is_none() && outcome.call.is_ok(), "original acceptance decoder was refused");
            artifact_app_laws::fixture_end_turn(&mut cx).expect("original acceptance decoder receipt settles");
            assert!(cx.native_receiving_identity().is_some(), "mounted commands require the accepted original decoder identity");
            return cx;
        }
    }
    panic!("original acceptance decoder exceeded its supplied turns");
}

/// 🎬️ Funds a complete independent acceptance scenario through its original caller and terminal decoder.
pub(super) async fn with_acceptance_caller<T>(run: impl for<'turn> AsyncFnOnce(&mut semio_framework_job::StepContext<'turn>) -> T) -> T {
    let mut caller = artifact_app_laws::OriginalFixtureCaller::admit().expect("original acceptance caller admission");
    let mut context = acceptance_context(&mut caller);
    let result = run(&mut context).await;
    let fuel = context.fuel_remaining();
    drop(context);
    caller.retire(fuel).expect("original acceptance caller paid retirement");
    result
}

/// 🪪️ The acceptance actor's action meta on the registered instance.
pub(super) fn acceptance_meta() -> ActionMeta {
    ActionMeta { view_state: Some(semio_plugin_testing_laws::receive_view_fixture(ViewModel::new(Locale::En, Terminology::Native), semio_plugin_testing_laws::OriginalWorkFixture::stock())), ..artifact_app_laws::meta(ACCEPTANCE_ACTOR) }
}

/// ⏯️ Retains one framework history-edit input across its original supplied turns.
pub(super) async fn acceptance_verb<A, M>(app: &mut VcsArtifactApp<A, M>, action: &str, args: Vec<(&str, DslValue)>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), AcceptanceError>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let args = DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let meta = ActionMeta { actor: app.stores.store.local_actor_id().0.as_str().into(), ..acceptance_meta() };
    let result = semio_plugin_testing_laws::with_authoring_identity!(|identity|
        artifact_app_laws::dispatch_fixture_action(app, action, Some(args), &meta, &mut identity, cx, acceptance_policy().close).await)
        .map_err(|fault| AcceptanceError::from(fault).context(format!("{action} faulted")))?;
    match result.output.get("rejected").and_then(DslValue::as_str) {
        Some(code) => Err(format!("{action} was refused: {code}").into()),
        None => Ok(()),
    }
}

/// ⏱️ Drives reactor turns within the original caller's fixed deadline, fuel and supplied turn bound.
pub(super) async fn acceptance_pump<A, M>(app: &mut VcsArtifactApp<A, M>, done: impl Fn(&VcsArtifactApp<A, M>) -> bool,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), AcceptanceError>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let entry_grant = cx.retained_grant();
    let (grant, _) = cx.original_turn_ceiling();
    let fuel = artifact_app_laws::OriginalFixtureCaller::turn_fuel();
    let work_before = cx.original_work_units_spent();
    let mut receipts = 0usize;
    let mut empty_receipts = 0usize;
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        if let Some((issued, progress)) = app.take_history_command_receipt() {
            assert!(progress.fits(issued), "history command receipt exceeds its supplied original grant");
            receipts += 1;
            empty_receipts += usize::from(progress == RetainedCloneProgress::default());
        }
        if done(app) {
            return Ok(());
        }
        artifact_app_laws::fixture_renew_turn(cx, fuel, grant).map_err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?;
        semio_plugin_testing_laws::with_authoring_identity!(|identity| app.advance_typed_operation_publication(&mut identity,cx).await)?;
        while app.take_typed_operation_ui_progress().is_some() {}
        artifact_app_laws::retain_original_ownership_receipts(app, acceptance_policy(), &mut Vec::new()).map_err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?;
    }
    let status = app.time_travel.status().map(|status| (status.stage, status.done, status.total));
    let maintenance = acceptance_policy().maintenance;
    let demands = app.time_travel.owners().map(|owner| owner.retirement_demands(maintenance.maximum_copy_bytes)).collect::<Vec<_>>();
    let work = cx.original_work_units_spent().saturating_sub(work_before);
    let operations = app.typed_operation_diagnostic();
    Err((format!(
        "the instance never settled within its original turn bound. History stage and progress {status:?}, preview progress {:?}, replay owner {}, processed replay items {:?}, pending history work {}, and pending typed operations {}. Entry grant {entry_grant:?}, renewed grant {grant:?}. Retired owner demands {demands:?}. Work {work}, history receipts {receipts}, empty receipts {empty_receipts}. {operations}",
        app.time_travel.preview_progress(),
        app.time_travel.replaying(),
        app.time_travel.processed(),
        app.time_travel.has_pending_work(),
        app.has_pending_typed_operations(),
    )).into())
}

/// 🧹️ Lets the session retire its owners, then closes the instance to its terminal-empty witness.
pub(super) async fn acceptance_close<A, M>(app: &mut VcsArtifactApp<A, M>,cx:&mut semio_framework_job::StepContext<'_>)
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let failed=Box::pin(acceptance_pump(app, |app| !app.time_travel.has_pending_work(),cx)).await.err();
    if let Err(close) = artifact_app_laws::close_registered_fixture_app(app, acceptance_policy(), cx) {
        if let Some(error) = failed {
            panic!("original acceptance pump failed with {error}; terminal close also refused with {close:?}");
        }
        panic!("original acceptance app terminal close failed: {close:?}");
    }
    if let Some(error) = failed {
        panic!("original acceptance driver failed before app close: {error}");
    }
}

/// 🪴️ A registered instance whose document is `base` (loaded through the document text path unless the instance boots on exactly
/// that text: its own initial document keeps the members it owns) followed by one clean edit per op
/// of `ops` (each leaving an applied operation when `strict`); `Err` names why the data cannot be seeded (the base, or the index
/// of the first op that does not apply cleanly), after closing the instance.
pub(super) async fn acceptance_seeded<A, M>(manifest: fn() -> App, base: &str, ops: &[&A::Mutation], strict: bool,cx:&mut semio_framework_job::StepContext<'_>) -> Result<VcsArtifactApp<A, M>, AcceptanceSeedFault>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let mut app = semio_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into()), acceptance_policy(), &mut identity).await);
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.into()));
        let mut files = app.document_text().await.map_err(|fault| AcceptanceSeedFault::Original(AcceptanceError::from(fault).context("the document does not print".into())))?;
        if files.dsl != base {
            files.dsl = base.to_string();
            semio_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::load_document_text(&mut app, &files, &mut identity,cx).await).map_err(|fault| AcceptanceSeedFault::Original(AcceptanceError::from(fault).context("the base document does not load".into())))?;
        }
        for (index, op) in ops.iter().enumerate() {
            let applied = app.stores.store.mutation_ops().map_or(0, |applied| applied.len());
            let (fuel, deadline) = (cx.fuel_remaining().min(1 << 24), cx.original_absolute_deadline_us());
            let mut dispatched = None;
            cx.with_original_work_loan(fuel, deadline, |loaned| {
                let mut work = loaned.native_encoder_work();
                let mut observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
                let observer: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut observer;
                let mut identity = semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, observer).expect("acceptance authoring identity admission").with_work_lender(Box::new(artifact_app_laws::CallerIdentityWork(&mut work)));
                dispatched = Some(block_on_acceptance(app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![(*op).clone()], transaction: None }, &mut identity)));
                drop(identity);
                loaned.consume_fuel(work.performed());
                Ok(())
            })
            .map_err(|failure| AcceptanceSeedFault::Operation(index, format!("the seed edit has no original work loan: {failure:?}")))?;
            let receipt = dispatched.expect("the seed dispatch ran inside its loan").map_err(|error| AcceptanceSeedFault::Operation(index, format!("the seed edit is refused: {error:?}")))?;
            if matches!(receipt.worst, Some(semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
                return Err(AcceptanceSeedFault::Operation(index, format!("the seed edit does not apply cleanly: {:?}", receipt.messages)));
            }
            if strict && app.stores.store.mutation_ops().map_or(0, |applied| applied.len()) <= applied {
                return Err(AcceptanceSeedFault::Operation(index, "the seed edit leaves no applied operation".into()));
            }
        }
        app.render_ctx().refresh_cache().await.map_err(|fault| AcceptanceSeedFault::Original(AcceptanceError::from(fault).context("the history does not backfill".into())))
    }
    .await;
    match seeded {
        Ok(()) => Ok(app),
        Err(reason) => {
            Box::pin(acceptance_close(&mut app,cx)).await;
            Err(reason)
        }
    }
}

/// 🧾️ The document head as its canonical value and its DSL text (the text also carries what a composed parent's value form
/// only references — its owned children); the clone is never dropped, so a fail-closed root never trips.
pub(super) fn acceptance_head<A, M>(app: &VcsArtifactApp<A, M>) -> Result<(DslValue, String), AcceptanceError>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let head = app.stores.store.snapshot().map_err(|error| format!("the head does not fold: {error:?}"))?;
    let value = (semio_framework_value::ToValue::to_value(&head), pack::snapshot::ArtifactDsl::print_dsl(&head));
    std::mem::forget(head);
    Ok(value)
}

/// 🔃️ Exercises successive mounted inputs before the original selected preview finishes and returns the last rendered snapshot.
pub async fn assert_latest_history_preview<A, M>(manifest: fn() -> App, fixtures: &std::path::Path, changes: &[(&str, DslValue)]) -> DslValue
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let cases = acceptance_cases::<A>(fixtures);
    assert_eq!(cases.len(), 1, "the preview law selects one committed wire witness");
    let case = cases.into_iter().next().unwrap();
    with_acceptance_caller(async |cx| {
        let seeded = Box::pin(acceptance_seeded::<A, M>(manifest, &case.base, &[&case.op], true, cx)).await;
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
        let mut app = match seeded {
            Ok(app) => app,
            Err(AcceptanceSeedFault::Operation(_, reason)) => panic!("{reason}"),
            Err(AcceptanceSeedFault::Original(reason)) => panic!("{reason}"),
        };
        let result = async {
            let target = app.stores.store.mutation_ops().unwrap().first().unwrap().mutation_id.0.clone();
            let begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target))];
            Box::pin(acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(), cx)).await?;
            assert!(app.time_travel.preview_progress().is_some(), "the first original preview is still active");
            for (path, value) in changes {
                let args = vec![("path", DslValue::String((*path).to_owned())), ("value", value.clone())];
                Box::pin(acceptance_verb(&mut app, HISTORY_EDIT_INPUT_ACTION_ID, args, cx)).await?;
                assert!(app.time_travel.editor().is_some_and(|editor| editor.refused.is_none()));
            }
            Box::pin(acceptance_pump(&mut app, |app| !app.time_travel.has_pending_work(), cx)).await?;
            let committed = app.stores.store.snapshot_owner();
            let shown = app.time_travel.render_snapshot_or(&app.tool_runs, &committed);
            let latest = semio_framework_value::ToValue::to_value(shown.as_ref());
            Box::pin(acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(), cx)).await?;
            Box::pin(acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(), cx)).await?;
            Box::pin(acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(), cx)).await?;
            assert!(app.time_travel.preview_progress().is_some());
            Box::pin(acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(), cx)).await?;
            Box::pin(acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(), cx)).await?;
            Box::pin(acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin, cx)).await?;
            let (path, value) = changes.first().expect("a draft to defer before closing");
            let args = vec![("path", DslValue::String((*path).to_owned())), ("value", value.clone())];
            Box::pin(acceptance_verb(&mut app, HISTORY_EDIT_INPUT_ACTION_ID, args, cx)).await?;
            assert!(app.time_travel.has_pending_work() || app.time_travel.preview_progress().is_some(), "the deferred draft keeps its preview owned");
            assert_eq!(app.time_travel.status().unwrap().stage, HistoryTimeTravelStage::Editing);
            Ok::<_, AcceptanceError>(latest)
        }.await;
        artifact_app_laws::close_registered_fixture_app(&mut app, acceptance_policy(), cx)
            .unwrap_or_else(|failure| panic!("mounted preview fixture could not close its live editor: {failure:?}"));
        result.expect("mounted latest preview and exit")
    }).await
}

/// ↪️ Checks one explicit History overwrite in a mounted fixture that produced its own committed mutation.
pub async fn assert_mounted_history_overwrite<A, M>(app: &mut VcsArtifactApp<A, M>, mutation_id: &str, path: &str, value: DslValue, cx: &mut semio_framework_job::StepContext<'_>)
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let change = (path.to_owned(), value);
    match Box::pin(acceptance_drive(app, mutation_id, None, Some(&change), None, None, None, cx)).await {
        Ok(applied) => assert_eq!(applied, change),
        Err(AcceptanceVerdict::Fail(reason)) => panic!("mounted History overwrite failed: {reason}"),
        Err(AcceptanceVerdict::Pass(reason) | AcceptanceVerdict::Skip(reason) | AcceptanceVerdict::SkipCase(reason) | AcceptanceVerdict::SkipBase(reason)) => panic!("mounted History overwrite did not run: {reason}"),
    }
}

/// 🚗️ One session over the mutation `target` of the member `store` (`<slot>/<childId>`, design §12; `None`: the document's own
/// store): begin, draft `change` (else the first change of [`acceptance_changes`] the editor accepts AND that changes the operation:
/// accepting a draft that rebuilds the same operation closes time travel without a trace, by design, and the next one is tried on a
/// re-opened session), accept, replay to a clean
/// review, finalize, and commit as an overwrite or as the new alternative `alternative`. With `as_of` (the document folded up to
/// and including `target`), what every render seam reads when the preview settles must be exactly that: the preview applies no
/// downstream mutation. With `kind` (and no `change`) only changes of that control kind are drafted. Answers the change drafted.
#[allow(clippy::too_many_arguments)]
pub(super) async fn acceptance_drive<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    target: &str,
    store: Option<&str>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: Option<&(DslValue, String)>,
    kind: Option<usize>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(String, DslValue), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let committed_revision = app.stores.store.content_revision_now();
    let mut begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.to_string()))];
    begin.extend(store.map(|store| (HISTORY_EDIT_ARG_STORE, DslValue::String(store.to_string()))));
    Box::pin(acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(),cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let (inputs, original) = match app.time_travel.editor() {
        None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
        Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail((format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused)).into())),
        Some(editor) => (editor.inputs.clone(), editor.value.clone()),
    };
    if let Some(as_of) = as_of {
        Box::pin(acceptance_pump(app, |app| !app.time_travel.has_pending_work(), cx)).await
            .map_err(|reason| AcceptanceVerdict::Fail(reason.context("the initial history preview does not settle".into())))?;
        let committed = app.stores.store.snapshot_owner();
        let shown = app.time_travel.render_snapshot_or(&app.tool_runs, &committed);
        let shown = (semio_framework_value::ToValue::to_value(shown.as_ref()), pack::snapshot::ArtifactDsl::print_dsl(shown.as_ref()));
        if shown != *as_of {
            let _ = Box::pin(acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx)).await;
            return Err(AcceptanceVerdict::Fail((format!("while editing, the preview is not the document as of the edited mutation (downstream not applied) at {}", acceptance_text_difference(&shown.1, &as_of.1))).into()));
        }
    }
    let changes = match (change, kind) {
        (Some(change), _) => vec![change.clone()],
        (None, Some(kind)) => acceptance_change_buckets(&inputs, &original).into_iter().nth(kind).unwrap_or_default().into_iter().take(ACCEPTANCE_CHANGES_PER_LEAF).collect(),
        (None, None) => acceptance_changes(&inputs, &original),
    };
    let (mut drafted, mut unchanged) = (None, 0);
    for candidate in changes {
        if app.time_travel.status().is_none() {
            Box::pin(acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(),cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        }
        if Box::pin(acceptance_verb(app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())],cx)).await.is_err()
            || !app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
        {
            continue;
        }
        Box::pin(acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new(),cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        Box::pin(acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying),cx)).await
            .map_err(|reason| AcceptanceVerdict::Fail(reason.context("after accepting history replacement".into())))?;
        if change.is_none() && app.time_travel.status().is_some_and(|status| status.review == Some(HistoryTimeTravelReview::Blocked)) {
            Box::pin(acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
            Box::pin(acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
            if store.is_none() && app.stores.store.supersessions().iter().any(|(id, _)| id.0 == target) {
                return Err(AcceptanceVerdict::Fail("discarding a blocked trial left a supersession of the mutation".into()));
            }
            continue;
        }
        if app.time_travel.status().is_some() {
            drafted = Some(candidate);
            break;
        }
        if store.is_none() && app.stores.store.supersessions().iter().any(|(id, _)| id.0 == target) {
            return Err(AcceptanceVerdict::Fail((format!("accepting {} = {:?}, which rebuilds the same operation, closed the session but left a supersession of the mutation", candidate.0, candidate.1)).into()));
        }
        unchanged += 1;
    }
    let Some(drafted) = drafted else {
        if app.time_travel.status().is_some() {
            let _ = Box::pin(acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx)).await;
        }
        return Err(AcceptanceVerdict::Skip(format!(
            "no schema-valid{} change of its {} input(s) is accepted ({unchanged} left the operation unchanged)",
            kind.map_or(String::new(), |kind| format!(" {}", ACCEPTANCE_CONTROL_KINDS[kind])),
            inputs.len()
        )));
    };
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting closed the session".into()))?;
    if status.review == Some(HistoryTimeTravelReview::Blocked) {
        let _ = Box::pin(acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx)).await;
        return Err(AcceptanceVerdict::Skip(format!("drafting {} = {:?} blocks the replay ({:?})", drafted.0, drafted.1, status.worst)));
    }
    if (status.stage, status.review) != (HistoryTimeTravelStage::Reviewing, Some(HistoryTimeTravelReview::Ready)) {
        return Err(AcceptanceVerdict::Fail((format!("the replay ends at {:?}/{:?}, not a ready review", status.stage, status.review)).into()));
    }
    if app.stores.store.content_revision_now() != committed_revision {
        return Err(AcceptanceVerdict::Fail("History review changed the committed document before finalization".into()));
    }
    Box::pin(acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new(),cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let commit = match alternative {
        Some(name) => vec![("name", DslValue::String(name.to_string()))],
        None => vec![("choice", DslValue::String(HISTORY_EDIT_CHOICE_OVERWRITE.to_string()))],
    };
    Box::pin(acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, commit,cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    Box::pin(acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx)).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    Ok(drafted)
}
