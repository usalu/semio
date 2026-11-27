//! ⛓️ LAW (design §16.3): every document leaf payload schema of the sourcing curation editor resolves through the runtime resolver.
#[path = "../🧰️subject/🦀️.rs"]
mod subject;
use semio_plugin_testing_acceptance::block_on_acceptance;
use subject::{fixtures, manifest, Members, Mounted, PLUGIN};


pub fn history_edit_inputs_resolve() {
    let (leaves, withdraw_only) = block_on_acceptance(semio_plugin_testing_acceptance::assert_input_schemas_resolve::<Mounted, Members>(PLUGIN, manifest, &fixtures()));
    println!("[history-edit-inputs] {PLUGIN}: {leaves} leaf payload schema(s) resolve, {withdraw_only} withdraw-only leaf(s) not judged");
}
