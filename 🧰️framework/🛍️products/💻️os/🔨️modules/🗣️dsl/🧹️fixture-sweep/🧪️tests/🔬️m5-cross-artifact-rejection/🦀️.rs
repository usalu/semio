use super::m5_auto_discovery;
use super::m5_soft_skip::soft_skip_missing;
use super::pilot_resolve;
use crate::os_dsl::{parse_grammar, Recognizer, SemioDialect};
use crate::os_store::semio_format::split_text_preamble;

async fn dsl_body_from_fixture(text: &str) -> String {
    if text.trim_start().starts_with("semio ") {
        split_text_preamble(text).map_or_else(|_| text.to_string(), |(env, body)| format!("{}\n{body}", env.envelope_id()))
    } else {
        text.to_string()
    }
}

#[semio_framework_async_macros::async_test]
async fn all_non_stdio_grammars_reject_each_others_shipped_fixtures() {
    let facets = m5_auto_discovery::discover_grammar_snapshot_facets();
    let mut usable: Vec<(String, Recognizer, String)> = Vec::new();
    for facet in &facets.await {
        if facet.is_stdio {
            continue;
        }
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
        let Ok(grammar) = parse_grammar(&grammar_text) else { continue };
        if grammar.dialect != SemioDialect::Grammar {
            continue;
        }
        usable.push((facet.label.clone(), Recognizer::compile(&grammar), dsl_body_from_fixture(&fixture_text).await));
    }

    if usable.len() < 2 {
        return;
    }

    let mut failures: Vec<String> = Vec::new();
    for i in 0..usable.len() {
        for j in (i + 1)..usable.len() {
            let (label_a, recognizer_a, body_a) = &usable[i];
            let (label_b, recognizer_b, body_b) = &usable[j];
            if recognizer_a.recognize(body_b).unwrap_or(false) {
                failures.push(format!("{label_a} grammar must reject {label_b}'s fixture body"));
            }
            if recognizer_b.recognize(body_a).unwrap_or(false) {
                failures.push(format!("{label_b} grammar must reject {label_a}'s fixture body"));
            }
        }
    }
    assert!(failures.is_empty(), "m5 cross-artifact rejection failed for {} pair(s):\n\n{}", failures.len(), failures.join("\n\n"));
}
