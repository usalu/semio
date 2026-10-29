//! 📋️ Fixed row replacement and ranked structural changes share exact original page custody.
use super::*;
/// 🧭️ Authored operations choose one existing persistent list transition.
#[derive(Clone,Copy,PartialEq,Eq)]pub enum RowChange{Set,Insert,Remove}
pub enum RowListPreparation<T:RetireOwned+Send+Sync+'static>{Set(PersistentSet<T>),Splice{cursor:PersistentSplice<T>,remove:bool,ready:bool}}
impl<T:RetireOwned+Send+Sync+'static> RowListPreparation<T>{
 pub fn new(kind:RowChange,rank:usize)->Self{match kind{RowChange::Set=>Self::Set(PersistentSet::new(rank)),RowChange::Insert=>Self::Splice{cursor:PersistentSplice::new(PersistentSpliceKind::Insert,rank),remove:false,ready:false},RowChange::Remove=>Self::Splice{cursor:PersistentSplice::new(PersistentSpliceKind::Remove,rank),remove:true,ready:false}}}
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,row:Option<RetainedCloneRef<'_,SealedShared<T>>>)->Result<RetirementDemand,ValueError>{match self{Self::Set(c)=>c.demand(original,row.ok_or_else(invalid)?),Self::Splice{cursor,remove,..}=>cursor.demand(original,if *remove{None}else{row})}}
 pub fn advance(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,row:Option<RetainedCloneRef<'_,SealedShared<T>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self{Self::Set(c)=>c.advance(original,row.ok_or_else(invalid)?,grant),Self::Splice{cursor,remove,ready}=>{let s=cursor.advance(original,if *remove{None}else{row},grant)?;*ready=matches!(s,RetainedCloneStep::Complete(_));Ok(s)}}}
 pub fn output_ready(&self)->bool{match self{Self::Set(c)=>c.output_ready(),Self::Splice{ready,..}=>*ready}}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{match self{Self::Set(c)=>c.take_demand(),Self::Splice{cursor,..}=>Ok(cursor.take_demand())}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(PersistentList<T>,RetainedCloneProgress)>,ValueError>{match self{Self::Set(c)=>c.take(g),Self::Splice{cursor,..}=>cursor.take(g)}}
 pub fn begin_close(&mut self){match self{Self::Set(c)=>c.begin_close(),Self::Splice{cursor,..}=>cursor.begin_close()}}
}
impl<T:RetireOwned+Send+Sync+'static> CursorClose for RowListPreparation<T>{
 fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{match self{Self::Set(c)=>c.close_demand(b),Self::Splice{cursor,..}=>cursor.close_demand(b)}}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self{Self::Set(c)=>c.close_step(g),Self::Splice{cursor,..}=>cursor.close_step(g)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Set(c)=>c.terminal_is_empty(),Self::Splice{cursor,..}=>cursor.terminal_is_empty()}}
}
