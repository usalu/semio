//! 🎹️ PptxComposer (ecma-376 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any, plus the real ISO/IEC 29500-1 Strict (✳️strict) and ISO/IEC 29500-4 Transitional
//! (✳️transitional) conformance-class subsets (ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::composer::PptxComposer as PptxRawAnyComposer;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::strict::composer::PptxStrictComposer;
use crate::artifacts::pptx::standards::v_ecma_376::subsets::transitional::composer::PptxTransitionalComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<PptxRawAnyComposer>(), composer_entry_of::<PptxStrictComposer>(), composer_entry_of::<PptxTransitionalComposer>()]).as_slice()
}
