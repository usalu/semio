//! 🧪️ Conformance runner + eval harness — packet `P2-catalog`, `📋️master.md` §5 "Catalog conformance
//! runner". `check()` is pure and deterministic (no LLM, no network) — every finding is derived
//! entirely from the compiled `Catalog`'s own data.

use crate::catalog::{CapabilityDefinition, CapabilityKind, CapabilityOwner, Catalog, CatalogSource};
use semio_framework::manifest::{ApprovalMode, UndoMode};
use semio_framework::{Locale, Terminology};
use std::collections::BTreeMap;

//#region 🔖️Finding
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Finding {
    pub severity: Severity,
    pub capability_id: Option<String>,
    pub message: String,
}

impl Finding {
    fn error(capability_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self { severity: Severity::Error, capability_id: Some(capability_id.into()), message: message.into() }
    }
}
//#endregion 🔖️Finding

//#region 🔖️KnownScopes
/// 📇️ `📋️master.md` §3.4's scope table, right-hand `CapabilityId`s — every value a `CapabilityPolicy`
/// is allowed to name. `fs.read:*`/`fs.write:*`/`http:*`/`secrets:*` are prefix families (the table's
/// own `<root>`/`<origin>`/`<name>` placeholders).
const KNOWN_EXACT_SCOPES: [&str; 12] = ["documents.read", "documents.write", "jobs.spawn", "shell.observe", "shell.control", "ui.window", "ui.dialog", "shell.navigate", "shell.raw", "shell.clipboard", "packages.install", "process.spawn"];
const KNOWN_SCOPE_PREFIXES: [&str; 4] = ["fs.read:", "fs.write:", "http:", "secrets:"];

fn is_known_scope(scope: &str) -> bool {
    KNOWN_EXACT_SCOPES.contains(&scope) || KNOWN_SCOPE_PREFIXES.iter().any(|prefix| scope.starts_with(prefix))
}
//#endregion 🔖️KnownScopes

//#region 🔖️IdGrammar
fn validate_id_grammar(capability: &CapabilityDefinition) -> Result<(), String> {
    let id = capability.id.as_str();
    if id.is_empty() || id.chars().any(char::is_whitespace) {
        return Err("id is empty or contains whitespace".to_string());
    }
    match &capability.owner {
        CapabilityOwner::Plugin { plugin_id, .. } => {
            let prefix = format!("{plugin_id}.");
            if !id.starts_with(&prefix) {
                return Err(format!("plugin-owned id must start with `{prefix}`, got `{id}`"));
            }
        }
        CapabilityOwner::Framework => {
            if !id.starts_with("framework.") {
                return Err(format!("framework-owned id must start with `framework.`, got `{id}`"));
            }
        }
        CapabilityOwner::Os => {
            if !id.starts_with("os.") {
                return Err(format!("os-owned id must start with `os.`, got `{id}`"));
            }
        }
        CapabilityOwner::Shell | CapabilityOwner::Gateway | CapabilityOwner::Extension { .. } => {}
    }
    Ok(())
}
//#endregion 🔖️IdGrammar

//#region 🔖️Check
/// 🧪️ Every conformance rule this packet's brief §2.5 names, run over one already-compiled
/// `Catalog`: schema validity (2020-12), examples validating against their own schema, kind/effects
/// consistency, scope vocabulary, id grammar, and no duplicate `(owner, title)`. En+de label
/// non-emptiness is a SEPARATE check (`check_bilingual_labels`, below) — it needs to compile the
/// source under both locales, which a single already-compiled `Catalog` cannot answer on its own.
pub fn check(catalog: &Catalog) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut seen_owner_title: BTreeMap<(String, String), String> = BTreeMap::new();

    for capability in &catalog.entries {
        let id = capability.id.as_str();

        for (label, schema) in [("input", &capability.input_schema), ("output", &capability.output_schema)] {
            if let Err(error) = jsonschema::Validator::new(schema) {
                findings.push(Finding::error(id, format!("{label} schema failed to compile as JSON Schema 2020-12: {error}")));
            }
        }

        if let Ok(validator) = jsonschema::Validator::new(&capability.input_schema) {
            for example in &capability.examples {
                if example.input.is_null() {
                    continue;
                }
                if !validator.is_valid(&example.input) {
                    findings.push(Finding::error(id, format!("example {:?} does not validate against its own input schema", example.request)));
                }
            }
        }

        if capability.kind == CapabilityKind::Mutation && capability.effects.writes.is_empty() {
            findings.push(Finding::error(id, "Mutation-kind capability declares no writes"));
        }
        if capability.effects.reversible && matches!(capability.execution.undo, UndoMode::None) {
            findings.push(Finding::error(id, "declared reversible but undo mode is None"));
        }
        if capability.effects.destructive && capability.policy.approval == ApprovalMode::Never {
            findings.push(Finding::error(id, "declared destructive but approval mode is Never"));
        }

        for scope in &capability.policy.scopes {
            if !is_known_scope(&scope.0) {
                findings.push(Finding::error(id, format!("unknown policy scope `{}`", scope.0)));
            }
        }

        if let Err(message) = validate_id_grammar(capability) {
            findings.push(Finding::error(id, message));
        }

        let key = (capability.owner.dedup_key(), capability.title.clone());
        if let Some(previous_id) = seen_owner_title.get(&key) {
            findings.push(Finding::error(id, format!("duplicate (owner, title) with `{previous_id}`")));
        } else {
            seen_owner_title.insert(key, id.to_string());
        }
    }

    findings
}

