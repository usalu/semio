//! 🚪️ IO stdio.ifc.2x3 (2x3/🏢️cobie) — reuses the ✳️base subset's `binary`/`txt` raw-codec DAG
//! leaves. Registration flows through `🎹️composer::register`, not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Composer as Ifc2x3AnyComposer;
    use crate::standards::v2x3::subsets::cobie::io::check_cobie_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_COBIE: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cobie") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Ifc2x3CobieComposerComposition;

    impl ArtifactComposition for Ifc2x3CobieComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT_COBIE;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_COBIE, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = Ifc2x3AnyComposer::compose(sources)?;
            let checks = check_cobie_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("Basic FM Handover (COBie) conformance violated: {} hard issue(s) -- not stamping the cobie dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct Ifc2x3CobieValidator;

    impl SubsetValidator for Ifc2x3CobieValidator {
        const DIALECT: Dialect = DIALECT_COBIE;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_cobie_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.ifc.2x3.cobie.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Ifc2x3Cobie SubsetValidator: payload did not decode as an Ifc2x3Snapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<Ifc2x3CobieValidator>)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's `SubsetValidator` as a row of the artifact's
    /// [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        builder.subset_validators(std::slice::from_ref(validator_entry()))
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
    #[cfg(test)]
    use crate::standards::v2x3::subsets::base::schema::mutations::set_snapshot;
    use crate::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation, upsert_instance, Ifc2x3Mutation};
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::cobie::io::check_cobie_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;
    use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn stage_mutation_errors(diagnostics: &mut Vec<Diagnostic>, outcome: &protocol::MutationOutcome<Ifc2x3Diff>) {
        diagnostics.extend(outcome.messages().iter().filter(|message| message.level >= Severity::Error).map(|message| Diagnostic {
            code: message.code.clone(),
            severity: message.level,
            span: semio_framework_diagnostic::TextSpan::at(1, 1),
            message: if message.target.is_empty() { message.message.clone() } else { format!("{} at {}", message.message, message.target.join("/")) },
            expected: None,
            scope: semio_framework_diagnostic::FaultScope::default(),
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
            let outcome = apply_ifc2x3_mutation(&mut self.snapshot, &Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }));
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
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
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
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::base::io::{Ifc2x3Analyzer as Ifc2x3AnyAnalyzer, Ifc2x3Parts};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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
        snapshot.document.header.file_schema.iter().any(|v| v.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some(name))))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn view_definition_names(snapshot: &Ifc2x3Snapshot, view: &str) -> bool {
        snapshot.document.header.file_description.first().and_then(|v| v.as_list()).is_some_and(|items| items.iter().any(|item| item.as_str().is_some_and(|s| s.contains(view))))
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
            let named = args.get(2).and_then(|v| v.as_str()).is_some_and(|s| !s.trim().is_empty());
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

        let has_type = snapshot.document.instances.iter().any(|i| i.primary().is_some_and(|(name, _)| name.ends_with("TYPE")));
        let has_type_rel = snapshot.document.by_type("IFCRELDEFINESBYTYPE").next().is_some();
        if !has_type || !has_type_rel {
            out.push(soft(CODE_TYPE_ASSIGNMENT, "no real IFC*TYPE + IFCRELDEFINESBYTYPE pairing found -- COBie's Type sheet needs maintainable products related to a type".into()));
        }

        out
    }
    /// 🛡️ Checks the actual COBie fields with controlled borrowed relationships.
    pub fn check_cobie_conformance_controlled(snapshot:&Ifc2x3Snapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_os_kernel::sqlite_snapshot::ValueError>{
        use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
        use crate::standards::v2x3::subsets::base::io::sqlite::snapshot::{mvd_header,mvd_instances,mvd_nonempty_name,mvd_entity,MvdDiagnostics};
        use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
        let mut out=MvdDiagnostics::default();let(schema,view)=mvd_header(snapshot,"FMHandOverView",control)?;
        for(condition,code,message)in [(!schema,CODE_FILE_SCHEMA,"FILE_SCHEMA does not declare IFC2X3"),(!view,CODE_VIEW_DEFINITION,"FILE_DESCRIPTION's ViewDefinition tuple does not name FMHandOverView")]{if condition{out.emit(code,Severity::Error,format_args!("{message}"),control)?;}}
        for space in mvd_instances(snapshot,"IFCSPACE",control)?{let named=match mvd_entity(space,"IFCSPACE",control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"missing IFC space arguments"))?.get(2).and_then(|v|v.as_str()){Some(text)=>mvd_nonempty_name(text,control)?,None=>false};if !named{out.emit(CODE_SPACE_NAME,Severity::Warning,format_args!("IFCSPACE #{} has no non-empty Name -- COBie's Space sheet is keyed by name",space.id),control)?;}}
        let building=!mvd_instances(snapshot,"IFCBUILDING",control)?.is_empty();let storey=!mvd_instances(snapshot,"IFCBUILDINGSTOREY",control)?.is_empty();if !building||!storey{out.emit(CODE_BUILDING_STOREY,Severity::Warning,format_args!("missing {}{}{} -- COBie's Facility/Floor sheets need both",if !building{"IFCBUILDING"}else{""},if !building&&!storey{" and "}else{""},if !storey{"IFCBUILDINGSTOREY"}else{""}),control)?;}
        let mut has_type=false;control.check_rows(snapshot.document.instances.len())?;for(index,instance)in snapshot.document.instances.iter().enumerate(){has_type|=instance.primary().is_some_and(|(name,_)|name.ends_with("TYPE"));if index%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index,snapshot.document.instances.len())?;}}
        if !has_type||mvd_instances(snapshot,"IFCRELDEFINESBYTYPE",control)?.is_empty(){let message="no real IFC*TYPE + IFCRELDEFINESBYTYPE pairing found -- COBie's Type sheet needs maintainable products related to a type";out.emit(CODE_TYPE_ASSIGNMENT,Severity::Warning,format_args!("{message}"),control)?;}
        Ok(out.finish())
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
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

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
