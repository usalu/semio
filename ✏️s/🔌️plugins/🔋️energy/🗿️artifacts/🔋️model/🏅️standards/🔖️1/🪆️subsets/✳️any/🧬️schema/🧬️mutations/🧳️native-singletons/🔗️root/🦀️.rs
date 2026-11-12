//! 🔗️ Root link intents retain complete authored targets, history pins and roles through sparse publication and inverse.
use crate::{snapshot::original::EnergyModelSnapshot,diff::original::EnergyModelDiff,row::RowOutcome,leaf::RowFieldPreparation,custody::{self,Cursor,Held,Bound},scalar::{Quote,birth,invalid,scalar,nested,child,add,receipt,bind},retained::{Done,Flight,Reply,conclude,native_child}};
use protocol::{ArtifactLink,LinkPin,RetainedFieldPreparation,retained_mutation_demand_fits};
use semio_framework_value::{RetirementDemand,RetirementPollExit,RetainedAllocationEffect,ValueError,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_progress}};
use semio_framework_value_persistent::RetainedSliceEquality;
use std::mem::size_of;
/// 🧭️ The schema names one actual sealed root slot; no model branch is materialized.
#[derive(Clone,Copy,PartialEq,Eq)]pub enum RootLinkSlot{Weather,ReferencedModel}
impl RootLinkSlot{
 /// ⛽️ Slot selection, original projection and sealed-value borrow are fixed operations.
 const SOURCE_WORK:u64=3;
 /// ⛽️ The empty diff has eight model headers and two root headers, followed by slot selection and assignment.
 const DIFF_WORK:u64=12;
 fn source(self,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Option<ArtifactLink>>{match self{Self::Weather=>o.project(0,|s|s.weather_link.get()),Self::ReferencedModel=>o.project(1,|s|s.referenced_model.get())}}
 fn diff(self,value:SealedShared<Option<ArtifactLink>>)->EnergyModelDiff{let mut diff=EnergyModelDiff::default();match self{Self::Weather=>diff.weather_link=Some(value),Self::ReferencedModel=>diff.referenced_model=Some(value)}diff}
}
/// 🏷️ Attach payloads lend their complete original link within declared fixed work without allocation, traversal or panic; detach payloads retain their actual binding.
pub trait RootLinkPath<L:Send+Sync+'static>:Send+Sync+'static{const SLOT:RootLinkSlot;const DESIRED_WORK:u64;fn desired(input:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,ArtifactLink>>;}
/// 🚦️ The authored identity contract checks only fixed headers and pin discriminants, never an unbounded text scan.
pub fn valid_link(link:&ArtifactLink)->bool{valid_link_inspection(link).0}
fn valid_link_inspection(link:&ArtifactLink)->(bool,u64){let t=&link.target;let present=!t.artifact_id.is_empty()&!t.dialect.artifact_kind.is_empty()&!t.dialect.standard.is_empty()&!t.dialect.subset.is_empty()&!link.role.is_empty();let(pin,work)=match &link.pin{LinkPin::Head=>(true,1),LinkPin::Checkpoint{id}=>(!id.is_empty(),2),LinkPin::Snapshot{blob}=>(!blob.hash.is_empty()&!blob.media_type.is_empty(),3)};(present&pin,5+work)}
#[path="🟰️equality/🦀️.rs"]mod equality;
use equality::RootLinkEquality;
type Link=Option<ArtifactLink>;
/// 🧳️ Every variable link field is prepared under its original source; the prior complete slot becomes the exact inverse.
pub struct RootLinkPreparation<S:RootLinkPath<L>,L:Send+Sync+'static>{
 authored:Bound,original:Bound,equal:RootLinkEquality,input:Option<Box<RetainedFieldPreparation<ArtifactLink>>>,previous:Option<Box<RetainedFieldPreparation<Link>>>,
 link:Held<ArtifactLink>,raw:Held<Link>,sealed:Held<SealedShared<Link>>,decision:Held<EnergyModelDiff>,inverse:Held<Link>,
 phase:u8,outcome:Option<RowOutcome>,closing:bool,body:Option<usize>,flight:Flight,marker:std::marker::PhantomData<fn()->(S,L)>,
}
impl<S:RootLinkPath<L>,L:Send+Sync+'static> RootLinkPreparation<S,L>{
 fn verify(&self,p:RetainedCloneRef<'_,L>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.authored.0)?;o.check_binding(&self.original.0)?;Ok(())}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(input,previous,link,raw,sealed,decision,inverse,original,authored);
 fn child_overhead(&self)->Result<u64,ValueError>{match self.phase{3=>S::DESIRED_WORK.checked_add(RootLinkSlot::SOURCE_WORK).and_then(|v|v.checked_mul(4)).and_then(|v|self.equal.selection_work().checked_mul(8).and_then(|n|v.checked_add(n))).and_then(|v|v.checked_add(self.equal.owner_quote_work())).and_then(|v|v.checked_add(1)).ok_or_else(invalid),5=>S::DESIRED_WORK.checked_mul(4).and_then(|v|v.checked_add(1)).ok_or_else(invalid),9=>RootLinkSlot::SOURCE_WORK.checked_mul(4).and_then(|v|v.checked_add(1)).ok_or_else(invalid),_=>Ok(1)}}
 fn budget(&self,max:u64)->Result<u64,ValueError>{Ok(max.saturating_sub(self.child_overhead()?))}
 fn with_overhead(&self,child:u64,max:u64)->Result<u64,ValueError>{if child==0{return Ok(max.saturating_add(1))}self.child_overhead()?.checked_add(child).ok_or_else(invalid)}
 pub fn quote(&self,p:RetainedCloneRef<'_,L>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,max:u64)->Result<Quote,ValueError>{
  if self.closing||self.phase>=99||self.body.is_some_and(|v|v!=b)||!self.flight.idle(){return Err(invalid())}self.verify(p,o)?;let nb=b.saturating_sub(1);let budget=self.budget(max)?;
  match self.phase{
   0=>Ok((nested(scalar(p.binding_copy_bytes()),1+size_of::<usize>())?,p.binding_work_demand(&self.authored.0)?)),
   1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),
   2=>Ok((scalar(2*size_of::<ArtifactLink>()+4),S::DESIRED_WORK.checked_add(RootLinkSlot::SOURCE_WORK).and_then(|v|v.checked_add(8+4)).ok_or_else(invalid)?)),
   3=>{let l=S::desired(p).ok_or_else(invalid)?;let r=S::SLOT.source(o);let r=r.get().as_ref().ok_or_else(invalid)?;Ok((nested(self.equal.demand(l.get(),r,nb,budget)?,1)?,self.with_overhead(self.equal.work_demand(l.get(),r,nb,budget)?,max)?))},
   4=>Ok((birth::<RetainedFieldPreparation<ArtifactLink>>(),1)),
   5=>{let s=self.input.as_ref().ok_or_else(invalid)?;let l=S::desired(p).ok_or_else(invalid)?;Ok((nested(s.demand(l,nb,budget)?,1)?,self.with_overhead(s.advance_work_demand(l,nb,budget)?,max)?))},
   6=>{let s=self.input.as_ref().ok_or_else(invalid)?;Ok((nested(s.exchange_demand()?,2*size_of::<Option<ArtifactLink>>()+1)?,1))},
   7=>Ok((scalar(2*size_of::<Link>()+size_of::<Option<Link>>()+1),1)),
   8=>{let mut d=SealedShared::<Link>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Option<Link>>()+1).ok_or_else(invalid)?;Ok((d,SealedShared::<Link>::source_admission_work_demand()))},
   9=>Ok((birth::<RetainedFieldPreparation<Link>>(),1)),
   10=>{let s=self.previous.as_ref().ok_or_else(invalid)?;let src=S::SLOT.source(o);Ok((nested(s.demand(src,nb,budget)?,1)?,self.with_overhead(s.advance_work_demand(src,nb,budget)?,max)?))},
   11=>{let s=self.previous.as_ref().ok_or_else(invalid)?;Ok((nested(s.exchange_demand()?,2*size_of::<Option<Link>>()+1)?,1))},
   12=>Ok((scalar(2*size_of::<EnergyModelDiff>()+2*size_of::<Option<SealedShared<Link>>>()+2),RootLinkSlot::DIFF_WORK.checked_add(1).ok_or_else(invalid)?)),
   _=>Err(invalid()),
  }
 }
 pub fn advance_reply<'r>(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.quote(payload,original,body,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   self.body=Some(body);let mut p=RetainedCloneProgress::default();let nb=body.saturating_sub(1);let budget=self.budget(max)?;
   match self.phase{
    0=>{let extra=1+size_of::<usize>();p=add(bind(payload,&mut self.authored,&mut effect.nested(child(g,extra)),&mut work)?,extra)?;self.phase=1;},
    1=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{let before=S::SLOT.source(original);let desired=S::desired(payload);let valid=desired.map_or(true,|v|valid_link(v.get()));p=receipt(scalar(1));
     if !valid{self.outcome=Some(RowOutcome::Invalid);self.phase=99;}else if desired.is_none()&&before.get().is_none(){self.outcome=Some(RowOutcome::Missing);self.phase=99;}else if desired.is_some()&&before.get().is_some(){self.phase=3;}else if desired.is_some(){self.phase=4;}else{self.raw.set(None);self.phase=8;}},
    3=>{let l=S::desired(payload).ok_or_else(invalid)?;let r=S::SLOT.source(original);let r=r.get().as_ref().ok_or_else(invalid)?;let step=self.equal.advance(l.get(),r,nb,budget,effect,&mut work)?;p=step.progress();if let Some(equal)=self.equal.result(){if equal{self.outcome=Some(RowOutcome::Unchanged);self.phase=99;}else{self.phase=4;}}},
    4=>{self.input=Some(Box::new(Default::default()));self.phase=5;p=receipt(birth::<RetainedFieldPreparation<ArtifactLink>>());},
    5=>{let l=S::desired(payload).ok_or_else(invalid)?;let stage=self.input.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,budget,None,work,|r| stage.advance_into(l,nb,budget,&mut effect.nested(child(g,1)),r),stage.recover_advance_into(l,nb,budget,&mut effect.nested(child(g,1)),r));
     p=add(step?.progress(),1)?;if stage.value().is_some(){self.phase=6;}},
    6=>{let extra=2*size_of::<Option<ArtifactLink>>()+1;let Self{input,link,..}=self;let stage=input.as_deref_mut().ok_or_else(invalid)?;let slot=link.slot();let step=native_child(effect,&mut work,|e,r|stage.take_into(slot,&mut e.nested(child(g,extra)),r))??;p=add(step.progress(),extra)?;self.phase=7;},
    7=>{let value=self.link.take().ok_or_else(invalid)?;self.raw.set(Some(value));self.phase=8;p=receipt(scalar(1));},
    8=>{let turn=SealedShared::admit_from(self.raw.slot(),SharedIssuer::owned(),self.sealed.slot(),&mut effect.nested(g));work+=turn.work;p=turn.result?.progress();if self.sealed.is_some(){self.phase=9;}},
    9=>{self.previous=Some(Box::new(Default::default()));self.phase=10;p=receipt(birth::<RetainedFieldPreparation<Link>>());},
    10=>{let src=S::SLOT.source(original);let stage=self.previous.as_mut().ok_or_else(invalid)?;
     let step=protocol::retained_routed!(self.flight,effect,recover,1,body,budget,None,work,|r| stage.advance_into(src,nb,budget,&mut effect.nested(child(g,1)),r),stage.recover_advance_into(src,nb,budget,&mut effect.nested(child(g,1)),r));
     p=add(step?.progress(),1)?;if stage.value().is_some(){self.phase=11;}},
    11=>{let extra=2*size_of::<Option<Link>>()+1;let Self{previous,inverse,..}=self;let stage=previous.as_deref_mut().ok_or_else(invalid)?;let slot=inverse.slot();let step=native_child(effect,&mut work,|e,r|stage.take_into(slot,&mut e.nested(child(g,extra)),r))??;p=add(step.progress(),extra)?;self.phase=12;},
    12=>{self.decision.set(S::SLOT.diff(self.sealed.take().ok_or_else(invalid)?));self.outcome=Some(RowOutcome::Changed);self.phase=99;p=receipt(scalar(1));},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy root link intent")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:RootLinkPath<L>,L:Send+Sync+'static] RootLinkPreparation<S,L>,[],|s|s.begin_close());
