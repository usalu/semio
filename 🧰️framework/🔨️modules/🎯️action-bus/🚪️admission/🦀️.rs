//! 🚪️ Original tool factories borrow native construction authority and preserve caller custody through refusal.
use super::RetainedToolWireInput;
use semio_framework_job::{Generation, InteractiveJob, Operation, OperationId, OriginalTurnFailure, StepContext};
use semio_framework_job_payload::{OriginalPayloadSourceIdentity};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

/// 📏️ One concrete factory declares the physical effects of its next original construction turn.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ToolJobAdmissionDemand {pub items:usize,pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}
impl ToolJobAdmissionDemand {
    /// 📦️ A new typed frame adopts the original payload value and releases its original Box backing.
    pub fn boxed_job<P,J>()->Self{Self{items:1,copy_bytes:0,capacity_bytes:size_of::<J>(),release_bytes:size_of::<P>(),depth:1}}
    /// 🔗️ A new typed frame and shared payload allocate their actual declared native layouts.
    pub fn shared_payload<P,J>()->Result<Self,ValueError>{let mut demand=Self::boxed_job::<P,J>();demand.capacity_bytes=demand.capacity_bytes.checked_add(semio_framework_value::retirement::shared::arc_bytes::<P>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool factory allocation extent overflow"))?;Ok(demand)}
    /// 🎒️ An already born job changes custody without allocating or releasing a replacement frame.
    pub fn reused_job()->Self{Self{items:1,copy_bytes:0,capacity_bytes:0,release_bytes:0,depth:1}}
    /// 🎟️ Every native axis must fit the actual original remaining turn before any slot transfers.
    pub fn fits(self,grant:RetainedCloneGrant)->bool{self.items==1&&self.depth>0&&self.items<=grant.maximum_items&&self.copy_bytes<=grant.maximum_copy_bytes&&self.capacity_bytes<=grant.maximum_capacity_bytes&&self.release_bytes<=grant.maximum_release_bytes&&self.depth<=grant.maximum_depth}
    fn physical(self)->RetirementDemand{RetirementDemand{copy_bytes:self.copy_bytes,capacity_bytes:self.capacity_bytes,release_bytes:self.release_bytes,depth:self.depth}}
    fn progress(self)->RetainedCloneProgress{RetainedCloneProgress{copied_items:self.items,copied_bytes:self.copy_bytes,retained_capacity_bytes:self.capacity_bytes,released_bytes:self.release_bytes}}
}

pub use semio_framework_job::OriginalSourceBinding as OriginalToolSourceBinding;
/// 🪪️ Construction receives an identity issued only inside the same admitted original source turn.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalToolJobIdentity{source:OriginalToolSourceBinding}
impl OriginalToolJobIdentity{
 pub fn operation(self)->OperationId{self.source.operation()}
 pub fn generation(self)->Generation{self.source.generation()}
 pub fn matches(self,context:&StepContext<'_>)->bool{self.source.matches(context)}
 pub fn source_binding(self)->OriginalToolSourceBinding{self.source}
}

/// 🫴️ Original slots transfer only inside the original decoder's admitted construction callback.
pub fn admit_original_tool_job<P:Send+'static,J:InteractiveJob+Send+'static>(
    context:&mut StepContext<'_>,operation:&Operation,payload:&mut Option<Box<P>>,
    input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<J>>,
    demand:ToolJobAdmissionDemand,
    construct:impl FnOnce(OriginalToolJobIdentity,Box<P>,Option<RetainedToolWireInput>,Option<RetainedToolWireInput>)->Box<J>,
)->Result<bool,OriginalTurnFailure>{
    admit_original_tool_job_measured(context,operation,payload,input,checkpoint,output,demand,|identity,payload,input,checkpoint|(construct(identity,payload,input,checkpoint),demand.progress()))
}

/// 🧮️ Actual construction effects accompany retained output before original receipt acceptance.
pub fn admit_original_tool_job_measured<P:Send+'static,J:InteractiveJob+Send+'static>(
    context:&mut StepContext<'_>,operation:&Operation,payload:&mut Option<Box<P>>,
    input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<J>>,
    demand:ToolJobAdmissionDemand,
    construct:impl FnOnce(OriginalToolJobIdentity,Box<P>,Option<RetainedToolWireInput>,Option<RetainedToolWireInput>)->(Box<J>,RetainedCloneProgress),
)->Result<bool,OriginalTurnFailure>{
    admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|identity,payload,input,checkpoint|Ok(construct(identity,payload.take().expect("admitted original payload"),input.take(),checkpoint.take())))
}

/// 🔐️ Fallible source preparation retains every original slot before a construction candidate exists.
pub fn admit_original_tool_job_controlled<P:Send+'static,J:InteractiveJob+Send+'static>(
    context:&mut StepContext<'_>,_operation:&Operation,payload:&mut Option<Box<P>>,
    input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<J>>,
    demand:ToolJobAdmissionDemand,
    construct:impl FnOnce(OriginalToolJobIdentity,&mut Option<Box<P>>,&mut Option<RetainedToolWireInput>,&mut Option<RetainedToolWireInput>)->Result<(Box<J>,RetainedCloneProgress),ValueError>,
)->Result<bool,OriginalTurnFailure>{
    if payload.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tool factory requires its original payload").into());}
    admit_original_tool_construction(context,output,demand,|identity|construct(identity,payload,input,checkpoint))
}

/// 🏭️ A non-cloneable loan of an already admitted construction quote exists only inside its original callback.
pub struct OriginalToolConstructionScope<'scope>{identity:OriginalToolJobIdentity,remaining:RetirementDemand,_loan:std::marker::PhantomData<&'scope mut ()>}
impl<'scope> OriginalToolConstructionScope<'scope>{
 fn loan(_original:&'scope mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,identity:OriginalToolJobIdentity,remaining:RetirementDemand)->Self{Self{identity,remaining,_loan:std::marker::PhantomData}}
 pub fn identity(&self)->OriginalToolJobIdentity{self.identity}
 /// 🪙️ Reserves only the still-available original owner quote before any physical source is born.
 pub fn admit_owner(&mut self,demand:RetirementDemand)->Result<(),ValueError>{if demand.depth>self.remaining.depth||demand.copy_bytes>self.remaining.copy_bytes||demand.capacity_bytes>self.remaining.capacity_bytes||demand.release_bytes>self.remaining.release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original construction owner exceeds its remaining admitted quote"))}self.remaining.copy_bytes-=demand.copy_bytes;self.remaining.capacity_bytes-=demand.capacity_bytes;self.remaining.release_bytes-=demand.release_bytes;Ok(())}
}

/// 🏗️ One original operation funds construction while its caller retains any born candidate through refusal.
pub fn admit_original_tool_construction<O>(context:&mut StepContext<'_>,output:&mut Option<O>,demand:ToolJobAdmissionDemand,construct:impl FnOnce(OriginalToolJobIdentity)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{
    admit_original_tool_construction_scoped(context,output,demand,|scope|construct(scope.identity()))
}

/// 🏗️ Original scope custody funds nested owner births before one actual candidate reaches its receipt boundary.
pub fn admit_original_tool_construction_scoped<O>(context:&mut StepContext<'_>,output:&mut Option<O>,demand:ToolJobAdmissionDemand,construct:impl FnOnce(&mut OriginalToolConstructionScope<'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{
    let source=OriginalToolSourceBinding::borrow(context);
    semio_framework_job::admit_original_construction(context,output,semio_framework_job::OriginalConstructionDemand{items:demand.items,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|control|{
        let identity=OriginalToolJobIdentity{source};
        let mut scope=OriginalToolConstructionScope::loan(control,identity,demand.physical());
        construct(&mut scope)
    })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub(crate) mod tests;
