//! 🚪️ IO stdio.json (rfc8259/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::JsonAnalyzer;
    use crate::artifacts::json::JsonSnapshot;
    use semio_framework_plugin::ArtifactAnalyzer as _;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct JsonComposerComposition;

    impl ArtifactComposition for JsonComposerComposition {
        type Snapshot = JsonSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
                return Err(ComposeError { message: "JsonComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = JsonAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "JsonComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure `ComposerEntry` aggregation, no
/// engine needed. NOTE: always reach this via a fully-qualified path
/// (`standards::v_rfc8259::subsets::any::io::io_registry::entries()`) — the artifact root's OWN
/// `io_registry` (`🗿️artifacts/🔣️json/🦀️component.rs`) shadows this name with a DIFFERENT return
/// type (`&'static [&'static ComposerEntry]` vs this module's `&'static [ComposerEntry]`); a bare
/// `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::JsonComposer as JsonRawAnyComposer;
    use crate::artifacts::json::standards::v_rfc8259::subsets::i_json::schema::JsonIJsonComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<JsonRawAnyComposer>(), composer_entry_of::<JsonIJsonComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
