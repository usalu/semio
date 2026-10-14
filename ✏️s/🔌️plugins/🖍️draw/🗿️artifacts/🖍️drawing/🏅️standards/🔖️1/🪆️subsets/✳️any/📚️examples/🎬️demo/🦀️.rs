//! 📚️ Example `demo`.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🗄️ The parsed Semio emblem example document, or the empty drawing when the committed text does not parse.
pub fn semio_drawing_example_document() -> DrawingSnapshot {
    semio_s_artifact_draw_drawing_model::io::text::snapshot::parse_dsl(PRIMARY_TEXT).unwrap_or_else(|_| semio_s_artifact_draw_drawing_model::empty_drawing_snapshot())
}

/// 🌉️ JSON bridge for `semio_framework_plugin`'s `App::example`/`VcsArtifactApp::render` override, which reads JSON;
/// derived from the DSL example rather than a second copy of it on disk.
pub fn semio_drawing_example_json() -> String {
    semio_framework_pack_json::to_json_string(&semio_drawing_example_document())
}

//#region 🪢️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
