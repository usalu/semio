//! ⚔️ LAW (design §23.2): a conflict a history edit of the sourcing curation editor causes downstream is classified, named and resolved end to end.
#[path = "../🧰️subject/🦀️.rs"]
mod subject;
use semio_plugin_testing_acceptance::block_on_acceptance;
use subject::{fixtures, manifest, Editor, Members, Mounted, PLUGIN};
use semio_plugin::ArtifactEditor;


pub fn conflict_history_edits_end_to_end() {
    let summary = block_on_acceptance(semio_plugin_testing_acceptance::assert_history_conflicts_resolve::<Mounted, Members>(PLUGIN, manifest, &fixtures(), &Editor::examples()));
    println!("[history-edit-cascade] {summary}");
}
