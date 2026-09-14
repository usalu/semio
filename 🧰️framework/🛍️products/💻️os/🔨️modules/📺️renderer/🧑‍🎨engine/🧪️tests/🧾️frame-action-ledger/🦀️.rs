//! 🧾️ Rust law over `🧫️fixtures/🧾️frame-action-ledger/🔣️.json`.
//!
//! Replays every fixture row against the LIVE renderer owners — the same [`FrameActionOwners`] the
//! frame's authorities push into and the same [`FrameDeferredCursor`] `start_frame_deferred` hands the
//! shell — so the answer here is the runtime's answer, not a second model of it. The TypeScript twin
//! (`🧪️tests/🧾️frame-action-ledger/🟦️.ts`) re-derives the same rows from the fixture alone.
//!
//! 🩸️ Written for the defect in `📓️wgpu-deferred-action-commit-2026-09-13.md`: on 6118 the inline
//! rename editor's `renameGeneration` commit reached `AppFrameTransactionPhase::InputEvents` with the
//! right arguments — and `AppFrameTransactionStep::Superseded` dropped the frame candidate holding it
//! 1 ms later. `take_action_step` had already taken the action OUT of the bounded input authority, so
//! nothing else owned it: no dispatch, no fault, not even a retirement. Every retained control's
//! commit crossed that same queue.

use super::*;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🧾️frame-action-ledger/🔣️.json");
const RENDERER_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
const FRAME_JOB_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");

/// 🧾️ One fixture row's frame runtime: the ledger its `owner` declares, the deferred owner a
/// completion mints from it, and the two answers the row is scored on.
struct Replay {
    runtime_owned: bool,
    ledger: FrameActionOwners,
    candidate: FrameActionOwners,
    deferred: Option<FrameDeferredCursor>,
    dispatched: Vec<String>,
    lost: Vec<String>,
}

impl Replay {
    fn new(owner: &str) -> Self {
        let runtime_owned = match owner {
            "runtime" => true,
            "candidate" => false,
            other => panic!("the fixture names owner {other}, which the renderer never had"),
        };
        Self { runtime_owned, ledger: FrameActionOwners::default(), candidate: FrameActionOwners::default(), deferred: None, dispatched: Vec::new(), lost: Vec::new() }
    }

    fn queue(&mut self) -> &mut FrameActionOwners {
        if self.runtime_owned {
            &mut self.ledger
        } else {
            &mut self.candidate
        }
    }

    fn discard_candidate(&mut self) {
        if self.runtime_owned {
            return;
        }
        while let Some(action) = self.candidate.pop_front() {
            self.lost.push(action.action);
        }
    }

    fn step(&mut self, row: &str, index: usize, step: &Value) {
        let op = step["op"].as_str().expect("every step names an operation");
        match op {
            "mint" => {
                let action = step["action"].as_str().expect("a mint names its action").to_string();
                let descriptor = ActionDescriptor { controller_id: "s.procedural.generation3d@1/*#editor".to_string(), action, args: None };
                assert!(self.queue().try_push(descriptor).is_ok(), "{row} step {index}: the ledger admits the minted action");
            }
            "supersede" | "retire" => self.discard_candidate(),
            "complete" => {
                let installs = step["installs"].as_bool().expect("a completion declares whether it installs");
                let live = self.deferred.is_some();
                let empty = if self.runtime_owned { self.ledger.is_empty() } else { self.candidate.is_empty() };
                assert_eq!(installs, !live && !empty, "{row} step {index}: the completion installs exactly when no owner is live and the ledger is not empty");
                if !installs {
                    return;
                }
                let actions = std::mem::take(self.queue());
                self.deferred = Some(FrameDeferredCursor::new(actions, false, false, false, false, 1, semio_framework_job::root_cancel_token()));
            }
            "dispatch" => {
                let expected = step["action"].as_str();
                let Some(cursor) = self.deferred.as_mut() else {
                    assert!(expected.is_none(), "{row} step {index}: no deferred owner can hand the shell {expected:?}");
                    return;
                };
                match cursor.take_next() {
                    Some(FrameDeferredWork::Action(action)) => {
                        assert_eq!(Some(action.action.as_str()), expected, "{row} step {index}: the owner hands the shell the action the fixture names");
                        self.dispatched.push(action.action);
                    }
                    None => {
                        assert!(expected.is_none(), "{row} step {index}: the owner still owes {expected:?}");
                        assert!(self.deferred.as_ref().is_some_and(FrameDeferredCursor::terminal_is_empty), "{row} step {index}: a drained owner is terminal-empty");
                        self.deferred = None;
                    }
                    Some(_) => panic!("{row} step {index}: this replay mints no maintenance work"),
                }
            }
            other => panic!("the fixture names operation {other}, which the renderer never performs"),
        }
    }
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("frame action ledger fixture parses")
}

