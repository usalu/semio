//! 🧯️ LAW: closing the sourcing curation editor mid undo, with the reserved route pending and with its commit queued, retires every commit's metadata.
#[path = "../🧰️subject/🦀️.rs"]
mod subject;
use semio_plugin_testing_acceptance::block_on_acceptance;
use subject::{fixtures, manifest, Members, Mounted, PLUGIN};


pub fn closes_mid_reserved_commit() {
    block_on_acceptance(semio_plugin_testing_acceptance::assert_closes_mid_reserved_commit::<Mounted, Members>(PLUGIN, manifest, &fixtures()));
}
