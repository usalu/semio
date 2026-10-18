//! 🔢️ End to end assertion, census and search: the history edits end to end assertion, the offered leaves, the census and the leaf search. See [`super`].

use super::*;

/// 🧑‍⚖️ LAW (design §16.3): a representative editable leaf of the app is edited in history end to end through the generic
/// mechanism — overwrite and new alternative, each head (value and document text) a fresh fold of the edited log, the row
/// labelled in every locale. The committed fixture cases under `fixtures` are tried first ([`acceptance_cases`]); when none
/// exercises the flow, every committed operation on every document the app ships ([`acceptance_example_cases`] over its initial
/// document and `examples`). The first passing case is the representative; a broken mechanism fails at once. Answers the passing
/// case's summary; an app whose aggregate has no leaf at all says so, and so does one whose every leaf is declared withdraw-only
/// (`"editable": false`, design §22.20); panics naming `plugin`, the leaf and the case otherwise. After the representative, every
/// editable leaf is swept ([`acceptance_census`]): one passing case per input-control kind and no broken case are law, the number
/// of leaves exercised is a census line appended to the summary.
pub async fn assert_history_edits_end_to_end<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("original acceptance caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let descriptors = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS;
    if descriptors.is_empty() {
        return format!("{plugin}: no parent-lane leaf (a composed parent edits its content on the child lane)");
    }
    let withdraw_only = acceptance_withdraw_only_kinds(fixtures);
    if descriptors.iter().all(|descriptor| withdraw_only.contains(descriptor.semantic_kind)) {
        return format!("{plugin}: withdraw-only artifact (all {} leaves declare editable: false, design §22.20)", descriptors.len());
    }
    match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, false,cx).await {
        AcceptanceSearch::Passed(summary) => format!("{summary}\n[history-edit-census] {}", acceptance_census::<A, M>(plugin, manifest, fixtures, examples, &withdraw_only,cx).await),
        AcceptanceSearch::Failed(failure) => panic!("{failure}"),
        AcceptanceSearch::Exhausted(report) => panic!("{report}"),
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original acceptance caller paid retirement");result
}

/// 🔭️ Which input-control kinds ([`ACCEPTANCE_CONTROL_KINDS`]) the draft editor of `case`'s operation offers a change for: the case
/// seeded alone, `historyEditBegin`, the editor's inputs read, `historyEditExit`. `Err` is the verdict that ends the case (its
/// document or operation does not seed: a skip; the editor of an editable operation does not open or close: a broken mechanism).
async fn acceptance_offered<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<[bool; 5], AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = match acceptance_seeded::<A, M>(manifest, &case.base, &[&case.op], true,cx).await {
        Ok(app) => app,
        Err(AcceptanceSeedFault::Operation(_, reason)) => return Err(AcceptanceVerdict::SkipCase(reason)),
        Err(AcceptanceSeedFault::Original(reason)) => return Err(AcceptanceVerdict::Fail(reason)),
    };
    let offered = async {
        let target = app
            .stores.store
            .mutation_ops()
            .map_err(|error| AcceptanceVerdict::Fail((format!("the applied operations do not fold: {error:?}")).into()))?
            .first()
            .map(|op| op.mutation_id.0.clone())
            .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
        acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target))],cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        let offered = match app.time_travel.editor() {
            None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
            Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail((format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused)).into())),
            Some(editor) => acceptance_change_buckets(&editor.inputs, &editor.value).map(|bucket| !bucket.is_empty()),
        };
        acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new(),cx).await.map_err(|reason|AcceptanceVerdict::Fail(reason.into()))?;
        Ok::<_, AcceptanceVerdict>(offered)
    }
    .await;
    acceptance_close(&mut app,cx).await;
    offered
}

