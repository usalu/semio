//! 🧳️ Ranked key sources lend actual original field projections through funded row observations.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,list::persistent::PersistentList,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};
use std::mem::size_of;
/// 📎️ Actual original roots and key selectors define one stable ranked key coordinate source.
pub trait NamedRankedSource{
 type Root:?Sized+Sync;
 fn original(&self)->RetainedCloneRef<'_,Self::Root>;
 fn len(&self)->usize;
 fn key_projection(&self)->usize;
 fn rank_demand(&self,ordinal:usize)->Result<RetirementDemand,ValueError>;
 fn observe_key(&self,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(RetainedCloneRef<'_,str>,RetainedCloneProgress)>,ValueError>;
}
/// 🗂️ Native vectors and persistent pages share the same retained borrowed key protocol.
pub struct VecRankedKeys<'a,T:Sync>{pub original:RetainedCloneRef<'a,Vec<T>>,pub key:fn(&T)->&str}
impl<T:Sync> NamedRankedSource for VecRankedKeys<'_,T>{
 type Root=Vec<T>;
 fn original(&self)->RetainedCloneRef<'_,Self::Root>{self.original}
 fn len(&self)->usize{self.original.get().len()}
 fn key_projection(&self)->usize{self.key as usize}
 fn rank_demand(&self,ordinal:usize)->Result<RetirementDemand,ValueError>{if ordinal>=self.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"rank is outside the original key vector"));}Ok(RetirementDemand{copy_bytes:size_of::<Vec<T>>()+size_of::<usize>()*2+size_of::<RetainedCloneRef<'_,T>>()*2+size_of::<RetainedCloneRef<'_,str>>()*2,depth:1,..Default::default()})}
 fn observe_key(&self,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(RetainedCloneRef<'_,str>,RetainedCloneProgress)>,ValueError>{let d=self.rank_demand(ordinal)?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let row=self.original.project(ordinal,|root|&root[ordinal]);let key=row.project(self.key as usize,self.key);Ok(Some((key,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
}
/// 🌳️ Immutable key rows are observed through the original private page owner's paid ranked path.
pub struct PersistentRankedKeys<'a,T:Send+Sync+'static>{pub original:RetainedCloneRef<'a,PersistentList<T>>,pub key:fn(&T)->&str}
impl<T:Send+Sync+'static> NamedRankedSource for PersistentRankedKeys<'_,T>{
 type Root=PersistentList<T>;
 fn original(&self)->RetainedCloneRef<'_,Self::Root>{self.original}
 fn len(&self)->usize{self.original.get().len()}
 fn key_projection(&self)->usize{self.key as usize}
 fn rank_demand(&self,ordinal:usize)->Result<RetirementDemand,ValueError>{let mut d=self.original.get().rank_observation_demand(ordinal)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<RetainedCloneRef<'_,str>>()*2).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"ranked key projection footprint overflow"))?;Ok(d)}
 fn observe_key(&self,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(RetainedCloneRef<'_,str>,RetainedCloneProgress)>,ValueError>{let d=self.rank_demand(ordinal)?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let(row,mut p)=PersistentList::observe_rank(self.original,ordinal,grant)?.expect("funded original ranked key row");let key=row.project(self.key as usize,self.key);p.copied_bytes=d.copy_bytes;Ok(Some((key,p)))}
}
