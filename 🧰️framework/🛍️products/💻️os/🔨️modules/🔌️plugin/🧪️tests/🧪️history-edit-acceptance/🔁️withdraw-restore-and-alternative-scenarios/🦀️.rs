//! 🔁️ Withdraw, restore and alternative scenarios: the withdraw and restore round trip, the session, the row label, the overwrite scenario, the alternative scenario, the first pass and the withdraw-only kinds. See [`super`].

use super::*;

/// 🧯️ The withdraw → restore round trip over `app`'s first document mutation (design §22.1, §22.16): the row action
/// `historyEditWithdraw{mutationId}` opens the session on a withdrawn draft, accepting it replays to a review (ready, or blocked by
/// a downstream mutation that needed it), and `historyEditRestore{mutationId}` takes the withdrawal back — the session closes, the
/// head is the one before and the store holds no supersession of the mutation. `Ok(false)`: the store's supersede law admits no
/// withdrawal of this operation (`timeTravel.not-withdrawable`), nothing was changed.
async fn acceptance_withdraw_restore<A, M>(app: &mut VcsArtifactApp<A, M>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let target = app
        .stores.store
        .mutation_ops()
        .map_err(|error| AcceptanceVerdict::Fail((format!("the applied operations do not fold: {error:?}")).into()))?
        .first()
        .map(|op| op.mutation_id.0.clone())
        .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
    let before = acceptance_head(app).map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let named = || vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.clone()))];
    match acceptance_verb(app, HISTORY_EDIT_WITHDRAW_ACTION_ID, named(),cx).await {
        Ok(()) => {}
        Err(refusal) if refusal.message.ends_with(semio_framework_time_travel::TIME_TRAVEL_NOT_WITHDRAWABLE_CODE) => return Ok(false),
        Err(refusal) => return Err(AcceptanceVerdict::Fail(refusal.context("the row's withdraw does not open".into()))),
    }
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new(),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the withdrawn draft is not accepted".into())))?;
    acceptance_pump(app, |app| app.time_travel.status().is_some_and(|status| status.stage != HistoryTimeTravelStage::Replaying),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting the withdrawal closed the session".into()))?;
    if status.stage != HistoryTimeTravelStage::Reviewing || !matches!(status.review, Some(HistoryTimeTravelReview::Ready | HistoryTimeTravelReview::Blocked)) {
        return Err(AcceptanceVerdict::Fail((format!("the replay of the withdrawal ends at {:?}/{:?}, not a review with a report", status.stage, status.review)).into()));
    }
    acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, named(),cx).await.map_err(|refusal| AcceptanceVerdict::Fail(refusal.context("the withdrawal is not restored".into())))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    if app.stores.store.supersessions().iter().any(|(id, _)| id.0 == target) {
        return Err(AcceptanceVerdict::Fail("restoring the only withdrawal left a supersession of the mutation in the store".into()));
    }
    let after = acceptance_head(app).map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
    if after != before {
        return Err(AcceptanceVerdict::Fail((format!("after withdraw and restore the head differs from the one before at {}", acceptance_text_difference(&after.1, &before.1))).into()));
    }
    Ok(true)
}

/// ✏️ [`acceptance_drive`] over `app`'s first document mutation (its preview checked against `as_of`), then the store's own account
/// of it: the superseding input decoded as the edited operation, unscoped for an overwrite, scoped to the new active alternative
/// otherwise. Answers the change drafted and the edited operation the store now folds.
async fn acceptance_session<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: &(DslValue, String),
    kind: Option<usize>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let target = app
        .stores.store
        .mutation_ops()
        .map_err(|error| AcceptanceVerdict::Fail((format!("the applied operations do not fold: {error:?}")).into()))?
        .first()
        .map(|op| op.mutation_id.0.clone())
        .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
    let drafted = acceptance_drive(app, &target, None, change, alternative, Some(as_of), kind,cx).await?;
    let (_, supersession) = app.stores.store.supersessions().iter().find(|(id, _)| id.0 == target).ok_or_else(|| AcceptanceVerdict::Fail("the finalize superseded nothing".into()))?;
    let scope = supersession.scope.clone();
    let protocol::InputReplacement::Input { payload, .. } = &supersession.replacement else {
        return Err(AcceptanceVerdict::Fail("the finalize withdrew the edited operation".into()));
    };
    let edited = <A::Mutation as ::protocol::OpBinary>::decode_op(payload.as_slice()).map_err(|error| AcceptanceVerdict::Fail((format!("the superseding input does not decode: {error:?}")).into()))?;
    let envelope = app.stores.store.envelope();
    let expected = alternative.map(|name| envelope.vcs.alternatives.iter().find(|candidate| candidate.name == name).map(|candidate| candidate.id.clone()));
    match expected {
        None if scope.is_some() => return Err(AcceptanceVerdict::Fail((format!("the overwrite is scoped to {scope:?}")).into())),
        Some(None) => return Err(AcceptanceVerdict::Fail("the new alternative is not listed".into())),
        Some(Some(id)) if envelope.active_alternative_id.as_deref() != Some(id.as_str()) || scope.as_deref() != Some(id.as_str()) => {
            return Err(AcceptanceVerdict::Fail((format!("the new alternative {id} is not active and scoping the supersession ({scope:?})")).into()))
        }
        _ => {}
    }
    Ok((drafted, edited))
}

