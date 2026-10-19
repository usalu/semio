use semio_framework_plugin::WindowMeasure;
use semio_framework_pack_json::json;
#[allow(unused_imports)]
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
#[allow(unused_imports)]
use crate::editor::puzzle2d::snapshot::{puzzle2d_action, puzzle2d_kind_ids, Puzzle2dScene};
#[allow(unused_imports)]
use semio_framework_plugin::MeasureSelectItem;
#[allow(unused_imports)]
use std::collections::BTreeMap;
use crate::editor::puzzle2d::modes::edit::options::brush::*;
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::snapshot::empty_board_snapshot;
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::modes::edit::canvas::puzzle2d_engagement;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
use crate::editor::puzzle2d::testkit::*;

#[test]
fn brush_params_are_tagged_utility_options_not_engagement_controls() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let host = puzzle_board_host();
    let group_tag = |measures: &[WindowMeasure], id: &str| {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Group { id: gid, active_utility_id, .. } if gid == id => Some(active_utility_id.clone()),
            _ => None,
        })
    };
    // 🖌️ Brush candidate picker becomes a fill-utility-sibling tagged group, present only once the host
    // has candidates to place (empty ⇒ absent, matching the old gated-control behaviour).
    let empty_brush = scene(empty_board_snapshot(), Puzzle2dPlayRuntime::default(), overview::utilities::brush::UTILITY_ID);
    assert_eq!(group_tag(&overview::window_measures(&empty_brush, labels), "puzzle2d-utility-options-brush"), Some(Some(overview::utilities::brush::UTILITY_ID.into())));
    let brush_runtime = Puzzle2dPlayRuntime { brush_candidates: vec![semio_framework_value::ToValue::to_value(&json!({ "nodeKind": "node" }))], ..Puzzle2dPlayRuntime::default() };
    let brush_scene = scene(empty_board_snapshot(), brush_runtime, overview::utilities::brush::UTILITY_ID);
    let brush_measures = overview::window_measures(&brush_scene, labels);
    assert_eq!(group_tag(&brush_measures, "puzzle2d-utility-options-brush"), Some(Some(overview::utilities::brush::UTILITY_ID.into())));
    assert!(puzzle2d_engagement(&brush_scene, &host, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, labels).control.is_none(), "brush engagement HUD must no longer carry the relocated control");
}
