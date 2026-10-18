/// 🚦️ Zero fuel, partial grants and limits: the zero fuel, deadline and interrupted close law, the partial grant rejection, the 256 plus one and terminal empty laws, malformed sources and the panic fault.

#[test]
fn retained_vcs_zero_fuel_deadline_and_interrupted_close_preserve_every_credit() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 13, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    let before = session.credits();
    let mut rejected = retained_grant();
    rejected.fuel = 0;
    assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
    rejected = retained_grant();
    rejected.interrupted = true;
    assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
    rejected = retained_grant();
    rejected.deadline_milliseconds = rejected.now_milliseconds;
    assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
    assert_eq!(session.credits(), before);
    session.cancel(handle, retained_grant()).expect("cancel checkpoint");
    rejected = retained_grant();
    rejected.items = 0;
    assert_eq!(session.close_operation_step(handle, rejected).map_err(|failure|failure.fault), Err(FlowVcsFault::InsufficientGrant));
    assert_eq!(session.credits(), before);
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_every_mutating_control_rejects_partial_grants_without_state_change() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 15, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    for grant in rejected_control_grants() {
        let before = session.resource_fingerprint();
        assert_eq!(session.cancel(handle, grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.fault(handle, grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.panic_fault(handle, grant), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.resource_fingerprint(), before);
    }
    session.cancel(handle, retained_grant()).expect("valid cancel");
    for grant in rejected_control_grants() {
        let before = session.resource_fingerprint();
        assert_eq!(session.close_operation_step(handle, grant).map_err(|failure|failure.fault), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.resource_fingerprint(), before);
    }
    while !matches!(session.close_operation_step(handle, retained_grant()).expect("valid operation close"),RetainedCloneStep::Complete(_)) {}
    for grant in rejected_control_grants() {
        let before = session.resource_fingerprint();
        assert_eq!(session.close_retired_step(grant).map_err(|failure|failure.fault), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.resource_fingerprint(), before);
    }
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_256_plus_one_and_terminal_empty_laws_hold() {
    let mut fixed = FlowFixedOwners::<FlowVcsAction, FLOW_VCS_MAX_ITEMS>::new();
    for _ in 0..FLOW_VCS_MAX_ITEMS {
        assert!(fixed.push(FlowVcsAction::Checkpoint).is_ok());
    }
    assert!(matches!(fixed.push(FlowVcsAction::Checkpoint), Err(FlowVcsAction::Checkpoint)));
    for remaining in (0..FLOW_VCS_MAX_ITEMS).rev() {
        assert!(matches!(fixed.pop(), Some(FlowVcsAction::Checkpoint)));
        assert_eq!(fixed.len(), remaining);
    }
    assert!(fixed.is_empty());

    let mut session = FlowRetainedVcs::new(retained_fixture(), 17, 1, 0);
    assert_eq!(session.preflight(FlowVcsCensus { items: FLOW_VCS_MAX_ITEMS + 1, bytes: 0, depth: 1 }), Err(FlowVcsFault::Limit));
    assert!(session.preflight(FlowVcsCensus { items: FLOW_VCS_MAX_ITEMS, bytes: 0, depth: 1 }).is_ok());
    assert_eq!(session.credits(), FlowVcsCredits::default());

    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    let rediscovered = session.rediscover(handle.operation, handle.generation).expect("lost handle rediscovery");
    assert_eq!(rediscovered, handle);
    drive_to_preview(&mut session, rediscovered);
    let page = session.take_page(rediscovered).expect("page");
    session.acknowledge_page(rediscovered, page.sequence).expect("ACK");
    while !matches!(session.close_operation_step(rediscovered, retained_grant()).expect("operation close"),RetainedCloneStep::Complete(_)) {}
    session.begin_close();
    while !matches!(session.close_retired_step(retained_grant()).expect("session close"),RetainedCloneStep::Complete(_)) {}
    assert!(session.terminal_is_empty());
    let terminal = session.resource_fingerprint();
    assert!(matches!(session.close_retired_step(retained_grant()).expect("idempotent close"),RetainedCloneStep::Complete(_)));
    assert!(matches!(session.close_retired_step(retained_grant()).expect("repeated idempotent close"),RetainedCloneStep::Complete(_)));
    assert_eq!(session.resource_fingerprint(), terminal);
}

#[test]
fn retained_vcs_malformed_sources_fail_before_transfer_with_exact_fingerprint() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 19, 1, 0);
    let before = session.resource_fingerprint();
    let mut wrong_id = FlowVcsSource::new("other".to_owned());
    let mut patch = FlowVcsSource::new(Widget::InputNote { id: "source".into(), text: "patched".into() });
    assert_eq!(session.begin_patch_widget(session.authority(), &mut wrong_id, &mut patch), Err(FlowVcsFault::InvalidMutation));
    assert!(wrong_id.retained() && patch.retained());
    assert_eq!(session.resource_fingerprint(), before);
    assert_eq!(session.begin_patch_widget(session.authority(), &mut wrong_id, &mut patch), Err(FlowVcsFault::InvalidMutation));
    assert_eq!(session.resource_fingerprint(), before);

    let mut valid_id = FlowVcsSource::new("source".to_owned());
    let mut valid_patch = FlowVcsSource::new(Widget::InputNote { id: "source".into(), text: "valid".into() });
    assert!(session.begin_patch_widget(session.authority(), &mut valid_id, &mut valid_patch).is_ok());
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_panic_fault_preserves_exact_resources_and_next_close_progresses() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 23, 1, 0);
    let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
    session.poll(handle, retained_grant()).expect("progress");
    let before = session.resource_fingerprint();
    session.panic_fault(handle, retained_grant()).expect("panic fault");
    assert_eq!(session.resource_fingerprint(), before);
    assert_eq!(session.panic_fault(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
    assert_eq!(session.resource_fingerprint(), before);
    while !matches!(session.close_operation_step(handle, retained_grant()).expect("close after panic"),RetainedCloneStep::Complete(_)) {}
    close_to_terminal(&mut session);
}
