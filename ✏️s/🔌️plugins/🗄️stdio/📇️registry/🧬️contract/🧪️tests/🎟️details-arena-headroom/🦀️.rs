//! 🎟️ The stdio details panel honours the process-wide `UiValue` arena: a render the arena cannot fully price
//! renders a SHORTER window — every section still stamps its full extent — and never an assembly refusal, and the
//! complete window returns as soon as the credit does. `take_arena_unbuilt_rows` names the rows a short window left out
//! for want of credit — what the reactor renders again once the credit returns — and stays 0 for a whole window. The arena prices ONE page of interactive rows
//! (`UI_VALUE_PAGE_ROWS` × `UI_VALUE_ROW_COLLECTIONS`) and a retained previous generation keeps its own collections
//! until the reactor retires it, so a quick edit → undo → redo renders against whatever headroom is left
//! (S16 program matrix, stdio/csv redo: `ui.snapshot-details.arguments: snapshot details UI admission failed`).
//! Cases: `✏️editing/🧫️fixtures/🎟️details-arena-headroom` (schema `✏️editing/🧬️schema/🎟️details-arena-headroom`). The
//! headroom of a case is a share of what the complete render of its document measurably holds, so the law follows the
//! panel's own cost as it evolves instead of pinning today's collection count.
//! Oracle: every materialised control's bound pointer resolves in the document through serde_json's RFC 6901
//! `pointer` (third-party) — an insert binds its parent — so a shortened window never binds a row that is not there.

use semio_framework_os_kernel::{DslValue, Number};
use semio_framework_plugin::{take_arena_unbuilt_rows, BuiltNode, Component, Locale, TreeWindows, UiMapBuilder, UiValue};
use semio_framework_ui_contract as ui;
use semio_s_artifact_stdio_contract::editing::{render_snapshot_details_provider, SnapshotDetailPathSegment, SnapshotDetailValue, SnapshotDetailsProvider};

/// 📊️ A csv-shaped snapshot: `records[r].fields[c] = {value, quoted}`, typed (schema-bound collections) or untyped.
struct Document {
    value: DslValue,
    typed: bool,
}

impl Document {
    fn csv(records: usize, fields: usize, typed: bool) -> Self {
        let field = |text: String| DslValue::Object(vec![("value".into(), DslValue::String(text)), ("quoted".into(), DslValue::Bool(false))]);
        let record = |row: usize| DslValue::Object(vec![("fields".into(), DslValue::Array((0..fields).map(|column| field(format!("r{row}c{column}"))).collect()))]);
        let value = DslValue::Object(vec![
            ("schema".into(), DslValue::String("stdio.csv".into())),
            ("hasHeader".into(), DslValue::Bool(true)),
            ("records".into(), DslValue::Array((0..records).map(record).collect())),
        ]);
        Self { value, typed }
    }

    fn at(&self, path: &[SnapshotDetailPathSegment]) -> Option<&DslValue> {
        path.iter().try_fold(&self.value, |value, segment| match (value, segment) {
            (DslValue::Object(entries), SnapshotDetailPathSegment::Key(key)) => entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value),
            (DslValue::Array(items), SnapshotDetailPathSegment::Index(index)) => items.get(*index),
            _ => None,
        })
    }

    fn json(&self) -> serde_json::Value {
        fn convert(value: &DslValue) -> serde_json::Value {
            match value {
                DslValue::Null => serde_json::Value::Null,
                DslValue::Bool(value) => serde_json::Value::Bool(*value),
                DslValue::Number(Number::UInt(value)) => serde_json::json!(value),
                DslValue::Number(Number::Int(value)) => serde_json::json!(value),
                DslValue::Number(Number::Float(value)) => serde_json::json!(value),
                DslValue::String(value) => serde_json::Value::String(value.clone()),
                DslValue::Array(items) => serde_json::Value::Array(items.iter().map(convert).collect()),
                DslValue::Object(entries) => serde_json::Value::Object(entries.iter().map(|(key, value)| (key.clone(), convert(value))).collect()),
            }
        }
        convert(&self.value)
    }
}

