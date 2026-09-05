//! ⌨️ Terminal UI target glue.

#[path = "../../../../⌨️tui/🦀️.rs"]
mod component;

#[path = "../../../../🧱️elements/🪙️Chip/🎯️targets/⌨️tui/🦀️.rs"]
pub mod chip;
#[path = "../../../../🧱️elements/➖️Divider/🎯️targets/⌨️tui/🦀️.rs"]
pub mod divider;
#[path = "../../../../🧱️elements/🔚️Footer/🎯️targets/⌨️tui/🦀️.rs"]
pub mod footer;
#[path = "../../../../🧱️elements/✏️Input/🎯️targets/⌨️tui/🦀️.rs"]
pub mod input;
#[path = "../../../../🧱️elements/🏷️Label/🎯️targets/⌨️tui/🦀️.rs"]
pub mod label;
#[path = "../../../../🧱️elements/📃️List/🎯️targets/⌨️tui/🦀️.rs"]
pub mod list;
#[path = "../../../../🧱️elements/🪵️Log/🎯️targets/⌨️tui/🦀️.rs"]
pub mod log;
#[path = "../../../../🧱️elements/🔝️Navbar/🎯️targets/⌨️tui/🦀️.rs"]
pub mod navbar;
#[path = "../../../../🧱️elements/🔽️Select/🎯️targets/⌨️tui/🦀️.rs"]
pub mod select;
#[path = "../../../../🧱️elements/📊️Table/🎯️targets/⌨️tui/🦀️.rs"]
pub mod table;
#[path = "../../../../🧱️elements/📑️Tabs/🎯️targets/⌨️tui/🦀️.rs"]
pub mod tabs;
#[path = "../../../../🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs"]
pub mod window;
#[path = "../../../../🧱️elements/🧙️Wizard/🎯️targets/⌨️tui/🦀️.rs"]
pub mod wizard;

pub use component::*;
