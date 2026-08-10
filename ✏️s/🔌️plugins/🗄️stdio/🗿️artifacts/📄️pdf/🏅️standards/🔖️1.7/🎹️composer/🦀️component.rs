//! 🎹️ PdfComposer (1.7 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any (the flat 1.7 read/write) and ✳️a-2b (D5's PDF/A-2b pilot -- the first real, non-`✳️any`
//! subset in the whole repo).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::pdf::standards::v1_7::subsets::any::composer::PdfComposer as PdfRawAnyComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::a2b::composer::PdfA2bComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<PdfRawAnyComposer>(), composer_entry_of::<PdfA2bComposer>()]).as_slice()
}
