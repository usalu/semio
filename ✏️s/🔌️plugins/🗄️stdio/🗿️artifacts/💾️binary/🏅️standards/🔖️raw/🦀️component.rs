//! 🏅️ Standard root for `s.stdio.binary` standard `raw` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot). Exports
//! `standard() -> StandardDeclaration`, mounting subset `any` (the only subset this standard
//! has). Media values are the REAL `FormatDescriptor` registration for `stdio.binary`, taken
//! verbatim from `🧬️schema/📜️artifact-definition.json`'s `representations[0]`
//! (`mimes: ["application/octet-stream"], extensions: [".bin"]`) — not invented.

use crate::artifacts::binary::standards::v_raw::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

/// 🌳️ `standard "raw"`'s complete declaration — one subset, `any`.
pub async fn standard() -> StandardDeclaration {
    StandardDeclaration {
        id: StandardId("raw"),
        media: MediaDeclaration { mimes: &["application/octet-stream"], extensions: &["bin"] },
        subsets: vec![subsets::any::subset()],
    }
}
