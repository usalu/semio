//! 🧪️ `commit-reconstruction` fixture — `⭐️rejects`.
//!
//! Source of truth is the committed JSON quintet of the fixture mirror, read at test runtime (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). `commit-reconstruction-sparse` is this vector's scenario id in
//! `../../../../🧪️tests/📸️mutate-remodeling-1/🥒️.feature`, where the same bytes are replayed against
//! this subset's independent Python reference.
//!
//! 🏞️ a sparse content handle naming content no `append-content` published is refused

/// ⚖️ The vector holds the law set its declared outcome selects, its inverse restores the before-document and its committed JSON is canonical.
#[test]
fn holds_its_laws() {
    crate::vectors::assert_case("🏁commit-reconstruction", "⭐️rejects");
}

/// ⚖️ An APPLIED commit over this scenario's base (every result lane cleared) inverts to the negative diff: the inverse rows sum to it and replaying them restores the base.
#[semio_framework_async_macros::async_test]
async fn an_applied_commit_inverts_to_the_negative_diff() {
    let commit = semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation::from(semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction { sparse: None, trajectory: None, mesh: None, geo: None, qc: None, assets: Vec::new() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&commit, &crate::vectors::Vector::load("🏁commit-reconstruction", "⭐️rejects").before()).await;
}
