//! 🧵️ Probe JSON preserves original first-party cursors and all refused partial owners.

use semio_framework_pack_json::JsonBorrowedWriteCursor;
use semio_framework_pack_json::JsonError;
use semio_framework_pack_json::JsonGrammarCursor;
use semio_framework_pack_json::JsonMemberPolicy;
use semio_framework_value::DslValue;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::NativeEncodeControl;
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::OriginalRetirementFailure;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_value::RetainedWorkPoll;
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use store::NativeSnapshotBodyWallet;
use store::NativeSnapshotDecodeOwner;
use store::NativeSnapshotEncodeOwner;
use crate::probe_document::ProbeSnapshot;

const PLAIN_GRANT:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16_777_216,maximum_copy_bytes:134_217_728,maximum_capacity_bytes:134_217_728,maximum_release_bytes:134_217_728,maximum_depth:256};

const CLOSE_GRANT:RetainedCloneGrant=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:134_217_728,maximum_release_bytes:134_217_728,maximum_depth:256};

const STEP_WORK:u64=4;

const FRAME_WORK:u64=16;

fn fuel(steps:usize)->Result<u64,ValueError>{u64::try_from(steps).ok().and_then(|steps|steps.checked_add(1)?.checked_mul(STEP_WORK)?.checked_add(FRAME_WORK)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON work quote overflow"))}

/// 🔁️ One close action costs one begin turn and at most one recovery turn of its pending reply.
const ACTION_TURNS:usize=2;

/// 🧮️ Every progressing close turn charges at least one unit of the input-size quote, so the quote bounds the turns exactly.
fn close_turns(quote:u64)->usize{usize::try_from(quote).unwrap_or(usize::MAX)}

pub(crate) fn validate(value:&DslValue)->Result<(),ValueError>{validate_with(value,&mut |_|Ok(()))}

pub(crate) fn validate_with(value:&DslValue,checkpoint:&mut impl FnMut(usize)->Result<(),ValueError>)->Result<(),ValueError>{
 fn node(value:&DslValue,depth:usize,checkpoint:&mut impl FnMut(usize)->Result<(),ValueError>)->Result<(),ValueError>{
  checkpoint(depth+1)?;
  if depth>semio_framework_pack_json::MAX_DEPTH as usize{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"Probe JSON exceeds declared grammar depth"))}
  match value{
   DslValue::Bytes(_)=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe JSON has no byte-array node")),
   DslValue::Number(semio_framework_value::Number::Float(value)) if !value.is_finite()=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe JSON requires finite numbers")),
   DslValue::Array(values)=>{for value in values{node(value,depth+1,checkpoint)?}Ok(())},
   DslValue::Object(entries)=>{for(index,(name,value))in entries.iter().enumerate(){for(previous,_)in &entries[..index]{checkpoint(depth+1)?;if previous.len()!=name.len(){continue}let mut equal=true;for(left,right)in previous.bytes().zip(name.bytes()){checkpoint(depth+1)?;if left!=right{equal=false;break}}if equal{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Probe JSON object contains duplicate members"))}}node(value,depth+1,checkpoint)?}Ok(())},
   _=>Ok(())
  }
 }
 node(value,0,checkpoint)
}

fn admit_inline<T>(body:&mut NativeSnapshotBodyWallet)->Result<(),ValueError>{body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<T>(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1})}

fn record_inline<T>(body:&mut NativeSnapshotBodyWallet)->Result<(),ValueError>{body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<T>(),..Default::default()})}

fn admit_turn(body:&NativeSnapshotBodyWallet,demand:semio_framework_value::RetirementDemand)->Result<(),ValueError>{body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth})}

