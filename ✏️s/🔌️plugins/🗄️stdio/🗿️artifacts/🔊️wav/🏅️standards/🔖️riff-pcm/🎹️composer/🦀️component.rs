//! 🎹️ WavComposer (riff-pcm standard) — aggregates its subsets' composer entries
//! value-level (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::wav::standards::riff_pcm::subsets::any::composer::WavComposer as WavRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<WavRawAnyComposer>()]).as_slice()
}
