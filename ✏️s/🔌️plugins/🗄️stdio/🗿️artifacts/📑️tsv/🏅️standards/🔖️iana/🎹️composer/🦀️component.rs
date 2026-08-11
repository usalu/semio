//! 🎹️ TsvComposer (iana standard) — aggregates its subsets' composer entries
//! value-level (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::tsv::standards::iana::subsets::any::composer::TsvComposer as TsvRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<TsvRawAnyComposer>()]).as_slice()
}
