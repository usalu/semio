//! 🚪️ IO stdio.step (ap214/✳️any) — registration now flows through the `s.stdio.step`
//! `ArtifactDeclaration` (`crate::artifacts::step::declaration`), not per-leaf register().
//#region 🔖️Submodules
/// 🧱 BrepMesh analyzer view, derived from the generic graph — never persisted itself.
#[path = "🧱️brep/🦀️component.rs"]
pub mod brep;
/// 🪜 Shared CC ladder classification + FILE_SCHEMA/PRODUCT-chain scans, reused by all six
/// `✳️ccN` subset analyzers (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
#[path = "🪜️ladder/🦀️component.rs"]
pub mod ladder;
/// 📐 Shared ISO 10303-21 tokenizer + generic graph — public, importable cross-artifact (ifc
/// reuses it) and cross-plugin (📐️cad reuses it too) — dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[path = "📐️part21/🦀️component.rs"]
pub mod part21;
//#endregion 🔖️Submodules

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::step::standards::v_ap214::subsets::any::schema::StepAnalyzer;
    use crate::artifacts::step::StepSnapshot;
    use semio_framework_plugin::ArtifactAnalyzer as _;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct StepComposerComposition;

    impl ArtifactComposition for StepComposerComposition {
        type Snapshot = StepSnapshot;
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
                return Err(ComposeError { message: "StepComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = StepAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "StepComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the composed `any` + `cc1`..`cc6` entry union `declaration()`'s `.composers(...)` reaches
/// through the `engine` barrel shim (`📦️glue.rs`'s `pub mod engine { pub use super::subsets::
/// any::io::*; pub use super::subsets::any::schema::*; }`).
pub mod io_registry {
    use crate::artifacts::step::standards::v_ap214::subsets::any::schema::StepComposer as StepRawAnyComposer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::StepCc1Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc2::schema::StepCc2Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc3::schema::StepCc3Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::StepCc4Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc5::schema::StepCc5Composer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::StepCc6Composer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<StepRawAnyComposer>(),
                    composer_entry_of::<StepCc1Composer>(),
                    composer_entry_of::<StepCc2Composer>(),
                    composer_entry_of::<StepCc3Composer>(),
                    composer_entry_of::<StepCc4Composer>(),
                    composer_entry_of::<StepCc5Composer>(),
                    composer_entry_of::<StepCc6Composer>(),
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
