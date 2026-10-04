//! 📚️ Example `demo`.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🪪️ The child id the bundled demo parent names for its `content` graph child.
pub const CONTENT_CHILD_ID: &str = "demo";

/// 🕸️ The bundled demo graph — the content of the demo's `content` child, derived for the genesis member store.
pub fn scene() -> crate::DagScene {
    let graph = <semio_framework_artifact_infinite_dag::DagSnapshot as store::ArtifactDsl>::parse_dsl(include_str!("🖼️assets/🕸️graph.dsl.semio")).expect("bundled graph document parses");
    crate::DagScene { nodes: graph.nodes, edges: graph.edges }
}

/// 📸️ The bundled demo parent document; its `content` child is the derivable [`scene`].
pub fn snapshot() -> crate::DagSnapshot {
    <crate::DagSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("bundled literal parent document parses")
}
