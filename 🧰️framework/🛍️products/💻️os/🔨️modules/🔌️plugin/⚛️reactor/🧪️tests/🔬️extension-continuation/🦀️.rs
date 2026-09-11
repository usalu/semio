//! 🔁️ The extension-continuation seam of `Event::Completed` — the delivery path that did not exist
//! before ticket 26/09/09/PROCEDURAL-3D-END-TO-END. Every producer used to hand-mint a `RequestId`
//! into `Effect::InvokeExtension`; that id owned no registry slot, so `RequestRegistry::resolve`
//! dropped the extension's answer on the floor and no `flowEvalResolve`/`flowTessellateResolve` was
//! ever dispatched. These tests drive the exact three calls `poll`'s event loop chains together.

use super::*;

fn brep_invocation(response_action: &str, request_json: &str) -> crate::app::ExtensionInvocation {
    crate::app::ExtensionInvocation::new("brep", "tessellate", request_json, response_action)
}

/// 📦️ The shape the SHELL puts on the ABI — `encodePackValue(JSON.parse(outputJson))`.
fn packed_answer(json: &str) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&dsl::json::from_json_str::<dsl::DslValue>(json).expect("fixture answer is JSON"))
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
    assert!(take_extension_response(req, Ok(Vec::new())).is_ok());
}

