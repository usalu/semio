//! 🎹️ SvgComposer (1.1 standard) — aggregates its subsets' composer entries value-level: ✳️any
//! (the flat 1.1 read/write), ✳️tiny (SVG Tiny 1.1), and ✳️basic (SVG Basic 1.1) -- W3C Mobile SVG
//! Profiles REC-SVGMobile-20030114, ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::svg::standards::v1_1::subsets::any::composer::SvgComposer as SvgRawAnyComposer;
use crate::artifacts::svg::standards::v1_1::subsets::tiny::composer::SvgTinyComposer;
use crate::artifacts::svg::standards::v1_1::subsets::basic::composer::SvgBasicComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<SvgRawAnyComposer>(), composer_entry_of::<SvgTinyComposer>(), composer_entry_of::<SvgBasicComposer>()]).as_slice()
}
