//! 🪈️ Staged slice of the `child-test-view` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🧪️ Fixture seam observes actual child publication and retained store owners.
use super::*;

impl<A: ArtifactApp> InstanceStores<A> {
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_parent_store_mut(&mut self)->&mut store::ArtifactStore<A::Snapshot,A::Mutation>{&mut self.store}
}
