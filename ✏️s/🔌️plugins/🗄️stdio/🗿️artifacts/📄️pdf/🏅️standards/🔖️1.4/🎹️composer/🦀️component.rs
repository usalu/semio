//! 🎹️ PdfComposer (1.4 standard) — aggregates its subsets' composer entries value-level: ✳️any,
//! plus the honestly-scope-limited ✳️a (ISO 19005-1) and ✳️x (ISO 15930-1/-3) real subsets added
//! in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2 -- see those subsets'
//! `🧐️analyzer` doc comments for why they're pass-through/schema-gapped rather than hard-gating
//! like 1.7's `✳️a`.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::pdf::standards::v1_4::subsets::any::composer::PdfComposer as PdfRawAnyComposer;
use crate::artifacts::pdf::standards::v1_4::subsets::a::composer::PdfAComposer;
use crate::artifacts::pdf::standards::v1_4::subsets::x::composer::PdfXComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<PdfRawAnyComposer>(), composer_entry_of::<PdfAComposer>(), composer_entry_of::<PdfXComposer>()]).as_slice()
}
