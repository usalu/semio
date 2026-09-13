//! 🎮️ Rust law over `🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json`.
//!
//! Replays every fixture row against the LIVE renderer mailbox — the same `BoundedCompletionQueue` and
//! `InteractionCheckoutLedger` `RuntimeMailbox::apply_pending_step` calls on the browser — so a change
//! to the admission arithmetic that the TypeScript twin (`🧪️tests/🎮️wgpu-runtime-mailbox-admission/🟦️.ts`)
//! does not follow fails here rather than silently swallowing one frame's input.
//!
//! 🩸️ Written for the defect in `📓️wgpu-server-input-present-2026-09-13.md` §5.2: input crossed the
//! wire, reached `WindowDelegate::handle_event`, was enqueued as `DispatchEvents` — and never reached
//! `Ui::dispatch_event`, because one interaction checkout that was never returned held the queue head
//! and the pump that would have drained it ran once per frame BUILD, not once per frame.

use super::{BoundedCompletionQueue, Completion, InteractionCheckoutLedger, InteractionCheckoutStep, INTERACTION_CHECKOUT_CREDITS};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json");

/// 🔟️ The widest bound any row asks for; `CAPACITY` is a const generic, so the replay runs one queue
/// per declared capacity rather than a runtime field.
const WIDE: usize = 8;
const NARROW: usize = 4;

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("runtime mailbox admission fixture parses")
}

/// 🔑️ The oracle spells only names the renderer itself mints, so the replay resolves them to the very
/// `&'static str` the production code uses instead of leaking one per row.
fn interned(value: &Value, field: &str) -> Option<&'static str> {
    match value.get(field).and_then(Value::as_str) {
        None => None,
        Some("window-metrics") => Some("window-metrics"),
        Some("dispatch-event") => Some("dispatch-event"),
        Some("frame-deferred") => Some("frame-deferred"),
        Some("submit-interaction") => Some("submit-interaction"),
        Some(other) => panic!("the fixture names {other}, which the renderer never mints"),
    }
}

struct Replay<const CAPACITY: usize> {
    queue: BoundedCompletionQueue<u64, CAPACITY>,
    ledger: InteractionCheckoutLedger,
    available: bool,
    notices: Vec<(String, u32)>,
}

impl<const CAPACITY: usize> Replay<CAPACITY> {
    fn new() -> Self {
        Self { queue: BoundedCompletionQueue::new(), ledger: InteractionCheckoutLedger::default(), available: true, notices: Vec::new() }
    }

