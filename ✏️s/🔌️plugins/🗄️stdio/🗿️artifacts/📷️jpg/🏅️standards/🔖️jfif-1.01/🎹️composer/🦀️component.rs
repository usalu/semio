//! 🎹️ JpgComposer (jfif-1.01 standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::composer::JpgComposer as JpgRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<JpgRawAnyComposer>()]).as_slice()
}
