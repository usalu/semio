//! 🧐️ Ifc2x3CobieAnalyzer — real buildingSMART Basic FM Handover MVD (IFC2x3, carries COBie 2.4)
//! conformance checks against the retained `Ifc2x3Snapshot.document` Part-21 graph. Checks:
//! - HARD: `FILE_SCHEMA` declares `IFC2X3`.
//! - HARD: `FILE_DESCRIPTION`'s ViewDefinition tuple names `FMHandOverView`.
//! - SOFT: every `IFCSPACE` instance has a non-empty `Name` (attribute index 2, per `IfcSpace`'s
//!   attribute order) -- COBie's `Space` sheet is keyed by name.
//! - SOFT: at least one `IFCBUILDING` AND at least one `IFCBUILDINGSTOREY` instance.
//! - SOFT: a real type/instance-of-type pairing exists for maintainable products -- at least one
//!   `IFC*TYPE` instance (heuristic: any instance whose primary type name ends in `TYPE`) AND at
//!   least one `IFCRELDEFINESBYTYPE` relationship instance.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::ifc::standards::v2x3::subsets::any::analyzer::{Ifc2x3Analyzer as Ifc2x3AnyAnalyzer, Ifc2x3Parts};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cobie") };

//#region 🔖️Codes
pub const CODE_FILE_SCHEMA: &str = "stdio.ifc.2x3.cobie.file-schema";
pub const CODE_VIEW_DEFINITION: &str = "stdio.ifc.2x3.cobie.view-definition";
pub const CODE_SPACE_NAME: &str = "stdio.ifc.2x3.cobie.space-missing-name";
pub const CODE_BUILDING_STOREY: &str = "stdio.ifc.2x3.cobie.missing-building-or-storey";
pub const CODE_TYPE_ASSIGNMENT: &str = "stdio.ifc.2x3.cobie.missing-type-assignment";
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
/// 🛡️ Real Basic FM Handover (COBie) conformance checks. Shared source of truth for
/// `Ifc2x3CobieComposer::compose`, `Ifc2x3CobieBuilder::build`, and the registered
/// `SubsetValidator`.
pub fn check_cobie_conformance(snapshot: &Ifc2x3Snapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    if !declares_schema(snapshot, "IFC2X3") {
        out.push(hard(CODE_FILE_SCHEMA, "FILE_SCHEMA does not declare IFC2X3".into()));
    }
    if !view_definition_names(snapshot, "FMHandOverView") {
        out.push(hard(CODE_VIEW_DEFINITION, "FILE_DESCRIPTION's ViewDefinition tuple does not name FMHandOverView".into()));
    }

    for space in snapshot.document.by_type("IFCSPACE") {
        let args = space.entity("IFCSPACE").expect("matched by_type");
        let named = args.get(2).and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);
        if !named {
            out.push(soft(CODE_SPACE_NAME, format!("IFCSPACE #{} has no non-empty Name -- COBie's Space sheet is keyed by name", space.id)));
        }
    }

    let has_building = snapshot.document.by_type("IFCBUILDING").next().is_some();
    let has_storey = snapshot.document.by_type("IFCBUILDINGSTOREY").next().is_some();
    if !has_building || !has_storey {
        out.push(soft(CODE_BUILDING_STOREY, format!("missing {}{}{} -- COBie's Facility/Floor sheets need both",
            if !has_building { "IFCBUILDING" } else { "" },
            if !has_building && !has_storey { " and " } else { "" },
            if !has_storey { "IFCBUILDINGSTOREY" } else { "" })));
    }

    let has_type = snapshot.document.instances.iter().any(|i| i.primary().map(|(name, _)| name.ends_with("TYPE")).unwrap_or(false));
    let has_type_rel = snapshot.document.by_type("IFCRELDEFINESBYTYPE").next().is_some();
    if !has_type || !has_type_rel {
        out.push(soft(CODE_TYPE_ASSIGNMENT, "no real IFC*TYPE + IFCRELDEFINESBYTYPE pairing found -- COBie's Type sheet needs maintainable products related to a type".into()));
    }

    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
pub struct Ifc2x3CobieAnalyzer;

impl ArtifactAnalyzer for Ifc2x3CobieAnalyzer {
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
            let checks = check_cobie_conformance(snapshot);
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
        let space = Part21Instance {
            id: 1,
            entities: vec![(
                "IFCSPACE".into(),
                vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("Room 101".into())],
            )],
        };
        let building = Part21Instance { id: 2, entities: vec![("IFCBUILDING".into(), vec![])] };
        let storey = Part21Instance { id: 3, entities: vec![("IFCBUILDINGSTOREY".into(), vec![])] };
        let door_type = Part21Instance { id: 4, entities: vec![("IFCDOORTYPE".into(), vec![])] };
        let rel = Part21Instance { id: 5, entities: vec![("IFCRELDEFINESBYTYPE".into(), vec![])] };
        Ifc2x3Snapshot {
            schema: "stdio.ifc.2x3".into(),
            document: Part21Document { header: header("FMHandOverView"), instances: vec![space, building, storey, door_type, rel] },
        }
    }

    #[test]
    fn conforming_snapshot_has_no_hard_diagnostics() {
        let diagnostics = check_cobie_conformance(&conforming_snapshot());
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn wrong_view_definition_is_hard() {
        let mut snap = conforming_snapshot();
        snap.document.header = header("CoordinationView");
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn unnamed_space_is_soft() {
        let mut snap = conforming_snapshot();
        for (name, args) in snap.document.instances[0].entities.iter_mut() {
            if name == "IFCSPACE" {
                args[2] = Part21Value::Str("   ".into());
            }
        }
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SPACE_NAME && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn missing_storey_is_soft() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCBUILDINGSTOREY"));
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_BUILDING_STOREY && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn missing_type_assignment_is_soft() {
        let mut snap = conforming_snapshot();
        snap.document.instances.retain(|i| !i.is_type("IFCRELDEFINESBYTYPE"));
        let diagnostics = check_cobie_conformance(&snap);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TYPE_ASSIGNMENT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
//#endregion 🧪️Tests
