//! 🎹️ PdfComposer (1.7 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any (the flat 1.7 read/write), ✳️a (D5's PDF/A pilot, restructured from `✳️a-2b` in ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2 -- the first real, non-`✳️any` subset
//! in the whole repo), and W3's five remaining real vocabularies: ✳️x (PDF/X-4), ✳️e (PDF/E-1),
//! ✳️ua (PDF/UA-1), ✳️vt (PDF/VT-1/-2, layered on ✳️x), ✳️h (PDF/H, all-soft).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::pdf::standards::v1_7::subsets::any::composer::PdfComposer as PdfRawAnyComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::a::composer::PdfAComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::x::composer::PdfXComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::e::composer::PdfEComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::ua::composer::PdfUaComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::vt::composer::PdfVtComposer;
use crate::artifacts::pdf::standards::v1_7::subsets::h::composer::PdfHComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| {
        vec![
            composer_entry_of::<PdfRawAnyComposer>(),
            composer_entry_of::<PdfAComposer>(),
            composer_entry_of::<PdfXComposer>(),
            composer_entry_of::<PdfEComposer>(),
            composer_entry_of::<PdfUaComposer>(),
            composer_entry_of::<PdfVtComposer>(),
            composer_entry_of::<PdfHComposer>(),
        ]
    })
    .as_slice()
}
