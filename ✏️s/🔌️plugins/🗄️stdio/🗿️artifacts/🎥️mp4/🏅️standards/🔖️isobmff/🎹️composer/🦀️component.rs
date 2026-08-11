//! 🎹️ Mp4Composer (isobmff standard) — aggregates its subsets' composer entries
//! value-level (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::mp4::standards::isobmff::subsets::any::composer::Mp4Composer as Mp4RawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<Mp4RawAnyComposer>()]).as_slice()
}
