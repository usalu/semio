//! ⚖️ Typed-command input retirement law: the gate every typed command payload passes before its original owner is retired under quoted grants.
//!
//! Mounted by `#[path]` into integration test binaries that install `semio_framework_trace::HeapWitness` as their global allocator.

use semio_framework_os_kernel as store;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{
    retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep},
    retirement::RetireOwned,
};

/// 📜️ What one owner cost to retire through the typed-command input lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedInputLaw {
    pub turns: usize,
    pub declared_births: usize,
    pub declared_releases: usize,
}

/// 🎟️ Admits `value` through `artifact_retirement_admit_owned` and drives its erased owner to the terminal-empty witness, granting each axis exactly its quoted demand.
pub fn typed_input_retirement_law<T: RetireOwned>(label: &str, value: T) -> TypedInputLaw {
    assert!(T::controlled_retirement_supported(), "{label}: the owner declares controlled retirement");
    assert!(value.retirement_birth_bytes().is_some(), "{label}: the owner quotes its exact birth");
    let mut pending = Some(value);
    let mut active: Option<Box<dyn store::ErasedSnapshotRetirement>> = None;
    let demand = store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&pending).unwrap_or_else(|error| panic!("{label}: the typed lane quotes the owner: {error}"));
    let admit = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 1 << 20, maximum_depth: demand.depth };
    let (admitted, heap) = observe(|| store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut pending, &mut active, admit));
    let admitted = admitted.unwrap_or_else(|error| panic!("{label}: the typed lane admits the owner: {error}"));
    let RetainedCloneStep::Progress(progress) = admitted else { panic!("{label}: admission reports its birth as progress") };
    assert!(pending.is_none(), "{label}: admission takes the original owner");
    assert_eq!(heap.requested_bytes, progress.retained_capacity_bytes, "{label}: the admitted frame is exactly its quoted birth");
    let mut owner = active.take().unwrap_or_else(|| panic!("{label}: admission installs the erased owner"));
    let mut law = TypedInputLaw { turns: 0, declared_births: progress.retained_capacity_bytes, declared_releases: progress.released_bytes };
    for _ in 0..1usize << 20 {
        if owner.terminal_is_empty() { break; }
        let copy = owner.next_copy_byte_demand().unwrap_or_else(|error| panic!("{label}: copy demand: {error}")).max(store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap_or_else(|error| panic!("{label}: capacity demand: {error}")),
            maximum_release_bytes: owner.next_release_byte_demand().unwrap_or_else(|error| panic!("{label}: release demand: {error}")),
            maximum_depth: owner.next_depth_demand().unwrap_or_else(|error| panic!("{label}: depth demand: {error}")).max(1),
        };
        let (step, heap) = observe(|| owner.close_step(grant));
        let step = step.unwrap_or_else(|error| panic!("{label}: a fully quoted turn closes: {error}"));
        admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "typed input law").unwrap_or_else(|error| panic!("{label}: the receipt fits its quoted grant: {error}"));
        let progress: RetainedCloneProgress = step.progress();
        assert_eq!(heap.requested_bytes, progress.retained_capacity_bytes, "{label}: turn {} allocates exactly its declared births", law.turns);
        assert!(heap.released_bytes >= progress.released_bytes, "{label}: turn {} releases at least its declared bytes", law.turns);
        law.turns += 1;
        law.declared_births += progress.retained_capacity_bytes;
        law.declared_releases += progress.released_bytes;
    }
    assert!(owner.terminal_is_empty(), "{label}: the owner reaches its terminal-empty witness");
    drop(owner);
    law
}

/// 📨️ Retires the exact typed command a retained command job holds for `mutation`.
pub fn typed_command_retirement_law<M>(label: &str, mutation: M) -> TypedInputLaw
where
    M: RetireOwned + Clone,
{
    typed_input_retirement_law(label, store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None })
}
