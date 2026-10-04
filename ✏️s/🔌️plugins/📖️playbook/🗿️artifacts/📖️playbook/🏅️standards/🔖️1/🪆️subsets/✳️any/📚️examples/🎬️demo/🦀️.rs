//! 📚️ Example `demo`.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
/// 🪪️ The `flow` child the demo's parent names (design §20.15: the parent carries the coordinate, never the steps).
pub const FLOW_ID: &str = "playbook-demo-flow";
/// 🌊️ The demo's steps in their own format: the `s.stdio.semio@v1/flow` text of its `flow` child.
pub const FLOW_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🌊️flow/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🌊️ The demo's `flow` child content, the genesis of [`FLOW_ID`] (`crate::playbook_genesis_flow`).
pub fn flow() -> Result<semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot, String> {
    <semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot as store::ArtifactDsl>::parse_dsl(FLOW_TEXT).map_err(|error| error.to_string())
}
