//! 📊️ LAW: a table is ONE column grid on the wgpu target — its header, its painted cells and an editable row's cell children
//! share `layout::table_column_rect`, and a row's actions are its `RowAction` props only: painted, hit and announced at
//! `layout::tree_row_action_rect`, each firing its own versioned binding.
//!
//! The defect: the reconcile painted a childless `TableRow` as ONE button labelled `cells.join(" · ")` and a row with children as a
//! bare horizontal stack (cells dropped, no header), and never painted `TableRowProps::row_actions` — so the SDK duplicated every
//! row action as a `row-action-<i>` child button and Home's 32-row window ran out of its item budget after 27 rows (ticket
//! 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP session 14, the WG11 × LB2 TableRow agreement).
//!
//! Oracle: `🖱️ui/🧫️fixtures/📊️table-row-grid/🔣️.json`; the editable row's cell rects are also solved by taffy.

use crate::wgpu::accessibility::{accessibility_projection, row_accessibility_action};
use crate::wgpu::arena::NodeId;
use crate::wgpu::chrome::UiDriverDrag;
use crate::wgpu::draw::{DrawList, KIND_GLYPH};
use crate::wgpu::events::{AccessibilityUiEvent, EventRouter, PointerButton, UiCommand, UiEvent};
use crate::wgpu::layout::{table_actions_width, table_column_rect, tree_row_action_rect, TreeRowMetrics};
use crate::wgpu::mounted_layout::layout_tree_now;
use crate::wgpu::paint::{paint_node_step_with_driver, RetainedNodePaintCursor, RetainedNodePaintStep};
use crate::wgpu::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileStep};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{UiDocumentTree, UiTree};
use serde_json::Value;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiRevision};

const GENERATION: u64 = 1;

fn law() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📊️table-row-grid/🔣️.json")).expect("📊️ the table-row-grid fixture parses")
}

fn number(value: &Value, key: &str) -> f32 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture number {key}")) as f32
}

fn close(left: f32, right: f32) -> bool {
    (left - right).abs() <= 1e-3
}

/// 📃️ Publishes the fixture document, reconciles it to completion and lays it out in the fixture viewport — the path a live
/// window takes (document → arena → mounted layout).
fn mounted(law: &Value) -> (UiTree, NodeId) {
    let source = &law["document"];
    let nodes = source["nodes"].as_array().expect("fixture nodes");
    let header = UiDocumentLeaseHeader {
        generation: GENERATION,
        surface: SurfaceId::try_from(source["surface"].as_str().expect("fixture surface")).expect("fixture surface id"),
        revision: UiRevision(source["revision"].as_u64().expect("fixture revision")),
        root: UiNodeId(source["root"].as_u64().expect("fixture root")),
        layout_epoch: source["layoutEpoch"].as_u64().expect("fixture layout epoch"),
        node_count: nodes.len(),
    };
    let mut document = UiDocumentTree::new(header).expect("fixture header admits");
    for node in nodes {
        let record: UiNodeRecord = serde_json::from_value(node.clone()).expect("fixture record deserializes against the contract");
        document.try_upsert_record(record).expect("fixture record admits");
    }
    let surface = source["surface"].as_str().expect("fixture surface");
    let controller = source["controller"].as_str().expect("fixture controller");
    let mut tree = UiTree::new();
    tree.publish_document(document);
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(GENERATION);
    let mut complete = false;
    for _ in 0..4096 {
        match tree.step_document_reconcile(&mut cursor, surface, controller) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => {
                complete = true;
                break;
            }
            UiDocumentReconcileStep::Fault(fault) => panic!("the table document must mount cleanly, got {fault:?}"),
        }
    }
    assert!(complete, "document reconcile terminates inside its node budget");
    let root = tree.root.expect("mounted table root");
    let viewport = &law["viewport"];
    assert!(layout_tree_now(&mut tree, root, Theme::default(), number(viewport, "width"), number(viewport, "height")), "layout pass");
    (tree, root)
}

fn mounted_node(tree: &UiTree, record: &Value) -> NodeId {
    let id = record["record"].as_u64().expect("fixture record id");
    tree.document_node(UiNodeId(id)).unwrap_or_else(|| panic!("record {id} mounted"))
}

fn fired(commands: &[UiCommand]) -> Vec<String> {
    commands
        .iter()
        .filter_map(|command| match command {
            UiCommand::App { intent, .. } => Some(intent.action.name.as_str().to_string()),
            _ => None,
        })
        .collect()
}

