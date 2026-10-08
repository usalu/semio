//! 🚪️ IO stdio.svg (1.1/🔰️basic) — reuses the ✳️any subset's `xml` import/export leaves rather
//! than duplicating them (same `SvgSnapshot` type, same catalog DAG edge). Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` already
//! established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_1::subsets::base::schema::snapshot::{SvgAttributeValue, set_element_attr, SvgSnapshot};
    use crate::standards::v1_1::subsets::base::io::SvgComposer as SvgAnyComposer;
    use crate::standards::v1_1::subsets::basic::schema::conformance::check_svg_basic_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_BASIC: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("basic") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct SvgBasicComposerComposition;

    impl ArtifactComposition for SvgBasicComposerComposition {
        type Snapshot = SvgSnapshot;
        const WRITES: Dialect = DIALECT_BASIC;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_BASIC, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = SvgAnyComposer::compose(sources)?;
            let mut snapshot = inner.snapshot;
            if let Some(root) = snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some(SvgAttributeValue::Text("basic".into())));
                set_element_attr(root, "version", Some(SvgAttributeValue::Text("1.1".into())));
            }
            let checks = check_svg_basic_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("SVG Basic 1.1 conformance violated: {} hard issue(s) -- not stamping the basic dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `1.1/basic` -- see the module doc comment for how this
    /// relates to (and honestly differs from) the composer's own pre-serialization hard gate above.
    pub struct SvgBasicValidator;

    impl SubsetValidator for SvgBasicValidator {
        const DIALECT: Dialect = DIALECT_BASIC;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SvgSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_svg_basic_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.svg.basic.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "SVG Basic SubsetValidator: payload did not decode as an SvgSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SvgBasicValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the 1.1 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::standards::v1_1::subsets::base::io::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    use crate::standards::v1_1::subsets::base::schema::snapshot::{set_element_attr,SvgAttributeValue};
    use crate::standards::v1_1::subsets::basic::schema::conformance::check_svg_basic_conformance;
    use crate::standards::v1_1::subsets::basic::schema::mutations::{SvgBasicMutation};
    use crate::{SvgDiff, SvgSnapshot};
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct SvgBasicBuilderConstruction {
        snapshot: SvgSnapshot,
    }

    impl ArtifactBuilder for SvgBasicBuilderConstruction {
        type Snapshot = SvgSnapshot;
        type Mutation = SvgBasicMutation;
        type Diff = SvgDiff;

        fn empty() -> Self {
            Self::default()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
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

        /// 🛡️ The real construction gate: injects the profile metadata, then a hard Basic 1.1
        /// violation (however `self.snapshot` got here) fails `build()` -- soft diagnostics (nested
        /// `<svg>`) pass through silently here, matching the `🔬️tiny`/PDF/A pilots' own `build()` shape.
        fn build(mut self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            if let Some(root) = self.snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some(SvgAttributeValue::Text("basic".into())));
                set_element_attr(root, "version", Some(SvgAttributeValue::Text("1.1".into())));
            }
            let hard: Vec<Diagnostic> = check_svg_basic_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
    use crate::standards::v1_1::subsets::base::io::SvgAnalyzer as SvgAnyAnalyzer;
    pub use crate::standards::v1_1::subsets::base::io::SvgParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use crate::schema::snapshot::{SvgAttributeValue, SvgAttr, SvgNode};
    use std::collections::HashMap;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("basic") };

    use crate::standards::v1_1::subsets::basic::schema::conformance::*;
    /// 🚦️ Binds physical SQLite progress to pure owned profile validation.
    pub fn check_svg_basic_conformance_controlled(snapshot:&SvgSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_value::ValueError>{crate::standards::v1_1::subsets::basic::schema::conformance::check_svg_basic_conformance_with(snapshot,&mut |completed,total|control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,completed,total))}

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.svg` (1.1/🔰️basic): delegates the real parse to the ✳️any subset's analyzer
    /// (same `SvgSnapshot`), then folds real SVG Basic 1.1 conformance diagnostics on top. `sniff`
    /// delegates too -- see `SvgTinyAnalyzer`'s doc comment, same rationale.
    pub struct SvgBasicAnalyzerAnalysis;

    impl ArtifactAnalysis for SvgBasicAnalyzerAnalysis {
        type Parts = SvgParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            SvgAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = SvgAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_svg_basic_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = semio_framework_plugin::io::Confidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SvgBasicBuilderFacets {
        construction: SvgBasicBuilderConstruction,
        analysis: SvgBasicAnalyzerAnalysis,
        composition: super::io::derived_composition::SvgBasicComposerComposition,
    }
    builder: SvgBasicBuilder,
    analyzer: SvgBasicAnalyzer,
    composer: SvgBasicComposer,
);
