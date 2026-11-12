//! 🔢️ Scalar row policies use the shared original semantic leaf owner.
use super::*;
use crate::leaf::{RowFieldPreparation,RowLeafSpec,RowLeafPreparation};
use protocol::RetainedDiagnosticSpec;
use crate::retained::{Done,Reply};
use semio_framework_value::RetainedAllocationEffect;
/// 🏷️ Each authored schema field supplies its native address and typed inverse, without an erased payload map.
pub trait ScalarLeafSpec<L,M>:ScalarRowPath{
 const INVALID_REFERENCE:protocol::RetainedDiagnosticSpec=Self::INVALID;
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
impl<L,M,S:ScalarLeafSpec<L,M>> RowLeafSpec<L,M> for ScalarSpec<S>{type Input=S::Scalar;type Inverse=S::Scalar;type Core=ScalarRowPreparation<S>;const INVALID:RetainedDiagnosticSpec=S::INVALID;const INVALID_REFERENCE:RetainedDiagnosticSpec=S::INVALID_REFERENCE;const UNCHANGED:RetainedDiagnosticSpec=S::UNCHANGED;const MISSING:RetainedDiagnosticSpec=S::MISSING;fn input(p:RetainedCloneRef<'_,L>)->RetainedCloneRef<'_,Self::Input>{S::input(p)}fn key(p:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,u32>>{Some(S::key(p))}fn inverse(p:RetainedCloneRef<'_,L>,v:Self::Inverse)->M{S::inverse(p,v)}fn diagnostic_targets(p:RetainedCloneRef<'_,L>,o:ScalarOutcome)->Result<[protocol::RetainedDiagnosticTarget<'_>;1],ValueError>{Ok([protocol::RetainedDiagnosticTarget::Unsigned(if o==ScalarOutcome::InvalidReference{S::reference_target(p)}else{S::key(p)})])}}
pub type ScalarLeafPreparation<L,M,S>=RowLeafPreparation<L,M,ScalarSpec<S>>;
impl<S:ScalarRowPath> RowFieldPreparation for ScalarRowPreparation<S>{
 type Input=S::Scalar;type Inverse=S::Scalar;
 fn new()->Self{Self::new()}
 fn check(&self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{Self::check(self,v,k.ok_or_else(invalid)?,o)}
 fn quote(&self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{Self::quote(self,v,k.ok_or_else(invalid)?,o,b,m)}
 fn advance_reply<'r>(&mut self,v:RetainedCloneRef<'_,Self::Input>,k:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{Self::advance_reply(self,v,k.ok_or_else(invalid)?,o,b,m,e,r,recover)}
 fn ready(&self)->bool{Self::ready(self)}fn outcome(&self)->Option<ScalarOutcome>{Self::outcome(self)}
 fn take_quote(&self)->Result<Quote,ValueError>{Self::take_quote(self)}
 fn take(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{Self::take(self,e)}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{Self::inverse_quote(self).map(|_|(scalar(1),1))}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.inverse.is_some()&&!self.inverse_taken}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{Self::inverse_quote(self)}
 fn take_inverse(&mut self,e:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{Self::take_inverse(self,e)}
}
