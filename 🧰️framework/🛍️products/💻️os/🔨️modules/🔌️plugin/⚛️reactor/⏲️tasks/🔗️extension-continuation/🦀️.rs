//! 🔗️ Host handles and the extension invocation continuation over the actor request registry.

use super::*;

/// 🌐️ Every `host::Host` handle vended to plugin/extension code shares this actor's one
/// `RequestRegistry`, scoped to `instance` (`RequestRegistry::for_instance` — design-abi.md §4's
/// per-request instance tagging, so `Event::InstanceClose` can cancel exactly this instance's
/// pending host round-trips and no other's) — see `host::Host::new`.
///
/// 🌐️ Captures the actor's existing request registry without polling a future.
pub fn host_for_instance(instance: u32) -> crate::host::Host {
    let registry = REGISTRY.with(|registry| registry.for_instance(instance));
    crate::host::Host::new(registry)
}
//#region 🔖️ExtensionContinuation
/// 🔁️ Mints one `Effect::InvokeExtension` for `instance` through this actor's `RequestRegistry` and
/// records `response_action` against the minted id — the ONE place in the repo where that effect is
/// created on the guest side outside `host::Host::invoke_extension`'s awaiting variant. Returns the
/// minted id (diagnostic; the effect itself rides the registry's own outbound queue, drained into
/// `turn-result.effects` by `poll` later in the SAME turn).
pub(crate) fn queue_extension_invocation(instance: u32, invocation: &crate::app_emission::ExtensionInvocation) -> Result<semio_framework::kernel::RequestId, semio_framework::Fault> {
    let registry = REGISTRY.with(|registry| registry.for_instance(instance));
    let extension_id = invocation.extension_id.clone();
    let capability = invocation.capability.clone();
    let request_json = invocation.request_json.clone();
    registry.request_continuation(invocation.response_action.clone(), invocation.request_json.clone(), move |req| Effect::invoke_extension(req, extension_id, capability, request_json))
}

/// 📤️ Drains this actor's queued outbound effects — exactly what `poll` folds into
/// `turn-result.effects` once per turn. A fixture that drives a `PluginApp` without a live
/// `PluginRuntime` has no `poll`, so this is its only way to observe a minted
/// `Effect::InvokeExtension` (`🔌️plugin/🦀️.rs`'s `artifact_app_laws::settle_extension_invocations`).
pub(crate) fn drain_queued_effects(instance: u32) -> Vec<Effect> {
    REGISTRY.with(|registry| registry.for_instance(instance).drain())
}

/// 📬️ Donates the decoded output or complete native Fault into the addressed response command.
/// The request retains its bounded correlation fields; failures cross separately as owned headers.
pub fn extension_response_args(request_json: &str, outcome: Result<Vec<u8>, semio_framework::Fault>) -> (semio_framework_value::DslValue, Option<semio_framework::Fault>) {
    let mut fields: Vec<(String, semio_framework_value::DslValue)> = match semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(request_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(semio_framework_value::DslValue::Object(object)) => object,
        _ => Vec::new(),
    };
    fields.retain(|(key, value)| {
        if key == "ok" || key == "output" || key == "fault" || key == "faultCode" || key == "faultMessage" {
            return false;
        }
        let over_bound = matches!(value, semio_framework_value::DslValue::String(text) if text.chars().map(semio_framework::public_invocation_char_cost).sum::<usize>() > semio_framework::PUBLIC_INVOCATION_STRING_BYTES);
        if over_bound && semio_framework_trace::runtime_diagnostics_enabled() {
            eprintln!("[TRACE] extension response dropped the oversized request field {key:?} from the echoed correlation");
        }
        !over_bound
    });
    let original_fault = match outcome {
        Ok(bytes) => match store::pack_rt::decode_wire_value(&bytes) {
            Ok(value) => {
                fields.push(("ok".to_string(), semio_framework_value::DslValue::Bool(true)));
                fields.push(("output".to_string(), value));
                None
            }
            Err(error) => {
                fields.push(("ok".to_string(), semio_framework_value::DslValue::Bool(false)));
                fields.push(("output".to_string(), semio_framework_value::DslValue::Null));
                Some(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("extension.answer-not-a-pack"), format!("{} answer bytes are not a pack wire value: {error}", bytes.len())))
            }
        },
        Err(fault) => {
            fields.push(("ok".to_string(), semio_framework_value::DslValue::Bool(false)));
            fields.push(("output".to_string(), semio_framework_value::DslValue::Null));
            Some(fault)
        }
    };
    (semio_framework_value::DslValue::object(fields), original_fault)
}

/// 🔁️ `Event::Completed`'s continuation branch, factored out of `poll`'s event loop so it is unit
/// testable without a live `PluginRuntime`: takes the id's continuation (if any) and builds the
/// exact `(instance, action, args, original_fault)` the follow-up dispatch uses. An unclaimed id is an
/// ordinary parked-future request and must go to `RequestRegistry::resolve` instead.
pub(crate) fn take_extension_response(req: semio_framework::kernel::RequestId, terminal: Result<Vec<u8>, semio_framework::Fault>) -> Result<(u32, String, semio_framework_value::DslValue, Option<semio_framework::Fault>), Result<Vec<u8>, semio_framework::Fault>> {
    let Some(continuation) = REGISTRY.with(|registry| registry.take_continuation(req)) else { return Err(terminal) };
    let (instance, response_action, request_json, outcome) = continuation.into_response(terminal);
    let (args, original_fault) = extension_response_args(&request_json, outcome);
    Ok((instance, response_action, args, original_fault))
}

/// 📄️ Accumulates one PROLOGUE page of `req`'s answer — the door a host answer larger than one
/// `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` page arrives through, whose terminal page rides the
/// `Event::Completed` that answers the request. `false` means `req` owns no continuation and the
/// caller must fall through to the parked-future chunk path.
pub(super) fn append_extension_response_page(req: semio_framework::kernel::RequestId, bytes: &[u8]) -> bool {
    REGISTRY.with(|registry| registry.append_continuation_chunk(req, bytes)) == requests::ContinuationChunkStep::Accumulated
}
//#endregion 🔖️ExtensionContinuation

/// 🌐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the instance-agnostic sibling of
/// `host_for_instance` — a `Host` scoped to instance 0, the SAME "no instance tag declared" default
/// `RequestRegistry::for_instance`'s own doc names. `⚛️reactor/💼️jobs/🦀️.rs::spawn_job`
/// calls this (as `crate::reactor::host()`, zero args): a job is actor-global, not tied to one open
/// instance the way an `AsyncTask` is, so it has no `instance: u32` to scope by in the first place.
pub fn host() -> crate::host::Host {
    REGISTRY.with(|registry| crate::host::Host::new(registry.clone()))
}
