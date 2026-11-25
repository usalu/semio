//! 🛑️ LAW: a durable batch that holds Store-issued commit metadata and a staged causal seed (the custody the original
//! commit preflight leaves before its commit) closes through the batch abort port of its issuing Store
//! (`ArtifactStore::prepare_apply_batch_close`), never through the generic maintenance handoff (fixture
//! `🧫️fixtures/🛑️original-batch-abort`). The Store keeps such a publication with its driver, the generic close refuses it
//! until the port returned the metadata and the seed, the generic close then reaches terminal emptiness, and the document
//! keeps its generation, revision, history and snapshot exactly, Store maintenance holds nothing of it, and the next
//! gesture is admitted on the same frontier. The PDF picker law
//! (`registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit`) drives the same port through the real
//! original commit route of a runtime-hosted instance.

#![cfg(test)]
#![allow(unused_imports, ambiguous_glob_reexports)]

use semio_os_store_testing_prelude::*;
use semio_os_store_testing_fixtures::*;
use semio_os_store_testing_doubles::*;
use semio_os_store_testing_doubles::store::ArtifactStore;
use semio_os_store_testing_fixtures::demo::{DemoMutation, SetN};
use semio_framework_value::testing::physical_test_close_grant;
use semio_framework_value::{RetainedCloneStep, ValueError};
use semio_framework_value_testing_allocator as _;

/// 🏛️ Returns every unit of Store custody through the issuing Store, then hands the custody-free publication to the Store's own maintenance queue, which retires it with the Store; the custody units returned and whether the Store accepted the handoff.
fn close_through_store(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, mut publication: ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>) -> (usize, bool) {
    let grant = physical_test_close_grant();
    let mut returned = 0;
    while publication.holds_store_custody() {
        let step = semio_os_store_testing_doubles::store::closed_with(grant, |effect| store.prepare_apply_batch_close(&mut publication, effect).map(|progress| RetainedCloneStep::Progress(progress.unwrap_or_default())).map_err(Into::into)).expect("the issuing Store returns its original custody");
        assert!(step.progress().fits(grant), "one custody unit stays inside its close grant");
        returned += 1;
        assert!(returned < 4_096, "the issuing Store returns the publication's custody");
    }
    let mut driver = Some(publication);
    let accepted = store.handoff_batch_publication_close(&mut driver).expect("the Store answers the handoff");
    (returned, accepted && driver.is_none())
}

/// 🧺️ Begins one document gesture of `value` on the live frontier.
fn begin_gesture(store: &ArtifactStore<DemoSnapshot, DemoMutation>, operation: u64, value: i32) -> ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation> {
    let factory: std::sync::Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = std::sync::Arc::new(DemoOneItemPreparationFactory::admissible());
    store
        .begin_apply_batch(semio_framework_job::OperationId(operation), store.generation_now(), store.content_revision_now(), store.local_actor_id().0.clone(), vec![DemoMutation::SetN(SetN { n: value })], HistoryLane::Document, Some(&factory), None)
        .unwrap_or_else(|rejected| panic!("batch admission: {}", rejected.reason))
}

#[semio_framework_async_macros::async_test]
async fn an_original_batch_abandoned_after_its_commit_metadata_returns_its_store_custody_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛑️original-batch-abort/🔣️.json")).expect("original batch abort fixture");
    assert_eq!(fixture["schema"], "semio.store.original-batch-abort/v1");
    let value = |key: &str| i32::try_from(fixture[key].as_i64().expect("fixture value")).expect("demo value");
    let initial = DemoSnapshot { n: Some(value("initial")) };
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "original-batch-abort", initial.clone(), None, &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error, _)| panic!("funded original catalog install refused: {error}"));
    store.closes_on_drop();
    let (generation, revision, edits, tail) = (store.generation_now(), store.content_revision_now(), store.envelope().vcs.edits.len(), store.applied_edit_ids().last().cloned());

    let mut publication = begin_gesture(&store, 1, value("abandoned"));
    let authority = publication.authority.clone().expect("admitted batch authority");
    let mut stage = semio_os_store_testing_prelude::ArtifactStore::<DemoSnapshot, DemoMutation>::empty_batch_stage(&authority);
    stage.commit_metadata = Some(semio_os_store_one_item_batch::lifted_member_group::BatchCommitMetadata::new(None, None));
    stage.seed_pending = Some(MutationId(format!("{}-seed", fixture["schema"].as_str().expect("fixture schema"))));
    publication.stage = Some(Box::new(stage));
    assert!(publication.holds_store_custody(), "the staged batch holds the Store's commit metadata and causal seed");
    let generic_refused = publication.stage.as_ref().expect("staged custody").retirement_demands(physical_test_close_grant().maximum_copy_bytes).is_err();
    let mut driver = Some(publication);
    let handoff_refused = !store.handoff_batch_publication_close(&mut driver).expect("the Store answers the handoff");
    let mut publication = driver.expect("a refused handoff leaves the publication with its driver");
    publication.abandon(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "the document changed before the commit"));
    let (returned, handed_off) = close_through_store(&mut store, publication);
    let unchanged = (store.generation_now() == generation, store.content_revision_now() == revision, store.envelope().vcs.edits.len() == edits && store.applied_edit_ids().last().cloned() == tail, store.snapshot_ref() == &initial);

    let mut next = begin_gesture(&store, 2, value("next"));
    let next_admitted = next.expected_generation == generation && next.expected_revision == revision;
    next.abandon(ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled, "the law only admits the next gesture"));
    close_through_store(&mut store, next);
    let maintenance_queued = !store.maintenance_retirements_terminal_is_empty();

    let actual = serde_json::json!({
        "handoffRefused": handoff_refused,
        "genericCloseRefused": generic_refused,
        "custodyReturned": returned > 0,
        "handedOffWithoutCustody": handed_off,
        "generationUnchanged": unchanged.0,
        "revisionUnchanged": unchanged.1,
        "historyUnchanged": unchanged.2,
        "snapshotUnchanged": unchanged.3,
        "maintenanceQueued": maintenance_queued,
        "nextAdmitted": next_admitted,
    });
    assert_eq!(actual, fixture["expect"]);
}
