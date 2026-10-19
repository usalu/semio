/// 🔁️ Rejection, stale ABA and vector signatures: the repeated rejection law, the stale ABA cancel and acknowledge law, the thirteen fixture operations and the language-neutral vector signatures.

#[test]
fn retained_vcs_repeated_rejection_preserves_source_and_credits_then_valid_control_progresses() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 7, 10, 9);
    let before = session.credits();
    let mut too_large = FlowVcsSource::new(Widget::InputNote { id: "large".into(), text: "x".repeat(FLOW_VCS_MAX_BYTES + 1) });
    assert_eq!(session.begin_add_widget(session.authority(), 2, &mut too_large), Err(FlowVcsFault::Limit));
    assert_eq!(session.begin_add_widget(session.authority(), 2, &mut too_large), Err(FlowVcsFault::Limit));
    assert!(too_large.retained());
    assert_eq!(session.credits(), before);

    let mut valid = FlowVcsSource::new(Widget::InputNote { id: "valid".into(), text: "next".into() });
    let handle = session.begin_add_widget(session.authority(), 2, &mut valid).expect("valid request follows repeated rejection");
    assert!(!valid.retained());
    assert!(matches!(drive_to_preview(&mut session, handle), FlowVcsPoll::Preview { .. }));
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_stale_aba_cancel_ack_and_incremental_close_are_fail_closed() {
    let mut session = FlowRetainedVcs::new(retained_fixture(), 3, 2, 1);
    session.bind_surface(41, 73, 5).expect("surface owner");
    let before_digest = flow_vcs_host_snapshot_scalar_digest(session.document.as_ref().expect("document").host_snapshot());
    let stale = FlowVcsAuthority { base_revision: 1, ..session.authority() };
    let mut source = FlowVcsSource::new("source".to_owned());
    let handle = session.begin_remove_widget(stale, &mut source).expect("stale work may be admitted but not published");
    session.poll(handle, retained_grant()).expect("progress");
    session.poll(handle, retained_grant()).expect("checkpoint");
    let credits = session.credits();
    assert_eq!(session.poll(handle, retained_grant()), Err(FlowVcsFault::StaleAuthority));
    assert_eq!(session.poll(handle, retained_grant()), Err(FlowVcsFault::StaleAuthority));
    assert_eq!(session.credits(), credits);
    assert_eq!(flow_vcs_host_snapshot_scalar_digest(session.document.as_ref().expect("document").host_snapshot()), before_digest);
    session.cancel(handle, retained_grant()).expect("valid cancel follows rejection");
    assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
    while !matches!(session.close_operation_step(handle, retained_grant()).expect("incremental close"),RetainedCloneStep::Complete(_)) {}
    while !matches!(session.close_retired_step(retained_grant()).expect("retired source close"),RetainedCloneStep::Complete(_)) {}
    assert_eq!(session.credits(), FlowVcsCredits::default());
    assert_eq!(session.rediscover(handle.operation, handle.generation), Err(FlowVcsFault::StaleHandle));
    close_to_terminal(&mut session);
}

#[test]
fn retained_vcs_all_thirteen_fixture_operations_match_independent_third_party_oracle_after_ack_close() {
    let source = read_node("🧪️tests/🧫️fixtures/🧾️semantic-history/🔣️.json");
    let expected = SerdeJsonFlowOracle.expected_operations(source);
    let independently_evaluated = SerdeJsonFlowOracle.evaluate_operations(source);
    assert_eq!(independently_evaluated, expected);
    assert_eq!(expected.len(), FLOW_HISTORY_FEATURES.len());

    let root: semio_framework_pack_json::Value = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("retained oracle fixture");
    let initial = <FlowHostSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&root.get("initial").expect("oracle initial document").clone())).expect("oracle initial Flow fixture");
    let operations = root.get("operations").and_then(semio_framework_pack_json::Value::as_array).expect("oracle operation ledger");
    let mut session = FlowRetainedVcs::new(initial, 77, 1, 0);
    let mut actual = Vec::new();
    for (operation, expected_feature) in operations.iter().zip(FLOW_HISTORY_FEATURES) {
        let feature = operation.get("feature").and_then(semio_framework_pack_json::Value::as_str).expect("oracle feature");
        assert_eq!(feature, expected_feature);
        let handle = flow_oracle_begin_operation(&mut session, operation);
        let page = publish_and_close(&mut session, handle);
        actual.push(flow_oracle_actual_case(feature, &session, page));
    }
    assert_eq!(actual, independently_evaluated);
    close_layout_session(&mut session);
}

