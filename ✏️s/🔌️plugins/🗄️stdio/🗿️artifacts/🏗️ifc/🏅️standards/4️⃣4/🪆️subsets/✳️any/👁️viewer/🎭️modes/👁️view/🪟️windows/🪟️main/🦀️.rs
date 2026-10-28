//! 👁️ IFC4 viewer main window uses the shared representation-derived scene.
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_framework_plugin::{BuiltNode, MeshWindowKit, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;

pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}

pub fn render(document: &IfcSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    crate::standards::v4::subsets::any::io::geometry::render(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
