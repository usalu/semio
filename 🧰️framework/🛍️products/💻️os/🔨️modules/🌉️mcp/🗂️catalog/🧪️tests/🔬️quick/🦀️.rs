
use super::*;
use crate::fixtures;

#[test]
fn compiling_the_same_source_twice_is_byte_identical() {
    let source = fixtures::note_and_cad_source();
    let first = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let second = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    assert_eq!(first.hash, second.hash);
    assert_eq!(first.entries, second.entries);
}

#[test]
fn entries_are_sorted_by_id() {
    let source = fixtures::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let ids: Vec<&str> = catalog.entries.iter().map(|entry| entry.id.as_str()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    assert_eq!(ids, sorted);
}

/// 🆔️ D3: two plugins declaring the SAME bare action id must compile into two distinct
/// capability ids — a bare action id is never a capability id.
#[test]
fn two_plugins_declaring_the_same_action_id_compile_to_distinct_capability_ids() {
    let source = fixtures::colliding_action_id_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles distinct ids without error");
    assert!(catalog.get("plugin-a.surface.deleteSelection").is_some());
    assert!(catalog.get("plugin-b.surface.deleteSelection").is_some());
    assert_ne!(catalog.get("plugin-a.surface.deleteSelection").unwrap().id, catalog.get("plugin-b.surface.deleteSelection").unwrap().id);
}

#[test]
fn cad_translate_selection_compiles_with_the_dxyz_input_schema() {
    let source = fixtures::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let capability = catalog.get("cad.editor.translateSelection").expect("translateSelection present");
    assert_eq!(capability.kind, CapabilityKind::Mutation);
    let properties = capability.input_schema["properties"].as_object().expect("object schema");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    assert_eq!(capability.id.as_str(), fixture["catalog"]["capabilityId"].as_str().expect("fixture capability id"));
    assert_eq!(properties, fixture["catalog"]["properties"].as_object().expect("fixture properties"));
}

#[test]
fn framework_actions_dedupe_into_one_entry_per_id_across_both_apps() {
    let source = fixtures::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let framework_undo_count = catalog.entries.iter().filter(|entry| entry.id.as_str() == "framework.undo").count();
    assert_eq!(framework_undo_count, 1);
    assert!(matches!(catalog.get("framework.undo").unwrap().owner, CapabilityOwner::Framework));
}

#[test]
fn duplicate_capability_id_is_rejected() {
    let mut source = fixtures::note_and_cad_source();
    let duplicate = source.gateway.first().cloned();
    if let Some(capability) = duplicate {
        source.gateway.push(capability);
    } else {
        source.gateway.push(ui_dialog_open_capability(&[]));
        source.gateway.push(ui_dialog_open_capability(&[]));
    }
    let result = compile(&source, Locale::En, Terminology::Native);
    assert!(matches!(result, Err(CatalogError::DuplicateCapabilityId(_))));
}
