//! 🏪️ Original catalog sources remain loaned until one funded constructor or installation turn.
use super::*;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactStoreConstructorKind { Recipient, Document, Config, Draft, Interaction }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ArtifactStoreConstructorPhase { Preparation, Close }

pub(super) struct OriginalCatalogConstructor<P,Mu>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue,
Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>{
 original:Option<Result<store::DocumentStoreOwners<P,Mu>,store::DocumentStoreOwnersAdmissionError<P,Mu>>>,
 refusal:Option<ValueError>,
 done:bool,
}
impl<P,Mu> OriginalCatalogConstructor<P,Mu>
where P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack+Send+Sync+'static,
Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>+protocol::OpBinary+protocol::OpText+Send+'static{
 pub(super) fn new()->Self{Self{original:None,refusal:None,done:false}}
 pub(super) fn demands(&self,body:usize,closing:bool,source:impl FnOnce()->Result<RetirementDemand,ValueError>)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  let installation=if closing{0}else{self.original.as_ref().and_then(|original|original.as_ref().ok()).filter(|owner|owner.constructor_is_complete()).map_or(0,store::DocumentStoreOwners::installation_copy_bytes)};
  let parent=std::mem::size_of_val(&self.original)+std::mem::size_of_val(&self.refusal)+std::mem::size_of_val(&self.done)+installation;
  let child=match self.original.as_ref(){Some(Ok(owner))=>if closing{owner.uninstalled_owners_demands(body.saturating_sub(parent))?}else if owner.constructor_is_complete(){RetirementDemand::default()}else{owner.constructor_demands(body.saturating_sub(parent))?},Some(Err(error))=>match error.owners.as_ref(){Some(owner)=>if closing{owner.uninstalled_owners_demands(body.saturating_sub(parent))?}else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"denied original catalog remains retained"))},None=>RetirementDemand::default()},None=>if closing{RetirementDemand::default()}else{source()?}};
  Ok(RetirementDemand{copy_bytes:parent.checked_add(child.copy_bytes).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"catalog source quote copy overflow"))?,capacity_bytes:child.capacity_bytes,release_bytes:child.release_bytes,depth:child.depth.max(1).checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"catalog quote depth overflow"))?})
 }
 pub(super) fn advance(&mut self,store:&mut ArtifactStore<P,Mu>,grant:RetainedCloneGrant,build:impl FnOnce(RetainedCloneGrant)->Option<Result<(store::DocumentStoreOwners<P,Mu>,RetainedCloneProgress),store::DocumentStoreOwnersAdmissionError<P,Mu>>>)->Result<RetainedCloneProgress,ValueError>{
  if self.done{return Ok(Default::default())}
  let installation=self.original.as_ref().and_then(|original|original.as_ref().ok()).filter(|owners|owners.constructor_is_complete()).map_or(0,store::DocumentStoreOwners::installation_copy_bytes);
  let parent=std::mem::size_of_val(&self.original)+std::mem::size_of_val(&self.refusal)+std::mem::size_of_val(&self.done)+installation;
  if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth<2{return Ok(Default::default())}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
  if self.refusal.as_ref().is_some_and(|error|matches!(error.message,std::borrow::Cow::Borrowed(_))){self.refusal=None;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:parent,..Default::default()})}
  if self.original.as_ref().is_some_and(|original|matches!(original,Err(error)if error.owners.is_none()&&matches!(error.error.message,std::borrow::Cow::Borrowed(_)))){self.original=None;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:parent,..Default::default()})}
  let mut receipt=if self.original.is_none(){
   match build(child){None=>{self.done=true;RetainedCloneProgress{copied_items:1,..Default::default()}},Some(Ok((owners,receipt)))=>{if !receipt.fits(child){self.original=Some(Ok(owners));return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog source exceeded caller authority"))}self.original=Some(Ok(owners));receipt},Some(Err(original))=>{let mut receipt=original.progress;if !receipt.fits(child){self.original=Some(Err(original));return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original refused catalog exceeded caller authority"))}self.original=Some(Err(original));receipt.copied_items=1;receipt}}
  }else{
   let Some(Ok(owners))=self.original.as_mut()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original denied catalog sources remain retained"))};
   if owners.constructor_is_complete(){
    let Some(Ok(owners))=self.original.take()else{unreachable!()};
    match store.install_document_store_owners_exact(owners){Ok(())=>self.done=true,Err((error,owners))=>self.original=Some(Err(store::DocumentStoreOwnersAdmissionError{error,owners:Some(owners),progress:Default::default()}))}RetainedCloneProgress{copied_items:1,..Default::default()}
   }else{let demand=owners.constructor_demands(child.maximum_copy_bytes)?;if child.maximum_copy_bytes<demand.copy_bytes||child.maximum_capacity_bytes<demand.capacity_bytes||child.maximum_release_bytes<demand.release_bytes||child.maximum_depth<demand.depth{return Ok(Default::default())}match owners.admit_constructor(child){Ok(receipt)=>receipt,Err((error,mut receipt))=>{self.refusal=Some(error);receipt.copied_items=1;receipt}}}
  };
  if receipt==Default::default(){return Ok(receipt)}
  if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog ticket exceeded caller authority"))}
  receipt.copied_bytes=receipt.copied_bytes.checked_add(parent).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"catalog parent copy extent overflow"))?;
  Ok(receipt)
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.done&&self.original.is_none()&&self.refusal.is_none()}
 pub(super) fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  let parent=std::mem::size_of_val(&self.original)+std::mem::size_of_val(&self.refusal)+std::mem::size_of_val(&self.done);
  if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth<2{return Ok(Default::default())}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
  if self.refusal.as_ref().is_some_and(|error|matches!(error.message,std::borrow::Cow::Owned(_))){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original owned catalog refusal has no controlled close"))}
  if let Some(Err(error))=self.original.as_ref(){if matches!(error.error.message,std::borrow::Cow::Owned(_)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original owned catalog admission error has no controlled close"))}}
  let owner=self.original.as_mut().and_then(|original|match original{Ok(owner)=>Some(owner),Err(error)=>error.owners.as_mut()});
  if let Some(owner)=owner{
   if !owner.uninstalled_owners_terminal_is_empty(){let demand=owner.uninstalled_owners_demands(child.maximum_copy_bytes)?;if child.maximum_copy_bytes<demand.copy_bytes||child.maximum_capacity_bytes<demand.capacity_bytes||child.maximum_release_bytes<demand.release_bytes||child.maximum_depth<demand.depth{return Ok(Default::default())}let step=owner.close_uninstalled_owners_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.uninstalled_owners_terminal_is_empty(),"original staged catalog")?;let mut receipt=step.progress();if receipt==Default::default(){return Ok(receipt)}receipt.copied_bytes+=parent;return Ok(receipt)}
  }
  self.original=None;self.refusal=None;self.done=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:parent,..Default::default()})
 }
}
