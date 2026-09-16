use super::*;
use crate::editor::puzzle5d::unit_tests::context::*;

#[test]
fn catalogue_tree_lists_all_four_kind_sections() {
    let mut app = app();
    let rendered = render_body(&mut app, BODY_KEY);
    for section in [PARTS_SECTION, GRIPS_SECTION, FASTENERS_SECTION, ROPES_SECTION] {
        assert!(rendered.contains(section), "catalogue must carry {section}");
    }
}

//#region 🪟️WindowLaws
const RETIREMENT_DRAIN_STEPS: usize = 4096;
/// 🏗️ Past `TREE_WINDOW_DEFAULT_ROWS` and past `UI_BUILT_CHILDREN_MAX`, the shape a plugin-wide part
/// catalogue really has.
const SCALE_KINDS: usize = 200;

fn drain_retired_ui_owners() {
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if semio_framework_ui_contract::close_built_node_page_one() {
            break;
        }
    }
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if semio_framework_ui_contract::close_ui_value_page_one() {
            break;
        }
    }
}

fn labels() -> &'static Puzzle5dLabels {
    crate::editor::puzzle5d::terminology::puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("an admitted host label axis")
}

/// 🏗️ A document whose `kindCatalogs.parts` carries `kinds` rows.
fn scaled_scene(kinds: usize) -> Puzzle5dScene {
    let rows: Vec<Value> = (0..kinds).map(|index| json!({ "id": format!("kind-{index}"), "name": format!("Kind {index}") })).collect();
    let mut document = crate::editor::puzzle5d::empty_document();
    document.kind_catalogs = Some(json!({ "parts": rows, "grips": [], "fasteners": [], "ropes": [] }));
    Puzzle5dScene { document, runtime: crate::editor::puzzle5d::config::Puzzle5dRuntime::default(), active_utility: String::new() }
}

fn window_of(node: &BuiltNode) -> Option<semio_framework_ui_contract::TreeWindow> {
    match &node.component {
        semio_framework_ui_contract::Component::TreeSection(props) => props.window,
        semio_framework_ui_contract::Component::TreeItem(props) => props.window,
        _ => None,
    }
}

fn child_of<'a>(node: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    node.children.iter().find(|child| child.key.as_str() == key).unwrap_or_else(|| panic!("container {key} must exist"))
}

/// 🪟️ (a)+(b) An oversized catalogue stamps its section's full extent, materialises no more than its
/// slice, and carries no `+N` row; the empty sections show their placeholder alone.
#[test]
fn the_catalogue_stamps_every_section_and_never_pages() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_KINDS);
    let windows = semio_framework_plugin::TreeWindows::unhosted();
    let tree = render(&scene, labels(), &windows).expect("an oversized catalogue must be admitted");
    let parts = child_of(&tree, PARTS_SECTION);
    assert_eq!(window_of(parts).expect("the parts section must stamp its window").total as usize, SCALE_KINDS);
    assert!(!parts.children.is_empty() && parts.children.len() <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX);
    for section in [GRIPS_SECTION, FASTENERS_SECTION, ROPES_SECTION] {
        assert_eq!(child_of(&tree, section).children.len(), 1, "an empty section shows exactly its placeholder row");
    }
    let body = serde_json::to_string(&tree).expect("serialize the catalogue body");
    assert!(!body.contains(".more"), "a windowed catalogue carries no continuation key");
    assert!(!body.contains("\"+"), "a windowed catalogue carries no `+N` label");
    drop(tree);
    drain_retired_ui_owners();
}

/// 🪟️ (c) A host window materialises exactly `[offset, offset + rows)`; every part row keeps its own
/// `addPartKind` binding, because a catalogue row is not a pick target.
#[test]
fn a_catalogue_window_materialises_its_slice_with_row_bindings_intact() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_KINDS);
    let (offset, rows) = (70u32, 8u32);
    let view = semio_framework_plugin::ViewModel {
        tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: BODY_KEY.into(), node_key: PARTS_SECTION.into(), open: Some(true), offset, rows }],
        ..Default::default()
    };
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, BODY_KEY);
    let tree = render(&scene, labels(), &windows).expect("a windowed catalogue must be admitted");
    let parts = child_of(&tree, PARTS_SECTION);
    assert_eq!(window_of(parts).expect("stamped window").offset, offset);
    assert_eq!(parts.children.len(), rows as usize);
    let keys: Vec<String> = parts.children.iter().map(|child| child.key.as_str().to_string()).collect();
    assert_eq!(keys, (offset..offset + rows).map(|index| format!("{PARTS_SECTION}.{index}.kind-{index}")).collect::<Vec<_>>());
    for row in parts.children.iter() {
        assert_eq!(row.bindings.len(), 1, "catalogue row {} keeps its own addPartKind binding", row.key.as_str());
    }
    drop(tree);
    drain_retired_ui_owners();
}
//#endregion 🪟️WindowLaws
