use super::*;
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::default_empty_fixture;
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::modes::edit::puzzle2d_engagement;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
use crate::editor::puzzle2d::unit_tests::context::*;
use semio_framework_plugin::{Locale, Terminology};
use serde_json::Value;

fn fill_children(runtime: Puzzle2dPlayRuntime, labels: &Puzzle2dLabels) -> Vec<WindowMeasure> {
    let measure = measures(&scene(default_empty_fixture(), runtime, overview::utilities::select::UTILITY_ID), labels);
    let WindowMeasure::Group { children, .. } = measure else { panic!("fill group") };
    children
}

/// 🛠️ Fill's count entry is a tool measure keyed by the fill tool id, not a window utility-options group.
#[test]
fn fill_count_entry_is_a_tool_measure() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    let host = puzzle_board_host();
    let fill_scene = scene(default_empty_fixture(), Puzzle2dPlayRuntime { fill_count: 3, ..Puzzle2dPlayRuntime::default() }, overview::utilities::select::UTILITY_ID);
    assert!(matches!(&measures(&fill_scene, labels), WindowMeasure::Group { id, active_utility_id: None, .. } if id == "puzzle2d-tool-options-fill"));
    assert!(!overview::window_measures(&fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle2d-tool-options-fill")), "fill must not surface in window_measures");
    assert!(puzzle2d_engagement(&fill_scene, &host, overview::WINDOW_KIND_ID, labels).control.is_none(), "fill engagement HUD must not carry the relocated control");
}

/// ♾️ The count is the tool's only measure: an unbounded `Number` carried verbatim, defaulting to 100, with no
/// plugin-local progress, cancel or retry control beside it.
#[test]
fn fill_count_is_the_only_unbounded_measure_and_defaults_to_one_hundred() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    let children = fill_children(Puzzle2dPlayRuntime { fill_count: 5_000, ..Puzzle2dPlayRuntime::default() }, labels);
    let [WindowMeasure::Number { id, value, min, max, step, ready, loading, waiting, .. }] = children.as_slice() else { panic!("the count is the only fill measure: {children:?}") };
    assert_eq!((id.as_str(), *value, *min, *max, *step), ("puzzle2d-fill-count", 5_000.0, Some(0.0), None, Some(1.0)));
    assert_eq!((*ready, *loading, *waiting), (None, None, None), "run progress lives in the ToolRun panel, not in the count");
    assert_eq!(PUZZLE2D_DEFAULT_FILL_COUNT, 100);
    assert_eq!(Puzzle2dPlayRuntime::default().fill_count, 100);
    assert!(matches!(fill_children(Puzzle2dPlayRuntime::default(), labels).first(), Some(WindowMeasure::Number { value, .. }) if *value == 100.0));
}

/// ⏯️ The fill tool declares its run and the declaration equals the language-neutral `$defs.Puzzle2dFillRun`
/// table: policies, trace kind, job kinds, stage/counter/reason ids and codes, verdicts and the EN/DE texts.
#[test]
fn fill_tool_declares_the_schema_tool_run_definition() {
    let schema: Value = serde_json::from_str(include_str!("../../../../../../../🧬️schema/🔣️.json")).expect("puzzle2d schema parses");
    let table = &schema["$defs"]["Puzzle2dFillRun"]["x-semio-toolRun"];
    let definition = definition(puzzle2d_localized(|labels| labels.fill));
    let run = definition.run.as_ref().expect("the fill tool declares run");
    run.validate().expect("the fill run definition validates");
    let json = serde_json::to_value(run).expect("run definition serializes");
    for key in ["mutating", "rebase", "reconfigure", "trace", "runJob", "revalidateJob"] {
        assert_eq!(json[key], table[key], "{key}");
    }
    let text = |label: &LocalizedLabel, locale: Locale| label.resolve(Terminology::Native, locale).to_string();
    assert_eq!((text(&run.unit, Locale::En), text(&run.unit, Locale::De)), (table["unit"]["en"].as_str().unwrap_or_default().to_string(), table["unit"]["de"].as_str().unwrap_or_default().to_string()));
    let rows = |key: &str| table[key].as_array().unwrap_or_else(|| panic!("{key} table")).clone();
    assert_eq!(run.stages.len(), rows("stages").len());
    for (stage, row) in run.stages.iter().zip(rows("stages")) {
        assert_eq!((stage.id.as_str(), text(&stage.label, Locale::En), text(&stage.label, Locale::De)), (row["id"].as_str().unwrap_or_default(), row["en"].as_str().unwrap_or_default().to_string(), row["de"].as_str().unwrap_or_default().to_string()));
    }
    assert_eq!(run.counters.len(), rows("counters").len());
    for (counter, row) in run.counters.iter().zip(rows("counters")) {
        assert_eq!((counter.id.as_str(), text(&counter.label, Locale::En), text(&counter.label, Locale::De)), (row["id"].as_str().unwrap_or_default(), row["en"].as_str().unwrap_or_default().to_string(), row["de"].as_str().unwrap_or_default().to_string()));
    }
    assert_eq!(run.reasons.len(), rows("reasons").len());
    for (reason, row) in run.reasons.iter().zip(rows("reasons")) {
        assert_eq!(u64::from(reason.code), row["code"].as_u64().unwrap_or(u64::MAX), "{}", reason.id);
        assert_eq!(reason.id, row["id"].as_str().unwrap_or_default());
        assert_eq!(serde_json::to_value(reason.verdict).expect("verdict serializes"), row["verdict"], "{}", reason.id);
        assert_eq!((text(&reason.template, Locale::En), text(&reason.template, Locale::De)), (row["en"].as_str().unwrap_or_default().to_string(), row["de"].as_str().unwrap_or_default().to_string()), "{}", reason.id);
    }
}

/// 🧰️ Declaring the run injects the framework tool run actions into the registered manifest; the plugin owns none.
#[test]
fn fill_run_declaration_injects_the_framework_tool_run_actions() {
    let injected = semio_framework::tool_run_action_definitions(&crate::editor::puzzle2d::create_puzzle2d_app());
    let ids: Vec<&str> = injected.iter().map(|action| action.id.as_str()).collect();
    for action in semio_framework_tool_run::TOOL_RUN_ACTION_IDS {
        assert!(ids.contains(&action), "{action} is injected once the fill tool declares run");
    }
}
