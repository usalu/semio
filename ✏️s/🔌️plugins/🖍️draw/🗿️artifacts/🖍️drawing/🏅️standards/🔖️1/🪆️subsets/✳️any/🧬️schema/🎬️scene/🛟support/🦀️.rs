//! 🛟 Raster, flatten and scene errors live outside the scene source boundary.
#[path = "⚠️error.rs"]
pub mod error;
#[path = "../📷️raster/🦀️.rs"]
pub mod raster;
#[path = "📽flatten.rs"]
mod flatten;
pub use flatten::{flatten_drawing_document_to_scene_nodes, flatten_drawing_document_with_transformation};
