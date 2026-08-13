//! 🚚️ World relocation action.

use semio_framework_plugin::ActionRef;

/// 🚚️ Returns the window-local world relocation action reference.
pub fn reference() -> ActionRef {
    "worldRelocate".into()
}
