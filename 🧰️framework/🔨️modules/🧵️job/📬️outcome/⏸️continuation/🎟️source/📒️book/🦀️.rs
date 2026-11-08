//! 🎟️ One genuine source holds only the previously issued currencies of a claimed Job return.
use semio_framework_trace::{OperationId,Generation};
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress as P,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value_retained_clone::retirement::controlled::{RetainedOwnerGate,RetainedOwnerGateGuard};
use semio_framework_job_return_escrow_publication::{OriginalEscrowPublisher,OriginalEscrowPublication};
use semio_framework_job_payload::{OriginalReturnEscrowMessages as E,OriginalReturnEscrowAccounting as M,OriginalReturnStructure as K,require_return_structure as S};
use semio_framework_job_payload::{OriginalReturnEscrowTable,OriginalReturnEscrowRow,OriginalReturnEscrowIdentity,OriginalReturnEscrowAmounts,ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES};

fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
#[derive(Clone,Copy)]
pub struct OriginalReturnSourceInput{source:usize,clock:u64,now:fn()->Option<u64>,deadline:u64,recipient:usize,operation:OperationId,generation:Generation,granted:RetainedCloneGrant,fuel:u64,spent:P,work:u128}
/// 🧭️ The defining source validates provenance while lending its same resident budget.
pub trait OriginalReturnCaptureContext{
 fn capture_return_frame(&self)->Result<OriginalReturnCaptureFrame,ValueError>;
 fn capture_return_budget(&self)->Option<&OriginalReturnBudget>;
}
/// 🪟️ Read-only original caller metadata carries no issued currencies.
pub struct OriginalReturnCaptureFrame{pub source:usize,pub clock:u64,pub now:fn()->Option<u64>,pub deadline:u64,pub operation:OperationId,pub generation:Generation,pub granted:RetainedCloneGrant,pub fuel:u64,pub spent:P,pub work:u128}
impl OriginalReturnSourceInput{
 pub fn capture(cx:&impl OriginalReturnCaptureContext)->Result<Self,ValueError>{let frame=cx.capture_return_frame()?;let budget=cx.capture_return_budget().ok_or_else(||refused("original return source lacks its preborn receiving budget"))?;if let Some(origin)=budget.origin{if origin.clock!=frame.clock||!std::ptr::fn_addr_eq(origin.now,frame.now)||frame.deadline!=origin.deadline{return Err(refused("original return source changed its initiating clock or deadline"))}return Ok(origin)}Ok(Self{source:frame.source,clock:frame.clock,now:frame.now,deadline:frame.deadline,recipient:budget as*const _ as usize,operation:frame.operation,generation:frame.generation,granted:frame.granted,fuel:frame.fuel,spent:frame.spent,work:frame.work})}
 pub fn source(&self)->usize{self.source}pub fn clock(&self)->u64{self.clock}pub fn now(&self)->fn()->Option<u64>{self.now}pub fn deadline(&self)->u64{self.deadline}pub fn recipient(&self)->usize{self.recipient}
}
/// 🔒️ See `README.md`.
pub struct OriginalReturnSource{budget:RetainedOwnerGate<OriginalReturnBudget>}
impl OriginalReturnSource{
 pub const fn empty(birth:u64)->Self{Self{budget:RetainedOwnerGate::new(OriginalReturnBudget::empty(birth))}}
 pub fn try_borrow(&self)->Option<OriginalReturnBudgetReference<'_>>{self.budget.try_lock().ok().map(OriginalReturnBudgetReference::Owned)}
 pub fn guards_header_pointer(&self,pointer:usize)->Result<bool,ValueError>{let b=self.try_borrow().ok_or_else(||refused("header Source busy"))?;Ok(b.get().publisher.as_ref().is_some_and(|p|p.owns_header_pointer(pointer)))}
 pub fn retains_claims(&self)->bool{self.budget.try_lock().map_or(true,|budget|budget.publication.is_some()||budget.claims!=0||budget.held_items!=0||budget.held_copy_bytes!=0||budget.held_fuel!=0||budget.table.as_ref().is_some_and(|table|!table.is_empty()&&!table.is_reclaim_only()))}
 pub fn terminal_is_empty(&self)->bool{self.budget.try_lock().is_ok_and(|budget|budget.origin.is_none()&&budget.table.is_none()&&budget.publisher.is_none()&&budget.publication.is_none()&&!budget.publisher_placed&&!budget.publisher_fenced&&budget.held_items==0&&budget.held_copy_bytes==0&&budget.held_fuel==0&&budget.claims==0)}
 /// 📐️ Quotes the selected original publication, table, claim hold or terminal header under its actual gate.
 pub fn retirement_demands(&self)->Result<semio_framework_value_retained_clone::RetirementDemand,ValueError>{let budget=self.budget.try_lock().map_err(|_|ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original return budget still lends its source"))?;Ok(budget.retirement_selection()?.0)}
 /// ⛽️ Observes the same defining producer's selected work without reserving or changing it.
 pub fn retirement_work_demand(&self)->Result<u64,ValueError>{let budget=self.budget.try_lock().map_err(|_|ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original return budget still lends its source"))?;Ok(budget.retirement_selection()?.1)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let budget=self.budget.get_mut();if budget.publication.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}if let Some(publisher)=budget.publisher.as_ref(){let(copy,release,work)=OriginalEscrowPublisher::retirement_demand();let copy=copy+2*std::mem::size_of::<bool>();if !publisher.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}drop(budget.publisher.take());budget.publisher_placed=false;budget.publisher_fenced=false;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})),work:u128::from(work)}}if let Some(table)=budget.table.as_ref(){if table.has_reclaimable(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}let result=budget.table.as_mut().unwrap().reclaim_one();return RetainedWorkOutcome{result:result.map(|_|RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES,..Default::default()})),work:1}}let bytes=std::mem::size_of::<OriginalReturnEscrowTable>();let copy=std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>();if !table.is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<bytes{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}drop(budget.table.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:copy,released_bytes:bytes,..Default::default()})),work:1}}if budget.origin.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P::default())),work:0}}
  if budget.held_items!=0||budget.held_copy_bytes!=0||budget.held_fuel!=0||budget.claims!=0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<std::mem::size_of::<OriginalReturnBudget>(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}
  *budget=OriginalReturnBudget::empty(budget.birth);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P{copied_items:1,copied_bytes:std::mem::size_of::<OriginalReturnBudget>(),..Default::default()})),work:1}
 }
}
impl Drop for OriginalReturnSource{fn drop(&mut self){let budget=self.budget.get_mut();assert!(std::thread::panicking()||budget.origin.is_none()&&budget.table.is_none()&&budget.publisher.is_none()&&budget.publication.is_none()&&!budget.publisher_placed&&!budget.publisher_fenced&&budget.held_items==0&&budget.held_copy_bytes==0&&budget.held_fuel==0&&budget.claims==0,"original return source retains its bound budget or live claims");}}
pub enum OriginalReturnBudgetReference<'a>{Owned(RetainedOwnerGateGuard<'a,OriginalReturnBudget>),Borrowed(&'a mut OriginalReturnBudget)}
impl OriginalReturnBudgetReference<'_>{
 pub fn get(&self)->&OriginalReturnBudget{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
 pub fn get_mut(&mut self)->&mut OriginalReturnBudget{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
}
pub use semio_framework_job_payload::OriginalReturnBudgetSnapshot;
#[derive(Clone,Copy,PartialEq,Eq)]
enum OriginalReturnDelivery{LocalWitness,CoreWireSnapshot}
pub struct OriginalReturnBudget{birth:u64,delivery:OriginalReturnDelivery,table:Option<Box<OriginalReturnEscrowTable>>,publisher:Option<Box<OriginalEscrowPublisher>>,publication:Option<OriginalEscrowPublication>,publisher_placed:bool,publisher_fenced:bool,origin:Option<OriginalReturnSourceInput>,spent:P,work:u128,held_items:usize,held_copy_bytes:usize,held_release_bytes:usize,held_capacity_bytes:usize,held_fuel:u64,claims:u8,serial:u64,failure:Option<(ValueRefusalKind,&'static str)>}
impl OriginalReturnBudget{
 const fn empty(birth:u64)->Self{Self{birth,delivery:OriginalReturnDelivery::LocalWitness,table:None,publisher:None,publication:None,publisher_placed:false,publisher_fenced:false,origin:None,spent:P{copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0},work:0,held_items:0,held_copy_bytes:0,held_release_bytes:0,held_capacity_bytes:0,held_fuel:0,claims:0,serial:0,failure:None}}
 pub fn is_bound(&self)->bool{self.origin.is_some()}
 pub fn bind(&mut self,input:OriginalReturnSourceInput)->Result<bool,ValueError>{
  if input.recipient!=self as*const Self as usize{return Err(refused("original return lifetime input requires its same preborn receiving budget"))}
  if let Some(origin)=self.origin{if origin.source!=input.source||origin.clock!=input.clock||!std::ptr::fn_addr_eq(origin.now,input.now)||origin.deadline!=input.deadline||origin.recipient!=input.recipient||origin.operation!=input.operation||origin.generation!=input.generation{return Err(refused("original return source cannot be rebound"))}return Ok(false)}
  if !input.spent.fits(input.granted)||input.work>u128::from(input.fuel){return Err(refused("original return source exceeds its admitting lifetime stock"))}
  self.origin=Some(input);self.spent=input.spent;self.work=input.work;Ok(true)
 }
 /// 🔁️ See `README.md`.
 pub fn renew_turn(&mut self)->Result<(),ValueError>{
  let origin=self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;
  if self.claims!=0||self.held_items!=0||self.held_copy_bytes!=0||self.held_fuel!=0||self.failure.is_some(){return Err(refused("original return source with live claims cannot renew its turn"))}
  self.spent=origin.spent;self.work=origin.work;Ok(())
 }
 pub fn snapshot(&self)->OriginalReturnBudgetSnapshot{let(origin,fuel)=self.origin.map_or((RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0},0),|origin|(origin.granted,origin.fuel));OriginalReturnBudgetSnapshot{granted:origin,fuel,spent:self.spent,work:self.work,held_items:self.held_items,held_copy_bytes:self.held_copy_bytes,held_release_bytes:self.held_release_bytes,held_capacity_bytes:self.held_capacity_bytes,held_fuel:self.held_fuel,claims:usize::from(self.claims),serial:self.serial}}
 pub fn refusal(&self)->Option<ValueError>{self.failure.as_ref().map(|(kind,message)|ValueError::literal(kind.clone(),message))}
 pub fn raw_remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if let Some(error)=self.refusal(){return Err(error)}self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;self.snapshot().raw_remaining()}
 pub fn remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if let Some(error)=self.refusal(){return Err(error)}self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;self.snapshot().remaining()}
 pub fn record_retained(&mut self,progress:P)->Result<(),ValueError>{if self.origin.is_none(){return Ok(())}let mut state=self.snapshot();let issue=state.advance_retained(progress);self.spent=state.spent;if let Some(issue)=issue{self.failure=Some(issue.failure());return Err(self.refusal().unwrap().with_retained_progress(progress))}Ok(())}
 pub fn record_work(&mut self,work:u64){if self.origin.is_none(){return}let mut state=self.snapshot();let issue=state.advance_work(work);self.work=state.work;if let Some(issue)=issue{self.failure=Some(issue.failure())}}
 fn claim_inner(&mut self,copy_bytes:usize,destination:usize,epoch:u64)->Result<Option<OriginalReturnClaimCredit>,ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return claim requires its admitted source"))?;if let Some(error)=self.refusal(){return Err(error)}let local=self.delivery==OriginalReturnDelivery::LocalWitness;if destination==0||!local&&epoch==0{return Err(refused("original return claim lacks its genuine currencies, epoch or destination"))}let overhead=2*M::UPDATE+M::FINAL+if local{M::LOCAL_ACK}else{0};let plan=self.snapshot().plan_claim(copy_bytes,original_return_claim_copy_bytes(),overhead)?;
  let identity=OriginalReturnEscrowIdentity{source:origin.source as u64,source_birth:self.birth,recipient:origin.recipient as u64,destination:destination as u64,extent:copy_bytes as u64,serial:plan.serial,origin_epoch:epoch};let held=OriginalReturnEscrowAmounts{items:2,copy_bytes:plan.reserved_copy as u64,work:2,release_bytes:0,capacity_bytes:0};let Some(slot)=self.table.as_mut().ok_or_else(||refused("original return table has not been admitted"))?.reserve(identity,held)?else{return Ok(None)};
  self.held_copy_bytes=plan.held_copy_bytes;self.serial=plan.serial;self.claims=plan.claims;self.held_items=plan.held_items;self.held_fuel=plan.held_fuel;Ok(Some(OriginalReturnClaimCredit{source:origin.source,serial:plan.serial,slot,local,capture:true,receive:true,capture_observed:false,receive_observed:false,live:true}))
 }
 fn redeem_inner(&mut self,credit:&mut OriginalReturnClaimCredit,witness:&mut OriginalReturnWitness)->Result<(),ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return source already closed"))?;if !credit.live&&!witness.redeemed||credit.source!=origin.source||credit.serial!=witness.serial||witness.source!=origin.source{return Err(refused("original return witness is foreign to its held claim"))}if witness.redeemed{return Ok(())}
  let observed=match witness.phase{OriginalReturnPhase::Capture=>credit.capture_observed,OriginalReturnPhase::Receive=>credit.receive_observed};if observed||match witness.phase{OriginalReturnPhase::Capture=>credit.capture,OriginalReturnPhase::Receive=>credit.receive}{return Err(refused("original return witness was not issued once by its claim"))}
  let identity=self.table.as_ref().and_then(|table|table.row_at(credit.slot,credit.serial)).ok_or_else(||refused("original return claim row disappeared"))?.identity;if identity.source!=origin.source as u64||identity.recipient!=origin.recipient as u64{return Err(refused("original return claim row changed Source"))}let extent=usize::try_from(identity.extent).map_err(|_|refused("original return extent exceeds native size"))?;if witness.release_capture&&(witness.phase!=OriginalReturnPhase::Receive||credit.capture_observed){return Err(refused("original return witness lost its genuine held currencies"))}let plan=self.snapshot().plan_redemption(extent,witness.phase==OriginalReturnPhase::Capture,witness.release_capture,credit.local,witness.progress,witness.work,(M::UPDATE,M::FINAL,M::LOCAL_ACK))?;let held=plan.held;let held_copy=plan.held_copy;let spent=plan.spent;let work=plan.work;let actual=plan.actual;let unused=plan.unused;self.table.as_mut().ok_or_else(||refused("original return table disappeared"))?.redeem(identity,actual,unused)?;self.held_items-=usize::from(held);self.held_copy_bytes-=held_copy;self.held_fuel-=u64::from(held);self.spent=spent;self.work=work;match witness.phase{OriginalReturnPhase::Capture=>credit.capture_observed=true,OriginalReturnPhase::Receive=>credit.receive_observed=true}if witness.release_capture{credit.capture_observed=true;}if credit.capture_observed&&credit.receive_observed{self.claims-=1;credit.live=false;}witness.redeemed=true;Ok(())
 }

}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnPhase{Capture,Receive}
pub struct OriginalReturnClaimCredit{source:usize,serial:u64,slot:usize,local:bool,capture:bool,receive:bool,capture_observed:bool,receive_observed:bool,live:bool}
pub fn original_return_claim_copy_bytes()->usize{std::mem::size_of::<OriginalReturnEscrowRow>()+std::mem::size_of::<Option<OriginalReturnClaimCredit>>()+2*std::mem::size_of::<u64>()+4*std::mem::size_of::<u8>()+std::mem::size_of::<usize>()}
impl OriginalReturnClaimCredit{
 pub fn terminal_is_empty(&self)->bool{!self.live&&!self.capture&&!self.receive&&self.capture_observed&&self.receive_observed}
 pub fn has_unused_capture(&self)->bool{self.live&&self.capture&&self.receive}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase,copy_bytes:usize)->Result<OriginalReturnWitness,ValueError>{if !self.live||!match phase{OriginalReturnPhase::Capture=>self.capture,OriginalReturnPhase::Receive=>self.receive&&!self.capture}{return Err(refused("original return claim phase was already issued or requires capture"))}let witness=self.witness(phase,false,copy_bytes)?;match phase{OriginalReturnPhase::Capture=>self.capture=false,OriginalReturnPhase::Receive=>self.receive=false}Ok(witness)}
 pub fn cancel_preissued(&mut self,w:&mut OriginalReturnWitness)->Result<(),ValueError>{if !self.live||self.capture||!self.receive||self.capture_observed||self.receive_observed||w.source!=self.source||w.serial!=self.serial||w.phase!=OriginalReturnPhase::Capture||w.redeemed||w.received{return Err(refused("original preissued capture cannot be cancelled"))}let copy=M::witness_copy(false,self.local,0)?;w.phase=OriginalReturnPhase::Receive;w.progress.copied_bytes=copy;w.release_capture=true;self.receive=false;Ok(())}
 pub fn issue_success_return(&mut self,copy_bytes:usize)->Result<OriginalReturnWitness,ValueError>{if !self.has_unused_capture(){return Err(refused("normal original return requires its unused capture and receiving holds"))}let witness=self.witness(OriginalReturnPhase::Receive,true,copy_bytes)?;self.capture=false;self.receive=false;Ok(witness)}
 fn witness(&self,phase:OriginalReturnPhase,release_capture:bool,copy_bytes:usize)->Result<OriginalReturnWitness,ValueError>{let copied=M::witness_copy(phase==OriginalReturnPhase::Capture,self.local,copy_bytes)?;Ok(OriginalReturnWitness{source:self.source,serial:self.serial,phase,progress:P{copied_items:1,copied_bytes:copied,..Default::default()},work:1,release_capture,redeemed:false,received:false})}

}
impl Drop for OriginalReturnClaimCredit{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original return claim retains unspent original currencies");}}
pub struct OriginalReturnWitness{source:usize,serial:u64,phase:OriginalReturnPhase,progress:P,work:u64,release_capture:bool,redeemed:bool,received:bool}
impl OriginalReturnWitness{pub fn is_capture(&self)->bool{self.phase==OriginalReturnPhase::Capture}pub fn releases_unused_capture(&self)->bool{self.release_capture}}
impl Drop for OriginalReturnWitness{fn drop(&mut self){assert!(std::thread::panicking()||self.received,"original held return witness requires its actual receiving caller");}}

impl OriginalReturnBudget{
 pub fn admit_table(&mut self,grant:RetainedCloneGrant,fuel:u64)->RetainedWorkOutcome<Option<P>>{if self.table.is_some(){return RetainedWorkOutcome{result:Ok(None),work:0}}let turn=OriginalReturnEscrowTable::admit(grant,fuel);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|(table,progress)|table.map(|table|{self.table=Some(table);progress}))}}
 pub fn origin(&self)->Option<OriginalReturnSourceInput>{self.origin}
 pub fn local_delivery(&self)->bool{self.delivery==OriginalReturnDelivery::LocalWitness}
 pub fn table_is_admitted(&self)->bool{self.table.is_some()}
 pub fn table_has_capacity(&self)->bool{self.table.as_ref().is_some_and(|table|table.has_capacity())}
 pub fn table_has_reclaimable(&self)->bool{self.table.as_ref().is_some_and(|table|table.has_reclaimable())}
 pub fn reclaim_one(&mut self)->Result<bool,ValueError>{self.table.as_mut().ok_or_else(||refused("original return table disappeared"))?.reclaim_one()}

 pub fn switch_core_delivery(&mut self,grant:RetainedCloneGrant,fuel:u64)->RetainedWorkOutcome<bool>{if !self.local_delivery(){return RetainedWorkOutcome{result:Ok(true),work:0}}if self.serial!=0||self.claims!=0{return RetainedWorkOutcome{result:Err(refused("core-wire delivery must precede every Source claim")),work:0}}let bytes=std::mem::size_of::<OriginalReturnDelivery>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||fuel==0{return RetainedWorkOutcome{result:Ok(false),work:0}}self.delivery=OriginalReturnDelivery::CoreWireSnapshot;RetainedWorkOutcome{result:Ok(true),work:1}}
 pub fn core_delivery_ready(&self)->Result<bool,ValueError>{if !self.local_delivery(){return Ok(true)}if self.serial!=0||self.claims!=0{return Err(refused("core-wire delivery must precede every Source claim"))}Ok(false)}
 pub fn delivery_copy_bytes()->usize{std::mem::size_of::<OriginalReturnDelivery>()}
 pub fn publisher_is_admitted(&self)->bool{self.publisher.is_some()}
 pub fn publisher_is_ready(&self)->bool{self.publisher_placed&&!self.publisher_fenced}
 pub fn fence_publisher(&mut self){self.publisher_fenced=true}
 pub fn admit_publisher(&mut self,grant:RetainedCloneGrant,fuel:u64)->RetainedWorkOutcome<Option<P>>{if self.publisher.is_some(){return RetainedWorkOutcome{result:Err(refused("original escrow publisher already admitted")),work:0}}let turn=OriginalEscrowPublisher::admit(grant,fuel);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|(publisher,mut progress)|publisher.map(|publisher|{progress.copied_bytes+=std::mem::size_of::<bool>();self.publisher=Some(publisher);self.publisher_placed=true;progress}))}}
 pub fn publish_extent(&mut self,source:usize,header:usize,grant:RetainedCloneGrant,fuel:u64)->Result<Option<OriginalEscrowPublication>,ValueError>{if self.local_delivery()||!self.publisher_is_ready()||self.publication.is_some(){return Err(refused("Core publication fenced"))}let origin=self.origin.ok_or_else(||refused("Core Source unbound"))?;if origin.source!=source||origin.recipient!=self as*const _ as usize{return Err(refused("Core Source changed"))}let table=self.table.as_mut().ok_or_else(||refused("Core table absent"))?;let publisher=self.publisher.as_mut().ok_or_else(||refused("Core publisher absent"))?;let publication=publisher.publish_extent(origin.source as u64,self.birth,origin.recipient as u64,table,header,grant,fuel)?;self.publication=publication;Ok(publication)}
 pub fn borrowed_publication(&self)->Result<&[u8],ValueError>{if self.publication.is_none(){return Err(refused("Core extent absent"))}self.publisher.as_ref().and_then(|p|p.borrowed()).ok_or_else(||refused("Core bytes absent"))}
 pub fn finish_header(&mut self,source:usize,loan:&mut semio_framework_job_return_escrow_publication::OriginalEscrowHeaderLoanCell,grant:RetainedCloneGrant,fuel:u64)->Result<bool,ValueError>{if self.local_delivery()||!self.publisher_is_ready()||self.publication.is_none(){return Err(refused("header publisher fenced"))}if let Some(error)=self.refusal(){return Err(error)}let origin=self.origin.ok_or_else(||refused("header Source unbound"))?;if origin.source!=source||origin.recipient!=self as*const _ as usize{return Err(refused("header Source changed"))}self.publisher.as_mut().ok_or_else(||refused(E::PUBLISHER_ABSENT))?.finish_header(loan,grant,fuel)}
 pub fn claim(&mut self,copy_bytes:usize,destination:usize,epoch:u64)->Result<Option<OriginalReturnClaimCredit>,ValueError>{self.claim_inner(copy_bytes,destination,epoch)}
 pub fn redeem(&mut self,credit:&mut OriginalReturnClaimCredit,witness:&mut OriginalReturnWitness)->Result<(),ValueError>{self.redeem_inner(credit,witness)}
 pub fn finish_received(&mut self,credit:&OriginalReturnClaimCredit,witness:&mut OriginalReturnWitness)->Result<(),ValueError>{witness.received=true;if credit.terminal_is_empty()&&self.local_delivery(){let table=self.table.as_mut().ok_or_else(||refused("original return table disappeared"))?;let identity=table.row_at(credit.slot,credit.serial).ok_or_else(||refused("original return claim row disappeared"))?.identity;table.mark_local_received(identity)?}Ok(())}
}
impl OriginalReturnClaimCredit{
 pub fn source_refusal(&self,budget:Option<&OriginalReturnBudget>)->Option<ValueError>{match budget.and_then(OriginalReturnBudget::origin){Some(origin)if self.live&&origin.source==self.source=>None,_=>Some(refused("original return body lost its claimed source"))}}
 pub fn validate_cached(&self,budget:&OriginalReturnBudget,destination:usize,copy_bytes:usize)->Result<(),ValueError>{let origin=budget.origin.ok_or_else(||refused("cached original return claim source is unbound"))?;if !self.live||budget.table.as_ref().and_then(|table|table.row_at(self.slot,self.serial)).is_none_or(|row|row.identity.destination!=destination as u64||row.identity.extent!=copy_bytes as u64)||self.source!=origin.source||self.serial>budget.serial{return Err(refused("cached original return claim is foreign or already consumed"))}Ok(())}
}
impl OriginalReturnWitness{pub fn source(&self)->usize{self.source}pub fn progress(&self)->P{self.progress}pub fn work(&self)->u64{self.work}pub fn is_received(&self)->bool{self.received}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
impl OriginalReturnBudget{
 fn retirement_selection(&self)->Result<(semio_framework_value_retained_clone::RetirementDemand,u64),ValueError>{use semio_framework_value_retained_clone::RetirementDemand as D;if self.publication.is_some(){return Ok((D::default(),0))}if let Some(publisher)=self.publisher.as_ref(){if !publisher.terminal_is_empty(){return Ok((D::default(),0))}let(copy,release,work)=OriginalEscrowPublisher::retirement_demand();return Ok((D{copy_bytes:copy+2*std::mem::size_of::<bool>(),release_bytes:release,depth:1,..Default::default()},work))}if let Some(table)=self.table.as_ref(){if table.has_reclaimable(){return Ok((D{copy_bytes:ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES,depth:1,..Default::default()},1))}if !table.is_empty(){return Ok((D::default(),0))}return Ok((D{copy_bytes:std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>(),release_bytes:std::mem::size_of::<OriginalReturnEscrowTable>(),depth:1,..Default::default()},1))}if self.origin.is_none()||self.held_items!=0||self.held_copy_bytes!=0||self.held_fuel!=0||self.claims!=0{return Ok((D::default(),0))}Ok((D{copy_bytes:std::mem::size_of::<Self>(),depth:1,..Default::default()},1))}
}
