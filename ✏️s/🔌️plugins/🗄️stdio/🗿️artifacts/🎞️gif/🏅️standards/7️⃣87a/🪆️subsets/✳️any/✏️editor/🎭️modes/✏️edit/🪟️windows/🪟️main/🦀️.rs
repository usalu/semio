//! ✏️ `gif` edit (any) — Main window: real `ImageWindowKit`
//! render of the current document (editable native canvas with per-frame region painting; typed edits live in Details).

use crate::standards::v87a::subsets::any::io::encode_gif;
use crate::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
use semio_plugin::app::{ImageView, ImageWindowKit};
use semio_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    let mut definition = ImageWindowKit::editable_window_kind();
    definition.actions.push(crate::editor::gif_87a::paint_region::action());
    definition
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &GifSnapshot) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    ImageWindowKit::render(&image_view(snapshot))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &GifSnapshot) -> ImageView {
    let bytes = encode_gif(snapshot).ok().unwrap_or_default();
    ImageView { width: snapshot.width, height: snapshot.height, mime: "image/gif".into(), base64: crate::base64_standard(&bytes) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
