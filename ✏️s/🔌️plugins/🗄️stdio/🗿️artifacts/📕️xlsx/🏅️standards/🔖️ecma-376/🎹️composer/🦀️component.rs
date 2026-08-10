//! 🎹️ XlsxComposer (ecma-376 standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::composer::XlsxComposer as XlsxRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<XlsxRawAnyComposer>()]).as_slice()
}
