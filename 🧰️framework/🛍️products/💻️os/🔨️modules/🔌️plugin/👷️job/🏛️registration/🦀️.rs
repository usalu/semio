//! 🏛️ The authentic registered tool catalog retains its original rows through paid shared metadata leases.
use crate::reserved_jobs::QualifiedBoundedFirstStepProof;
use semio_plugin_decl_app::tool_contract::{ArtifactToolPublicContract,ArtifactToolRegistration};
use semio_framework_value_persistent::list::PagedList;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value::{RetainedAllocationEffect,OriginalRetirementPoll,RetainedWorkPoll,RetirementPollExit,retirement::{RetirementStep,RetirementCursor,controlled::ControlledRetirement,arc_custody::ArcLease}};
use std::{sync::Arc,mem::{ManuallyDrop,size_of}};
type CatalogLease=ControlledRetirement<ArcLease<DeclaredToolCatalog>>;
const ROWS:usize=isize::MAX as usize;
/// 📜️ The author-bound registrations preserve their actual factory, owner and schema witnesses.
#[derive(semio_framework_value::RetireOwned)]
pub struct DeclaredToolCatalog{
 pub controller_id:String,
 pub bus:semio_framework::RegisteredActionBusSource,
 pub bounded_proofs:PagedList<(String,QualifiedBoundedFirstStepProof),ROWS>,
 pub bounded_contracts:PagedList<ArtifactToolPublicContract,ROWS>,
 pub framework:PagedList<(String,ArtifactToolRegistration),ROWS>,
 pub app:PagedList<(String,ArtifactToolRegistration),ROWS>,
}
impl DeclaredToolCatalog{
 pub fn app(&self,id:&str)->Option<&ArtifactToolRegistration>{self.app.iter().find_map(|(key,row)|(key==id).then_some(row))}
 pub fn framework(&self,id:&str)->Option<&ArtifactToolRegistration>{self.framework.iter().find_map(|(key,row)|(key==id).then_some(row))}
 pub fn bounded(&self,id:&str)->Option<&QualifiedBoundedFirstStepProof>{self.bounded_proofs.iter().find_map(|(key,row)|(key==id).then_some(row))}
}
/// 🧷️ Only the original program producer can lend the catalog capability used by a mounted alias.
#[derive(Clone,Copy)]
pub struct DeclaredToolCatalogSource<'a>{original:&'a Arc<DeclaredToolCatalog>,committed:bool}
impl std::ops::Deref for DeclaredToolCatalogSource<'_>{type Target=DeclaredToolCatalog;fn deref(&self)->&DeclaredToolCatalog{self.original}}
/// 🏛️ The program retains its exact catalog allocation until every live instance returns its lease.
pub struct OriginalDeclaredToolCatalog{original:ManuallyDrop<Option<Arc<DeclaredToolCatalog>>>,lease:Option<CatalogLease>,committed:bool}
impl OriginalDeclaredToolCatalog{
 pub fn new(original:DeclaredToolCatalog)->Self{Self{original:ManuallyDrop::new(Some(Arc::new(original))),lease:None,committed:false}}
 pub fn commit_producer(&mut self){self.committed=true;}
 pub fn source(&self)->Option<DeclaredToolCatalogSource<'_>>{self.original.as_ref().map(|original|DeclaredToolCatalogSource{original,committed:self.committed})}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.original.as_ref().is_some_and(|original|Arc::strong_count(original)!=1){return Ok(RetirementDemand{depth:1,..Default::default()})}lease_demands(&self.original,&self.lease,body)}
 pub fn close_work(&self,body:usize)->Result<u64,ValueError>{lease_work(&self.lease,body)}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{if self.original.as_ref().is_some_and(|owner|Arc::strong_count(owner)!=1){return RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}close_lease(&mut self.original,&mut self.lease,effect)}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.lease.is_none()}
}
impl Drop for OriginalDeclaredToolCatalog{fn drop(&mut self){if !self.committed&&self.lease.is_none()&&self.original.as_ref().is_none_or(|owner|Arc::strong_count(owner)==1){drop(self.original.take());return}assert!(std::thread::panicking()||self.terminal_is_empty(),"original tool declarations wait for every mounted lease and typed metadata closure")}}
fn lease_demands(original:&Option<Arc<DeclaredToolCatalog>>,lease:&Option<CatalogLease>,body:usize)->Result<RetirementDemand,ValueError>{
 if let Some(lease)=lease{return if lease.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<CatalogLease>>(),depth:1,..Default::default()})}else{let mut d=lease.next_demand(body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool catalog lease depth overflow"))?;Ok(d)}}
 Ok(if original.is_some(){RetirementDemand{copy_bytes:size_of::<Option<Arc<DeclaredToolCatalog>>>()+size_of::<Option<CatalogLease>>(),depth:1,..Default::default()}}else{Default::default()})
}
fn lease_work(lease:&Option<CatalogLease>,body:usize)->Result<u64,ValueError>{match lease{Some(lease) if !lease.terminal_is_empty()=>lease.next_computational_work(body),Some(_)=>Ok(1),None=>Ok(1)}}
fn close_lease(original:&mut ManuallyDrop<Option<Arc<DeclaredToolCatalog>>>,lease:&mut Option<CatalogLease>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let grant=effect.grant();
 let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
  if original.is_none()&&lease.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=lease_demands(original,lease,grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||effect.maximum_work_units()==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(owner)=lease.as_mut(){
   if owner.terminal_is_empty(){*lease=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
   let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
   let step=RetirementPollExit::resume(owner.step(grant.maximum_copy_bytes,&mut effect.nested(child)))?;
   return Ok(RetainedCloneStep::Progress(step.progress()))
  }
  let source=original.take().expect("tool catalog lease source");
  *lease=Some(ControlledRetirement::retain(ArcLease::new(source)));
  Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 })();
 RetirementPollExit::settle(effect,result)
}
/// 🪢️ A mounted lease borrows all original rows and returns only its alias during instance cleanup.
pub struct DeclaredToolCatalogAlias{original:ManuallyDrop<Option<Arc<DeclaredToolCatalog>>>,lease:Option<CatalogLease>}
impl DeclaredToolCatalogAlias{
 /// 👁️ The admitted lease lends the same committed producer while its original catalog remains owned.
 pub fn source(&self)->Option<DeclaredToolCatalogSource<'_>>{self.original.as_ref().map(|original|DeclaredToolCatalogSource{original,committed:true})}
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+size_of::<Arc<DeclaredToolCatalog>>(),depth:1,..Default::default()}}
 pub fn admit(source:DeclaredToolCatalogSource<'_>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{if !source.committed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mounted tool metadata requires its committed declaration producer"))}let d=Self::birth_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}Ok(Some((Self{original:ManuallyDrop::new(Some(Arc::clone(source.original))),lease:None},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn catalog(&self)->Option<&DeclaredToolCatalog>{self.original.as_deref()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{lease_demands(&self.original,&self.lease,body)}
 pub fn close_work(&self,body:usize)->Result<u64,ValueError>{lease_work(&self.lease,body)}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{close_lease(&mut self.original,&mut self.lease,effect)}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.lease.is_none()}
}
impl Drop for DeclaredToolCatalogAlias{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"mounted tool metadata lease must return through its original close")}}