/// 🫴️ Moves the direct grammar result while retaining cursor and original diagnostic custody.
fn read_original<T:semio_framework_value::retirement::RetireOwned>(text:&str,output:&mut Option<T>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,construct:fn(DslValue)->T)->semio_framework_value::RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
 type Receiving<T>=(JsonGrammarCursor<DslValue>,Option<DslValue>,Option<JsonError>,Option<T>);
 owner.receive::<Receiving<T>,T,store::NativeSnapshotFailure>(output,&mut |slot,received,_marker,_unwind,native,body|store::native_snapshot_turn(|work|{work.performed();(||->Result<(),ValueError>{
  admit_inline::<Receiving<T>>(body)?;
  *slot=Some((JsonGrammarCursor::new(JsonMemberPolicy::Reject),None,None,None));
  record_inline::<Receiving<T>>(body)?;
  loop{
   let(cursor,output,refusal,typed)=slot.as_mut().unwrap();
   let demand=match cursor.normal_step_demands(text){Ok(demand)=>demand,Err(JsonError::Native(error))=>return Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);return Err(ValueError::literal(kind,"Probe JSON source was refused"))}};
   admit_turn(body,demand)?;
   let Some(result)=native.with_allocated_effect(demand,STEP_WORK,|native,effect|cursor.step(text,1,native,effect))? else{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON step work quote was not admitted"))};
   let performed=cursor.normal_step_progress();
   let outcome=match result{Ok(Some(value))=>{*output=Some(value);Ok(true)},Ok(None)=>Ok(false),Err(JsonError::Native(error))=>Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);Err(ValueError::literal(kind,"Probe JSON grammar refused original source"))}};
   body.record_progress(performed)?;
   native.admit_turn_capacity(body.remaining_grant().maximum_capacity_bytes)?;
   if outcome?{
    validate_with(output.as_ref().unwrap(),&mut |depth|{body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_depth:depth,..Default::default()})?;native.checkpoint()?;body.record_progress(RetainedCloneProgress{copied_items:1,..Default::default()})})?;
    admit_inline::<T>(body)?;
    native.checkpoint()?;
    *typed=Some(construct(output.take().unwrap()));
    record_inline::<T>(body)?;
    *received=typed.take();
    return Ok(())
   }
   if performed==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON parser has no funded frontier"))}
  }
 })().map_err(Into::into)}))
}

