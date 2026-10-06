//! 🚪️ IO stdio.jpg (jfif-1.01/🧱️baseline) — reuses the 🧾️document subset's `binary` raw-codec DAG
//! leaf rather than duplicating it (same `JpgSnapshot` type, same catalog DAG edge). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `🧾️document/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance;
    use crate::standards::v_jfif_1_01::subsets::document::io::JpgComposer as JpgAnyComposer;
    use crate::JpgSnapshot;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_BASELINE: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("baseline") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct JpgBaselineComposerComposition;

    impl ArtifactComposition for JpgBaselineComposerComposition {
        type Snapshot = JpgSnapshot;
        const WRITES: Dialect = DIALECT_BASELINE;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_BASELINE, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = JpgAnyComposer::compose(sources)?;
            let checks = check_baseline_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("baseline conformance violated: {} hard issue(s) -- not stamping the baseline dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `jfif-1.01/baseline` -- see the module doc comment for
    /// how this relates to the composer's own pre-serialization hard gate above.
    pub struct JpgBaselineValidator;

    impl SubsetValidator for JpgBaselineValidator {
        const DIALECT: Dialect = DIALECT_BASELINE;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <JpgSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <JpgSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_baseline_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.jpg.baseline.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "baseline SubsetValidator: payload did not decode as a JpgSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<JpgBaselineValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the jfif-1.01 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::subsets::document::io::io_registry::entries()`), matching how `🧾️document`'s
    /// own entry is registered.
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
    use crate::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance;
    use crate::standards::v_jfif_1_01::subsets::document::io::JpgBuilder as JpgAnyBuilder;
    use crate::{JpgDiff, JpgMutation, JpgSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct JpgBaselineBuilderConstruction(JpgAnyBuilder);

    impl ArtifactBuilder for JpgBaselineBuilderConstruction {
        type Snapshot = JpgSnapshot;
        type Mutation = JpgMutation;
        type Diff = JpgDiff;

        fn empty() -> Self {
            Self(JpgAnyBuilder::empty())
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self(JpgAnyBuilder::from_snapshot(snapshot))
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self(JpgAnyBuilder::from_text(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self(JpgAnyBuilder::from_binary(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (inner, diff) = self.0.mutate(mutation);
            (Self(inner), diff)
        }
        fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            Ok(Self(self.0.absorb(diff)?))
        }

        /// 🛡️ The real construction gate: however the wrapped snapshot got here, a hard baseline
        /// violation fails `build()` -- soft diagnostics are not surfaced here (`ArtifactBuilder`'s
        /// `build` has no diagnostics-on-success channel), matching `JpgAnyBuilder::build`'s existing
        /// contract of "diagnostics accumulated during mutation, not from validation".
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            let snapshot = self.0.build()?;
            let hard: Vec<semio_framework_diagnostic::Diagnostic> = check_baseline_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(snapshot)
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
    use crate::schema::snapshot::JpgHuffmanClass;
    use crate::standards::v_jfif_1_01::subsets::document::io::JpgAnalyzer as JpgAnyAnalyzer;
    pub use crate::standards::v_jfif_1_01::subsets::document::io::JpgParts;
    use crate::JpgSnapshot;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
    use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use semio_framework_value::ValueError;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("baseline") };

    /// 🏷️ SOF0 (baseline sequential DCT) marker byte, T.81 Table B.1.
    pub const SOF0: u8 = 0xC0;

    //#region 🔖️Conformance
    pub const CODE_NO_FRAME: &str = "stdio.jpg.baseline.no-frame";
    pub const CODE_SOF_MARKER: &str = "stdio.jpg.baseline.sof-marker";
    pub const CODE_PRECISION: &str = "stdio.jpg.baseline.precision";
    pub const CODE_ARITHMETIC: &str = "stdio.jpg.baseline.arithmetic-conditioning-present";
    pub const CODE_HUFFMAN_TABLE_COUNT: &str = "stdio.jpg.baseline.huffman-table-count";
    pub const CODE_COMPONENT_SAMPLING: &str = "stdio.jpg.baseline.component-sampling";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ITU-T T.81 / ISO 10918-1 Annex F baseline sequential DCT conformance checks (JFIF
    /// 1.01 container) against one already-decoded `JpgSnapshot`. Shared single source of truth:
    /// `JpgBaselineComposer::compose` hard-gates on this (pre-serialization, authoritative),
    /// `JpgBaselineBuilder::build` hard-gates on this too, and the registered `SubsetValidator`
    /// re-runs it post-hoc against the wire payload for the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_baseline_conformance(snapshot: &JpgSnapshot) -> Vec<Diagnostic> {
        check_baseline_conformance_with(snapshot, &mut |_, _| Ok(())).expect("the unbounded conformance callback cannot fail")
    }

    /// ⏱️ Applies the baseline rules with bounded traversal checkpoints.
    pub fn check_baseline_conformance_with(snapshot: &JpgSnapshot, checkpoint: &mut dyn FnMut(usize, usize) -> Result<(), ValueError>) -> Result<Vec<Diagnostic>, ValueError> {
        checkpoint(0, snapshot.huffman_tables.len() + snapshot.frame.as_ref().map_or(0, |frame| frame.components.len()))?;
        let mut out = Vec::new();

        let Some(frame) = &snapshot.frame else {
            out.push(hard(CODE_NO_FRAME, "no SOF0 frame header retained on this snapshot -- baseline conformance cannot be certified without one (never decoded, or built without going through engine::decode_jpg)".into()));
            return Ok(out);
        };

        if snapshot.sof_marker != SOF0 {
            out.push(hard(CODE_SOF_MARKER, format!("frame marker 0x{:02X} is not SOF0 (0x{SOF0:02X}) -- T.81 Annex F baseline sequential DCT is SOF0 only (no progressive/extended/arithmetic SOFn variants)", snapshot.sof_marker)));
        }
        if frame.precision != 8 {
            out.push(hard(CODE_PRECISION, format!("sample precision {} is not 8 -- T.81 §4.2 baseline sequential DCT mandates 8-bit samples", frame.precision)));
        }
        if snapshot.arithmetic {
            out.push(hard(CODE_ARITHMETIC, "a DAC (arithmetic-coding conditioning) segment was present -- T.81 Annex F baseline sequential DCT is Huffman-entropy-coded only".into()));
        }

        let mut dc_count = 0;
        let mut ac_count = 0;
        for (position, table) in snapshot.huffman_tables.iter().enumerate() {
            if position % 256 == 0 { checkpoint(position, snapshot.huffman_tables.len())?; }
            match table.class { JpgHuffmanClass::Dc => dc_count += 1, JpgHuffmanClass::Ac => ac_count += 1 }
        }
        if dc_count > 2 || ac_count > 2 {
            out.push(soft(CODE_HUFFMAN_TABLE_COUNT, format!("{dc_count} DC / {ac_count} AC Huffman table(s) defined -- typical JFIF baseline practice never needs more than 2 of each (one luma, one chroma)")));
        }
        if frame.components.len() > 4 {
            out.push(soft(CODE_COMPONENT_SAMPLING, format!("{} frame components -- JFIF 1.01 conventionally encodes grayscale (1) or YCbCr (3) images; more than 4 is unusual", frame.components.len())));
        }
        for (position, c) in frame.components.iter().enumerate() {
            if position % 256 == 0 { checkpoint(position, frame.components.len())?; }
            if !(1..=4).contains(&c.h_sampling) || !(1..=4).contains(&c.v_sampling) {
                out.push(soft(CODE_COMPONENT_SAMPLING, format!("component {} has sampling factors {}x{} outside JFIF's conventional 1..=4 range", c.id, c.h_sampling, c.v_sampling)));
            }
        }
        Ok(out)
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.jpg` (jfif-1.01/🧱️baseline): delegates the real parse to the 🧾️document
    /// subset's analyzer (same `JpgSnapshot`), then folds real T.81 baseline conformance diagnostics
    /// on top. `sniff` also delegates -- a subset-level sniff for `baseline` is "is this recognizable
    /// as a JPEG at all", the same SOI magic-byte probe every jfif-1.01 dialect shares; conformance is
    /// a separate, heavier question answered by `analyze`/`check_baseline_conformance`, not by `sniff`.
    pub struct JpgBaselineAnalyzerAnalysis;

    impl ArtifactAnalysis for JpgBaselineAnalyzerAnalysis {
        type Parts = JpgParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            JpgAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = JpgAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_baseline_conformance(snapshot);
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec JpgBaselineBuilderFacets {
        construction: JpgBaselineBuilderConstruction,
        analysis: JpgBaselineAnalyzerAnalysis,
        composition: super::io::derived_composition::JpgBaselineComposerComposition,
    }
    builder: JpgBaselineBuilder,
    analyzer: JpgBaselineAnalyzer,
    composer: JpgBaselineComposer,
);
