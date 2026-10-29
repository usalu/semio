//! 🎛️ Acceptance policy, pump and drive: the policy, meta and verb helpers, the pump, close, seeded document, head and the drive of one leaf. See [`super`].

use super::*;

/// 🎟️ The explicit mounted owner policy every acceptance instance runs under.
pub(super) fn acceptance_policy() -> semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
    semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant }
}

/// 🪪️ The acceptance actor's action meta on the registered instance.
pub(super) fn acceptance_meta() -> ActionMeta {
    ActionMeta { view_state: Some(ViewModel::new(Locale::En, Terminology::Native)), ..artifact_app_laws::meta(ACCEPTANCE_ACTOR) }
}

/// ⏯️ Retains one framework history-edit input across its original supplied turns.
pub(super) async fn acceptance_verb<A, M>(app: &mut VcsArtifactApp<A, M>, action: &str, args: Vec<(&str, DslValue)>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), AcceptanceError>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let mut args = Some(DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect()));
    let mut meta=Some(acceptance_meta());
    let outcome=semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| async{
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            if cx.should_yield(){break}
            let result=app.handle_action(action,&mut args,&mut meta,&mut identity,cx).await.map_err(|fault|AcceptanceError::from(fault).context(format!("{action} faulted")))?;
            if let Some(result)=result{return match result.output.get("rejected").and_then(DslValue::as_str){Some(code)=>Err(format!("{action} was refused: {code}").into()),None=>Ok(())}}
        }
        Err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original acceptance action remains pending at caller opportunity limit")).into())
    }.await);
    let mut input=semio_framework_value::retirement::controlled::ControlledRetirement::new(args).unwrap_or_else(|_|panic!("original acceptance action input supports controlled retirement"));
    let loan=acceptance_policy().close;
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
        if input.terminal_is_empty(){return outcome}
        cx.with_retained_loan(loan,|same|semio_framework_job::close_original_retained_step(same,input.selected_demand(loan.maximum_copy_bytes)?,|grant|input.step(grant))).expect("original acceptance action input paid retirement");
    }
    panic!("original acceptance action input exceeded its supplied close turns")
}

/// ⏱️ Drives reactor turns within the original caller's fixed deadline, fuel and supplied turn bound.
pub(super) async fn acceptance_pump<A, M>(app: &mut VcsArtifactApp<A, M>, done: impl Fn(&VcsArtifactApp<A, M>) -> bool,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), AcceptanceError>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
        if cx.should_yield(){return Err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original acceptance driver exhausted caller opportunity")).into())}
        if done(app) {
            return Ok(());
        }
        semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| app.advance_typed_operation_publication(&mut identity,cx).await)?;
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    Err((format!(
        "the instance never settled within its original turn bound; time travel rests at {:?} (pending history work: {}, pending typed operations: {})",
        app.time_travel.status().map(|status| status.stage),
        app.time_travel.has_pending_work(),
        app.has_pending_typed_operations()
    )).into())
}

/// 🧹️ Lets the session retire its owners, then closes the instance to its terminal-empty witness.
pub(super) async fn acceptance_close<A, M>(app: &mut VcsArtifactApp<A, M>,cx:&mut semio_framework_job::StepContext<'_>)
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let failed=acceptance_pump(app, |app| !app.time_travel.has_pending_work(),cx).await.err();
    artifact_app_laws::close_registered_fixture_app(app,acceptance_policy(),cx).expect("original acceptance app terminal close");
    if let Some(failed)=failed{panic!("original acceptance driver failure after paid app close: {failed}")}
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
    let mut app = semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into()), acceptance_policy(), &mut identity).await);
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(ACCEPTANCE_ACTOR.into()));
        let mut files = app.document_text().await.map_err(|fault| AcceptanceSeedFault::Original(AcceptanceError::from(fault).context("the document does not print".into())))?;
        if files.dsl != base {
            files.dsl = base.to_string();
            semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::load_document_text(&mut app, &files, &mut identity,cx).await).map_err(|fault| AcceptanceSeedFault::Original(AcceptanceError::from(fault).context("the base document does not load".into())))?;
        }
        for (index, op) in ops.iter().enumerate() {
            let applied = app.stores.store.mutation_ops().map_or(0, |applied| applied.len());
            let receipt =
                semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![(*op).clone()], transaction: None }, &mut identity).await).map_err(|error| AcceptanceSeedFault::Operation(index, format!("the seed edit is refused: {error:?}")))?;
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
            acceptance_close(&mut app,cx).await;
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

/// 🚗️ One session over the mutation `target` of the member `store` (`<slot>/<childId>`, design §12; `None`: the document's own
/// store): begin, draft `change` (else the first change of [`acceptance_changes`] the editor accepts AND that changes the operation:
/// accepting a draft that rebuilds the same operation closes time travel without a trace, by design, and the next one is tried on a
/// re-opened session), accept, replay to a clean
/// review, finalize, and commit as an overwrite or as the new alternative `alternative`. With `as_of` (the document folded up to
/// and including `target`), what every render seam reads right after the begin must be exactly that: the preview applies no
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
    let mut begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.to_string()))];
    begin.extend(store.map(|store| (HISTORY_EDIT_ARG_STORE, DslValue::String(store.to_string()))));
    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let (inputs, original) = match app.time_travel.editor() {
        None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
        Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail((format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused)).into())),
        Some(editor) => (editor.inputs.clone(), editor.value.clone()),
    };
    if let Some(as_of) = as_of {
        let committed = app.stores.store.snapshot_owner();
        let shown = app.time_travel.render_snapshot_or(&app.tool_runs, &committed);
        let shown = (semio_framework_value::ToValue::to_value(shown.as_ref()), pack::snapshot::ArtifactDsl::print_dsl(shown.as_ref()));
        if shown != *as_of {
            let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx).await;
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
            acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        }
        if acceptance_verb(app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())],cx).await.is_err()
            || !app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
        {
            continue;
        }
        acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        if change.is_none() && app.time_travel.status().is_some_and(|status| status.review == Some(HistoryTimeTravelReview::Blocked)) {
            acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
            acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
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
            let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx).await;
        }
        return Err(AcceptanceVerdict::Skip(format!(
            "no schema-valid{} change of its {} input(s) is accepted ({unchanged} left the operation unchanged)",
            kind.map_or(String::new(), |kind| format!(" {}", ACCEPTANCE_CONTROL_KINDS[kind])),
            inputs.len()
        )));
    };
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting closed the session".into()))?;
    if status.review == Some(HistoryTimeTravelReview::Blocked) {
        let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx).await;
        return Err(AcceptanceVerdict::Skip(format!("drafting {} = {:?} blocks the replay ({:?})", drafted.0, drafted.1, status.worst)));
    }
    if (status.stage, status.review) != (HistoryTimeTravelStage::Reviewing, Some(HistoryTimeTravelReview::Ready)) {
        return Err(AcceptanceVerdict::Fail((format!("the replay ends at {:?}/{:?}, not a ready review", status.stage, status.review)).into()));
    }
    acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let commit = match alternative {
        Some(name) => vec![("name", DslValue::String(name.to_string()))],
        None => vec![("choice", DslValue::String(HISTORY_EDIT_CHOICE_OVERWRITE.to_string()))],
    };
    acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, commit,cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    Ok(drafted)
}