/// 🏷️ The edited mutation's history row names it in every locale with its leaf label, never with its text line.
async fn acceptance_row_label<A, M>(app: &mut VcsArtifactApp<A, M>, edited: &A::Mutation) -> Result<String, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let target = app.stores.store.mutation_ops().map_err(|error| format!("{error:?}"))?.first().map(|op| op.mutation_id.0.clone()).unwrap_or_default();
    let patch = app.render_ctx().history_patch(true).await.map_err(|fault| AcceptanceError::from(fault).context("the history does not project".into()))?;
    let row = patch.upserts.iter().flat_map(|entry| entry.mutations.iter()).find(|mutation| mutation.mutation_id == target).ok_or_else(|| "the edited mutation has no history row".to_string())?;
    let (en, de) = (row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string());
    if en.trim().is_empty() || de.trim().is_empty() || en == edited.print_op() || de == edited.print_op() {
        return Err((format!("the edited mutation's row label is {en:?} / {de:?}, not its leaf label in every locale")).into());
    }
    Ok(format!("{en} / {de}"))
}

/// 🎭️ One full scenario for `case` (with an optional `downstream` edit after it): overwrite on one instance, new alternative on
/// another, both heads against a fresh fold of the edited log, the edited row's label in every locale. With `reload` both
/// sessions run on the seeded document after it was saved and loaded into a fresh instance ([`acceptance_reloaded`]). With `kind`
/// the overwrite drafts an input of that control kind only ([`ACCEPTANCE_CONTROL_KINDS`]).
pub(super) async fn acceptance_scenario<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, downstream: Option<&A::Mutation>, reload: bool, kind: Option<usize>,cx:&mut semio_framework_job::StepContext<'_>) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let seed: Vec<&A::Mutation> = std::iter::once(&case.op).chain(downstream).collect();
    let mut overwrite = match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true,cx).await {
        Ok(app) => app,
        Err(AcceptanceSeedFault::Operation(0, reason)) => return AcceptanceVerdict::SkipCase(reason),
        Err(AcceptanceSeedFault::Operation(_, reason)) => return AcceptanceVerdict::Skip(format!("the downstream edit does not seed: {reason}")),
        Err(AcceptanceSeedFault::Original(reason)) => return AcceptanceVerdict::Fail(reason),
    };
    if reload {
        let reloaded = acceptance_reloaded(&overwrite, manifest,cx).await;
        acceptance_close(&mut overwrite,cx).await;
        overwrite = match reloaded {
            Ok(app) => app,
            Err(reason) => return AcceptanceVerdict::Fail(reason.context("the seeded document does not reload".into())),
        };
    }
    let as_of = match downstream {
        None => acceptance_head(&overwrite),
        Some(_) => match acceptance_seeded::<A, M>(manifest, &case.base, &[&case.op], true,cx).await {
            Ok(mut alone) => {
                let head = acceptance_head(&alone);
                acceptance_close(&mut alone,cx).await;
                head
            }
            Err(fault) => Err(fault.into_error().context("the edited mutation alone does not seed".into())),
        },
    };
    let as_of = match as_of {
        Ok(head) => head,
        Err(reason) => {
            acceptance_close(&mut overwrite,cx).await;
            return AcceptanceVerdict::Fail(reason);
        }
    };
    let (trail, session) = match acceptance_withdraw_restore(&mut overwrite,cx).await {
        Ok(withdrawn) => (if withdrawn { "withdrawn and restored without a trace" } else { "withdrawal not admitted by the store's law" }, acceptance_session(&mut overwrite, None, None, &as_of, kind,cx).await),
        Err(verdict) => ("", Err(verdict)),
    };
    let (change, edited) = match session {
        Ok(done) => done,
        Err(verdict) => {
            acceptance_close(&mut overwrite,cx).await;
            return verdict;
        }
    };
    let label = acceptance_row_label(&mut overwrite, &edited).await;
    let head = acceptance_head(&overwrite);
    let reloaded_head = acceptance_reloaded_head(&overwrite, manifest,cx).await;
    acceptance_close(&mut overwrite,cx).await;
    let edited_seed: Vec<&A::Mutation> = std::iter::once(&edited).chain(downstream).collect();
    let mut fresh = match acceptance_seeded::<A, M>(manifest, &case.base, &edited_seed, false,cx).await {
        Ok(app) => app,
        Err(fault) => {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
            return AcceptanceVerdict::Fail(fault.into_error().context("a fresh fold of the edited log does not seed".into()));
        }
    };
    let fold = acceptance_head(&fresh);
    acceptance_close(&mut fresh,cx).await;
    let verdict = match (label, head, fold, reloaded_head) {
        (Err(reason), _, _, _) | (_, Err(reason), _, _) | (_, _, Err(reason), _) => AcceptanceVerdict::Fail(reason),
        (_, _, _, Err(reason)) if reason.original.is_some() => AcceptanceVerdict::Fail(reason),
        (_, _, _, Err(reason)) => match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true,cx).await {
            Err(fault) => AcceptanceVerdict::Fail(fault.into_error().context(format!("the edited document does not survive save and load ({reason}) and its control does not seed"))),
            Ok(mut control) => {
                let unedited = acceptance_reloaded_head(&control, manifest,cx).await;
                acceptance_close(&mut control,cx).await;
                match unedited {
                    Err(seeded) if seeded.original.is_some() => AcceptanceVerdict::Fail(seeded),
                    Err(seeded) => AcceptanceVerdict::SkipCase(format!("the seeded document does not survive save and load before any edit ({seeded}): a seed through the store alone does not complete what this operation owns")),
                    Ok(_) => AcceptanceVerdict::Fail(reason.context(format!("after {} = {:?} the edited document does not survive save and load although the seeded one does", change.0, change.1))),
                }
            }
        },
        (Ok(_), Ok(head), Ok(fold), _) if head.0 != fold.0 => AcceptanceVerdict::Fail((format!("the overwrite head differs from a fresh fold of the edited log after {} = {:?}", change.0, change.1)).into()),
        (Ok(_), Ok(head), Ok(fold), _) if head.1 != fold.1 => {
            AcceptanceVerdict::Fail((format!("the overwrite head's document text differs from a fresh fold of the edited log after {} = {:?} at {}", change.0, change.1, acceptance_text_difference(&head.1, &fold.1))).into())
        }
        (Ok(_), Ok(_), Ok(fold), Ok(reloaded)) if reloaded != fold => AcceptanceVerdict::Fail((format!(
            "the edited document reloads to a head differing from a fresh fold of the edited log (store initializer ignores the supersession) after {} = {:?} at {}",
            change.0,
            change.1,
            acceptance_text_difference(&reloaded.1, &fold.1)
        )).into()),
        (Ok(label), Ok(_), Ok(fold), Ok(_)) => match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true,cx).await {
            Err(fault) => AcceptanceVerdict::Fail(fault.into_error().context("the alternative instance does not seed".into())),
            Ok(mut branched) if reload => {
                let reloaded = acceptance_reloaded(&branched, manifest,cx).await;
                acceptance_close(&mut branched,cx).await;
                match reloaded {
                    Err(reason) => AcceptanceVerdict::Fail(reason.context("the alternative instance does not reload".into())),
                    Ok(mut branched) => {
                        let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some(), &as_of, trail,cx).await;
                        acceptance_close(&mut branched,cx).await;
                        verdict
                    }
                }
            }
            Ok(mut branched) => {
                let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some(), &as_of, trail,cx).await;
                acceptance_close(&mut branched,cx).await;
                verdict
            }
        },
    };
    ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
    verdict
}

