//! 🔗️ An original sealed candidate stays in custody through a private group-identity transition.
use super::*;
use std::mem::{ManuallyDrop,size_of};

pub struct ArtifactStorePreparedBinding<P,Mutation>{original:ManuallyDrop<Option<ArtifactStoreOneItemPrepared<P,Mutation>>>,staged:bool}
impl<P,Mutation> ArtifactStorePreparedBinding<P,Mutation>{
    pub fn admit(original:ArtifactStoreOneItemPrepared<P,Mutation>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,ArtifactStoreOneItemPrepared<P,Mutation>)>{let copy=size_of::<Self>()+size_of::<ArtifactStoreOneItemPrepared<P,Mutation>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"binding admission lacks its exact original frame grant"),original));}Ok((Self{original:ManuallyDrop::new(Some(original)),staged:false},RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))}
    pub fn original(&self)->&ArtifactStoreOneItemPrepared<P,Mutation>{assert!(!self.staged,"staged group metadata has no original sealed witness");self.original.as_ref().expect("binding retains its original candidate")}
    pub fn edit(&self)->&Edit<Mutation>{self.original.as_ref().expect("binding retains its original edit").edit()}
    pub fn post_snapshot(&self)->&Arc<P>{self.original.as_ref().expect("binding retains its original post root").post_snapshot()}
    pub fn source_authority(&self)->&Arc<ArtifactStoreOneItemLiveAuthority>{self.original.as_ref().expect("binding retains its original authority").authority()}
    pub fn stage_group(&mut self,group:String,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,String)>{
        let copy=size_of::<Option<String>>()+size_of::<bool>();
        if self.staged||self.edit().mutation_meta.len()!=1||self.edit().mutation_meta[0].group_id.is_some(){return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"binding requires one original ungrouped metadata row"),group));}
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"binding group transfer lacks its exact frame grant"),group));}
        self.original.as_mut().unwrap().edit.mutation_meta[0].group_id=Some(group);self.staged=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})
    }
    pub fn rollback_group(&mut self,grant:RetainedCloneGrant)->Option<(Option<String>,RetainedCloneProgress)>{
        let copy=size_of::<Option<String>>()+size_of::<bool>();if !self.staged{return Some((None,Default::default()));}
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return None;}
        let group=self.original.as_mut()?.edit.mutation_meta[0].group_id.take();self.staged=false;Some((group,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }
    pub fn take_original(&mut self,grant:RetainedCloneGrant)->Option<(ArtifactStoreOneItemPrepared<P,Mutation>,RetainedCloneProgress)>{
        let copy=size_of::<ArtifactStoreOneItemPrepared<P,Mutation>>();if self.staged||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return None;}
        Some((self.original.take()?,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }
    pub fn finish_rebind(&mut self,authority:Arc<ArtifactStoreOneItemLiveAuthority>,digest:[u8;32],grant:RetainedCloneGrant)->Result<(ArtifactStoreOneItemPrepared<P,Mutation>,Arc<ArtifactStoreOneItemLiveAuthority>,RetainedCloneProgress),(ValueError,Arc<ArtifactStoreOneItemLiveAuthority>)>{
        let copy=size_of::<ArtifactStoreOneItemPrepared<P,Mutation>>()+size_of::<ArtifactStoreOneItemSeal>()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>();
        if !self.staged{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"binding has no admitted group identity"),authority));}
        if let Err(error)=authority.validate_semantic_edit(self.edit()){return Err((error,authority));}
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"binding result lacks its exact original frame grant"),authority));}
        let mut original=self.original.take().expect("binding result retains its original candidate");let next_clock=authority.next_clock();let seal=ArtifactStoreOneItemSeal{authority,edit_address:original.edit.as_ref()as*const Edit<Mutation>as usize,post_address:Arc::as_ptr(&original.post_snapshot)as usize,digest};let previous=std::mem::replace(&mut original.seal,seal);original.edit_digest=digest;original.next_clock=next_clock;self.staged=false;Ok((original,previous.authority,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }
    pub fn inspect_group<T,E>(original:&mut ArtifactStoreOneItemPrepared<P,Mutation>,group:String,grant:RetainedCloneGrant,inspect:impl FnOnce(&ArtifactStoreOneItemPrepared<P,Mutation>)->Result<T,E>)->Result<(Result<T,E>,String,RetainedCloneProgress),(ValueError,String)>{
        let copy=2*size_of::<Option<String>>();if original.edit.mutation_meta.len()!=1||original.edit.mutation_meta[0].group_id.is_some(){return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"group inspection requires one original metadata row"),group));}if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"group inspection lacks exact field roundtrip grant"),group));}original.edit.mutation_meta[0].group_id=Some(group);let result=inspect(original);let group=original.edit.mutation_meta[0].group_id.take().expect("private group inspection retains its identity");Ok((result,group,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }
    pub fn commit_group(mut original:ArtifactStoreOneItemPrepared<P,Mutation>,group:String,authority:Arc<ArtifactStoreOneItemLiveAuthority>,digest:[u8;32],grant:RetainedCloneGrant)->Result<(ArtifactStoreOneItemPrepared<P,Mutation>,Arc<ArtifactStoreOneItemLiveAuthority>,RetainedCloneProgress),(ValueError,ArtifactStoreOneItemPrepared<P,Mutation>,String,Arc<ArtifactStoreOneItemLiveAuthority>)>{
        let copy=size_of::<ArtifactStoreOneItemPrepared<P,Mutation>>()+size_of::<ArtifactStoreOneItemSeal>()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<Option<String>>();if original.edit.mutation_meta.len()!=1||original.edit.mutation_meta[0].group_id.is_some(){return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"group commit requires one original metadata row"),original,group,authority));}if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"group commit lacks its exact original frame grant"),original,group,authority));}original.edit.mutation_meta[0].group_id=Some(group);if let Err(error)=authority.validate_semantic_edit(original.edit()){let group=original.edit.mutation_meta[0].group_id.take().expect("refused group commit retains its exact identity");return Err((error,original,group,authority));}original.edit_digest=digest;original.next_clock=authority.next_clock;let seal=ArtifactStoreOneItemSeal{authority,edit_address:original.edit.as_ref()as*const Edit<Mutation>as usize,post_address:Arc::as_ptr(&original.post_snapshot)as usize,digest};let previous=std::mem::replace(&mut original.seal,seal);Ok((original,previous.authority,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
    }
    pub fn commit_copy_demand()->usize{size_of::<ArtifactStoreOneItemPrepared<P,Mutation>>()+size_of::<ArtifactStoreOneItemSeal>()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<Option<String>>()}
    pub fn terminal_is_empty(&self)->bool{self.original.is_none()}
}
impl<P,Mutation> Drop for ArtifactStorePreparedBinding<P,Mutation>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"binding must return or rebind every original prepared piece before Drop");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
