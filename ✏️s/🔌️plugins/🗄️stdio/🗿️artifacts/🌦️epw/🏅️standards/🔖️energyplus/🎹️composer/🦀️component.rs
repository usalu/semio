//! 🎹️ EpwComposer (energyplus standard) — aggregates its subsets' composer entries
//! value-level (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::epw::standards::energyplus::subsets::any::composer::EpwComposer as EpwRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<EpwRawAnyComposer>()]).as_slice()
}