/// 🌳️ The new-alternative half of a scenario on `branched`: the change the overwrite accepted, committed as a new alternative, must
/// reach the same head as the fresh fold `fold`, live and after the document reloads through the app's own store initializer; its
/// preview is checked against `as_of` like the overwrite's, and `trail` (the overwrite's withdraw round trip) joins the summary.
#[allow(clippy::too_many_arguments)]
async fn acceptance_alternative<A, M>(
    branched: &mut VcsArtifactApp<A, M>,
    manifest: fn() -> App,
    change: &(String, DslValue),
    fold: &(DslValue, String),
    label: &str,
    downstream: bool,
    as_of: &(DslValue, String),
    trail: &str,cx:&mut semio_framework_job::StepContext<'_>) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    match acceptance_session(branched, Some(change), Some(ACCEPTANCE_ALTERNATIVE), as_of, None,cx).await {
        Err(AcceptanceVerdict::Skip(reason)) => AcceptanceVerdict::Fail((format!("the change the overwrite accepted fails as a new alternative: {reason}")).into()),
        Err(verdict) => verdict,
        Ok((_, alternative_edit)) => {
            let head = acceptance_head(branched);
            let reloaded = acceptance_reloaded_head(branched, manifest,cx).await;
            ::protocol::Mutation::<A::Snapshot>::retire_cold(alternative_edit);
            match (head, reloaded) {
                (Err(reason), _) | (_, Err(reason)) => AcceptanceVerdict::Fail(reason),
                (Ok(head), _) if head != *fold => AcceptanceVerdict::Fail((format!("the new alternative's head differs from a fresh fold of the edited log at {}", acceptance_text_difference(&head.1, &fold.1))).into()),
                (Ok(_), Ok(reloaded)) if reloaded != *fold => AcceptanceVerdict::Fail((format!(
                    "the new alternative reloads to a head differing from a fresh fold of the edited log (store initializer ignores the supersession) at {}",
                    acceptance_text_difference(&reloaded.1, &fold.1)
                )).into()),
                (Ok(_), Ok(_)) => AcceptanceVerdict::Pass(format!(
                    "{} = {:?}, row \"{label}\"{}, previewed as of the mutation, {trail}, both reloads fold alike",
                    change.0,
                    change.1,
                    if downstream { ", one downstream edit replayed" } else { "" }
                )),
            }
        }
    }
}

