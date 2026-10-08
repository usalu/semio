//! 🎨️ Cohesive module for the direct OS UI-preference mutation leaves.

#[path = "../🌗️set-appearance/🦀️.rs"]
pub mod set_appearance;
#[path = "../🚗️set-custom-driver/🦀️.rs"]
pub mod set_custom_driver;
#[path = "../🎨️set-custom-theme/🦀️.rs"]
pub mod set_custom_theme;
#[path = "../🕹️set-driver/🦀️.rs"]
pub mod set_driver;
#[path = "../⌨️set-keybinding-override/🦀️.rs"]
pub mod set_keybinding_override;
#[path = "../📐️set-layout/🦀️.rs"]
pub mod set_layout;
#[path = "../🗣️set-locale/🦀️.rs"]
pub mod set_locale;
#[path = "../🗂️set-named-layout/🦀️.rs"]
pub mod set_named_layout;
#[path = "../📖️set-terminology/🦀️.rs"]
pub mod set_terminology;
#[path = "../🖼️set-theme/🦀️.rs"]
pub mod set_theme;

pub use set_appearance::*;
pub use set_custom_driver::*;
pub use set_custom_theme::*;
pub use set_driver::*;
pub use set_keybinding_override::*;
pub use set_layout::*;
pub use set_locale::*;
pub use set_named_layout::*;
pub use set_terminology::*;
pub use set_theme::*;

#[cfg(test)]
#[path = "🧪️tests/🎨️updates-every-os-ui-preference/🦀️.rs"]
mod tests_updates_every_os_ui_preference;