/// 🈴️ Compiles `source` under both `Locale::En` and `Locale::De` and asserts every capability's
/// `title` resolves non-empty in each — the "en+de labels non-empty" conformance rule, which needs
/// two compiles (a single `Catalog` only ever carries one locale's resolved strings).
pub fn check_bilingual_labels(source: &CatalogSource) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (locale, name) in [(Locale::En, "en"), (Locale::De, "de")] {
        match crate::catalog::compile(source, locale, Terminology::Native) {
            Ok(catalog) => {
                for capability in &catalog.entries {
                    if capability.title.trim().is_empty() {
                        findings.push(Finding::error(capability.id.as_str(), format!("empty {name} title")));
                    }
                }
            }
            Err(error) => findings.push(Finding { severity: Severity::Error, capability_id: None, message: format!("{name} compile failed: {error}") }),
        }
    }
    findings
}
//#endregion 🔖️Check

//#region 🔖️Eval
/// 📖️ One `🧫️fixtures/🔣️eval.json` row.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvalCase {
    pub request: String,
    pub locale: String,
    pub expected_capability_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvalMiss {
    pub request: String,
    pub expected_capability_id: String,
    pub got_top1: Option<String>,
}

/// 📊️ `📋️master.md` §5: top-1/top-3 accuracy of `search::search` over `🧫️fixtures/🔣️eval.json` — no
/// LLM anywhere in this path. `misses` lists every case whose expected id did not appear in the top 3
/// (the diagnostic surface `📓️terra-P2-report.md` reports honestly, per the brief's "record the
/// numbers you actually measure" instruction).
#[derive(Clone, Debug, PartialEq)]
pub struct EvalReport {
    pub total: usize,
    pub top1_hits: usize,
    pub top3_hits: usize,
    pub misses: Vec<EvalMiss>,
}

impl EvalReport {
    pub fn top1_accuracy(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.top1_hits as f64 / self.total as f64
        }
    }

    pub fn top3_accuracy(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.top3_hits as f64 / self.total as f64
        }
    }
}

/// 🏃️ Runs every `EvalCase` in `cases` through `search::search` (no filters) over `catalog`.
pub fn run_eval(catalog: &Catalog, cases: &[EvalCase]) -> EvalReport {
    let mut top1_hits = 0usize;
    let mut top3_hits = 0usize;
    let mut misses = Vec::new();
    for case in cases {
        let hits = crate::search::search(catalog, &case.request, &crate::search::SearchFilters::default());
        let top1 = hits.first().map(|hit| hit.capability_id.clone());
        if top1.as_deref() == Some(case.expected_capability_id.as_str()) {
            top1_hits += 1;
        }
        let in_top3 = hits.iter().take(3).any(|hit| hit.capability_id == case.expected_capability_id);
        if in_top3 {
            top3_hits += 1;
        } else {
            misses.push(EvalMiss { request: case.request.clone(), expected_capability_id: case.expected_capability_id.clone(), got_top1: top1 });
        }
    }
    EvalReport { total: cases.len(), top1_hits, top3_hits, misses }
}
//#endregion 🔖️Eval

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::catalog::compile;
    use crate::fixtures;

    fn compiled() -> Catalog {
        compile(&fixtures::note_and_cad_source(), Locale::En, Terminology::Native).expect("compiles")
    }

    #[test]
    fn note_and_cad_fixtures_produce_zero_conformance_findings() {
        let catalog = compiled();
        let findings = check(&catalog);
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn note_and_cad_fixtures_have_non_empty_bilingual_labels() {
        let findings = check_bilingual_labels(&fixtures::note_and_cad_source());
        assert!(findings.is_empty(), "unexpected bilingual findings: {findings:?}");
    }

    #[test]
    fn mutation_without_writes_is_flagged() {
        let mut catalog = compiled();
        let mut capability = catalog.entries[0].clone();
        capability.kind = CapabilityKind::Mutation;
        capability.effects.writes.clear();
        catalog.entries[0] = capability;
        let findings = check(&catalog);
        assert!(findings.iter().any(|finding| finding.message.contains("declares no writes")));
    }

    #[test]
    fn unknown_scope_is_flagged() {
        let mut catalog = compiled();
        let mut capability = catalog.entries[0].clone();
        capability.policy.scopes.push(semio_framework::manifest::kernel::CapabilityId("not.a.real.scope".to_string()));
        catalog.entries[0] = capability;
        let findings = check(&catalog);
        assert!(findings.iter().any(|finding| finding.message.contains("unknown policy scope")));
    }

    #[test]
    fn bare_action_id_grammar_violation_is_flagged() {
        let mut catalog = compiled();
        let mut capability = catalog.entries[0].clone();
        capability.id = crate::catalog::CapabilityRef("bareActionId".to_string());
        catalog.entries[0] = capability;
        let findings = check(&catalog);
        assert!(findings.iter().any(|finding| finding.message.contains("must start with")));
    }

    #[test]
    fn eval_harness_measures_top1_and_top3_accuracy_deterministically() {
        let catalog = compiled();
        let cases = fixtures::eval_cases();
        let first = run_eval(&catalog, &cases);
        let second = run_eval(&catalog, &cases);
        assert_eq!(first, second, "eval must be fully deterministic");
        assert!(first.total >= 60);
        assert!(first.top1_accuracy() >= 0.0 && first.top1_accuracy() <= 1.0);
        assert!(first.top3_accuracy() >= first.top1_accuracy());
    }
}
//#endregion 🧪️Tests
