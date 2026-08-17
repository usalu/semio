//! 🏅️ Standard root for `s.stdio.txt` standard `utf-8` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot). Exports
//! `standard() -> StandardDeclaration`, mounting subset `any` (the only subset this standard
//! has). Media values are the REAL `FormatDescriptor` registration for `stdio.txt`, taken
//! verbatim from `🧬️schema/📜️artifact-definition.json`'s `representations[0]`
//! (`mimes: ["text/plain"], extensions: [".txt"]`) — not invented.

use crate::artifacts::txt::standards::v_utf_8::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🌳️ `standard "utf-8"`'s complete declaration — one subset, `any`.
pub fn standard() -> StandardDeclaration {
    StandardDeclaration {
        id: StandardId("utf-8"),
        media: MediaDeclaration { mimes: &["text/plain"], extensions: &["txt"] },
        subsets: vec![subsets::any::subset()],
    }
}
