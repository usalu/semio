//! 🚪️ IO stdio.ifc.2x3 (2x3/🧮️sav) — reuses the ✳️base subset's `binary`/`txt` raw-codec DAG
//! leaves. Registration flows through `🎹️composer::register`, not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Composer as Ifc2x3AnyComposer;
    use crate::standards::v2x3::subsets::sav::io::check_sav_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_SAV: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("sav") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Ifc2x3SavComposerComposition;

    impl ArtifactComposition for Ifc2x3SavComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT_SAV;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_SAV, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = Ifc2x3AnyComposer::compose(sources)?;
            let checks = check_sav_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("Structural Analysis View conformance violated: {} hard issue(s) -- not stamping the sav dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct Ifc2x3SavValidator;

    impl SubsetValidator for Ifc2x3SavValidator {
        const DIALECT: Dialect = DIALECT_SAV;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_sav_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.ifc.2x3.sav.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Ifc2x3Sav SubsetValidator: payload did not decode as an Ifc2x3Snapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<Ifc2x3SavValidator>)
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
    use crate::standards::v2x3::subsets::base::schema::mutations::remove_instance;
    use crate::standards::v2x3::subsets::base::schema::mutations::{upsert_instance, Ifc2x3Mutation};
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::sav::io::check_sav_conformance;
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
            file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [StructuralAnalysisView]".into())]), Part21Value::Str("2;1".into())],
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
        diagnostics: Vec<Diagnostic>,
    }

    impl Ifc2x3SavBuilderConstruction {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: seeded_document(), edm_preamble: None }, diagnostics: Vec::new() }
        }

        /// ⚖️ Adds a load group (`IFCSTRUCTURALLOADGROUP`) instance.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_load_group(mut self, id: u64) -> Self {
            let (next, outcome) = store::apply_outcome(&self.snapshot, protocol::Mutation::diff(&Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: Part21Instance { id, entities: vec![("IFCSTRUCTURALLOADGROUP".into(), vec![])] }, index: None }), &self.snapshot));
            self.snapshot = next;
            stage_mutation_errors(&mut self.diagnostics, &outcome);
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
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (next, diff) = store::apply_outcome(&self.snapshot, protocol::Mutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let Self { snapshot, mut diagnostics } = self;
            diagnostics.extend(check_sav_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)));
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
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("sav") };

    //#region 🔖️Codes
    pub const CODE_FILE_SCHEMA: &str = "stdio.ifc.2x3.sav.file-schema";
    pub const CODE_VIEW_DEFINITION: &str = "stdio.ifc.2x3.sav.view-definition";
    pub const CODE_NO_ANALYSIS_MODEL: &str = "stdio.ifc.2x3.sav.no-analysis-model";
    pub const CODE_NO_GROUP_ASSIGNMENT: &str = "stdio.ifc.2x3.sav.no-group-assignment";
    pub const CODE_NO_LOADS: &str = "stdio.ifc.2x3.sav.no-loads";
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
    /// 🛡️ Real Structural Analysis View conformance checks. Shared source of truth for
    /// `Ifc2x3SavComposer::compose`, `Ifc2x3SavBuilder::build`, and the registered `SubsetValidator`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    /// 🛡️ Checks Structural Analysis View without native encoding or unbounded scans.
    pub fn check_sav_conformance_controlled(snapshot:&Ifc2x3Snapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_os_kernel::sqlite_snapshot::ValueError>{

        use crate::standards::v2x3::subsets::base::io::sqlite::snapshot::{mvd_header,mvd_instances,MvdDiagnostics};
        let mut out=MvdDiagnostics::default();let(schema,view)=mvd_header(snapshot,"StructuralAnalysisView",control)?;
        for(condition,code,message,error)in [(!schema,CODE_FILE_SCHEMA,"FILE_SCHEMA does not declare IFC2X3",true),(!view,CODE_VIEW_DEFINITION,"FILE_DESCRIPTION's ViewDefinition tuple does not name StructuralAnalysisView",true),(mvd_instances(snapshot,"IFCSTRUCTURALANALYSISMODEL",control)?.is_empty(),CODE_NO_ANALYSIS_MODEL,"no IFCSTRUCTURALANALYSISMODEL instance -- a StructuralAnalysisView document must have at least one",true),(mvd_instances(snapshot,"IFCRELASSIGNSTOGROUP",control)?.is_empty(),CODE_NO_GROUP_ASSIGNMENT,"no IFCRELASSIGNSTOGROUP instance -- structural members/connections are not related to their analysis model",false),(mvd_instances(snapshot,"IFCSTRUCTURALLOADGROUP",control)?.is_empty(),CODE_NO_LOADS,"no IFCSTRUCTURALLOADGROUP instance -- no loads present",false)]{if condition{out.emit(code,if error{Severity::Error}else{Severity::Warning},format_args!("{message}"),control)?;}}
        Ok(out.finish())
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    pub struct Ifc2x3SavAnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3SavAnalyzerAnalysis {
        type Parts = Ifc2x3Parts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            Ifc2x3AnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = Ifc2x3AnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_sav_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = semio_framework_plugin::io::Confidence::Low;
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
    pub spec Ifc2x3SavBuilderFacets {
        construction: Ifc2x3SavBuilderConstruction,
        analysis: Ifc2x3SavAnalyzerAnalysis,
        composition: super::io::derived_composition::Ifc2x3SavComposerComposition,
    }
    builder: Ifc2x3SavBuilder,
    analyzer: Ifc2x3SavAnalyzer,
    composer: Ifc2x3SavComposer,
);
