//! 📦️ Shared original publication assembly for cloned and domain-prepared semantic candidates.
use super::*;
use semio_framework_value::{RetirementDemand,RetainedWorkOutcome,retirement::arc_custody::shared_retirement_allocation_bytes};
/// 🧳️ Borrows the caller's exact semantic outputs until each funded frontier transfers its custody.
pub struct OriginalPublicationInputs<'a,P,M>{
 pub forward:&'a mut Option<M>,pub post:&'a mut Option<P>,pub inverse:&'a mut Option<Vec<M>>,
 pub authority:&'a mut Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
 pub mutations:&'a mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
 pub snapshots:&'a mut Option<Arc<dyn SnapshotRetirementFactory<P>>>,
}
/// 🧬️ Owns intermediate metadata and sealing frames without repeating a semantic edit or snapshot clone.
pub struct OriginalPublicationAssembly<P:RetireOwned+Sync,M:RetireOwned+Sync>{
 pub(super) publication_post:Option<Arc<P>>,pub(super) publication_post_close:Option<SharedControlledRetirement<P>>,
 pub(super) publication_inverse:Option<Box<OriginalVectorCursor<M>>>,
 pub(super) publication_metadata:Option<Box<NativeEditMetadata<M>>>,pub(super) publication_metadata_close:Option<ControlledRetirement<Box<NativeEditMetadata<M>>>>,
 pub(super) publication_edit:Option<Edit<M>>,pub(super) publication_edit_close:Option<ControlledRetirement<Edit<M>>>,
 pub(super) sealer:Option<ArtifactStoreOneItemSealer<P,M>>,stage:u8,
}
impl<P:RetireOwned+Sync,M:RetireOwned+Sync> Default for OriginalPublicationAssembly<P,M>{
 fn default()->Self{Self{publication_post:None,publication_post_close:None,publication_inverse:None,publication_metadata:None,publication_metadata_close:None,publication_edit:None,publication_edit_close:None,sealer:None,stage:4}}
}
fn funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items!=0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
impl<P:RetireOwned+Send+Sync+'static,M:ArtifactCanonicalJsonTree+RetireOwned+Send+Sync+'static> OriginalPublicationAssembly<P,M>{
 /// 📏️ Borrows the next physical frontier without scanning, allocating or transferring inputs.
 pub fn advance_demands(&self,authority:Option<&Arc<ArtifactStoreOneItemLiveAuthority>>,body:usize)->Result<RetirementDemand,ValueError>{
  Ok(match self.stage{
            4=>RetirementDemand{copy_bytes:size_of::<P>()+size_of::<Arc<P>>(),capacity_bytes:shared_retirement_allocation_bytes::<P>(),depth:1,..Default::default()},
            5=>{let mut d=OriginalVectorCursor::<M>::constructor_demand();d.copy_bytes=d.copy_bytes.checked_add(size_of::<OriginalVectorCursor<M>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original inverse vector transfer overflow"))?;d.capacity_bytes=size_of::<OriginalVectorCursor<M>>();d},
            6=>NativeEditMetadata::<M>::constructor_demand(),
            7=>{let owner=self.publication_metadata.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata frame missing"))?;owner.next_demand(authority.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata authority missing"))?,self.publication_inverse.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse cursor missing"))?)?},
            8=>RetirementDemand{copy_bytes:size_of::<Option<Box<NativeEditMetadata<M>>>>(),release_bytes:size_of::<NativeEditMetadata<M>>(),depth:1,..Default::default()},
            9=>{let owner=self.publication_inverse.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse backing missing"))?;if owner.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()}}else{let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original inverse backing depth overflow"))?;demand}},
            10=>RetirementDemand{copy_bytes:size_of::<Option<Box<OriginalVectorCursor<M>>>>(),release_bytes:size_of::<OriginalVectorCursor<M>>(),depth:1,..Default::default()},
            11=>{let birth=ArtifactStoreOneItemSealer::<P,M>::constructor_demand();RetirementDemand{copy_bytes:ArtifactStoreOneItemSealer::<P,M>::constructor_copy_byte_demand()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<Arc<P>>()+size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>()+size_of::<Arc<dyn SnapshotRetirementFactory<P>>>(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()}},
    _=>return Err(ValueError::invariant("original semantic assembly has no pending frontier")),
   } )
 }
 /// 🎟️ Advances one already admitted semantic frontier while retaining every refused original.
 pub fn advance(&mut self,input:OriginalPublicationInputs<'_,P,M>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{
  let OriginalPublicationInputs{forward,post,inverse,authority:authority_slot,mutations,snapshots}=input;
  let mut progress=RetainedCloneProgress::default();let mut work=0u64;
  let result=(||{
   let demand=self.advance_demands(authority_slot.as_ref(),g.maximum_copy_bytes)?;
   if !funds(g,demand){return Ok(false);}
   let invoice=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes};
   match self.stage{
            4=>{self.publication_post=Some(Arc::new(post.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original exclusive post missing before shared publication birth"))?));self.stage=5;progress=invoice;},
            5=>{let original=inverse.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse vector missing before publication handoff"))?;let turn=OriginalVectorCursor::admit(original,g);work=work.saturating_add(turn.work);match turn.result {Ok((owner,receipt))=>{self.publication_inverse=Some(Box::new(owner));progress=RetainedCloneProgress{copied_bytes:receipt.copied_bytes+size_of::<OriginalVectorCursor<M>>(),retained_capacity_bytes:size_of::<OriginalVectorCursor<M>>(),..receipt};self.stage=6;},Err((error,original))=>{*inverse=Some(original);return Err(error);}}},
            6=>{let(owner,receipt)=NativeEditMetadata::<M>::admit(authority_slot.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original authority missing at metadata admission"))?,g)?;self.publication_metadata=Some(owner);progress=receipt;self.stage=7;},
            7=>{let owner=self.publication_metadata.as_mut().unwrap();if owner.ready(){let(original,receipt)=owner.take(g)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata take lacks grant"))?;self.publication_edit=Some(original);progress=receipt;self.stage=8;}else{progress=owner.step(authority_slot.as_ref().unwrap(),forward,self.publication_inverse.as_mut().unwrap(),g)?;if progress==Default::default(){return Ok(false);}}},
            8=>{if !self.publication_metadata.as_ref().unwrap().terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original metadata frame still retains fields before release"));}self.publication_metadata=None;self.stage=9;progress=invoice;},
            9=>{let owner=self.publication_inverse.as_mut().ok_or_else(||ValueError::invariant("publication_inverse absent"))?;if owner.terminal_is_empty(){self.stage=10;progress=invoice;}else{owner.begin_close();let turn=owner.step(RetainedCloneGrant{maximum_items:1,maximum_depth:g.maximum_depth-1,..g});work=work.saturating_add(turn.work);progress=turn.result?.progress();}},
            10=>{if !self.publication_inverse.as_ref().unwrap().terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original inverse frame retains backing before release"));}self.publication_inverse=None;self.stage=11;progress=invoice;},
            11=>{if authority_slot.is_none()||self.publication_edit.is_none()||self.publication_post.is_none()||mutations.is_none()||snapshots.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication seal is missing an original owner"));}let authority=authority_slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication authority missing at seal"))?;let original=self.publication_edit.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication edit missing at seal"))?;let post=self.publication_post.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original publication post missing at seal"))?;let mutation_retirement=mutations.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original mutation factory missing at seal"))?;let snapshot_retirement=snapshots.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original snapshot factory missing at seal"))?;match ArtifactStoreOneItemSealer::admit(authority,original,post,mutation_retirement,snapshot_retirement,g){Ok((owner,receipt))=>{self.sealer=Some(owner);progress=RetainedCloneProgress{copied_bytes:demand.copy_bytes,..receipt};self.stage=12;},Err((error,authority,original,post,mutation_retirement,snapshot_retirement))=>{*authority_slot=Some(authority);self.publication_edit=Some(original);self.publication_post=Some(post);*mutations=Some(mutation_retirement);*snapshots=Some(snapshot_retirement);return Err(error);}}},
    _=>unreachable!(),
   }
   progress=admit_retained_clone_progress(g,progress,"original semantic publication assembly")?;
   Ok(true)
  })();
  work=work.max(u64::from(progress!=Default::default()));
  RetainedWorkOutcome{result:result.map(|advanced|advanced.then_some(progress)).map_err(|error:ValueError|if error.retained_progress()==Default::default(){error.with_retained_progress(progress)}else{error}),work}
 }
 /// 📐️ Quotes the exact intermediate owner selected for cancellation.
 pub fn close_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if let Some(owner)=self.sealer.as_ref(){return owner.retirement_demands(body).map(Some);}
        if let Some(owner)=self.publication_metadata_close.as_ref(){if owner.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Box<NativeEditMetadata<M>>>>>(),depth:1,..Default::default()}));}let copy=owner.next_copy_byte_demand()?;return Ok(Some(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"native metadata close depth overflow"))?}));}
        if self.publication_metadata.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Box<NativeEditMetadata<M>>>()+size_of::<ControlledRetirement<Box<NativeEditMetadata<M>>>>(),depth:1,..Default::default()}));}
        if let Some(owner)=self.publication_edit_close.as_ref(){if owner.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Edit<M>>>>(),depth:1,..Default::default()}));}let copy=owner.next_copy_byte_demand()?;return Ok(Some(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original assembled edit close depth overflow"))?}));}
        if self.publication_edit.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Edit<M>>()+size_of::<ControlledRetirement<Edit<M>>>(),depth:1,..Default::default()}));}
        if let Some(owner)=self.publication_inverse.as_ref(){if owner.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<Box<OriginalVectorCursor<M>>>>(),release_bytes:size_of::<OriginalVectorCursor<M>>(),depth:1,..Default::default()}));}let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original inverse vector parent depth overflow"))?;return Ok(Some(demand));}
        if let Some(owner)=self.publication_post_close.as_ref(){if owner.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<SharedControlledRetirement<P>>>(),depth:1,..Default::default()}));}let copy=owner.next_copy_byte_demand()?;return Ok(Some(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original publication post parent depth overflow"))?}));}
        if self.publication_post.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Arc<P>>()+size_of::<SharedControlledRetirement<P>>(),depth:1,..Default::default()}));}
  Ok(None)
 }
 /// ♻️ Closes only the intermediate custody held here; untransferred inputs remain with the parent.
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;
  let result=(||{
   let Some(d)=self.close_demands(g.maximum_copy_bytes)?else{return Ok(RetainedCloneStep::Complete(Default::default()));};
   if !funds(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
   if let Some(owner)=self.sealer.as_mut(){owner.begin_close();let result=owner.close_step(g);work=owner.last_turn_work();let step=result?;if owner.terminal_is_empty(){self.sealer=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
   let child=RetainedCloneGrant{maximum_items:1,maximum_depth:g.maximum_depth-1,..g};
   macro_rules! controlled{
    ($field:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field=None;work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}let result=owner.step(child);work=owner.last_turn_work();return result.map(|step|RetainedCloneStep::Progress(step.progress()));}};
   }
   controlled!(publication_metadata_close);
   if let Some(original)=self.publication_metadata.take(){match ControlledRetirement::new(original){Ok(owner)=>self.publication_metadata_close=Some(owner),Err((error,original))=>{self.publication_metadata=Some(original);return Err(error);}}work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
   controlled!(publication_edit_close);
   if let Some(original)=self.publication_edit.take(){match ControlledRetirement::new(original){Ok(owner)=>self.publication_edit_close=Some(owner),Err((error,original))=>{self.publication_edit=Some(original);return Err(error);}}work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
   if let Some(owner)=self.publication_inverse.as_mut(){if owner.terminal_is_empty(){self.publication_inverse=None;work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}));}owner.begin_close();let turn=owner.step(child);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()));}
   controlled!(publication_post_close);
   if let Some(original)=self.publication_post.take(){self.publication_post_close=Some(SharedControlledRetirement::new(original));work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
   Err(ValueError::invariant("original semantic assembly lost its selected closing owner"))
  })();
  RetainedWorkOutcome{result,work}
 }
 /// 🧵️ Borrows the common canonical sealer after all original metadata has transferred.
 pub fn sealer(&self)->Option<&ArtifactStoreOneItemSealer<P,M>>{self.sealer.as_ref()}
 /// 🪡️ Advances or takes the common canonical sealer without transferring its custody.
 pub fn sealer_mut(&mut self)->Option<&mut ArtifactStoreOneItemSealer<P,M>>{self.sealer.as_mut()}
}
impl<P:RetireOwned+Sync,M:RetireOwned+Sync> OriginalPublicationAssembly<P,M>{
 /// 📭️ Confirms every intermediate owner was transferred or retired.
 pub fn terminal_is_empty(&self)->bool{self.publication_post.is_none()&&self.publication_post_close.is_none()&&self.publication_inverse.is_none()&&self.publication_metadata.is_none()&&self.publication_metadata_close.is_none()&&self.publication_edit.is_none()&&self.publication_edit_close.is_none()&&self.sealer.is_none()}
}
impl<P:RetireOwned+Sync,M:RetireOwned+Sync> Drop for OriginalPublicationAssembly<P,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original semantic assembly dropped before all custody transferred or retired");}}
