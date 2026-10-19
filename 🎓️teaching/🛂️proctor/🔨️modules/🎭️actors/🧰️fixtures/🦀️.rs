//! 🧰️ The ids, envelopes, admission and perfect answers the proctor's own tests play, shared with the tests of the crates above (`testkit`).

use super::*;
use server::contract::HybridLogicalClock;

pub const TENANT: &str = "proctor-fixture";

pub const ADA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

pub const BOB: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

/// 📨️ The §9a envelope of one quiz command, as a client sends it.
pub fn envelope(command: &Command) -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId(command.id().clone()),
        kind: command_kind(command),
        version: WIRE_VERSION,
        target: command_target(command, TENANT).unwrap_or_else(|_| learner_key(TENANT, command.learner())),
        scope: Scope(TENANT.to_string()),
        principal: Principal::Anonymous,
        session: None,
        device: None,
        payload: serde_json::to_vec(command).unwrap(),
        causal_frontier: None,
        client_hlc: HybridLogicalClock::default(),
        expected_revision: None,
        idempotency_key: Some(IdempotencyKey(command.id().clone())),
        capability_proof: None,
        trace: TraceContext::default(),
    }
}

pub fn id(seed: u8) -> String {
    format!("{seed:032x}")
}

/// 🛃️ An admission of the fixture catalog under the default caps, counting `learners`.
pub fn admission(learners: u64) -> Arc<Admission> {
    Arc::new(Admission::new(TENANT, Arc::new(AtomicU64::new(learners))))
}

/// 💯️ The perfect answer to one presented task, read off the quiz's solutions: the keys assigned
/// where the sheet shows them, the true values guessed where it hides them.
pub fn perfect(quiz: &course::Quiz, presented: &course::SheetTask) -> course::Answer {
    use course::{Answer, ClassificationAnswer, MatchingAnswer, SheetTask, SortingAnswer, Task};
    let task = quiz.tasks.iter().find(|task| task.id() == presented.id()).expect("the sheet task exists");
    match (task, presented) {
        (Task::Sorting(task), SheetTask::Sorting(sheet)) => {
            let mut order: Vec<&course::SortingItem> = task.items.iter().filter(|item| sheet.items.iter().any(|shown| shown.id == item.id)).collect();
            order.sort_by(|left, right| left.value.total_cmp(&right.value));
            let guesses = sheet.keys.is_none().then(|| order.iter().map(|item| (item.id.clone(), item.value)).collect());
            Answer::Sorting(SortingAnswer { order: order.iter().map(|item| item.id.clone()).collect(), guesses })
        }
        (Task::Classification(task), SheetTask::Classification(sheet)) => Answer::Classification(ClassificationAnswer { assignments: sheet.items.iter().map(|shown| (shown.id.clone(), task.items.iter().find(|item| item.id == shown.id).expect("item").category.clone())).collect() }),
        (Task::Matching(task), SheetTask::Matching(sheet)) => {
            let value = |item: &str, dimension: &str| task.items.iter().find(|defined| defined.id == item).expect("item").values[dimension];
            let assignments = sheet.dimensions.iter().filter_map(|dimension| {
                let cards = dimension.cards.as_ref()?;
                let mut used = std::collections::BTreeSet::new();
                let assigned = sheet.items.iter().map(|shown| {
                    let card = (0..cards.len()).find(|card| cards[*card] == value(&shown.id, &dimension.id) && !used.contains(card)).expect("a card of that value");
                    used.insert(card);
                    (shown.id.clone(), card)
                });
                Some((dimension.id.clone(), assigned.collect()))
            });
            let guesses = sheet.dimensions.iter().filter(|dimension| dimension.cards.is_none()).map(|dimension| (dimension.id.clone(), sheet.items.iter().map(|shown| (shown.id.clone(), value(&shown.id, &dimension.id))).collect()));
            let (assignments, guesses): (std::collections::BTreeMap<_, _>, std::collections::BTreeMap<_, _>) = (assignments.collect(), guesses.collect());
            Answer::Matching(MatchingAnswer { assignments: (!assignments.is_empty()).then_some(assignments), guesses: (!guesses.is_empty()).then_some(guesses) })
        }
        _ => panic!("the sheet task has the quiz task's kind"),
    }
}
