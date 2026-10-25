//! ⚔️ Cascade withdraw, edit and conflict resolution: the cascade withdraw, the cascade edit, the cascade, the conflicts resolve assertion and the blocking executor. See [`super`].

use super::*;

/// 🧨️ The withdraw half of the conflict law on `app`, seeded with `seed` (the case's operation, then its inverse): the root is
/// withdrawn and the report classified; a review that blocks is resolved, restored to its original report, taken back without a
/// trace, resolved again and finalized. `Err(Skip)`: withdrawing the root blocks nothing (no dependents).
async fn acceptance_cascade_withdraw<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, seed: &[&A::Mutation],cx:&mut semio_framework_job::StepContext<'_>) -> Result<String, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    let ids = acceptance_applied_ids(app)?;
    let root = ids.first().cloned().ok_or_else(|| AcceptanceVerdict::Fail("the seeded history left no applied operation".into()))?;
    if ids.len() != seed.len() {
        return Err(AcceptanceVerdict::SkipCase(format!("its history lists {} applied operations for {} seeded ones", ids.len(), seed.len())));
    }
    let before = acceptance_head(app).map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    if let Err(reason) = acceptance_reloaded_head(app, manifest,cx).await {
        if reason.original.is_some(){return Err(AcceptanceVerdict::Fail(reason))}
        return Err(AcceptanceVerdict::SkipCase(format!("its seeded history does not survive save and load before any edit ({reason})")));
    }
    if !acceptance_withdraw_accept(app, &root,cx).await? {
        return Err(AcceptanceVerdict::SkipCase("the store's law admits no withdrawal of its operation".into()));
    }
    let restore = |id: &str| vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(id.to_string()))];
    let (original, blocked) = acceptance_classified(app, &ids)?;
    if !blocked {
        acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(root.as_str()),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the withdrawal of the root is not restored".into())))?;
        acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        return Err(AcceptanceVerdict::Skip("withdrawing its operation blocks nothing downstream".into()));
    }
    if acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new(),cx).await.is_ok() {
        return Err(AcceptanceVerdict::Fail("a blocked review lets historyEditFinalize through".into()));
    }
    let resolved = acceptance_resolve(app, &ids,cx).await?;
    for blocker in resolved.iter().rev() {
        acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(blocker.as_str()),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the withdrawn blocker {blocker} is not restored".into())))?;
        acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    }
    let (again, _) = acceptance_classified(app, &ids)?;
    if again != original {
        return Err(AcceptanceVerdict::Fail((format!("restoring the {} resolved blocker(s) does not return to the original report: {again:?} instead of {original:?}", resolved.len())).into()));
    }
    acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(root.as_str()),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the withdrawal of the root is not restored".into())))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    if app.stores.store.supersessions().iter().any(|(id, _)| ids.contains(&id.0)) {
        return Err(AcceptanceVerdict::Fail("restoring every withdrawal left a supersession in the store".into()));
    }
    let after = acceptance_head(app).map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    if after != before {
        return Err(AcceptanceVerdict::Fail((format!("after every withdrawal was restored the head differs from the one before at {}", acceptance_text_difference(&after.1, &before.1))).into()));
    }
    if !acceptance_withdraw_accept(app, &root,cx).await? {
        return Err(AcceptanceVerdict::Fail("the root was withdrawn once and cannot be withdrawn again".into()));
    }
    let resolved = acceptance_resolve(app, &ids,cx).await?;
    let kept: Vec<&A::Mutation> = ids.iter().zip(seed).skip(1).filter(|(id, _)| !resolved.contains(*id)).map(|(_, op)| *op).collect();
    acceptance_overwrite_folds(app, manifest, case, &kept,cx).await?;
    let blocking = original.iter().filter(|(_, worst)| acceptance_blocks(*worst)).count();
    Ok(format!(
        "its withdrawal is classified over {} mutation(s) and blocks {blocking}; nextProblem named each of the {} blocker(s) resolved by withdrawal; restored to the original report and then without a trace; resolved again and finalized: head and reload equal the fresh fold of the {} mutation(s) left",
        original.len(),
        resolved.len(),
        kept.len()
    ))
}

