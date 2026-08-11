//! 🎹️ XlsxComposer (ecma-376 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any (the flat ecma-376 read/write), ✳️strict (ISO/IEC 29500-1 Strict), and ✳️transitional
//! (ISO/IEC 29500-4 Transitional) — the latter two added in ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::composer::XlsxComposer as XlsxRawAnyComposer;
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::composer::XlsxStrictComposer;
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::composer::XlsxTransitionalComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES
        .get_or_init(|| vec![composer_entry_of::<XlsxRawAnyComposer>(), composer_entry_of::<XlsxStrictComposer>(), composer_entry_of::<XlsxTransitionalComposer>()])
        .as_slice()
}
