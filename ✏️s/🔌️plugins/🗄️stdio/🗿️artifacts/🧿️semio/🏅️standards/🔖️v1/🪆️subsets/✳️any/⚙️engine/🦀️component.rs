//! ⚙️ Semio v1 engine — holds the two REAL shared submodules (🧮️geometry, 🧰️triples — see their
//! own doc comments) and `register()`, which registers all 15 subsets' schema descriptors +
//! document codecs + SubsetValidators (14 domain subsets each register a validator; the ✳️any
//! envelope does not — subset `"*"` is exempt per `policyStandardsSubsetVocabularyBreaches`,
//! matching every other stdio artifact's `✳️any`). `text` (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM,
//! W2a) is the 14th domain subset — a LEAF (no import/export io entries yet, see its own
//! `🚪️io/🦀️component.rs` doc comment).

pub fn register() {
    crate::artifacts::semio::standards::v1::subsets::brep::io::register();
    crate::artifacts::semio::standards::v1::subsets::mesh::io::register();
    crate::artifacts::semio::standards::v1::subsets::model::io::register();
    crate::artifacts::semio::standards::v1::subsets::value::io::register();
    crate::artifacts::semio::standards::v1::subsets::document::io::register();
    crate::artifacts::semio::standards::v1::subsets::cad::io::register();
    crate::artifacts::semio::standards::v1::subsets::drawing::io::register();
    crate::artifacts::semio::standards::v1::subsets::image::io::register();
    crate::artifacts::semio::standards::v1::subsets::video::io::register();
    crate::artifacts::semio::standards::v1::subsets::audio::io::register();
    crate::artifacts::semio::standards::v1::subsets::animation::io::register();
    crate::artifacts::semio::standards::v1::subsets::presentation::io::register();
    crate::artifacts::semio::standards::v1::subsets::flow::io::register();
    crate::artifacts::semio::standards::v1::subsets::text::io::register();
    crate::artifacts::semio::standards::v1::subsets::table::io::register();
    crate::artifacts::semio::standards::v1::subsets::graph::io::register();
    crate::artifacts::semio::standards::v1::subsets::object::io::register();
    crate::artifacts::semio::standards::v1::subsets::kit::io::register();
    crate::artifacts::semio::standards::v1::subsets::any::io::register();
}
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::SemioBrepComposer;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::SemioMeshComposer;
    use crate::artifacts::semio::standards::v1::subsets::model::schema::SemioModelComposer;
    use crate::artifacts::semio::standards::v1::subsets::value::schema::SemioValueComposer;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::SemioDocumentComposer;
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::SemioCadComposer;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::SemioDrawingComposer;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::SemioImageComposer;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::SemioVideoComposer;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::SemioAudioComposer;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::SemioAnimationComposer;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::SemioPresentationComposer;
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::SemioFlowComposer;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::SemioTextComposer;
    use crate::artifacts::semio::standards::v1::subsets::table::schema::SemioTableComposer;
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::SemioGraphComposer;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::SemioObjectComposer;
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::SemioKitComposer;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::SemioComposer as SemioRawAnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<SemioBrepComposer>(),
            composer_entry_of::<SemioMeshComposer>(),
            composer_entry_of::<SemioModelComposer>(),
            composer_entry_of::<SemioValueComposer>(),
            composer_entry_of::<SemioDocumentComposer>(),
            composer_entry_of::<SemioCadComposer>(),
            composer_entry_of::<SemioDrawingComposer>(),
            composer_entry_of::<SemioImageComposer>(),
            composer_entry_of::<SemioVideoComposer>(),
            composer_entry_of::<SemioAudioComposer>(),
            composer_entry_of::<SemioAnimationComposer>(),
            composer_entry_of::<SemioPresentationComposer>(),
            composer_entry_of::<SemioFlowComposer>(),
            composer_entry_of::<SemioTextComposer>(),
            composer_entry_of::<SemioTableComposer>(),
            composer_entry_of::<SemioGraphComposer>(),
            composer_entry_of::<SemioObjectComposer>(),
            composer_entry_of::<SemioKitComposer>(),
            composer_entry_of::<SemioRawAnyComposer>(),
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
