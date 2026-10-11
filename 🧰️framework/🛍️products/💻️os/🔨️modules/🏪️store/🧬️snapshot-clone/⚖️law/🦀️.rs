//! ⚖️ Law driver: runs any one-item preparation factory through the real canonical sealer to `Prepared` and closes it under exact quoted grants.
//!
//! Allocation receipts are checked against the allocator witness, so call it from test binaries that install `semio_framework_trace::HeapWitness`.
use crate::os_store::{ArtifactCanonicalJsonTree, ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPreparationStep, HistoryLane, SnapshotRead, SnapshotReadRegistryHandle};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::retirement::{OwnedValueRetirementFactory, RetireOwned, SharedValueRetirementFactory};
use std::sync::Arc;

/// ⚖️ What a lawful preparation published: the post snapshot, the ordered inverse rows and the cumulative physical cost.
pub struct OneItemPreparationLaw<P, M> {
    pub post: Arc<P>,
    pub inverse: Vec<M>,
    pub turns: usize,
    pub retained_capacity_bytes: usize,
    pub peak_live_capacity_bytes: usize,
}

/// ⚖️ Drives `mutation` over `snapshot` to `Prepared`, then closes the preparation with quoted grants only, asserting every receipt fits its grant and equals the observed allocation.
pub fn drive_one_item_preparation_law<P, M>(factory: &Arc<dyn ArtifactStoreOneItemPreparationFactory<P, M>>, snapshot: P, mutation: M, grant: ArtifactStoreOneItemGrant) -> OneItemPreparationLaw<P, M>
where
    P: RetireOwned + Send + Sync + 'static,
    M: RetireOwned + ArtifactCanonicalJsonTree + Clone + Send + Sync + 'static,
{
    let registry = SnapshotReadRegistryHandle::new();
    let root = Arc::new(snapshot);
    let lease = registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_| panic!("law registry admission"));
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority { operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(1), base_revision: [0; 32], base_applied_edit_count: 0, next_sequence_number: 1, next_clock: crate::os_spr::HybridLogicalTimestamp::new(1, 0), actor: "law-actor".into(), line: None, group_id: None, stamped_edit_id: None });
    let request = ArtifactStoreOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(1),
        generation: semio_framework_job::Generation(1),
        base_revision: [0; 32],
        lane: HistoryLane::Document,
        authority,
        base: SnapshotRead::new(Arc::clone(&root), lease),
        mutation,
        mutation_retirement: Arc::new(OwnedValueRetirementFactory::<M>::default()),
        snapshot_retirement: Arc::new(SharedValueRetirementFactory::<P>::default()),
    };
    let footprint = factory.preflight(&request.mutation, request.lane).unwrap_or_else(|error| panic!("law preflight refused: {error}"));
    assert!(footprint.is_admissible(), "law footprint {footprint:?} is not admissible");
    let (mut owner, birth) = match factory.begin(request, grant) {
        Ok(begun) => begun,
        Err((error, _)) => panic!("law preparation refused its birth: {error}"),
    };
    assert!(birth.fits(grant.retained_grant()));
    let (mut retained, mut idle, mut turns) = (birth.retained_capacity_bytes, 0usize, 0usize);
    let (mut live, mut peak) = (birth.retained_capacity_bytes, birth.retained_capacity_bytes);
    let prepared = loop {
        turns += 1;
        match owner.advance(grant).unwrap_or_else(|error| panic!("law preparation failed at turn {turns}: {error}")) {
            ArtifactStoreOneItemPreparationStep::Prepared(_, progress) => {
                assert!(progress.fits(grant.retained_grant()));
                retained += progress.retained_capacity_bytes;
                live += progress.retained_capacity_bytes;
                peak = peak.max(live);
                break owner.take_prepared().expect("a prepared step owns its prepared edit");
            }
            ArtifactStoreOneItemPreparationStep::Progress(_, progress) => {
                assert!(progress.fits(grant.retained_grant()), "law receipt {progress:?} exceeds {grant:?}");
                retained += progress.retained_capacity_bytes;
                live += progress.retained_capacity_bytes;
                peak = peak.max(live);
                live = live.saturating_sub(progress.released_bytes);
                idle = 0;
            }
            ArtifactStoreOneItemPreparationStep::Blocked => {
                idle += 1;
                assert!(idle < 64, "law preparation stalled under {grant:?}");
            }
        }
    };
    let report = OneItemPreparationLaw { post: Arc::clone(&prepared.post_snapshot), inverse: prepared.edit.inverse.iter().cloned().collect(), turns, retained_capacity_bytes: retained, peak_live_capacity_bytes: peak };
    drop(prepared);
    owner.begin_close();
    for _ in 0..100_000 {
        if owner.terminal_is_empty() {
            assert_eq!(observe(|| drop(owner)).1.requested_bytes, 0);
            return report;
        }
        let copy = owner.next_close_copy_byte_demand().unwrap();
        let quote = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_close_release_byte_demand().unwrap(), maximum_depth: owner.next_close_depth_demand().unwrap() };
        let (step, heap) = observe(|| owner.close_step(quote).unwrap());
        assert!(step.progress().fits(quote.retained_grant()), "law close receipt {:?} exceeds its quote {quote:?}", step.progress());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes), "law close accounting disagrees with its receipt");
    }
    panic!("law preparation did not close under its quoted grants");
}
