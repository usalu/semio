//! 🔎️ Deterministic capability search — packet `P2-catalog`. BM25 (`k1=1.2`, `b=0.75`) over five
//! weighted fields (id ×3, title ×3, use_when ×2, description ×1, category/owner ×0.5), a
//! camelCase/kebab-case-aware tokenizer, en+de stopword filtering, and structural filters. **No LLM,
//! no randomness, no `HashMap` iteration anywhere in this file** — every collection that could leak
//! nondeterminism into the ranking is a `Vec`/`BTreeMap`/`BTreeSet`, and every tie is broken by
//! capability id (`Ord` on `String`) so two searches over the same catalog always agree byte-for-byte.

use crate::catalog::{CapabilityDefinition, CapabilityOwner, Catalog};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Tokenizer
/// 🌐️ English + German stopwords — short, closed, deliberately conservative (a false-positive
/// stopword would silently swallow a real query term; missing one just costs a little precision).
const STOPWORDS: &[&str] = &[
    "the", "a", "an", "of", "to", "in", "on", "for", "and", "or", "is", "are", "with", "by", "at", "from", "this", "that", "it", "as", "be", "der", "die", "das", "den", "dem", "des", "ein", "eine", "einen", "einem", "eines", "und", "oder", "mit",
    "für", "von", "zu", "im", "am", "ist", "sind",
];

/// 🧩️ Lowercases, splits on every non-alphanumeric boundary (covers kebab-case AND snake_case for
/// free) AND on a lower→upper transition inside a run of letters (covers camelCase), then drops
/// stopwords and empty tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    let mut raw_tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    for character in text.chars() {
        if character.is_alphanumeric() {
            if character.is_uppercase() {
                if let Some(previous) = current.chars().last() {
                    if previous.is_lowercase() || previous.is_numeric() {
                        raw_tokens.push(std::mem::take(&mut current));
                    }
                }
            }
            current.push(character);
        } else if !current.is_empty() {
            raw_tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        raw_tokens.push(current);
    }
    raw_tokens.into_iter().map(|token| token.to_lowercase()).filter(|token| !token.is_empty() && !STOPWORDS.contains(&token.as_str())).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️Filters
/// 🎛️ `capabilities_search` structural filters — `📋️master.md` §3.2: `kind[]`, `owner`,
/// `artifactKind`, `requiresScope`.
#[derive(Clone, Debug, Default)]
pub struct SearchFilters {
    pub kind: Vec<crate::catalog::CapabilityKind>,
    pub owner: Option<String>,
    pub artifact_kind: Option<String>,
    pub requires_scope: Option<String>,
}

fn owner_matches(owner: &CapabilityOwner, filter: &str) -> bool {
    match owner {
        CapabilityOwner::Os => filter == "os",
        CapabilityOwner::Framework => filter == "framework",
        CapabilityOwner::Shell => filter == "shell",
        CapabilityOwner::Gateway => filter == "gateway",
        CapabilityOwner::Extension { extension_id } => extension_id == filter,
        CapabilityOwner::Plugin { plugin_id, .. } => plugin_id == filter,
    }
}

fn passes_filters(capability: &CapabilityDefinition, filters: &SearchFilters) -> bool {
    if !filters.kind.is_empty() && !filters.kind.contains(&capability.kind) {
        return false;
    }
    if let Some(owner) = &filters.owner {
        if !owner_matches(&capability.owner, owner) {
            return false;
        }
    }
    if let Some(artifact_kind) = &filters.artifact_kind {
        if capability.artifact_kind.as_deref() != Some(artifact_kind.as_str()) {
            return false;
        }
    }
    if let Some(scope) = &filters.requires_scope {
        if !capability.policy.scopes.iter().any(|granted| &granted.0 == scope) {
            return false;
        }
    }
    true
}
//#endregion 🔖️Filters

//#region 🔖️RankedHit
#[derive(Clone, Debug, PartialEq)]
pub struct RankedHit {
    pub capability_id: String,
    pub score: f64,
}
//#endregion 🔖️RankedHit

//#region 🔖️Bm25
/// 🏷️ Field weights, in the order every document's field vector is built — `id ×3, title ×3,
/// use_when ×2, description ×1, category/owner ×0.5`.
const K1: f64 = 1.2;
const B: f64 = 0.75;

struct WeightedField {
    tokens: Vec<String>,
    weight: f64,
}

fn owner_text(owner: &CapabilityOwner) -> String {
    match owner {
        CapabilityOwner::Os => "os".to_string(),
        CapabilityOwner::Framework => "framework".to_string(),
        CapabilityOwner::Shell => "shell".to_string(),
        CapabilityOwner::Gateway => "gateway".to_string(),
        CapabilityOwner::Extension { extension_id } => extension_id.clone(),
        CapabilityOwner::Plugin { plugin_id, app_id, .. } => format!("{plugin_id} {}", app_id.clone().unwrap_or_default()),
    }
}

