//! 🎹️ Mp3Composer (mpeg1-layer3 standard) — aggregates its subsets' composer entries
//! value-level (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::composer::Mp3Composer as Mp3RawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<Mp3RawAnyComposer>()]).as_slice()
}
