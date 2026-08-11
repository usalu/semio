//! 🧐️ Ifc2x3Cv20Analyzer — real buildingSMART Coordination View 2.0 MVD (IFC2x3, ISO/PAS
//! 16739:2005 schema) conformance checks against the retained `Ifc2x3Snapshot.document` Part-21
//! graph. Checks implemented as real, honest scans over the actual `Part21Instance`/`Part21Value`
//! data this snapshot genuinely retains (never fabricated against a field the engine doesn't
//! parse):
//! - HARD: `FILE_SCHEMA` declares `IFC2X3` -- independent re-check of the retained header, not
//!   just relying on `⚙️engine::decode_ifc2x3`'s own decode-time rejection (same "independent of
//!   decode" reasoning the PDF/A pilot's `/Encrypt` check documents).
//! - HARD: `FILE_DESCRIPTION`'s first tuple (the `ViewDefinition` string list) contains an entry
//!   naming `CoordinationView`.
//! - HARD: no `IFCSTRUCTURALANALYSISMODEL`/`IFCSTRUCTURALCURVEMEMBER`/`IFCSTRUCTURALLOADGROUP`
//!   instances -- CV2.0 is an architectural/coordination MVD, not a structural-analysis one (see
//!   `✳️sav` for the MVD that actually wants these).
//! - SOFT: exactly one `IFCPROJECT` instance, and it carries a non-`$` `UnitsInContext`
//!   (`IfcProject`'s 9th attribute, index 8, per the IFC2x3 EXPRESS attribute order).
//! - SOFT: every "geometry-bearing" product (a curated common `IfcProduct` subtype list -- the
//!   full `IfcProduct` hierarchy isn't schema-typed here, only the generic Part-21 graph is, so
//!   this is a deliberately-scoped, honestly-documented proxy) resolves its `ObjectPlacement`
//!   attribute (index 5, per `IfcProduct`'s attribute order) to a real `IFCLOCALPLACEMENT`
//!   instance.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::ifc::standards::v2x3::subsets::any::analyzer::{Ifc2x3Analyzer as Ifc2x3AnyAnalyzer, Ifc2x3Parts};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cv20") };

//#region 🔖️Codes
pub const CODE_FILE_SCHEMA: &str = "stdio.ifc.2x3.cv20.file-schema";
pub const CODE_VIEW_DEFINITION: &str = "stdio.ifc.2x3.cv20.view-definition";
pub const CODE_STRUCTURAL_ENTITY: &str = "stdio.ifc.2x3.cv20.structural-entity-present";
pub const CODE_PROJECT_UNITS: &str = "stdio.ifc.2x3.cv20.project-unit-assignment";
pub const CODE_PRODUCT_PLACEMENT: &str = "stdio.ifc.2x3.cv20.product-missing-placement";
//#endregion 🔖️Codes

//#region 🔖️Shared
/// 🚫️ Entity types explicitly forbidden by CV2.0's architectural/coordination scope.
const FORBIDDEN_STRUCTURAL_TYPES: &[&str] = &["IFCSTRUCTURALANALYSISMODEL", "IFCSTRUCTURALCURVEMEMBER", "IFCSTRUCTURALLOADGROUP"];

/// 🏗️ Curated common `IfcProduct` subtypes this honestly-scoped placement check applies to (see
/// module doc comment for why this is a proxy list, not the full `IfcProduct` hierarchy).
const GEOMETRY_BEARING_PRODUCT_TYPES: &[&str] = &[
    "IFCWALL", "IFCWALLSTANDARDCASE", "IFCDOOR", "IFCWINDOW", "IFCSLAB", "IFCBEAM", "IFCCOLUMN", "IFCROOF", "IFCSTAIR", "IFCBUILDINGELEMENTPROXY",
];

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
//#endregion 🔖️Shared

