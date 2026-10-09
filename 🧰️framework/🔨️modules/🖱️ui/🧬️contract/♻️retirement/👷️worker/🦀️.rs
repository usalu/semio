//! 👷️ Fixed caller policy for original UI worker retirement.
use semio_framework_value::{ValueRefusalKind,RetirementDemand,retained_clone::RetainedCloneGrant};
pub const UI_WORKER_RETIREMENT_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
/// 🛡️ Refuses an oversized frontier without advancing or enlarging its caller policy.
pub fn ui_worker_retirement_permits(demand:RetirementDemand)->bool{
 demand.copy_bytes<=UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes&&demand.capacity_bytes<=UI_WORKER_RETIREMENT_POLICY.maximum_capacity_bytes&&demand.release_bytes<=UI_WORKER_RETIREMENT_POLICY.maximum_release_bytes&&demand.depth<=UI_WORKER_RETIREMENT_POLICY.maximum_depth
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 🚥️ Publishes structural denial without spending any original owner currency.
pub fn ui_worker_retirement_admission(demand:RetirementDemand)->Result<(),ValueRefusalKind>{
 if demand.depth>UI_WORKER_RETIREMENT_POLICY.maximum_depth{return Err(ValueRefusalKind::DepthLimit)}
 if demand.capacity_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_capacity_bytes||demand.release_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_release_bytes{return Err(ValueRefusalKind::OwnershipLimit)}
 if demand.copy_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes{return Err(ValueRefusalKind::WorkLimit)}
 Ok(())
}
