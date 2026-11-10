//! 🧳️ Ranked key sources lend actual original field projections through funded row observations.
use crate::{retirement::RetireOwned,RetirementDemand,ValueError,ValueRefusalKind,list::persistent::PersistentList,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};
/// 📎️ Actual original roots and key selectors define one stable ranked key coordinate source.
pub trait NamedRankedSource{
 type Root:?Sized+Sync;
 fn original(&self)->RetainedCloneRef<'_,Self::Root>;
 fn len(&self)->usize;
 fn key_projection(&self)->usize;
 fn rank_demand(&self,ordinal:usize)->Result<RetirementDemand,ValueError>;
 fn observe_key(&self,ordinal:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(RetainedCloneRef<'_,str>,RetainedCloneProgress)>>;
}
/// 🗂️ Native vectors and persistent pages share the same retained borrowed key protocol.
pub struct VecRankedKeys<'a,T:Sync>{pub original:RetainedCloneRef<'a,Vec<T>>,pub key:fn(&T)->&str}
impl<T:Sync> NamedRankedSource for VecRankedKeys<'_,T>{
 type Root=Vec<T>;
 fn original(&self)->RetainedCloneRef<'_,Self::Root>{self.original}
 fn len(&self)->usize{self.original.get().len()}
 fn key_projection(&self)->usize{self.key as usize}
 fn rank_demand(&self,ordinal:usize)->Result<RetirementDemand,ValueError>{if ordinal>=self.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"rank is outside the original key vector"));}Ok(RetirementDemand{depth:1,..Default::default()})}
 fn observe_key(&self,ordinal:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(RetainedCloneRef<'_,str>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.rank_demand(ordinal)?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let row=self.original.project(ordinal,|root|&root[ordinal]);let key=row.project(self.key as usize,self.key);work=1;Ok(Some((key,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))})();crate::RetainedWorkOutcome{result,work}}
}
/// 🌳️ Immutable key rows are observed through the original private page owner's paid ranked path.
pub struct PersistentRankedKeys<'a,T:RetireOwned+Send+Sync+'static>{pub original:RetainedCloneRef<'a,PersistentList<T>>,pub key:fn(&T)->&str}
impl<T:RetireOwned+Send+Sync+'static> NamedRankedSource for PersistentRankedKeys<'_,T>{
 type Root=PersistentList<T>;
 fn original(&self)->RetainedCloneRef<'_,Self::Root>{self.original}
 fn len(&self)->usize{self.original.get().len()}
 fn key_projection(&self)->usize{self.key as usize}
 fn rank_demand(&self,ordinal:usize)->Result<RetirementDemand,ValueError>{self.original.get().rank_observation_demand(ordinal)}
 fn observe_key(&self,ordinal:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(RetainedCloneRef<'_,str>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.rank_demand(ordinal)?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let turn=PersistentList::observe_rank(self.original,ordinal,grant);work=turn.work;let(row,mut p)=turn.result?.expect("funded original ranked key row");let key=row.project(self.key as usize,self.key);work+=1;p.copied_bytes=d.copy_bytes;Ok(Some((key,p)))})();crate::RetainedWorkOutcome{result,work}}
}

