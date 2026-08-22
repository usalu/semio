//! 📷️ 2D camera action.

use semio_framework_plugin::ActionRef;

/// 📷️ Returns the window-local 2D camera action reference.
pub fn reference() -> ActionRef {
    "setCamera2d".into()
}
