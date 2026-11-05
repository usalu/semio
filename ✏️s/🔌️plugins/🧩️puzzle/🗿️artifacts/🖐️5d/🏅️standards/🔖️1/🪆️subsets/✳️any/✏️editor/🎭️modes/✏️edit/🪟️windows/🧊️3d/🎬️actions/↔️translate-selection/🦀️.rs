//! ↔️ Selection translation action.

use semio_plugin::ActionRef;

/// ↔️ Returns the window-local selection translation action reference.
pub fn reference() -> ActionRef {
    "translateSelection".into()
}
