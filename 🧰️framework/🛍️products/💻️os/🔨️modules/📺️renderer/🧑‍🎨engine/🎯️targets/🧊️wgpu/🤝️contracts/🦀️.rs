//! 🤝️ Cell and vocabulary types two or more elements of the wgpu renderer must both name and cannot own: worker cells, scene pointer targets, raster upload refusals, the framework widget context and the program fault.
//!
//! 🧭️ Native and wasm32, no first-party dependencies: types, constants, statics and pure accessors only. The element contracts module file keeps its place in the target tree; items lifted out of other packages arrive below `📥️lifted` and are mounted here: the program fault, the framework widget context, the retained UI engine cell with its close queue and pointer owners, the admitted surface map, the shell find items and the hub remote status.

#[cfg(test)]
extern crate infinite_canvas as infinite_world;
use std::sync::MutexGuard;
use ui_contract::SurfaceId;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_wgpu_widget_kit::{WidgetContext, WidgetInteractionMaps};

#[path = "../🧬️element-contracts/🦀️.rs"]
mod element_contracts;
pub use element_contracts::*;

#[path = "📥️lifted/🌉️ProgramBridge/🦀️.rs"]
mod program_fault;
pub use program_fault::*;

#[path = "📥️lifted/🖼️ui-image-loading/🦀️.rs"]
mod ui_image_loading;
pub use ui_image_loading::*;

#[path = "📥️lifted/🧩️retained-ui-engine/🦀️.rs"]
mod retained_ui_engine;
pub use retained_ui_engine::*;

#[path = "📥️lifted/🗃️scene-state/🦀️.rs"]
mod admitted_surfaces;
pub use admitted_surfaces::*;

#[path = "📥️lifted/🔖️identity-and-types/🦀️.rs"]
mod find_items;
pub use find_items::*;

#[path = "📥️lifted/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod hub_remote;
pub use hub_remote::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
