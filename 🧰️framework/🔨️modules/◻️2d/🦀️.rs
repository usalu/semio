//! ◻️ Composes the framework's implemented 2D domains under their declared public owner.

pub use semio_framework_2d_engine::{DrawingError, PathSegment, Vec2};
pub use semio_framework_2d_retire::physical_work_retirement;
pub use semio_framework_2d_compute as compute;
pub use semio_framework_2d_retire as retirement;
pub use semio_framework_2d_path_flatten as flatten;
pub use semio_framework_2d_path_stroke as stroke;
pub use semio_framework_2d_booleans as booleans;
pub use semio_framework_2d_trace as trace;
pub use semio_framework_2d_regions as regions;
pub use semio_framework_2d_text as text;

#[cfg(test)]
#[path = "🧪️tests/📤️exports/🦀️.rs"]
mod export_contract;
