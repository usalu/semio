//! 🎹️ Puzzle5dComposer (1 standard) — aggregates its subsets' composer entries value-level.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::puzzle5d::standards::v1::subsets::any::composer::Puzzle5dComposer as Puzzle5dAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<Puzzle5dAnyComposer>()]).as_slice()
}
