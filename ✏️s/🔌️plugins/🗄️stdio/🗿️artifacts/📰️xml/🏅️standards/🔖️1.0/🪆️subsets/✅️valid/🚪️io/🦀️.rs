//! 🚪️ IO stdio.xml (1.0/✳️valid) — reuses the ✳️any subset's `txt` raw-codec DAG leaf rather than
//! duplicating it (same `XmlSnapshot` type, same catalog DAG edges). Registration flows through
//! `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` already
//! established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_0::subsets::base::schema::snapshot::XmlSnapshot;
    use crate::standards::v1_0::subsets::base::io::XmlComposer as XmlAnyComposer;
    use crate::standards::v1_0::subsets::valid::schema::check_valid_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_VALID: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("valid") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct XmlValidComposerComposition;

    impl ArtifactComposition for XmlValidComposerComposition {
        type Snapshot = XmlSnapshot;
        const WRITES: Dialect = DIALECT_VALID;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_VALID, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = XmlAnyComposer::compose(sources)?;
            let checks = check_valid_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("XML 1.0 validity violated: {} hard issue(s) -- not stamping the valid dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `1.0/valid`.
    pub struct XmlValidValidator;

    impl SubsetValidator for XmlValidValidator {
        const DIALECT: Dialect = DIALECT_VALID;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <XmlSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_valid_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.xml.valid.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "XML valid SubsetValidator: payload did not decode as an XmlSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<XmlValidValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the 1.0 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::standards::v1_0::engine::io_registry::entries()`).
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
    use crate::standards::v1_0::subsets::base::schema::diff::XmlDiff;
    use crate::standards::v1_0::subsets::base::schema::snapshot::XmlSnapshot;
    use crate::standards::v1_0::subsets::valid::schema::check_valid_conformance;
    use crate::standards::v1_0::subsets::valid::schema::valid_mutations::{apply_xml_valid_mutation, XmlValidMutation};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Holds the shared `XmlSnapshot` directly rather than wrapping the `✳️any` builder: this
    /// subset's `mutate` speaks `XmlValidMutation`, which the `✳️any` builder has no impl for, and
    /// forwarding would have to unwrap a snapshot the `✳️any` builder keeps private. The decode
    /// entry points are the same `ArtifactDsl`/`ArtifactPack` codecs the `✳️any` builder itself
    /// calls, so nothing about the parse is duplicated or diverges.
    #[derive(Clone, Debug, Default)]
    pub struct XmlValidBuilderConstruction {
        snapshot: XmlSnapshot,
    }

    impl ArtifactBuilder for XmlValidBuilderConstruction {
        type Snapshot = XmlSnapshot;
        type Mutation = XmlValidMutation;
        type Diff = XmlDiff;

        fn empty() -> Self {
            Self::default()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<XmlSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XmlSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = apply_xml_valid_mutation(&mut self.snapshot, &mutation);
            (self, outcome)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <XmlDiff as protocol::MutationDiff<XmlSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard XML 1.0 §5.1
        /// validity violation fails `build()` -- soft/advisory diagnostics pass through as `Ok`.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let snapshot = self.snapshot;
            let hard: Vec<Diagnostic> = check_valid_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    use crate::standards::v1_0::subsets::base::schema::snapshot::{XmlNode, XmlSnapshot};
    use crate::standards::v1_0::subsets::base::io::XmlAnalyzer as XmlAnyAnalyzer;
    pub use crate::standards::v1_0::subsets::base::io::XmlParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("valid") };

    //#region 🔖️Conformance
    pub const CODE_DOCTYPE_MISSING: &str = "stdio.xml.valid.doctype-missing";
    pub const CODE_ROOT_NAME_MISMATCH: &str = "stdio.xml.valid.root-name-mismatch";
    pub const CODE_STANDALONE_EXTERNAL_SUBSET: &str = "stdio.xml.valid.standalone-external-subset";
    pub const CODE_VALIDITY_NOT_VERIFIED: &str = "stdio.xml.valid.validity-not-fully-verified";

    /// 🌳️ The actual root element's tag name, if a root element is present at all.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn root_element_name(snapshot: &XmlSnapshot) -> Option<&str> {
        match &snapshot.doc.root {
            Some(XmlNode::Element { name, .. }) => Some(name.as_str()),
            _ => None,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real, scope-limited W3C XML 1.0 Fifth Edition §5.1 validity checks against one
    /// already-decoded `XmlSnapshot`. Shared single source of truth: `XmlValidComposer::compose`
    /// hard-gates on this (pre-serialization, authoritative), `XmlValidBuilder::build` hard-gates on
    /// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload for
    /// the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_valid_conformance(snapshot: &XmlSnapshot) -> Vec<Diagnostic> {
        check_conformance(snapshot, &mut |_, _| Ok(())).expect("uncontrolled validity callback cannot refuse")
    }

    /// ⏱️ Checks owned validity with cancellation inside the literal boundary collections.
    pub fn check_valid_conformance_controlled(snapshot: &XmlSnapshot, control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Vec<Diagnostic>, semio_framework_os_kernel::sqlite_snapshot::ValueError> {
        check_conformance(snapshot, &mut |completed, total| control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, completed, total))
    }

    fn check_conformance(snapshot: &XmlSnapshot, progress: &mut dyn FnMut(usize, usize) -> Result<(), semio_framework_os_kernel::sqlite_snapshot::ValueError>) -> Result<Vec<Diagnostic>, semio_framework_os_kernel::sqlite_snapshot::ValueError> {
        let mut out = Vec::new();
        let total = snapshot.doc.prolog.len().checked_add(snapshot.doc.epilog.len()).ok_or_else(|| semio_framework_os_kernel::sqlite_snapshot::ValueError::new(semio_framework_os_kernel::sqlite_snapshot::ValueRefusalKind::WorkLimit,"XML validity boundary count overflow"))?; progress(0, total)?;
        if root_element_name(snapshot).is_none() { out.push(hard("stdio.xml.valid.document-element-missing", "XML validity requires a document element".into())); }
        let mut invalid_boundary = false;
        for (ordinal, node) in snapshot.doc.prolog.iter().chain(&snapshot.doc.epilog).enumerate() { if ordinal % 256 == 0 { progress(ordinal, total)?; } invalid_boundary |= !matches!(node, XmlNode::Comment { .. } | XmlNode::ProcessingInstruction { .. }); }
        if invalid_boundary { out.push(hard("stdio.xml.valid.boundary-kind", "XML validity permits only comments and processing instructions outside the document element".into())); }
        if snapshot.doc.doctype.as_ref().is_some_and(|value| value.prolog_position > snapshot.doc.prolog.len() as u64) { out.push(hard("stdio.xml.valid.doctype-position", "XML validity requires the doctype to occupy its declared prolog position".into())); }
        if let Some(declaration) = &snapshot.doc.declaration {
            if declaration.version != "1.0" { out.push(hard("stdio.xml.valid.version", "XML 1.0 validity requires declaration version 1.0".into())); }
            if let Some(encoding) = &declaration.encoding {
                let mut valid = !encoding.is_empty(); progress(0, encoding.len())?;
                for (ordinal, byte) in encoding.bytes().enumerate() { if ordinal % 256 == 0 { progress(ordinal, encoding.len())?; } valid &= if ordinal == 0 { byte.is_ascii_alphabetic() } else { byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-') }; }
                progress(encoding.len(), encoding.len())?;
                if !valid { out.push(hard("stdio.xml.valid.encoding-name", "XML validity requires a syntactically valid declared encoding name".into())); }
            }
        }
        match &snapshot.doc.doctype {
            None => {
                out.push(hard(CODE_DOCTYPE_MISSING, "no <!DOCTYPE ...> declaration present -- XML 1.0 §5.1 validity requires one (a document without one can be well-formed at best)".into()));
            }
            Some(doctype) => {
                if let Some(actual_root) = root_element_name(snapshot) {
                    if doctype.name != actual_root {
                        out.push(hard(CODE_ROOT_NAME_MISMATCH, format!("doctype declares root name '{}' but the actual root element is '<{actual_root}>' -- §2.8 requires the DOCTYPE Name to match the document element", doctype.name)));
                    }
                }
                if doctype.external_id.is_some() && snapshot.doc.declaration.as_ref().and_then(|d| d.standalone) == Some(true) {
                    out.push(soft(CODE_STANDALONE_EXTERNAL_SUBSET, "XML declaration says standalone=\"yes\" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9".into()));
                }
            }
        }
        out.push(soft(
            CODE_VALIDITY_NOT_VERIFIED,
            "validity not fully verified: the typed doctype retains its name, external identifiers and entity declarations; DTD element and attribute content models (§3.2/§3.3) are absent from this owned schema".into(),
        ));
        progress(total, total)?; Ok(out)
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xml` (1.0/✳️valid): delegates the real parse to the ✳️any subset's
    /// analyzer (same `XmlSnapshot`), then folds real XML validity conformance diagnostics on top.
    pub struct XmlValidAnalyzerAnalysis;

    impl ArtifactAnalysis for XmlValidAnalyzerAnalysis {
        type Parts = XmlParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            XmlAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = XmlAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_valid_conformance(snapshot);
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
    pub spec XmlValidBuilderFacets {
        construction: XmlValidBuilderConstruction,
        analysis: XmlValidAnalyzerAnalysis,
        composition: crate::standards::v1_0::subsets::valid::io::derived_composition::XmlValidComposerComposition,
    }
    builder: XmlValidBuilder,
    analyzer: XmlValidAnalyzer,
    composer: XmlValidComposer,
);