/// 🎚️ The edit half of the conflict law on a fresh instance seeded with `seed`: an input of the root is drafted to another valid
/// value until one blocks the replay downstream; that review is resolved and finalized, and the head and the reload must equal a
/// fresh fold of the edited root followed by the mutations left. An input change that makes the root itself fail, or that only
/// withdrawing the root resolves, is no downstream conflict: it is taken back and the next one drafted. `Ok(None)`: no input change of
/// the root conflicts downstream.
async fn acceptance_cascade_edit<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, seed: &[&A::Mutation],cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    let mut app = acceptance_seeded::<A, M>(manifest, &case.base, seed, true,cx).await.map_err(|fault| AcceptanceVerdict::Fail(fault.into_error().context("the history that seeded once does not seed again".into())))?;
    let outcome = async {
        let ids = acceptance_applied_ids(&app)?;
        let root = ids.first().cloned().ok_or_else(|| AcceptanceVerdict::Fail("the seeded history left no applied operation".into()))?;
        let begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(root.clone()))];
        acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        let (inputs, original) = match app.time_travel.editor() {
            None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
            Some(editor) => (editor.inputs.clone(), editor.value.clone()),
        };
        for candidate in acceptance_changes(&inputs, &original) {
            if app.time_travel.status().is_none() {
                acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
            }
            if acceptance_verb(&mut app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())],cx).await.is_err()
                || !app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
            {
                continue;
            }
            acceptance_verb(&mut app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
            acceptance_pump(&mut app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
            if app.time_travel.status().is_none() {
                continue;
            }
            let (classified, blocked) = acceptance_classified(&app, &ids)?;
            let own = app.time_travel.status().and_then(|status| status.next_problem).is_some_and(|problem| problem.mutation_id == root);
            if !blocked || own {
                acceptance_verb(&mut app, HISTORY_EDIT_RESTORE_ACTION_ID, begin.clone(),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("an accepted draft of the root is not restored".into())))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
                continue;
            }
            let resolved = acceptance_resolve(&mut app, &ids,cx).await?;
            if resolved.contains(&root) {
                acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("a review that only withdrawing the edited root resolves does not exit".into())))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
                continue;
            }
            let edited = match app.time_travel.session().accepted_draft(&protocol::MutationId(root.clone())).map(|draft| &draft.replacement) {
                Some(protocol::InputReplacement::Input { payload, .. }) => <A::Mutation as ::protocol::OpBinary>::decode_op(payload.as_slice()).map_err(|error| AcceptanceVerdict::Fail((format!("the root's edited input does not decode: {error:?}")).into()))?,
                Some(_) => return Err(AcceptanceVerdict::Fail("the root's accepted draft is a withdrawal, not its edited input".into())),
                None => return Err(AcceptanceVerdict::Fail("the resolved review no longer holds the root's draft".into())),
            };
            let kept: Vec<&A::Mutation> = std::iter::once(&edited).chain(ids.iter().zip(seed).skip(1).filter(|(id, _)| !resolved.contains(*id)).map(|(_, op)| *op)).collect();
            let folded = acceptance_overwrite_folds(&mut app, manifest, case, &kept,cx).await;
            let left = kept.len() - 1;
            ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
            folded?;
            return Ok(Some(format!(
                "editing its input {} = {:?} is classified over {} mutation(s) and blocks downstream; {} blocker(s) resolved by withdrawal; finalized: head and reload equal the fresh fold of the edited root and the {left} mutation(s) left",
                candidate.0,
                candidate.1,
                classified.len(),
                resolved.len()
            )));
        }
        if app.time_travel.status().is_some() {
            let _ = acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx).await;
        }
        Ok::<Option<String>, AcceptanceVerdict>(None)
    }
    .await;
    acceptance_close(&mut app,cx).await;
    outcome
}

