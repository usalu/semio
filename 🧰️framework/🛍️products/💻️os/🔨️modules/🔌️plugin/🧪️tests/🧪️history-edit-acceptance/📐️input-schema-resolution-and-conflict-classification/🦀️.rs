//! 📐️ Input schema resolution and conflict classification: the input schema resolution failures and assertion, the applied ids, blocks, classification, withdraw accept, resolve and overwrite folds. See [`super`].

use super::*;

/// 🔗️ Every breach of complete input-schema resolution among `A`'s document leaves, through the resolver the history editor uses
/// (`registered_input_schema_document`): a leaf whose input descriptors the reader refuses, a `$id` its payload schema references
/// (transitively) that nothing published, or a payload validator that does not compile. One line per breach, naming the leaf. The
/// kinds of `withdraw_only` (declared `"editable": false`: no editor ever opens on them, design §22.20) are not judged.
pub fn input_schema_resolution_failures<A: ArtifactApp>(withdraw_only: &BTreeSet<String>) -> Vec<String> {
    fn references(value: &DslValue, into: &mut Vec<String>) {
        match value {
            DslValue::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("$ref", DslValue::String(reference)) if !reference.starts_with('#') => into.push(reference.split('#').next().unwrap_or_default().to_string()),
                        _ => references(value, into),
                    }
                }
            }
            DslValue::Array(items) => items.iter().for_each(|item| references(item, into)),
            _ => {}
        }
    }
    let descriptors = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS;
    let schemas = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMAS;
    let mut failures = Vec::new();
    for (descriptor, schema) in descriptors.iter().zip(schemas) {
        let kind = descriptor.semantic_kind;
        if withdraw_only.contains(kind) {
            continue;
        }
        if let Err(error) = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document) {
            failures.push(format!("{kind}: the input reader refuses its payload schema: {error:?}"));
        }
        let Ok(root) = semio_framework_pack_json::parse(schema, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            failures.push(format!("{kind}: its payload schema is not JSON"));
            continue;
        };
        let mut pending = Vec::new();
        references(&semio_framework_pack_json::to_dsl_value(&root), &mut pending);
        let (mut seen, mut documents, mut unresolved) = (BTreeSet::new(), Vec::new(), Vec::new());
        while let Some(id) = pending.pop() {
            if id.is_empty() || !seen.insert(id.clone()) {
                continue;
            }
            match semio_framework::registered_input_schema_document(&id) {
                Some(document) => {
                    references(&document, &mut pending);
                    documents.push(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&document)));
                }
                None => unresolved.push(id),
            }
        }
        if !unresolved.is_empty() {
            failures.push(format!("{kind}: nothing publishes the referenced schema document(s) {unresolved:?}"));
        }
        if let Err(error) = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()) {
            failures.push(format!("{kind}: its payload validator does not compile: {error}"));
        }
    }
    failures
}

/// 🩺️ LAW (design §16.3, the editable-everything gate at runtime): on a registered instance — which publishes every document its
/// leaves reference, as in production — every editable document leaf's payload schema resolves completely (a leaf under `fixtures`
/// declared `"editable": false` is withdraw-only and not judged). Answers the number of leaves read and the number declared
/// withdraw-only; panics naming `plugin` and every breaching leaf otherwise.
pub async fn assert_input_schemas_resolve<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path) -> (usize, usize)
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("original acceptance caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest, protocol::ActorId(ACCEPTANCE_ACTOR.into()), acceptance_policy(), &mut identity).await);
    let declared = acceptance_withdraw_only_kinds(fixtures);
    let failures = input_schema_resolution_failures::<A>(&declared);
    acceptance_close(&mut app,cx).await;
    assert!(failures.is_empty(), "{plugin}: {} leaf payload schema(s) do not resolve through the runtime resolver:\n{}", failures.len(), failures.join("\n"));
    let withdraw_only = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.iter().filter(|descriptor| declared.contains(descriptor.semantic_kind)).count();
    (<A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMAS.len() - withdraw_only, withdraw_only)

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original acceptance caller paid retirement");result
}

/// 🆔️ The ids of `app`'s applied document mutations, in applied order.
pub(super) fn acceptance_applied_ids<A, M>(app: &VcsArtifactApp<A, M>) -> Result<Vec<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    Ok(app.stores.store.mutation_ops().map_err(|error| AcceptanceVerdict::Fail((format!("the applied operations do not fold: {error:?}")).into()))?.iter().map(|op| op.mutation_id.0.clone()).collect())
}

/// ⛔️ Whether a replay outcome blocks finalizing: an error or a fatal (the floor `ReplayReport::blocks_finalize` reads).
pub(super) fn acceptance_blocks(worst: Option<semio_framework_diagnostic::Severity>) -> bool {
    matches!(worst, Some(semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal))
}

