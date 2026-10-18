//! 👁️ Lift file: 2 item(s), about 9 lines, of `📜️declaration/👁️view/🦀️.rs` that belong to crate `operation` (WP1).
//!
//! WP2 owns the source file and cuts the items here (ArtifactOperationProgress, CANCEL_TYPED_OPERATION_ACTION_ID); WP1 wires this file with one `mod` line in the crate root. Until `operation` exists, the source module re-exports the items with `pub use`.
use semio_framework_ui_locale::LocalizedLabel;

#[derive(Clone, Debug)]
pub struct ArtifactOperationProgress {
    pub operation_id: u64,
    pub generation: u64,
    pub label: LocalizedLabel,
    pub completed_units: u64,
    pub cancelling: bool,
}

pub const CANCEL_TYPED_OPERATION_ACTION_ID: &str = "cancelTypedOperation";
