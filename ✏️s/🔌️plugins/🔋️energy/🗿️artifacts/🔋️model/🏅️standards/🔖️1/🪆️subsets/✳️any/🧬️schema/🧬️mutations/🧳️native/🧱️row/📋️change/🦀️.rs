//! 📋️ Fixed row replacement and ranked structural changes share exact original page custody.
use super::*;
use crate::custody::paid_list_cursor;
/// 🧭️ Authored operations choose one existing persistent list transition.
#[derive(Clone,Copy,PartialEq,Eq)]pub enum RowChange{Set,Insert,Remove}
pub enum RowListPreparation<T:RetireOwned+Send+Sync+'static>{Set(PersistentSet<T>),Splice{cursor:PersistentSplice<T>,remove:bool,ready:bool}}
impl<T:RetireOwned+Send+Sync+'static> RowListPreparation<T>{
 pub fn new(kind:RowChange,rank:usize)->Self{match kind{RowChange::Set=>Self::Set(PersistentSet::new(rank)),RowChange::Insert=>Self::Splice{cursor:PersistentSplice::new(PersistentSpliceKind::Insert,rank),remove:false,ready:false},RowChange::Remove=>Self::Splice{cursor:PersistentSplice::new(PersistentSpliceKind::Remove,rank),remove:true,ready:false}}}
 pub fn quote(&self,original:RetainedCloneRef<'_,PersistentList<T>>,row:Option<RetainedCloneRef<'_,SealedShared<T>>>)->Result<Quote,ValueError>{match self{
  Self::Set(c)=>{let row=row.ok_or_else(invalid)?;Ok((c.demand(original,row)?,c.advance_work_demand(original,row)?))},
  Self::Splice{cursor,remove,..}=>{let row=if *remove{None}else{row};Ok((cursor.demand(original,row)?,cursor.advance_work_demand(original,row)?))}}}
 pub fn advance(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,row:Option<RetainedCloneRef<'_,SealedShared<T>>>,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<RetainedCloneStep,ValueError>{match self{
  Self::Set(c)=>{let row=row.ok_or_else(invalid)?;native_child(effect,work,|e,r|c.advance_into(original,row,e,r))?},
  Self::Splice{cursor,remove,ready}=>{let row=if *remove{None}else{row};let s=native_child(effect,work,|e,r|cursor.advance_into(original,row,e,r))??;*ready=matches!(s,RetainedCloneStep::Complete(_));Ok(s)}}}
 pub fn output_ready(&self)->bool{match self{Self::Set(c)=>c.output_ready(),Self::Splice{ready,..}=>*ready}}
 pub fn take_quote(&self)->Result<Quote,ValueError>{match self{Self::Set(c)=>Ok((c.take_demand()?,c.take_work_demand()?)),Self::Splice{cursor,..}=>Ok((cursor.take_demand(),cursor.take_work_demand()))}}
 pub fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<(PersistentList<T>,RetainedCloneProgress)>,ValueError>{let mut out=None;let step=match self{Self::Set(c)=>native_child(effect,work,|e,r|c.take_into(&mut out,e,r))??,Self::Splice{cursor,..}=>native_child(effect,work,|e,r|cursor.take_into(&mut out,e,r))??};Ok(out.map(|list|(list,step.progress())))}
}
paid_list_cursor!(RowListPreparation);
