//! 🎹️ DeflateComposer (rfc1950 standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::composer::DeflateComposer as DeflateRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<DeflateRawAnyComposer>()]).as_slice()
}
