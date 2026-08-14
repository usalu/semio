//! 🧬️ Ifc2x3Snapshot schema (2x3/✳️sav) — reuses the ✳️any subset's `Ifc2x3Snapshot` verbatim.
//! Structural Analysis View is a validation-gated dialect STAMP, not a new snapshot type.

pub use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::ifc::standards::v2x3::subsets::sav::schema::check_sav_conformance;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

    fn seeded_document() -> Part21Document {
        let header = Part21Header {
            file_description: vec![
                Part21Value::List(vec![Part21Value::Str("ViewDefinition [StructuralAnalysisView]".into())]),
                Part21Value::Str("2;1".into()),
            ],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        };
        let model = Part21Instance { id: 1, entities: vec![("IFCSTRUCTURALANALYSISMODEL".into(), vec![])] };
        Part21Document { header, instances: vec![model] }
    }

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct Ifc2x3SavBuilderConstruction {
        snapshot: Ifc2x3Snapshot,
    }

    impl Ifc2x3SavBuilderConstruction {
        pub fn new() -> Self {
            Self { snapshot: Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: seeded_document(), edm_preamble: None } }
        }

        /// ⚖️ Adds a load group (`IFCSTRUCTURALLOADGROUP`) instance.
        pub fn add_load_group(mut self, id: u64) -> Self {
            apply_ifc2x3_mutation(&mut self.snapshot, &Ifc2x3Mutation::UpsertInstance { instance: Part21Instance { id, entities: vec![("IFCSTRUCTURALLOADGROUP".into(), vec![])] } });
            self
        }
    }

    impl Default for Ifc2x3SavBuilderConstruction {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ArtifactBuilder for Ifc2x3SavBuilderConstruction {
        type Snapshot = Ifc2x3Snapshot;
        type Mutation = Ifc2x3Mutation;
        type Diff = Ifc2x3Diff;

        fn empty() -> Self {
            Self::new()
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_sav_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() { Ok(self.snapshot) } else { Err(hard) }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_builds_clean() {
            let snapshot = Ifc2x3SavBuilderConstruction::new().add_load_group(2).build().expect("conforming construction must build");
            assert_eq!(snapshot.document.instances.len(), 2);
        }

        #[test]
        fn removing_the_analysis_model_via_raw_mutate_still_fails_build() {
            let snapshot = Ifc2x3SavBuilderConstruction::new().build().unwrap();
            let (mutated, _diff) = Ifc2x3SavBuilderConstruction::from_snapshot(snapshot).mutate(Ifc2x3Mutation::RemoveInstance { id: 1 });
            let err = mutated.build().expect_err("removing the only analysis model must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::ifc::standards::v2x3::subsets::sav::schema::CODE_NO_ANALYSIS_MODEL));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::{Ifc2x3Analyzer as Ifc2x3AnyAnalyzer, Ifc2x3Parts};
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
    pub struct Ifc2x3SavAnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3SavAnalyzerAnalysis {
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
                edm_preamble: None,
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3SavBuilderFacets {
        construction: derived_construction::Ifc2x3SavBuilderConstruction,
        analysis: derived_analysis::Ifc2x3SavAnalyzerAnalysis,
        composition: super::io::derived_composition::Ifc2x3SavComposerComposition,
    }
    builder: Ifc2x3SavBuilder,
    analyzer: Ifc2x3SavAnalyzer,
    composer: Ifc2x3SavComposer,
);
//#endregion 🧬️DerivedArtifactFacets
