//! ⚖️ Law driver: runs any one-item preparation factory through the real canonical sealer to `Prepared` and closes it under exact quoted grants.
//!
//! Allocation receipts are checked against the allocator witness, so call it from test binaries that install `semio_framework_trace::HeapWitness`.
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_os_store_one_item::{ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest};
use semio_framework_os_store_command::HistoryLane;
use semio_framework_value_snapshot_read::{SnapshotRead, SnapshotReadRegistryHandle};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::factory::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use semio_framework_value::retirement::RetireOwned;
use std::sync::Arc;
use semio_framework_value::{NativeDecodeConstructionRecipient,NativeDecodeProgress,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
struct LawConstruction { original:RetainedCloneGrant, spent:RetainedCloneProgress }
impl NativeDecodeConstructionRecipient for LawConstruction {
 fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{if !semio_framework_value::retained_clone::retained_clone_grant_admits(self.original,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"law original construction denied"));}Ok(self.original)}
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{if !p.fits(g){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"law original construction receipt changed"));}self.spent=self.spent.checked_add(p)?;if !self.spent.fits(self.original){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"law original construction turn exhausted"));}Ok(())}
}

/// ⚖️ What a lawful preparation published: the post snapshot, the ordered inverse rows and the cumulative physical cost.
pub struct OneItemPreparationLaw<P, M> {
    pub post: Arc<P>,
    pub inverse: Vec<M>,
    pub turns: usize,
    pub close_turns: usize,
    pub retained_capacity_bytes: usize,
    pub peak_live_capacity_bytes: usize,
}

/// 🪪️ Issues a real preparation request while lending the test an alias of the exact original snapshot for ownership observations.
pub fn one_item_preparation_law_request<P, M>(snapshot: P, mutation: M) -> (Arc<P>, ArtifactStoreOneItemPreparationRequest<P, M, M>)
where
    P: RetireOwned + Send + Sync + 'static,
    M: RetireOwned + Send + Sync + 'static,
{
    let registry = SnapshotReadRegistryHandle::new();
    let root = Arc::new(snapshot);
    let lease = registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_| panic!("law registry admission"));
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), [0; 32], 0, 1, semio_framework_os_spr::HybridLogicalTimestamp::new(1, 0), "law-actor".into(), None, None, None));
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
    (root, request)
}

/// ⚖️ Drives `mutation` over `snapshot` to `Prepared`, then closes the preparation with quoted grants only, asserting every receipt fits its grant and equals the observed allocation.
pub fn drive_one_item_preparation_law<P, M>(factory: &Arc<dyn ArtifactStoreOneItemPreparationFactory<P, M>>, snapshot: P, mutation: M, body:usize, grant: ArtifactStoreOneItemGrant) -> OneItemPreparationLaw<P, M>
where
    P: RetireOwned + Send + Sync + 'static,
    M: RetireOwned + ArtifactCanonicalJsonTree + Clone + Send + Sync + 'static,
{
    let (_root, request) = one_item_preparation_law_request(snapshot, mutation);
    let footprint = factory.preflight(&request.mutation, request.lane).unwrap_or_else(|error| panic!("law preflight refused: {error}"));
    assert!(footprint.is_admissible(), "law footprint {footprint:?} is not admissible");
    let (mut owner, birth) = match factory.begin(request, grant) {
        Ok(begun) => begun,
        Err((error, _)) => panic!("law preparation refused its birth: {error}"),
    };
    assert!(birth.fits(grant.retained_grant()));
    let (mut retained, mut idle, mut turns) = (birth.retained_capacity_bytes, 0usize, 0usize);
    let (mut live, mut peak) = (birth.retained_capacity_bytes, birth.retained_capacity_bytes);
    let mut receiving=semio_framework_job::OriginalJobDecodeOwner::empty();
    let cancelled=false;
    let mut observer=|_:NativeDecodeProgress|!cancelled;
    let prepared = loop {
        turns += 1;
        let mut construction=LawConstruction{original:grant.retained_grant(),spent:Default::default()};
        let step=receiving.receive(grant.retained_grant(),&mut observer,&mut construction,|control|owner.advance(body,grant,control)).unwrap_or_else(|error| panic!("law preparation failed at turn {turns}: {error}"));
        let progress=match &step{ArtifactStoreOneItemPreparationStep::Prepared(_,p)|ArtifactStoreOneItemPreparationStep::Progress(_,p)=>*p,ArtifactStoreOneItemPreparationStep::Blocked=>Default::default()};
        assert!(progress.copied_bytes>=construction.spent.copied_bytes&&progress.retained_capacity_bytes>=construction.spent.retained_capacity_bytes&&progress.released_bytes>=construction.spent.released_bytes,"law enclosing receipt lost its original construction");
        match step {
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
    let report = OneItemPreparationLaw { post: Arc::clone(&prepared.post_snapshot()), inverse: prepared.edit().inverse.iter().cloned().collect(), turns, close_turns: 0, retained_capacity_bytes: retained, peak_live_capacity_bytes: peak };
    drop(prepared);
    owner.begin_close();
    let mut report = report;
    for close_turns in 0..100_000 {
        if owner.terminal_is_empty() {
            assert_eq!(observe(|| drop(owner)).1.requested_bytes, 0);
            while !receiving.terminal_is_empty(){let d=receiving.retirement_demands(grant.maximum_copy_bytes).unwrap();let original=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};let(step,heap)=observe(||receiving.close_step(original).unwrap());assert!(step.progress().fits(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
            report.close_turns = close_turns;
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

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
