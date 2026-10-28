//! 🎯️ A declared action producer owns original command construction and wire encoding under the initiating continuation.
use semio_framework::{Fault,action_bus::RetainedToolWireInput};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{DslValue,RetirementDemand,ValueError,retirement::RetireOwned};

/// 🏭️ A genuine application declaration supplies the concrete producer before any argument moves.
pub trait ArtifactOriginalCommandProducerFactory<C:RetireOwned+Send+Sync+'static>:Send+Sync{
 fn birth_demands(&self)->Result<RetirementDemand,ValueError>;
 fn admit(&self,cx:&mut StepContext<'_>,output:&mut Option<Box<dyn ArtifactOriginalCommandProducer<C>>>)->Result<bool,OriginalTurnFailure>;
}

/// 🧬️ The producer retains every partial input, typed owner and encoded page until exact handoff or original closure.
pub trait ArtifactOriginalCommandProducer<C:RetireOwned+Send+Sync+'static>:Send{
 fn demands(&self,action:&str,args:&Option<DslValue>,fault:&Option<Fault>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn ready(&self)->bool;
 fn take_demands(&self)->Result<RetirementDemand,ValueError>;
 fn take_into(&mut self,command:&mut Option<Box<C>>,wire:&mut Option<RetainedToolWireInput>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
}
