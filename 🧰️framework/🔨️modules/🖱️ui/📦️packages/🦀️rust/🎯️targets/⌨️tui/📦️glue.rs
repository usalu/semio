//! ⌨️ Terminal UI target glue.

#[path = "../../../../⌨️tui/🦀️component.rs"]
mod component;

#[path = "../../../../🧱️elements/🏷️Chip/⌨️component.rs"]
pub mod chip;
#[path = "../../../../🧱️elements/➖️Divider/⌨️component.rs"]
pub mod divider;
#[path = "../../../../🧱️elements/🔚️Footer/⌨️component.rs"]
pub mod footer;
#[path = "../../../../🧱️elements/✏️Input/⌨️component.rs"]
pub mod input;
#[path = "../../../../🧱️elements/🏷️Label/⌨️component.rs"]
pub mod label;
#[path = "../../../../🧱️elements/📃️List/⌨️component.rs"]
pub mod list;
#[path = "../../../../🧱️elements/🪵️Log/⌨️component.rs"]
pub mod log;
#[path = "../../../../🧱️elements/🔝️Navbar/⌨️component.rs"]
pub mod navbar;
#[path = "../../../../🧱️elements/☑️Select/⌨️component.rs"]
pub mod select;
#[path = "../../../../🧱️elements/📊️Table/⌨️component.rs"]
pub mod table;
#[path = "../../../../🧱️elements/📑️Tabs/⌨️component.rs"]
pub mod tabs;
#[path = "../../../../🧱️elements/🪟️Window/⌨️component.rs"]
pub mod window;
#[path = "../../../../🧱️elements/🧙️Wizard/⌨️component.rs"]
pub mod wizard;

pub use component::*;
