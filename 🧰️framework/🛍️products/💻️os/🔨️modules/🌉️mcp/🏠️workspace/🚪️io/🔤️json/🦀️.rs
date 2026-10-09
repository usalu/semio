//! 🧵️ Probe JSON preserves original first-party cursors and all refused partial owners.
use semio_framework_pack_json::{JsonBorrowedWriteCursor,JsonError,JsonGrammarCursor,JsonMemberPolicy};
use semio_framework_value::{DslValue,NativeDecodeControl,NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use store::{NativeSnapshotBodyWallet,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner};

const PLAIN_GRANT:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16_777_216,maximum_copy_bytes:134_217_728,maximum_capacity_bytes:134_217_728,maximum_release_bytes:134_217_728,maximum_depth:256};
const CLOSE_GRANT:RetainedCloneGrant=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:134_217_728,maximum_release_bytes:134_217_728,maximum_depth:256};

pub(super) fn validate(value:&DslValue)->Result<(),ValueError>{validate_with(value,&mut |_|Ok(()))}
pub(super) fn validate_with(value:&DslValue,checkpoint:&mut impl FnMut(usize)->Result<(),ValueError>)->Result<(),ValueError>{
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
pub(super) fn read(text:&str,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<DslValue,ValueError>{
 type Receiving=(JsonGrammarCursor<DslValue>,Option<DslValue>,Option<JsonError>);
 owner.receive::<Receiving,DslValue>(|slot,native,body|{
  admit_inline::<Receiving>(body)?;
  *slot=Some((JsonGrammarCursor::new(JsonMemberPolicy::Reject),None,None));
  record_inline::<Receiving>(body)?;
  loop{
   let(cursor,output,refusal)=slot.as_mut().unwrap();
   let demand=match cursor.normal_step_demands(text){Ok(demand)=>demand,Err(JsonError::Native(error))=>return Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);return Err(ValueError::literal(kind,"Probe JSON source was refused"))}};
   admit_turn(body,demand)?;
   let grant=body.remaining_grant();
   let result=cursor.step(text,grant.maximum_items.min(1024),native,grant);
   let performed=cursor.normal_step_progress();
   let outcome=match result{Ok(Some(value))=>{*output=Some(value);Ok(true)},Ok(None)=>Ok(false),Err(JsonError::Native(error))=>Err(error),Err(error)=>{let kind=error.kind();*refusal=Some(error);Err(ValueError::literal(kind,"Probe JSON grammar refused original source"))}};
   body.record_progress(performed)?;
   if outcome?{
    validate_with(output.as_ref().unwrap(),&mut |depth|{body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_depth:depth,..Default::default()})?;native.checkpoint()?;body.record_progress(RetainedCloneProgress{copied_items:1,..Default::default()})})?;
    return Ok(output.take().unwrap())
   }
   if performed==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON parser has no funded frontier"))}
  }
 })
}

/// 🪶️ Borrows the unchanged tree and publishes the original canonical String backing.
pub(super) fn write(value:&DslValue,maximum_bytes:usize,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->Result<String,ValueError>{
 type Receiving=(JsonBorrowedWriteCursor,Option<String>);
 owner.receive::<Receiving,String>(|slot,native,body|{
  validate_with(value,&mut |depth|{body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_depth:depth,..Default::default()})?;native.checkpoint()?;body.record_progress(RetainedCloneProgress{copied_items:1,..Default::default()})})?;
  admit_inline::<Receiving>(body)?;
  *slot=Some((JsonBorrowedWriteCursor::new(),None));
  record_inline::<Receiving>(body)?;
  loop{
   let(cursor,output)=slot.as_mut().unwrap();
   if cursor.progress().0>maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON output exceeds original file byte ceiling"))}
   admit_turn(body,cursor.normal_step_demands(value)?)?;
   let grant=body.remaining_grant();
   let result=cursor.step(value,1,native,grant);
   let performed=cursor.normal_step_progress();
   let outcome=match result{Ok(Some(text))=>{*output=Some(text);Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(error)};
   body.record_progress(performed)?;
   if outcome?{if output.as_ref().unwrap().len()>maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON output exceeds original file byte ceiling"))}return Ok(output.take().unwrap())}
   if performed==RetainedCloneProgress::default(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON writer has no funded frontier"))}
  }
 })
}

/// 🌱️ Supplies a bounded synchronous caller and its own separate original retirement turns.
pub(super) fn parse(text:&str)->Result<DslValue,ValueError>{
 if text.len()>33_554_432{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Probe JSON input exceeds synchronous caller policy"))}
 let mut accepted=|_|true;
 let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
 let mut native=NativeDecodeControl::new(268_435_456,&mut accepted);
 native.install_retirement_recipient(&mut recipient)?;
 let result={let mut owner=NativeSnapshotDecodeOwner::new(&mut native,PLAIN_GRANT);read(text,&mut owner)};
 for _ in 0..100_000{if !native.has_retirement_owner(){return result}let step=native.close_retirement_recipient(CLOSE_GRANT)?;if !step.progress().fits(CLOSE_GRANT){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe JSON retirement exceeded original caller turn"))}}
 Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON original decoder retirement did not terminate"))
}

/// 🔤️ Supplies a bounded synchronous writer without an intermediate projected tree.
pub(super) fn print(value:&DslValue)->Result<String,ValueError>{
 let mut accepted=|_|true;
 let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
 let mut native=NativeEncodeControl::new(268_435_456,&mut accepted);
 native.install_retirement_recipient(&mut recipient)?;
 let result={let mut owner=NativeSnapshotEncodeOwner::new(&mut native,PLAIN_GRANT);write(value,33_554_432,&mut owner)};
 for _ in 0..100_000{if !native.has_retirement_owner(){return result}let step=native.close_retirement_recipient(CLOSE_GRANT)?;if !step.progress().fits(CLOSE_GRANT){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Probe JSON retirement exceeded original caller turn"))}}
 Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Probe JSON original encoder retirement did not terminate"))
}