//#region 🔖️Conformance
/// 🛡️ Real ISO/PAS 16739:2005 (IFC2X3) Coordination View 2.0 conformance checks against one
/// already-decoded `Ifc2x3Snapshot`. Shared single source of truth: `Ifc2x3Cv20Composer::compose`
/// hard-gates on this pre-serialization, `Ifc2x3Cv20Builder::build` hard-gates on it too, and the
/// registered `SubsetValidator` re-runs it post-hoc against the wire payload.
pub fn check_cv20_conformance(snapshot: &Ifc2x3Snapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    if !declares_schema(snapshot, "IFC2X3") {
        out.push(hard(CODE_FILE_SCHEMA, "FILE_SCHEMA does not declare IFC2X3 -- Coordination View 2.0 is an IFC2x3 MVD".into()));
    }
    if !view_definition_names(snapshot, "CoordinationView") {
        out.push(hard(CODE_VIEW_DEFINITION, "FILE_DESCRIPTION's ViewDefinition tuple does not name CoordinationView".into()));
    }
    for ty in FORBIDDEN_STRUCTURAL_TYPES {
        for inst in snapshot.document.by_type(ty) {
            out.push(hard(CODE_STRUCTURAL_ENTITY, format!("instance #{} is {ty} -- CV2.0 is architectural/coordination scope, not structural analysis", inst.id)));
        }
    }

    let projects: Vec<_> = snapshot.document.by_type("IFCPROJECT").collect();
    if projects.len() != 1 {
        out.push(soft(CODE_PROJECT_UNITS, format!("expected exactly one IFCPROJECT, found {}", projects.len())));
    } else {
        let args = projects[0].entity("IFCPROJECT").expect("matched by_type");
        let has_units = args.get(8).map(|v| !v.is_unset()).unwrap_or(false);
        if !has_units {
            out.push(soft(CODE_PROJECT_UNITS, format!("IFCPROJECT #{} has no UnitsInContext (IfcUnitAssignment)", projects[0].id)));
        }
    }

    for ty in GEOMETRY_BEARING_PRODUCT_TYPES {
        for inst in snapshot.document.by_type(ty) {
            let args = inst.entity(ty).expect("matched by_type");
            let placed = args
                .get(5)
                .and_then(|v| v.as_ref_id())
                .and_then(|id| snapshot.document.instance(id))
                .map(|placement| placement.is_type("IFCLOCALPLACEMENT"))
                .unwrap_or(false);
            if !placed {
                out.push(soft(CODE_PRODUCT_PLACEMENT, format!("{ty} instance #{} does not resolve ObjectPlacement to an IFCLOCALPLACEMENT", inst.id)));
            }
        }
    }

    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.ifc.2x3` (2x3/✳️cv20): delegates the real parse to the ✳️any subset's
/// analyzer (same `Ifc2x3Snapshot`), then folds real CV2.0 conformance diagnostics on top.
pub struct Ifc2x3Cv20Analyzer;

impl ArtifactAnalyzer for Ifc2x3Cv20Analyzer {
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
            let checks = check_cv20_conformance(snapshot);
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
        let placement = Part21Instance { id: 10, entities: vec![("IFCLOCALPLACEMENT".into(), vec![])] };
        let project = Part21Instance {
            id: 1,
            entities: vec![(
                "IFCPROJECT".into(),
                vec![
                    Part21Value::Str("guid".into()),
                    Part21Value::Unset,
                    Part21Value::Str("Project".into()),
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Ref(20), // UnitsInContext
                ],
            )],
        };
        let wall = Part21Instance {
            id: 2,
            entities: vec![(
                "IFCWALL".into(),
                vec![
                    Part21Value::Str("guid2".into()),
                    Part21Value::Unset,
                    Part21Value::Str("Wall".into()),
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Ref(10), // ObjectPlacement
                ],
            )],
        };
        let units = Part21Instance { id: 20, entities: vec![("IFCUNITASSIGNMENT".into(), vec![])] };
        Ifc2x3Snapshot {
            schema: "stdio.ifc.2x3".into(),
            document: Part21Document { header: header("CoordinationView"), instances: vec![placement, project, wall, units] },
        }
    }

    #[test]
    fn conforming_snapshot_has_no_hard_diagnostics() {
        let diagnostics = check_cv20_conformance(&conforming_snapshot());
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn wrong_file_schema_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("IFC4".into())])];
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILE_SCHEMA && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_view_definition_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header = header("StructuralAnalysisView");
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn structural_entity_present_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.instances.push(Part21Instance { id: 99, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] });
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRUCTURAL_ENTITY && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_unit_assignment_is_soft() {
        let mut snap = conforming_snapshot();
        for (name, args) in snap.document.instances[1].entities.iter_mut() {
            if name == "IFCPROJECT" {
                args[8] = Part21Value::Unset;
            }
        }
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PROJECT_UNITS && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn product_without_placement_is_soft() {
        let mut snap = conforming_snapshot();
        for (name, args) in snap.document.instances[2].entities.iter_mut() {
            if name == "IFCWALL" {
                args[5] = Part21Value::Unset;
            }
        }
        let diagnostics = check_cv20_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRODUCT_PLACEMENT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
//#endregion 🧪️Tests
