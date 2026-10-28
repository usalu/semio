//! 🔢️ Scalar row policies use the shared original semantic leaf owner.
use super::*;
use crate::leaf::{RowFieldPreparation,RowLeafSpec,RowLeafPreparation};
use protocol::mutation::retained::RetainedDiagnosticSpec;
/// 🏷️ Each authored schema field supplies its native address and typed inverse, without an erased payload map.
pub trait ScalarLeafSpec<L,M>:ScalarRowPath{
 const INVALID_REFERENCE:protocol::mutation::retained::RetainedDiagnosticSpec=Self::INVALID;
 const INVALID:RetainedDiagnosticSpec;
 const UNCHANGED:RetainedDiagnosticSpec;
 const MISSING:RetainedDiagnosticSpec;
 fn input(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,Self::Scalar>;
 fn key(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,u32>;
 fn inverse(payload:RetainedCloneRef<'_,L>,value:Self::Scalar)->M;
 fn reference_target(payload:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,u32>{Self::key(payload)}
}


/// 🧭️ Scalar recipe policy keeps its copied inverse type while sharing the diagnostic and factory owner.
pub struct ScalarSpec<S>(std::marker::PhantomData<fn()->S>);
impl<L,M,S:ScalarLeafSpec<L,M>> RowLeafSpec<L,M> for ScalarSpec<S>{type Input=S::Scalar;type Inverse=S::Scalar;type Core=ScalarRowPreparation<S>;const INVALID:RetainedDiagnosticSpec=S::INVALID;const INVALID_REFERENCE:RetainedDiagnosticSpec=S::INVALID_REFERENCE;const UNCHANGED:RetainedDiagnosticSpec=S::UNCHANGED;const MISSING:RetainedDiagnosticSpec=S::MISSING;fn input(p:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,Self::Input>{S::input(p)}fn key(p:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,u32>>{Some(S::key(p))}fn inverse(p:RetainedCloneRef<'_,L>,v:Self::Inverse)->M{S::inverse(p,v)}fn diagnostic_targets(p:RetainedCloneRef<'_,L>,o:ScalarOutcome)->Result<[protocol::mutation::retained::RetainedDiagnosticTarget<'_>;1],ValueError>{Ok([protocol::mutation::retained::RetainedDiagnosticTarget::Unsigned(if o==ScalarOutcome::InvalidReference{S::reference_target(p)}else{S::key(p)})])}}
pub type ScalarLeafPreparation<L,M,S>=RowLeafPreparation<L,M,ScalarSpec<S>>;
impl<S:ScalarRowPath> RowFieldPreparation for ScalarRowPreparation<S>{
 type Input=S::Scalar;type Inverse=S::Scalar;
 fn new()->Self{Self::new()}
 fn check(&self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{Self::check(self,v,k.ok_or_else(invalid)?,o)}
 fn demand(&self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize)->Result<RetirementDemand,ValueError>{Self::demand(self,v,k.ok_or_else(invalid)?,o,b)}
 fn advance(&mut self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::advance(self,v,k.ok_or_else(invalid)?,o,b,g)}
 fn ready(&self)->bool{Self::ready(self)}fn outcome(&self)->Option<ScalarOutcome>{Self::outcome(self)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Self::take_demand(self)}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{Self::take(self,g)}
 fn inverse_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Self::inverse_demand(self).map(|_|scalar(1))}
 fn prepare_inverse(&mut self,_:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=scalar(1);Ok(if retained_mutation_demand_fits(d,g){RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(Default::default())})}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.inverse.is_some()&&!self.inverse_taken}
 fn take_inverse_demand(&self)->Result<RetirementDemand,ValueError>{Self::inverse_demand(self)}
 fn take_inverse(&mut self,g:RetainedCloneGrant)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{Self::take_inverse(self,g)}
 fn begin_close(&mut self){Self::begin_close(self)}
 fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{Self::close_demand(self,b)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
}
