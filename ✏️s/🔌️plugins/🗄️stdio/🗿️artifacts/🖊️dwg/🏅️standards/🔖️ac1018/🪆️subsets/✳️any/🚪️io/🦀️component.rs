//! 🚪️ IO stdio.dwg (ac1018/✳️any) — registration now flows through the `s.stdio.dwg`
//! `ArtifactDeclaration` (`crate::artifacts::dwg::declaration`, combined-composers per its own doc
//! comment), not per-leaf register(). ac1018's OWN `register()`/schema/inference/language
//! registration is confirmed dead repo-wide (superseded by real R2004+/ac1024 decode per
//! Decision #5) and was deleted outright with the rest of `⚙️engine` — only its composer entries
//! and pure document helpers survive, since `declaration()`'s `dwg_combined_composer_entries()`
//! unions both standards' `io_registry::entries()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::DwgAnalyzer;
    use crate::artifacts::dwg::DwgSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct DwgComposerComposition;

    impl ArtifactComposition for DwgComposerComposition {
        type Snapshot = DwgSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "DwgComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = DwgAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "DwgComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// unioned with ac1024's own `io_registry::entries()` by the root `crate::artifacts::dwg::
/// declaration()`'s `dwg_combined_composer_entries()`.
pub mod io_registry {
    use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::DwgComposer as DwgRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<DwgRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