    fn step(&mut self, row: &str, index: usize, step: &Value) {
        let op = step["op"].as_str().expect("every step names an operation");
        let repeat = step.get("repeat").and_then(Value::as_u64).unwrap_or(1);
        for iteration in 0..repeat {
            let last = iteration + 1 == repeat;
            match op {
                "enqueue" => {
                    let revision = step["revision"].as_u64().expect("enqueue names a revision");
                    let completion = Completion {
                        key: interned(step, "key"),
                        revision,
                        requires_interaction: step["requiresInteraction"].as_bool().expect("enqueue names its interaction need"),
                        apply: revision,
                    };
                    let admitted = self.queue.enqueue(completion);
                    assert_eq!(admitted, step["admitted"].as_bool().expect("enqueue names its answer"), "{row} step {index}: enqueue admission");
                }
                "reserveInteraction" => {
                    let admitted = self.queue.reserve_interaction();
                    assert_eq!(admitted, step["admitted"].as_bool().expect("reserve names its answer"), "{row} step {index}: interaction reserve");
                }
                "finish" => {
                    let revision = step["revision"].as_u64().expect("finish names a revision");
                    self.queue.finish(Completion {
                        key: interned(step, "key"),
                        revision,
                        requires_interaction: step["requiresInteraction"].as_bool().expect("finish names its interaction need"),
                        apply: revision,
                    });
                }
                "checkOut" => {
                    let site = interned(step, "site").expect("check-out names a site");
                    let admitted = self.ledger.check_out(site);
                    assert_eq!(admitted, step["admitted"].as_bool().expect("check-out names its answer"), "{row} step {index}: checkout admission");
                    if admitted {
                        self.available = false;
                    }
                }
                "checkIn" => {
                    self.ledger.check_in();
                    self.available = true;
                }
                "apply" => {
                    let head_requires = self.queue.head_requires_interaction();
                    let admission = self.ledger.admit(head_requires, self.available);
                    let index_applied = self.queue.first_applicable(self.available);
                    let applied = index_applied.and_then(|at| self.queue.take_at(at)).map(|completion| completion.revision);
                    if let Some((site, opportunities)) = self.ledger.take_stale_notice() {
                        self.notices.push((site.to_string(), opportunities));
                    }
                    if !last {
                        continue;
                    }
                    let expected = match step["admission"].as_str().expect("apply names its admission") {
                        "admitted" => InteractionCheckoutStep::Admitted,
                        "deferred" => InteractionCheckoutStep::Deferred,
                        "stale" => InteractionCheckoutStep::Stale,
                        other => panic!("{row} step {index}: unknown admission {other}"),
                    };
                    assert_eq!(admission, expected, "{row} step {index}: admission verdict");
                    assert_eq!(applied, step["appliedRevision"].as_u64(), "{row} step {index}: applied revision");
                    assert_eq!(u64::from(self.ledger.opportunities()), step["opportunities"].as_u64().expect("apply names the checkout age"), "{row} step {index}: checkout age");
                }
                other => panic!("{row} step {index}: unknown operation {other}"),
            }
        }
    }

    fn expect(&self, row: &str, expect: &Value) {
        assert_eq!(self.queue.len(), expect["length"].as_u64().expect("row names a final length") as usize, "{row}: final mailbox length");
        let ready: Vec<u64> = self.queue.ready.iter().map(|completion| completion.revision).collect();
        let expected: Vec<u64> = expect["readyRevisions"].as_array().expect("row names its ready revisions").iter().map(|value| value.as_u64().expect("revision")).collect();
        assert_eq!(ready, expected, "{row}: ready order");
        let notices: Vec<(String, u32)> = expect["staleNotices"]
            .as_array()
            .expect("row names its stale notices")
            .iter()
            .map(|notice| (notice["site"].as_str().expect("notice site").to_string(), notice["opportunities"].as_u64().expect("notice age") as u32))
            .collect();
        assert_eq!(self.notices, notices, "{row}: stale checkout diagnostics");
    }
}

fn replay(row: &Value) {
    let id = row["id"].as_str().expect("every row is named");
    let capacity = row["capacity"].as_u64().expect("every row names a capacity");
    let steps = row["steps"].as_array().expect("every row declares steps");
    match capacity {
        capacity if capacity as usize == WIDE => {
            let mut replay = Replay::<WIDE>::new();
            for (index, step) in steps.iter().enumerate() {
                replay.step(id, index, step);
            }
            replay.expect(id, &row["expect"]);
        }
        capacity if capacity as usize == NARROW => {
            let mut replay = Replay::<NARROW>::new();
            for (index, step) in steps.iter().enumerate() {
                replay.step(id, index, step);
            }
            replay.expect(id, &row["expect"]);
        }
        other => panic!("{id}: fixture capacity {other} has no mounted queue"),
    }
}

#[test]
fn every_fixture_row_replays_on_the_live_mailbox() {
    let fixture = fixture();
    let rows = fixture["rows"].as_array().expect("fixture declares rows");
    assert!(!rows.is_empty(), "the admission oracle is empty");
    for row in rows {
        replay(row);
    }
}

#[test]
fn the_fixture_pins_the_credits_the_ledger_actually_spends() {
    let fixture = fixture();
    assert_eq!(fixture["credits"].as_u64().expect("fixture names the credits"), u64::from(INTERACTION_CHECKOUT_CREDITS), "the oracle and the ledger must agree on the bounded-step credits");
}
