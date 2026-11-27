//! 🧰️ The one subject of every history acceptance target: the sourcing curation editor, its member set, its manifest and the artifact tree its fixtures live in.
pub use semio_s_artifact_sourcing_curation::editor::sourcing::SourcingCurationApp as Editor;
use semio_plugin::{App, ArtifactEditor, EditorApp};
use semio_s_artifact_sourcing_curation::editor::sourcing::create_sourcing_curation_app;

pub const PLUGIN: &str = "sourcing";

pub type Mounted = EditorApp<Editor>;

pub type Members = <Editor as ArtifactEditor>::Members;

pub fn manifest() -> App {
    App { definition: create_sourcing_curation_app(), examples: Vec::new() }
}

pub fn fixtures() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../..")
}