#[test]
fn a_commit_outlives_every_frame_candidate_that_was_discarded_before_it_dispatched() {
    let fixture = fixture();
    let rows = fixture["rows"].as_array().expect("the fixture declares rows");
    assert!(rows.len() >= 9, "the oracle keeps both owner models and every discard path");
    for row in rows {
        let id = row["id"].as_str().expect("every row is named");
        let mut replay = Replay::new(row["owner"].as_str().expect("every row names its owner"));
        for (index, step) in row["steps"].as_array().expect("every row has steps").iter().enumerate() {
            replay.step(id, index, step);
        }
        let expected_dispatched: Vec<&str> = row["expect"]["dispatched"].as_array().expect("dispatched").iter().map(|value| value.as_str().expect("an action name")).collect();
        let expected_lost: Vec<&str> = row["expect"]["lost"].as_array().expect("lost").iter().map(|value| value.as_str().expect("an action name")).collect();
        assert_eq!(replay.dispatched, expected_dispatched, "{id}: the actions the shell was handed");
        assert_eq!(replay.lost, expected_lost, "{id}: the actions no owner ever handed anyone");
        let depth = row["expect"]["ledgerDepth"].as_u64().expect("ledgerDepth");
        assert_eq!(replay.ledger.is_empty() && replay.candidate.is_empty(), depth == 0, "{id}: the ledger depth the row ends on");
    }
}

#[test]
fn the_ledger_is_the_runtimes_and_no_frame_candidate_owns_one() {
    assert!(RENDERER_SOURCE.contains("    frame_actions: FrameActionOwners,"), "the runtime declares the ledger");
    assert!(RENDERER_SOURCE.contains("app.frame_actions.try_push(action)"), "the frame's authorities push onto the RUNTIME's ledger");
    assert!(RENDERER_SOURCE.contains("std::mem::take(&mut self.frame_actions)"), "only a completing frame takes it, and takes all of it");
    for candidate in ["struct FrameTransaction {", "struct FrameBuildCursor {", "struct AppFrameAfterChrome {", "struct FrameFinishCursor {"] {
        let start = RENDERER_SOURCE.find(candidate).unwrap_or_else(|| panic!("{candidate} still exists"));
        let body = &RENDERER_SOURCE[start..start + RENDERER_SOURCE[start..].find("\n}").expect("a struct body ends")];
        assert!(!body.contains("deferred_actions"), "{candidate} must own no action queue — a frame candidate is discardable by construction");
    }
    assert!(!RENDERER_SOURCE.contains("deferred_actions.pop_front()"), "no close ladder retires a user's commit as a retirement unit");
    assert!(FRAME_JOB_SOURCE.contains("AppFrameTransactionStep::Superseded => {\n                        self.phase = ActiveFramePhase::Terminal;"), "a superseded candidate is still dropped rather than closed, which is what makes the ledger's owner load-bearing");
}

/// 🧹️ A STALE world3d draw rebuild is closed, never recorded as a frame fault.
///
/// `record_frame_fault` is terminal on this target — `BrowserFrameTransport.quarantine()` closes the
/// surface for good — so every arm that reaches it decides whether a session survives. The world
/// module's own rule is that `WorldDrawRebuildStep::Stale` means "this cursor's revision/generation
/// was overtaken; close it down its ladder and begin the next one"
/// (`♾️infinite/🌍️world` §`retained_draw_rebuild_stale_and_interrupted_close_never_publish`), and the
/// reference driver `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` faults on `Fault` alone.
/// This host once faulted on both: measured on 6118, a hover on the ELEVENTH gesture of a session
/// published `frame-credits: world3d retained draw rebuild faulted` and froze the frame wire at
/// 3 751 batches for the remaining nine gestures (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn a_stale_world3d_draw_rebuild_is_closed_and_never_quarantines_the_surface() {
    let start = RENDERER_SOURCE.find("match step_world3d_draw_rebuild(state, context) {").expect("the host steps the retained draw rebuild");
    let arm = &RENDERER_SOURCE[start..start + RENDERER_SOURCE[start..].find("\n                }").expect("the match ends")];
    assert!(arm.contains("WorldDrawRebuildStep::Stale => {"), "`Stale` has its own arm");
    assert!(arm.contains("close_world3d_draw_rebuild_step(state, context)"), "a stale rebuild is CLOSED down the module's own ladder");
    assert!(
        !arm.contains("WorldDrawRebuildStep::Stale | WorldDrawRebuildStep::Fault"),
        "`Stale` is never lumped in with `Fault` — `record_frame_fault` quarantines the surface for good"
    );
    let fault_arm = &arm[arm.find("WorldDrawRebuildStep::Fault").expect("`Fault` still faults")..];
    assert!(fault_arm.contains("record_frame_fault(\"world3d retained draw rebuild faulted\")"), "a real `Fault` still records the frame fault");
}
