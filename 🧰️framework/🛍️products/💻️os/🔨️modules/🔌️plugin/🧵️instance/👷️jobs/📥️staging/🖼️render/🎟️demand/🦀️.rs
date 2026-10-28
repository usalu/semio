//! 🎟️ Every render action keeps its original independent work and physical currencies.
use semio_framework_value::{RetirementDemand,retained_clone::RetainedCloneGrant};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RenderReadiness{Enter,Pending,Refused,Cancelled}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
pub fn render_readiness(demand:RetirementDemand,remaining:RetainedCloneGrant,total:RetainedCloneGrant,cancelled:bool)->RenderReadiness{if cancelled{return RenderReadiness::Cancelled;}if !fits(demand,total){return RenderReadiness::Refused;}if fits(demand,remaining){RenderReadiness::Enter}else{RenderReadiness::Pending}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
