//! 🏗️ Ergonomic semantic builders (`ui::stack()`, `ui::button()`) — wasip2-safe, no engine.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! ⚠️ Decision (flagged for the coordinator, detailed in `📓️terra-contract-builder-report.md`):
//! [`BuiltNode`] duplicates the shape the sibling `semio-framework-ui-runtime` crate's own
//! `ComponentTree` needs, because this crate must not depend on that crate (the dependency runs the
//! other way — see `🦀️.rs`). The runtime converts a `BuiltNode` tree into its own reconciler
//! input; if the two shapes drift, that conversion is the one place to fix, never a second builder.

use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::{LazyLock, Mutex};

use serde::{Deserialize,Serialize};
pub use semio_framework_ui_contract_components::*;
use semio_framework_ui_contract_values::retirement_protocol as typed;
#[path="♻️retirement/🦀️.rs"]
mod tree_retirement;
pub use tree_retirement::{BuiltTreeRetirement,OriginalBuiltTreeRetirement};

#[path = "♻️retirement/📋️children/🦀️.rs"]
mod built_retirement;
pub(crate) use built_retirement::BuiltChildRetirementNext;

//#region 🔖️Builder

//#region 🧱️BuiltNode

#[path = "🧒️built-children-and-retirement/🦀️.rs"]
mod built_children_and_retirement;
use built_children_and_retirement::{credited_bindings, BUILT_CHILD_RETIRE_AUTHORITY};
pub use built_children_and_retirement::{
    built_node_pages_are_terminal_empty, close_built_node_page_one, BuiltChildren,
    BuiltChildrenIntoIter,
};

#[path = "🧱️built-node-and-base/🦀️.rs"]
mod built_node_and_base;
pub use built_node_and_base::{BuiltNode, NodeBase};
use built_node_and_base::{assemble, positional_key};

#[path = "🔧️builder-traits/🦀️.rs"]
mod builder_traits;
pub use builder_traits::{Buildable, HasBase, HasChildren, HasStackLayout, RetainedChildren};

#[path = "📚️stack-container-text-and-button-builders/🦀️.rs"]
mod stack_container_text_and_button_builders;
pub use stack_container_text_and_button_builders::{
    button, column, field, row, section, stack, text, ButtonBuilder, ContainerBuilder, StackBuilder,
    TextBuilder,
};

#[path = "⌨️input-toggle-and-select-builders/🦀️.rs"]
mod input_toggle_and_select_builders;
pub use input_toggle_and_select_builders::{
    input, select, toggle, InputBuilder, SelectBuilder, ToggleBuilder,
};

#[path = "🎚️icon-select-slider-stepper-and-progress-builders/🦀️.rs"]
mod icon_select_slider_stepper_and_progress_builders;
pub use icon_select_slider_stepper_and_progress_builders::{
    icon_select, number_stepper, progress, slider, IconSelectBuilder, NumberStepperBuilder,
    ProgressBuilder, SliderBuilder,
};

#[path = "🌲️tree-builders/🦀️.rs"]
mod tree_builders;
pub use tree_builders::{
    tree, tree_item, tree_section, TreeBuilder, TreeItemBuilder, TreeSectionBuilder,
};

#[path = "📊️table-image-surface-and-extension-builders/🦀️.rs"]
mod table_image_surface_and_extension_builders;
pub use table_image_surface_and_extension_builders::{
    extension, image, surface, table, table_row, ExtensionBuilder, HasAlt, ImageBuilder, NoAlt,
    SurfaceBuilder, TableBuilder, TableRowBuilder,
};

#[path = "🧭️vector-reference-and-colour-input-builders/🦀️.rs"]
mod vector_reference_and_colour_input_builders;
pub use vector_reference_and_colour_input_builders::{
    color_input, reference_list, vector_input, ColorInputBuilder, ColorInputLabels,
    ReferenceListBuilder, ReferenceListLabels, VectorInputBuilder,
};

use semio_framework_ui_contract_budget::{UI_BUILT_CHILDREN_MAX,UI_BUILT_CHILD_RETIRE_SLOTS};

//#endregion 🧱️BuiltNode

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#endregion 🔖️Builder
