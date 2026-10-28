//! 🏗️ IFC2x3 source representation geometry shared by editor and viewer.
use crate::standards::v2x3::subsets::sav::schema::snapshot::Ifc2x3Snapshot;
use crate::standards::v2x3::subsets::sav::schema::geometry;
#[cfg(test)]use semio_framework_plugin::{MeshWindowKit,WindowKit};
#[path="../../../../../../../🚪️io/📐️geometry/🦀️.rs"]
mod representation_surface;
pub use representation_surface::{render,render_in_context,render_with,render_selected_in_context};
pub const WINDOW_KIND_ID:&str=semio_framework_plugin::MeshWindowKit::KIND_ID;
pub const BODY_KEY:&str=semio_framework_plugin::MeshWindowKit::KIND_ID;
pub fn definition()->semio_framework_plugin::WindowKindDefinition{<semio_framework_plugin::MeshWindowKit as semio_framework_plugin::WindowKit>::window_kind()}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]mod tests;
