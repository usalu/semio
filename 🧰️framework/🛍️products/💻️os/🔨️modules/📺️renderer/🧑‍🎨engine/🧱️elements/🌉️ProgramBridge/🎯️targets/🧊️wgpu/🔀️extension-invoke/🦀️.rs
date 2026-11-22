//! 🔀️ Extension invoke: the native twin of the browser's `dispatchInvokeExtension` (`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🔌️plugin-module-loader/📞️extension-and-request-actors`). A requester's `Effect::InvokeExtension` is routed to the live-activated extension actor (`KernelRequest::ActivateExtension`) as one inbound `Event::Request`, the actor's `Effect::Respond` is settled into the SAME answer or fault the browser completes with, and exactly one `Event::Completed` goes back to the requesting instance. The routing and the settling are pure ([`route_extension_invoke`], [`settle_extension_answer`]) and shared with the law; the kernel exchange is native only. See [`super`].

use super::*;
use semio_framework::kernel::{Fault, FaultOrigin, FaultScope};

/// 📏️ The largest request one capability call may carry (the contiguous guest-request ceiling the browser refuses with `extension.request-too-large`).
pub const EXTENSION_REQUEST_CEILING_BYTES: usize = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
/// 📏️ The largest answer the host accepts back (the host-answer ceiling the browser refuses with `extension.answer-too-large`).
pub const EXTENSION_ANSWER_CEILING_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES;

//#region 🧭️PureRouting
/// 🔎️ What the registry knows of an enabled extension program: the host plugin it extends and its live actor instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtensionTarget<'a> {
    pub host: Option<&'a str>,
    pub instance: Option<u32>,
}

/// 🧭️ Where a request goes: to the extension's live instance, or to an instance activated beside the requester first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtensionRoute {
    Live(u32),
    ActivateBeside,
}

/// 📥️ What the extension actor answered, as the exchange saw it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtensionAnswer {
    Ok(Vec<u8>),
    Fault(Vec<u8>),
    Unanswered,
    Failed(String),
}

/// 🧯️ A fault of the browser's shape: origin `os`, never retryable, scoped to the extension (and the requesting instance unless `instance` is `None`).
pub fn extension_fault(code: &str, message: impl Into<String>, extension_id: &str, instance: Option<u32>) -> Fault {
    Fault::new(FaultOrigin::Os, code.to_string(), message).with_scope(FaultScope { plugin_id: Some(extension_id.to_string()), instance_id: instance.map(|id| id.to_string()), ..FaultScope::default() })
}

/// 🧭️ Routes one invoke. Refused exactly as the browser refuses: no enabled program, or one that extends a plugin other than the requester's, is `extension.invoke-unavailable`; an empty capability is `extension.invoke-failed`; a request over the ceiling is `extension.request-too-large`. A program with neither a live instance nor a declared host has nothing to activate and is unavailable.
pub fn route_extension_invoke(target: Option<ExtensionTarget<'_>>, requester_plugin: &str, requester_instance: u32, extension_id: &str, capability: &str, request_bytes: usize) -> Result<ExtensionRoute, Fault> {
    let unavailable = || extension_fault("extension.invoke-unavailable", "extension.invoke-unavailable", extension_id, Some(requester_instance));
    let target = target.ok_or_else(unavailable)?;
    if target.host.is_some_and(|host| host != requester_plugin) {
        return Err(unavailable());
    }
    if capability.is_empty() {
        return Err(extension_fault("extension.invoke-failed", "extension.capability-required", extension_id, Some(requester_instance)));
    }
    if request_bytes > EXTENSION_REQUEST_CEILING_BYTES {
        return Err(extension_fault("extension.request-too-large", format!("{capability} request of {request_bytes} B for {extension_id} exceeds the {EXTENSION_REQUEST_CEILING_BYTES}-byte contiguous guest-request ceiling"), extension_id, None));
    }
    match (target.instance, target.host) {
        (Some(instance), _) => Ok(ExtensionRoute::Live(instance)),
        (None, Some(_)) => Ok(ExtensionRoute::ActivateBeside),
        (None, None) => Err(unavailable()),
    }
}

