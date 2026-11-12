//! 🛫️ TXT owns one admitted output backing through the original native receiving frame.
use super::{native_length,TxtSnapshot};
use semio_os_kernel::{NativeSnapshotEncodeOwner,NativeSnapshotBodyWallet,NativeSnapshotFailure,os_store::serialization::{SnapshotReceivingOutcome,SnapshotReceivingOutput},space_history::io::sqlite::snapshot::settlement::TypedFileSettlement,sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteStageFailure as Stage}};
use semio_framework_value::native_encoding::NativeEncodeProgress;
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use store::io_schema::IoPayload;
use std::mem::size_of;
const TOKEN:&[u8]=b"stdio.txt.pack v1";
fn refused(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::literal(kind,message)}
pub fn extent(body:usize,encoding:SnapshotEncoding)->Result<usize,ValueError>{body.checked_add(if encoding==SnapshotEncoding::Binary{12+TOKEN.len()}else{0}).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"TXT original output extent exceeds address space"))}
fn work(body:&mut NativeSnapshotBodyWallet,bytes:usize)->Result<(),ValueError>{body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:2})?;body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})}
fn append(output:&mut Vec<u8>,bytes:&[u8],native:&mut NativeEncodeControl<'_>,body:&mut NativeSnapshotBodyWallet)->Result<(),ValueError>{
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes.len(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:2})?;
 if bytes.len()>output.capacity().saturating_sub(output.len()){return Err(refused(ValueRefusalKind::InvariantViolated,"TXT original append exceeds admitted backing"))}
 output.extend_from_slice(bytes);body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes.len(),..Default::default()})?;native.advance(bytes.len())
}
/// 🫴️ Installs the typed Vec before allocation and records every canceled byte prefix.
pub fn receive_into(source:&TxtSnapshot,encoding:SnapshotEncoding,body_bytes:usize,slot:&mut Option<Vec<u8>>,native:&mut NativeEncodeControl<'_>,body:&mut NativeSnapshotBodyWallet)->Result<IoPayload,ValueError>{
 if slot.is_some(){return Err(refused(ValueRefusalKind::InvariantViolated,"TXT original receiving slot must be empty"))}
 let physical=extent(body_bytes,encoding)?;let separators=source.lines.len().saturating_sub(1).checked_add(usize::from(source.trailing_newline)).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"TXT original separator census overflow"))?;
 let mut items=3usize.checked_add(separators).and_then(|items|items.checked_add(if encoding==SnapshotEncoding::Binary{3}else{0})).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"TXT original copy census overflow"))?;
 let headers=2*size_of::<Option<Vec<u8>>>()+size_of::<Vec<u8>>()+size_of::<IoPayload>();let copy=physical.checked_add(headers).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"TXT original copy extent overflow"))?;
 body.admit_frontier(RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:physical,maximum_release_bytes:0,maximum_depth:2})?;
 if native.owned_bytes().checked_add(physical).is_none_or(|bytes|bytes>native.maximum_bytes()){return Err(refused(ValueRefusalKind::OwnershipLimit,"TXT original backing exceeds native allowance"))}
 for(ordinal,line)in source.lines.iter().enumerate(){if ordinal%256==0{native.checkpoint()?;}items=items.checked_add(line.len().div_ceil(65536)).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"TXT original copy census overflow"))?;if items>body.remaining_grant().maximum_items{return Err(refused(ValueRefusalKind::WorkLimit,"TXT original copy census exceeds original items"))}}
 body.admit_frontier(RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:physical,maximum_release_bytes:0,maximum_depth:2})?;
 let result=native.scoped_stage(|native|{
  native.begin_stage(physical)?;work(body,size_of::<Option<Vec<u8>>>())?;*slot=Some(Vec::new());body.allocate_encode_vec_into(native,physical,slot.as_mut().unwrap(),2)?;
  let output=slot.as_mut().unwrap();if encoding==SnapshotEncoding::Binary{append(output,&store::semio_format::BINARY_MAGIC,native,body)?;append(output,&(TOKEN.len()as u32).to_le_bytes(),native,body)?;append(output,TOKEN,native,body)?;}
  let separator=source.line_ending.as_str().as_bytes();for(ordinal,line)in source.lines.iter().enumerate(){if ordinal!=0{append(output,separator,native,body)?;}for bytes in line.as_bytes().chunks(65536){append(output,bytes,native,body)?;}}
  if source.trailing_newline{append(output,separator,native,body)?;}if output.len()!=physical{return Err(refused(ValueRefusalKind::InvariantViolated,"TXT original output differs from borrowed extent"))}
  native.checkpoint()?;work(body,size_of::<Option<Vec<u8>>>()+size_of::<IoPayload>())?;
  match encoding{SnapshotEncoding::Binary=>Ok(IoPayload::Binary(slot.take().unwrap())),SnapshotEncoding::Text=>match String::from_utf8(slot.take().unwrap()){Ok(text)=>Ok(IoPayload::Text(text)),Err(error)=>{*slot=Some(error.into_bytes());Err(refused(ValueRefusalKind::InvariantViolated,"TXT original borrowed lines lost UTF8"))}}}
 });result.map_err(|error|error.with_retained_progress(body.progress()))
}
pub(super) fn encode(source:&TxtSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<IoPayload>>)->SnapshotReceivingOutcome{
 let limits=control.limits();
 let admitted=(||->Result<(usize,usize,usize),ValueError>{
 if output.is_some(){return Err(refused(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))}
 owner.native().checkpoint()?;let minimum=source.lines.len().saturating_sub(1).checked_add(4).and_then(|items|items.checked_add(if encoding==SnapshotEncoding::Binary{3}else{0})).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"TXT original borrowed census overflow"))?;if minimum>owner.remaining_grant().maximum_items{return Err(refused(ValueRefusalKind::WorkLimit,"TXT original borrowed census exceeds original items"))}
 let body_bytes=native_length(source,control,Some(owner.native()))?;let physical=extent(body_bytes,encoding)?;if physical>limits.max_file_bytes{return Err(refused(ValueRefusalKind::OwnershipLimit,"native text output exceeds file byte limit"))}control.check_value_bytes(physical)?;
 let before=owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes().min(limits.max_value_bytes)).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"TXT original SQL allocation ceiling overflow"))?;
 Ok((body_bytes,before,maximum))
 })();
 let(body_bytes,before,maximum)=match admitted{Ok(parts)=>parts,Err(error)=>return RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
 let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::EncodeNative);
 let mut observer=|event:NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
 let turn=owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Vec<u8>,SnapshotReceivingOutput<IoPayload>,NativeSnapshotFailure>(output,&mut |slot,published,_marker,_unwind,native,body|{
  let before_work=native.actual_work();
  let operation=receive_into(source,encoding,body_bytes,slot,native,body).map_err(NativeSnapshotFailure::from);
  let settled=settlement.finish_body(native.owned_bytes());
  *published=Some(match settled{Ok(())=>operation.map_err(Stage::Operation),Err(refusal)=>Err(Stage::Settlement{operation,refusal,receipt:None})});
  RetainedWorkOutcome{result:Ok(()),work:native.actual_work().checked_sub(before_work).expect("original native work is monotone")}
 }));
 let result=match settlement.cause.borrow_mut().take(){Some(refusal)=>Err(Stage::Settlement{operation:turn.result,refusal,receipt:None}),None=>turn.result.map_err(Stage::Operation)};
 RetainedWorkOutcome{result,work:turn.work}
}
