//! 🛂️ Authored deletion restrictions inspect only actual original rows and retained document sources.
use super::*;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedWorkPoll,retained_clone::RetainedCloneGrant};
/// 🔒️ A deletion restriction completes before any private removal page is published.
pub trait RowDeletionGuard<R:EntityRow>:Cursor+Send+'static{
 fn new()->Self;
 fn quote(&self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64)->Result<Quote,ValueError>;
 fn advance_reply<'r>(&mut self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn result(&self)->Option<bool>;
 fn begin_close(&mut self){Cursor::mark(self)}
}
/// 🧾️ Explicitly unrestricted row kinds still admit their semantic decision and original cursor closure.
pub struct NoRowReferences{ready:bool,closing:bool}
impl Cursor for NoRowReferences{
 fn mark(&mut self){self.closing=true;}
 fn finished(&self)->bool{self.closing}
 fn demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(scalar(0))}
 fn work(&self,_:usize)->Result<u64,ValueError>{Ok(0)}
 fn birth(&self)->bool{false}
 fn control(&self)->bool{false}
 fn birth_step(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:RetainedCloneGrant)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn close<'r>(&mut self,_:Option<&mut OriginalRetirementCloseLoan<'_>>,_:usize,_:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,_:bool)->Result<Done<'r>,ValueError>{Ok(r.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
}
impl<R:EntityRow> RowDeletionGuard<R> for NoRowReferences{
 fn new()->Self{Self{ready:false,closing:false}}
 fn quote(&self,_:RetainedCloneRef<'_,R>,_:RetainedCloneRef<'_,EnergyModelSnapshot>,_:usize,_:u64)->Result<Quote,ValueError>{if self.ready||self.closing{return Err(invalid());}Ok((scalar(1),1))}
 fn advance_reply<'r>(&mut self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let q=if recover{None}else{Some(<Self as RowDeletionGuard<R>>::quote(self,row,original,body,m))};custody::gated(e,r,q,m,|_,_|{self.ready=true;Ok(RetainedCloneStep::Complete(receipt(scalar(1))))})}
 fn result(&self)->Option<bool>{self.ready.then_some(true)}
}

#[path="🔗️reference/🦀️.rs"]mod reference;pub use reference::{RowReferenceSource,ReferenceRowGuard};

#[path="🤝️all/🦀️.rs"]mod all;pub use all::AllRowReferences;

#[path="🌳️root/🦀️.rs"]mod root;pub use root::{RootReferenceSource,RootReferenceGuard};
