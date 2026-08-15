//! 🚪️ IO stdio.svg (1.1/✳️tiny) — reuses the ✳️any subset's `xml` import/export leaves rather
//! than duplicating them (same `SvgSnapshot` type, same catalog DAG edge). Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` already
//! established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::{set_element_attr, SvgSnapshot};
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::SvgComposer as SvgAnyComposer;
    use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::check_svg_tiny_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_TINY: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("tiny") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct SvgTinyComposerComposition;

    impl ArtifactComposition for SvgTinyComposerComposition {
        type Snapshot = SvgSnapshot;
        const WRITES: Dialect = DIALECT_TINY;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_TINY, DEP_XML]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = SvgAnyComposer::compose(sources)?;
            let mut snapshot = inner.snapshot;
            if let Some(root) = snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some("tiny".into()));
                set_element_attr(root, "version", Some("1.1".into()));
            }
            let checks = check_svg_tiny_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("SVG Tiny 1.1 conformance violated: {} hard issue(s) -- not stamping the tiny dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `1.1/tiny` -- see the module doc comment for how this
    /// relates to (and honestly differs from) the composer's own pre-serialization hard gate above.
    pub struct SvgTinyValidator;

    impl SubsetValidator for SvgTinyValidator {
        const DIALECT: Dialect = DIALECT_TINY;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SvgSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_svg_tiny_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.svg.tiny.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "SVG Tiny SubsetValidator: payload did not decode as an SvgSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SvgTinyValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the 1.1 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::svg::standards::v1_1::engine::io_registry::entries()`), matching how `✳️any`'s own
    /// entry is registered.
    pub fn register() {
        register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::SvgTinyBuilder;
        use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::{CODE_ATTRIBUTE, CODE_ELEMENT};
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        #[test]
        fn conforming_document_composes_and_stamps_tiny() {
            let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="10" height="10"/></svg>"#;
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
            let composed = SvgTinyComposerComposition::compose(&sources).expect("clean document must compose to tiny");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
            match &composed.snapshot.doc.root {
                Some(crate::artifacts::xml::schema::snapshot::XmlNode::Element { attrs, .. }) => {
                    assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value == "tiny"));
                    assert!(attrs.iter().any(|a| a.name == "version" && a.value == "1.1"));
                }
                other => panic!("expected element root, got {other:?}"),
            }
        }

        #[test]
        fn blocklisted_element_fails_compose_with_real_diagnostic() {
            let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><linearGradient id="g1"/></svg>"#;
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
            let err = SvgTinyComposerComposition::compose(&sources).expect_err("a document with a linearGradient must not stamp tiny");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_ELEMENT && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn blocklisted_attribute_fails_compose_with_real_diagnostic() {
            let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="10" height="10" filter="url(#f1)"/></svg>"#;
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
            let err = SvgTinyComposerComposition::compose(&sources).expect_err("a document with a filter attribute must not stamp tiny");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_ATTRIBUTE && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn subset_validator_recheck_flags_no_hard_issue_on_a_clean_builder_document() {
            let snapshot = SvgTinyBuilder::empty().build().expect("empty document builds clean");
            let bytes = <SvgSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = SvgTinyValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
