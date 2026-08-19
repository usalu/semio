//! 🧩️ Board-event dispatch action.

use semio_framework_plugin::ActionRef;

/// 🧩️ Returns the window-local board-event action reference.
pub async fn reference() -> ActionRef {
    "applyBoardEvents".into()
}