impl SnapshotDetailsProvider for Document {
    fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
        Some(match self.at(path)? {
            DslValue::Null => SnapshotDetailValue::Null,
            DslValue::Bool(value) => SnapshotDetailValue::Bool(*value),
            DslValue::Number(value) => SnapshotDetailValue::Number(*value),
            DslValue::String(value) => SnapshotDetailValue::String(value.clone()),
            DslValue::Array(_) => SnapshotDetailValue::Array,
            DslValue::Object(_) => SnapshotDetailValue::Object,
        })
    }

    fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        match self.at(path) {
            Some(DslValue::Array(items)) => items.len(),
            Some(DslValue::Object(entries)) => entries.len(),
            _ => 0,
        }
    }

    fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
        match self.at(path)? {
            DslValue::Object(entries) => entries.get(index).map(|(key, _)| key.clone()),
            _ => None,
        }
    }

    fn has_source(&self) -> bool {
        true
    }

    fn source(&self) -> Option<String> {
        Some(self.json().to_string())
    }

    fn creation_template(&self, path: &[SnapshotDetailPathSegment], collection_item: bool) -> Option<DslValue> {
        (self.typed && collection_item && matches!(self.at(path)?, DslValue::Array(_))).then(|| DslValue::Object(vec![("value".into(), DslValue::String(String::new())), ("quoted".into(), DslValue::Bool(false))]))
    }

    fn allows_untyped_creation(&self, _path: &[SnapshotDetailPathSegment]) -> bool {
        !self.typed
    }

    fn allows_collection_insert(&self, path: &[SnapshotDetailPathSegment]) -> bool {
        matches!(self.at(path), Some(DslValue::Array(_) | DslValue::Object(_)))
    }
}

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

/// 🧮️ The arena collections a complete render of `document` holds while its tree is alive, measured on an idle arena.
fn complete_cost(document: &Document, locale: Locale, controller: &str) -> usize {
    drain();
    let before = ui::ui_value_headroom().collections;
    let complete = render_snapshot_details_provider(document, locale, controller, &TreeWindows::unhosted()).expect("a render on an idle arena succeeds");
    let cost = before - ui::ui_value_headroom().collections;
    drop(complete);
    drain();
    cost
}

/// 🎟️ The collections a case leaves free: a share of the complete render's cost, or that cost plus one priced row.
fn free_collections(share: &str, cost: usize) -> usize {
    match share {
        "none" => 0,
        "third" => cost / 3,
        "half" => cost / 2,
        "complete-plus-one-row" => cost + ui::UI_VALUE_ROW_COLLECTIONS,
        other => panic!("unknown headroom share {other}"),
    }
}

/// 🎟️ Holds empty argument maps until exactly `free` arena collections are left — the credit a previous
/// generation still holds while the reactor retires it.
fn hold_until(free: usize) -> Vec<UiMapBuilder> {
    let mut held = Vec::new();
    while ui::ui_value_headroom().collections > free {
        held.push(UiMapBuilder::try_new().expect("the arena admits while headroom remains"));
    }
    assert_eq!(ui::ui_value_headroom().collections, free, "filler reached the requested headroom");
    held
}

/// 🧾️ Materialised detail rows and the extent the details section stamps.
fn census(node: &BuiltNode, section: &str) -> (usize, Option<u32>) {
    fn rows(node: &BuiltNode) -> usize {
        usize::from(matches!(node.component, Component::TreeItem(_))) + node.children.iter().map(rows).sum::<usize>()
    }
    fn find<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        (node.key.as_str() == key).then_some(node).or_else(|| node.children.iter().find_map(|child| find(child, key)))
    }
    let section = find(node, section).expect("the details section is always built");
    let total = match &section.component {
        Component::TreeSection(props) => props.window.as_ref().map(|window| window.total),
        _ => None,
    };
    (rows(section), total)
}

