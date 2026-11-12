//! 🧪️ Original ordered sparse decisions share SQLite output and genuine typed source custody.
#[path="../../../../../../🌱️value/📽️projection/👁️post/🧪️testing/🦀️.rs"]
mod post_fixture;
use post_fixture::PostFixture;
use protocol::{RetainedDiffSequence,RetainedDiff,RetainedDiffFragment,RetainedDiffApply,RetainedDiffOwner,RetainedDiffPreflightAdvance,RetainedDiffPreflightStep,RetainedFieldPreparation,RetainedDiagnosticPreparation,RetainedDiagnosticSpec,MutationDiff,DiffAlgebra,MutationMessage,ApplyCapability};
use protocol::retained::{Done,Reply,native_child,settle,settle_exit,settle_value};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,FromValue,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_clone_source_testing::{FixtureSource,drive::{drive,drive_reply}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value_testing_allocator as _;
const UNITS:u64=1<<20;
const BODY:usize=4096;
#[derive(Debug,PartialEq,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone,semio_framework_value::SnapshotProjection,semio_framework_value::ToValue,semio_framework_value::FromValue)]
struct Snapshot{a:i64,b:i64,text:String}
impl Clone for Snapshot{fn clone(&self)->Self{panic!("ordered native candidate must use actual SnapshotProjection")}}
#[derive(Clone,Default,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase")]
struct Edit{#[value(default)]set_a:Option<i64>,#[value(default)]set_b:Option<i64>,#[value(default)]require_a:Option<i64>,#[value(default)]warning:bool}
struct EditApply{a:RetainedFieldPreparation<i64>,b:RetainedFieldPreparation<i64>,diagnostic:RetainedDiagnosticPreparation<0>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,retirement:RetainedCloneClose,phase:u8,part:u8,refused:bool,closing:bool}
impl RetainedDiffFragment<Snapshot> for Edit{type FragmentApply=EditApply;}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original ordered fixture is outside its typed frontier")}
fn control()->usize{size_of::<usize>()*4+size_of::<bool>()*2+size_of::<u8>()*2}
fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:control(),depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=control();d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-control(),maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{if p!=Default::default(){p.copied_bytes+=control();}p}
fn source_a(d:RetainedCloneRef<'_,Edit>)->RetainedCloneRef<'_,i64>{d.project(1,|d|d.set_a.as_ref().unwrap())}
fn source_b(d:RetainedCloneRef<'_,Edit>)->RetainedCloneRef<'_,i64>{d.project(2,|d|d.set_b.as_ref().unwrap())}
impl EditApply{
 fn preflight_step(&mut self,d:RetainedCloneRef<'_,Edit>,s:RetainedCloneRef<'_,Snapshot>,post:RetainedCloneRef<'_,Snapshot>,body:usize,max:u64,eff:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedDiffPreflightAdvance,OriginalRetirementFailure>{
  let base=eff.performed_work();let mut work:u128=0;
  let result=(||->Result<RetainedDiffPreflightAdvance,OriginalRetirementFailure>{
   let g=eff.grant();
   if self.preflight_work_demand(d,s,post,body,max)?==0{return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}
   let demand=self.preflight_demand(d,s,post,body,max)?;if !protocol::retained_mutation_demand_fits(demand,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}
   let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
   match self.phase{
    0=>{work=1;self.refused=d.get().require_a.is_some_and(|expected|expected!=s.get().a);self.phase=if self.refused{3}else{1};},
    1=>if d.get().set_a.is_some()&&self.a.value().is_none(){let step=native_child(eff,&mut work,|e,r|self.a.advance_into(source_a(d),body,max,&mut e.nested(child(g)),r))??;p=receipt(step.progress());}else{work=1;self.phase=2;},
    2=>if d.get().set_b.is_some()&&self.b.value().is_none(){let step=native_child(eff,&mut work,|e,r|self.b.advance_into(source_b(d),body,max,&mut e.nested(child(g)),r))??;p=receipt(step.progress());}else{work=1;self.phase=3;},
    3=>{work=1;if self.refused||d.get().warning{p=receipt(self.diagnostic.start(RetainedDiagnosticSpec{level:if self.refused{semio_framework_diagnostic::Severity::Fatal}else{semio_framework_diagnostic::Severity::Warning},code:"sequence.original",message:"Original ordered sparse row."},child(g))?);self.phase=4;}else{self.phase=6;return Ok(RetainedDiffPreflightAdvance::Ready(p,true));}},
    4=>{let step=native_child(eff,&mut work,|e,r|self.diagnostic.advance_into([],body,max,&mut e.nested(child(g)),r))??;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5;}},
    5=>{let turn=self.diagnostic.take(child(g));work+=turn.work;let(rows,progress)=turn.result?.ok_or_else(invalid)?;*self.messages=Some(rows);self.phase=6;return Ok(RetainedDiffPreflightAdvance::Ready(receipt(progress),!self.refused));},
    _=>return Err(invalid().into()),
   }Ok(RetainedDiffPreflightAdvance::Progress(p))
  })();
  settle_value(eff,base,result,work,RetainedDiffPreflightAdvance::progress)
 }
 fn close_poll(&mut self,body:usize,eff:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let base=eff.performed_work();let mut work:u128=0;
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   if !self.closing{return Err(invalid().into());}
   let g=eff.grant();let demand=self.close_demand(body)?;if !protocol::retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()));}
   for field in [&mut self.a,&mut self.b]{if !field.terminal_is_empty(){if !field.close_started(){let turn=field.begin_close_paid(eff);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?));}let step=native_child(eff,&mut work,|e,r|field.close_into(body,e,r))??;return Ok(RetainedCloneStep::Progress(step.progress()));}}
   if !self.diagnostic.terminal_is_empty(){let step=native_child(eff,&mut work,|e,r|self.diagnostic.close_into(body,e,r))??;return Ok(RetainedCloneStep::Progress(step.progress()));}
   if !self.retirement.is_empty(){let step=native_child(eff,&mut work,|e,r|self.retirement.step_into(body,e,r))??;return Ok(RetainedCloneStep::Progress(step.progress()));}
   let turn=self.retirement.begin_granted(&mut self.messages,eff);work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(step.progress()));}
   Ok(RetainedCloneStep::Complete(Default::default()))
  })();
  settle_exit(eff,base,result,work)
 }
}
impl RetainedDiffApply<Snapshot,Edit> for EditApply{
 fn new()->Self{Self{a:Default::default(),b:Default::default(),diagnostic:Default::default(),messages:ManuallyDrop::new(None),retirement:Default::default(),phase:0,part:0,refused:false,closing:false}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,Edit>,_:RetainedCloneRef<'_,Snapshot>,_:RetainedCloneRef<'_,Snapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase==6{return Err(invalid());}Ok(match self.phase{
   0=>scalar(),
   1 if d.get().set_a.is_some()&&self.a.value().is_none()=>nested(self.a.demand(source_a(d),body,max)?),
   2 if d.get().set_b.is_some()&&self.b.value().is_none()=>nested(self.b.demand(source_b(d),body,max)?),
   3 if self.refused||d.get().warning=>nested(RetainedDiagnosticPreparation::<0>::start_demand()),
   4=>nested(self.diagnostic.demand([],body,max)?),
   5=>nested(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()}),
   _=>scalar(),
  })
 }
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,Edit>,s:RetainedCloneRef<'_,Snapshot>,post:RetainedCloneRef<'_,Snapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  self.preflight_demand(d,s,post,body,max)?;
  let work=match self.phase{
   1 if d.get().set_a.is_some()&&self.a.value().is_none()=>self.a.advance_work_demand(source_a(d),body,max)?,
   2 if d.get().set_b.is_some()&&self.b.value().is_none()=>self.b.advance_work_demand(source_b(d),body,max)?,
   4=>self.diagnostic.advance_work_demand([],body,max)?,
   5=>self.diagnostic.take_work_demand()?,
   _=>1,
  };Ok(if work<=max{work}else{0})
 }
 fn preflight_reply<'r>(&mut self,d:RetainedCloneRef<'_,Edit>,s:RetainedCloneRef<'_,Snapshot>,post:RetainedCloneRef<'_,Snapshot>,body:usize,max:u64,eff:&mut RetainedAllocationEffect<'_>,_:&mut NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{
  if recover{return Err(invalid());}
  Ok(reply.complete(self.preflight_step(d,s,post,body,max,eff).result))
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{self.messages.take().unwrap_or_default()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,Edit>,_:RetainedCloneRef<'_,Snapshot>,_:&Snapshot,_:usize)->Result<RetirementDemand,ValueError>{if self.phase!=6||self.refused{return Err(invalid());}Ok(match self.part{0 if d.get().set_a.is_some()=>nested(self.a.exchange_demand()?),1 if d.get().set_b.is_some()=>nested(self.b.exchange_demand()?),_=>scalar()})}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,Edit>,s:RetainedCloneRef<'_,Snapshot>,post:&Snapshot,body:usize)->Result<u64,ValueError>{self.commit_demand(d,s,post,body)?;Ok(1)}
 fn commit(&mut self,d:RetainedCloneRef<'_,Edit>,s:RetainedCloneRef<'_,Snapshot>,post:&mut Snapshot,eff:&mut RetainedAllocationEffect<'_>,_:ApplyCapability)->OriginalRetirementOutcome{
  let base=eff.performed_work();let mut work:u128=0;
  let result=(||->Result<RetainedCloneStep,OriginalRetirementFailure>{
   let g=eff.grant();let demand=self.commit_demand(d,s,post,g.maximum_copy_bytes)?;if !protocol::retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()));}
   let p=match self.part{
    0 if d.get().set_a.is_some()=>{let mut value=None;let step=native_child(eff,&mut work,|e,r|self.a.take_into(&mut value,&mut e.nested(child(g)),r))??;post.a=value.take().ok_or_else(invalid)?;receipt(step.progress())},
    1 if d.get().set_b.is_some()=>{let mut value=None;let step=native_child(eff,&mut work,|e,r|self.b.take_into(&mut value,&mut e.nested(child(g)),r))??;post.b=value.take().ok_or_else(invalid)?;receipt(step.progress())},
    _=>{work=1;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}},
   };
   self.part+=1;Ok(if self.part==2{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();
  settle(eff,base,result,work)
 }
 fn begin_close(&mut self){self.closing=true;self.diagnostic.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  for field in [&self.a,&self.b]{if !field.terminal_is_empty(){return if field.close_started(){field.close_demand(body)}else{Ok(field.begin_close_demand())};}}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body);}
  self.retirement.next_owner_demands::<Vec<MutationMessage>>(self.messages.is_some(),&None,body)
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demand(body)?;
  for field in [&self.a,&self.b]{if !field.terminal_is_empty(){return if field.close_started(){field.close_work_demand(body)}else{Ok(field.begin_close_work_demand())};}}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_work_demand(body);}
  self.retirement.next_owner_work::<Vec<MutationMessage>>(self.messages.is_some(),&None,body)
 }
 fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,eff:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if original.is_some()||recover{return Err(ValueError::invariant("original edit fixture close has no original Birth and never stalls"));}
  Ok(match self.close_poll(body,eff){RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result),RetainedWorkPoll::Pending{..}=>reply.pending()})
 }
 fn terminal_is_empty(&self)->bool{self.a.terminal_is_empty()&&self.b.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.messages.is_none()&&self.retirement.is_empty()}
}
impl Drop for EditApply{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original edit needs funded closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.messages);}}}}
#[derive(Clone,Default,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
struct Sequence{edits:Vec<Edit>}
impl DiffAlgebra<Snapshot> for Sequence{fn is_empty(&self)->bool{self.edits.iter().all(|edit|edit.set_a.is_none()&&edit.set_b.is_none()&&edit.require_a.is_none()&&!edit.warning)}}
impl MutationDiff<Snapshot> for Sequence{}
struct SequenceApply(RetainedDiffSequence<Snapshot,Edit>);
impl RetainedDiff<Snapshot> for Sequence{type Apply=SequenceApply;}
impl RetainedDiffApply<Snapshot,Sequence> for SequenceApply{
 fn new()->Self{Self(RetainedDiffSequence::new())}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:RetainedCloneRef<'_,Snapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{self.0.preflight_demand(d.project(0,|d|&d.edits),s,post,body,max)}
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:RetainedCloneRef<'_,Snapshot>,body:usize,max:u64)->Result<u64,ValueError>{self.0.preflight_work_demand(d.project(0,|d|&d.edits),s,post,body,max)}
 fn preflight_reply<'r>(&mut self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:RetainedCloneRef<'_,Snapshot>,body:usize,max:u64,eff:&mut RetainedAllocationEffect<'_>,receiving:&mut NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{
  let edits=d.project(0,|d|&d.edits);
  if recover{self.0.recover_preflight_into(edits,s,post,body,max,eff,receiving,reply)}else{self.0.preflight_into(edits,s,post,body,max,eff,receiving,reply)}
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{self.0.take_messages()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:&Snapshot,body:usize)->Result<RetirementDemand,ValueError>{self.0.commit_demand(d.project(0,|d|&d.edits),s,post,body)}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:&Snapshot,body:usize)->Result<u64,ValueError>{self.0.commit_work_demand(d.project(0,|d|&d.edits),s,post,body)}
 fn commit(&mut self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:&mut Snapshot,eff:&mut RetainedAllocationEffect<'_>,c:ApplyCapability)->OriginalRetirementOutcome{self.0.commit(d.project(0,|d|&d.edits),s,post,eff,c)}
 fn begin_close(&mut self){self.0.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.0.close_work_demand(body)}
 fn requires_original_birth(&self)->bool{self.0.requires_original_birth()}
 fn requires_original_control(&self)->bool{self.0.requires_original_control()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.0.close_original_birth_step(original,grant)}
 fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,eff:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  match(original,recover){(None,false)=>self.0.close_into(body,eff,reply),(None,true)=>self.0.recover_close_into(body,eff,reply),(Some(original),false)=>self.0.close_original_into(original,body,eff,reply),(Some(original),true)=>self.0.recover_close_original_into(original,body,eff,reply)}
 }
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn denied(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut grants=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes>0{grants.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g});}if d.capacity_bytes>0{grants.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g});}if d.release_bytes>0{grants.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g});}if d.depth>0{grants.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g});}grants}
fn exact<T>(run:impl FnOnce()->T,p:impl FnOnce(&T)->RetainedCloneProgress)->T{let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);let p=p(&value);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(!heap.overflowed);value}
fn source<T:RetireOwned+Sync+'static>(value:T)->FixtureSource<T>{FixtureSource::from_owner(value)}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();while !owner.terminal_is_empty(){let d=owner.next_demand(BODY).unwrap();let units=owner.next_computational_work(BODY).unwrap();let step=drive(grant(d),units,|effect|owner.step(BODY,effect)).result.unwrap_or_else(|_|panic!("original retirement turn refused"));assert!(step.progress().fits(grant(d)));}}
fn decode<T:FromValue>(value:&serde_json::Value)->T{semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn preflight(owner:&mut RetainedDiffOwner<Snapshot,Sequence>,diff:RetainedCloneRef<'_,Sequence>,original:RetainedCloneRef<'_,Snapshot>,post:RetainedCloneRef<'_,Snapshot>,body:usize,g:RetainedCloneGrant,receiving:&mut NativeDecodeControl<'_>)->RetainedDiffPreflightStep{
 drive_reply::<RetainedDiffPreflightStep,OriginalRetirementFailure>(g,UNITS,|answer|match answer{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply:Reply<'_,RetainedDiffPreflightStep>,recover|->Result<Done<'_,RetainedDiffPreflightStep>,ValueError>{if recover{protocol::recover_preflight_retained_diff_into(owner,diff,original,post,body,UNITS,effect,&mut *receiving,reply)}else{protocol::preflight_retained_diff_into(owner,diff,original,post,body,UNITS,effect,&mut *receiving,reply)}}).result.unwrap_or_else(|_|panic!("original central preflight refused"))
}
fn commit(owner:&mut RetainedDiffOwner<Snapshot,Sequence>,diff:RetainedCloneRef<'_,Sequence>,original:RetainedCloneRef<'_,Snapshot>,post:&mut PostFixture<Snapshot>,g:RetainedCloneGrant)->RetainedCloneStep{
 drive(g,UNITS,|effect|RetainedWorkPoll::Complete(protocol::commit_retained_diff(owner,diff,original,&mut *post,effect))).result.unwrap_or_else(|_|panic!("original central commit refused"))
}
fn close_once(owner:&mut RetainedDiffOwner<Snapshot,Sequence>,body:usize,g:RetainedCloneGrant)->RetainedCloneStep{
 drive_reply::<RetainedCloneStep,OriginalRetirementFailure>(g,UNITS,|answer|match answer{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply:Reply<'_>,recover|->Result<Done<'_>,ValueError>{if recover{owner.recover_close_into(body,effect,reply)}else{owner.close_into(body,effect,reply)}}).result.unwrap_or_else(|_|panic!("original central close refused"))
}
fn run(row:&serde_json::Value,base:&serde_json::Value,body:usize,cut:Option<usize>,commit_cut:Option<usize>){let mut receiving_callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut receiving=NativeDecodeControl::new(1<<24,&mut receiving_callback);
 let original=source(decode::<Snapshot>(base));let mut post=PostFixture::new(decode::<Snapshot>(base));semio_framework_value_snapshot_projection::snapshot_projection::testkit::take_field_advances();let pointer=post.text.as_ptr();let diff=source(Sequence{edits:decode(&row["edits"])});let(mut owner,_)=exact(||protocol::begin_retained_diff::<Snapshot,Sequence>(grant(protocol::retained_diff_birth_demand::<Snapshot,Sequence>())).unwrap(),|value|value.1);let mut turns=0;let mut settled=None;
 while cut.is_none_or(|cut|turns<cut){
  let d=owner.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),body,UNITS).unwrap();for denied in denied(grant(d),d){let step=preflight(&mut owner,diff.borrow(),original.borrow(),post.borrow(),body,denied,&mut receiving);assert_eq!(step.progress(),Default::default());assert_eq!(owner.preflight_demand(diff.borrow(),original.borrow(),post.borrow(),body,UNITS).unwrap(),d);}
  let step=preflight(&mut owner,diff.borrow(),original.borrow(),post.borrow(),body,grant(d),&mut receiving);assert_eq!(&post,original.borrow().get());assert_eq!(post.text.as_ptr(),pointer);turns+=1;
  match step{RetainedDiffPreflightStep::Admitted(_,rows)=>{assert_eq!(rows.len(),row["messageCount"].as_u64().unwrap()as usize);drain(rows);settled=Some(true);break;},RetainedDiffPreflightStep::Refused(_,rows)=>{assert_eq!(rows.len(),row["messageCount"].as_u64().unwrap()as usize);drain(rows);settled=Some(false);break;},_=>{}}
 }
 if cut.is_none(){assert_eq!(settled,Some(row["accepted"].as_bool().unwrap()));if settled==Some(true){let mut commit_turns=0;let mut complete=false;while commit_cut.is_none_or(|cut|commit_turns<cut){let d=owner.commit_demand(diff.borrow(),original.borrow(),post.mutable_read(),body).unwrap();for denied in denied(grant(d),d){let step=commit(&mut owner,diff.borrow(),original.borrow(),&mut post,denied);assert_eq!(step.progress(),Default::default());}let step=commit(&mut owner,diff.borrow(),original.borrow(),&mut post,grant(d));commit_turns+=1;if matches!(step,RetainedCloneStep::Complete(_)){complete=true;break;}}if complete{assert_eq!(post,decode::<Snapshot>(&row["expected"]));}if commit_cut.is_none(){assert!(complete);}}else{assert_eq!(&post,original.borrow().get());}let advances=semio_framework_value_snapshot_projection::snapshot_projection::testkit::take_field_advances();if diff.borrow().get().edits.len()<=1{assert_eq!(advances,0);assert_eq!(post.text.as_ptr(),pointer);}}
 owner.begin_close();while !owner.terminal_is_empty(){let d=owner.close_demand(body).unwrap();for denied in denied(grant(d),d){let step=close_once(&mut owner,body,denied);assert_eq!(step.progress(),Default::default());}close_once(&mut owner,body,grant(d));}
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(diff);drop(original);drop(post);
}
#[test]
fn retained_diff_sequence_matches_original_intermediate_sqlite_values_and_refusal(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in law["cases"].as_array().unwrap(){for body in[1,4096]{run(row,&law["base"],body,None,None);}}
}
#[test]
fn retained_diff_sequence_cancellation_closes_original_sources_diagnostics_and_candidates(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in law["cases"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){run(row,&law["base"],1,Some(cut.as_u64().unwrap()as usize),None);if row["accepted"]==true{run(row,&law["base"],1,None,Some(cut.as_u64().unwrap()as usize));}}}
}
#[test]
fn retained_diff_sequence_wide_original_projection_and_every_custody_cut_are_genuine(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let text=law["wideText"]["text"].as_str().unwrap().repeat(law["wideText"]["repeat"].as_u64().unwrap()as usize);let mut base=law["base"].clone();base["text"]=text.clone().into();
 for index in[1,2,3]{let mut row=law["cases"][index].clone();if row["accepted"]==true{row["expected"]["text"]=text.clone().into();}for body in[1,4096]{run(&row,&base,body,None,None);}for cut in[0,1,7,19,71,1025]{run(&row,&base,1,Some(cut),None);}if row["accepted"]==true{for cut in[0,1,7]{run(&row,&base,1,None,Some(cut));}}}
}
