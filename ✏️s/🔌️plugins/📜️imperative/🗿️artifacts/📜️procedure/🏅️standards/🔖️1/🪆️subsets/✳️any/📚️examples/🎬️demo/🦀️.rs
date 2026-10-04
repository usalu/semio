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

/// 🪪️ The child ids the bundled demo parent names for its `flow` and `text` children.
pub const FLOW_CHILD_ID: &str = "demo-flow";
pub const TEXT_CHILD_ID: &str = "demo-text";

/// 🕸️ The bundled demo program (two steps, empty seed) — the content of the demo's children, derived at genesis.
pub fn scene() -> crate::ProcedureScene {
    crate::ProcedureScene { path: crate::schema::default_path(), seed: Default::default() }
}

/// 📸️ The bundled demo parent document; its children are the derivable [`scene`].
pub fn snapshot() -> crate::ProcedureSnapshot {
    <crate::ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("bundled literal parent document parses")
}