/// 🔎️ Every bound pointer of the tree resolves in `document` (an insert: its parent), by serde_json's RFC 6901.
fn assert_bound_pointers_resolve(node: &BuiltNode, document: &serde_json::Value, case: &str) -> usize {
    let mut checked = 0;
    for binding in node.bindings.iter() {
        let Some(UiValue::Map(arguments)) = &binding.args else { continue };
        let insert = binding.action.name.as_str() == "insertSnapshotValue";
        for (key, value) in arguments.iter() {
            if !matches!(key.as_str(), "path" | "from") {
                continue;
            }
            let UiValue::Text(pointer) = value else { continue };
            let pointer = pointer.as_str();
            let resolved = if insert && key.as_str() == "path" { pointer.rsplit_once('/').map_or("", |(parent, _)| parent) } else { pointer };
            assert!(document.pointer(resolved).is_some(), "{case}: bound pointer {pointer:?} ({}) resolves in the document", binding.action.name.as_str());
            checked += 1;
        }
    }
    checked + node.children.iter().map(|child| assert_bound_pointers_resolve(child, document, case)).sum::<usize>()
}

#[test]
fn details_render_a_shorter_window_when_the_arena_is_short_and_recover_when_it_is_not() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../✏️editing/🧫️fixtures/🎟️details-arena-headroom/🔣️.json")).expect("neutral arena-headroom fixture");
    let section = fixture["section"].as_str().expect("section key");
    let controller = fixture["controller"].as_str().expect("controller");
    let locales = [(Locale::En, "en"), (Locale::De, "de")];
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let document = Document::csv(case["records"].as_u64().unwrap() as usize, case["fields"].as_u64().unwrap() as usize, case["typed"].as_bool().unwrap());
        let json = document.json();
        for (locale, tag) in locales {
            drain();
            let _ = take_arena_unbuilt_rows();
            let complete = render_snapshot_details_provider(&document, locale, controller, &TreeWindows::unhosted()).unwrap_or_else(|error| panic!("{id}/{tag}: a render on an idle arena succeeds: {} {}", error.code, error.message));
            let (complete_rows, complete_total) = census(&complete, section);
            assert!(complete_rows > 0, "{id}/{tag}: the idle arena materialises detail rows");
            assert_eq!(take_arena_unbuilt_rows(), 0, "{id}/{tag}: an idle arena leaves no row unbuilt");
            assert!(assert_bound_pointers_resolve(&complete, &json, id) > 0, "{id}/{tag}: controls bind document pointers");
            drop(complete);
            drain();
            let cost = complete_cost(&document, locale, controller);
            assert!(cost > 0, "{id}/{tag}: a complete render holds arena collections");
            let free = free_collections(case["headroom"].as_str().expect("headroom share"), cost);
            let held = hold_until(free);
            let short = render_snapshot_details_provider(&document, locale, controller, &TreeWindows::unhosted()).unwrap_or_else(|error| panic!("{id}/{tag}: a short arena shortens the window instead of refusing: {} {}", error.code, error.message));
            let (short_rows, short_total) = census(&short, section);
            assert_eq!(short_total, complete_total, "{id}/{tag}: the section stamps its full extent");
            match case["expect"].as_str().unwrap() {
                "shortened" => {
                    assert!(short_rows < complete_rows, "{id}/{tag}: {short_rows} rows under {free} free collections, {complete_rows} complete ({cost} collections)");
                    assert!(take_arena_unbuilt_rows() > 0, "{id}/{tag}: the arena, not the host's request, cut the window");
                }
                "complete" => {
                    assert_eq!(short_rows, complete_rows, "{id}/{tag}: headroom for the whole window ({free} free, {cost} priced) renders it whole");
                    assert_eq!(take_arena_unbuilt_rows(), 0, "{id}/{tag}: headroom for the whole window leaves no row unbuilt");
                }
                other => panic!("{id}: unknown expectation {other}"),
            }
            assert_bound_pointers_resolve(&short, &json, id);
            drop(short);
            drop(held);
            drain();
            let recovered = render_snapshot_details_provider(&document, locale, controller, &TreeWindows::unhosted()).unwrap_or_else(|error| panic!("{id}/{tag}: the returned credit renders again: {} {}", error.code, error.message));
            assert_eq!(census(&recovered, section).0, complete_rows, "{id}/{tag}: the complete window returns with the credit");
            assert_eq!(take_arena_unbuilt_rows(), 0, "{id}/{tag}: the returned credit leaves no row unbuilt");
        }
    }
}
