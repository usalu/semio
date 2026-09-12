use super::*;
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::default_empty_fixture;
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::modes::edit::puzzle2d_engagement;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
use crate::editor::puzzle2d::unit_tests::context::*;

#[test]
fn brush_params_are_tagged_utility_options_not_engagement_controls() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    let host = puzzle_board_host();
    let group_tag = |measures: &[WindowMeasure], id: &str| {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Group { id: gid, active_utility_id, .. } if gid == id => Some(active_utility_id.clone()),
            _ => None,
        })
    };
    // 🖌️ Brush candidate picker becomes a fill-utility-sibling tagged group, present only once the host
    // has candidates to place (empty ⇒ absent, matching the old gated-control behaviour).
    let empty_brush = scene(default_empty_fixture(), Puzzle2dPlayRuntime::default(), overview::utilities::brush::UTILITY_ID);
    assert_eq!(group_tag(&overview::window_measures(&empty_brush, labels), "puzzle2d-utility-options-brush"), Some(Some(overview::utilities::brush::UTILITY_ID.into())));
    let brush_runtime = Puzzle2dPlayRuntime { brush_candidates: vec![json!({ "nodeKind": "node" }).into()], ..Puzzle2dPlayRuntime::default() };
    let brush_scene = scene(default_empty_fixture(), brush_runtime, overview::utilities::brush::UTILITY_ID);
    let brush_measures = overview::window_measures(&brush_scene, labels);
    assert_eq!(group_tag(&brush_measures, "puzzle2d-utility-options-brush"), Some(Some(overview::utilities::brush::UTILITY_ID.into())));
    assert!(puzzle2d_engagement(&brush_scene, &host, overview::WINDOW_KIND_ID, labels).control.is_none(), "brush engagement HUD must no longer carry the relocated control");
}
