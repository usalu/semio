//! 🎹️ TiffComposer (6.0 standard) — aggregates its subsets' composer entries value-level: ✳️any,
//! plus the honestly-scope-limited ✳️baseline (Adobe TIFF 6.0 Part 1 "Baseline TIFF") real subset
//! added in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3 -- see that subset's
//! `🧐️analyzer` doc comment for why it's schema-gapped/pass-through.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::tiff::standards::v6_0::subsets::any::composer::TiffComposer as TiffRawAnyComposer;
use crate::artifacts::tiff::standards::v6_0::subsets::baseline::composer::TiffBaselineComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<TiffRawAnyComposer>(), composer_entry_of::<TiffBaselineComposer>()]).as_slice()
}
