use super::*;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::puzzle5d_labels;
use crate::editor::puzzle5d::unit_tests::context::*;
use crate::editor::puzzle5d::{empty_document, Puzzle5dInteractionSnapshot};
use semio_framework_plugin::{ViewModel, ViewWindowInstance};

fn labels() -> &'static Puzzle5dLabels {
    puzzle5d_labels(&ViewModel::default()).expect("an admitted host label axis")
}

fn drain() {
    for _ in 0..4096 {
        if semio_framework_ui_contract::close_built_node_page_one() {
            break;
        }
    }
    for _ in 0..4096 {
        if semio_framework_ui_contract::close_ui_value_page_one() {
            break;
        }
    }
}

fn scene() -> Puzzle5dScene {
    Puzzle5dScene { document: empty_document(), runtime: Puzzle5dRuntime::default(), active_utility: String::new(), interaction: Puzzle5dInteractionSnapshot::default() }
}

fn keys(node: &BuiltNode, out: &mut Vec<String>) {
    out.push(node.key.as_str().to_string());
    for child in node.children.iter() {
        keys(child, out);
    }
}

fn bindings(node: &BuiltNode, out: &mut Vec<String>) {
    for binding in node.bindings.iter() {
        out.push(binding.action.name.as_str().to_string());
    }
    for child in node.children.iter() {
        bindings(child, out);
    }
}

/// ⚙️ Every stepper the 5d settings panel owes puzzle 3d and puzzle 2d together is authored, and each
/// one dispatches the verb that field is stored under.
#[test]
fn the_settings_panel_authors_every_stepper_and_its_verb() {
    drain();
    let node = render(&scene(), labels(), Some("window-0")).expect("settings body");
    let mut rows = Vec::new();
    keys(&node, &mut rows);
    for field in ["fill-count", "suggestion-offset", "contact-tolerance", "proximity-radius", "chunk-size", "grid-factor", "grid-spacing"] {
        assert!(rows.iter().any(|row| row == &format!("puzzle5d-play-settings.{field}")), "the settings panel must author {field}: {rows:?}");
    }
    let mut verbs = Vec::new();
    bindings(&node, &mut verbs);
    for verb in ["setFillCount", "setSuggestionOffset", "setBrushPlacementContactTolerance", "setProximityRadius", "setChunkSize", "setGridFactor", "setGridSpacing"] {
        assert!(verbs.iter().any(|name| name == verb), "a stepper must dispatch {verb}: {verbs:?}");
    }
    drop(node);
    drain();
}

/// 🪟️ The panel addresses the focused pane when the render itself names none, and no pane at all when
/// the host mounts none — an empty `windowId` would be refused as an unknown window instance.
#[test]
fn the_panel_addresses_the_focused_pane_and_none_without_one() {
    assert_eq!(panel_window_id(&ViewModel::default()), None);
    let focused = ViewModel { focused_window_id: Some("pane-2".into()), ..Default::default() };
    assert_eq!(panel_window_id(&focused), Some("pane-2"));
    let roster = ViewModel { window_instances: vec![ViewWindowInstance { id: "pane-9".into(), window_kind_id: "puzzle5d-board".into() }], ..Default::default() };
    assert_eq!(panel_window_id(&roster), Some("pane-9"));
}

/// ⚙️ The registered app really serves this body — a panel whose body key no render arm answers
/// renders the "Unknown body" placeholder instead.
#[test]
fn the_registered_app_renders_the_settings_body() {
    let mut app = app();
    let rendered = render_body(&mut app, BODY_KEY);
    assert!(rendered.contains("puzzle5d-play-settings"), "the app must serve the settings body: {rendered:.400}");
    assert!(!rendered.contains("Unknown body"), "{rendered:.400}");
}