/// 📋️ LAW (design §16.3, session-5 decision): the sweep behind [`assert_history_edits_end_to_end`] over every editable leaf of the
/// aggregate (the kinds of `withdraw_only` aside) and its committed cases under `fixtures` — for a leaf that commits no case with a
/// document, its committed operations on the documents the app ships ([`acceptance_example_cases`]) — at most
/// [`ACCEPTANCE_CASES_PER_LEAF`] per leaf. Law: every input-control kind some case's editor offers ([`acceptance_offered`]) has one case that passes the whole
/// scenario drafting an input of that kind, and no case breaks the generic mechanism — it panics naming `plugin`, the kind or every
/// broken case otherwise. Census, never failing: the leaves exercised of the editable ones, those without a committed editable
/// case, and the leaf that proved each kind — the line it answers.
async fn acceptance_census<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource], withdraw_only: &BTreeSet<String>,cx:&mut semio_framework_job::StepContext<'_>) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let leaves: Vec<&str> = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.iter().map(|descriptor| descriptor.semantic_kind).filter(|kind| !withdraw_only.contains(*kind)).collect();
    let mut cases = acceptance_cases::<A>(fixtures);
    let leaf_of = |case: &AcceptanceCase<A::Mutation>| protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
    let committed: BTreeSet<&str> = cases.iter().map(leaf_of).collect();
    for case in acceptance_example_cases::<A>(fixtures, examples).await {
        match committed.contains(leaf_of(&case)) {
            true => ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op),
            false => cases.push(case),
        }
    }
    let shipped = leaves.iter().filter(|leaf| !committed.contains(**leaf) && cases.iter().any(|case| leaf_of(case) == **leaf)).count();
    let (mut exercised, mut uncased, mut attempted) = (BTreeSet::new(), Vec::new(), BTreeSet::new());
    let mut offering: [BTreeSet<&str>; 5] = Default::default();
    let mut proven: [Option<&str>; 5] = [None; 5];
    let (mut broken, mut skipped) = (Vec::new(), Vec::new());
    for leaf in leaves.iter().copied() {
        let own: Vec<usize> = cases.iter().enumerate().filter(|(_, case)| leaf_of(case) == leaf).map(|(index, _)| index).take(ACCEPTANCE_CASES_PER_LEAF).collect();
        if own.is_empty() {
            uncased.push(leaf);
        }
        for index in own {
            let case = &cases[index];
            let offered = match acceptance_offered::<A, M>(manifest, case,cx).await {
                Ok(offered) => offered,
                Err(AcceptanceVerdict::Fail(reason)) => {
                    broken.push(reason.context(format!("{leaf} ({})", case.directory)));
                    continue;
                }
                Err(verdict) => {
                    skipped.push(format!("{leaf} ({}): {}", case.directory, verdict.text()));
                    continue;
                }
            };
            for kind in (0..5).filter(|kind| offered[*kind]) {
                offering[kind].insert(leaf);
            }
            if !offered.contains(&true) {
                skipped.push(format!("{leaf} ({}): its editor offers no number, boolean, option, vector or text input to change (references, lists or opaque values only)", case.directory));
                continue;
            }
            let mut wanted: Vec<usize> = (0..5).filter(|kind| offered[*kind] && proven[*kind].is_none()).collect();
            if wanted.is_empty() && !exercised.contains(leaf) {
                wanted.extend((0..5).find(|kind| offered[*kind]));
            }
            for kind in wanted {
                attempted.insert((index, kind));
                match acceptance_scenario::<A, M>(manifest, case, None, false, Some(kind),cx).await {
                    AcceptanceVerdict::Pass(_) => {
                        exercised.insert(leaf);
                        proven[kind].get_or_insert(leaf);
                    }
                    AcceptanceVerdict::Fail(reason) => broken.push(reason.context(format!("{leaf} [{}] ({})", ACCEPTANCE_CONTROL_KINDS[kind], case.directory))),
                    verdict => skipped.push(format!("{leaf} [{}] ({}): {}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory, verdict.text())),
                }
            }
            if exercised.contains(leaf) {
                break;
            }
        }
    }
    for kind in 0..5 {
        let pending: Vec<usize> = cases.iter().enumerate().filter(|(index, case)| offering[kind].contains(leaf_of(case)) && !attempted.contains(&(*index, kind))).map(|(index, _)| index).take(ACCEPTANCE_KIND_ATTEMPTS).collect();
        for index in pending {
            if proven[kind].is_some() {
                break;
            }
            let (case, leaf) = (&cases[index], leaf_of(&cases[index]));
            match acceptance_scenario::<A, M>(manifest, case, None, false, Some(kind),cx).await {
                AcceptanceVerdict::Pass(_) => {
                    exercised.insert(leaf);
                    proven[kind] = Some(leaf);
                }
                AcceptanceVerdict::Fail(reason) => broken.push(reason.context(format!("{leaf} [{}] ({})", ACCEPTANCE_CONTROL_KINDS[kind], case.directory))),
                verdict => skipped.push(format!("{leaf} [{}] ({}): {}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory, verdict.text())),
            }
        }
    }
    let gaps: Vec<String> = (0..5).filter(|kind| proven[*kind].is_none() && !offering[*kind].is_empty()).map(|kind| format!("{} (offered by {})", ACCEPTANCE_CONTROL_KINDS[kind], offering[kind].iter().copied().collect::<Vec<_>>().join(", "))).collect();
    let kinds: Vec<String> = (0..5)
        .map(|kind| match proven[kind] {
            Some(leaf) => format!("{} proven by {leaf}", ACCEPTANCE_CONTROL_KINDS[kind]),
            None if offering[kind].is_empty() => format!("{} not offered", ACCEPTANCE_CONTROL_KINDS[kind]),
            None => format!("{} UNPROVEN", ACCEPTANCE_CONTROL_KINDS[kind]),
        })
        .collect();
    let idle: Vec<String> = leaves
        .iter()
        .copied()
        .filter(|leaf| !exercised.contains(leaf) && !uncased.contains(leaf))
        .map(|leaf| {
            let reason = skipped.iter().chain(broken.iter().map(|error|&error.message)).find(|entry| entry.starts_with(&format!("{leaf} "))).and_then(|entry| entry.rsplit_once("): ")).map_or("no reason recorded", |(_, reason)| reason);
            format!("{leaf} [{}]", reason.chars().take(110).collect::<String>())
        })
        .collect();
    let census = format!(
        "{plugin}: {} of {} editable leaves exercised ({} withdraw-only by declaration, {} swept only on a shipped document for want of a committed one, {} without any editable case{}{}, {} with cases but not exercised{}{}); control kinds: {}",
        exercised.len(),
        leaves.len(),
        <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.len() - leaves.len(),
        shipped,
        uncased.len(),
        if uncased.is_empty() { "" } else { ": " },
        uncased.iter().take(12).copied().collect::<Vec<_>>().join(", "),
        idle.len(),
        if idle.is_empty() { "" } else { ": " },
        idle.iter().take(12).cloned().collect::<Vec<_>>().join("; "),
        kinds.join(", ")
    );
    for case in cases {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    let shown = skipped.len().min(12);
    assert!(broken.is_empty(), "{plugin}: {} leaf case(s) break the generic mechanism (first {}):\n{}\n[history-edit-census] {census}", broken.len(), broken.len().min(12), broken.iter().take(12).map(ToString::to_string).collect::<Vec<_>>().join("\n"));
    assert!(gaps.is_empty(), "{plugin}: no committed case exercises an input of control kind {} end to end ({} skip(s), first {shown}):\n{}\n[history-edit-census] {census}", gaps.join("; "), skipped.len(), skipped[..shown].join("\n"));
    census
}

/// 📊️ The outcome of a representative-leaf search: a passing case's summary, a broken mechanism, or every case skipped (with the
/// skip report; an app with no editable leaf at all reports zero cases).
pub(super) enum AcceptanceSearch {
    Passed(String),
    Failed(AcceptanceError),
    Exhausted(String),
}

/// 🔎️ The search behind [`assert_history_edits_end_to_end`] and [`assert_documents_reload_identically`]: committed cases first, then
/// derived ones, each scenario on a reloaded document when `reload`.
pub(super) async fn acceptance_search<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource], reload: bool,cx:&mut semio_framework_job::StepContext<'_>) -> AcceptanceSearch
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut skipped = Vec::new();
    let committed = acceptance_cases::<A>(fixtures);
    let mut counts = vec![committed.len()];
    let mut outcome = acceptance_first_pass::<A, M>(plugin, manifest, &committed, &mut skipped, reload,cx).await;
    for case in committed {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    if matches!(outcome, Ok(None)) {
        let derived = acceptance_example_cases::<A>(fixtures, examples).await;
        counts.push(derived.len());
        outcome = acceptance_first_pass::<A, M>(plugin, manifest, &derived, &mut skipped, reload,cx).await;
        for case in derived {
            ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
        }
    }
    match outcome {
        Ok(Some(summary)) => AcceptanceSearch::Passed(summary),
        Err(failure) => AcceptanceSearch::Failed(failure),
        Ok(None) => {
            let shown = skipped.len().min(40);
            AcceptanceSearch::Exhausted(format!(
                "{plugin}: none of the {} committed and {} derived editable case(s) under {} exercises a history edit end to end ({} skip(s), first {shown}):\n{}",
                counts[0],
                counts.get(1).copied().unwrap_or(0),
                fixtures.display(),
                skipped.len(),
                skipped[..shown].join("\n")
            ))
        }
    }
}