/// 🧮️ The review after a replay, as the conflict law reads it: the session reviews, its report classifies every mutation of
/// `expected` (one outcome each), the status blocks exactly when an outcome is an error or a fatal, and `nextProblem` names the
/// first such outcome. Answers the outcomes in replay order and whether the review is blocked.
pub(super) fn acceptance_classified<A, M>(app: &VcsArtifactApp<A, M>, expected: &[String]) -> Result<(Vec<(String, Option<semio_framework_diagnostic::Severity>)>, bool), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("the replay closed the session".into()))?;
    if status.stage != HistoryTimeTravelStage::Reviewing {
        return Err(AcceptanceVerdict::Fail((format!("the replay ends at {:?}, not a review", status.stage)).into()));
    }
    let outcomes: Vec<(String, Option<semio_framework_diagnostic::Severity>)> = match app.time_travel.session().report.as_ref() {
        Some(report) => report.outcomes.iter().map(|outcome| (outcome.mutation_id.0.clone(), outcome.worst)).collect(),
        None => return Err(AcceptanceVerdict::Fail((format!("the review holds no replay report (review {:?}, fault {:?})", status.review, status.fault)).into())),
    };
    if let Some(missing) = expected.iter().find(|id| !outcomes.iter().any(|(classified, _)| classified == *id)) {
        return Err(AcceptanceVerdict::Fail((format!("the replay report classifies {} mutation(s) but not the applied mutation {missing}", outcomes.len())).into()));
    }
    let first = outcomes.iter().find(|(_, worst)| acceptance_blocks(*worst)).map(|(id, _)| id.clone());
    if (status.review == Some(HistoryTimeTravelReview::Blocked)) != first.is_some() || status.blocking != first.is_some() {
        return Err(AcceptanceVerdict::Fail((format!("the review is {:?} (blocking: {}) although the report's first blocking outcome is {first:?}", status.review, status.blocking)).into()));
    }
    let named = status.next_problem.as_ref().map(|problem| problem.mutation_id.clone());
    if named != first {
        return Err(AcceptanceVerdict::Fail((format!("nextProblem names {named:?} although the report's first blocking outcome is {first:?}")).into()));
    }
    Ok((outcomes, first.is_some()))
}

/// ✂️ A history row's Withdraw of the mutation `id` (it opens the session, or stacks into the open review), accepted and replayed to
/// its review. `Ok(false)`: the store's supersede law admits no withdrawal of it, nothing changed.
pub(super) async fn acceptance_withdraw_accept<A, M>(app: &mut VcsArtifactApp<A, M>, id: &str,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    match acceptance_verb(app, HISTORY_EDIT_WITHDRAW_ACTION_ID, vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(id.to_string()))],cx).await {
        Ok(()) => {}
        Err(refusal) if refusal.message.ends_with(semio_framework_time_travel::TIME_TRAVEL_NOT_WITHDRAWABLE_CODE) => return Ok(false),
        Err(refusal) => return Err(AcceptanceVerdict::Fail(refusal.context("the withdraw of {id} does not open".into()))),
    }
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new(),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the withdrawal of {id} is not accepted".into())))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    Ok(true)
}

/// 🧗️ Resolves a blocked review blocker by blocker: the mutation `nextProblem` names is withdrawn, accepted and replayed, until the
/// review is ready. Answers the ids withdrawn, in order; a blocker that cannot be withdrawn, one named twice, or more than
/// [`ACCEPTANCE_CASCADE_BLOCKERS`] of them break the mechanism (design §22.1: a blocking mutation is always resolvable).
pub(super) async fn acceptance_resolve<A, M>(app: &mut VcsArtifactApp<A, M>, expected: &[String],cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    let mut resolved: Vec<String> = Vec::new();
    loop {
        let (_, blocked) = acceptance_classified(app, expected)?;
        if !blocked {
            return Ok(resolved);
        }
        if resolved.len() >= ACCEPTANCE_CASCADE_BLOCKERS {
            return Err(AcceptanceVerdict::Fail((format!("withdrawing {} blockers in turn does not reach a ready review", resolved.len())).into()));
        }
        let blocker = app.time_travel.status().and_then(|status| status.next_problem).map(|problem| problem.mutation_id).ok_or_else(|| AcceptanceVerdict::Fail("a blocked review names no next problem".into()))?;
        if resolved.contains(&blocker) {
            return Err(AcceptanceVerdict::Fail((format!("nextProblem names {blocker} again after it was withdrawn")).into()));
        }
        if !acceptance_withdraw_accept(app, &blocker,cx).await? {
            return Err(AcceptanceVerdict::Fail((format!("the blocking mutation {blocker} cannot be withdrawn")).into()));
        }
        resolved.push(blocker);
    }
}

/// 🏗️ Finalizes the ready review as an overwrite and compares the result with a fresh fold of `kept` on `case`'s document: the
/// live head and the head the document reloads to must both equal it. `Err` names the departure.
pub(super) async fn acceptance_overwrite_folds<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, kept: &[&A::Mutation],cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new(),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the resolved review does not finalize".into())))?;
    acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, vec![("choice", DslValue::String(HISTORY_EDIT_CHOICE_OVERWRITE.to_string()))],cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the resolved review does not commit".into())))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let head = acceptance_head(app).map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let reloaded = acceptance_reloaded_head(app, manifest,cx).await.map_err(|reason| AcceptanceVerdict::Fail(reason.context("after the conflict was resolved the finalized document does not survive save and load".into())))?;
    let mut fresh = acceptance_seeded::<A, M>(manifest, &case.base, kept, false,cx).await.map_err(|fault| AcceptanceVerdict::Fail(fault.into_error().context("a fresh fold of the effective history does not seed".into())))?;
    let fold = acceptance_head(&fresh);
    acceptance_close(&mut fresh,cx).await;
    let fold = fold.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    if head != fold {
        return Err(AcceptanceVerdict::Fail((format!("after the conflict was resolved the overwrite head differs from a fresh fold of the effective history at {}", acceptance_text_difference(&head.1, &fold.1))).into()));
    }
    if reloaded != fold {
        return Err(AcceptanceVerdict::Fail((format!("after the conflict was resolved the document reloads to a head differing from a fresh fold of the effective history at {}", acceptance_text_difference(&reloaded.1, &fold.1))).into()));
    }
    Ok(())
}
