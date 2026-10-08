//! 🧮️ Composite-plan folding and the fold-footprint law: store-side machinery that turns a composite kind's plan into one
//! forward diff and its inverse rows. It routes through the central applier ([`apply_diff`]); no mutation leaf lives here.
use super::command::{apply_diff, worst_level, CompositeMutationKind, Mutation, MutationDiff, MutationMessage, MutationOutcome, PlanError, PlanStep, Planner};

/// 🧬️ Folds a composite's LOCAL steps into one [`MutationOutcome`] via
/// [`MutationDiff::absorb`], applying each step against the snapshot as it stood right before that
/// step (matching [`Planner::call`]'s own advance-as-you-go semantics) — so a successful
/// `apply_diff(fold_plan_diff(k, b).diff(), &b)` equals sequential application of the plan's local steps.
/// Foreign steps never contribute to the folded diff (LAW 5 of the contract freeze). **All-or-
/// nothing** (§C4): if planning itself fails (`PlanError`) or any step's messages reach `Error` or
/// worse, the returned diff is empty (`Default::default()`) — but every message collected along the
/// way is still kept, so a caller sees exactly why. A `PlanError` additionally contributes one
/// `Fatal` message: a [`PlanError::Refused`] composite precondition its own domain-coded refusal
/// (`mutation.target-missing`, `mutation.duplicate-id`, …), any other planning failure
/// `"mutation.invariant"`. Never panics, matching this fn's frozen non-`Result` signature.
pub fn fold_plan_diff<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff> {
    let mut planner = Planner::new(base);
    let plan_result = kind.plan(base, &mut planner);
    let (steps, mut messages) = planner.into_parts();
    if let Err(error) = &plan_result {
        messages.push(match error {
            PlanError::Refused(refusal) => refusal.clone(),
            other => MutationMessage::fatal("mutation.invariant", other.to_string()),
        });
    }
    let rejected = plan_result.is_err() || matches!(worst_level(&messages), Some(level) if level >= semio_framework_diagnostic::Severity::Error);
    if rejected {
        return MutationOutcome::new(<Op as Mutation<P>>::Diff::default()).absorb_messages(messages);
    }

    let mut current = base.clone();
    let mut folded = <Op as Mutation<P>>::Diff::default();
    for step in steps {
        if let PlanStep::Local(op) = step {
            let diff = op.diff(&current).into_parts().0;
            match apply_diff(&diff, &current) {
                Ok(next) => current = next,
                Err(error) => {
                    messages.push(MutationMessage::fatal("mutation.invariant", error.to_string()).at(error.target));
                    return MutationOutcome::new(<Op as Mutation<P>>::Diff::default()).absorb_messages(messages);
                }
            }
            folded.absorb(diff);
        }
    }
    MutationOutcome::new(folded).absorb_messages(messages)
}

/// ↩️ Stores each local step's inverse against its own pre-state in forward local-step order.
/// The returned flat vector uses the same storage order as [`Mutation::inverse`]; Store reverses
/// that entire vector once when applying it. Preserving both the group order and each group's
/// stored order is required for checked or otherwise noncommutative steps. A planning failure
/// folds to an empty vector.
pub fn fold_plan_inverse<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let mut planner = Planner::new(base);
    if let Err(error) = kind.plan(base, &mut planner) {
        let (steps, _) = planner.into_steps_with_pre_states();
        for step in steps {
            if let PlanStep::Local(op) = step { op.retire_cold(); }
        }
        return Err(error.into_value_error());
    }
    let (steps, pre_states) = planner.into_steps_with_pre_states();
    let mut pending = steps.into_iter().zip(pre_states);
    let mut inverses = Vec::new();
    while let Some((step, pre_state)) = pending.next() {
        if let PlanStep::Local(op) = step {
            let Some(pre_state) = pre_state else {
                op.retire_cold();
                for value in inverses { <Op as Mutation<P>>::retire_cold(value); }
                for (step, _) in pending { if let PlanStep::Local(value) = step { value.retire_cold(); } }
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "local inverse pre-state is missing"));
            };
            let result = op.inverse(&pre_state);
            op.retire_cold();
            match result {
                Ok(values) => inverses.extend(values),
                Err(error) => {
                    for value in inverses { <Op as Mutation<P>>::retire_cold(value); }
                    for (step, _) in pending { if let PlanStep::Local(value) = step { value.retire_cold(); } }
                    return Err(error);
                }
            }
        }
    }
    Ok(inverses)
}

/// 🧾️ The fold-footprint law of an aggregate (design §20.5, census L3): for every committed fixture case under `root` whose
/// `🦠️mutation` decodes as `M` and whose `📸️snapshot/⬅️before` decodes as `P`, the rows `Mutation::inverse` yields on that base
/// never exceed the leaf's declared [`Mutation::inverse_rows`] — so `ArtifactStore::fold_batch_item` never refuses a
/// footprint `ArtifactStoreOneItemFootprint::for_leaf` declared. Returns one line per breach and the number of cases checked. A
/// decoded base is never dropped: a snapshot may own fail-closed roots that only its store retires.
#[cfg(any(test, feature = "mutation-testing"))]
pub fn mutation_inverse_rows_failures<P: crate::FromValue, M: Mutation<P> + crate::FromValue>(root: &std::path::Path) -> (Vec<String>, usize) {
    fn walk(directory: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if path.is_dir() {
                if name == "🦠️mutation" {
                    if let Some(case) = path.parent() {
                        found.push(case.to_path_buf());
                    }
                } else if !name.starts_with('.') && !["target", "dist", "node_modules"].contains(&name.as_str()) {
                    walk(&path, found);
                }
            }
        }
    }
    let decode = |path: std::path::PathBuf| std::fs::read_to_string(path).ok().and_then(|text| semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).map(|json| semio_framework_pack_json::to_dsl_value(&json));
    let mut cases = Vec::new();
    walk(root, &mut cases);
    cases.sort();
    let (mut failures, mut checked) = (Vec::new(), 0usize);
    for case in cases {
        let Some(op) = decode(case.join("🦠️mutation").join("🔣️.json")).and_then(|value| M::from_value(value).ok()) else { continue };
        let Some(before) = decode(case.join("📸️snapshot").join("⬅️before").join("🔣️.json")).and_then(|value| P::from_value(value).ok()) else {
            Mutation::<P>::retire_cold(op);
            continue;
        };
        let before = std::mem::ManuallyDrop::new(before);
        let inverse = match op.inverse(&before) {
            Ok(inverse) => inverse,
            Err(error) => {
                failures.push(format!("{} ({}): inverse refused: {}", case.display(), op.descriptor().semantic_kind, error.into_message()));
                Mutation::<P>::retire_cold(op);
                checked += 1;
                continue;
            }
        };
        let (actual, declared) = (inverse.len(), Mutation::<P>::inverse_rows(&op));
        if actual > declared {
            failures.push(format!("{} ({}): inverse yields {actual} row(s), the leaf declares {declared}", case.display(), op.descriptor().semantic_kind));
        }
        for row in inverse {
            Mutation::<P>::retire_cold(row);
        }
        Mutation::<P>::retire_cold(op);
        checked += 1;
    }
    (failures, checked)
}
