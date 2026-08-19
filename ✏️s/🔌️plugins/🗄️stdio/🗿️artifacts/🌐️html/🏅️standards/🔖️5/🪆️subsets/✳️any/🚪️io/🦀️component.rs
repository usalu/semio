//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
    use crate::artifacts::html::standards::v5::subsets::any::schema::HtmlAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.html", standard: StandardId("5"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct HtmlComposerComposition;

    impl ArtifactComposition for HtmlComposerComposition {
        type Snapshot = HtmlSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "HtmlComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = HtmlAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "HtmlComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    pub async fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::html::standards::v5::subsets::any::schema::html_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<HtmlSnapshot, crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation>(
            crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::STDIO_HTML_DOCUMENT_SCHEMA,
        ));
    }

    /// 💡️ Registers `s.stdio.html.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to the artifact schema descriptor above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    pub async fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::html::standards::v5::subsets::any::schema::inferences::html_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 📥️Sniff
/// ⚙️ Html (5, WHATWG) sniff — 🚧 scaffolded by W1b: real `<!DOCTYPE html>` detection (case-
/// insensitive, leading-whitespace-tolerant — genuinely inspects the bytes, not a fixed offset
/// check). The full tokenizer/node tree (Element/Text/Comment/RawText, void-element set) lands
/// in W3.
pub mod import {
    pub mod deserializers {
        pub async fn sniff_real_bytes(bytes: &[u8]) -> bool {
            let text = String::from_utf8_lossy(bytes);
            text.trim_start().to_ascii_lowercase().starts_with("<!doctype html")
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[semio_framework_async_macros::async_test]
            async fn sniffs_a_real_doctype_case_insensitively() {
                assert!(sniff_real_bytes(b"<!DOCTYPE html>\n<html></html>"));
                assert!(sniff_real_bytes(b"  \n<!doctype HTML>"));
            }

            #[semio_framework_async_macros::async_test]
            async fn rejects_non_html() {
                assert!(!sniff_real_bytes(b"just some text"));
            }
        }
    }
}
//#endregion 📥️Sniff

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::html::standards::v5::subsets::any::schema::HtmlComposer as HtmlRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<HtmlRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
