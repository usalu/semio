//! 🧯️ LAW: closing the sourcing curation editor mid undo, with the reserved route pending and with its commit queued, retires every commit's metadata.
#[test]
fn closes_mid_reserved_commit() {
    semio_s_artifact_sourcing_curation_history_close::closes_mid_reserved_commit();
}
