//! 🔄️ Selection rotation action.

use semio_framework_plugin::ActionRef;

/// 🔄️ Returns the window-local selection rotation action reference.
pub async fn reference() -> ActionRef {
    "rotateSelection".into()
}
