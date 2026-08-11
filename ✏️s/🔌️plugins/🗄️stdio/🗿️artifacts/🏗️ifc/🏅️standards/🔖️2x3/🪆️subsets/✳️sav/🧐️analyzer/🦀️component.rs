//! 🧐️ Ifc2x3SavAnalyzer — real buildingSMART Structural Analysis View MVD (IFC2x3) conformance
//! checks against the retained `Ifc2x3Snapshot.document` Part-21 graph. Checks:
//! - HARD: `FILE_SCHEMA` declares `IFC2X3`.
//! - HARD: `FILE_DESCRIPTION`'s ViewDefinition tuple names `StructuralAnalysisView`.
//! - HARD: at least one `IFCSTRUCTURALANALYSISMODEL` instance -- a document claiming this MVD
//!   with no analysis model at all has nothing to validate structurally.
//! - SOFT: structural members are related to their analysis model via an
//!   `IFCRELASSIGNSTOGROUP` instance (real scan: any such instance present at all, since the full
//!   `RelatingGroup`/`RelatedObjects` graph-walk is out of this honestly-scoped check).
//! - SOFT: at least one `IFCSTRUCTURALLOADGROUP` instance (loads present).

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::ifc::standards::v2x3::subsets::any::analyzer::{Ifc2x3Analyzer as Ifc2x3AnyAnalyzer, Ifc2x3Parts};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("sav") };

//#region 🔖️Codes
pub const CODE_FILE_SCHEMA: &str = "stdio.ifc.2x3.sav.file-schema";
pub const CODE_VIEW_DEFINITION: &str = "stdio.ifc.2x3.sav.view-definition";
pub const CODE_NO_ANALYSIS_MODEL: &str = "stdio.ifc.2x3.sav.no-analysis-model";
pub const CODE_NO_GROUP_ASSIGNMENT: &str = "stdio.ifc.2x3.sav.no-group-assignment";
pub const CODE_NO_LOADS: &str = "stdio.ifc.2x3.sav.no-loads";
//#endregion 🔖️Codes

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}
fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn declares_schema(snapshot: &Ifc2x3Snapshot, name: &str) -> bool {
    snapshot.document.header.file_schema.iter().any(|v| v.as_list().map(|items| items.iter().any(|item| item.as_str() == Some(name))).unwrap_or(false))
}
fn view_definition_names(snapshot: &Ifc2x3Snapshot, view: &str) -> bool {
    snapshot
        .document
        .header
        .file_description
        .first()
        .and_then(|v| v.as_list())
        .map(|items| items.iter().any(|item| item.as_str().map(|s| s.contains(view)).unwrap_or(false)))
        .unwrap_or(false)
}

//#region 🔖️Conformance
/// 🛡️ Real Structural Analysis View conformance checks. Shared source of truth for
/// `Ifc2x3SavComposer::compose`, `Ifc2x3SavBuilder::build`, and the registered `SubsetValidator`.
pub fn check_sav_conformance(snapshot: &Ifc2x3Snapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    if !declares_schema(snapshot, "IFC2X3") {
        out.push(hard(CODE_FILE_SCHEMA, "FILE_SCHEMA does not declare IFC2X3".into()));
    }
    if !view_definition_names(snapshot, "StructuralAnalysisView") {
        out.push(hard(CODE_VIEW_DEFINITION, "FILE_DESCRIPTION's ViewDefinition tuple does not name StructuralAnalysisView".into()));
    }
    if snapshot.document.by_type("IFCSTRUCTURALANALYSISMODEL").next().is_none() {
        out.push(hard(CODE_NO_ANALYSIS_MODEL, "no IFCSTRUCTURALANALYSISMODEL instance -- a StructuralAnalysisView document must have at least one".into()));
    }
    if snapshot.document.by_type("IFCRELASSIGNSTOGROUP").next().is_none() {
        out.push(soft(CODE_NO_GROUP_ASSIGNMENT, "no IFCRELASSIGNSTOGROUP instance -- structural members/connections are not related to their analysis model".into()));
    }
    if snapshot.document.by_type("IFCSTRUCTURALLOADGROUP").next().is_none() {
        out.push(soft(CODE_NO_LOADS, "no IFCSTRUCTURALLOADGROUP instance -- no loads present".into()));
    }
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
pub struct Ifc2x3SavAnalyzer;

impl ArtifactAnalyzer for Ifc2x3SavAnalyzer {
    type Parts = Ifc2x3Parts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        Ifc2x3AnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = Ifc2x3AnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        let mut confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            let checks = check_sav_conformance(snapshot);
            if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                confidence = IoConfidence::Low;
            }
            diagnostics.extend(checks);
        }
        Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

    fn header(view: &str) -> Part21Header {
        Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str(format!("ViewDefinition [{view}]"))]), Part21Value::Str("2;1".into())],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        }
    }

    fn conforming_snapshot() -> Ifc2x3Snapshot {
        let model = Part21Instance { id: 1, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] };
        let group = Part21Instance { id: 2, entities: vec![("IFCRELASSIGNSTOGROUP".into(), vec![])] };
        let loads = Part21Instance { id: 3, entities: vec![("IFCSTRUCTURALLOADGROUP".into(), vec![])] };
        Ifc2x3Snapshot {
            schema: "stdio.ifc.2x3".into(),
            document: Part21Document { header: header("StructuralAnalysisView"), instances: vec![model, group, loads] },
        }
    }

    #[test]
    fn conforming_snapshot_has_no_hard_diagnostics() {
        let diagnostics = check_sav_conformance(&conforming_snapshot());
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_analysis_model_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCSTRUCTURALANALYSISMODEL"));
        let diagnostics = check_sav_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_ANALYSIS_MODEL && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn wrong_view_definition_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header = header("CoordinationView");
        let diagnostics = check_sav_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_loads_and_group_assignment_are_soft() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCRELASSIGNSTOGROUP") && !i.is_type("IFCSTRUCTURALLOADGROUP"));
        let diagnostics = check_sav_conformance(&snap);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_GROUP_ASSIGNMENT));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_LOADS));
    }
}
//#endregion 🧪️Tests
