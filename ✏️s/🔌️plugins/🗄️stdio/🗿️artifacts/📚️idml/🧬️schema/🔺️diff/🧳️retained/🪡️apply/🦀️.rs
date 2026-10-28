//! 🪡️ Central IDML commit moves untouched live fields and stages only genuine authored fragments.
use super::overlay::{IdmlOverlayCursor,IdmlFieldAddress};
use crate::{IdmlSnapshot,IdmlDiff,schema::IdmlPart};
use crate::io::admission::IdmlAdmissionGrant;
use semio_framework_os_kernel as protocol;
use protocol::{RetainedDiffApply,RetainedDiffPreflightAdvance,RetainedFieldPreparation,RetainedDiagnosticPreparation,RetainedDiagnosticSpec,MutationMessage,MutationMessageLedgerRetirement,ApplyCapability};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{ManuallyDrop,size_of};
#[derive(Default,semio_framework_value::RetireOwned)]
struct Member{path:Option<String>,bytes:Option<Vec<u8>>}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=128;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-128,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=128;p.copied_items=p.copied_items.max(1);p}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn path_source<'a>(diff:RetainedCloneRef<'a,IdmlDiff>,at:IdmlFieldAddress)->RetainedCloneRef<'a,String>{let step=at.step.expect("authored path");diff.project(step,move|diff|&diff.steps[step]).project(at.part,move|patch|&patch.parts[at.part]).project(0,|part|&part.path)}
fn bytes_source<'a>(diff:RetainedCloneRef<'a,IdmlDiff>,at:IdmlFieldAddress)->RetainedCloneRef<'a,Vec<u8>>{let step=at.step.expect("authored bytes");diff.project(step,move|diff|&diff.steps[step]).project(at.part,move|patch|&patch.parts[at.part]).project(1,|part|part.bytes.as_ref().expect("live authored bytes"))}
fn version_source<'a>(diff:RetainedCloneRef<'a,IdmlDiff>,step:usize)->RetainedCloneRef<'a,String>{diff.project(step,move|diff|&diff.steps[step]).project(2,|patch|patch.dom_version.as_ref().expect("authored version"))}
/// 🧳️ All partial, authored and displaced ownership remains closeable under the central host.
pub struct RetainedIdmlApply{overlay:IdmlOverlayCursor,path:RetainedFieldPreparation<String>,bytes:RetainedFieldPreparation<Vec<u8>>,version:RetainedFieldPreparation<String>,staged:ManuallyDrop<Option<Vec<Member>>>,output:ManuallyDrop<Option<Vec<IdmlPart>>>,retirement:RetainedCloneClose,diagnostic:RetainedDiagnosticPreparation<0>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,phase:u8,index:usize,field:u8,prepared:bool,committed:bool,closing:bool,error:Option<&'static str>}
impl protocol::RetainedDiff<IdmlSnapshot> for IdmlDiff{type Apply=RetainedIdmlApply;}
impl RetainedIdmlApply{
 fn work(body:usize)->usize{body.saturating_sub(512).checked_div(64).unwrap_or(0).clamp(1,1024)}
 fn overlay_demand(&self,body:usize)->RetirementDemand{RetirementDemand{copy_bytes:Self::work(body)*64+512+self.overlay.next_copy_bytes(),capacity_bytes:self.overlay.next_capacity_bytes(),release_bytes:self.overlay.advance_release_bytes(),depth:2}}
 fn transfer_field()->RetirementDemand{fixed(size_of::<Member>()*2+128)}
 fn reserve_demand(&self)->RetirementDemand{let count=self.overlay.member_count().expect("admitted numeric members");RetirementDemand{copy_bytes:size_of::<Vec<Member>>()+size_of::<Vec<IdmlPart>>()+128,capacity_bytes:count*(size_of::<Member>()+size_of::<IdmlPart>()),depth:1,..Default::default()}}
 fn close_field_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.field==3{self.path.close_demand(body)}else{self.bytes.close_demand(body)}}
}
impl RetainedDiffApply<IdmlSnapshot,IdmlDiff> for RetainedIdmlApply{
 fn new()->Self{Self{overlay:IdmlOverlayCursor::new(),path:Default::default(),bytes:Default::default(),version:Default::default(),staged:ManuallyDrop::new(Some(Vec::new())),output:ManuallyDrop::new(Some(Vec::new())),retirement:Default::default(),diagnostic:Default::default(),messages:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),phase:0,index:0,field:0,prepared:false,committed:false,closing:false,error:None}}
 fn preflight_demand(&self,diff:RetainedCloneRef<'_,IdmlDiff>,_:RetainedCloneRef<'_,IdmlSnapshot>,_:RetainedCloneRef<'_, IdmlSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.prepared{return Err(fault("IDML central preflight already settled"));}
  Ok(match self.phase{
   0=>self.overlay_demand(body),1=>self.reserve_demand(),2=>{
    if self.index==self.overlay.member_count().expect("admitted members"){fixed(128)}else{let(path,bytes)=self.overlay.member(self.index).expect("resolved member");match self.field{
     0|4|8=>fixed(256),1=>if self.path.value().is_some(){Self::transfer_field()}else{nested(self.path.demand(path_source(diff,path),body)?)},2=>Self::transfer_field(),3|7=>nested(self.close_field_demand(body)?),5=>if self.bytes.value().is_some(){Self::transfer_field()}else{nested(self.bytes.demand(bytes_source(diff,bytes),body)?)},6=>Self::transfer_field(),_=>return Err(fault("IDML authored field phase"))}}
   },3=>{if let Some(step)=self.overlay.version_step(){if self.version.value().is_some(){fixed(128)}else{nested(self.version.demand(version_source(diff,step),body)?)}}else{fixed(128)}},100=>nested(RetainedDiagnosticPreparation::<0>::start_demand()),101=>nested(self.diagnostic.demand([],body)?),102=>fixed(size_of::<Vec<MutationMessage>>()*3+128),_=>return Err(fault("IDML preflight phase"))
  })
 }
 fn preflight(&mut self,diff:RetainedCloneRef<'_,IdmlDiff>,original:RetainedCloneRef<'_,IdmlSnapshot>,post:RetainedCloneRef<'_, IdmlSnapshot>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>{
  let d=self.preflight_demand(diff,original,post,body)?;if !protocol::retained_mutation_demand_fits(d,grant){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}receiving.checkpoint()?;
  let mut p=progress(d);
  match self.phase{
   0=>{let step=self.overlay.advance(post.get(),diff.get(),IdmlAdmissionGrant{maximum_work:Self::work(body),maximum_copy_bytes:grant.maximum_copy_bytes-Self::work(body)*64-512,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes});p=RetainedCloneProgress{copied_items:1,copied_bytes:step.work*64+512+step.copied_bytes,retained_capacity_bytes:step.retained_capacity_bytes,released_bytes:step.released_bytes,..Default::default()};if let Some(error)=step.error{self.error=Some(error);self.phase=100;}else if step.done{self.phase=1;}},
   1=>{let count=self.overlay.member_count().expect("admitted members");let staged=self.staged.as_mut().expect("staged member metadata");let output=self.output.as_mut().expect("output metadata");let result=staged.try_reserve_exact(count).and_then(|_|output.try_reserve_exact(count));p.retained_capacity_bytes=staged.capacity()*size_of::<Member>()+output.capacity()*size_of::<IdmlPart>();if result.is_err()||p.retained_capacity_bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"IDML typed field reservation failed").with_retained_progress(p));}self.phase=2;},
   2=>{if self.index==self.overlay.member_count().expect("admitted members"){self.phase=3;}else{let(path,bytes)=self.overlay.member(self.index).expect("resolved original fields");match self.field{
    0=>{self.staged.as_mut().expect("staged metadata").push(Member::default());self.field=if path.step.is_some(){1}else if bytes.step.is_some(){5}else{8};},
    1=>{if self.path.value().is_some(){self.field=2;}else{let step=self.path.advance(path_source(diff,path),body,child(grant))?;p=receipt(step.progress());}},
    2=>{let(value,paid)=self.path.take(child(grant))?;p=receipt(paid);self.staged.as_mut().expect("staged path")[self.index].path=value;self.path.begin_close();self.field=3;},
    3=>{let step=self.path.close_step(child(grant))?;p=receipt(step.progress());if self.path.terminal_is_empty(){self.field=4;}},
    4=>{self.path=Default::default();self.field=if bytes.step.is_some(){5}else{8};},
    5=>{if self.bytes.value().is_some(){self.field=6;}else{let step=self.bytes.advance(bytes_source(diff,bytes),body,child(grant))?;p=receipt(step.progress());}},
    6=>{let(value,paid)=self.bytes.take(child(grant))?;p=receipt(paid);self.staged.as_mut().expect("staged payload")[self.index].bytes=value;self.bytes.begin_close();self.field=7;},
    7=>{let step=self.bytes.close_step(child(grant))?;p=receipt(step.progress());if self.bytes.terminal_is_empty(){self.field=8;}},
    8=>{if self.bytes.terminal_is_empty(){self.bytes=Default::default();}self.index+=1;self.field=0;},_=>return Err(fault("IDML field phase"))}}},
   3=>{if let Some(step)=self.overlay.version_step(){if self.version.value().is_none(){let step=self.version.advance(version_source(diff,step),body,child(grant))?;return Ok(RetainedDiffPreflightAdvance::Progress(receipt(step.progress())));}}self.prepared=true;self.index=0;return Ok(RetainedDiffPreflightAdvance::Ready(p,true));},
   100=>{p=receipt(self.diagnostic.start(RetainedDiagnosticSpec{level:semio_framework_diagnostic::Severity::Fatal,code:self.error.unwrap_or("stdio.idml.invariant"),message:"The authored IDML batch is inadmissible."},child(grant))?);self.phase=101;},
   101=>{let step=self.diagnostic.advance([],body,child(grant))?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=102;}},
   102=>{let(rows,paid)=self.diagnostic.take(child(grant))?.expect("funded IDML refusal diagnostic");p=receipt(paid);*self.messages=Some(rows);self.prepared=true;return Ok(RetainedDiffPreflightAdvance::Ready(p,false));},_=>return Err(fault("IDML preflight phase"))
  }Ok(RetainedDiffPreflightAdvance::Progress(p))
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{self.messages.take().unwrap_or_default()}
 fn commit_demand(&self,_:RetainedCloneRef<'_,IdmlDiff>,_:RetainedCloneRef<'_,IdmlSnapshot>,_:&IdmlSnapshot,_:usize)->Result<RetirementDemand,ValueError>{if !self.prepared||self.committed||self.error.is_some()||self.closing{return Err(fault("IDML commit lacks admitted preflight"));}if self.index<self.overlay.member_count().expect("admitted members"){Ok(fixed(size_of::<IdmlPart>()*5+size_of::<Member>()*3+256))}else if self.overlay.version_step().is_some()&&self.phase!=5{let mut d=self.version.exchange_demand()?;d.copy_bytes+=128;Ok(d)}else{Ok(fixed(size_of::<Vec<IdmlPart>>()*4+256))}}
 fn commit(&mut self,diff:RetainedCloneRef<'_,IdmlDiff>,original:RetainedCloneRef<'_,IdmlSnapshot>,post:&mut IdmlSnapshot,grant:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let d=self.commit_demand(diff,original,post,grant.maximum_copy_bytes)?;if !protocol::retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}let _authority=capability;let mut p=progress(d);
  if self.index<self.overlay.member_count().expect("admitted members"){let(path,_)=self.overlay.member(self.index).expect("resolved commit row");let fields=&mut self.staged.as_mut().expect("staged authored fields")[self.index];let mut row=if path.step.is_none(){std::mem::replace(&mut post.parts[path.part],IdmlPart{path:String::new(),bytes:Vec::new()})}else{IdmlPart{path:fields.path.take().expect("staged authored path"),bytes:Vec::new()}};if let Some(bytes)=fields.bytes.take(){fields.bytes=Some(std::mem::replace(&mut row.bytes,bytes));}self.output.as_mut().expect("reserved final members").push(row);self.index+=1;return Ok(RetainedCloneStep::Progress(p));}
  if self.overlay.version_step().is_some()&&self.phase!=5{let mut receipt=self.version.install(&mut post.dom_version,capability,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-128,..grant})?;receipt.copied_bytes+=128;self.phase=5;return Ok(RetainedCloneStep::Progress(receipt));}
  let output=self.output.take().expect("admitted final member vector");*self.output=Some(std::mem::replace(&mut post.parts,output));self.committed=true;p.copied_items=1;Ok(RetainedCloneStep::Complete(p))
 }
 fn begin_close(&mut self){self.closing=true;self.path.begin_close();self.bytes.begin_close();self.version.begin_close();self.diagnostic.begin_close();self.overlay.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.path.terminal_is_empty(){return self.path.close_demand(body);}if !self.bytes.terminal_is_empty(){return self.bytes.close_demand(body);}if !self.version.terminal_is_empty(){return self.version.close_demand(body);}if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body);}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Vec<Member>>(false,&None,body);}if self.staged.is_some(){return self.retirement.next_owner_demands::<Vec<Member>>(true,&None,body);}if self.output.is_some(){return self.retirement.next_owner_demands::<Vec<IdmlPart>>(true,&None,body);}if self.messages.is_some(){return Ok(fixed(size_of::<Vec<MutationMessage>>()+size_of::<MutationMessageLedgerRetirement>()));}if let Some(ledger)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_demand(ledger,body);}if !self.overlay.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:128,release_bytes:self.overlay.next_release_bytes(),depth:1,..Default::default()});}Ok(Default::default())
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(fault("IDML apply close not started"));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !protocol::retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.path.terminal_is_empty(){return self.path.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.bytes.terminal_is_empty(){return self.bytes.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.version.terminal_is_empty(){return self.version.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(step)=self.retirement.begin_granted(&mut self.staged,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}if let Some(step)=self.retirement.begin_granted(&mut self.output,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}if let Some(rows)=self.messages.take(){*self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),rows));return Ok(RetainedCloneStep::Progress(progress(d)));}if let Some(ledger)=self.ledger.as_mut(){let step=ledger.close_step(grant)?;if ledger.terminal_is_empty(){*self.ledger=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}if !self.overlay.terminal_is_empty(){let released=self.overlay.close_one(grant.maximum_release_bytes);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:released,..progress(d)}));}Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.path.terminal_is_empty()&&self.bytes.terminal_is_empty()&&self.version.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.staged.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.messages.is_none()&&self.ledger.is_none()&&self.overlay.terminal_is_empty()}
}
impl Drop for RetainedIdmlApply{fn drop(&mut self){let empty=<Self as RetainedDiffApply<IdmlSnapshot,IdmlDiff>>::terminal_is_empty(self);assert!(std::thread::panicking()||empty,"IDML central apply requires exact typed source closure");if empty{unsafe{ManuallyDrop::drop(&mut self.staged);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.messages);ManuallyDrop::drop(&mut self.ledger);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
