//! 🎹️ BinaryComposer (raw standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today; a future second subset appends here without touching the
//! artifact-level composer).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::binary::standards::v_raw::subsets::any::composer::BinaryComposer as BinaryRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

/// 🎹️ Every composer entry this standard can serve.
pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<BinaryRawAnyComposer>()]).as_slice()
}
