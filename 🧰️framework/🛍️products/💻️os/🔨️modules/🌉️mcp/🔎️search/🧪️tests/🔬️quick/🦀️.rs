
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

//#region 🧪️M5aAgentUsabilityLaws
/// 🎯️ M5a law 1 — an agent's query DISCRIMINATES. Before this slice `"draw rectangle"` returned 20
/// draw verbs at two distinct BM25 scores (only the plugin id/title matched, every description was
/// empty); the intent verb had to win on its own text.
#[test]
fn rectangle_query_ranks_the_intent_verb_first_and_spreads_the_scores() {
    let catalog = compile(&source_builders::note_cad_and_draw_source(), Locale::En, Terminology::Native).expect("compiles");
    let hits = search(&catalog, "draw rectangle", &SearchFilters::default());
    assert!(!hits.is_empty(), "expected hits for `draw rectangle`");
    assert_eq!(hits[0].capability_id, "draw.s.draw.drawing@1/*#editor.addLayer");
    let top: Vec<&RankedHit> = hits.iter().take(5).collect();
    let distinct: std::collections::BTreeSet<String> = top.iter().map(|hit| format!("{:.4}", hit.score)).collect();
    assert_eq!(distinct.len(), top.len(), "no two of the top five may tie on score (the pre-M5a defect was 16 verbs at 6.9096): {top:?}");
    assert!(hits[0].score > hits[1].score * 1.5, "the intent verb must win clearly: {:?} vs {:?}", hits[0], hits[1]);
}

/// 🎯️ M5a law 2 — `"layer"` finds the layer verbs, and `"export pdf"` finds the export verb, with
/// the right plugin's verb on top in both cases.
#[test]
fn layer_and_export_pdf_queries_discriminate() {
    let catalog = compile(&source_builders::note_cad_and_draw_source(), Locale::En, Terminology::Native).expect("compiles");
    let layer = search(&catalog, "rename a layer", &SearchFilters::default());
    assert_eq!(layer[0].capability_id, "draw.s.draw.drawing@1/*#editor.patchLayer");
    let export = search(&catalog, "export the document as pdf", &SearchFilters::default());
    assert_eq!(export[0].capability_id, "draw.s.draw.drawing@1/*#editor.exportDocument");
}

/// 🖱️ M5a law 3 — no raw input event is reachable from the agent catalog at all, whatever the query.
#[test]
fn input_events_are_never_published_to_agents() {
    let catalog = compile(&source_builders::note_cad_and_draw_source(), Locale::En, Terminology::Native).expect("compiles");
    for id in ["draw.s.draw.drawing@1/*#editor.canvasPointerMove", "draw.s.draw.drawing@1/*#editor.canvasPointerDown", "draw.s.draw.drawing@1/*#editor.canvasDoubleClick", "draw.s.draw.drawing@1/*#editor.canvasEscape", "draw.s.draw.drawing@1/*#editor.engagementInput", "draw.s.draw.drawing@1/*#editor.setCamera"] {
        assert!(catalog.get(id).is_none(), "{id} must not be in the agent catalog");
    }
    for query in ["draw rectangle", "canvas pointer", "move the pointer on the canvas", "escape"] {
        for hit in search(&catalog, query, &SearchFilters::default()) {
            let capability = catalog.get(&hit.capability_id).expect("hit resolves");
            assert_eq!(capability.audience, crate::catalog::CapabilityAudience::Agent, "{} leaked into a `{query}` result", hit.capability_id);
        }
    }
}

/// 💬️ M5a law 4 — every published draw verb carries a real localized description; an agent never has
/// to choose between two verbs whose only difference is their id.
#[test]
fn every_published_draw_verb_has_a_description_in_both_locales() {
    for locale in [Locale::En, Locale::De] {
        let catalog = compile(&source_builders::note_cad_and_draw_source(), locale, Terminology::Native).expect("compiles");
        for capability in catalog.entries.iter().filter(|entry| entry.id.as_str().starts_with("draw.")) {
            assert!(!capability.description.trim().is_empty(), "{} has no description in {locale:?}", capability.id);
        }
    }
}

