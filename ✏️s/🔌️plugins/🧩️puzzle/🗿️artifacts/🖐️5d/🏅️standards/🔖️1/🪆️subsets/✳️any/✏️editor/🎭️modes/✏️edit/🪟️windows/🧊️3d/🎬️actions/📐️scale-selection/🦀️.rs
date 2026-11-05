//! 📐️ Selection scaling action.

use semio_plugin::ActionRef;

/// 📐️ Returns the window-local selection scaling action reference.
pub fn reference() -> ActionRef {
    "scaleSelection".into()
}
