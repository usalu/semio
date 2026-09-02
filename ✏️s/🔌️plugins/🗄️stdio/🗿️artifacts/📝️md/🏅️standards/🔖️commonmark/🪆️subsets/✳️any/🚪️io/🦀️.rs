//! 🚪️ IO stdio.md (commonmark/✳️any) — registration flows through `md::declaration()`
//! (`🗄️stdio/🗿️artifacts/📝️md/🦀️.rs`), not a side-effecting `register()`; `⚙️engine`
//! dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — `MdEngine` (zero
//! construction sites) deleted outright; its orphaned `register()`/`register_artifact_schema()`/
//! `register_artifact_inferences()`/`register_pilot_languages()` (zero callers, superseded by
//! `declaration()`) deleted outright too; `parse_markdown_blocks` + the block/inline parser moved
//! to `📥️import/🧩️deserializers`; `render_markdown_blocks` + the block/inline renderer moved to
//! `📤️export/🧵️serializers`; `io_registry` moved here from `⚙️engine`, live (`md::declaration()`'s
//! `.composers(...)` and this artifact's own root `io_registry` both reach it).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::MdAnalyzer;
    use crate::artifacts::md::MdSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct MdComposerComposition;

    impl ArtifactComposition for MdComposerComposition {
        type Snapshot = MdSnapshot;
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
                return Err(ComposeError { message: "MdComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = MdAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "MdComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::MdComposer as MdRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<MdRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