/// 🌪️ The conflict scenario for `case`: its operation followed by its own inverse on its own document is the history; the withdraw
/// half ([`acceptance_cascade_withdraw`]) must find a blocked review, then the edit half ([`acceptance_cascade_edit`]) runs on a
/// fresh instance. `Pass` carries both summaries; a skip says why this case shows no conflict; `Fail` is a broken mechanism.
async fn acceptance_cascade<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    let mut probe = match acceptance_seeded::<A, M>(manifest, &case.base, &[], true,cx).await {
        Ok(app) => app,
        Err(AcceptanceSeedFault::Original(reason)) => return AcceptanceVerdict::Fail(reason),
        Err(AcceptanceSeedFault::Operation(index,reason)) => return AcceptanceVerdict::SkipBase(format!("seed operation #{index}: {reason}")),
    };
    let inverse = match probe.stores.store.snapshot() {
        Ok(before) => {
            let inverse = <A::Mutation as ::protocol::ColdMutation<A::Snapshot>>::inverse(&case.op, &before).map_err(|error| format!("its operation states no inverse on its document: {error:?}"));
            std::mem::forget(before);
            inverse
        }
        Err(error) => Err((format!("its document does not fold: {error:?}")).into()),
    };
    acceptance_close(&mut probe,cx).await;
    let inverse = match inverse {
        Ok(inverse) if !inverse.is_empty() => inverse,
        Ok(_) => return AcceptanceVerdict::SkipCase("its operation has an empty inverse on its document".into()),
        Err(reason) => return AcceptanceVerdict::SkipCase(reason),
    };
    let seed: Vec<&A::Mutation> = std::iter::once(&case.op).chain(&inverse).collect();
    let verdict = match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true,cx).await {
        Err(AcceptanceSeedFault::Operation(0, reason)) => AcceptanceVerdict::SkipCase(reason),
        Err(AcceptanceSeedFault::Operation(_, reason)) => AcceptanceVerdict::SkipCase(format!("its inverse does not seed after it: {reason}")),
        Err(AcceptanceSeedFault::Original(reason)) => AcceptanceVerdict::Fail(reason),
        Ok(mut app) => {
            let withdrawn = acceptance_cascade_withdraw(&mut app, manifest, case, &seed,cx).await;
            acceptance_close(&mut app,cx).await;
            match withdrawn {
                Err(verdict) => verdict,
                Ok(summary) => match acceptance_cascade_edit::<A, M>(manifest, case, &seed,cx).await {
                    Ok(Some(edited)) => AcceptanceVerdict::Pass(format!("{summary}; {edited}")),
                    Ok(None) => AcceptanceVerdict::Pass(format!("{summary}; no input change of the root conflicts downstream")),
                    Err(AcceptanceVerdict::Fail(reason)) => AcceptanceVerdict::Fail(reason.context("the edit scenario".into())),
                    Err(verdict) => AcceptanceVerdict::Pass(format!("{summary}; the edit scenario was skipped: {}", verdict.text())),
                },
            }
        }
    };
    drop(seed);
    for op in inverse {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
    }
    verdict
}

/// 🏛️ LAW (design §23.2, the generic conflict case): on the first history of this app — a committed operation followed by its own
/// inverse, on its own document or a shipped one — whose first mutation, withdrawn, blocks the replay downstream, the conflict is
/// resolved end to end ([`acceptance_cascade`]). Answers that case's summary; an app none of whose histories blocks answers
/// "no dependents" with its census, one without a parent-lane leaf says so; panics naming `plugin`, the leaf and the case when the
/// mechanism breaks.
pub async fn assert_history_conflicts_resolve<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
{
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("original acceptance caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    if <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.is_empty() {
        return format!("{plugin}: no parent-lane leaf (conflicts between child-lane mutations are not driven by this law)");
    }
    let mut cases = acceptance_cases::<A>(fixtures);
    cases.extend(acceptance_example_cases::<A>(fixtures, examples).await);
    let (mut tried, mut unbuilt) = (0, 0);
    let (mut found, mut broken) = (None, None);
    let mut reasons: Vec<String> = Vec::new();
    let mut unloadable = BTreeSet::new();
    for case in &cases {
        if tried >= ACCEPTANCE_CASCADE_CASES || unbuilt >= ACCEPTANCE_CASCADE_UNBUILT {
            break;
        }
        if unloadable.contains(&case.base_key) {
            continue;
        }
        let leaf = protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
        match acceptance_cascade::<A, M>(manifest, case,cx).await {
            AcceptanceVerdict::Pass(summary) => {
                found = Some(format!("{leaf} ({}) — {summary}", case.directory));
                break;
            }
            AcceptanceVerdict::Fail(reason) => {
                broken = Some(reason.context(format!("{plugin}: the conflict law on {leaf} ({}) breaks the generic mechanism", case.directory)));
                break;
            }
            AcceptanceVerdict::Skip(reason) => {
                tried += 1;
                reasons.push(format!("{leaf}: {reason}"));
            }
            AcceptanceVerdict::SkipCase(reason) => {
                unbuilt += 1;
                reasons.push(format!("{leaf}: {reason}"));
            }
            AcceptanceVerdict::SkipBase(reason) => {
                unbuilt += 1;
                unloadable.insert(case.base_key.clone());
                reasons.push(format!("{leaf}: {reason}"));
            }
        }
    }
    for case in cases {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    if let Some(failure) = broken {
        panic!("{failure}");
    }
    match found {
        Some(summary) => format!("{plugin}: {summary}"),
        None => format!(
            "{plugin}: no dependents — {tried} histories of a committed operation and its inverse block nothing when their first mutation is withdrawn, {unbuilt} could not be built (first reasons: {})",
            reasons.iter().take(3).map(|reason| reason.chars().take(160).collect::<String>()).collect::<Vec<_>>().join(" | ")
        ),
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original acceptance caller paid retirement");result
}

/// ⏳️ Polls the law's future to completion on the calling thread, yielding between polls — the dependency-free executor every
/// fixture app is driven by, so a plugin crate needs no async test macro to run the law.
pub fn block_on_acceptance<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}
