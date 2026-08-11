//! 🎹️ JpgComposer (jfif-1.01 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any (the flat jfif-1.01 read/write) and ✳️baseline (ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES's real ITU-T T.81 / ISO 10918-1 Annex F
//! baseline-sequential-DCT conformance subset).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::composer::JpgComposer as JpgRawAnyComposer;
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::composer::JpgBaselineComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<JpgRawAnyComposer>(), composer_entry_of::<JpgBaselineComposer>()]).as_slice()
}
