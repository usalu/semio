//! 🧭️ Ordered views into one retained original reachability scope.

use super::*;
use semio_framework_3d_brep_engine::TopologyAlias;
use semio_framework_value::RetirementDemand;

impl Session {
    pub fn original_topology_kind(&self, handle: &str) -> Result<&'static str, ValueError> {
        self.with_original_geometry_query(|kernel| kernel.original_topology_kind(handle).map(kind_label))
    }

    pub fn plan_original_topology_reach(&self) -> Result<NativeTopologyReach, ValueError> {
        self.with_original_geometry_query(|kernel| Ok(NativeTopologyReach::new(kernel)))
    }

    pub fn original_topology_reach_demand(&self, reach: &NativeTopologyReach, handle: &str, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.with_original_geometry_query(|kernel| reach.demand(kernel, handle, copy))
    }

    pub fn step_original_topology_reach(&self, reach: &mut NativeTopologyReach, handle: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
        self.with_original_geometry_query(|kernel| reach.step(kernel, handle, effect))
    }

    pub fn reached_topology_depth(&self, reach: &NativeTopologyReach, kind: ReachedTopology, rank: usize) -> Result<usize, ValueError> {
        self.with_original_geometry_query(|kernel| reach.member_depth(kernel, kind, rank))
    }

    pub fn reached_topology_alias(&self, reach: &NativeTopologyReach, kind: ReachedTopology, rank: usize) -> Result<TopologyAlias, ValueError> {
        self.with_original_geometry_query(|kernel| reach.member_alias(kernel, kind, rank))
    }
}
