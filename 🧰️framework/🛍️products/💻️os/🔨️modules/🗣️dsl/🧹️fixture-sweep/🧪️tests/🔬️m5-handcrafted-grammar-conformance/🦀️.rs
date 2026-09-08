use super::m5_auto_discovery::{self, ConformanceFacet};
use super::m5_soft_skip::soft_skip_missing;
use super::pilot_resolve;
use crate::os_dsl::{parse_grammar, Recognizer, SemioDialect};
use crate::os_store::semio_format::split_text_preamble;

pub(super) async fn dsl_body_from_fixture(text: &str) -> String {
    if text.trim_start().starts_with("semio ") {
        split_text_preamble(text).map_or_else(|_| text.to_string(), |(env, body)| format!("{}\n{body}", env.envelope_id()))
    } else {
        text.to_string()
    }
}

/// @emoji ✅️ Real check, no panics — lets the caller choose hard-assert vs. soft-log per facet.
async fn check_grammar_recognizes(grammar_semio: &str, fixture_semio: &str) -> Result<(), String> {
    let grammar = parse_grammar(grammar_semio).map_err(|error| format!("parse grammar.semio: {error:?}"))?;
    if grammar.dialect != SemioDialect::Grammar {
        return Err("expected grammar dialect".to_string());
    }
    let recognizer = Recognizer::compile(&grammar);
    let body = dsl_body_from_fixture(fixture_semio);
    let ok = recognizer.recognize(&body.await).map_err(|error| format!("recognize failed: {error:?}"))?;
    if !ok {
        return Err("grammar did not recognize shipped fixture DSL body".to_string());
    }
    Ok(())
}

#[semio_framework_async_macros::async_test]
async fn all_discovered_snapshot_grammars_recognize_their_shipped_fixtures() {
    let facets = m5_auto_discovery::discover_grammar_snapshot_facets().await;
    assert!(!facets.is_empty(), "auto-discovery found zero 🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio files under ✏️s/🔌️plugins — discovery walk is broken");

    let mut hard_failures: Vec<String> = Vec::new();
    let mut soft_failures: Vec<String> = Vec::new();
    let mut checked = 0usize;
    let mut soft_skipped = 0usize;

    for facet in &facets {
        let grammar_text = std::fs::read_to_string(&facet.file_path).unwrap_or_else(|error| panic!("{}: read {}: {error}", facet.label, facet.file_path.display()));
        if soft_skip_missing(&format!("{}.grammar", facet.label), &grammar_text).await {
            soft_skipped += 1;
            continue;
        }
        let Some(fixture_text) = pilot_resolve::read_example_text(&facet.artifact_rel, facet.standard.as_deref(), ".dsl.semio").await else {
            eprintln!("[DEBUG] soft-skip {}.fixture: no .dsl.semio under 📚️examples (🖼️assets-first walk)", facet.label);
            soft_skipped += 1;
            continue;
        };
        if soft_skip_missing(&format!("{}.fixture", facet.label), &fixture_text).await {
            soft_skipped += 1;
            continue;
        }
        checked += 1;
        if let Err(detail) = check_grammar_recognizes(&grammar_text, &fixture_text).await {
            if facet.is_stdio && m5_auto_discovery::stdio_is_exempt(ConformanceFacet::Grammar, &facet.artifact, facet.standard.as_deref()).await {
                eprintln!("[DEBUG] soft (stdio-exempt, pre-FG-wave) grammar conformance failure for {}: {detail}", facet.label);
                soft_failures.push(facet.label.clone());
            } else {
                hard_failures.push(format!("{}: {detail}", facet.label));
            }
        }
    }

    eprintln!("[dsl-fixture-sweep] m5 grammar auto-discovery: {} facet(s) found, {} checked, {} soft-skipped, {} stdio-exempt soft failure(s), {} hard failure(s)", facets.len(), checked, soft_skipped, soft_failures.len(), hard_failures.len());
    assert!(hard_failures.is_empty(), "m5 grammar conformance failed for {} artifact(s):\n\n{}", hard_failures.len(), hard_failures.join("\n\n"));
}
