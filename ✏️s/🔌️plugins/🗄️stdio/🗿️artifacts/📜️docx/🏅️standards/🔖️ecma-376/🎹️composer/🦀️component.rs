//! 🎹️ DocxComposer (ecma-376 standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::composer::DocxComposer as DocxRawAnyComposer;
use crate::artifacts::docx::standards::v_ecma_376::subsets::strict::composer::DocxStrictComposer;
use crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::composer::DocxTransitionalComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES
        .get_or_init(|| {
            vec![
                composer_entry_of::<DocxRawAnyComposer>(),
                composer_entry_of::<DocxStrictComposer>(),
                composer_entry_of::<DocxTransitionalComposer>(),
            ]
        })
        .as_slice()
}
