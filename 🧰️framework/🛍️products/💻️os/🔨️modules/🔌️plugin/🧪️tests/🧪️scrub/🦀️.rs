//! 🎚️ Runtime laws of the continuous-control scrub (design §13.1): the render overlay folds every open press's absolute
//! leaves over the committed document without touching it, refolds only when a press changed or the document moved, and
//! hands every displaced alias back for store retirement; operation tags are bounded and taken once.

use super::*;
use crate::test_app_mutation_fixture::{SetCount, SetLabel, TestMutation, TestSnapshot};
use semio_framework_tool_machine::{ScrubInput, ToolStep};

fn committed(count: i32, label: &str) -> Arc<TestSnapshot> {
    Arc::new(TestSnapshot { count, label: label.into(), ..TestSnapshot::default() })
}

fn tick(runtime: &mut ScrubRuntime<TestSnapshot, TestMutation>, window: &str, gesture: &str, leaves: Vec<TestMutation>, clock: u64) -> ToolStep<TestMutation> {
    let clock = protocol::HybridLogicalTimestamp { actor: 0, physical_ms: clock, logical: 0 };
    runtime.ledger.send(window, "s.test@1/*#editor#setCount", &protocol::ActorId("actor".into()), "r1", ScrubInput::Tick { gesture: gesture.into(), leaves }, clock).expect("a tick is never refused")
}

fn release(runtime: &mut ScrubRuntime<TestSnapshot, TestMutation>, window: &str, gesture: &str, leaves: Vec<TestMutation>) -> ToolStep<TestMutation> {
    let clock = protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 9_000, logical: 0 };
    runtime.ledger.send(window, "s.test@1/*#editor#setCount", &protocol::ActorId("actor".into()), "r1", ScrubInput::Commit { gesture: gesture.into(), leaves }, clock).expect("a release is never refused")
}

/// ⚖️ LAW: while a press is open every render reads committed ⊕ its absolute leaves — two windows' presses fold in window
/// order — and the committed document itself never changes; with no press open the render reads `committed` again.
#[test]
fn the_overlay_folds_open_presses_over_an_untouched_committed_document() {
    let mut runtime = ScrubRuntime::<TestSnapshot, TestMutation>::default();
    let head = committed(1, "a");
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &head), "no press, no overlay");
    assert!(matches!(tick(&mut runtime, "w1", "g1", vec![SetCount { value: 5 }.into()], 1), ToolStep::Open));
    assert!(matches!(tick(&mut runtime, "w2", "g2", vec![SetLabel { value: "z".into() }.into()], 2), ToolStep::Open));
    assert_eq!(runtime.follow(&head, 1, true).len(), 1, "the first fold displaces only its intermediate");
    let overlay = runtime.overlay_or(&head);
    assert_eq!((overlay.count, overlay.label.as_str()), (5, "z"));
    assert_eq!((head.count, head.label.as_str()), (1, "a"), "the committed document is untouched");
    assert!(matches!(release(&mut runtime, "w1", "g1", vec![SetCount { value: 6 }.into()]), ToolStep::Committed(..)));
    assert!(matches!(runtime.ledger.abort("w2", Some("g2"), semio_framework_tool_machine::ToolAbortReason::Blur), ToolStep::Aborted(..)));
    assert_eq!(runtime.follow(&head, 1, true).len(), 1, "the dropped overlay goes back for retirement");
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &head), "every press closed, the render reads committed again");
}

/// ⚖️ LAW: an unchanged press over an unchanged document keeps its overlay; a moved document refolds the same absolute
/// leaves on the new head and displaces every intermediate and the previous overlay; a leaf the base refuses is skipped.
#[test]
fn the_overlay_refolds_only_when_a_press_changed_or_the_document_moved() {
    let mut runtime = ScrubRuntime::<TestSnapshot, TestMutation>::default();
    tick(&mut runtime, "w1", "g1", vec![SetCount { value: 7 }.into(), SetLabel { value: "q".into() }.into()], 1);
    let head = committed(1, "a");
    assert_eq!(runtime.follow(&head, 1, true).len(), 1, "the intermediate of the two-leaf fold is displaced");
    let first = Arc::clone(runtime.overlay_or(&head));
    assert!(runtime.follow(&head, 1, false).is_empty(), "same press, same document: no refold");
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &first));
    let moved = committed(3, "b");
    assert_eq!(runtime.follow(&moved, 2, false).len(), 2, "the moved document refolds: previous overlay and intermediate displaced");
    let overlay = runtime.overlay_or(&moved);
    assert_eq!((overlay.count, overlay.label.as_str()), (7, "q"), "absolute leaves fold on any base");
}

/// ⚖️ LAW: an operation's tag is taken exactly once, and the tags of operations that never complete are bounded by the
/// live operation slots — the oldest yields its place.
#[test]
fn operation_tags_are_taken_once_and_bounded() {
    let mut runtime = ScrubRuntime::<TestSnapshot, TestMutation>::default();
    let tag = |index: u64| ScrubTag { window: "w".into(), tool: "t#v".into(), phase: semio_framework_tool_machine::ScrubPhase::Tick { gesture: format!("g{index}") } };
    for operation in 0..(ARTIFACT_LIVE_OUTPUT_SLOTS as u64 + 3) {
        runtime.bind(operation, tag(operation));
    }
    assert_eq!(runtime.operations.len(), ARTIFACT_LIVE_OUTPUT_SLOTS);
    assert_eq!(runtime.take_operation(0), None, "the oldest never-completed tag yielded its place");
    assert_eq!(runtime.take_operation(5), Some(tag(5)));
    assert_eq!(runtime.take_operation(5), None, "a tag is taken once");
}
