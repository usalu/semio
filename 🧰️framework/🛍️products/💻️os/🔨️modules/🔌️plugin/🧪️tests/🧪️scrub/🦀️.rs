//! 🎚️ Runtime laws of the continuous-control scrub (design §13.1, §20.1): the render overlay folds every open press's absolute
//! leaves over the committed document without touching it, refolds only when a press changed or the document moved, and
//! hands every displaced alias back for store retirement; every lane of a press rides ONE ledger step; operation tags are
//! bounded and taken once.

use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ScrubTag;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::ActorId;
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework::kernel::HybridLogicalTimestamp;
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_tool_machine::ScrubPhase;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
use semio_framework_plugin_window_config::WindowConfigOwner;
#[cfg(test)]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_tool_machine::{ScrubInput, ToolAbortReason, ToolStep};
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::PressLeaf;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolTag;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;

#[doc(hidden)]
pub type Runtime = ToolMachineRuntime<TestSnapshot, TestMutation, TestSnapshot, TestMutation>;
type Leaf = PressLeaf<TestMutation, TestMutation>;

#[doc(hidden)]
pub fn committed(count: i32, label: &str) -> Arc<TestSnapshot> {
    Arc::new(TestSnapshot { count, label: label.into(), ..TestSnapshot::default() })
}

#[doc(hidden)]
pub fn member(mutation: impl Into<TestMutation>) -> Leaf {
    PressLeaf::Member(mutation.into())
}

#[doc(hidden)]
pub fn config(mutation: impl Into<TestMutation>) -> Leaf {
    PressLeaf::Config(mutation.into())
}

#[doc(hidden)]
pub fn tick(runtime: &mut Runtime, window: &str, gesture: &str, leaves: Vec<Leaf>, clock: u64) -> ToolStep<Leaf> {
    let clock = HybridLogicalTimestamp { actor: 0, physical_ms: clock, logical: 0 };
    runtime.presses.send(window, "s.test@1/*#editor#setCount", &ActorId("actor".into()), "r1", &mut Some(ScrubInput::Tick { gesture: gesture.into(), leaves }), clock).expect("a tick is never refused")
}

#[doc(hidden)]
pub fn release(runtime: &mut Runtime, window: &str, gesture: &str, leaves: Vec<Leaf>) -> ToolStep<Leaf> {
    let clock = HybridLogicalTimestamp { actor: 0, physical_ms: 9_000, logical: 0 };
    runtime.presses.send(window, "s.test@1/*#editor#setCount", &ActorId("actor".into()), "r1", &mut Some(ScrubInput::Commit { gesture: gesture.into(), leaves }), clock).expect("a release is never refused")
}

#[doc(hidden)]
pub fn lanes(leaves: Vec<Leaf>) -> (Vec<TestMutation>, Vec<TestMutation>) {
    let mut lanes = (Vec::new(), Vec::new());
    for leaf in leaves {
        match leaf {
            PressLeaf::Member(leaf) => lanes.0.push(leaf),
            PressLeaf::Config(leaf) => lanes.1.push(leaf),
            PressLeaf::Child(_) | PressLeaf::WindowConfig(_) => panic!("no other lane rode the press"),
        }
    }
    lanes
}

/// ⚖️ LAW: while a press is open every render reads committed ⊕ its absolute leaves — two windows' presses fold in window
/// order — and the committed document itself never changes; with no press open the render reads `committed` again.
#[test]
fn the_overlay_folds_open_presses_over_an_untouched_committed_document() {
    let mut runtime = Runtime::default();
    let head = committed(1, "a");
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &head), "no press, no overlay");
    assert!(matches!(tick(&mut runtime, "w1", "g1", vec![member(SetCount { value: 5 })], 1), ToolStep::Open));
    assert!(matches!(tick(&mut runtime, "w2", "g2", vec![member(SetLabel { value: "z".into() })], 2), ToolStep::Open));
    assert_eq!(runtime.follow(&head, 1, true).0.len(), 1, "the first fold displaces only its intermediate");
    let overlay = runtime.overlay_or(&head);
    assert_eq!((overlay.count, overlay.label.as_str()), (5, "z"));
    assert_eq!((head.count, head.label.as_str()), (1, "a"), "the committed document is untouched");
    assert!(matches!(release(&mut runtime, "w1", "g1", vec![member(SetCount { value: 6 })]), ToolStep::Committed(..)));
    assert!(matches!(runtime.presses.abort("w2", Some("g2"), ToolAbortReason::Blur), ToolStep::Aborted(..)));
    assert_eq!(runtime.follow(&head, 1, true).0.len(), 1, "the dropped overlay goes back for retirement");
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &head), "every press closed, the render reads committed again");
}

