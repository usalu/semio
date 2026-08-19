//! 🧬️ XmlSnapshot schema (1.0/✳️valid) — reuses the ✳️any subset's `XmlSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.xml` schema id). W3C XML 1.0 Fifth Edition §5.1 validity is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✳️valid/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::xml::standards::v1_0::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::diff::XmlDiff;
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::mutations::XmlMutation;
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::XmlSnapshot;
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::XmlBuilder as XmlAnyBuilder;
    use crate::artifacts::xml::standards::v1_0::subsets::valid::schema::check_valid_conformance;
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct XmlValidBuilderConstruction(XmlAnyBuilder);

    impl ArtifactBuilder for XmlValidBuilderConstruction {
        type Snapshot = XmlSnapshot;
        type Mutation = XmlMutation;
        type Diff = XmlDiff;

        async fn empty() -> Self {
            Self(XmlAnyBuilder::empty())
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self(XmlAnyBuilder::from_snapshot(snapshot))
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self(XmlAnyBuilder::from_text(text)?))
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self(XmlAnyBuilder::from_binary(bytes)?))
        }

        async fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (inner, diff) = self.0.mutate(mutation);
            (Self(inner), diff)
        }

        async fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            Ok(Self(self.0.absorb(diff)?))
        }

        /// 🛡️ The real construction gate: however `self.0`'s inner snapshot got here, a hard XML 1.0
        /// §5.1 validity violation fails `build()` -- soft/advisory diagnostics pass through as `Ok`.
        async fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let snapshot = self.0.build()?;
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
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn conforming_snapshot_builds_clean() {
            let snapshot = XmlValidBuilderConstruction::from_text("<!DOCTYPE root>\n<root/>").expect("parses").build().expect("conforming construction must build");
            assert_eq!(snapshot.doc.doctype.as_ref().map(|doctype| doctype.name.as_str()), Some("root"));
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_doctype_fails_build() {
            let err = XmlValidBuilderConstruction::from_text("<root/>").expect("parses").build().expect_err("a document without a doctype must fail build()");
            assert!(err.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing"));
        }

        #[semio_framework_async_macros::async_test]
        async fn root_name_mismatch_injected_via_raw_mutate_still_fails_build() {
            let built = XmlValidBuilderConstruction::from_text("<!DOCTYPE root>\n<root/>").expect("parses").build().expect("clean build");
            let mut mismatched = built;
            mismatched.doc.doctype = Some("<!DOCTYPE somethingElse>".into());
            let (mutated, _diff) = XmlValidBuilderConstruction::from_snapshot(XmlSnapshot::default()).mutate(XmlMutation::SetSnapshot { snapshot: mismatched });
            let err = mutated.build().expect_err("a doctype/root name mismatch must fail build()");
            assert!(err.iter().any(|d| d.code.0 == "stdio.xml.valid.root-name-mismatch"));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::{XmlNode, XmlSnapshot};
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::XmlAnalyzer as XmlAnyAnalyzer;
    pub use crate::artifacts::xml::standards::v1_0::subsets::any::schema::XmlParts;
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
    async fn root_element_name(snapshot: &XmlSnapshot) -> Option<&str> {
        match &snapshot.doc.root {
            Some(XmlNode::Element { name, .. }) => Some(name.as_str()),
            _ => None,
        }
    }

    async fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    async fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real, scope-limited W3C XML 1.0 Fifth Edition §5.1 validity checks against one
    /// already-decoded `XmlSnapshot`. Shared single source of truth: `XmlValidComposer::compose`
    /// hard-gates on this (pre-serialization, authoritative), `XmlValidBuilder::build` hard-gates on
    /// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload for
    /// the D5 validate-on-build hook.
    pub async fn check_valid_conformance(snapshot: &XmlSnapshot) -> Vec<Diagnostic> {
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
                if doctype.external_id.is_some() {
                    if snapshot.doc.declaration.as_ref().and_then(|d| d.standalone) == Some(true) {
                        out.push(soft(CODE_STANDALONE_EXTERNAL_SUBSET, "XML declaration says standalone=\"yes\" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9".into()));
                    }
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

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            XmlAnyAnalyzer::sniff(source)
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
    mod tests {
        use super::*;
        use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::{XmlDeclaration, XmlDocument};

        async fn snapshot_with(doctype: Option<&str>, standalone: Option<bool>, root_name: &str) -> XmlSnapshot {
            XmlSnapshot {
                doc: XmlDocument {
                    declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: None, standalone }),
                    doctype: doctype.map(Into::into),
                    prolog: Vec::new(),
                    root: Some(XmlNode::Element { name: root_name.into(), attrs: Vec::new(), children: Vec::new() }),
                },
                ..XmlSnapshot::default()
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_doctype_reports_only_the_always_on_advisory() {
            let snapshot = snapshot_with(Some("<!DOCTYPE html>"), None, "html");
            let diagnostics = check_valid_conformance(&snapshot);
            assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
            assert_eq!(diagnostics[0].code.0, CODE_VALIDITY_NOT_VERIFIED);
            assert_eq!(diagnostics[0].severity, Severity::Warning);
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_doctype_is_hard() {
            let snapshot = snapshot_with(None, None, "html");
            let diagnostics = check_valid_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DOCTYPE_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn root_name_mismatch_is_hard() {
            let snapshot = snapshot_with(Some("<!DOCTYPE book>"), None, "html");
            let diagnostics = check_valid_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ROOT_NAME_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn standalone_yes_with_external_subset_is_soft() {
            let snapshot = snapshot_with(Some("<!DOCTYPE html SYSTEM \"http://example.com/html.dtd\">"), Some(true), "html");
            let diagnostics = check_valid_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STANDALONE_EXTERNAL_SUBSET && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn standalone_yes_without_external_subset_is_clean() {
            let snapshot = snapshot_with(Some("<!DOCTYPE html>"), Some(true), "html");
            let diagnostics = check_valid_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_STANDALONE_EXTERNAL_SUBSET), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn public_external_subset_reference_is_detected() {
            let snapshot = snapshot_with(Some("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">"), Some(true), "html");
            let diagnostics = check_valid_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STANDALONE_EXTERNAL_SUBSET), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec XmlValidBuilderFacets {
        construction: XmlValidBuilderConstruction,
        analysis: XmlValidAnalyzerAnalysis,
        composition: crate::artifacts::xml::standards::v1_0::subsets::valid::io::derived_composition::XmlValidComposerComposition,
    }
    builder: XmlValidBuilder,
    analyzer: XmlValidAnalyzer,
    composer: XmlValidComposer,
);
//#endregion 🧬️DerivedArtifactFacets