/// 🔁️ Runs `cases` in order until one passes: each case with up to [`ACCEPTANCE_DOWNSTREAM_PER_LEAF`] downstream edits from cases
/// on the same document, then alone; a case whose own operation does not seed, and every case on a document that does not load,
/// is skipped at once. `Ok(Some)` names the passing case, `Ok(None)` means every case was skipped (reasons appended to `skipped`),
/// `Err` is a broken mechanism.
pub(super) async fn acceptance_first_pass<A, M>(plugin: &str, manifest: fn() -> App, cases: &[AcceptanceCase<A::Mutation>], skipped: &mut Vec<String>, reload: bool,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<String>, AcceptanceError>
where
    A: ArtifactApp + Default,
    M: ReferenceSpaceMember + MemberFactory + Send + 'static,
    <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl,
{
    let mut unloadable = BTreeSet::new();
    for case in cases {
        if unloadable.contains(&case.base_key) {
            continue;
        }
        let leaf = protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
        let downstream: Vec<&A::Mutation> = cases.iter().filter(|other| other.base_key == case.base_key && other.directory != case.directory).map(|other| &other.op).take(ACCEPTANCE_DOWNSTREAM_PER_LEAF).collect();
        for next in downstream.into_iter().map(Some).chain(std::iter::once(None)) {
            match acceptance_scenario::<A, M>(manifest, case, next, reload, None,cx).await {
                AcceptanceVerdict::Pass(summary) => return Ok(Some(format!("{plugin}: {leaf} ({}) — {summary}", case.directory))),
                AcceptanceVerdict::Skip(reason) => skipped.push(format!("{leaf} ({}): {reason}", case.directory)),
                AcceptanceVerdict::SkipCase(reason) => {
                    skipped.push(format!("{leaf} ({}): {reason}", case.directory));
                    break;
                }
                AcceptanceVerdict::SkipBase(reason) => {
                    skipped.push(format!("{leaf} ({}): {reason}", case.directory));
                    unloadable.insert(case.base_key.clone());
                    break;
                }
                AcceptanceVerdict::Fail(reason) => return Err(reason.context(format!("{plugin}: the history edit of {leaf} ({}) breaks the generic mechanism", case.directory))),
            }
        }
    }
    Ok(None)
}

/// 🚫️ The semantic kinds whose leaf descriptor under `root` (a `🔣️.json` naming `semanticKind` and `aggregateVariant`, fixtures and
/// tests aside) declares `"editable": false`: withdraw-only by declaration (design §22.20, §22.23).
pub fn acceptance_withdraw_only_kinds(root: &std::path::Path) -> BTreeSet<String> {
    fn walk(directory: &std::path::Path, found: &mut BTreeSet<String>) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if !path.is_dir() || name.starts_with('.') || ["target", "dist", "node_modules", "🗑️generated", "🧫️fixtures", "🧪️tests"].contains(&name.as_str()) {
                continue;
            }
            if let Some(descriptor) = acceptance_json(&path.join("🔣️.json")).filter(|descriptor| descriptor.get("aggregateVariant").is_some()) {
                if descriptor.get("editable").and_then(DslValue::as_bool) == Some(false) {
                    found.extend(descriptor.get("semanticKind").and_then(DslValue::as_str).map(str::to_string));
                }
                continue;
            }
            walk(&path, found);
        }
    }
    let mut kinds = BTreeSet::new();
    walk(root, &mut kinds);
    kinds
}
