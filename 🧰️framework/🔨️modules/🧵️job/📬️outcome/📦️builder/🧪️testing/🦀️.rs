//! 🧪️ One finite payload-domain issuer exercises the public port without claiming a host context.
use semio_framework_job_payload::{JobPayloadAuthority,JobPayloadOperationLedger,JobPayloadStep,JobPayloadStream,RetainedPayloadBuilder,RetainedJobPayload};
use semio_framework_trace::{OperationId,Generation};
use semio_framework_job_payload::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError};
use semio_framework_value_control_contract::RetainedAllocationEffect;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use std::sync::Arc;
pub struct PayloadCase{authority:JobPayloadAuthority,stock:RetainedCloneGrant,spent:RetainedCloneProgress,work:u128,offered:RetainedCloneGrant,granted:bool,bank:EffectBank}
impl PayloadCase{
 pub fn new(operation:OperationId,generation:Generation)->Self{
  let law:serde_json::Value=serde_json::from_str(include_str!("case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("case.schema.json")).unwrap();for(key,value)in law.as_object().unwrap(){assert_eq!(&schema["properties"][key]["const"],value);}
  let s=&law["stock"];let stock=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:s["release"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};
  let mut actual=None;let turn=JobPayloadAuthority::admit(operation,generation,stock,&mut actual);assert_eq!(turn.work,u128::from(JobPayloadAuthority::admission_work_demand()));let(authority,spent)=turn.result.unwrap().unwrap();assert!(spent.fits(stock));Self{authority,stock,spent,work:turn.work,offered:Default::default(),granted:false,bank:EffectBank::new(Wallet::new(stock,stock.maximum_items as u64))}
 }
 pub fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
 pub fn apply<T>(&mut self,offer:RetainedCloneGrant,body:impl FnOnce(&mut Self)->RetainedWorkOutcome<T>,effects:impl FnOnce(&T)->RetainedCloneProgress)->T{
  self.offered=offer;self.granted=false;let grant=self.retained_grant();let turn=body(self);self.settle(grant,turn,effects)
 }
 fn settle<T>(&mut self,grant:RetainedCloneGrant,turn:RetainedWorkOutcome<T>,effects:impl FnOnce(&T)->RetainedCloneProgress)->T{
  assert!(turn.work<=grant.maximum_items as u128);let p=match &turn.result{Ok(value)=>effects(value),Err(error)=>error.retained_progress()};assert!(p.fits(grant));self.work=self.work.checked_add(turn.work).unwrap();self.spent=self.spent.checked_add(p).unwrap();assert!(self.work<=self.stock.maximum_items as u128);assert!(self.spent.fits(self.stock));turn.result.unwrap()
 }
 /// 🪙️ Runs one effect-threaded closer under the finite stock and returns its progress; `work` is the closer's own quote.
 pub fn close_with(&mut self,offer:RetainedCloneGrant,work:u64,body:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->RetainedCloneStep{
  self.offered=offer;self.granted=false;let grant=self.retained_grant();let turn=self.bank.run_outcome(grant,work,body);let _=self.bank.take_admitted();self.settle(grant,turn,|step|step.progress())
 }
 /// 🪙️ Funds the effect-threaded closer with exactly its quoted demand.
 pub fn close(&mut self,demand:RetirementDemand,work:u64,body:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->RetainedCloneStep{self.close_with(Self::exact(demand),work,body)}
 pub fn step(&mut self,offer:RetainedCloneGrant,body:impl FnOnce(&mut Self)->RetainedWorkOutcome)->RetainedCloneStep{self.apply(offer,body,|step|step.progress())}
 pub fn initialize(&mut self,stream:JobPayloadStream)->RetainedPayloadBuilder{let mut b=RetainedPayloadBuilder::new(stream);for _ in 0..1024{if b.is_initialized(){return b}self.step(Self::exact(b.initialization_demand()),|cx|b.advance_initialization(cx));}panic!("finite payload initialization did not settle")}
 pub fn stage(&mut self,b:&mut RetainedPayloadBuilder,bytes:&[u8]){for _ in 0..16{if b.staged_page_len().is_some(){break}self.step(Self::exact(b.staged_page_birth_demands().unwrap()),|cx|b.begin_staged_page_funded(cx));}assert_eq!(b.staged_page_len(),Some(0));self.step(Self::exact(b.staged_source_demands(bytes).unwrap()),|cx|b.write_staged_source_funded(cx,bytes));}
 pub fn page(&mut self,b:&mut RetainedPayloadBuilder,bytes:&[u8]){self.stage(b,bytes);self.step(Self::exact(b.staged_commit_demands().unwrap()),|cx|b.commit_staged_page_funded(cx));}
 pub fn finish(&mut self,b:&mut RetainedPayloadBuilder)->RetainedJobPayload{self.step(Self::exact(RetainedPayloadBuilder::seal_demand()),|cx|b.seal(cx));let mut out=None;self.step(Self::exact(RetainedPayloadBuilder::take_finished_demand()),|cx|b.take_finished(&mut out,cx.retained_grant()));assert!(b.terminal_is_empty());out.unwrap()}
 pub fn authority(&self)->&JobPayloadAuthority{&self.authority}
 pub fn source(&mut self,extent:usize)->semio_framework_job_payload::JobPayloadPageSource{let d=RetirementDemand{capacity_bytes:extent,depth:1,..Default::default()};self.apply(Self::exact(d),|cx|semio_framework_job_payload::JobPayloadPageSource::admit(extent,cx.retained_grant()),|value|value.as_ref().map_or(Default::default(),|(_,p)|*p)).unwrap().0}
 pub fn alias(&mut self,ledger:&Arc<JobPayloadOperationLedger>)->Arc<JobPayloadOperationLedger>{self.apply(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},|_|RetainedWorkOutcome{result:Ok(Arc::clone(ledger)),work:1},|_|RetainedCloneProgress{copied_items:1,..Default::default()})}
 pub fn close_ledger(&mut self,ledger:&mut Option<Arc<JobPayloadOperationLedger>>){let d=semio_framework_job_payload::ledger_demands(ledger);let work=semio_framework_job_payload::ledger_work_demand(ledger);self.close(d,work,|effect|semio_framework_job_payload::close_ledger(ledger,effect));}
 pub fn detached(operation:OperationId,generation:Generation)->Self{let(ledger,mut case)=Self::new(operation,generation).into_ledger();case.close_ledger(&mut Some(ledger));case}
 pub fn into_ledger(mut self)->(Arc<JobPayloadOperationLedger>,Self){
  let ledger=self.apply(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},|cx|RetainedWorkOutcome{result:Ok(Arc::clone(cx.payload_ledger())),work:1},|_|RetainedCloneProgress{copied_items:1,..Default::default()});
  let g=Self::exact(self.authority.retirement_demands());let work=self.authority.retirement_work_demand().unwrap();assert_eq!(work,1);self.offered=g;self.granted=false;let grant=self.retained_grant();let turn=self.bank.run_outcome(grant,work,|effect|self.authority.close_step(effect));let _=self.bank.take_admitted();assert_eq!(turn.work,1);let step=self.settle(grant,turn,|step|step.progress());assert!(step.progress().fits(g));(ledger,self)
 }
}
impl JobPayloadStep for PayloadCase{
 fn payload_ledger(&self)->&Arc<JobPayloadOperationLedger>{self.authority.original().unwrap()}
 fn payload_page_granted(&self)->bool{self.granted}
 fn grant_payload_page(&mut self){self.granted=true}
 fn is_cancelled(&self)->bool{false}
 fn should_yield(&self)->bool{self.work>=self.stock.maximum_items as u128}
 fn retained_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.offered.maximum_items.min(self.stock.maximum_items.saturating_sub(self.work as usize)),maximum_copy_bytes:self.offered.maximum_copy_bytes.min(self.stock.maximum_copy_bytes-self.spent.copied_bytes),maximum_capacity_bytes:self.offered.maximum_capacity_bytes.min(self.stock.maximum_capacity_bytes-self.spent.retained_capacity_bytes),maximum_release_bytes:self.offered.maximum_release_bytes.min(self.stock.maximum_release_bytes-self.spent.released_bytes),maximum_depth:self.offered.maximum_depth.min(self.stock.maximum_depth)}}
 fn consume_retained(&mut self,_:RetainedCloneProgress)->Result<(),ValueError>{panic!("payload owner must return its unposted actual outcome")}
}
