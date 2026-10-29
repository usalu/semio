//! 🛂️ Authored deletion restrictions inspect only actual original rows and retained document sources.
use super::*;
/// 🔒️ A deletion restriction completes before any private removal page is published.
pub trait RowDeletionGuard<R:EntityRow>:CursorClose+Send+'static{
 fn new()->Self;
 fn demand(&self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn result(&self)->Option<bool>;
 fn begin_close(&mut self);
}
/// 🧾️ Explicitly unrestricted row kinds still admit their semantic decision and original cursor closure.
pub struct NoRowReferences{ready:bool,closing:bool}
impl<R:EntityRow> RowDeletionGuard<R> for NoRowReferences{
 fn new()->Self{Self{ready:false,closing:false}}
 fn demand(&self,_:RetainedCloneRef<'_,R>,_:RetainedCloneRef<'_,EnergyModelSnapshot>,_:usize)->Result<RetirementDemand,ValueError>{if self.ready||self.closing{return Err(invalid());}Ok(scalar(1))}
 fn advance(&mut self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=<Self as RowDeletionGuard<R>>::demand(self,row,original,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}self.ready=true;Ok(RetainedCloneStep::Complete(receipt(d)))}
 fn result(&self)->Option<bool>{self.ready.then_some(true)}
 fn begin_close(&mut self){self.closing=true;}
}
impl CursorClose for NoRowReferences{fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}Ok(scalar(0))}fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(invalid());}Ok(RetainedCloneStep::Complete(Default::default()))}fn terminal_is_empty(&self)->bool{self.closing}}

#[path="🔗️reference/🦀️.rs"]mod reference;pub use reference::{RowReferenceSource,ReferenceRowGuard};

#[path="🤝️all/🦀️.rs"]mod all;pub use all::AllRowReferences;

#[path="🌳️root/🦀️.rs"]mod root;pub use root::{RootReferenceSource,RootReferenceGuard};
