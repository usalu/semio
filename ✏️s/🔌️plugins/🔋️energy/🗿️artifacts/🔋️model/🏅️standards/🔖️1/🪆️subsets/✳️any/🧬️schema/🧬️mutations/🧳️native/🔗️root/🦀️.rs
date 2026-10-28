//! 🔗️ Root link intents retain complete authored targets, history pins and roles through sparse publication and inverse.
use crate::{snapshot::original::EnergyModelSnapshot,diff::original::EnergyModelDiff,row::RowOutcome,leaf::RowFieldPreparation,scalar::{cursor_demand,close_cursor,birth,invalid,scalar,nested,child,add,tag,receipt}};
use protocol::{ArtifactLink,LinkPin,mutation::retained::{RetainedFieldPreparation,retained_mutation_demand_fits}};
use semio_framework_value::{RetirementDemand,ValueError,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_persistent::RetainedSliceEquality;
use std::mem::{size_of,ManuallyDrop};
/// 🧭️ The schema names one actual sealed root slot; no model branch is materialized.
#[derive(Clone,Copy,PartialEq,Eq)]pub enum RootLinkSlot{Weather,ReferencedModel}
impl RootLinkSlot{
 fn source(self,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Option<ArtifactLink>>{match self{Self::Weather=>o.project(0,|s|s.weather_link.get()),Self::ReferencedModel=>o.project(1,|s|s.referenced_model.get())}}
 fn diff(self,value:SealedShared<Option<ArtifactLink>>)->EnergyModelDiff{let mut diff=EnergyModelDiff::default();match self{Self::Weather=>diff.weather_link=Some(value),Self::ReferencedModel=>diff.referenced_model=Some(value)}diff}
}
/// 🏷️ Attach payloads lend their complete original link; detach payloads still retain their own actual source binding.
pub trait RootLinkPath<L:Send+Sync+'static>:Send+Sync+'static{const SLOT:RootLinkSlot;fn desired(input:RetainedCloneRef<'_,L>)->Option<RetainedCloneRef<'_,ArtifactLink>>;}
/// 🚦️ The authored identity contract checks only fixed headers and pin discriminants, never an unbounded text scan.
pub fn valid_link(link:&ArtifactLink)->bool{let t=&link.target;!t.artifact_id.is_empty()&&!t.dialect.artifact_kind.is_empty()&&!t.dialect.standard.is_empty()&&!t.dialect.subset.is_empty()&&!link.role.is_empty()&&match &link.pin{LinkPin::Head=>true,LinkPin::Checkpoint{id}=>!id.is_empty(),LinkPin::Snapshot{blob}=>!blob.hash.is_empty()&&!blob.media_type.is_empty()}}
#[path="🟰️equality/🦀️.rs"]mod equality;
use equality::RootLinkEquality;
/// 🧳️ Every variable link field is prepared under its original source; the prior complete slot becomes the exact inverse.
pub struct RootLinkPreparation<S:RootLinkPath<L>,L:Send+Sync+'static>{
 authored:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,equal:RootLinkEquality,input:Option<Box<RetainedFieldPreparation<ArtifactLink>>>,previous:Option<Box<RetainedFieldPreparation<Option<ArtifactLink>>>>,
 raw:ManuallyDrop<Option<Option<ArtifactLink>>>,sealed:ManuallyDrop<Option<SealedShared<Option<ArtifactLink>>>>,decision:ManuallyDrop<Option<EnergyModelDiff>>,inverse:ManuallyDrop<Option<Option<ArtifactLink>>>,retirement:RetainedCloneClose,
 phase:u8,outcome:Option<RowOutcome>,closing:bool,body:Option<usize>,marker:std::marker::PhantomData<fn()->(S,L)>,
}
impl<S:RootLinkPath<L>,L:Send+Sync+'static> RootLinkPreparation<S,L>{
 fn verify(&self,p:RetainedCloneRef<'_,L>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.authored)?;o.check_binding(&self.original)?;Ok(())}
 fn decision_demand(&self,p:RetainedCloneRef<'_,L>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=99||self.body.is_some_and(|v|v!=b){return Err(invalid())}self.verify(p,o)?;match self.phase{
 0=>nested(scalar(p.binding_copy_bytes()),1+size_of::<usize>()),1=>nested(scalar(o.binding_copy_bytes()),1),2=>Ok(scalar(2*size_of::<ArtifactLink>()+4)),
 3=>nested(self.equal.demand(S::desired(p).ok_or_else(invalid)?.get(),S::SLOT.source(o).get().as_ref().ok_or_else(invalid)?,b.saturating_sub(1))?,1),
 4=>Ok(birth::<RetainedFieldPreparation<ArtifactLink>>()),5=>nested(self.input.as_ref().ok_or_else(invalid)?.demand(S::desired(p).ok_or_else(invalid)?,b.saturating_sub(1))?,1),
 6=>nested(self.input.as_ref().ok_or_else(invalid)?.exchange_demand()?,2*size_of::<Option<Option<ArtifactLink>>>()+1),
 7=>Ok(RetirementDemand{copy_bytes:2*size_of::<Option<ArtifactLink>>()+2*size_of::<SealedShared<Option<ArtifactLink>>>()+1,capacity_bytes:SealedShared::<Option<ArtifactLink>>::birth_bytes(),depth:1,..Default::default()}),
 8=>Ok(birth::<RetainedFieldPreparation<Option<ArtifactLink>>>()),9=>nested(self.previous.as_ref().ok_or_else(invalid)?.demand(S::SLOT.source(o),b.saturating_sub(1))?,1),
 10=>nested(self.previous.as_ref().ok_or_else(invalid)?.exchange_demand()?,2*size_of::<Option<Option<ArtifactLink>>>()+1),
 11=>Ok(scalar(2*size_of::<EnergyModelDiff>()+2*size_of::<Option<SealedShared<Option<ArtifactLink>>>>()+2)),_=>Err(invalid())}}
 fn decide(&mut self,payload:RetainedCloneRef<'_,L>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.decision_demand(payload,original,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}self.body=Some(body);let mut p=receipt(d);match self.phase{
 0=>{p=add(payload.bind(&mut self.authored,child(g,1+size_of::<usize>()))?.ok_or_else(invalid)?,1+size_of::<usize>())?;self.phase=1;},
 1=>{p=add(original.bind(&mut self.original,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=2;},
 2=>{let before=S::SLOT.source(original);let desired=S::desired(payload);if desired.is_some_and(|v|!valid_link(v.get())){self.outcome=Some(RowOutcome::Invalid);self.phase=99;}else if desired.is_none()&&before.get().is_none(){self.outcome=Some(RowOutcome::Missing);self.phase=99;}else if desired.is_some()&&before.get().is_some(){self.phase=3;}else if desired.is_some(){self.phase=4;}else{*self.raw=Some(None);self.phase=7;}},
 3=>{let step=self.equal.advance(S::desired(payload).ok_or_else(invalid)?.get(),S::SLOT.source(original).get().as_ref().ok_or_else(invalid)?,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?;p=add(step.progress(),1)?;if let Some(equal)=self.equal.result(){if equal{self.outcome=Some(RowOutcome::Unchanged);self.phase=99;}else{self.phase=4;}}},
 4=>{self.input=Some(Box::new(Default::default()));self.phase=5;},
 5=>{let stage=self.input.as_mut().ok_or_else(invalid)?;p=add(stage.advance(S::desired(payload).ok_or_else(invalid)?,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if stage.value().is_some(){self.phase=6;}},
 6=>{let extra=2*size_of::<Option<Option<ArtifactLink>>>()+1;let(value,progress)=self.input.as_mut().ok_or_else(invalid)?.take(child(g,extra)).map_err(|e|tag(e,extra))?;*self.raw=Some(Some(value.ok_or_else(invalid)?));self.phase=7;p=add(progress,extra)?;},
 7=>{let value=self.raw.take().ok_or_else(invalid)?;match SealedShared::admit(value,SharedIssuer::owned(),g){Ok((value,progress))=>{*self.sealed=Some(value);p.retained_capacity_bytes=progress.retained_capacity_bytes;self.phase=8;},Err((error,value))=>{*self.raw=Some(value);return Err(error)}}},
 8=>{self.previous=Some(Box::new(Default::default()));self.phase=9;},
 9=>{let stage=self.previous.as_mut().ok_or_else(invalid)?;p=add(stage.advance(S::SLOT.source(original),body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if stage.value().is_some(){self.phase=10;}},
 10=>{let extra=2*size_of::<Option<Option<ArtifactLink>>>()+1;let(value,progress)=self.previous.as_mut().ok_or_else(invalid)?.take(child(g,extra)).map_err(|e|tag(e,extra))?;*self.inverse=Some(value.ok_or_else(invalid)?);self.phase=11;p=add(progress,extra)?;},
 11=>{*self.decision=Some(S::SLOT.diff(self.sealed.take().ok_or_else(invalid)?));self.outcome=Some(RowOutcome::Changed);self.phase=99;},_=>return Err(invalid())}semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original Energy root link intent")?;Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
}
impl<S:RootLinkPath<L>,L:Send+Sync+'static> RowFieldPreparation for RootLinkPreparation<S,L>{
 type Input=L;type Inverse=Option<ArtifactLink>;
 fn new()->Self{Self{authored:None,original:None,equal:RootLinkEquality::new(),input:None,previous:None,raw:ManuallyDrop::new(None),sealed:ManuallyDrop::new(None),decision:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),retirement:Default::default(),phase:0,outcome:None,closing:false,body:None,marker:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,L>,key:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if key.is_some(){return Err(invalid())}self.verify(p,o)}
 fn demand(&self,p:RetainedCloneRef<'_,L>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize)->Result<RetirementDemand,ValueError>{self.decision_demand(p,o,b)}
 fn advance(&mut self,p:RetainedCloneRef<'_,L>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.decide(p,o,b,g)}
 fn ready(&self)->bool{!self.closing&&self.phase==99}fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid())}Ok(scalar(2*size_of::<Option<EnergyModelDiff>>()+1))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None)}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=100||self.inverse.is_none(){return Err(invalid())}Ok(scalar(1))}
 fn prepare_inverse(&mut self,_:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.inverse_demand(0)?;Ok(if retained_mutation_demand_fits(d,g){RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(Default::default())})}
 fn inverse_ready(&self)->bool{!self.closing&&self.phase==100&&self.inverse.is_some()}
 fn take_inverse_demand(&self)->Result<RetirementDemand,ValueError>{if !self.inverse_ready(){return Err(invalid())}Ok(scalar(2*size_of::<Option<Option<ArtifactLink>>>()+1))}
 fn take_inverse(&mut self,g:RetainedCloneGrant)->Result<Option<(Self::Inverse,RetainedCloneProgress)>,ValueError>{let d=self.take_inverse_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None)}self.phase=101;Ok(Some((self.inverse.take().ok_or_else(invalid)?,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;if let Some(p)=self.input.as_mut(){p.begin_close()}if let Some(p)=self.previous.as_mut(){p.begin_close()}}
 fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}let body=self.body.unwrap_or(b);if self.input.is_some(){return cursor_demand(&self.input,body)}if self.previous.is_some(){return cursor_demand(&self.previous,body)}if !self.retirement.is_empty()||self.decision.is_some(){return self.retirement.next_owner_demands::<EnergyModelDiff>(self.decision.is_some(),&None,body)}if self.sealed.is_some(){return self.retirement.next_owner_demands::<SealedShared<Option<ArtifactLink>>>(true,&None,body)}if self.raw.is_some()||self.inverse.is_some(){return self.retirement.next_owner_demands::<Option<ArtifactLink>>(true,&None,body)}RetainedCloneBinding::close_demand(if self.original.is_some(){&self.original}else{&self.authored},body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..g};if self.input.is_some(){return close_cursor(&mut self.input,g).map(RetainedCloneStep::Progress)}if self.previous.is_some(){return close_cursor(&mut self.previous,g).map(RetainedCloneStep::Progress)}if !self.retirement.is_empty(){return self.retirement.select(self.body.unwrap_or(g.maximum_copy_bytes))?.step(g)}if let Some(s)=self.retirement.begin_granted(&mut self.decision,g)?{return Ok(RetainedCloneStep::Progress(s.progress()))}if let Some(s)=self.retirement.begin_granted(&mut self.sealed,g)?{return Ok(RetainedCloneStep::Progress(s.progress()))}if let Some(s)=self.retirement.begin_granted(&mut self.raw,g)?{return Ok(RetainedCloneStep::Progress(s.progress()))}if let Some(s)=self.retirement.begin_granted(&mut self.inverse,g)?{return Ok(RetainedCloneStep::Progress(s.progress()))}RetainedCloneBinding::close_one(if self.original.is_some(){&mut self.original}else{&mut self.authored},g)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.input.is_none()&&self.previous.is_none()&&self.raw.is_none()&&self.sealed.is_none()&&self.decision.is_none()&&self.inverse.is_none()&&self.retirement.is_empty()&&self.original.is_none()&&self.authored.is_none()}
}
impl<S:RootLinkPath<L>,L:Send+Sync+'static> Drop for RootLinkPreparation<S,L>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original root link intent retains full original source and inverse custody");if empty{unsafe{ManuallyDrop::drop(&mut self.raw);ManuallyDrop::drop(&mut self.sealed);ManuallyDrop::drop(&mut self.decision);ManuallyDrop::drop(&mut self.inverse)}}}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