/// ⚖️ LAW: an unchanged press over an unchanged document keeps its overlay; a moved document refolds the same absolute
/// leaves on the new head and displaces every intermediate and the previous overlay; a leaf the base refuses is skipped.
#[test]
fn the_overlay_refolds_only_when_a_press_changed_or_the_document_moved() {
    let mut runtime = Runtime::default();
    tick(&mut runtime, "w1", "g1", vec![member(SetCount { value: 7 }), member(SetLabel { value: "q".into() })], 1);
    let head = committed(1, "a");
    assert_eq!(runtime.follow(&head, 1, true).0.len(), 1, "the intermediate of the two-leaf fold is displaced");
    let first = Arc::clone(runtime.overlay_or(&head));
    assert!(runtime.follow(&head, 1, false).0.is_empty(), "same press, same document: no refold");
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &first));
    let moved = committed(3, "b");
    assert_eq!(runtime.follow(&moved, 2, false).0.len(), 2, "the moved document refolds: previous overlay and intermediate displaced");
    let overlay = runtime.overlay_or(&moved);
    assert_eq!((overlay.count, overlay.label.as_str()), (7, "q"), "absolute leaves fold on any base");
}

/// ⚖️ LAW (CLOSURE-3): every lane of a press rides ONE ledger step. A tick holds its document and config leaves as
/// provisional overlays over committed values that never change; the release commits both lanes in ONE transaction, so the
/// config edit leaves only together with the document edit; a late release of the closed press is silent on every lane;
/// an abort drops every lane with zero trace and its late release stays silent too.
#[test]
fn every_lane_of_a_press_rides_one_ledger_step() {
    let mut runtime = Runtime::default();
    let (head, settings) = (committed(1, "a"), committed(0, "cfg"));
    tick(&mut runtime, "w1", "g1", vec![member(SetCount { value: 5 }), config(SetLabel { value: "c5".into() })], 1);
    runtime.follow(&head, 1, true);
    runtime.follow_config(&settings, 1, true);
    assert_eq!((runtime.overlay_or(&head).count, runtime.config_overlay_or(&settings).label.as_str()), (5, "c5"), "both lanes render provisionally");
    assert_eq!((head.count, settings.label.as_str()), (1, "cfg"), "the committed document and config are untouched");
    let ToolStep::Committed(_, leaves) = release(&mut runtime, "w1", "g1", vec![member(SetCount { value: 6 }), config(SetLabel { value: "c6".into() })]) else { panic!("the release commits") };
    assert_eq!(lanes(leaves), (vec![SetCount { value: 6 }.into()], vec![SetLabel { value: "c6".into() }.into()]), "ONE transaction carries the release's document and config lanes");
    assert!(matches!(release(&mut runtime, "w1", "g1", vec![config(SetLabel { value: "late".into() })]), ToolStep::Idle), "a late release is silent on every lane");
    assert_eq!(runtime.follow_config(&settings, 1, true).len(), 1, "the closed press's config overlay goes back for retirement");
    assert!(Arc::ptr_eq(runtime.config_overlay_or(&settings), &settings));
    tick(&mut runtime, "w1", "g2", vec![config(SetLabel { value: "c7".into() })], 2);
    runtime.follow(&head, 1, true);
    runtime.follow_config(&settings, 1, true);
    assert!(Arc::ptr_eq(runtime.overlay_or(&head), &head), "a config-only press leaves the document overlay empty");
    assert_eq!(runtime.config_overlay_or(&settings).label, "c7");
    assert!(matches!(runtime.presses.abort("w1", Some("g2"), ToolAbortReason::Blur), ToolStep::Aborted(..)));
    runtime.follow_config(&settings, 1, true);
    assert!(Arc::ptr_eq(runtime.config_overlay_or(&settings), &settings), "an abort leaves zero trace");
    assert!(matches!(release(&mut runtime, "w1", "g2", vec![config(SetLabel { value: "c8".into() })]), ToolStep::Idle), "the aborted press's late release stays silent");
}

/// ⚖️ LAW: the input commit-mode lint (audit F-4) answers exactly the language-agnostic fixture's findings — per-keystroke
/// text fields bound to document verbs; never blur-committed fields, number fields, file fields or view verbs.
#[test]
fn the_input_commit_lint_answers_the_fixture() {
    let law: Value = serde_json::from_str(include_str!("../🧫️fixtures/🧫️input-commit-lint/🔣️.json")).expect("lint fixture");
    let verbs: Vec<&str> = law["documentVerbs"].as_array().expect("verbs").iter().filter_map(Value::as_str).collect();
    for case in law["cases"].as_array().expect("cases") {
        let findings = artifact_app_laws::document_input_commit_findings(&case["tree"], &|verb| verbs.contains(&verb));
        assert_eq!(serde_json::json!(findings), case["findings"], "{}", case["name"]);
    }
}