/// 📐️ M5a law 5 — the published input schema is a real JSON Schema with the verb's own parameters,
/// including the enumerated vocabulary the guest actually accepts.
#[test]
fn add_layer_publishes_its_real_input_schema() {
    let catalog = compile(&source_builders::note_cad_and_draw_source(), Locale::En, Terminology::Native).expect("compiles");
    let capability = catalog.get("draw.s.draw.drawing@1/*#editor.addLayer").expect("addLayer is published");
    let schema = &capability.input_schema;
    assert_eq!(schema["type"], "object");
    let kind = &schema["properties"]["kind"];
    assert_eq!(kind["type"], "string");
    let enumerated: Vec<&str> = kind["enum"].as_array().expect("kind is an enum").iter().filter_map(|value| value.as_str()).collect();
    assert!(enumerated.contains(&"shape:rect"), "the rectangle vocabulary must be published: {enumerated:?}");
    assert!(!capability.presentation.args.is_empty(), "the arg summary search indexes must be populated");
}

/// 🏷️ M5a law 6 — the declaring plugin's display name is indexed, so naming the product finds it.
#[test]
fn plugin_display_name_is_searchable() {
    let catalog = compile(&source_builders::note_cad_and_draw_source(), Locale::En, Terminology::Native).expect("compiles");
    let capability = catalog.get("draw.s.draw.drawing@1/*#editor.addLayer").expect("addLayer is published");
    assert!(matches!(&capability.owner, CapabilityOwner::Plugin { label, .. } if label.as_deref() == Some("Draw")));
    assert_eq!(capability.artifact_kind.as_deref(), Some("s.draw.drawing"));
}
//#endregion 🧪️M5aAgentUsabilityLaws

//#region 🧪️M5aApprovalAndAuditLaws
/// ⚠️ M5a law 7 — a delete-class verb published to an agent carries `effects.destructive` AND the
/// `WhenDestructive` approval mode, which is the single fact `🛡️policy::requires_approval` reads.
/// Without it the gateway commits a deletion on an agent's word alone.
#[test]
fn a_delete_class_verb_is_marked_destructive_and_gates_on_approval() {
    let catalog = compile(&source_builders::note_cad_and_draw_source(), Locale::En, Terminology::Native).expect("compiles");
    let capability = catalog.get("draw.s.draw.drawing@1/*#editor.deleteLayer").expect("deleteLayer is published");
    assert!(capability.effects.destructive, "deleteLayer must declare itself destructive");
    assert_eq!(capability.policy.approval, semio_framework::manifest::ApprovalMode::WhenDestructive);
    let add = catalog.get("draw.s.draw.drawing@1/*#editor.addLayer").expect("addLayer is published");
    assert!(!add.effects.destructive, "an additive verb must not ask for approval");
    assert_eq!(add.policy.approval, semio_framework::manifest::ApprovalMode::WhenDestructive, "the kind default is unchanged — it is `destructive` that decides");
}

/// 🚨️ M5a law 8 — the audit is the thing that makes §7.4 detectable: a `Mutation`-kind gesture route
/// published to agents with NO declared audience is a finding, and a destructive-class mutation with
/// `effects.destructive = false` is a finding. The hand-annotated fixture is clean; a source that
/// drops both declarations is not.
#[test]
fn the_capability_audit_catches_an_undeclared_gesture_route_and_an_unmarked_destructive_verb() {
    let clean = source_builders::note_cad_and_draw_source();
    assert_eq!(crate::catalog::audit_source(&clean), Vec::new(), "the annotated fixture must audit clean");

    let mut regressed = clean.clone();
    for descriptor in regressed.descriptors.iter_mut().filter(|descriptor| descriptor.manifest.plugin_id == "draw") {
        for app in descriptor.manifest.apps.iter_mut() {
            for window_kind in app.window_kinds.iter_mut() {
                for action in window_kind.actions.iter_mut() {
                    if action.id == "canvasPointerDown" {
                        action.semantics.audience = None;
                        action.in_palette = true;
                    }
                    if action.id == "deleteLayer" {
                        action.semantics.effects.destructive = false;
                    }
                }
            }
        }
    }
    let findings = crate::catalog::audit_source(&regressed);
    assert!(
        findings.contains(&crate::catalog::CatalogAuditFinding::UndeclaredGestureRoute { capability_id: "draw.s.draw.drawing@1/*#editor.canvasPointerDown".into(), matched: "pointerdown" }),
        "the undeclared pointer route must be a finding: {findings:?}"
    );
    assert!(
        findings.contains(&crate::catalog::CatalogAuditFinding::UnmarkedDestructiveVerb { capability_id: "draw.s.draw.drawing@1/*#editor.deleteLayer".into(), matched: "delete" }),
        "the unmarked destructive verb must be a finding: {findings:?}"
    );
}
//#endregion 🧪️M5aApprovalAndAuditLaws
