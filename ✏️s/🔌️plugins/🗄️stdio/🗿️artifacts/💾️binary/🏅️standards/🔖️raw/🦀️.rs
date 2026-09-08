//! 🏅️ Standard root for `s.stdio.binary` standard `raw` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot). Exports
//! `standard() -> StandardDeclaration`, mounting subset `any` (the only subset this standard
//! has). Media values are the REAL `FormatDescriptor` registration for `stdio.binary`, taken
//! verbatim from `📜️artifact-definition.json`'s `representations[0]`
//! (`mimes: ["application/octet-stream"], extensions: [".bin"]`) — not invented.

#[cfg(feature = "component-app-assembly")]
use crate::standards::v_raw::subsets;
#[cfg(feature = "component-app-assembly")]
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
#[cfg(feature = "component-app-assembly")]
use semio_framework_plugin::StandardId;

/// 🌳️ `standard "raw"`'s complete declaration — one subset, `any`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "component-app-assembly")]
pub fn standard() -> StandardDeclaration<crate::BinaryApps> {
    StandardDeclaration { id: StandardId("raw"), media: MediaDeclaration { mimes: &["application/octet-stream"], extensions: &["bin"] }, subsets: vec![subsets::any::subset()] }
}
