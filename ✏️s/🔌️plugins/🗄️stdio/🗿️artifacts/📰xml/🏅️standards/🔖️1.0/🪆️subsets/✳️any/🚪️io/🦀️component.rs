//! 🚪️ IO stdio.xml (1.0/✳️any) — registration flows through `xml::declaration()`
//! (`🗄️stdio/🗿️artifacts/📰xml/🦀️component.rs`), not a side-effecting `register()`; `⚙️engine`
//! dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — `XmlEngine` (zero
//! construction sites) deleted outright; its orphaned `register()`/`register_artifact_schema()`/
//! `register_artifact_inferences()`/`register_pilot_languages()` (zero callers, superseded by
//! `declaration()`) deleted outright too; `io_registry` moved here from `⚙️engine`, live
//! (`xml::declaration()`'s `.composers(...)` and this artifact's own root `io_registry` both reach
//! it). `empty_xml_snapshot`/`demo_xml_snapshot` + tests moved to `subsets::any::schema` — xml has
//! no dedicated `📤️export/🧵️serializers`/`📥️import/🧩️deserializers` codec of its own (the real
//! text codec, `xml_document_from_text`/`xml_document_to_text`, already lives in
//! `subsets::any::schema::snapshot`, unmoved).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::XmlAnalyzer;
    use crate::artifacts::xml::XmlSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct XmlComposerComposition;

    impl ArtifactComposition for XmlComposerComposition {
        type Snapshot = XmlSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "XmlComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = XmlAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "XmlComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::XmlComposer as XmlRawAnyComposer;
    use crate::artifacts::xml::standards::v1_0::subsets::valid::schema::XmlValidComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<XmlRawAnyComposer>(), composer_entry_of::<XmlValidComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
