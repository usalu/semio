//! 🧐️ StepCc5Analyzer (ap214/✳️cc5) — real ISO 10303-214 CC5 (faceted B-Rep) conformance checks against the
//! retained lossless `Part21Document` graph (`StepSnapshot::to_part21_document()`). Checks implemented as
//! real, honest scans over `by_type`/`instances` (never fabricated against an unmodeled field),
//! shared with the other five `✳️ccN` subsets via `⚙️engine::ladder`:
//! - HARD: `FILE_SCHEMA` does not declare `AUTOMOTIVE_DESIGN` — every AP214 file must.
//! - HARD: any `*_SHAPE_REPRESENTATION` instance whose ladder rung exceeds 5 (see
//!   `⚙️engine::ladder::ladder_rung_of` for the classification and rationale; cc5 allows rungs
//!   up to 5).
//! - SOFT: no `PRODUCT` + `PRODUCT_DEFINITION_FORMATION` + `PRODUCT_DEFINITION` chain found —
//!   real AP214 data normally carries one.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::step::standards::v_ap214::subsets::any::analyzer::StepAnalyzer as StepAnyAnalyzer;
pub use crate::artifacts::step::standards::v_ap214::subsets::any::analyzer::StepParts;
use crate::artifacts::step::standards::v_ap214::subsets::any::schema::snapshot::StepSnapshot;
use crate::artifacts::step::standards::v_ap214::engine::ladder::{file_schema_contains, has_product_definition_chain, ladder_violations};

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc5") };

/// 🔢️ Maximum ladder rung cc5 permits (see `⚙️engine::ladder::ladder_rung_of`).
pub const MAX_RUNG: u8 = 5;

//#region 🔖️Conformance
pub const CODE_FILE_SCHEMA: &str = "stdio.step.cc5.file-schema-automotive-design";
pub const CODE_PRODUCT_CHAIN: &str = "stdio.step.cc5.product-definition-chain";
pub const CODE_LADDER: &str = "stdio.step.cc5.representation-above-rung";

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real ISO 10303-214 CC5 (faceted B-Rep) conformance checks against one already-decoded `StepSnapshot`. Shared
/// single source of truth: `StepCc5Composer::compose` hard-gates on this (pre-serialization,
/// authoritative), `StepCc5Builder::build` hard-gates on this too, and the registered
/// `SubsetValidator` (from `🎹️composer::register`) re-runs it post-hoc against the wire payload
/// for the D5 validate-on-build hook.
pub fn check_cc5_conformance(snapshot: &StepSnapshot) -> Vec<Diagnostic> {
    let doc = snapshot.to_part21_document();
    let mut out = Vec::new();
    if !file_schema_contains(&doc, "AUTOMOTIVE_DESIGN") {
        out.push(hard(CODE_FILE_SCHEMA, "FILE_SCHEMA does not declare AUTOMOTIVE_DESIGN -- ISO 10303-214 requires the AP214 EXPRESS schema".into()));
    }
    for (id, type_name, rung) in ladder_violations(&doc, MAX_RUNG) {
        out.push(hard(
            CODE_LADDER,
            format!("instance #{id} is a {type_name} (ladder rung {rung}) -- exceeds cc5's max rung 5"),
        ));
    }
    if !has_product_definition_chain(&doc) {
        out.push(soft(CODE_PRODUCT_CHAIN, "no PRODUCT + PRODUCT_DEFINITION_FORMATION + PRODUCT_DEFINITION chain found -- real AP214 data normally carries one".into()));
    }
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.step` (ap214/✳️cc5): delegates the real parse to the ✳️any subset's
/// analyzer (same `StepSnapshot`), then folds real ISO 10303-214 CC5 (faceted B-Rep) conformance diagnostics on top.
/// `sniff` also delegates -- subset-level sniff is "is this recognizable as a STEP file at all",
/// the same probe every ap214 dialect shares; conformance is a separate, heavier question
/// answered by `analyze`/`check_cc5_conformance`, not by `sniff`.
pub struct StepCc5Analyzer;

impl ArtifactAnalyzer for StepCc5Analyzer {
    type Parts = StepParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        StepAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = StepAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        let mut confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            let checks = check_cc5_conformance(snapshot);
            if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                confidence = IoConfidence::Low;
            }
            diagnostics.extend(checks);
        }
        Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

    fn base_doc() -> Part21Document {
        Part21Document {
            header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
            instances: vec![
                Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
            ],
        }
    }

    #[test]
    fn conforming_document_reports_no_diagnostics() {
        let snapshot = StepSnapshot::from_part21_document(base_doc());
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[test]
    fn missing_file_schema_is_hard() {
        let mut doc = base_doc();
        doc.header.file_schema = vec![];
        let snapshot = StepSnapshot::from_part21_document(doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILE_SCHEMA && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_product_chain_is_soft() {
        let mut doc = base_doc();
        doc.instances.clear();
        let snapshot = StepSnapshot::from_part21_document(doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRODUCT_CHAIN && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn representation_at_max_rung_is_clean() {
        let mut doc = base_doc();
        doc.instances.push(Part21Instance { id: 4, entities: vec![("FACETED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
        let snapshot = StepSnapshot::from_part21_document(doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_LADDER), "got {diagnostics:?}");
    }

    #[test]
    fn representation_above_max_rung_is_hard() {
        let mut doc = base_doc();
        doc.instances.push(Part21Instance { id: 4, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
        let snapshot = StepSnapshot::from_part21_document(doc);
        let diagnostics = check_cc5_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LADDER && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
