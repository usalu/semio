#[cfg(test)]
fn minimal_component_without_actor_world() -> &'static [u8] {
    b"\0asm\x0d\0\x01\0"
}

#[cfg(test)]
async fn wait_for_scripted_guest_relay_release(runtime: &GuestRuntimes, completion: &GuestRelayCompletion) {
    let GuestRuntimes::Mock(mock) = runtime else {
        return;
    };
    let barrier = match completion {
        GuestRelayCompletion::Started(Err(_)) => mock.take_relay_start_failure_release(),
        GuestRelayCompletion::Stepped(Err(_)) => mock.take_relay_step_failure_release(),
        _ => None,
    };
    if let Some(barrier) = barrier {
        barrier.wait().await;
    }
}

#[cfg_attr(not(test), allow(dead_code))]
// 🚫️async: E1 — pure in-memory encoder, no suspension point; reverted per R9 (its only
// consumer FakeCluster::exchange must itself be sync to satisfy run_transaction/undo_group
// production FnMut(...) -> Result<...> closure signature).
fn host_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    let code = code.into();
    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message))
}
