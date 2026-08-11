//! 🎹️ Ifc2x3Composer (2x3 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any (the flat 2x3 read/write) plus the three real conformance subsets (✳️cv20 Coordination
//! View 2.0, ✳️sav Structural Analysis View, ✳️cobie Basic FM Handover / COBie).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::ifc::standards::v2x3::subsets::any::composer::Ifc2x3Composer as Ifc2x3RawAnyComposer;
use crate::artifacts::ifc::standards::v2x3::subsets::cv20::composer::Ifc2x3Cv20Composer;
use crate::artifacts::ifc::standards::v2x3::subsets::sav::composer::Ifc2x3SavComposer;
use crate::artifacts::ifc::standards::v2x3::subsets::cobie::composer::Ifc2x3CobieComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES
        .get_or_init(|| {
            vec![
                composer_entry_of::<Ifc2x3RawAnyComposer>(),
                composer_entry_of::<Ifc2x3Cv20Composer>(),
                composer_entry_of::<Ifc2x3SavComposer>(),
                composer_entry_of::<Ifc2x3CobieComposer>(),
            ]
        })
        .as_slice()
}