fn glyphs_in(draw: &DrawList, x: f32, y: f32, width: f32, height: f32) -> usize {
    draw.layers
        .iter()
        .flat_map(|layer| layer.ui_instances.iter())
        .filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01)
        .filter(|instance| {
            let (center_x, center_y) = (instance.rect[0] + instance.rect[2] * 0.5, instance.rect[1] + instance.rect[3] * 0.5);
            center_x >= x && center_x <= x + width && center_y >= y && center_y <= y + height
        })
        .count()
}

#[test]
fn the_grid_metrics_are_the_ones_the_fixture_was_pinned_against() {
    let law = law();
    let pinned = &law["metrics"];
    let theme = Theme::default();
    let metrics = TreeRowMetrics::from_theme(&theme);
    assert!(close(metrics.row_height, number(pinned, "rowHeightPx")));
    assert!(close(metrics.gap, number(pinned, "gapPx")));
    assert!(close(metrics.drag_handle_extent, number(pinned, "actionIconPx")));
    assert!(close(metrics.action_gap, number(pinned, "actionGapPx")));
    assert!(close(theme.control_height_small, number(pinned, "controlHeightSmallPx")));
}

#[test]
fn rows_columns_and_editable_cells_are_laid_out_on_one_grid() {
    let law = law();
    let expected = &law["expected"];
    let (tree, _) = mounted(&law);
    let metrics = TreeRowMetrics::from_theme(&Theme::default());
    let width = number(&law["viewport"], "width");
    let actions = table_actions_width(expected["actionSlots"].as_array().expect("action slots").len(), &metrics);
    assert!(close(actions, number(expected, "actionsWidth")), "actions column {actions}");
    let columns = expected["columns"].as_array().expect("columns");
    for (index, column) in columns.iter().enumerate() {
        let rect = table_column_rect(width, metrics.row_height, index, columns.len(), actions, &metrics);
        assert!(close(rect.x, number(column, "x")) && close(rect.w, number(column, "width")), "column {index}: ({}, {})", rect.x, rect.w);
    }
    for row in expected["rows"].as_array().expect("rows") {
        let (x, y, w, h) = tree.mounted_layout(mounted_node(&tree, row)).expect("row layout");
        assert!(close(x, 0.0) && close(y, number(row, "y")) && close(w, width) && close(h, number(row, "height")), "{}: ({x}, {y}, {w}, {h})", row["key"]);
    }
    for cell in expected["editableCells"].as_array().expect("editable cells") {
        let (x, y, w, h) = tree.mounted_layout(mounted_node(&tree, cell)).expect("cell layout");
        assert!(close(x, number(cell, "x")) && close(y, number(cell, "y")) && close(w, number(cell, "width")) && close(h, number(cell, "height")), "{}: ({x}, {y}, {w}, {h})", cell["key"]);
    }
}

#[test]
fn taffy_lays_the_editable_rows_cells_where_the_grid_does() {
    use taffy::style_helpers::length;
    let law = law();
    let metrics = TreeRowMetrics::from_theme(&Theme::default());
    let width = number(&law["viewport"], "width");
    let actions = number(&law["expected"], "actionsWidth");
    let cells = law["expected"]["editableCells"].as_array().expect("editable cells");
    let mut solver: taffy::TaffyTree<()> = taffy::TaffyTree::new();
    solver.disable_rounding();
    let leaves = cells
        .iter()
        .map(|cell| solver.new_leaf(taffy::Style { flex_grow: 1.0, flex_shrink: 1.0, size: taffy::geometry::Size { width: length(0.0), height: length(number(cell, "height")) }, ..taffy::Style::default() }).expect("taffy leaf"))
        .collect::<Vec<_>>();
    let row = solver
        .new_with_children(
            taffy::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Row,
                align_items: Some(taffy::style::AlignItems::Center),
                gap: taffy::geometry::Size { width: length(metrics.gap), height: length(0.0) },
                padding: taffy::geometry::Rect { left: length(metrics.gap), right: length(actions), top: length(0.0), bottom: length(0.0) },
                size: taffy::geometry::Size { width: length(width), height: length(metrics.row_height) },
                ..taffy::Style::default()
            },
            &leaves,
        )
        .expect("taffy row");
    solver.compute_layout(row, taffy::geometry::Size { width: taffy::style::AvailableSpace::MaxContent, height: taffy::style::AvailableSpace::MaxContent }).expect("taffy solves the row");
    for (cell, leaf) in cells.iter().zip(leaves) {
        let solved = solver.layout(leaf).expect("taffy layout");
        assert!(
            close(solved.location.x, number(cell, "x")) && close(solved.location.y, number(cell, "y")) && close(solved.size.width, number(cell, "width")) && close(solved.size.height, number(cell, "height")),
            "taffy {}: ({}, {}, {}, {})",
            cell["key"],
            solved.location.x,
            solved.location.y,
            solved.size.width,
            solved.size.height
        );
    }
}

