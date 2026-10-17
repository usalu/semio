//! ✍️ Editor owns projection text and typed adornments; application document authority stays
//! with the composing application.

use canvas::camera::{Camera, Viewport};
use canvas::text as canvas_text;
pub use canvas::{self, *};
use serde::Deserialize;
use semio_framework_value_derive::{FromValue, ToValue};

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use js_sys::Function;

#[path = "🧬️scene/🦀️.rs"]
pub mod scene;
pub use scene::EditorScene;

#[path = "⚠️error/🦀️.rs"]
mod editor_error;
pub use editor_error::{EditorError, EditorErrorKind};

fn parse_editor_json(text: &str) -> Result<serde_json::Value, EditorError> {
    serde_json::from_str(text).map_err(|error| EditorError::from_cause(EditorErrorKind::Json, error))
}

#[path = "🎨️canvas-theme/🦀️.rs"]
mod canvas_theme;
use canvas_theme::*;

#[path = "📐️viewport-and-settings-json/🦀️.rs"]
mod viewport_and_settings_json;
use viewport_and_settings_json::*;

#[path = "🖊️host-state/🦀️.rs"]
mod host_state;
pub use host_state::*;

#[path = "⌨️host-impl/🦀️.rs"]
mod host_impl;

#[path = "🔤️text-geometry/🦀️.rs"]
mod text_geometry;
use text_geometry::*;

// #region 🔖️Wasm
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this session bridge is browser-only
// (attaches an `HtmlCanvasElement`), so it is narrowed to exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_async::browser::future_to_promise;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use std::cell::RefCell;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use std::rc::Rc;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use web_sys::HtmlCanvasElement;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🌐️wasm-session/🦀️.rs"]
mod wasm_session;
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use wasm_session::*;

// #endregion 🔖️Wasm

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/⌨️text-input/🦀️.rs"]
mod text_input_laws;
// #endregion 🔖️Tests
