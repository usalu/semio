//! 🚪️ IO stdio.ifc.2x3 (2x3/🤝️cv20) — reuses the ✳️base subset's `binary`/`txt` raw-codec DAG
//! leaves rather than duplicating them (same `Ifc2x3Snapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Composer as Ifc2x3AnyComposer;
    use crate::standards::v2x3::subsets::cv20::schema::check_cv20_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_CV20: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cv20") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Ifc2x3Cv20ComposerComposition;

    impl ArtifactComposition for Ifc2x3Cv20ComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT_CV20;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_CV20, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = Ifc2x3AnyComposer::compose(sources)?;
            let checks = check_cv20_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("Coordination View 2.0 conformance violated: {} hard issue(s) -- not stamping the cv20 dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct Ifc2x3Cv20Validator;

    impl SubsetValidator for Ifc2x3Cv20Validator {
        const DIALECT: Dialect = DIALECT_CV20;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_cv20_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.ifc.2x3.cv20.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Ifc2x3Cv20 SubsetValidator: payload did not decode as an Ifc2x3Snapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<Ifc2x3Cv20Validator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator`. Called from the `2x3` standard's own
    /// `⚙️engine::register()`. The `ComposerEntry` itself is registered separately via the standard's
    /// own `composer::entries()` aggregation.
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
    use crate::standards::v2x3::subsets::cv20::schema::check_cv20_conformance;
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

    //#region 🔖️Seed
    const PLACEMENT_ID: u64 = 10;
    const UNITS_ID: u64 = 20;
    const PROJECT_ID: u64 = 1;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn seeded_document() -> Part21Document {
        let header = Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView]".into())]), Part21Value::Str("2;1".into())],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        };
        let placement = Part21Instance { id: PLACEMENT_ID, entities: vec![("IFCLOCALPLACEMENT".into(), vec![])] };
        let units = Part21Instance { id: UNITS_ID, entities: vec![("IFCUNITASSIGNMENT".into(), vec![])] };
        let project = Part21Instance {
            id: PROJECT_ID,
            entities: vec![(
                "IFCPROJECT".into(),
                vec![
                    Part21Value::Str("0000000000000000000000".into()),
                    Part21Value::Unset,
                    Part21Value::Str("Project".into()),
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Unset,
                    Part21Value::Ref(UNITS_ID),
                ],
            )],
        };
        Part21Document { header, instances: vec![placement, units, project] }
    }
    //#endregion 🔖️Seed

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct Ifc2x3Cv20BuilderConstruction {
        snapshot: Ifc2x3Snapshot,
        diagnostics: Vec<Diagnostic>,
    }

    impl Ifc2x3Cv20BuilderConstruction {
        /// ➕ The recommended entry point: always produces a document with `IFC2X3`/`CoordinationView`
        /// header and a real project+units pair.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: seeded_document(), edm_preamble: None }, diagnostics: Vec::new() }
        }

        /// 🧱️ Adds a product instance of `type_name` (must be one of the geometry-bearing product
        /// types this MVD checks), always wiring `ObjectPlacement` (attribute index 5) to the seeded
        /// `IFCLOCALPLACEMENT`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_product(mut self, id: u64, type_name: &str, name: &str) -> Self {
            let instance = Part21Instance {
                id,
                entities: vec![(type_name.to_string(), vec![Part21Value::Str(format!("guid-{id}")), Part21Value::Unset, Part21Value::Str(name.to_string()), Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(PLACEMENT_ID)])],
            };
            let outcome = apply_ifc2x3_mutation(&mut self.snapshot, &Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }));
            stage_mutation_errors(&mut self.diagnostics, &outcome);
            self
        }
    }

    impl Default for Ifc2x3Cv20BuilderConstruction {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ArtifactBuilder for Ifc2x3Cv20BuilderConstruction {
        type Snapshot = Ifc2x3Snapshot;
        type Mutation = Ifc2x3Mutation;
        type Diff = Ifc2x3Diff;

        /// ⚠️ `ArtifactBuilder::empty()` is mandated no-arg by the SDK trait -- falls back to
        /// `Ifc2x3Cv20BuilderConstruction::new()`'s seeded document rather than a truly empty (non-conforming)
        /// one, since `build()` requires conformance regardless.
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
            let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard CV2.0 violation
        /// fails `build()`; soft diagnostics pass through as advisory (the `Err` path is not taken).
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let Self { snapshot, mut diagnostics } = self;
            diagnostics.extend(check_cv20_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)));
            if diagnostics.is_empty() {
                Ok(snapshot)
            } else {
                Err(diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::base::schema::{Ifc2x3Analyzer as Ifc2x3AnyAnalyzer, Ifc2x3Parts};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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
    /// 🚫️ Entity types explicitly forbidden by CV2.0's architectural/coordination scope. `pub`
    /// because `../🧬️mutations/🦀️.rs`'s `SetStructuralEntity` is guarded by exactly this
    /// list -- the mutation vocabulary and the conformance check must never disagree about which
    /// types the MVD excludes.
    pub const FORBIDDEN_STRUCTURAL_TYPES: &[&str] = &["IFCSTRUCTURALANALYSISMODEL", "IFCSTRUCTURALCURVEMEMBER", "IFCSTRUCTURALLOADGROUP"];

    /// 🏗️ Curated common `IfcProduct` subtypes this honestly-scoped placement check applies to (see
    /// module doc comment for why this is a proxy list, not the full `IfcProduct` hierarchy).
    pub const GEOMETRY_BEARING_PRODUCT_TYPES: &[&str] = &["IFCWALL", "IFCWALLSTANDARDCASE", "IFCDOOR", "IFCWINDOW", "IFCSLAB", "IFCBEAM", "IFCCOLUMN", "IFCROOF", "IFCSTAIR", "IFCBUILDINGELEMENTPROXY"];

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
    //#endregion 🔖️Shared

    //#region 🔖️Conformance
    /// 🛡️ Real ISO/PAS 16739:2005 (IFC2X3) Coordination View 2.0 conformance checks against one
    /// already-decoded `Ifc2x3Snapshot`. Shared single source of truth: `Ifc2x3Cv20Composer::compose`
    /// hard-gates on this pre-serialization, `Ifc2x3Cv20Builder::build` hard-gates on it too, and the
    /// registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
            let has_units = args.get(8).is_some_and(|v| !v.is_unset());
            if !has_units {
                out.push(soft(CODE_PROJECT_UNITS, format!("IFCPROJECT #{} has no UnitsInContext (IfcUnitAssignment)", projects[0].id)));
            }
        }

        for ty in GEOMETRY_BEARING_PRODUCT_TYPES {
            for inst in snapshot.document.by_type(ty) {
                let args = inst.entity(ty).expect("matched by_type");
                let placed = args.get(5).and_then(|v| v.as_ref_id()).and_then(|id| snapshot.document.instance(id)).is_some_and(|placement| placement.is_type("IFCLOCALPLACEMENT"));
                if !placed {
                    out.push(soft(CODE_PRODUCT_PLACEMENT, format!("{ty} instance #{} does not resolve ObjectPlacement to an IFCLOCALPLACEMENT", inst.id)));
                }
            }
        }

        out
    }
    /// 🛡️ Checks the exact Coordination View rules with bounded borrowed scans.
    pub fn check_cv20_conformance_controlled(snapshot:&Ifc2x3Snapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_os_kernel::sqlite_snapshot::ValueError>{
        use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
        use crate::standards::v2x3::subsets::base::schema::snapshot::sqlite_snapshot::{mvd_header,mvd_instances,mvd_identity_index,mvd_entity,MvdDiagnostics};
        let mut out=MvdDiagnostics::default();let(schema,view)=mvd_header(snapshot,"CoordinationView",control)?;
        if !schema{let message="FILE_SCHEMA does not declare IFC2X3 -- Coordination View 2.0 is an IFC2x3 MVD";out.emit(CODE_FILE_SCHEMA,Severity::Error,format_args!("{message}"),control)?;}
        if !view{let message="FILE_DESCRIPTION's ViewDefinition tuple does not name CoordinationView";out.emit(CODE_VIEW_DEFINITION,Severity::Error,format_args!("{message}"),control)?;}
        for ty in FORBIDDEN_STRUCTURAL_TYPES{for inst in mvd_instances(snapshot,ty,control)?{out.emit(CODE_STRUCTURAL_ENTITY,Severity::Error,format_args!("instance #{} is {ty} -- CV2.0 is architectural/coordination scope, not structural analysis",inst.id),control)?;}}
        let projects=mvd_instances(snapshot,"IFCPROJECT",control)?;
        if projects.len()!=1{out.emit(CODE_PROJECT_UNITS,Severity::Warning,format_args!("expected exactly one IFCPROJECT, found {}",projects.len()),control)?;}else if !mvd_entity(projects[0],"IFCPROJECT",control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"missing IFC project arguments"))?.get(8).is_some_and(|v|!v.is_unset()){out.emit(CODE_PROJECT_UNITS,Severity::Warning,format_args!("IFCPROJECT #{} has no UnitsInContext (IfcUnitAssignment)",projects[0].id),control)?;}
        let identities=mvd_identity_index(snapshot,control)?;
        for ty in GEOMETRY_BEARING_PRODUCT_TYPES{for inst in mvd_instances(snapshot,ty,control)?{let placement=mvd_entity(inst,ty,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"missing IFC product arguments"))?.get(5).and_then(|v|v.as_ref_id()).and_then(|id|identities.resolve(id));let placed=match placement{Some(instance)=>mvd_entity(instance,"IFCLOCALPLACEMENT",control)?.is_some(),None=>false};if !placed{out.emit(CODE_PRODUCT_PLACEMENT,Severity::Warning,format_args!("{ty} instance #{} does not resolve ObjectPlacement to an IFCLOCALPLACEMENT",inst.id),control)?;}}}
        Ok(out.finish())
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.ifc.2x3` (2x3/🤝️cv20): delegates the real parse to the ✳️base subset's
    /// analyzer (same `Ifc2x3Snapshot`), then folds real CV2.0 conformance diagnostics on top.
    pub struct Ifc2x3Cv20AnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3Cv20AnalyzerAnalysis {
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3Cv20BuilderFacets {
        construction: Ifc2x3Cv20BuilderConstruction,
        analysis: Ifc2x3Cv20AnalyzerAnalysis,
        composition: super::io::derived_composition::Ifc2x3Cv20ComposerComposition,
    }
    builder: Ifc2x3Cv20Builder,
    analyzer: Ifc2x3Cv20Analyzer,
    composer: Ifc2x3Cv20Composer,
);
