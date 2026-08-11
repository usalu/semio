//! 🎹️ ZipComposer (2.0 standard) — aggregates its subsets' composer entries value-level
//! (✳️any, plus real subset ✳️iso21320).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::zip::standards::v2_0::subsets::any::composer::ZipComposer as ZipRawAnyComposer;
use crate::artifacts::zip::standards::v2_0::subsets::iso21320::composer::ZipIso21320Composer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<ZipRawAnyComposer>(), composer_entry_of::<ZipIso21320Composer>()]).as_slice()
}
