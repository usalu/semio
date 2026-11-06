//! 🛬️ Original IO refusal receiving retains every cause, diagnostic and copied prefix.
use super::{IoError,Diagnostic,NativeSnapshotFailure};
use semio_framework_io_control::{NativeSnapshotDecodeOwner,NativeSnapshotBodyWallet,NativeSnapshotWork,native_snapshot_turn};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,FromValue};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_diagnostic::{FaultCode,FaultScope,ExpectedSet,Severity,TextSpan};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"IO refusal differs from its closed transport schema")}
fn fields<'a,const N:usize>(value:&'a DslValue,names:[&str;N])->Result<[Option<&'a DslValue>;N],ValueError>{
 let DslValue::Object(entries)=value else{return Err(invalid())};let mut out=[None;N];
 for(key,value)in entries{let Some(index)=names.iter().position(|name|*name==key)else{return Err(invalid())};if out[index].replace(value).is_some(){return Err(invalid())}}
 Ok(out)
}
fn required(value:Option<&DslValue>)->Result<&DslValue,ValueError>{value.ok_or_else(invalid)}
fn movement(bytes:usize,depth:usize,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:depth,..Default::default()})?;work.admit(native)?;
 body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(())
}
fn text(value:&DslValue,output:&mut String,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let DslValue::String(value)=value else{return Err(invalid().into())};
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:value.len(),maximum_capacity_bytes:value.len(),maximum_depth:3,..Default::default()})?;work.admit(native)?;
 body.copy_text_into(native,value,output,3).map_err(Into::into)
}
fn optional_text(value:Option<&DslValue>,output:&mut Option<String>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 if let Some(value)=value{if !matches!(value,DslValue::String(_)){return Err(invalid().into())}movement(size_of::<Option<String>>(),3,native,body,work)?;*output=Some(String::new());text(value,output.as_mut().unwrap(),native,body,work)?}Ok(())
}
fn vector<T>(count:usize,output:&mut Vec<T>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let bytes=count.checked_mul(size_of::<T>()).ok_or_else(invalid)?;
 body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_depth:3,..Default::default()})?;work.admit(native)?;
 body.allocate_vec_into(native,count,output,3).map_err(Into::into)
}
fn strings(value:&DslValue,output:&mut Vec<String>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let DslValue::Array(values)=value else{return Err(invalid().into())};vector(values.len(),output,native,body,work)?;
 for value in values{if !matches!(value,DslValue::String(_)){return Err(invalid().into())}movement(size_of::<String>(),3,native,body,work)?;output.push(String::new());text(value,output.last_mut().unwrap(),native,body,work)?}Ok(())
}
#[derive(semio_framework_value::RetireOwned)]
struct Frame{cause:Option<NativeSnapshotFailure>,diagnostics:Vec<Diagnostic>,pending:Option<Diagnostic>}
impl IoError{
 /// 🧳️ Receives the complete refusal through the same caller and retains partial diagnostics until funded close.
 pub fn receive_value(value:&DslValue,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{
  owner.receive_nested::<Frame,Self,NativeSnapshotFailure>(|slot,owner|native_snapshot_turn(|work|{
   let[cause,diagnostics]=fields(value,["cause","diagnostics"])?;
   let cause=required(cause)?;let DslValue::Array(diagnostics)=required(diagnostics)? else{return Err(invalid().into())};
   let g=owner.remaining_grant();let bytes=size_of::<Frame>();if g.maximum_items==0||g.maximum_copy_bytes<bytes||g.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"IO refusal frame exceeds original grant").into())}
   work.admit(owner.native())?;*slot=Some(Frame{cause:None,diagnostics:Vec::new(),pending:None});owner.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
   slot.as_mut().unwrap().cause=Some(work.take(NativeSnapshotFailure::receive_value(cause,owner))?);
   work.take(owner.receive::<Frame,Self,NativeSnapshotFailure>(|destination,native,body|native_snapshot_turn(|work|{
    movement(size_of::<Option<Frame>>(),1,native,body,work)?;*destination=slot.take();let frame=destination.as_mut().unwrap();
    vector(diagnostics.len(),&mut frame.diagnostics,native,body,work)?;
    for value in diagnostics{
     receive_diagnostic(value,&mut frame.pending,native,body,work)?;
     movement(size_of::<Diagnostic>(),2,native,body,work)?;frame.diagnostics.push(frame.pending.take().unwrap());
    }
    movement(size_of::<Self>(),1,native,body,work)?;
    Ok(Self{cause:frame.cause.take().unwrap(),diagnostics:std::mem::take(&mut frame.diagnostics)})
   })))
  }))
 }
}
fn receive_diagnostic(value:&DslValue,output:&mut Option<Diagnostic>,native:&mut NativeDecodeControl<'_>,body:&mut NativeSnapshotBodyWallet,work:&mut NativeSnapshotWork)->Result<(),NativeSnapshotFailure>{
 let[code,severity,span,message,expected,scope]=fields(value,["code","severity","span","message","expected","scope"])?;
 let code=required(code)?;let message=required(message)?;
 let severity=Severity::from_value_controlled(required(severity)?,native)?;let span=TextSpan::from_value_controlled(required(span)?,native)?;
 movement(size_of::<Diagnostic>(),2,native,body,work)?;
 *output=Some(Diagnostic{code:FaultCode(String::new()),severity,span,message:String::new(),expected:None,scope:FaultScope::default()});
 let out=output.as_mut().unwrap();text(code,&mut out.code.0,native,body,work)?;text(message,&mut out.message,native,body,work)?;
 if let Some(expected)=expected{
  let[tokens,keywords,keys]=fields(expected,["tokens","keywords","keys"])?;
  movement(size_of::<Option<ExpectedSet>>(),2,native,body,work)?;
  out.expected=Some(ExpectedSet{tokens:Vec::new(),keywords:Vec::new(),keys:Vec::new()});let target=out.expected.as_mut().unwrap();
  strings(required(tokens)?,&mut target.tokens,native,body,work)?;strings(required(keywords)?,&mut target.keywords,native,body,work)?;strings(required(keys)?,&mut target.keys,native,body,work)?;
 }
 if let Some(scope)=scope{
  let[plugin,app,instance,module,body_key]=fields(scope,["pluginId","appId","instanceId","module","bodyKey"])?;
  optional_text(plugin,&mut out.scope.plugin_id,native,body,work)?;optional_text(app,&mut out.scope.app_id,native,body,work)?;optional_text(instance,&mut out.scope.instance_id,native,body,work)?;optional_text(module,&mut out.scope.module,native,body,work)?;optional_text(body_key,&mut out.scope.body_key,native,body,work)?;
 }
 Ok(())
}