#[test]
fn retained_vcs_language_neutral_vector_signatures_detect_every_field_and_value_mutation() {
    let oracle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🧾️semantic-history/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("oracle fixture");
    let lifecycle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🔄️lifecycle/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("lifecycle fixture");
    let owners: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🫴️owners/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owner fixture");
    let operations = oracle.get("operations").and_then(semio_framework_pack_json::Value::as_array).expect("operation ledger");
    assert_eq!(operations.len(), FLOW_HISTORY_FEATURES.len());
    for (operation, feature) in operations.iter().zip(FLOW_HISTORY_FEATURES) {
        assert_eq!(operation.get("feature").and_then(semio_framework_pack_json::Value::as_str), Some(feature));
        let expected = operation.get("expected").expect("expected operation ledger");
        assert!(operation.get("input").is_some());
        assert!(expected.get("document").is_some());
        assert!(expected.get("page").is_some());
        assert!(expected.get("history").is_some());
        assert!(expected.get("handback").is_some());
    }
    let signatures = lifecycle.get("hostileVectorDigests").expect("hostile vector signatures");
    for name in ["byteVectors", "authorityVectors", "malformedVectors", "grantVectors", "transferControlLedger"] {
        let values = lifecycle.get(name).and_then(semio_framework_pack_json::Value::as_array).expect("hostile vector collection");
        let expected = signatures.get(name).and_then(semio_framework_pack_json::Value::as_array).expect("hostile vector digest collection");
        assert_eq!(values.len(), expected.len());
        for (value, digest) in values.iter().zip(expected) {
            flow_hostile_assert_every_scalar_is_signed(value, digest.as_str().and_then(|value| value.parse().ok()).expect("hostile vector digest"));
        }
    }
    for name in ["fingerprints", "expectedStates", "protocolDocuments"] {
        let values = lifecycle.get(name).and_then(semio_framework_pack_json::Value::as_object).expect("hostile vector map");
        let expected = signatures.get(name).and_then(semio_framework_pack_json::Value::as_object).expect("hostile vector digest map");
        assert_eq!(values.len(), expected.len());
        for (key, value) in values {
            flow_hostile_assert_every_scalar_is_signed(value, expected.get(key).and_then(semio_framework_pack_json::Value::as_str).and_then(|value| value.parse().ok()).expect("hostile map digest"));
        }
    }
    assert_eq!(lifecycle.get("byteVectors").and_then(semio_framework_pack_json::Value::as_array).expect("byte vectors").len(), 3);
    assert_eq!(lifecycle.get("authorityVectors").and_then(semio_framework_pack_json::Value::as_array).expect("authority vectors").len(), 4);
    assert_eq!(lifecycle.get("malformedVectors").and_then(semio_framework_pack_json::Value::as_array).expect("malformed vectors").len(), 3);
    assert_eq!(lifecycle.get("grantVectors").and_then(semio_framework_pack_json::Value::as_array).expect("grant vectors").len(), 5);
    let transfers = lifecycle.get("transferControlLedger").and_then(semio_framework_pack_json::Value::as_array).expect("transfer ledgers");
    assert_eq!(transfers.len(), 24);
    assert!(transfers.iter().all(|value| value.get("controls").and_then(semio_framework_pack_json::Value::as_array).is_some_and(|controls| controls.len() == 2)));
    let rollback = transfers.iter().filter(|value| value.get("protocol").and_then(|protocol| protocol.get("target")).and_then(|target| target.get("rollbackSteps")).is_some()).collect::<Vec<_>>();
    assert_eq!(rollback.len(), 5);
    for value in rollback {
        let controls = value.get("controls").and_then(semio_framework_pack_json::Value::as_array).expect("rollback controls");
        assert_eq!(controls[0].get("control").and_then(semio_framework_pack_json::Value::as_str), Some("cancel"));
        assert_eq!(controls[1].get("control").and_then(semio_framework_pack_json::Value::as_str), Some("fault"));
        assert!(controls.iter().all(|control| control.get("expected").and_then(|expected| expected.get("result")).and_then(semio_framework_pack_json::Value::as_str) == Some("ok")));
        assert_eq!(controls[0].get("expected").and_then(|expected| expected.get("atBoundary")).and_then(|boundary| boundary.get("stage")).and_then(semio_framework_pack_json::Value::as_str), Some("Cancelled"));
        assert_eq!(controls[1].get("expected").and_then(|expected| expected.get("atBoundary")).and_then(|boundary| boundary.get("stage")).and_then(semio_framework_pack_json::Value::as_str), Some("Faulted"));
    }
    assert_eq!(owners.get("fixtureLedgers").and_then(|value| value.get("hostileOmissionLaws")).and_then(semio_framework_pack_json::Value::as_array).expect("hostile omission laws").len(), 17);

    let laws = read_node("🧪️tests/🔬️flow-vcs/🦀️.rs");
    let source = &laws[..laws.find("fn retained_vcs_language_neutral_vector_signatures_detect_every_field_and_value_mutation").expect("this law's own definition")];
    for required in ["evaluate_operations", "flow_oracle_apply_operation", "flow_oracle_actual_case(feature, &session, page)", "flow_hostile_expected_state", "flow_hostile_actual_state", "flow_hostile_assert_every_scalar_is_signed"] {
        assert!(source.contains(required), "oracle extraction source law lacks {required}");
        assert!(!source.replace(required, "").contains(required), "hostile omission must fail the extraction gate for {required}");
    }
    let forbidden_literals = [["semantic: \"widget", "Count+1\""].concat(), ["semantic: \"inverse", "Published\""].concat(), ["feature_", "cases("].concat()];
    for forbidden in forbidden_literals {
        assert!(!source.contains(&forbidden), "literal oracle label remains reachable: {forbidden}");
    }
}