/// 🎯️ The completion for that exact id resolves to a `response_action` dispatch carrying the
/// ORIGINAL request's own correlation fields plus the outcome payload — the whole point of the lane.
#[semio_framework_async_macros::async_test]
async fn a_completion_for_a_minted_id_dispatches_the_response_action_with_the_outcome() {
    let req = queue_extension_invocation(3, &brep_invocation("flowTessellateResolve", r#"{"nodeHash":99,"handle":"h7","tolerance":0.5}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let (instance, action, args) = take_extension_response(req, Ok(packed_answer(r#"{"positions":[]}"#))).expect("a minted continuation must answer its own completion");
    assert_eq!(instance, 3, "the dispatch must target the instance that asked");
    assert_eq!(action, "flowTessellateResolve");
    assert_eq!(args.get("nodeHash").and_then(dsl::DslValue::as_u64), Some(99), "the request's own correlation must survive: {args:?}");
    assert_eq!(args.get("handle").and_then(dsl::DslValue::as_str), Some("h7"));
    assert_eq!(args.get("ok").and_then(dsl::DslValue::as_bool), Some(true));
    assert_eq!(args.get("outputJson").and_then(dsl::DslValue::as_str), Some(r#"{"positions":[]}"#), "the packed answer must reach the app as its own JSON, not as container bytes: {args:?}");
    assert!(take_extension_response(req, Ok(Vec::new())).is_err(), "a continuation answers exactly once");
}

/// 🎯️ A faulted invocation still reaches the app — as `ok: false` plus the typed fault, never as a
/// silent drop, so a plugin can clear its own in-flight bookkeeping.
#[semio_framework_async_macros::async_test]
async fn a_faulted_invocation_dispatches_the_response_action_with_the_fault() {
    let req = queue_extension_invocation(1, &brep_invocation("flowEvalResolve", r#"{"nodeHash":5}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let fault = semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("brep.unsupported"), "no such operator".to_string());
    let (_, action, args) = take_extension_response(req, Err(fault)).expect("a fault must still answer the continuation");
    assert_eq!(action, "flowEvalResolve");
    assert_eq!(args.get("ok").and_then(dsl::DslValue::as_bool), Some(false));
    assert_eq!(args.get("faultCode").and_then(dsl::DslValue::as_str), Some("brep.unsupported"));
    assert!(args.get("outputJson").is_none(), "a fault carries no output payload: {args:?}");
}

/// 🎯️ An id that never was a continuation (an ordinary parked-future request, or pure noise) must
/// fall through so `poll` still routes it to `RequestRegistry::resolve`.
#[semio_framework_async_macros::async_test]
async fn an_unknown_id_is_not_claimed_by_the_continuation_branch() {
    assert!(take_extension_response(semio_framework::kernel::RequestId(u64::MAX), Ok(Vec::new())).is_err());
}

/// ⚖️ LAW: a host answer larger than the guest's declared contiguous-request ceiling reaches the
/// response action WHOLE, delivered as prologue pages plus the terminal page the completion carries.
///
/// 🧨️ Boot #12 of ticket 26/09/09/PROCEDURAL-3D-END-TO-END proved why paging is not optional: one
/// unbounded `pack` is lowered with ONE `cabi_realloc`, and a block the guest allocator refuses
/// aborts the actor before any guest code runs — no fault, no diagnosis, an endless restore loop.
#[semio_framework_async_macros::async_test]
async fn a_paged_answer_reaches_the_response_action_whole() {
    let req = queue_extension_invocation(4, &brep_invocation("flowTessellateResolve", r#"{"nodeHash":11}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let mesh = "m".repeat(1_048_576);
    let answer = packed_answer(&dsl::json::to_json_string(&dsl::DslValue::object([("mesh".to_string(), dsl::DslValue::String(mesh.clone()))])));
    assert!(answer.len() > 1_048_576, "the fixture answer must be the megabyte the law names");
    let page_bytes = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
    let full_pages = (answer.len() - 1) / page_bytes;
    let (prologue, terminal) = answer.split_at(full_pages * page_bytes);
    assert_eq!(full_pages + 1, semio_framework_trace::guest_host_answer_pages(answer.len()), "the cut must agree with the declared page count");
    assert!(!terminal.is_empty() && terminal.len() <= page_bytes);
    for page in prologue.chunks(page_bytes) {
        assert!(page.len() <= page_bytes, "no page may exceed the contiguous-request ceiling");
        assert!(append_extension_response_page(req, page), "a continuation must claim its own prologue pages");
    }
    let (_, action, args) = take_extension_response(req, Ok(terminal.to_vec())).expect("the terminal page answers the continuation");
    assert_eq!(action, "flowTessellateResolve");
    assert_eq!(args.get("ok").and_then(dsl::DslValue::as_bool), Some(true), "a paged megabyte must decode, not fault: {args:?}");
    let decoded = dsl::json::from_json_str::<dsl::DslValue>(args.get("outputJson").and_then(dsl::DslValue::as_str).expect("outputJson")).expect("outputJson is JSON");
    assert_eq!(decoded.get("mesh").and_then(dsl::DslValue::as_str).map(str::len), Some(mesh.len()), "every delivered page must survive the assembly");
}

/// ⚖️ LAW: a page over the contiguous-request ceiling, or an assembled answer over the host-answer
/// ceiling, is answered with a typed fault the app SEES — never with an allocation the guest cannot
/// serve. `ok: false` reaches the response action either way.
#[semio_framework_async_macros::async_test]
async fn an_over_ceiling_answer_faults_instead_of_growing_the_guest() {
    let oversized_page = queue_extension_invocation(5, &brep_invocation("flowEvalResolve", r#"{"nodeHash":12}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    assert!(append_extension_response_page(oversized_page, &vec![0u8; semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES + 1]));
    let (_, _, args) = take_extension_response(oversized_page, Ok(Vec::new())).expect("a poisoned accumulator still answers");
    assert_eq!(args.get("ok").and_then(dsl::DslValue::as_bool), Some(false));
    assert_eq!(args.get("faultCode").and_then(dsl::DslValue::as_str), Some("plugin.request-registry.answer-too-large"));

    let over_total = queue_extension_invocation(6, &brep_invocation("flowEvalResolve", r#"{"nodeHash":13}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let page = vec![0u8; semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES];
    for _ in 0..=semio_framework_trace::guest_host_answer_pages(semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES) {
        assert!(append_extension_response_page(over_total, &page));
    }
    let (_, _, args) = take_extension_response(over_total, Ok(Vec::new())).expect("a poisoned accumulator still answers");
    assert_eq!(args.get("faultCode").and_then(dsl::DslValue::as_str), Some("plugin.request-registry.answer-too-large"));
}

/// ⚖️ LAW: the echoed correlation is correlation, not the request BODY. A field the shell could not
/// have SENT as a structurally addressed command argument is not one the SDK hands back — echoing it
/// would carry the payload into the guest a second time, on top of the outcome.
#[semio_framework_async_macros::async_test]
async fn the_echoed_correlation_drops_a_request_body_the_shell_could_not_have_sent() {
    let body = "x".repeat(semio_framework::PUBLIC_INVOCATION_STRING_BYTES + 1);
    let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
        ("nodeHash".to_string(), dsl::DslValue::uint(77)),
        ("windowId".to_string(), dsl::DslValue::String("preview".to_string())),
        ("inputJson".to_string(), dsl::DslValue::String(body)),
    ]));
    let req = queue_extension_invocation(2, &crate::app::ExtensionInvocation::new("math", "evaluate", &request_json, "flowEvalResolve")).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let (_, _, args) = take_extension_response(req, Ok(packed_answer(r#"{"value":1}"#))).expect("a minted continuation must answer its own completion");
    assert_eq!(args.get("nodeHash").and_then(dsl::DslValue::as_u64), Some(77), "bounded correlation survives: {args:?}");
    assert_eq!(args.get("windowId").and_then(dsl::DslValue::as_str), Some("preview"));
    assert!(args.get("inputJson").is_none(), "the request body must not ride back into the guest: {args:?}");
    assert_eq!(args.get("outputJson").and_then(dsl::DslValue::as_str), Some(r#"{"value":1}"#));
}

/// ⚖️ LAW: `completion-result.fault` is a `pack` of the fault value — the same bytes the shell
/// writes with `encodePackValue(fault)` and the native host writes with `encode_fault_pack`.
/// JSON (`encode_fault_bytes`) is not this ABI arm. Guest `outcome_to_result` must recover
/// `code` and `message` from that pack, or a readable error is mojibake.
#[semio_framework_async_macros::async_test]
async fn a_packed_host_fault_round_trips_through_outcome_to_result() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/extension-result-fault-pack.json")).expect("fault-pack fixture");
    let fault_json = serde_json::to_string(&fixture["fault"]).expect("fault object");
    let fault_value = dsl::json::from_json_str::<dsl::DslValue>(&fault_json).expect("fixture fault is a DSL value");
    let fault: semio_framework::Fault = dsl::from_dsl_value(fault_value).expect("fixture fault is a Fault");
    let packed = crate::host::encode_fault_pack(&fault);
    assert!(serde_json::from_slice::<serde_json::Value>(&packed).is_err(), "the ABI fault arm must not be a JSON string: {packed:?}");
    let decoded = crate::host::outcome_to_result(semio_framework::kernel::RequestOutcome::Err(packed)).expect_err("the err arm decodes a fault");
    assert_eq!(decoded.code.0, "extension.missing");
    assert_eq!(decoded.message, "no such extension");
    let req = queue_extension_invocation(2, &brep_invocation("flowEvalResolve", r#"{"nodeHash":8}"#)).expect("continuation admission");
    let _ = REGISTRY.with(|registry| registry.drain());
    let (_, action, args) = take_extension_response(req, Err(decoded)).expect("a packed host fault must reach the response action readable");
    assert_eq!(action, "flowEvalResolve");
    assert_eq!(args.get("ok").and_then(dsl::DslValue::as_bool), Some(false));
    assert_eq!(args.get("faultCode").and_then(dsl::DslValue::as_str), Some("extension.missing"));
    assert_eq!(args.get("faultMessage").and_then(dsl::DslValue::as_str), Some("no such extension"));
}
