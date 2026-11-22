//! 🔀️ LAW: the native extension invoke routes and settles exactly as the browser's `dispatchInvokeExtension` does (same refusal codes, scopes and ceilings; the codes are read from the language-neutral fixture both sides share), and never reaches an extension that is not the requester's own host extension.

use super::*;

const FIXTURE: &str = include_str!("../../../🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("the shared fixture is JSON")
}

fn code(fault: &Fault) -> &str {
    &fault.code.0
}

fn host(name: &str, instance: Option<u32>) -> Option<ExtensionTarget<'_>> {
    Some(ExtensionTarget { host: Some(name), instance })
}

fn route(target: Option<ExtensionTarget<'_>>, capability: &str, bytes: usize) -> Result<ExtensionRoute, Fault> {
    route_extension_invoke(target, "flow", 7, "flow-extension-text", capability, bytes)
}

fn no_fault(_: &[u8]) -> Option<Fault> {
    None
}

#[test]
fn an_unreachable_extension_faults_with_the_fixtures_invoke_unavailable_code() {
    let expected = fixture()["missing"].as_array().unwrap().iter().find(|row| row["phase"] == "invoke").unwrap()["code"].as_str().unwrap().to_string();
    for target in [None, host("other-plugin", Some(3)), Some(ExtensionTarget { host: None, instance: None })] {
        let fault = route(target, "evaluate", 10).unwrap_err();
        assert_eq!(code(&fault), expected);
        assert_eq!(fault.message, expected);
        assert_eq!((fault.scope.plugin_id.as_deref(), fault.scope.instance_id.as_deref()), (Some("flow-extension-text"), Some("7")));
        assert!(!fault.retryable);
    }
}

#[test]
fn a_hosted_extension_routes_to_its_live_instance_or_is_activated_beside_the_requester() {
    assert_eq!(route(host("flow", Some(9)), "evaluate", 10), Ok(ExtensionRoute::Live(9)));
    assert_eq!(route(host("flow", None), "evaluate", 10), Ok(ExtensionRoute::ActivateBeside));
    assert_eq!(route(Some(ExtensionTarget { host: None, instance: Some(4) }), "evaluate", 10), Ok(ExtensionRoute::Live(4)));
}

#[test]
fn an_empty_capability_and_an_oversized_request_are_refused_before_any_actor_is_reached() {
    let empty = route(host("flow", Some(9)), "", 10).unwrap_err();
    assert_eq!((code(&empty), empty.message.as_str()), ("extension.invoke-failed", "extension.capability-required"));
    assert_eq!(route(host("flow", Some(9)), "evaluate", EXTENSION_REQUEST_CEILING_BYTES), Ok(ExtensionRoute::Live(9)));
    let large = route(host("flow", Some(9)), "evaluate", EXTENSION_REQUEST_CEILING_BYTES + 1).unwrap_err();
    assert_eq!(code(&large), "extension.request-too-large");
    assert!(large.message.starts_with(&format!("evaluate request of {} B for flow-extension-text exceeds the {EXTENSION_REQUEST_CEILING_BYTES}-byte", EXTENSION_REQUEST_CEILING_BYTES + 1)));
    assert_eq!((large.scope.plugin_id.as_deref(), large.scope.instance_id.as_deref()), (Some("flow-extension-text"), None));
}

#[test]
fn an_answer_settles_to_its_json_text_and_every_failure_to_the_browsers_fault() {
    let fixture_response = fixture()["response"].to_string();
    assert_eq!(settle_extension_answer(ExtensionAnswer::Ok(fixture_response.clone().into_bytes()), "e", "evaluate", 7, no_fault), Ok(fixture_response));
    let failure = |answer| settle_extension_answer(answer, "e", "evaluate", 7, no_fault).unwrap_err();
    assert_eq!(code(&failure(ExtensionAnswer::Ok(vec![0xff, 0xfe]))), "extension.invoke-failed");
    assert_eq!(code(&failure(ExtensionAnswer::Ok(b"not json".to_vec()))), "extension.invoke-failed");
    let too_large = failure(ExtensionAnswer::Ok(vec![b' '; EXTENSION_ANSWER_CEILING_BYTES + 1]));
    assert_eq!(code(&too_large), "extension.answer-too-large");
    assert_eq!(too_large.scope.instance_id, None);
    assert_eq!(code(&failure(ExtensionAnswer::Fault(vec![1, 2, 3]))), "extension.answer-not-a-fault");
    assert_eq!(code(&failure(ExtensionAnswer::Unanswered)), "extension.request-unanswered");
    let failed = failure(ExtensionAnswer::Failed("kernel: gone".into()));
    assert_eq!((code(&failed), failed.message.as_str(), failed.scope.instance_id.as_deref()), ("extension.invoke-failed", "kernel: gone", Some("7")));
}

#[test]
fn the_extensions_own_typed_fault_reaches_the_requester_unchanged() {
    let typed = extension_fault("extension.evaluate.bad-request", "Invalid authored input", "flow-extension-text", None);
    let expected = typed.clone();
    let settled = settle_extension_answer(ExtensionAnswer::Fault(vec![9]), "flow-extension-text", "evaluate", 7, move |bytes| (bytes == [9]).then(|| typed.clone())).unwrap_err();
    assert_eq!(settled, expected);
}
