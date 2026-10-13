//! ⚖️ Window transient preparation law: a transient owner's own bundle prepares a mutation, prepares its inverse back to the base, prepares it again, and retires its state and mutation through the bundle's own controlled retirement factories.
//!
//! Mounted by `#[path]` into integration test binaries of artifact crates that own a `WindowTransientOwner`.

use semio_framework_os_kernel::{self as store, os_spr::Mutation as _};
use semio_framework_plugin::{WindowTransientOwner, WindowTransientOwnerBundle};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use std::{fmt::Debug, sync::Arc};

fn grant() -> store::ArtifactStoreOneItemGrant {
    store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 }
}

fn prepare<P: Send + Sync + 'static, M: Send + 'static>(label: &str, bundle: &WindowTransientOwnerBundle<P, M>, base: &Arc<P>, mutation: M) -> Arc<P> {
    let request = store::ArtifactEphemeralOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(1),
        generation: semio_framework_job::Generation(0),
        base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(base.clone())),
        mutation,
    };
    let mut preparation = match bundle.preparation.begin(request) {
        Ok(preparation) => preparation,
        Err(_) => panic!("{label}: the bundle's preparation factory admits the mutation"),
    };
    let mut prepared = false;
    for _ in 0..1usize << 16 {
        match preparation.advance(grant()).unwrap_or_else(|error| panic!("{label}: a granted turn prepares: {error}")) {
            store::ArtifactStoreOneItemPreparationStep::Prepared(..) => { prepared = true; break; }
            store::ArtifactStoreOneItemPreparationStep::Progress(..) | store::ArtifactStoreOneItemPreparationStep::Blocked => {}
        }
    }
    assert!(prepared, "{label}: the preparation reaches its prepared root");
    let root = preparation.take_prepared().unwrap_or_else(|| panic!("{label}: the prepared root is taken exactly once")).next_root;
    preparation.begin_close();
    for _ in 0..1usize << 16 {
        if preparation.terminal_is_empty() { break; }
        let copy = preparation.next_close_copy_byte_demand().unwrap_or_else(|error| panic!("{label}: close copy demand: {error}")).max(1);
        let turn = store::ArtifactStoreOneItemGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: preparation.next_close_capacity_byte_demand(copy).unwrap_or_else(|error| panic!("{label}: close capacity demand: {error}")),
            maximum_release_bytes: preparation.next_close_release_byte_demand().unwrap_or_else(|error| panic!("{label}: close release demand: {error}")),
            maximum_depth: preparation.next_close_depth_demand().unwrap_or_else(|error| panic!("{label}: close depth demand: {error}")).max(1),
        };
        preparation.close_step(turn).unwrap_or_else(|error| panic!("{label}: a quoted close turn closes: {error}"));
    }
    assert!(preparation.terminal_is_empty(), "{label}: the preparation closes to its terminal-empty witness");
    root
}

fn retire<T: Send + 'static>(label: &str, factory: &Arc<dyn store::ArtifactOwnedValueRetirementFactory<T>>, value: T) {
    let capacity = factory.retirement_birth_bytes(&value);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: capacity, maximum_release_bytes: 1 << 20, maximum_depth: 4096 };
    let (mut owner, _) = match factory.retire_owned(value, grant) {
        Ok(admitted) => admitted,
        Err((error, _)) => panic!("{label}: the bundle's controlled retirement admits the owner: {error}"),
    };
    store::test_support::drive_retirement(owner.as_mut()).unwrap_or_else(|error| panic!("{label}: the owner retires to its terminal-empty witness: {error}"));
}

/// 🔁️ Applies `mutation` to `base`, undoes it through its inverse rows, redoes it, and retires every owner through the bundle.
pub fn transient_preparation_law<O: WindowTransientOwner>(label: &str, base: O::State, mutation: O::Mutation, expected: O::State)
where
    O::State: Clone + PartialEq + Debug,
    O::Mutation: Clone + Debug + store::os_spr::Mutation<O::State>,
{
    let bundle = O::build_owners();
    assert_ne!(base, expected, "{label}: the mutation must change the state");
    let base_root = Arc::new(base.clone());
    let applied = prepare(label, &bundle, &base_root, mutation.clone());
    assert_eq!(*applied, expected, "{label}: Apply prepares the exact post state");
    let inverse = mutation.inverse(&base).unwrap_or_else(|error| panic!("{label}: the mutation inverts: {error}"));
    let mut current = applied.clone();
    for row in inverse.into_iter().rev() {
        current = prepare(label, &bundle, &current, row);
    }
    assert_eq!(*current, base, "{label}: Undo restores the base");
    let redone = prepare(label, &bundle, &current, mutation.clone());
    assert_eq!(*redone, expected, "{label}: Redo prepares the same post state");
    for root in [base_root, applied, current, redone] {
        retire(label, &bundle.state_retirement, Arc::try_unwrap(root).unwrap_or_else(|_| panic!("{label}: the state has one owner left")));
    }
    retire(label, &bundle.mutation_retirement, mutation);
}
