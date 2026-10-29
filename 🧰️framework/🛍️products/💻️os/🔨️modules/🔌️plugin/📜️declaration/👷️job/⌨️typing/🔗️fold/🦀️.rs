//! 🧵️ Original typing folds receive the actual old and incoming net under their initiating source.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,ValueError,retirement::RetireOwned};
use semio_framework_tool_typing_edit::OriginalTypingFold;

/// 🪢️ An explicit algebra selects a split, the actual incoming owner, or an original bounded producer.
pub enum OriginalTypingFoldPolicy<M:RetireOwned+Send+Sync+'static>{
 Split,
 ReplaceIncoming,
 Factory(&'static dyn ArtifactOriginalTypingFoldFactory<M>),
}
impl<M:RetireOwned+Send+Sync+'static> Copy for OriginalTypingFoldPolicy<M>{}
impl<M:RetireOwned+Send+Sync+'static> Clone for OriginalTypingFoldPolicy<M>{fn clone(&self)->Self{*self}}

/// 🏭️ A declared native algebra admits its concrete producer before either original net may move.
pub trait ArtifactOriginalTypingFoldFactory<M:RetireOwned+Send+Sync+'static>:Send+Sync{
 fn birth_demands(&self,previous:&[M],incoming:&[M],body:usize)->Result<RetirementDemand,ValueError>;
 fn admit(&self,previous:&[M],incoming:&[M],cx:&mut StepContext<'_>,output:&mut Option<Box<dyn ArtifactOriginalTypingFoldProducer<M>>>)->Result<bool,OriginalTurnFailure>;
}

/// 🧶️ The actual pair stays borrowed until its retained decision is received and the producer closes.
pub trait ArtifactOriginalTypingFoldProducer<M:RetireOwned+Send+Sync+'static>:Send{
 fn demands(&self,previous:&[M],incoming:&[M],body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,previous:&[M],incoming:&[M],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn ready(&self)->bool;
 fn take_demands(&self)->Result<RetirementDemand,ValueError>;
 fn take_into(&mut self,output:&mut Option<OriginalTypingFold<M>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
}
