//! ↔️ Selection translation action.

use semio_framework_plugin::ActionRef;

/// ↔️ Returns the window-local selection translation action reference.
pub async fn reference() -> ActionRef {
    "translateSelection".into()
}
