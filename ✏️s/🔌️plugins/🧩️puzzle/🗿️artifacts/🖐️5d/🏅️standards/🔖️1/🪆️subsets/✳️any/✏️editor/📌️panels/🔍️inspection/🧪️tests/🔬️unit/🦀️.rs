use super::*;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::puzzle5d_labels;
use crate::editor::puzzle5d::{empty_document, Puzzle5dGrip, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d};
use crate::editor::puzzle5d::unit_tests::context::*;
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

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

fn keys(node: &BuiltNode, out: &mut Vec<String>) {
    out.push(node.key.as_str().to_string());
    for child in node.children.iter() {
        keys(child, out);
    }
}

fn row_keys(node: &BuiltNode) -> Vec<String> {
    let mut out = Vec::new();
    keys(node, &mut out);
    out
}

fn grip(id: &str) -> Puzzle5dGrip {
    Puzzle5dGrip { id: id.into(), grip_kind: "socket".into(), grip_2d: Default::default(), grip_3d: Default::default() }
}

/// 🏗️ A document of `ids` parts, each with one `g0` grip, plus one fastener between the first two.
fn scene(ids: &[String]) -> Puzzle5dScene {
    let mut document = empty_document();
    document.parts = ids
        .iter()
        .map(|id| Puzzle5dPart {
            id: id.clone(),
            part_kind: "capsule".into(),
            anchor: Default::default(),
            part_2d: Puzzle5dPart2d { x: 4.0, y: 8.0, ..Default::default() },
            part_3d: Puzzle5dPart3d::default(),
            grips: vec![grip("g0")],
        })
        .collect();
    if ids.len() >= 2 {
        document.fasteners = vec![crate::editor::puzzle5d::Puzzle5dFastener {
            id: "fastener-0".into(),
            source: puzzle5d_grip_full_id(&ids[0], "g0"),
            target: puzzle5d_grip_full_id(&ids[1], "g0"),
            fastener_kind: Some("clip".into()),
            gap: 0.5,
            shift: 0.0,
            rise: 0.0,
            rotation: 0.0,
            turn: 0.0,
            tilt: 0.0,
            x: 0.0,
            y: 0.0,
        }];
    }
    Puzzle5dScene { document, runtime: Puzzle5dRuntime::default(), active_utility: String::new(), interaction: Puzzle5dInteractionSnapshot::default() }
}

fn inspect(scene: &Puzzle5dScene, view: &ViewModel) -> BuiltNode {
    drain();
    render(scene, labels(), &TreeWindows::for_body(view, BODY_KEY)).expect("inspector body")
}

fn unhosted(scene: &Puzzle5dScene) -> BuiltNode {
    drain();
    render(scene, labels(), &TreeWindows::unhosted()).expect("inspector body")
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: BODY_KEY.to_string(), node_key: node_key.to_string(), open, offset, rows }
}

fn hosted(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

#[test]
fn empty_selection_renders_the_document_summary() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-inspector.empty"));
}

/// 🔍️ A selected PART renders its own editable field group and both flag rows, never the summary.
#[test]
fn a_selected_part_renders_its_field_group_with_both_flag_rows() {
    let mut scene = scene(&["part-0".into(), "part-1".into()]);
    scene.interaction = Puzzle5dInteractionSnapshot { granularity: PUZZLE5D_GRANULARITY_PART.into(), selected: vec!["part-0".into()], hovered: Vec::new() };
    let node = unhosted(&scene);
    let rows = row_keys(&node);
    for key in ["puzzle5d-play-inspector.part.id", "puzzle5d-play-inspector.part.kind", "puzzle5d-play-inspector.part.x", "puzzle5d-play-inspector.part.hidden", "puzzle5d-play-inspector.part.locked"] {
        assert!(rows.iter().any(|row| row == key), "a selected part must render {key}: {rows:?}");
    }
    assert!(!rows.iter().any(|row| row.ends_with(".empty")), "a resolved selection never keeps the summary: {rows:?}");
    drop(node);
    drain();
}

