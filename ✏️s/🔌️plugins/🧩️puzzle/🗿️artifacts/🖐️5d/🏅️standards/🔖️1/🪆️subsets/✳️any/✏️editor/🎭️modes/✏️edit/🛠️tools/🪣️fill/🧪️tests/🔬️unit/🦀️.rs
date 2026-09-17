use super::*;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::puzzle5d_labels;
use crate::editor::puzzle5d::{concrete_forest_example_document, default_document, PUZZLE5D_DEFAULT_FILL_COUNT};
use semio_framework_tool_run::ToolRunTraceKind;

fn labels() -> &'static Puzzle5dLabels {
    puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("the host's own default axes are authored")
}

fn scene(document: crate::editor::puzzle5d::Puzzle5dDocument, fill_count: u32) -> Puzzle5dScene {
    Puzzle5dScene { document, runtime: Puzzle5dRuntime { fill_count, ..Puzzle5dRuntime::default() }, active_utility: TOOL_ID.into(), interaction: Default::default() }
}

/// ⚖️ LAW: fill is a MUTATING run revalidated at finalize and resumed on a count change — never restarted, or a
/// raise would throw away the sequence the operator is watching — and it declares both the run and the
/// revalidation job the ledger builds at those two moments.
#[test]
fn fill_run_declaration_is_mutating_revalidated_and_resumable() {
    let definition = run_definition();
    assert!(definition.mutating, "fill authors provisional placements");
    assert_eq!(definition.rebase, ToolRunRebasePolicy::Revalidate);
    assert_eq!(definition.reconfigure, ToolRunReconfigurePolicy::Resume);
    assert_eq!(definition.run_job, JobKindId::new(RUN_JOB_KIND));
    assert_eq!(definition.revalidate_job, Some(JobKindId::new(REVALIDATE_JOB_KIND)));
    assert!(definition.windows.is_empty(), "fill is a whole-document generator, bound to no window");
}

/// 🌉️ LAW: the declared trace kind is what `Puzzle5dPlannerBoard::translate` actually emits as its PRIMARY
/// record — an `instance3d` subject for the world pane, each of which the bridge pairs with a `placement2d` twin
/// for the board pane under the twin-bit key.
#[test]
fn fill_run_declares_the_trace_kind_the_planner_bridge_emits() {
    assert_eq!(run_definition().trace, ToolRunTraceKind::Instance3d);
}

/// 🎚️ LAW: the settings the run watches are exactly the planner's inputs — the count plus the contact tolerance
/// and the two kind-weight maps its candidate order digests. A camera or window option is not among them, so
/// moving the camera never reconfigures a live run.
#[test]
fn fill_run_settings_read_only_the_planner_inputs() {
    let definition = run_definition();
    assert_eq!(definition.settings.config, vec!["/fillCount".to_string(), "/contactTolerance".to_string(), "/objectKindWeights".to_string(), "/vortexKindWeights".to_string()]);
    assert!(definition.settings.config.iter().all(|pointer| !pointer.contains("amera")), "a camera move must never reconfigure a run");
}

/// 🗣️ LAW: every stage, counter and reason the panel can show is declared, so no tick can report a code the
/// panel cannot name.
#[test]
fn fill_run_declares_its_whole_step_vocabulary() {
    let definition = run_definition();
    assert_eq!(definition.stages.len(), puzzle5d_fill_run_stages().len());
    assert_eq!(definition.counters.len(), puzzle5d_fill_run_counters().len());
    assert_eq!(definition.reasons.len(), puzzle5d_fill_run_reasons().len());
    assert!(!definition.stages.is_empty() && !definition.counters.is_empty() && !definition.reasons.is_empty());
}

/// ♾️ LAW: the count entry carries no ceiling — the planner plans toward whatever the operator types and a
/// document that cannot hold that many says so as a visible stall, never as a silent clamp.
#[test]
fn fill_count_measure_is_unbounded_and_reads_the_shared_count() {
    let envelope = scene(default_document(), PUZZLE5D_DEFAULT_FILL_COUNT);
    let WindowMeasure::Number { id, value, min, max, step, on_change, loading, .. } = count_measure(&envelope, labels(), None) else { panic!("the fill count is a number entry") };
    assert_eq!(id, "puzzle5d-fill-count");
    assert_eq!(value, f64::from(PUZZLE5D_DEFAULT_FILL_COUNT));
    assert_eq!((min, max, step), (Some(0.0), None, Some(1.0)));
    assert_eq!(on_change.action, "setFillCount");
    assert_eq!(loading, None, "without a live run the entry is idle");
}

/// ⚖️ LAW: the tool's measures are the count entry and the distribution trees — and the trees exist for a
/// document that authors no `kindCatalogs`, derived from the kinds it already carries, so fill is configurable
/// on a catalogue-less document (the concrete forest) exactly as on a catalogued one.
#[test]
fn fill_measures_carry_the_count_and_kind_rows_even_without_authored_catalogs() {
    let document = concrete_forest_example_document();
    assert!(document.kind_catalogs.is_none(), "the concrete forest authors no kind catalogs");
    let envelope = scene(document, 8);
    let measures = measures(&envelope, labels(), None);
    assert_eq!(measures.len(), 2, "the count entry and the distribution group");
    assert!(matches!(&measures[0], WindowMeasure::Number { id, .. } if id == "puzzle5d-fill-count"));
    let WindowMeasure::Group { children, active_utility_id, .. } = &measures[1] else { panic!("the distribution tree is a group") };
    assert_eq!(*active_utility_id, None, "the fill tool's measures are tool options, never a utility rail group");
    let rows: usize = children
        .iter()
        .map(|child| match child {
            WindowMeasure::Group { children, .. } => children.len(),
            _ => 0,
        })
        .sum();
    assert!(rows > 0, "a catalogue-less document still offers one weight row per kind it carries");
}

/// 🛑️ LAW: the abort descriptor exists only while THIS tool's run is live, and carries that run's exact
/// identity — a foreign or terminal run offers nothing to abort.
#[test]
fn abort_action_is_offered_only_for_a_live_fill_run() {
    assert!(abort_action(None).is_none(), "without a run there is nothing to abort");
    assert!(live_fill_run(None).is_none());
}