fn build_fields(capability: &CapabilityDefinition) -> Vec<WeightedField> {
    vec![
        WeightedField { tokens: tokenize(capability.id.as_str()), weight: 3.0 },
        WeightedField { tokens: tokenize(&capability.title), weight: 3.0 },
        WeightedField { tokens: tokenize(&capability.use_when.join(" ")), weight: 2.0 },
        WeightedField { tokens: tokenize(&capability.description), weight: 1.0 },
        WeightedField { tokens: tokenize(&format!("{} {}", capability.presentation.category.clone().unwrap_or_default(), owner_text(&capability.owner))), weight: 0.5 },
    ]
}

fn weighted_term_frequency(fields: &[WeightedField], term: &str) -> f64 {
    fields.iter().map(|field| field.tokens.iter().filter(|token| token.as_str() == term).count() as f64 * field.weight).sum()
}

fn weighted_doc_length(fields: &[WeightedField]) -> f64 {
    fields.iter().map(|field| field.tokens.len() as f64 * field.weight).sum()
}

/// 🔎️ Ranks every capability in `catalog` (after `filters`) against `query` via BM25F-style scoring
/// over the five weighted fields. Empty/no-match query → empty result (never all-zero-score noise).
/// Ties broken by capability id — deterministic across repeated calls on the same catalog.
pub fn search(catalog: &Catalog, query: &str, filters: &SearchFilters) -> Vec<RankedHit> {
    let query_tokens = tokenize(query);
    if query_tokens.is_empty() {
        return Vec::new();
    }
    let candidates: Vec<&CapabilityDefinition> = catalog.entries.iter().filter(|capability| passes_filters(capability, filters)).collect();
    if candidates.is_empty() {
        return Vec::new();
    }
    let field_sets: Vec<Vec<WeightedField>> = candidates.iter().map(|capability| build_fields(capability)).collect();
    let doc_lengths: Vec<f64> = field_sets.iter().map(|fields| weighted_doc_length(fields)).collect();
    let total_docs = candidates.len() as f64;
    let average_length = doc_lengths.iter().sum::<f64>() / total_docs;

    let mut document_frequency: BTreeMap<String, usize> = BTreeMap::new();
    for fields in &field_sets {
        let mut present: BTreeSet<&str> = BTreeSet::new();
        for field in fields {
            for token in &field.tokens {
                present.insert(token.as_str());
            }
        }
        for token in present {
            *document_frequency.entry(token.to_string()).or_insert(0) += 1;
        }
    }

    let mut scored: Vec<(String, f64)> = Vec::new();
    for (index, capability) in candidates.iter().enumerate() {
        let fields = &field_sets[index];
        let doc_length = doc_lengths[index];
        let mut score = 0.0_f64;
        for term in &query_tokens {
            let term_frequency = weighted_term_frequency(fields, term);
            if term_frequency <= 0.0 {
                continue;
            }
            let document_count = *document_frequency.get(term).unwrap_or(&0) as f64;
            let inverse_document_frequency = ((total_docs - document_count + 0.5) / (document_count + 0.5) + 1.0).ln();
            let normalization = if average_length > 0.0 { doc_length / average_length } else { 1.0 };
            score += inverse_document_frequency * (term_frequency * (K1 + 1.0)) / (term_frequency + K1 * (1.0 - B + B * normalization));
        }
        if score > 0.0 {
            scored.push((capability.id.as_str().to_string(), score));
        }
    }

    scored.sort_by(|left, right| right.1.partial_cmp(&left.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| left.0.cmp(&right.0)));
    scored.into_iter().map(|(capability_id, score)| RankedHit { capability_id, score }).collect()
}
//#endregion 🔖️Bm25

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::catalog::compile;
    use crate::fixtures;
    use semio_framework::{Locale, Terminology};

    #[test]
    fn tokenizer_splits_camel_case_and_kebab_case_and_drops_stopwords() {
        assert_eq!(tokenize("translateSelection"), vec!["translate", "selection"]);
        assert_eq!(tokenize("set-grid-visible"), vec!["set", "grid", "visible"]);
        assert_eq!(tokenize("move the selection"), vec!["move", "selection"]);
    }

    #[test]
    fn move_the_selection_finds_cad_translate_selection_as_top_hit() {
        let source = fixtures::note_and_cad_source();
        let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        let hits = search(&catalog, "move the selection", &SearchFilters::default());
        assert!(!hits.is_empty(), "expected at least one hit");
        assert_eq!(hits[0].capability_id, "cad.editor.translateSelection");
    }

    #[test]
    fn search_is_deterministic_across_repeated_calls() {
        let source = fixtures::note_and_cad_source();
        let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        let first = search(&catalog, "delete the selection", &SearchFilters::default());
        let second = search(&catalog, "delete the selection", &SearchFilters::default());
        assert_eq!(first, second);
    }

    #[test]
    fn kind_filter_excludes_non_matching_capabilities() {
        let source = fixtures::note_and_cad_source();
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
        let source = fixtures::note_and_cad_source();
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
        let source = fixtures::note_and_cad_source();
        let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
        assert!(search(&catalog, "   ", &SearchFilters::default()).is_empty());
    }
}
//#endregion 🧪️Tests