/// 🗂️ The author-declared role selects its original row collection.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DeclaredToolRole{Bounded,Framework,App}
/// 🪪️ Only a genuine committed catalog source can mint an ordinal row capability.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct DeclaredToolRowKey{source:usize,role:DeclaredToolRole,ordinal:usize}
impl DeclaredToolRowKey{
 pub fn role(self)->DeclaredToolRole{self.role}
 pub fn ordinal(self)->usize{self.ordinal}
}
/// 👁️ All registration metadata remains borrowed from its sole declared catalog owner.
pub enum DeclaredToolRow<'a>{Bounded{proof:&'a QualifiedBoundedFirstStepProof,contract:&'a ArtifactToolPublicContract},Framework(&'a ArtifactToolRegistration),App(&'a ArtifactToolRegistration)}
impl DeclaredToolCatalog{
 fn row_at(&self,role:DeclaredToolRole,ordinal:usize)->Option<DeclaredToolRow<'_>>{match role{DeclaredToolRole::Bounded=>Some(DeclaredToolRow::Bounded{proof:&self.bounded_proofs.get(ordinal)?.1,contract:self.bounded_contracts.get(ordinal)?}),DeclaredToolRole::Framework=>self.framework.get(ordinal).map(|(_,row)|DeclaredToolRow::Framework(row)),DeclaredToolRole::App=>self.app.get(ordinal).map(|(_,row)|DeclaredToolRow::App(row))}}
}
impl DeclaredToolCatalogSource<'_>{
 /// 🎟️ The immutable original collection supplies a bounded ordinal capability without scanning or copying rows.
 pub fn row_key(&self,role:DeclaredToolRole,ordinal:usize)->Result<Option<DeclaredToolRowKey>,ValueError>{if !self.committed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registered tool row requires committed original declarations"))}Ok(self.original.row_at(role,ordinal).map(|_|DeclaredToolRowKey{source:Arc::as_ptr(self.original)as usize,role,ordinal}))}
}
impl DeclaredToolCatalogAlias{
 /// 🔒️ A retained proof reads only the row minted by this exact original catalog allocation.
 pub fn row(&self,key:DeclaredToolRowKey)->Result<DeclaredToolRow<'_>,ValueError>{let original=self.original.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"registered tool row alias has entered closure"))?;if Arc::as_ptr(original)as usize!=key.source{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registered tool row belongs to a different original catalog"))}original.row_at(key.role,key.ordinal).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"registered tool ordinal no longer names an authored row"))}
}

/// 🪪️ The one original cursor of a mounted tool metadata lease, registered proof or bounded proof.
#[doc(hidden)]
pub struct DeclaredToolAliasRetirement{alias:Option<DeclaredToolCatalogAlias>}
impl DeclaredToolAliasRetirement{
 pub fn bounded()->Self{Self{alias:None}}
 fn close_step(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
  let Some(alias)=self.alias.as_mut().filter(|alias|!alias.terminal_is_empty())else{return RetirementStep::Complete(0)};
  match alias.close_step(effect){RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 }
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.alias.as_ref().map_or_else(||Ok(Default::default()),|alias|alias.close_demands(body))}
}
impl semio_framework_value::retirement::RetireOwned for DeclaredToolCatalogAlias{
 type RetirementCursor=DeclaredToolAliasRetirement;
 fn retirement_seed(self)->DeclaredToolAliasRetirement{DeclaredToolAliasRetirement{alias:Some(self)}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<DeclaredToolAliasRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for DeclaredToolAliasRetirement{
 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.alias.as_ref().map_or(Ok(0),|alias|alias.close_work(body))}
 fn terminal_is_empty(&self)->bool{self.alias.as_ref().is_none_or(|alias|alias.terminal_is_empty())}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(usize::MAX)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(usize::MAX).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(usize::MAX)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

/// 🎟️ A qualified proof retains only its authentic original catalog lease and sealed registered row ordinal.
pub struct DeclaredToolRegistrationAlias{alias:DeclaredToolCatalogAlias,key:DeclaredToolRowKey}
impl DeclaredToolRegistrationAlias{
 pub fn birth_demands()->RetirementDemand{let mut d=DeclaredToolCatalogAlias::birth_demands();d.copy_bytes+=2*size_of::<DeclaredToolRowKey>();d}
 /// 🔒️ Validates the actual source and registered role before any original metadata alias transfers.
 pub fn admit(source:DeclaredToolCatalogSource<'_>,key:DeclaredToolRowKey,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{if !source.committed||key.source!=Arc::as_ptr(source.original)as usize||matches!(key.role,DeclaredToolRole::Bounded)||source.original.row_at(key.role,key.ordinal).is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registered proof requires its actual original App or Framework row"))}let d=Self::birth_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-2*size_of::<DeclaredToolRowKey>(),..grant};let Some((alias,mut p))=DeclaredToolCatalogAlias::admit(source,child)?else{return Ok(None)};p.copied_bytes+=2*size_of::<DeclaredToolRowKey>();Ok(Some((Self{alias,key},p)))}
 pub fn adoption_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),depth:1,..Default::default()}}
 /// 🤝️ A completed lookup donates its same original catalog alias into the selected registration proof.
 pub fn adopt(original:&mut Option<DeclaredToolCatalogAlias>,key:DeclaredToolRowKey,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let alias=original.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"registered proof adoption has no original catalog alias"))?;if !matches!(alias.row(key)?,DeclaredToolRow::App(_)|DeclaredToolRow::Framework(_)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registered proof adoption requires its actual App or Framework row"))}let d=Self::adoption_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}Ok(Some((Self{alias:original.take().unwrap(),key},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn row_key(&self)->DeclaredToolRowKey{self.key}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.alias.close_demands(body)}
 pub fn close_work(&self,body:usize)->Result<u64,ValueError>{self.alias.close_work(body)}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.alias.close_step(effect)}
 pub fn into_catalog_alias(self)->DeclaredToolCatalogAlias{self.alias}
 pub fn terminal_is_empty(&self)->bool{self.alias.terminal_is_empty()}
}
impl std::ops::Deref for DeclaredToolRegistrationAlias{
 type Target=ArtifactToolRegistration;
 fn deref(&self)->&Self::Target{match self.alias.row(self.key).expect("retained original registration remains readable before closure"){DeclaredToolRow::App(row)|DeclaredToolRow::Framework(row)=>row,DeclaredToolRow::Bounded{..}=>unreachable!("registered proof preserves its admitted original role")}}
}
impl semio_framework_value::retirement::RetireOwned for DeclaredToolRegistrationAlias{
 type RetirementCursor=DeclaredToolAliasRetirement;
 fn retirement_seed(self)->DeclaredToolAliasRetirement{DeclaredToolAliasRetirement{alias:Some(self.alias)}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<DeclaredToolAliasRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
