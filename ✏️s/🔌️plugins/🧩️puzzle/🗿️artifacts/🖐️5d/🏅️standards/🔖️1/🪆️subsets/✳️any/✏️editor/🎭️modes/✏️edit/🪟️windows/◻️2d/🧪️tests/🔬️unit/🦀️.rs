
use super::*;
use crate::editor::puzzle5d::unit_tests::context::*;

#[test]
fn renders_the_board_scene() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("board-2d"));
}

//#region 🕹️Interaction
fn board_document() -> Puzzle5dDocument {
    let projection = serde_json::json!({
        "schema": "puzzle.5d",
        "parts": [{ "id": "teil-ä", "partKind": "Part", "2d": { "x": 1.0, "y": 2.0 }, "3d": { "origin": [0.0, 0.0, 0.0] }, "grips": [{ "id": "g1", "gripKind": "griff-ü", "2d": {}, "3d": {} }] }],
        "fasteners": []
    });
    <Puzzle5dDocument as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(&projection.to_string()).expect("projection"))).expect("document")
}

fn board_scene_with(interaction: crate::editor::puzzle5d::Puzzle5dInteractionSnapshot, runtime: Puzzle5dRuntime) -> semio_framework_plugin::Board2dScene {
    puzzle5d_board_scene(&Puzzle5dScene { document: board_document(), runtime, active_utility: "select".into(), interaction })
}

/// 🕹️ Law: the board pane paints the SAME framework `vortex` selection and hover the world pane
/// projects — one shared interaction domain, so selecting a part in either pane highlights it in the
/// other. The world half of this law lives in the sibling `🧊️3d` window's tests.
#[test]
fn board_paints_the_live_selection_and_hover() {
    let interaction = crate::editor::puzzle5d::Puzzle5dInteractionSnapshot {
        granularity: crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_PART.into(),
        selected: vec!["teil-ä".into()],
        hovered: vec!["teil-ä".into()],
    };
    let scene = board_scene_with(interaction, Puzzle5dRuntime::default());
    assert_eq!(scene.selection_json, "[\"teil-ä\"]");
    assert_eq!(scene.hovered_id.as_deref(), Some("teil-ä"));
    let idle = board_scene_with(crate::editor::puzzle5d::Puzzle5dInteractionSnapshot::default(), Puzzle5dRuntime::default());
    assert_eq!(idle.selection_json, "[]");
    assert!(idle.hovered_id.is_none());
}

/// 🎯️ Law: this pane's own grid and selectable-kind options reach the board engine's scene fields.
#[test]
fn board_scene_carries_the_pane_grid_and_pick_filter() {
    let runtime = Puzzle5dRuntime {
        grid_visible: false,
        grid_snap_enabled: false,
        grid_factor: 2.5,
        selectable_kinds: crate::editor::puzzle5d::config::Puzzle5dSelectableKinds { parts: true, grips: false, fasteners: false },
        ..Puzzle5dRuntime::default()
    };
    let scene = board_scene_with(crate::editor::puzzle5d::Puzzle5dInteractionSnapshot::default(), runtime);
    assert!(!scene.grid_visible);
    assert!(!scene.grid_snap_enabled);
    assert_eq!(scene.grid_factor, 2.5);
    assert!(scene.selectable_nodes && !scene.selectable_handles && !scene.selectable_edges);
}
//#endregion 🕹️Interaction

//#region ☑️Options
/// 🎚️ Law: every option group this pane owns is present and reflects the runtime it was built from.
#[test]
fn window_measures_expose_every_board_option_group() {
    let labels = &crate::editor::puzzle5d::terminology::Puzzle5dLabels::NATIVE_EN;
    let runtime = Puzzle5dRuntime { grid_visible: false, grid_factor: 4.0, ..Puzzle5dRuntime::default() };
    let envelope = Puzzle5dScene { document: board_document(), runtime, active_utility: "select".into(), interaction: crate::editor::puzzle5d::Puzzle5dInteractionSnapshot::default() };
    let measures = window_measures(&envelope, labels);
    let ids = measure_ids(&measures);
    for expected in [
        "puzzle5d-play-board-grid",
        "puzzle5d-play-board-grid-visible",
        "puzzle5d-play-board-grid-snap",
        "puzzle5d-play-board-grid-factor",
        "puzzle5d-play-board-select",
        "puzzle5d-play-board-select-parts",
        "puzzle5d-play-board-select-grips",
        "puzzle5d-play-board-select-fasteners",
    ] {
        assert!(ids.iter().any(|id| id == expected), "missing measure {expected} in {ids:?}");
    }
    assert_eq!(toggle_pressed(&measures, "puzzle5d-play-board-grid-visible"), Some(false));
    assert_eq!(slider_value(&measures, "puzzle5d-play-board-grid-factor"), Some(4.0));
}
//#endregion ☑️Options

//#region 🎣️Suggestions
/// 🎣️ Law: the board pane publishes the SAME open grip suggestion menu the world pane lists, in the board host's
/// shape — the grip as `handleId`, the submenu presentation, and pending until the search has published.
#[test]
fn the_board_publishes_the_open_grip_suggestion_menu() {
    let labels = &crate::editor::puzzle5d::terminology::Puzzle5dLabels::NATIVE_EN;
    let closed = Puzzle5dScene { document: board_document(), runtime: Puzzle5dRuntime::default(), active_utility: "select".into(), interaction: Default::default() };
    assert_eq!(board_suggestion_menu_json(&closed, labels, None), None, "no menu, no record");
    let menu = crate::editor::puzzle5d::window::Puzzle5dSuggestionMenu { x: 3.0, y: 4.0, window_id: WINDOW_KIND_ID.into(), vortex_full_id: "teil-ä:g1".into(), submenu: true };
    let open = Puzzle5dScene { runtime: Puzzle5dRuntime { suggestion_menu: Some(menu), brush_candidate_index: 2, ..Puzzle5dRuntime::default() }, ..closed };
    let record: Value = serde_json::from_str(&board_suggestion_menu_json(&open, labels, None).expect("an open menu publishes")).expect("board suggestion json");
    assert_eq!(
        record,
        serde_json::json!({ "open": true, "x": 3.0, "y": 4.0, "windowId": WINDOW_KIND_ID, "handleId": "teil-ä:g1", "hoveredIndex": 2, "submenu": true, "pending": true, "candidates": [] })
    );
}

/// 🖱️ Law: a right-click on a board handle (the board engine's `handle` pick domain) is the GRIP's menu, even
/// with its part selected — the suggest row the board host turns into its live submenu.
#[test]
fn a_board_handle_hit_is_the_grips_menu() {
    let surface = semio_framework_plugin::ContextMenuSurfaceTarget {
        surface_id: SURFACE_ID.into(),
        kind: "board2d".into(),
        hits: vec![
            semio_framework_plugin::ContextMenuHit { domain: "handle".into(), id: "teil-ä:g1".into(), label: None },
            semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "teil-ä".into(), label: None },
        ],
        selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec!["teil-ä:g1".into()] }],
        text: None,
    };
    let selection = crate::editor::puzzle5d::Puzzle5dContextSelection::from_surface(Some(&surface));
    assert_eq!(selection.grip_ids, vec!["teil-ä:g1".to_string()]);
    assert!(selection.part_ids.is_empty(), "the handle under the pointer is the whole subject");
}
//#endregion 🎣️Suggestions
