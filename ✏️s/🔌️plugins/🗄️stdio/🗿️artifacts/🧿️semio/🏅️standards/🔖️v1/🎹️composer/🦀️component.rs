//! 🎹️ SemioComposer (v1 standard) — aggregates ALL 14 subsets' composer entries value-level
//! (13 domain subsets + the ✳️any envelope). Type-erased via `composer_entry_of`, so the
//! aggregation doesn't need every subset to share a Snapshot type.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::semio::standards::v1::subsets::brep::composer::SemioBrepComposer;
use crate::artifacts::semio::standards::v1::subsets::mesh::composer::SemioMeshComposer;
use crate::artifacts::semio::standards::v1::subsets::model::composer::SemioModelComposer;
use crate::artifacts::semio::standards::v1::subsets::object::composer::SemioObjectComposer;
use crate::artifacts::semio::standards::v1::subsets::document::composer::SemioDocumentComposer;
use crate::artifacts::semio::standards::v1::subsets::cad::composer::SemioCadComposer;
use crate::artifacts::semio::standards::v1::subsets::drawing::composer::SemioDrawingComposer;
use crate::artifacts::semio::standards::v1::subsets::image::composer::SemioImageComposer;
use crate::artifacts::semio::standards::v1::subsets::video::composer::SemioVideoComposer;
use crate::artifacts::semio::standards::v1::subsets::audio::composer::SemioAudioComposer;
use crate::artifacts::semio::standards::v1::subsets::animation::composer::SemioAnimationComposer;
use crate::artifacts::semio::standards::v1::subsets::presentation::composer::SemioPresentationComposer;
use crate::artifacts::semio::standards::v1::subsets::workflow::composer::SemioWorkflowComposer;
use crate::artifacts::semio::standards::v1::subsets::any::composer::SemioComposer as SemioRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![
        composer_entry_of::<SemioBrepComposer>(),
        composer_entry_of::<SemioMeshComposer>(),
        composer_entry_of::<SemioModelComposer>(),
        composer_entry_of::<SemioObjectComposer>(),
        composer_entry_of::<SemioDocumentComposer>(),
        composer_entry_of::<SemioCadComposer>(),
        composer_entry_of::<SemioDrawingComposer>(),
        composer_entry_of::<SemioImageComposer>(),
        composer_entry_of::<SemioVideoComposer>(),
        composer_entry_of::<SemioAudioComposer>(),
        composer_entry_of::<SemioAnimationComposer>(),
        composer_entry_of::<SemioPresentationComposer>(),
        composer_entry_of::<SemioWorkflowComposer>(),
        composer_entry_of::<SemioRawAnyComposer>(),
    ]).as_slice()
}
