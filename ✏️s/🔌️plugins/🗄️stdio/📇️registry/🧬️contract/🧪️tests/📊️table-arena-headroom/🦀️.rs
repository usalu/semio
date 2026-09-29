//! 📊️ The stdio structural table honours the process-wide `UiValue` arena: a cell, row-action or structure argument map
//! the arena cannot admit is the framework's capacity refusal (`ui.fixed-capacity`), so the row window ends with a shorter
//! run instead of refusing the editor render, and the structure controls (add row / add column) are admitted before the
//! windowed body, so a body that exhausts the arena never takes them down. Measured before (S18 served matrix, stdio
//! csv/tsv undo, 2026-09-28): `stdio.table.cell-arguments: cell argument map capacity` refused the render after the verb.
//! Cases: `🧫️fixtures/📊️table-arena-headroom` (schema beside it). Headroom is a share of what a case's complete render
//! measurably holds beyond its structure controls, so the law follows the table's own cost as it evolves.
//! Oracle: every materialised cell binds an address `(row, column)` that serde_json's RFC 6901 `pointer` resolves in the
//! case's document, so a shortened window never binds a cell that is not there.

use semio_framework_plugin::app::{editable_table_window_row_at, row_action, row_target, TableWindowKit, WindowKit, WindowedEditableTableCell};
use semio_framework_plugin::{BuiltNode, Component, Locale, PluginAssemblyError, RowActionPlacement, TreeWindows, UiAssemblyResult, UiMapBuilder, UiValue};
use semio_framework_ui_contract as ui;
use semio_s_artifact_stdio_contract::{render_structural_table, window_kit_indexed_revision_arguments, window_kit_revision_arguments, window_kit_revisioned_cell_arguments, REMOVE_TABLE_ROW_ACTION_ID};

const CONTROLLER: &str = "s.stdio.csv@rfc4180/*#editor";

/// 🧹️ Returns every handed-back value and queued built page to the arena, as the reactor's retirement pump does.
fn drain() {
    for _ in 0..4_000_000 {
        let nodes = ui::close_built_node_page_one();
        let values = ui::close_ui_value_page_with_grant(64, 4_096).expect("value retirement queue stays valid").complete;
        if nodes && values {
            return;
        }
    }
    panic!("the arena retirement queues did not settle");
}

/// 🎟️ Holds empty argument maps until exactly `free` arena collections are left — the credit a previous generation still
/// holds while the reactor retires it.
fn hold_until(free: usize) -> Vec<UiMapBuilder> {
    let mut held = Vec::new();
    while ui::ui_value_headroom().collections > free {
        held.push(UiMapBuilder::try_new().expect("the arena admits while headroom remains"));
    }
    assert_eq!(ui::ui_value_headroom().collections, free, "filler reached the requested headroom");
    held
}

/// 📊️ One csv-shaped structural table — `rows` × `width` editable cells, a remove-row action per row — rendered through the
/// same SDK kit and stdio helpers the csv/tsv editors use.
fn structural_table(rows: usize, width: usize, editable_headers: bool, revision: &str, locale: Locale, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let header = |column: usize| format!("c{column}");
    let table = || {
        TableWindowKit::render_indexed_matrix_with_id(
            windows,
            TableWindowKit::KIND_ID,
            "Table",
            "Row",
            "Column",
            width,
            |column| ui::Label::try_from(header(column)).map_err(|_| PluginAssemblyError::new("law.column-label", "column label admission")),
            Some("Actions"),
            rows,
            |row, columns| {
                let column_offset = columns.start;
                let cells = columns
                    .map(|column| window_kit_revisioned_cell_arguments(row, column, revision).map(|arguments| WindowedEditableTableCell::new(format!("r{row}c{column}"), header(column), "set-cell", arguments)))
                    .collect::<UiAssemblyResult<Vec<_>>>()?;
                let remove = row_action("trash-2", "Remove row", REMOVE_TABLE_ROW_ACTION_ID, RowActionPlacement::Row)?;
                let target = row_target(CONTROLLER, Some(window_kit_indexed_revision_arguments("row", row, revision)?), None)?;
                editable_table_window_row_at(&format!("row-{row}"), CONTROLLER, locale, column_offset, cells, [remove], Some(target))
            },
        )
    };
    render_structural_table(width, header, editable_headers, CONTROLLER, revision, locale, windows, table)
}

fn find<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    (node.key.as_str() == key).then_some(node).or_else(|| node.children.iter().find_map(|child| find(child, key)))
}

/// 🧾️ Materialised body rows and the extent the body stamps.
fn census(node: &BuiltNode) -> (usize, Option<u32>) {
    let table = find(node, TableWindowKit::KIND_ID).expect("the table body is always built");
    let total = match &table.component {
        Component::Table(props) => props.window.as_ref().map(|window| window.total),
        _ => None,
    };
    (table.children.iter().filter(|row| matches!(row.component, Component::TableRow(_))).count(), total)
}

