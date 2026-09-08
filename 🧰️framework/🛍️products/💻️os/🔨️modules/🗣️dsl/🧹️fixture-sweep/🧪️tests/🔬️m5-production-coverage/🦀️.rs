
use super::m5_auto_discovery::{self, ConformanceFacet};
use super::m5_soft_skip::soft_skip_missing;
use super::pilot_resolve;
use crate::os_dsl::{Recognizer, parse_grammar};
use crate::os_store::semio_format::split_text_preamble;

async fn dsl_body_from_fixture(text: &str) -> String {
    if text.trim_start().starts_with("semio ") { split_text_preamble(text).map_or_else(|_| text.to_string(), |(env, body)| format!("{}\n{body}", env.envelope_id())) } else { text.to_string() }
}

#[semio_framework_async_macros::async_test]
async fn all_discovered_grammars_report_uncovered_productions_for_their_shipped_fixture() {
    let facets = m5_auto_discovery::discover_grammar_snapshot_facets().await;
    assert!(!facets.is_empty(), "auto-discovery found zero snapshot grammar.semio files — discovery walk is broken");

    let mut hard_failures: Vec<String> = Vec::new();
    let mut soft_failures: Vec<String> = Vec::new();
    let mut checked = 0usize;

    for facet in &facets {
        let Ok(grammar_text) = std::fs::read_to_string(&facet.file_path) else { continue };
        if soft_skip_missing(&format!("{}.grammar", facet.label), &grammar_text).await {
            continue;
        }
        let Some(fixture_text) = pilot_resolve::read_example_text(&facet.artifact_rel, facet.standard.as_deref(), ".dsl.semio").await else {
            eprintln!("[DEBUG] soft-skip {}.fixture: no .dsl.semio under 📚️examples (🖼️assets-first walk)", facet.label);
            continue;
        };
        if soft_skip_missing(&format!("{}.fixture", facet.label), &fixture_text).await {
            continue;
        }
        // A grammar that fails to even parse is grammar_conformance's failure to surface —
        // this diagnostic only covers the uncovered-productions signal once a grammar parses.
        let Ok(grammar) = parse_grammar(&grammar_text) else { continue };
        let recognizer = Recognizer::compile(&grammar);
        let body = dsl_body_from_fixture(&fixture_text).await;
        let Ok(uncovered) = recognizer.uncovered_productions(&body) else { continue };
        if !uncovered.is_empty() {
            eprintln!("[DEBUG] {}: uncovered productions ({}) = {}", facet.label, uncovered.len(), uncovered.join(", "));
        }
        checked += 1;
        // Soft assertion for now (matches the pre-P2-M3 design): recognition must succeed;
        // the uncovered list itself stays advisory until a later wave enforces full coverage.
        if !recognizer.recognize(&body).unwrap_or(false) {
            if facet.is_stdio && m5_auto_discovery::stdio_is_exempt(ConformanceFacet::Grammar, &facet.artifact, facet.standard.as_deref()).await {
                soft_failures.push(facet.label.clone());
            } else {
                hard_failures.push(format!("{}: fixture must still recognize while coverage is tracked", facet.label));
            }
        }
    }

    eprintln!("[dsl-fixture-sweep] m5 production coverage auto-discovery: {} facet(s) found, {} checked, {} stdio-exempt soft failure(s), {} hard failure(s)", facets.len(), checked, soft_failures.len(), hard_failures.len());
    assert!(hard_failures.is_empty(), "m5 production coverage failed for {} artifact(s):\n\n{}", hard_failures.len(), hard_failures.join("\n\n"));
}
