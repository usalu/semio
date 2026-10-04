//! 🧩 Installed remote inference codecs and revocable owner contribution lifetime.
use super::*;
use semio_framework_value::DslValue;
use semio_framework_os_kernel::os_directory::client::{DocumentHttpPortDeclarationV1,DocumentHttpPortCodeV1,HttpMethod};
use semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1;

/// 📦 The executable installs owner behavior; transport authority remains with the gateway.
#[derive(Clone)]
pub struct RemoteInferenceProtocolV1 {
    pub owner:&'static str,
    pub service_id:&'static str,
    pub route:&'static str,
    pub declaration:fn()->DocumentHttpPortDeclarationV1,
    pub encode:fn(&str,&DslValue)->Result<DslValue,InferenceRouteErrorV1>,
    pub receipt:fn(DslValue)->Result<DslValue,InferenceRouteErrorV1>,
    pub page:fn(DslValue,&str)->Result<DslValue,InferenceRouteErrorV1>,
    pub approval:fn(DslValue,&DocumentScope,&str,&str)->Result<DslValue,InferenceRouteErrorV1>,
    pub undo:fn(DslValue,&DslValue)->Result<DslValue,InferenceRouteErrorV1>,
    pub error:fn(u16,&[u8])->InferenceRouteErrorV1,
}

struct RetainedProtocolV1 {
    protocol:RemoteInferenceProtocolV1,
    active:std::sync::atomic::AtomicBool,
    calls:std::sync::Mutex<Vec<semio_framework_async::CancelToken>>,
}

static PROTOCOLS:std::sync::OnceLock<std::sync::Mutex<Vec<Arc<RetainedProtocolV1>>>>=std::sync::OnceLock::new();
fn protocols()->&'static std::sync::Mutex<Vec<Arc<RetainedProtocolV1>>> {PROTOCOLS.get_or_init(Default::default)}

/// 🔐 Installs a bounded unique inventory and retires every prior call before publication.
pub fn install_remote_inference_protocols_v1(entries:Vec<RemoteInferenceProtocolV1>)->Result<(),GatewayError> {
    if entries.len()>64 || entries.iter().enumerate().any(|(index,entry)|entry.owner.is_empty() || entry.owner.len()>128 || entry.service_id.is_empty() || entry.service_id.len()>256 || !is_hub_inference_route(entry.route) || entries[..index].iter().any(|prior|prior.service_id==entry.service_id || prior.route==entry.route)) {return Err(GatewayError::new(GatewayErrorCode::InputInvalid,"invalid installed remote service inventory"));}
    for entry in &entries {let declaration=(entry.declaration)();if declaration.service_id!=entry.service_id {return Err(GatewayError::new(GatewayErrorCode::InputInvalid,"installed service identity differs from its owner declaration"));}CompiledDocumentHttpPortV1::compile(entry.owner,declaration).map_err(|_|GatewayError::new(GatewayErrorCode::InputInvalid,"invalid owner document service declaration"))?;}
    let mut current=protocols().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    for entry in current.drain(..) {
        entry.active.store(false,std::sync::atomic::Ordering::Release);
        for cancel in entry.calls.lock().unwrap_or_else(std::sync::PoisonError::into_inner).drain(..) {cancel.cancel_now();}
    }
    *current=entries.into_iter().map(|protocol|Arc::new(RetainedProtocolV1 {protocol,active:std::sync::atomic::AtomicBool::new(true),calls:Default::default()})).collect();
    Ok(())
}

pub fn remote_inference_protocol_v1(route:&str)->Result<RemoteInferenceProtocolV1,InferenceRouteErrorV1> {
    protocols().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().find(|entry|entry.protocol.route==route && entry.active.load(std::sync::atomic::Ordering::Acquire)).map(|entry|entry.protocol.clone()).ok_or(InferenceRouteErrorV1::Unavailable)
}

pub async fn call_remote_inference_v1<T:InferenceHubTransport>(transport:&T,context:&OperationContext,origin:&str,scope:&DocumentScope,route:&str,action:&str,payload:&DslValue)->Result<DslValue,InferenceRouteErrorV1> {
    if context.cancel.is_cancelled_now() {return Err(InferenceRouteErrorV1::Cancelled);}
    let retained=protocols().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().find(|entry|entry.protocol.route==route && entry.active.load(std::sync::atomic::Ordering::Acquire)).cloned().ok_or(InferenceRouteErrorV1::Unavailable)?;
    CompiledDocumentHttpPortV1::validate_payload(payload,64*1024).map_err(port_error)?;
    let protocol=&retained.protocol;
    let port=CompiledDocumentHttpPortV1::compile(protocol.owner,(protocol.declaration)()).map_err(port_error)?;
    let request=port.prepare(action,&(protocol.encode)(action,payload)?).map_err(port_error)?;
    let path=format!("/spaces/{}/documents/{}/{}{}",percent_encode(&scope.space_id),percent_encode(&scope.document_id),request.segments.iter().map(|segment|percent_encode(segment)).collect::<Vec<_>>().join("/"),request.after.map(|after|format!("?after={after}")).unwrap_or_default());
    let mut call_context=context.clone();
    call_context.cancel=context.cancel.child_now();
    {
        let mut calls=retained.calls.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        calls.retain(|cancel|!cancel.is_cancelled_now());
        if !retained.active.load(std::sync::atomic::Ordering::Acquire) {return Err(InferenceRouteErrorV1::Cancelled);}
        if calls.len()>=32 {return Err(InferenceRouteErrorV1::Capacity);}
        calls.push(call_context.cancel.clone());
    }
    let response=transport.request(&call_context,&InferenceHubRequestV1 {hub_origin:origin.into(),method:if request.method==HttpMethod::Get {InferenceHubMethodV1::Get}else{InferenceHubMethodV1::Post},path,body:request.body.unwrap_or_default(),maximum_response_bytes:request.response_max_bytes}).await;
    call_context.cancel.cancel_now();
    if !retained.active.load(std::sync::atomic::Ordering::Acquire) || context.cancel.is_cancelled_now() {return Err(InferenceRouteErrorV1::Cancelled);}
    let response=response.map_err(InferenceRouteErrorV1::from)?;
    if !(200..=299).contains(&response.status) {return Err((protocol.error)(response.status,&response.body));}
    port.decode(action,&response.body).map_err(port_error)
}

fn port_error(code:DocumentHttpPortCodeV1)->InferenceRouteErrorV1 {
    match code {
        DocumentHttpPortCodeV1::Invalid=>InferenceRouteErrorV1::Invalid,
        DocumentHttpPortCodeV1::Bounds=>InferenceRouteErrorV1::Bounds,
        DocumentHttpPortCodeV1::Cancelled=>InferenceRouteErrorV1::Cancelled,
        DocumentHttpPortCodeV1::Denied=>InferenceRouteErrorV1::Denied,
        DocumentHttpPortCodeV1::NotFound=>InferenceRouteErrorV1::NotFound,
        DocumentHttpPortCodeV1::Conflict=>InferenceRouteErrorV1::Conflict,
        DocumentHttpPortCodeV1::Gone=>InferenceRouteErrorV1::Expired,
        DocumentHttpPortCodeV1::Capacity=>InferenceRouteErrorV1::Capacity,
        _=>InferenceRouteErrorV1::Unavailable,
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
