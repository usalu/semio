//! 🧬️ XmlSnapshot schema (1.0/✳️valid) — reuses the ✳️any subset's `XmlSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.xml` schema id). W3C XML 1.0 Fifth Edition §5.1 validity is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✅️valid/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::standards::v1_0::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ THIS subset's own mutation vocabulary — `XmlValidMutation`, not the `✳️any` subset's
/// `XmlMutation` the glob re-export above would otherwise supply. Declared here rather than in the
/// crate's module glue so the vocabulary lives with the subset that owns it; the explicit item wins
/// over the glob import, which is exactly the intent. Its own gate (a `SetSnapshot` that would land
/// a hard §2.8 violation is refused outright) is tested inside that module;
/// `derived_construction`'s tests below cover the SECOND, independent layer — `build()`, which
/// catches a snapshot that arrived around the vocabulary entirely.
#[path = "🧬️mutations/🦀️.rs"]
pub mod valid_mutations;
pub use valid_mutations::{apply_xml_valid_mutation, inverse_xml_valid_mutation, XmlValidMutation, KINDS as VALID_MUTATION_KINDS};
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v1_0::subsets::base::schema::diff::XmlDiff;
    use crate::standards::v1_0::subsets::base::schema::snapshot::XmlSnapshot;
    use crate::standards::v1_0::subsets::valid::schema::check_valid_conformance;
    use crate::standards::v1_0::subsets::valid::schema::valid_mutations::{apply_xml_valid_mutation, XmlValidMutation};
    use dsl::{Diagnostic, Severity};
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

        fn from_text(text: &str) -> Result<Self, store::TextError> {
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
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v1_0::subsets::base::schema::snapshot::{XmlNode, XmlSnapshot};
    use crate::standards::v1_0::subsets::base::schema::XmlAnalyzer as XmlAnyAnalyzer;
    pub use crate::standards::v1_0::subsets::base::schema::XmlParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
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
        let mut out = Vec::new();
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
                if doctype.external_id.is_some()
                    && snapshot.doc.declaration.as_ref().and_then(|d| d.standalone) == Some(true) {
                        out.push(soft(CODE_STANDALONE_EXTERNAL_SUBSET, "XML declaration says standalone=\"yes\" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9".into()));
                    }
            }
        }
        out.push(soft(
            CODE_VALIDITY_NOT_VERIFIED,
            "validity not fully verified: this schema retains <!DOCTYPE ...> as a raw unparsed String with no internal/external subset markup declarations parsed, so full DTD element/attribute-list content-model validation (§3.2/§3.3) is out of scope from this data -- only the presence of a doctype and its declared-root-name/actual-root-name agreement are checked".into(),
        ));
        out
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
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
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
//#endregion 🧬️DerivedArtifactFacets
