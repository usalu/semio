
use super::*;
use crate::catalog::compile;
use crate::source_builders;
use semio_framework::{Locale, Terminology};

#[test]
fn tokenizer_splits_camel_case_and_kebab_case_and_drops_stopwords() {
    assert_eq!(tokenize("translateSelection"), vec!["translate", "selection"]);
    assert_eq!(tokenize("set-grid-visible"), vec!["set", "grid", "visible"]);
    assert_eq!(tokenize("move the selection"), vec!["move", "selection"]);
}

#[test]
fn move_the_selection_finds_cad_translate_selection_as_top_hit() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let hits = search(&catalog, "move the selection", &SearchFilters::default());
    assert!(!hits.is_empty(), "expected at least one hit");
    assert_eq!(hits[0].capability_id, "cad.editor.translateSelection");
}

#[test]
fn search_is_deterministic_across_repeated_calls() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let first = search(&catalog, "delete the selection", &SearchFilters::default());
    let second = search(&catalog, "delete the selection", &SearchFilters::default());
    assert_eq!(first, second);
}

#[test]
fn kind_filter_excludes_non_matching_capabilities() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let filters = SearchFilters { kind: vec![crate::catalog::CapabilityKind::Shell], ..Default::default() };
    let hits = search(&catalog, "move the selection", &filters);
    for hit in &hits {
        let capability = catalog.get(&hit.capability_id).expect("hit resolves");
        assert_eq!(capability.kind, crate::catalog::CapabilityKind::Shell);
    }
}

#[test]
fn owner_filter_restricts_to_one_plugin() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let filters = SearchFilters { owner: Some("note".to_string()), ..Default::default() };
    let hits = search(&catalog, "delete", &filters);
    for hit in &hits {
        let capability = catalog.get(&hit.capability_id).expect("hit resolves");
        assert!(matches!(&capability.owner, CapabilityOwner::Plugin { plugin_id, .. } if plugin_id == "note"));
    }
}

#[test]
fn empty_query_returns_no_hits() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    assert!(search(&catalog, "   ", &SearchFilters::default()).is_empty());
}
