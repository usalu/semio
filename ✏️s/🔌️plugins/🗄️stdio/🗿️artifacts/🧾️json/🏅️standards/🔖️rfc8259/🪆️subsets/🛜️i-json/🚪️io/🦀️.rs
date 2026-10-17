//! 🚪️ IO stdio.json (rfc8259/🛜️i-json) — reuses the ✳️any subset's `txt` raw-codec DAG leaf
//! rather than duplicating it (same `JsonSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::standards::v_rfc8259::subsets::base::io::JsonComposer as JsonAnyComposer;
    use crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_I_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct JsonIJsonComposerComposition;

    impl ArtifactComposition for JsonIJsonComposerComposition {
        type Snapshot = JsonSnapshot;
        const WRITES: Dialect = DIALECT_I_JSON;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_I_JSON, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = JsonAnyComposer::compose(sources)?;
            let checks = check_i_json_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("I-JSON (RFC 7493) conformance violated: {} hard issue(s) -- not stamping the i-json dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `rfc8259/i-json`.
    pub struct JsonIJsonValidator;

    impl SubsetValidator for JsonIJsonValidator {
        const DIALECT: Dialect = DIALECT_I_JSON;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <JsonSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <JsonSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_i_json_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.json.i-json.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "I-JSON SubsetValidator: payload did not decode as a JsonSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<JsonIJsonValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the rfc8259 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::standards::v_rfc8259::subsets::base::io::io_registry::entries()`).
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
    use crate::standards::v_rfc8259::subsets::base::schema::diff::JsonDiff;
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance;
    #[cfg(test)] use crate::apply_mutation;
    use crate::standards::v_rfc8259::subsets::i_json::schema::mutations::JsonIJsonMutation;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Holds the snapshot directly rather than wrapping the ✳️any builder, because this subset's
    /// `Mutation` is its OWN `JsonIJsonMutation` (see `mutations/🦀️.rs`) and not the ✳️any
    /// sibling's -- the same shape `ZipIso21320BuilderConstruction` already uses for the same reason.
    /// `from_text`/`from_binary` stay exactly the ✳️any codec (`ArtifactDsl`/`ArtifactPack` on the
    /// SHARED `JsonSnapshot`); only the mutation vocabulary and the RFC 7493 build gate are this
    /// subset's own.
    #[derive(Clone, Debug, Default)]
    pub struct JsonIJsonBuilderConstruction {
        snapshot: JsonSnapshot,
    }

    impl ArtifactBuilder for JsonIJsonBuilderConstruction {
        type Snapshot = JsonSnapshot;
        type Mutation = JsonIJsonMutation;
        type Diff = JsonDiff;

        fn empty() -> Self {
            Self { snapshot: JsonSnapshot::default() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { snapshot: <JsonSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <JsonSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)? })
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, outcome)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however the inner snapshot got here, a hard RFC 7493
        /// violation fails `build()` -- soft/advisory diagnostics pass through as `Ok`. Two doors now
        /// lead to the same clauses: this gate rejects a violating STATE, and `JsonIJsonMutation`'s
        /// own `diff()` rejects a violating EDIT before it can produce one.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_i_json_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonSnapshot, JsonValue};
    use crate::standards::v_rfc8259::subsets::base::io::JsonAnalyzer as JsonAnyAnalyzer;
    pub use crate::standards::v_rfc8259::subsets::base::io::JsonParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };

    use crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance;
    #[cfg(test)] use crate::standards::v_rfc8259::subsets::i_json::schema::{CODE_DUPLICATE_MEMBER, CODE_STRING_NONCHARACTER, CODE_TOP_LEVEL_SCALAR, CODE_UNSAFE_INTEGER};

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.json` (rfc8259/🛜️i-json): delegates the real parse to the ✳️any subset's
    /// analyzer (same `JsonSnapshot`), then folds real I-JSON conformance diagnostics on top.
    pub struct JsonIJsonAnalyzerAnalysis;

    impl ArtifactAnalysis for JsonIJsonAnalyzerAnalysis {
        type Parts = JsonParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            JsonAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = JsonAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_i_json_conformance(snapshot);
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
    pub spec JsonIJsonBuilderFacets {
        construction: JsonIJsonBuilderConstruction,
        analysis: JsonIJsonAnalyzerAnalysis,
        composition: JsonIJsonComposerComposition,
    }
    builder: JsonIJsonBuilder,
    analyzer: JsonIJsonAnalyzer,
    composer: JsonIJsonComposer,
);
