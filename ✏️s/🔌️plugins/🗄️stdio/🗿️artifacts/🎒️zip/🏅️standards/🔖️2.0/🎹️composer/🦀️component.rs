//! 🎹️ ZipComposer (2.0 standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::zip::standards::v2_0::subsets::any::composer::ZipComposer as ZipRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<ZipRawAnyComposer>()]).as_slice()
}
