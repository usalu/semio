//! 🔁️ Closed snapshot refusal transport preserves all independent original causes.
use super::{NativeSnapshotCause,NativeSnapshotFailure};
use crate::{NativeSnapshotDecodeOwner,NativeSnapshotBodyWallet,native_snapshot_turn,NativeSnapshotWork};
use semio_framework_value::{DslValue,NativeEncodeControl,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ToValue,FromValue};
use semio_framework_value_native_owned::{OriginalReceivingPrimary,native_decoding::NativeDecodeControl};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native snapshot failure differs from its closed transport schema")}
fn fields<'a,const N:usize>(value:&'a DslValue,names:[&str;N])->Result<[&'a DslValue;N],ValueError>{
 let DslValue::Object(entries)=value else{return Err(invalid())};if entries.len()!=N{return Err(invalid())}
 let mut result=[None;N];for(key,value)in entries{let Some(index)=names.iter().position(|name|*name==key)else{return Err(invalid())};if result[index].replace(value).is_some(){return Err(invalid())}}
 Ok(result.map(|value|value.expect("closed transport has every declared field")))
}
fn optional_error(value:Option<&ValueError>,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{match value{Some(value)=>value.to_value_controlled(control),None=>Ok(DslValue::Null)}}
fn object<const N:usize>(control:&mut NativeEncodeControl<'_>,produce:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<[(&'static str,DslValue);N],ValueError>)->Result<DslValue,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(N)?;let mut fields=DslValue::object_encoding_controlled(N,control)?;for(name,value)in produce(control)?{DslValue::push_encoding_controlled(fields.get_mut(),name,value,control)?;control.step()?;}Ok(DslValue::Object(fields.take()))})
}
impl NativeSnapshotFailure{
 /// 🛫️ Projects all causes through the supplied original encode control.
 pub fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
  object(control,|control|Ok([
   ("cause",match &self.cause{
    None=>DslValue::Null,
    Some(NativeSnapshotCause::Value(value))=>object(control,|control|Ok([("type",DslValue::String(control.copy_text("value")?)),("value",value.to_value_controlled(control)?)]))?,
    Some(NativeSnapshotCause::Receiving(value))=>object(control,|control|Ok([("type",DslValue::String(control.copy_text("receiving")?)),("primary",optional_error(value.primary.as_ref(),control)?),("quoteRefusal",optional_error(value.quote_refusal.as_ref(),control)?),("allocationRefusal",optional_error(value.allocation_refusal.as_ref(),control)?),("progress",value.progress.to_value_controlled(control)?)]))?
   }),
   ("receiptRefusal",optional_error(self.receipt_refusal.as_ref(),control)?),
   ("contextRefusal",optional_error(self.context_refusal.as_ref(),control)?),
   ("settlementRefusal",optional_error(self.settlement_refusal.as_ref(),control)?),
   ("progress",self.progress.to_value_controlled(control)?)
  ]))
 }
 /// 🛬️ Retains every decoded refusal and cancellable message prefix in one original receiving frame.
 pub fn receive_value(value:&DslValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,Self>{
  owner.receive::<FailureFrame,Self,Self>(|slot,native,body|native_snapshot_turn(|work|{
   let [cause,receipt,context,settlement,progress]=fields(value,["cause","receiptRefusal","contextRefusal","settlementRefusal","progress"])?;
   let bytes=size_of::<FailureFrame>();body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;native.checkpoint()?;work.admit(native)?;
   *slot=Some(FailureFrame{failure:Some(Self{cause:None,receipt_refusal:None,context_refusal:None,settlement_refusal:None,progress:Default::default()}),value:None,pending:String::new()});
   body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
   let frame=slot.as_mut().unwrap();
   if !matches!(cause,DslValue::Null){
    let DslValue::Object(entries)=cause else{return Err(invalid().into())};
    let kind=entries.iter().find(|(key,_)|key=="type").map(|(_,value)|value);
    match kind{
     Some(DslValue::String(kind))if kind=="value"=>{
      let[_,value]=fields(cause,["type","value"])?;if matches!(value,DslValue::Null){return Err(invalid().into())}
      receive_error(value,&mut frame.value,&mut frame.pending,native,body,work)?;
      frame.failure.as_mut().unwrap().cause=Some(NativeSnapshotCause::Value(frame.value.take().unwrap()));
     },
     Some(DslValue::String(kind))if kind=="receiving"=>{
      let[_,primary,quote,allocation,progress]=fields(cause,["type","primary","quoteRefusal","allocationRefusal","progress"])?;
      frame.failure.as_mut().unwrap().cause=Some(NativeSnapshotCause::Receiving(OriginalReceivingPrimary{primary:None,quote_refusal:None,allocation_refusal:None,progress:Default::default()}));
      let Some(NativeSnapshotCause::Receiving(cause))=&mut frame.failure.as_mut().unwrap().cause else{unreachable!()};
      receive_error(primary,&mut cause.primary,&mut frame.pending,native,body,work)?;
      receive_error(quote,&mut cause.quote_refusal,&mut frame.pending,native,body,work)?;
      receive_error(allocation,&mut cause.allocation_refusal,&mut frame.pending,native,body,work)?;
      cause.progress=RetainedCloneProgress::from_value_controlled(progress,native)?;
     },
     _=>return Err(invalid().into())
    }
   }
   let failure=frame.failure.as_mut().unwrap();
   receive_error(receipt,&mut failure.receipt_refusal,&mut frame.pending,native,body,work)?;
   receive_error(context,&mut failure.context_refusal,&mut frame.pending,native,body,work)?;
   receive_error(settlement,&mut failure.settlement_refusal,&mut frame.pending,native,body,work)?;
   failure.progress=RetainedCloneProgress::from_value_controlled(progress,native)?;
   let bytes=size_of::<Self>();body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;native.checkpoint()?;work.admit(native)?;
   body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
   Ok(frame.failure.take().unwrap())
  }))
 }
}
#[derive(semio_framework_value::RetireOwned)]
struct FailureFrame{failure:Option<NativeSnapshotFailure>,value:Option<ValueError>,pending:String}
fn receive_error(value:&DslValue,output:&mut Option<ValueError>,pending:&mut String,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 if matches!(value,DslValue::Null){return Ok(())}
 let[kind,message,progress]=fields(value,["kind","message","retainedProgress"])?;
 let DslValue::String(kind)=kind else{return Err(invalid().into())};let kind=ValueRefusalKind::from_wire(kind).ok_or_else(invalid)?;
 let DslValue::String(message)=message else{return Err(invalid().into())};let progress=RetainedCloneProgress::from_value_controlled(progress,native)?;
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:message.len(),maximum_capacity_bytes:message.len(),maximum_depth:2,..Default::default()})?;native.checkpoint()?;work.admit(native)?;
 body.copy_text_into(native,message,pending,2)?;
 let bytes=size_of::<ValueError>();body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:2,..Default::default()})?;native.checkpoint()?;work.admit(native)?;
 *output=Some(ValueError::new(kind,std::mem::take(pending)).with_retained_progress(progress));
 body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
 Ok(())
}
