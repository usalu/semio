//! ⏮️ LAW (design §16.3): a representative editable leaf of the sourcing curation editor is edited in history end to end.
#[path = "../🧰️subject/🦀️.rs"]
mod subject;
use semio_plugin_testing_acceptance::block_on_acceptance;
use subject::{fixtures, manifest, Editor, Members, Mounted, PLUGIN};
use semio_plugin::ArtifactEditor;


pub fn history_edits_end_to_end() {
    let summary = block_on_acceptance(semio_plugin_testing_acceptance::assert_history_edits_end_to_end::<Mounted, Members>(PLUGIN, manifest, &fixtures(), &Editor::examples()));
    println!("[history-edit-acceptance] {summary}");
}
