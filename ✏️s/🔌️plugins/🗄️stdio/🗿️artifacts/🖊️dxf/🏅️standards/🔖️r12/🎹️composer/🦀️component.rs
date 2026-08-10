//! 🎹️ DxfComposer (r12 standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::dxf::standards::v_r12::subsets::any::composer::DxfComposer as DxfRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<DxfRawAnyComposer>()]).as_slice()
}
