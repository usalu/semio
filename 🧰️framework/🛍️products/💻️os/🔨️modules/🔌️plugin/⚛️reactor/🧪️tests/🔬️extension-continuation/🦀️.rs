//! 🔁️ The extension-continuation seam of `Event::Completed` — the delivery path that did not exist
//! before ticket 26/09/09/PROCEDURAL-3D-END-TO-END. Every producer used to hand-mint a `RequestId`
//! into `Effect::InvokeExtension`; that id owned no registry slot, so `RequestRegistry::resolve`
//! dropped the extension's answer on the floor and no `flowEvalResolve`/`flowTessellateResolve` was
//! ever dispatched. These tests drive the exact three calls `poll`'s event loop chains together.

use super::*;

fn brep_invocation(response_action: &str, request_json: &str) -> crate::app::ExtensionInvocation {
    crate::app::ExtensionInvocation::new("brep", "tessellate", request_json, response_action)
}

/// 🎯️ A drained `Emit::extension_invocations` entry allocates a REAL registry slot and queues
/// exactly one `Effect::InvokeExtension` whose `req` is that slot's id — never a literal.
#[semio_framework_async_macros::async_test]
async fn a_queued_invocation_allocates_a_registry_slot_and_one_effect() {
    let drained_before = REGISTRY.with(|registry| registry.drain()).len();
    assert_eq!(drained_before, 0, "the per-thread registry must start this test empty");
    let req = queue_extension_invocation(7, &brep_invocation("flowTessellateResolve", r#"{"nodeHash":42,"handle":"h1"}"#)).expect("continuation admission");
    let effects = REGISTRY.with(|registry| registry.drain());
    assert_eq!(effects.len(), 1, "exactly one effect must be queued, got {effects:?}");
    match &effects[0] {
        Effect::InvokeExtension { req: minted, extension_id, capability, .. } => {
            assert_eq!(*minted, req, "the queued effect must carry the registry-minted id");
            assert_eq!(extension_id, "brep");
            assert_eq!(capability, "tessellate");
        }
        other => panic!("expected an InvokeExtension effect, got {other:?}"),
    }
    // 🧹️ Leave the shared per-thread registry as this test found it.
    assert!(take_extension_response(req, &Ok(Vec::new())).is_some());
}

/// 🎯️ The completion for that exact id resolves to a `response_action` dispatch carrying the
/// ORIGINAL request's own correlation fields plus the outcome payload — the whole point of the lane.
#[semio_framework_async_macros::async_test]
async fn a_completion_for_a_minted_id_dispatches_the_response_action_with_the_outcome() {
    let req = queue_extension_invocation(3, &brep_invocation("flowTessellateResolve", r#"{"nodeHash":99,"handle":"h7","tolerance":0.5}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let (instance, action, args) = take_extension_response(req, &Ok(br#"{"positions":[]}"#.to_vec())).expect("a minted continuation must answer its own completion");
    assert_eq!(instance, 3, "the dispatch must target the instance that asked");
    assert_eq!(action, "flowTessellateResolve");
    assert_eq!(args.get("nodeHash").and_then(dsl::DslValue::as_u64), Some(99), "the request's own correlation must survive: {args:?}");
    assert_eq!(args.get("handle").and_then(dsl::DslValue::as_str), Some("h7"));
    assert_eq!(args.get("ok").and_then(dsl::DslValue::as_bool), Some(true));
    assert_eq!(args.get("outputJson").and_then(dsl::DslValue::as_str), Some(r#"{"positions":[]}"#));
    assert!(take_extension_response(req, &Ok(Vec::new())).is_none(), "a continuation answers exactly once");
}

/// 🎯️ A faulted invocation still reaches the app — as `ok: false` plus the typed fault, never as a
/// silent drop, so a plugin can clear its own in-flight bookkeeping.
#[semio_framework_async_macros::async_test]
async fn a_faulted_invocation_dispatches_the_response_action_with_the_fault() {
    let req = queue_extension_invocation(1, &brep_invocation("flowEvalResolve", r#"{"nodeHash":5}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("brep.unsupported"), "no such operator".to_string());
    let (_, action, args) = take_extension_response(req, &Err(fault)).expect("a fault must still answer the continuation");
    assert_eq!(action, "flowEvalResolve");
    assert_eq!(args.get("ok").and_then(dsl::DslValue::as_bool), Some(false));
    assert_eq!(args.get("faultCode").and_then(dsl::DslValue::as_str), Some("brep.unsupported"));
    assert!(args.get("outputJson").is_none(), "a fault carries no output payload: {args:?}");
}

/// 🎯️ An id that never was a continuation (an ordinary parked-future request, or pure noise) must
/// fall through so `poll` still routes it to `RequestRegistry::resolve`.
#[semio_framework_async_macros::async_test]
async fn an_unknown_id_is_not_claimed_by_the_continuation_branch() {
    assert!(take_extension_response(semio_framework::kernel::RequestId(u64::MAX), &Ok(Vec::new())).is_none());
}
