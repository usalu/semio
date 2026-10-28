//! 🪪️ Singleton identity children adopt their prepared sealed branch through the original model identity.
use super::*;
use crate::snapshot::state::{StateSite,StateGroundTemperatureConfig,ModelIdentityState};
macro_rules! identity_output{($path:ident,$alias:ident,$child:ty,$field:ident)=>{
 pub struct $path;
 impl NestedBranchPath for $path{type Branch=$child;type Parent=ModelIdentityState;type Output=DirectBranchOutput<ModelIdentityState>;fn parent(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Parent>{o.project(0,|s|s.model.identity.get())}fn select(p:&mut Self::Parent)->&mut SealedShared<Self::Branch>{&mut p.$field}fn output()->Self::Output{DirectBranchOutput::new(|identity|{let mut d=EnergyModelDiff::default();d.model.identity=Some(identity);d})}}
 pub type $alias=NestedBranchOutput<$path>;
};}
identity_output!(SiteBranchParent,SiteBranchOutput,StateSite,site);
identity_output!(GroundBranchParent,GroundBranchOutput,StateGroundTemperatureConfig,ground_temperature);
identity_output!(RunPeriodBranchParent,RunPeriodBranchOutput,crate::calendar::RunPeriod,run_period);