/// 🔎️ Every materialised `set-cell` binding addresses a cell of `document` (serde_json RFC 6901), with the case revision.
fn assert_cells_resolve(node: &BuiltNode, document: &serde_json::Value, revision: &str, case: &str) -> usize {
    let mut checked = 0;
    for binding in node.bindings.iter().filter(|binding| binding.action.name.as_str() == "set-cell") {
        let Some(UiValue::Map(arguments)) = &binding.args else { panic!("{case}: a cell binding carries its address") };
        let number = |name: &str| {
            arguments.iter().find_map(|(key, value)| (key.as_str() == name).then_some(value)).and_then(|value| match value {
                UiValue::Number(number) => Some(number as usize),
                _ => None,
            })
        };
        let (Some(row), Some(column)) = (number("row"), number("column")) else { panic!("{case}: a cell binding names its row and column") };
        assert!(document.pointer(&format!("/rows/{row}/{column}")).is_some(), "{case}: cell ({row}, {column}) resolves in the document");
        assert!(arguments.iter().any(|(key, value)| key.as_str() == "revision" && matches!(value, UiValue::Text(text) if text.as_str() == revision)), "{case}: the cell carries the rendered revision");
        checked += 1;
    }
    checked + node.children.iter().map(|child| assert_cells_resolve(child, document, revision, case)).sum::<usize>()
}

/// 🧮️ The arena collections `render` holds while its tree is alive, measured on an idle arena.
fn cost(render: impl FnOnce() -> UiAssemblyResult<BuiltNode>) -> usize {
    drain();
    let before = ui::ui_value_headroom().collections;
    let built = render().expect("a render on an idle arena succeeds");
    let cost = before - ui::ui_value_headroom().collections;
    drop(built);
    drain();
    cost
}

#[test]
fn every_table_argument_the_arena_refuses_is_a_capacity_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📊️table-arena-headroom/🔣️.json")).expect("neutral table arena fixture");
    let revision = fixture["revision"].as_str().expect("revision");
    drain();
    let held = hold_until(0);
    for (stage, refused) in [("cell", window_kit_revisioned_cell_arguments(0, 0, revision).err()), ("structure", window_kit_revision_arguments(revision).err()), ("indexed", window_kit_indexed_revision_arguments("row", 0, revision).err())] {
        let refused = refused.unwrap_or_else(|| panic!("{stage}: an exhausted arena refuses the argument map"));
        assert_eq!(refused.code, "ui.fixed-capacity", "{stage}: the arena refusal is the capacity refusal the row window ends on ({})", refused.message);
    }
    drop(held);
    drain();
}

#[test]
fn a_short_arena_shortens_the_table_window_keeps_the_structure_controls_and_recovers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📊️table-arena-headroom/🔣️.json")).expect("neutral table arena fixture");
    let revision = fixture["revision"].as_str().expect("revision");
    let locales = [(Locale::En, "en"), (Locale::De, "de")];
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let (rows, width) = (case["rows"].as_u64().unwrap() as usize, case["width"].as_u64().unwrap() as usize);
        let editable = case["editableHeaders"].as_bool().unwrap();
        let document = serde_json::json!({ "rows": (0..rows).map(|row| (0..width).map(|column| format!("r{row}c{column}")).collect::<Vec<_>>()).collect::<Vec<_>>() });
        for (locale, tag) in locales {
            let render = || structural_table(rows, width, editable, revision, locale, &TreeWindows::unhosted());
            drain();
            let complete = render().unwrap_or_else(|error| panic!("{id}/{tag}: an idle arena renders: {} {}", error.code, error.message));
            let (complete_rows, complete_total) = census(&complete);
            assert!(complete_rows > 0, "{id}/{tag}: the idle arena materialises body rows");
            assert!(assert_cells_resolve(&complete, &document, revision, id) > 0, "{id}/{tag}: cells bind document addresses");
            drop(complete);
            let controls = cost(|| structural_table(0, 0, editable, revision, locale, &TreeWindows::unhosted()));
            let body = cost(render) - controls;
            assert!(controls > 0 && body > 0, "{id}/{tag}: controls ({controls}) and body ({body}) both hold arena collections");
            for share in case["headroom"].as_array().expect("headroom shares") {
                let share = share.as_str().expect("share");
                let free = controls
                    + match share {
                        "controls" => 0,
                        "third" => body / 3,
                        "half" => body / 2,
                        "complete" => body,
                        other => panic!("{id}: unknown headroom share {other}"),
                    };
                drain();
                let held = hold_until(free);
                let short = render().unwrap_or_else(|error| panic!("{id}/{tag}/{share}: a short arena shortens the window instead of refusing: {} {}", error.code, error.message));
                assert!(find(&short, "add-row").is_some() && find(&short, "add-column").is_some(), "{id}/{tag}/{share}: the structure controls are admitted before the body");
                let (short_rows, short_total) = census(&short);
                assert_eq!(short_total, complete_total, "{id}/{tag}/{share}: the body stamps its full extent");
                if share == "complete" {
                    assert_eq!(short_rows, complete_rows, "{id}/{tag}: headroom for the whole body renders it whole");
                } else {
                    assert!(short_rows < complete_rows, "{id}/{tag}/{share}: {short_rows} rows under {free} free collections, {complete_rows} complete");
                }
                assert_cells_resolve(&short, &document, revision, id);
                drop(short);
                drop(held);
            }
            drain();
            let recovered = render().unwrap_or_else(|error| panic!("{id}/{tag}: the returned credit renders again: {} {}", error.code, error.message));
            assert_eq!(census(&recovered).0, complete_rows, "{id}/{tag}: the complete window returns with the credit");
        }
    }
}