/// 🌱️ Publishes the original value through the same retained receiving lifecycle.
pub(crate) fn read(text:&str,output:&mut Option<DslValue>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<(),store::NativeSnapshotFailure>{read_original(text,output,owner,std::convert::identity)}

/// 🧩️ Constructs only this domain's allocation-free snapshot wrapper inside original receiving custody.
pub(crate) fn receive_snapshot(text:&str,output:&mut Option<ProbeSnapshot>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<(),store::NativeSnapshotFailure>{read_original(text,output,owner,ProbeSnapshot)}

/// 🪶️ Borrows the unchanged tree and publishes the original canonical String backing.
pub(crate) fn write(value:&DslValue,maximum_bytes:usize,produced:&mut Option<String>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
 type Receiving=(JsonBorrowedWriteCursor,Option<String>);
 owner.receive::<Receiving,String,store::NativeSnapshotFailure>(produced,&mut |slot,received,_marker,_unwind,native,body|store::native_snapshot_turn(|work|{work.performed();(||->Result<(),store::NativeSnapshotFailure>{
  validate_with(value,&mut |depth|{body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_depth:depth,..Default::default()})?;native.checkpoint()?;body.record_progress(RetainedCloneProgress{copied_items:1,..Default::default()})})?;
  admit_inline::<Receiving>(body)?;
  *slot=Some((JsonBorrowedWriteCursor::new(),None));
  record_inline::<Receiving>(body)?;
  loop{
   let(cursor,output)=slot.as_mut().unwrap();
   if cursor.progress().0>maximum_bytes{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON output exceeds original file byte ceiling"))?}
   let demand=cursor.normal_step_demands(value)?;
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   admit_turn(body,demand)?;
   let Some(result)=native.with_effect(grant,STEP_WORK,|native,_|cursor.step(value,1,native,grant))? else{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON writer step work quote was not admitted").into())};
   let performed=cursor.normal_step_progress();
   let outcome=match result{Ok(Some(text))=>{*output=Some(text);Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(error)};
   body.record_progress(performed)?;
   native.admit_turn_capacity(body.remaining_grant().maximum_capacity_bytes)?;
   if outcome?{if output.as_ref().unwrap().len()>maximum_bytes{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON output exceeds original file byte ceiling"))?}*received=output.take();return Ok(())}
   if performed==RetainedCloneProgress::default(){Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON writer has no funded frontier"))?}
  }
 })().map_err(Into::into)}))
}

/// 🧯️ Collapses the original snapshot failure into the plain value refusal sync callers return.
fn refused(failure:store::NativeSnapshotFailure)->ValueError{ValueError::new(failure.kind(),failure.message().to_string()).with_retained_progress(failure.retained_progress())}

/// 🚰️ Closes the decoder's retained owners through owner-less retirement turns, each funded by its exact work quote.
fn drain_decode(native:&mut NativeDecodeControl<'_>,input:u64)->Result<(),ValueError>{
 let mut stalled=0;
 for _ in 0..close_turns(input){
  if !native.has_retirement_owner(){return Ok(())}
  let quote=native.with_retirement(CLOSE_GRANT,|loan|loan.retirement_work_demand(CLOSE_GRANT.maximum_copy_bytes))?;
  let ran=OriginalRetirementPump::new().turn(quote,u64::MAX,||Some(0),CLOSE_GRANT).and_then(|mut turn|turn.with_effect(|effect|native.close_retirement_recipient(CLOSE_GRANT.maximum_copy_bytes,effect)))?;
  let progressed=match ran{
   Some((RetainedWorkPoll::Complete(turn),_))=>{let step=turn.result.map_err(OriginalRetirementFailure::into_value_error)?;if !step.progress().fits(CLOSE_GRANT){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe JSON retirement exceeded original caller turn"))}step.progress()!=RetainedCloneProgress::default()||turn.work>0},
   Some((RetainedWorkPoll::Pending{work},_))=>work>0,
   None=>false
  };
  stalled=if progressed{0}else{stalled+1};
  if stalled>=ACTION_TURNS{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON original decoder retirement stalled"))}
 }
 Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON original decoder retirement did not terminate"))
}

/// 🚰️ Closes the encoder's retained owners through owner-less retirement turns, each funded by its exact work quote.
fn drain_encode(native:&mut NativeEncodeControl<'_>,input:u64)->Result<(),ValueError>{
 let mut answer=None;let mut pending=false;let mut stalled=0;
 for _ in 0..close_turns(input){
  if !native.has_retirement_owner(){return Ok(())}
  let quote=native.retirement_work_demand(CLOSE_GRANT.maximum_copy_bytes)?;
  let ran=OriginalRetirementPump::new().turn(quote,u64::MAX,||Some(0),CLOSE_GRANT).and_then(|mut turn|turn.with_effect(|effect|{
   if answer.is_none(){let reply=OriginalEffectReply::borrow(&mut answer)?;(if pending{native.recover_close_retirement_recipient_into(CLOSE_GRANT.maximum_copy_bytes,effect,reply)}else{native.close_retirement_recipient_into(CLOSE_GRANT.maximum_copy_bytes,effect,reply)})?;}
   Ok::<(),ValueError>(())
  }))?;
  let progressed=match ran{
   Some((closed,performed))=>{closed?;match answer.take(){Some(Ok(step))=>{pending=false;if !step.progress().fits(CLOSE_GRANT){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe JSON retirement exceeded original caller turn"))}step.progress()!=RetainedCloneProgress::default()||performed>0},Some(Err(failure))=>return Err(failure.into_value_error()),None=>{pending=true;performed>0}}},
   None=>false
  };
  stalled=if progressed{0}else{stalled+1};
  if stalled>=ACTION_TURNS{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON original encoder retirement stalled"))}
 }
 Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON original encoder retirement did not terminate"))
}

/// 🌱️ Supplies a bounded synchronous caller: the read runs in one owner-less retirement turn, its retained owners close in further quoted turns.
pub(crate) fn parse(text:&str)->Result<DslValue,ValueError>{
 if text.len()>33_554_432{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON input exceeds synchronous caller policy"))}
 let mut accepted=|_|true;
 let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
 let mut native=NativeDecodeControl::new_retained(&mut accepted);
 native.admit_turn_capacity(PLAIN_GRANT.maximum_capacity_bytes)?;
 native.install_retirement_recipient(&mut recipient)?;
 let mut value=None;
 let input=fuel(text.len())?;
 let result=native.scoped_maximum(268_435_456,|native|OriginalRetirementPump::new().turn(input,u64::MAX,||Some(0),PLAIN_GRANT)?.with_native_decoder_work(native,|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,PLAIN_GRANT);read(text,&mut value,&mut owner).result.map_err(refused)}));
 let closed=drain_decode(&mut native,input);
 match(result,closed){
  (Ok(()),Ok(()))=>value.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe JSON read produced no value")),
  (Err(error),Ok(()))=>Err(error),
  (result,Err(error))=>{std::mem::forget(native);std::mem::forget(recipient);result.and(Err(error))}
 }
}

/// 🔤️ Supplies a bounded synchronous writer without an intermediate projected tree.
pub(crate) fn print(value:&DslValue)->Result<String,ValueError>{
 let mut accepted=|_|true;
 let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
 let mut native=NativeEncodeControl::new_retained(&mut accepted);
 native.admit_turn_capacity(PLAIN_GRANT.maximum_capacity_bytes)?;
 native.install_retirement_recipient(&mut recipient)?;
 let mut text=None;
 let input=fuel(33_554_432)?;
 let result=native.scoped_maximum(268_435_456,|native|OriginalRetirementPump::new().turn(input,u64::MAX,||Some(0),PLAIN_GRANT)?.with_native_encoder_work(native,|native|{let mut owner=NativeSnapshotEncodeOwner::new(native,PLAIN_GRANT);write(value,33_554_432,&mut text,&mut owner).result.map_err(refused)}));
 let closed=drain_encode(&mut native,input);
 match(result,closed){
  (Ok(()),Ok(()))=>text.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe JSON write produced no text")),
  (Err(error),Ok(()))=>Err(error),
  (result,Err(error))=>{std::mem::forget(native);std::mem::forget(recipient);result.and(Err(error))}
 }
}
