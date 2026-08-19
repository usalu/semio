//! 📷️ 3D camera action.

use semio_framework_plugin::ActionRef;

/// 📷️ Returns the window-local 3D camera action reference.
pub async fn reference() -> ActionRef {
    "setCamera3d".into()
}