#[test]
fn every_action_slot_sits_at_the_inline_end_of_its_row() {
    let law = law();
    let metrics = TreeRowMetrics::from_theme(&Theme::default());
    let width = number(&law["viewport"], "width");
    for slot in law["expected"]["actionSlots"].as_array().expect("action slots") {
        let rect = tree_row_action_rect(width, metrics.row_height, slot["slot"].as_u64().expect("slot") as usize, 0.0, &metrics);
        assert!(close(rect.x, number(slot, "x")) && close(rect.y, number(slot, "y")) && close(rect.w, number(slot, "size")) && close(rect.h, number(slot, "size")), "slot {}: ({}, {}, {}, {})", slot["slot"], rect.x, rect.y, rect.w, rect.h);
    }
}

#[test]
fn a_click_on_an_action_icon_fires_that_action_and_anywhere_else_on_the_row_its_activation() {
    let law = law();
    let (mut tree, root) = mounted(&law);
    for click in law["expected"]["clicks"].as_array().expect("clicks") {
        let (x, y) = (number(click, "x"), number(click, "y"));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: Default::default() });
        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x, y, button: PointerButton::Primary, modifiers: Default::default() });
        assert_eq!(fired(&commands), vec![click["fires"].as_str().expect("fires").to_string()], "click ({x}, {y}): {}", click["why"]);
    }
}

#[test]
fn the_table_paints_its_header_and_plain_cells_once_in_their_columns_and_leaves_editable_cells_to_their_children() {
    let law = law();
    let (tree, root) = mounted(&law);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let layout = tree.accepted_layout(root).expect("table layout");
    let absolute = tree.absolute_rect(root).expect("table bounds");
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    let mut complete = false;
    for _ in 0..16_384 {
        match paint_node_step_with_driver(&tree, root, absolute.x - layout.x, absolute.y - layout.y, &theme, &mut atlas, None, false, UiDriverDrag::Handle, false, ui_contract::FlowInline::Ltr, &mut draw, &mut cursor) {
            RetainedNodePaintStep::Pending => {}
            RetainedNodePaintStep::Complete => {
                complete = true;
                break;
            }
            RetainedNodePaintStep::Fault => panic!("table paint fault"),
        }
    }
    assert!(complete, "the table paints inside its fixed budget");
    for text in law["expected"]["paintedText"].as_array().expect("painted text") {
        let value = text["text"].as_str().expect("text");
        let glyphs = glyphs_in(&draw, number(text, "x"), number(text, "y"), number(text, "width"), number(text, "height"));
        assert_eq!(glyphs, value.chars().filter(|scalar| !scalar.is_whitespace()).count(), "{value} paints once, in its column");
    }
    let band = &law["expected"]["unpaintedBand"];
    assert_eq!(glyphs_in(&draw, 0.0, number(band, "y"), number(&law["viewport"], "width"), number(band, "height")), 0, "{}", band["why"]);
}

#[test]
fn every_row_action_is_an_announced_button_that_fires_its_own_binding() {
    let law = law();
    let (mut tree, _) = mounted(&law);
    let projection = accessibility_projection(&tree);
    for expected in law["expected"]["accessibility"].as_array().expect("accessibility") {
        let key = expected["key"].as_str().expect("key");
        let node = projection.iter().find(|node| node.key == key).unwrap_or_else(|| panic!("{key} is projected"));
        assert_eq!((node.role.as_str(), node.label.as_deref(), node.actionable), (expected["role"].as_str().expect("role"), expected["label"].as_str(), true), "{key}");
    }
    let activation = &law["expected"]["accessibilityActivation"];
    let key = activation["key"].as_str().expect("activation key");
    let record_id = UiNodeId(activation["record"].as_u64().expect("activation record"));
    let index = {
        let record = tree.document().and_then(|document| document.record(record_id)).expect("row record");
        assert_eq!(row_accessibility_action(record, &format!("{}::row-action::9", record.key.as_str())), None, "a key naming no action activates nothing");
        row_accessibility_action(record, key).expect("the virtual key names one of the row's actions")
    };
    let row = mounted_node(&tree, activation);
    let mut router = EventRouter::new("main");
    let commands = router.dispatch_accessibility_row_action(&mut tree, row, index, &AccessibilityUiEvent::Activate);
    assert_eq!(fired(&commands), vec![activation["fires"].as_str().expect("fires").to_string()]);
}
