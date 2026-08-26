//! 🧬️ Ifc2x3Snapshot schema (2x3/✳️cobie) — reuses the ✳️any subset's `Ifc2x3Snapshot` verbatim.
//! Basic FM Handover (carries COBie 2.4) is a validation-gated dialect STAMP, not a new type.

pub use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::*;

//#region 🧬️Mutations
/// 🧬️ This subset's OWN mutation vocabulary — one kind per Basic FM Handover conformance rule, not a copy of
/// the `✳️any` subset's generic Part-21 graph editing. The module re-exports `✳️any`'s
/// `Ifc2x3Mutation`/`apply_ifc2x3_mutation` as well, since this explicit declaration shadows the
/// glob re-export those names used to arrive through.
#[path = "🧬️mutations/🦀️component.rs"]
pub mod mutations;
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::check_cobie_conformance;
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn stage_mutation_errors(diagnostics: &mut Vec<Diagnostic>, outcome: &protocol::MutationOutcome<Ifc2x3Diff>) {
        diagnostics.extend(outcome.messages().iter().filter(|message| message.level >= Severity::Error).map(|message| Diagnostic {
            code: message.code.clone(),
            severity: message.level,
            span: dsl::TextSpan::at(1, 1),
            message: if message.target.is_empty() { message.message.clone() } else { format!("{} at {}", message.message, message.target.join("/")) },
            expected: None,
            scope: dsl::FaultScope::default(),
        }));
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn seeded_document() -> Part21Document {
        let header = Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [FMHandOverView]".into())]), Part21Value::Str("2;1".into())],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        };
        let building = Part21Instance { id: 1, entities: vec![("IFCBUILDING".into(), vec![])] };
        let storey = Part21Instance { id: 2, entities: vec![("IFCBUILDINGSTOREY".into(), vec![])] };
        let door_type = Part21Instance { id: 3, entities: vec![("IFCDOORTYPE".into(), vec![])] };
        let rel = Part21Instance { id: 4, entities: vec![("IFCRELDEFINESBYTYPE".into(), vec![])] };
        Part21Document { header, instances: vec![building, storey, door_type, rel] }
    }

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct Ifc2x3CobieBuilderConstruction {
        snapshot: Ifc2x3Snapshot,
        next_id: u64,
        diagnostics: Vec<Diagnostic>,
    }

    impl Ifc2x3CobieBuilderConstruction {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: seeded_document(), edm_preamble: None }, next_id: 100, diagnostics: Vec::new() }
        }

        /// 🏷️ Adds a named `IFCSPACE` (COBie's `Space` sheet row).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_space(mut self, name: &str) -> Self {
            let id = self.next_id;
            self.next_id += 1;
            let instance = Part21Instance { id, entities: vec![("IFCSPACE".into(), vec![Part21Value::Str(format!("guid-{id}")), Part21Value::Unset, Part21Value::Str(name.to_string())])] };
            let outcome = apply_ifc2x3_mutation(&mut self.snapshot, &Ifc2x3Mutation::UpsertInstance { instance });
            stage_mutation_errors(&mut self.diagnostics, &outcome);
            self
        }
    }

    impl Default for Ifc2x3CobieBuilderConstruction {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ArtifactBuilder for Ifc2x3CobieBuilderConstruction {
        type Snapshot = Ifc2x3Snapshot;
        type Mutation = Ifc2x3Mutation;
        type Diff = Ifc2x3Diff;

        fn empty() -> Self {
            Self::new()
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, next_id: 100, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let Self { snapshot, mut diagnostics, .. } = self;
            diagnostics.extend(check_cobie_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)));
            if diagnostics.is_empty() {
                Ok(snapshot)
            } else {
                Err(diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn new_builds_clean() {
            let snapshot = Ifc2x3CobieBuilderConstruction::new().add_space("Room 101").build().expect("conforming construction must build");
            assert_eq!(snapshot.document.instances.len(), 5);
        }

        #[semio_framework_async_macros::async_test]
        async fn wrong_schema_via_raw_mutate_still_fails_build() {
            let snapshot = Ifc2x3CobieBuilderConstruction::new().build().unwrap();
            let mut bad = snapshot.clone();
            bad.document.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("IFC4".into())])];
            let (mutated, _diff) = Ifc2x3CobieBuilderConstruction::from_snapshot(Ifc2x3Snapshot::default()).mutate(Ifc2x3Mutation::SetSnapshot { snapshot: bad });
            let err = mutated.build().expect_err("a non-IFC2X3 FILE_SCHEMA must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::CODE_FILE_SCHEMA));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::{Ifc2x3Analyzer as Ifc2x3AnyAnalyzer, Ifc2x3Parts};
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cobie") };

    //#region 🔖️Codes
    pub const CODE_FILE_SCHEMA: &str = "stdio.ifc.2x3.cobie.file-schema";
    pub const CODE_VIEW_DEFINITION: &str = "stdio.ifc.2x3.cobie.view-definition";
    pub const CODE_SPACE_NAME: &str = "stdio.ifc.2x3.cobie.space-missing-name";
    pub const CODE_BUILDING_STOREY: &str = "stdio.ifc.2x3.cobie.missing-building-or-storey";
    pub const CODE_TYPE_ASSIGNMENT: &str = "stdio.ifc.2x3.cobie.missing-type-assignment";
    //#endregion 🔖️Codes

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn declares_schema(snapshot: &Ifc2x3Snapshot, name: &str) -> bool {
        snapshot.document.header.file_schema.iter().any(|v| v.as_list().map(|items| items.iter().any(|item| item.as_str() == Some(name))).unwrap_or(false))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn view_definition_names(snapshot: &Ifc2x3Snapshot, view: &str) -> bool {
        snapshot.document.header.file_description.first().and_then(|v| v.as_list()).map(|items| items.iter().any(|item| item.as_str().map(|s| s.contains(view)).unwrap_or(false))).unwrap_or(false)
    }

    //#region 🔖️Conformance
    /// 🛡️ Real Basic FM Handover (COBie) conformance checks. Shared source of truth for
    /// `Ifc2x3CobieComposer::compose`, `Ifc2x3CobieBuilder::build`, and the registered
    /// `SubsetValidator`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
            out.push(soft(
                CODE_BUILDING_STOREY,
                format!("missing {}{}{} -- COBie's Facility/Floor sheets need both", if !has_building { "IFCBUILDING" } else { "" }, if !has_building && !has_storey { " and " } else { "" }, if !has_storey { "IFCBUILDINGSTOREY" } else { "" }),
            ));
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
    pub struct Ifc2x3CobieAnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3CobieAnalyzerAnalysis {
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

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn header(view: &str) -> Part21Header {
            Part21Header {
                file_description: vec![Part21Value::List(vec![Part21Value::Str(format!("ViewDefinition [{view}]"))]), Part21Value::Str("2;1".into())],
                file_name: vec![],
                file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
            }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn conforming_snapshot() -> Ifc2x3Snapshot {
            let space = Part21Instance { id: 1, entities: vec![("IFCSPACE".into(), vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("Room 101".into())])] };
            let building = Part21Instance { id: 2, entities: vec![("IFCBUILDING".into(), vec![])] };
            let storey = Part21Instance { id: 3, entities: vec![("IFCBUILDINGSTOREY".into(), vec![])] };
            let door_type = Part21Instance { id: 4, entities: vec![("IFCDOORTYPE".into(), vec![])] };
            let rel = Part21Instance { id: 5, entities: vec![("IFCRELDEFINESBYTYPE".into(), vec![])] };
            Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header: header("FMHandOverView"), instances: vec![space, building, storey, door_type, rel] }, edm_preamble: None }
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_snapshot_has_no_hard_diagnostics() {
            let diagnostics = check_cobie_conformance(&conforming_snapshot());
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn wrong_view_definition_is_hard() {
            let mut snap = conforming_snapshot();
            snap.document.header = header("CoordinationView");
            let diagnostics = check_cobie_conformance(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn unnamed_space_is_soft() {
            let mut snap = conforming_snapshot();
            for (name, args) in snap.document.instances[0].entities.iter_mut() {
                if name == "IFCSPACE" {
                    args[2] = Part21Value::Str("   ".into());
                }
            }
            let diagnostics = check_cobie_conformance(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SPACE_NAME && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_storey_is_soft() {
            let mut snap = conforming_snapshot();
            snap.document.instances.retain(|i| !i.is_type("IFCBUILDINGSTOREY"));
            let diagnostics = check_cobie_conformance(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_BUILDING_STOREY && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_type_assignment_is_soft() {
            let mut snap = conforming_snapshot();
            snap.document.instances.retain(|i| !i.is_type("IFCRELDEFINESBYTYPE"));
            let diagnostics = check_cobie_conformance(&snap);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TYPE_ASSIGNMENT && d.severity == Severity::Warning), "got {diagnostics:?}");
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3CobieBuilderFacets {
        construction: Ifc2x3CobieBuilderConstruction,
        analysis: Ifc2x3CobieAnalyzerAnalysis,
        composition: super::io::derived_composition::Ifc2x3CobieComposerComposition,
    }
    builder: Ifc2x3CobieBuilder,
    analyzer: Ifc2x3CobieAnalyzer,
    composer: Ifc2x3CobieComposer,
);
//#endregion 🧬️DerivedArtifactFacets