/// 🔍️ A selected GRIP wins over its owning part and renders the grip field group.
#[test]
fn a_selected_grip_renders_the_grip_field_group() {
    let mut scene = scene(&["part-0".into(), "part-1".into()]);
    scene.interaction = Puzzle5dInteractionSnapshot { granularity: PUZZLE5D_GRANULARITY_GRIP.into(), selected: vec![puzzle5d_grip_full_id("part-1", "g0")], hovered: Vec::new() };
    let node = unhosted(&scene);
    let rows = row_keys(&node);
    assert!(rows.iter().any(|row| row == "puzzle5d-play-inspector.grip.full-id"), "{rows:?}");
    assert!(rows.iter().any(|row| row == "puzzle5d-play-inspector.grip.angle"), "{rows:?}");
    assert!(!rows.iter().any(|row| row.starts_with("puzzle5d-play-inspector.part.")), "a grip selection must not fall through to the part group: {rows:?}");
    drop(node);
    drain();
}

/// 🔍️ A selected FASTENER renders every joint scalar as its own patch row.
#[test]
fn a_selected_fastener_renders_every_joint_scalar() {
    let mut scene = scene(&["part-0".into(), "part-1".into()]);
    scene.interaction = Puzzle5dInteractionSnapshot { granularity: PUZZLE5D_GRANULARITY_FASTENER.into(), selected: vec!["fastener-0".into()], hovered: Vec::new() };
    let node = unhosted(&scene);
    let rows = row_keys(&node);
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt"] {
        assert!(rows.iter().any(|row| row == &format!("puzzle5d-play-inspector.fastener.{key}")), "a selected fastener must render {key}: {rows:?}");
    }
    drop(node);
    drain();
}

/// 🕹️ A granularity-less leftover selection still resolves against the document instead of showing
/// the summary — the browser-shaped snapshot a host hands back after a reload.
#[test]
fn a_granularity_less_selection_falls_through_to_the_part_group() {
    let mut scene = scene(&["part-0".into(), "part-1".into()]);
    scene.interaction = Puzzle5dInteractionSnapshot { granularity: String::new(), selected: vec!["part-1".into()], hovered: Vec::new() };
    let node = unhosted(&scene);
    let rows = row_keys(&node);
    assert!(rows.iter().any(|row| row == "puzzle5d-play-inspector.part.id"), "{rows:?}");
    assert!(!rows.iter().any(|row| row.ends_with(".empty")), "{rows:?}");
    drop(node);
    drain();
}

/// 🪟️ A wide selection's id list is a WINDOW: the section stamps every selected id and materialises
/// only the host's slice, keyed by the raw id.
#[test]
fn a_wide_selection_stamps_every_id_and_materialises_only_its_window() {
    let ids: Vec<String> = (0..120).map(|index| format!("part-{index}")).collect();
    let mut scene = scene(&ids);
    scene.interaction = Puzzle5dInteractionSnapshot { granularity: PUZZLE5D_GRANULARITY_PART.into(), selected: ids.clone(), hovered: Vec::new() };
    let node = inspect(&scene, &hosted(vec![request(IDS_SECTION, Some(true), 0, 8)]));
    let section = node.children.iter().find(|child| child.key.as_str() == IDS_SECTION).expect("the ids section");
    let window = match &section.component {
        semio_framework_ui_contract::Component::TreeSection(props) => props.window.expect("the ids section stamps its window"),
        _ => panic!("the ids section is a tree section"),
    };
    assert_eq!(window.total as usize, ids.len());
    assert_eq!(section.children.len(), 8, "exactly the eight rows the host asked for");
    let keys: Vec<String> = section.children.iter().map(|child| child.key.as_str().to_string()).collect();
    assert_eq!(keys, (0..8).map(|index| format!("{IDS_SECTION}.part-{index}")).collect::<Vec<_>>());
    drop(node);
    drain();
}