/// ⚖️ Settles the actor's answer into the capability's JSON text, or the fault the requester is completed with. `decode_fault` reads the actor's fault bytes; undecodable ones are `extension.answer-not-a-fault`.
pub fn settle_extension_answer(answer: ExtensionAnswer, extension_id: &str, capability: &str, instance: u32, decode_fault: impl Fn(&[u8]) -> Option<Fault>) -> Result<String, Fault> {
    match answer {
        ExtensionAnswer::Ok(bytes) if bytes.len() > EXTENSION_ANSWER_CEILING_BYTES => Err(extension_fault("extension.answer-too-large", format!("extension answer of {} B exceeds the {EXTENSION_ANSWER_CEILING_BYTES}-byte host-answer ceiling", bytes.len()), extension_id, None)),
        ExtensionAnswer::Ok(bytes) => {
            let text = String::from_utf8(bytes).map_err(|error| extension_fault("extension.invoke-failed", error.to_string(), extension_id, Some(instance)))?;
            serde_json::from_str::<serde_json::Value>(&text).map_err(|error| extension_fault("extension.invoke-failed", error.to_string(), extension_id, Some(instance)))?;
            Ok(text)
        }
        ExtensionAnswer::Fault(bytes) => Err(decode_fault(&bytes).unwrap_or_else(|| extension_fault("extension.answer-not-a-fault", format!("extension {extension_id} refused {capability} with {} undecodable bytes", bytes.len()), extension_id, None))),
        ExtensionAnswer::Unanswered => Err(extension_fault("extension.request-unanswered", format!("extension {extension_id} did not answer {capability} within 1 of 1 turns"), extension_id, None)),
        ExtensionAnswer::Failed(message) => Err(extension_fault("extension.invoke-failed", message, extension_id, Some(instance))),
    }
}
//#endregion 🧭️PureRouting

#[cfg(not(target_arch = "wasm32"))]
static INBOUND_REQUEST_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// 🔀️ One invoke, end to end: route, activate beside the requester when needed, exchange, settle, and complete the requester once. Every refusal reaches the requester as a fault completion (never an `Err`), like the browser; only a failed completion is an `Err`.
#[cfg(not(target_arch = "wasm32"))]
pub(super) async fn dispatch_native(requester: &ProgramBridgeEntry, instance_id: u32, extension_id: &str, capability: &str, request_json: &str, req: u64) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
    let result = match invoke_answer(requester, instance_id, extension_id, capability, request_json).await {
        Ok(json) => match wasm_program_exchange::json_to_pack(&json) {
            Ok(pack) => semio_framework::kernel::RequestOutcome::Ok(pack),
            Err(message) => semio_framework::kernel::RequestOutcome::Err(wasm_program_exchange::fault_to_pack(&extension_fault("extension.invoke-failed", message, extension_id, Some(instance_id)))),
        },
        Err(fault) => semio_framework::kernel::RequestOutcome::Err(wasm_program_exchange::fault_to_pack(&fault)),
    };
    requester.complete_extension_request(instance_id, req, result).await
}

#[cfg(not(target_arch = "wasm32"))]
async fn invoke_answer(requester: &ProgramBridgeEntry, instance_id: u32, extension_id: &str, capability: &str, request_json: &str) -> Result<String, Fault> {
    let target = extension_splice::extension_invoke_target(extension_id);
    let route = route_extension_invoke(target.as_ref().map(|target| ExtensionTarget { host: target.host.as_deref(), instance: target.instance }), &requester.plugin_id, instance_id, extension_id, capability, request_json.len())?;
    let target = target.expect("a routed extension has a target");
    let payload = request_json.as_bytes().to_vec();
    let sequence = || semio_framework::kernel::RequestId(INBOUND_REQUEST_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
    let activate = || async {
        let host = target.host.as_deref().unwrap_or(&requester.plugin_id);
        let instance = target.entry.activate_extension_instance(instance_id, host, &target.capabilities).await.map_err(|error| extension_fault("extension.invoke-failed", error.to_string(), extension_id, Some(instance_id)))?;
        extension_splice::remember_extension_instance(extension_id, Some(instance));
        Ok::<u32, Fault>(instance)
    };
    let live = match route {
        ExtensionRoute::Live(instance) => instance,
        ExtensionRoute::ActivateBeside => activate().await?,
    };
    let mut answer = target.entry.answer_extension_request(live, instance_id, sequence(), capability, payload.clone()).await;
    if matches!(answer, ExtensionAnswer::Failed(_)) && target.host.is_some() && matches!(route, ExtensionRoute::Live(_)) {
        if let Ok(fresh) = activate().await {
            answer = target.entry.answer_extension_request(fresh, instance_id, sequence(), capability, payload).await;
        }
    }
    settle_extension_answer(answer, extension_id, capability, instance_id, |bytes| wasm_program_exchange::fault_from_pack(bytes))
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔀️extension-invoke/🦀️.rs"]
mod extension_invoke_tests;
