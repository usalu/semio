//! ⏮️ The history acceptance law of the sourcing curation editor: a representative edit is undone, redone and replayed end to end.
pub use semio_s_artifact_sourcing_curation::*;
fn sourcing_manifest_for_tests() -> semio_plugin::App {
    semio_plugin::App { definition: editor::sourcing::create_sourcing_curation_app(), examples: Vec::new() }
}
semio_plugin::history_edit_acceptance_law!("sourcing", editor::sourcing::SourcingCurationApp, sourcing_manifest_for_tests, "../..");
