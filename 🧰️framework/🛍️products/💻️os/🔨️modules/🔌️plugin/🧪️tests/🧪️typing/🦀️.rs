//! ⌨️ Runtime laws of the typing run (design §13.2): the render overlay folds every open run's net leaves over the committed
//! document without touching it, a run whose leaves a moved document refuses aborts with zero trace (`baseMoved`), and a
//! frozen document drops every run.

use super::*;
use crate::test_app_mutation_fixture::{SetCount, SetLabel, SetSlotChildren, TestMutation, TestSnapshot};
use semio_framework_tool_machine::{ToolStep, TypingFold, TypingInput};

fn committed(count: i32, label: &str) -> Arc<TestSnapshot> {
    Arc::new(TestSnapshot { count, label: label.into(), ..TestSnapshot::default() })
}

fn replace(_net: &[TestMutation], next: &[TestMutation]) -> TypingFold<TestMutation> {
    TypingFold::Net(next.to_vec())
}

fn type_into(runtime: &mut ToolMachineRuntime<TestSnapshot, TestMutation>, window: &str, leaves: Vec<TestMutation>, clock: u64) -> Vec<ToolStep<TestMutation>> {
    let clock = HybridLogicalTimestamp { actor: 0, physical_ms: clock, logical: 0 };
    runtime.typing.send(window, "s.test@1/*#editor#setLabel", &ActorId("actor".into()), TypingInput::Edit { buffer: "label".into(), leaves }, replace, clock).expect("typing is never refused")
}

/// ⚖️ LAW: while a run is open every render reads committed ⊕ its net leaves — a press and a run fold together, the press
/// first — and the committed document itself never changes; once the run commits the render reads `committed` again.
#[test]
fn the_overlay_folds_open_runs_over_an_untouched_committed_document() {
    let mut runtime = ToolMachineRuntime::<TestSnapshot, TestMutation>::default();
    let head = committed(1, "a");
    assert!(matches!(type_into(&mut runtime, "w1", vec![SetLabel { value: "h".into() }.into()], 1_000).as_slice(), [ToolStep::Open]));
    assert!(matches!(type_into(&mut runtime, "w1", vec![SetLabel { value: "hello".into() }.into()], 1_100).as_slice(), [ToolStep::Open]));
    let clock = HybridLogicalTimestamp { actor: 0, physical_ms: 1_101, logical: 0 };
    runtime
        .presses
        .send("w2", "s.test@1/*#editor#setCount", &ActorId("actor".into()), "r1", semio_framework_tool_machine::ScrubInput::Tick { gesture: "g".into(), leaves: vec![PressLeaf::Member(SetCount { value: 5 }.into())] }, clock)
        .expect("a tick");
    let (displaced, aborted) = runtime.follow(&head, 1, true);
    assert_eq!((displaced.len(), aborted.len()), (1, 0), "the press's intermediate is displaced, no run conflicts");
    let overlay = runtime.overlay_or(&head);
    assert_eq!((overlay.count, overlay.label.as_str()), (5, "hello"));
    assert_eq!((head.count, head.label.as_str()), (1, "a"), "the committed document is untouched");
    let clock = HybridLogicalTimestamp { actor: 0, physical_ms: 1_200, logical: 0 };
    assert!(matches!(runtime.typing.commit("w1", TypingCommit::Blur, clock), Ok(ToolStep::Committed(_, ref leaves)) if leaves == &vec![TestMutation::from(SetLabel { value: "hello".into() })]));
    runtime.presses.abort("w2", None, ToolAbortReason::Blur);
    assert_eq!(runtime.follow(&head, 1, true).0.len(), 1, "the dropped overlay goes back for retirement");
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &head), "every run committed, the render reads committed again");
}

/// ⚖️ LAW: a moved document refolds every open run; a run whose leaves the new head refuses is a conflict and aborts with
/// zero trace (`baseMoved`), while the runs it does not conflict with stay open on the new head.
#[test]
fn a_run_the_moved_document_refuses_aborts_with_zero_trace() {
    let mut runtime = ToolMachineRuntime::<TestSnapshot, TestMutation>::default();
    type_into(&mut runtime, "w1", vec![SetLabel { value: "kept".into() }.into()], 1_000);
    type_into(&mut runtime, "w2", vec![SetSlotChildren { children: vec!["not a child uri".into()] }.into()], 1_001);
    let head = committed(1, "a");
    let (_, aborted) = runtime.follow(&head, 1, true);
    assert!(matches!(aborted.as_slice(), [ToolStep::Aborted(_, ToolAbortReason::BaseMoved)]), "the refused run aborts: {aborted:?}");
    assert_eq!(runtime.typing.windows().collect::<Vec<_>>(), vec!["w1"], "only the conflicting run is dropped");
    assert_eq!(runtime.overlay_or(&head).label, "kept");
}