impl<S:RootLinkPath<L>,L:Send+Sync+'static> RowFieldPreparation for RootLinkPreparation<S,L>{
 type Input=L;type Inverse=Link;
 fn new()->Self{Self{authored:Default::default(),original:Default::default(),equal:RootLinkEquality::new(),input:None,previous:None,link:Default::default(),raw:Default::default(),sealed:Default::default(),decision:Default::default(),inverse:Default::default(),phase:0,outcome:None,closing:false,body:None,flight:Default::default(),marker:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,L>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if key.is_some(){return Err(invalid())}self.verify(p,o)}
 fn quote(&self,p:RetainedCloneRef<'_,L>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{Self::quote(self,p,o,b,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,L>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{Self::advance_reply(self,p,o,b,m,e,r,recover)}
 fn ready(&self)->bool{!self.closing&&self.phase==99}fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid())}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}if !effect.performed(1)?{return Err(invalid())}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_quote(&self,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase!=100||self.inverse.is_none(){return Err(invalid())}Ok((scalar(1),1))}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.inverse.is_some()}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !self.inverse_ready(){return Err(invalid())}Ok((scalar(2*size_of::<Option<Link>>()+1),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}self.phase=101;Ok(Some((self.inverse.take().ok_or_else(invalid)?,receipt(d))))}
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
