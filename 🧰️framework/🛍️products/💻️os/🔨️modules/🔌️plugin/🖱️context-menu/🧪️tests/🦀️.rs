//! 🗣️ Context-menu selection laws over the language-agnostic `🧫️fixtures/🔣️.json` (ICU-checked by `🟦️.ts`): the Rust glossary
//! is exactly the fixture's, every phrase case and delete row reproduces, and `Menu` resolves declared rows in the caller's
//! axes with a disabled row carrying its reason.

use super::*;

const CONTEXT_MENU_SELECTION_JSON: &str = include_str!("../🧫️fixtures/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(CONTEXT_MENU_SELECTION_JSON).expect("the context-menu selection fixture")
}

fn locale_of(value: &serde_json::Value) -> Locale {
    match value.as_str() {
        Some("en") => Locale::En,
        Some("de") => Locale::De,
        other => panic!("the fixture names an unknown locale {other:?}"),
    }
}

fn kind_of(value: &serde_json::Value) -> SelectionKind {
    SelectionKind::ALL.into_iter().find(|kind| value.as_str() == Some(kind.key())).unwrap_or_else(|| panic!("the fixture names an unknown kind {value}"))
}

fn ids(count: u64, prefix: &str) -> Vec<String> {
    (0..count).map(|index| format!("{prefix}{index}")).collect()
}

/// ⚖️ LAW: the Rust selection-kind table and labels are exactly the fixture glossary, both ways, in both locales.
#[test]
fn the_selection_glossary_is_the_fixture_glossary() {
    let fixture = fixture();
    let kinds = fixture["kinds"].as_object().expect("kinds");
    assert_eq!(kinds.len(), SelectionKind::ALL.len(), "every fixture kind has a Rust variant");
    for kind in SelectionKind::ALL {
        for (locale, key) in [(Locale::En, "en"), (Locale::De, "de")] {
            let forms = &kinds[kind.key()][key];
            assert_eq!(kind.words(locale), (forms["one"].as_str().expect("one"), forms["other"].as_str().expect("other")), "{} {key}", kind.key());
            assert_eq!(delete_selection().resolve(Terminology::Native, locale), fixture["labels"]["deleteSelection"][key].as_str().expect("delete label"));
            assert_eq!(nothing_selected().resolve(Terminology::Native, locale), fixture["labels"]["nothingSelected"][key].as_str().expect("reason"));
        }
    }
}

/// ⚖️ LAW: every fixture phrase case reproduces, including the uncounted `None`.
#[test]
fn every_selection_phrase_case_reproduces() {
    for case in fixture()["cases"].as_array().expect("cases") {
        let counts: Vec<(usize, SelectionKind)> = case["counts"].as_array().expect("counts").iter().map(|entry| (entry["count"].as_u64().expect("count") as usize, kind_of(&entry["kind"]))).collect();
        assert_eq!(selection_count_phrase(locale_of(&case["locale"]), &counts).as_deref(), case["expected"].as_str(), "{}", case["name"]);
    }
}

/// ⚖️ LAW: every fixture delete row reproduces under both dispatches; an empty selection is disabled with its reason and no
/// action, and a selection names its ids in the one `delete` row.
#[test]
fn every_delete_row_case_reproduces_and_an_empty_selection_never_dispatches() {
    for case in fixture()["deleteRows"].as_array().expect("delete rows") {
        let locale = locale_of(&case["locale"]);
        let (nodes, edges) = (ids(case["nodes"].as_u64().expect("nodes"), "n"), ids(case["edges"].as_u64().expect("edges"), "e"));
        for dispatch in [NodeGraphDeleteDispatch::Direct, NodeGraphDeleteDispatch::ViaNodeGraphEdit] {
            let row = node_graph_delete_selection_spec(delete_selection().resolve(Terminology::Native, locale), &ViewModel::new(locale, Terminology::Native), &nodes, &edges, dispatch);
            assert_eq!(row.label.as_deref(), case["label"].as_str(), "{}", case["name"]);
            assert_eq!(row.reason.as_deref(), case["reason"].as_str(), "{}", case["name"]);
            assert_eq!((row.id.as_str(), row.destructive), ("delete-selection", Some(true)));
            match case["reason"].as_str() {
                Some(_) => assert_eq!((row.disabled, row.action.as_deref(), row.args.is_none()), (Some(true), None, true), "{}", case["name"]),
                None => {
                    assert_eq!(row.disabled, None, "{}", case["name"]);
                    let expected_action = match dispatch {
                        NodeGraphDeleteDispatch::Direct => "deleteSelection",
                        NodeGraphDeleteDispatch::ViaNodeGraphEdit => "nodeGraphEdit",
                    };
                    assert_eq!(row.action.as_deref(), Some(expected_action));
                }
            }
        }
    }
}

/// ⚖️ LAW: `Menu` resolves a declared row in the caller's locale and keeps a disabled row's reason in that locale.
#[test]
fn a_menu_resolves_declared_rows_and_reasons_in_the_callers_locale() {
    let mut registry = AppActionRegistry::default();
    registry.actions.insert("clearSelection".to_string(), ActionDefinition::bounded_catalog("clearSelection", LocalizedLabel::native("Clear selection", "Auswahl aufheben"), ActionKind::View));
    for (locale, label, because) in [(Locale::En, "Clear selection", "Nothing selected"), (Locale::De, "Auswahl aufheben", "Nichts ausgewählt")] {
        let rows = Menu::of(&registry, &ViewModel::new(locale, Terminology::Native)).action("clearSelection").destructive("clearSelection").disabled_because(&nothing_selected()).build();
        assert_eq!(rows.len(), 2);
        assert_eq!((rows[0].label.as_deref(), rows[0].disabled, rows[0].reason.as_deref()), (Some(label), None, None));
        assert_eq!((rows[1].label.as_deref(), rows[1].disabled, rows[1].reason.as_deref(), rows[1].action.as_deref(), rows[1].destructive), (Some(label), Some(true), Some(because), Some("clearSelection"), Some(true)));
    }
}
