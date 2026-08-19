//! 📐️ Selection scaling action.

use semio_framework_plugin::ActionRef;

/// 📐️ Returns the window-local selection scaling action reference.
pub async fn reference() -> ActionRef {
    "scaleSelection".into()
}
