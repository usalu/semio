//! 🛬️ Original IO refusal receiving retains every cause, diagnostic and copied prefix.
use super::{IoError,Diagnostic,NativeSnapshotFailure};
use semio_framework_io_control::{NativeSnapshotDecodeOwner,NativeSnapshotBodyWallet,NativeSnapshotDecodeEffectQuote,NativeSnapshotWork,native_snapshot_turn};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,FromValue};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_diagnostic::{FaultCode,FaultScope,ExpectedSet,Severity,TextSpan};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"IO refusal differs from its closed transport schema")}
fn fields<'a,const N:usize>(value:&'a DslValue,names:[&str;N])->Result<[Option<&'a DslValue>;N],ValueError>{
 let DslValue::Object(entries)=value else{return Err(invalid())};if entries.len()>N{return Err(invalid())}let mut out=[None;N];
 for(key,value)in entries{let Some(index)=names.iter().position(|name|*name==key)else{return Err(invalid())};if out[index].replace(value).is_some(){return Err(invalid())}}
 Ok(out)
}
fn required(value:Option<&DslValue>)->Result<&DslValue,ValueError>{value.ok_or_else(invalid)}
fn movement(bytes:usize,depth:usize,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork,effect:impl FnOnce())->Result<(),NativeSnapshotFailure>{
 let grant=body.admit_constructed(native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:depth,..Default::default()})?;work.admit(native)?;
 effect();body.record_constructed_progress(native,grant,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})
}
fn text(value:&DslValue,output:&mut String,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let DslValue::String(value)=value else{return Err(invalid().into())};
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:value.len(),maximum_capacity_bytes:value.len(),maximum_depth:3,..Default::default()})?;work.admit(native)?;
 body.copy_text_receiving(native,value,output,3).map_err(Into::into)
}
fn optional_text(value:Option<&DslValue>,output:&mut Option<String>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 if let Some(value)=value.filter(|value|!matches!(value,DslValue::Null)){if !matches!(value,DslValue::String(_)){return Err(invalid().into())}movement(size_of::<Option<String>>(),3,native,body,work,||*output=Some(String::new()))?;text(value,output.as_mut().unwrap(),native,body,work)?}Ok(())
}
fn vector<T>(count:usize,output:&mut Vec<T>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let bytes=count.checked_mul(size_of::<T>()).ok_or_else(invalid)?;
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_depth:3,..Default::default()})?;work.admit(native)?;
 body.allocate_vec_receiving(native,count,output,3).map_err(Into::into)
}
fn strings(value:&DslValue,output:&mut Vec<String>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let DslValue::Array(values)=value else{return Err(invalid().into())};vector(values.len(),output,native,body,work)?;
 for value in values{if !matches!(value,DslValue::String(_)){return Err(invalid().into())}movement(size_of::<String>(),3,native,body,work,||output.push(String::new()))?;text(value,output.last_mut().unwrap(),native,body,work)?}Ok(())
}
#[derive(semio_framework_value::RetireOwned)]
struct Frame{cause:Option<NativeSnapshotFailure>,diagnostics:Vec<Diagnostic>,pending:Option<Diagnostic>}
#[derive(semio_framework_value::RetireOwned)]
struct Envelope{frame:Option<Frame>,output:Option<IoError>,effect:semio_framework_io_control::NativeSnapshotEffectProgress,unwind:Option<NativeSnapshotFailure>}
impl IoError{
 /// 🧳️ Receives the complete refusal through the same caller and retains partial diagnostics until funded close.
 pub fn receive_value(value:&DslValue,output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  owner.receive_nested::<Envelope,Self,NativeSnapshotFailure>(output,&mut |slot,published,_marker,_failure,owner|native_snapshot_turn(|work|{
   work.admit(owner.native())?;let[cause,diagnostics]=fields(value,["cause","diagnostics"])?;
   let cause=required(cause)?;let DslValue::Array(diagnostics)=required(diagnostics)? else{return Err(invalid().into())};
   let bytes=size_of::<Envelope>();work.take(owner.effect(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()},_marker,_failure,&mut |performed,physical|{*slot=Some(Envelope{frame:Some(Frame{cause:None,diagnostics:Vec::new(),pending:None}),output:None,effect:Default::default(),unwind:None});*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;_marker.reset_completed()?;
   let envelope=slot.as_mut().unwrap();work.take(NativeSnapshotFailure::receive_value(cause,&mut envelope.frame.as_mut().unwrap().cause,owner))?;
   let Envelope{frame,output:intermediate,effect,unwind}=envelope;
   work.take(owner.receive::<Frame,Self,NativeSnapshotFailure>(intermediate,&mut |destination,result,_marker,_failure,native,body|native_snapshot_turn(|work|{
    if frame.is_none()||destination.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IO refusal lost its original diagnostics frame").into())}let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})?;
    work.take(quote.run(_marker,_failure,&mut |performed,physical|{*destination=frame.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())}))?;_marker.reset_completed()?;let frame=destination.as_mut().unwrap();
    vector(diagnostics.len(),&mut frame.diagnostics,native,body,work)?;
    for value in diagnostics{
     receive_diagnostic(value,&mut frame.pending,native,body,work)?;
     movement(size_of::<Diagnostic>(),2,native,body,work,||frame.diagnostics.push(frame.pending.take().unwrap()))?;
    }
    if frame.cause.is_none()||result.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IO refusal lost its retained cause").into())}let bytes=size_of::<Self>();let quote=NativeSnapshotDecodeEffectQuote::decode(body,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
    work.take(quote.run(_marker,_failure,&mut |performed,physical|{*result=Some(Self{cause:frame.cause.take().unwrap(),diagnostics:std::mem::take(&mut frame.diagnostics)});*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())}))?;Ok(())
   })))?;
   if intermediate.is_none()||published.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IO refusal lost its retained output slot").into())}
   work.take(owner.move_existing(intermediate,published,effect,unwind))?;Ok(())
  }))
 }
}
fn receive_diagnostic(value:&DslValue,output:&mut Option<Diagnostic>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 work.admit(native)?;let[code,severity,span,message,expected,scope]=fields(value,["code","severity","span","message","expected","scope"])?;
 let code=required(code)?;let message=required(message)?;
 work.admit(native)?;let severity=work.measured(native,|native|Severity::from_value_controlled(required(severity)?,native))?;work.admit(native)?;let span=work.measured(native,|native|TextSpan::from_value_controlled(required(span)?,native))?;
 movement(size_of::<Diagnostic>(),2,native,body,work,||*output=Some(Diagnostic{code:FaultCode(String::new()),severity,span,message:String::new(),expected:None,scope:FaultScope::default()}))?;
 let out=output.as_mut().unwrap();text(code,&mut out.code.0,native,body,work)?;text(message,&mut out.message,native,body,work)?;
 if let Some(expected)=expected.filter(|value|!matches!(value,DslValue::Null)){
  work.admit(native)?;let[tokens,keywords,keys]=fields(expected,["tokens","keywords","keys"])?;
  movement(size_of::<Option<ExpectedSet>>(),2,native,body,work,||out.expected=Some(ExpectedSet{tokens:Vec::new(),keywords:Vec::new(),keys:Vec::new()}))?;let target=out.expected.as_mut().unwrap();
  for (value,output) in [(tokens,&mut target.tokens),(keywords,&mut target.keywords),(keys,&mut target.keys)]{if let Some(value)=value.filter(|value|!matches!(value,DslValue::Null)){strings(value,output,native,body,work)?}} 
 }
 if let Some(scope)=scope.filter(|value|!matches!(value,DslValue::Null)){
  work.admit(native)?;let[plugin,app,instance,module,body_key]=fields(scope,["pluginId","appId","instanceId","module","bodyKey"])?;
  optional_text(plugin,&mut out.scope.plugin_id,native,body,work)?;optional_text(app,&mut out.scope.app_id,native,body,work)?;optional_text(instance,&mut out.scope.instance_id,native,body,work)?;optional_text(module,&mut out.scope.module,native,body,work)?;optional_text(body_key,&mut out.scope.body_key,native,body,work)?;
 }
 Ok(())
}
