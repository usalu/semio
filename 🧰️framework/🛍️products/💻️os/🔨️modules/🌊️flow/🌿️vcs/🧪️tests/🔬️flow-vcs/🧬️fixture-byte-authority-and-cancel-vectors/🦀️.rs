/// 🧬️ Fixture byte, authority and cancel vectors: the exact multibyte and maximum byte vectors, the authority, malformed and grant vectors and the cancel and fault transitions.

#[test]
fn retained_vcs_fixture_byte_vectors_execute_exact_multibyte_max_and_max_plus_one_results() {
    let oracle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🧾️semantic-history/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("oracle fixture");
    let lifecycle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🔄️lifecycle/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("lifecycle fixture");
    for vector in lifecycle.get("byteVectors").and_then(semio_framework_pack_json::Value::as_array).expect("byte vectors") {
        let protocol = vector.get("protocol").expect("byte protocol");
        let input = protocol.get("operation").expect("byte operation");
        assert_eq!(input.get("feature").and_then(semio_framework_pack_json::Value::as_str), Some("removeWidget"));
        let value = match input.get("encoding").and_then(semio_framework_pack_json::Value::as_str).expect("byte encoding") {
            "literal" => input.get("value").and_then(semio_framework_pack_json::Value::as_str).expect("byte literal").to_owned(),
            "repeatUtf8" => input.get("unit").and_then(semio_framework_pack_json::Value::as_str).expect("byte unit").repeat(flow_oracle_usize(input, "repetitions")),
            encoding => panic!("unsupported byte encoding {encoding}"),
        };
        assert_eq!(value.chars().count(), flow_oracle_usize(input, "characterCount"));
        assert_eq!(value.len(), flow_oracle_usize(input, "byteLength"));
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        let mut source = FlowVcsSource::new(value);
        let authority = flow_hostile_authority(&session, input);
        let result = session.begin_remove_widget(authority, &mut source);
        let expected = vector.get("expected").expect("byte expected result");
        let expected_result = expected.get("result").and_then(semio_framework_pack_json::Value::as_str).expect("byte result");
        match result {
            Ok(handle) => {
                assert_eq!(expected_result, "accepted");
                assert_eq!(handle, flow_hostile_expected_handle(expected.get("expectedHandle").expect("byte expected handle")));
                assert_eq!(source.retained(), expected.get("sourceRetained").and_then(semio_framework_pack_json::Value::as_bool).expect("byte retained result"));
                assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("admissionState").and_then(semio_framework_pack_json::Value::as_str).expect("byte admission state")));
                let grant_name = protocol.get("cleanup").and_then(|value| value.get("grant")).and_then(semio_framework_pack_json::Value::as_str).expect("byte cleanup grant");
                let grant = flow_hostile_named_grant(&lifecycle, grant_name);
                let cleanup = protocol.get("cleanup").and_then(|value| value.get("control")).and_then(semio_framework_pack_json::Value::as_str).expect("byte cleanup control");
                assert_eq!(cleanup, "cancel");
                session.cancel(handle, grant).expect("byte vector cleanup cancel");
                flow_hostile_close_and_drain(&mut session, handle, grant);
            }
            Err(fault) => {
                assert_eq!(flow_hostile_fault_name(fault), expected_result);
                assert!(expected.get("expectedHandle").is_some_and(semio_framework_pack_json::Value::is_null));
                assert_eq!(source.retained(), expected.get("sourceRetained").and_then(semio_framework_pack_json::Value::as_bool).expect("byte retained result"));
            }
        }
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(semio_framework_pack_json::Value::as_str).expect("byte final state")));
        close_to_terminal(&mut session);
    }
}

#[test]
fn retained_vcs_fixture_authority_malformed_and_grant_vectors_execute_exact_results() {
    let oracle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🧾️semantic-history/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("oracle fixture");
    let lifecycle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🔄️lifecycle/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("lifecycle fixture");
    let valid_grant = flow_hostile_named_grant(&lifecycle, "valid");

    for vector in lifecycle.get("authorityVectors").and_then(semio_framework_pack_json::Value::as_array).expect("authority vectors") {
        let protocol = vector.get("protocol").expect("authority protocol");
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        let operation = protocol.get("operation").expect("authority operation");
        assert_eq!(operation.get("feature").and_then(semio_framework_pack_json::Value::as_str), Some("checkpoint"));
        let authority = operation.get("authority").map_or_else(
            || session.authority(),
            |value| FlowVcsAuthority {
                session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("authority session"),
                base_revision: flow_oracle_u64(value, "baseRevision"),
                parent_revision: flow_oracle_u64(value, "parentRevision"),
            },
        );
        let handle = session.begin_checkpoint(authority).expect("authority checkpoint admission");
        assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedAdmittedHandle").expect("authority admitted handle")));
        let call = protocol.get("call").expect("authority call");
        assert_eq!(call.get("method").and_then(semio_framework_pack_json::Value::as_str), Some("poll"));
        let grant = flow_hostile_named_grant(&lifecycle, call.get("grant").and_then(semio_framework_pack_json::Value::as_str).expect("authority grant"));
        let result = if let Some(polls) = call.get("polls").and_then(semio_framework_pack_json::Value::as_u64) {
            for _ in 1..polls {
                session.poll(handle, grant).expect("authority setup poll");
            }
            session.poll(handle, grant)
        } else {
            let forged = call.get("handle").expect("forged handle");
            if let Some(prior) = call.get("priorGeneration").and_then(semio_framework_pack_json::Value::as_u64) {
                assert_eq!(prior, u64::from(handle.generation));
            }
            session.poll(
                FlowVcsHandle {
                    operation: flow_oracle_u64(forged, "operation"),
                    slot: u8::try_from(flow_oracle_u64(forged, "slot")).expect("forged slot"),
                    generation: u32::try_from(flow_oracle_u64(forged, "generation")).expect("forged generation"),
                },
                grant,
            )
        };
        let expected = vector.get("expected").expect("authority expected");
        assert_eq!(flow_hostile_fault_name(result.expect_err("authority rejection")), expected.get("result").and_then(semio_framework_pack_json::Value::as_str).expect("authority result"));
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(semio_framework_pack_json::Value::as_str).expect("authority result state")));
        session.cancel(handle, valid_grant).expect("authority cleanup cancel");
        flow_hostile_close_and_drain(&mut session, handle, valid_grant);
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(semio_framework_pack_json::Value::as_str).expect("authority final state")));
        close_to_terminal(&mut session);
    }

    for vector in lifecycle.get("malformedVectors").and_then(semio_framework_pack_json::Value::as_array).expect("malformed vectors") {
        let protocol = vector.get("protocol").expect("malformed protocol");
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        let operation = protocol.get("operation").expect("malformed operation");
        assert_eq!(operation.get("feature").and_then(semio_framework_pack_json::Value::as_str), Some("patchWidget"));
        let id = operation.get("id").expect("malformed id");
        let widget = operation.get("widget").expect("malformed widget");
        let mut id_source =
            FlowVcsSource { value: id.get("present").and_then(semio_framework_pack_json::Value::as_bool).filter(|present| *present).map(|_| id.get("value").and_then(semio_framework_pack_json::Value::as_str).expect("malformed id value").to_owned()) };
        let mut widget_source = FlowVcsSource {
            value: widget
                .get("present")
                .and_then(semio_framework_pack_json::Value::as_bool)
                .filter(|present| *present)
                .map(|_| <Widget as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&widget.get("value").expect("malformed widget value").clone())).expect("malformed widget")),
        };
        let authority = flow_hostile_authority(&session, operation);
        let result = session.begin_patch_widget(authority, &mut id_source, &mut widget_source);
        let expected = vector.get("expected").expect("malformed expected");
        assert_eq!(flow_hostile_fault_name(result.expect_err("malformed rejection")), expected.get("result").and_then(semio_framework_pack_json::Value::as_str).expect("malformed result"));
        assert!(expected.get("expectedHandle").is_some_and(semio_framework_pack_json::Value::is_null));
        let sources = expected.get("sources").expect("malformed source results");
        assert_eq!(id_source.retained(), sources.get("idRetained").and_then(semio_framework_pack_json::Value::as_bool).expect("malformed id retained"));
        assert_eq!(widget_source.retained(), sources.get("widgetRetained").and_then(semio_framework_pack_json::Value::as_bool).expect("malformed widget retained"));
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(semio_framework_pack_json::Value::as_str).expect("malformed result state")));
        close_to_terminal(&mut session);
        retire_widget_source(&mut widget_source);
    }

    for vector in lifecycle.get("grantVectors").and_then(semio_framework_pack_json::Value::as_array).expect("grant vectors") {
        let protocol = vector.get("protocol").expect("grant protocol");
        let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
        assert_eq!(protocol.get("operation").and_then(|value| value.get("feature")).and_then(semio_framework_pack_json::Value::as_str), Some("checkpoint"));
        let handle = flow_hostile_begin_operation(&mut session, &lifecycle, &oracle, protocol.get("operation").expect("grant operation"));
        assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedHandle").expect("grant expected handle")));
        let call = protocol.get("call").expect("grant call");
        assert_eq!(call.get("method").and_then(semio_framework_pack_json::Value::as_str), Some("poll"));
        let result = session.poll(handle, flow_hostile_grant(call.get("grant").expect("grant input")));
        let actual_result = match result {
            Ok(FlowVcsPoll::Progress { .. }) => "progress",
            Ok(_) => "unexpectedPoll",
            Err(fault) => flow_hostile_fault_name(fault),
        };
        let expected = vector.get("expected").expect("grant expected");
        assert_eq!(actual_result, expected.get("result").and_then(semio_framework_pack_json::Value::as_str).expect("grant result"));
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(semio_framework_pack_json::Value::as_str).expect("grant result state")));
        session.cancel(handle, valid_grant).expect("grant cleanup cancel");
        flow_hostile_close_and_drain(&mut session, handle, valid_grant);
        assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(semio_framework_pack_json::Value::as_str).expect("grant final state")));
        close_to_terminal(&mut session);
    }
}

#[test]
fn retained_vcs_fixture_cancel_and_fault_execute_all_twenty_four_exact_transfer_states() {
    let oracle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🧾️semantic-history/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("oracle fixture");
    let lifecycle: semio_framework_pack_json::Value = semio_framework_pack_json::parse(read_node("🧪️tests/🧫️fixtures/🔄️lifecycle/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("lifecycle fixture");
    for boundary in lifecycle.get("transferControlLedger").and_then(semio_framework_pack_json::Value::as_array).expect("transfer control ledger") {
        let protocol = boundary.get("protocol").expect("transfer protocol");
        let target = protocol.get("target").expect("transfer target");
        let grant = flow_hostile_named_grant(&lifecycle, protocol.get("grant").and_then(semio_framework_pack_json::Value::as_str).expect("transfer grant"));
        for control in boundary.get("controls").and_then(semio_framework_pack_json::Value::as_array).expect("transfer controls") {
            let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
            flow_hostile_apply_setup(&mut session, protocol.get("setup").expect("transfer setup"));
            let handle = flow_hostile_begin_operation(&mut session, &lifecycle, &oracle, protocol.get("operation").expect("transfer operation"));
            assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedHandle").expect("transfer expected handle")));
            session.poll(handle, grant).expect("transfer admission progress");
            session.poll(handle, grant).expect("transfer admission checkpoint");

            let rollback_steps = target.get("rollbackSteps").and_then(semio_framework_pack_json::Value::as_u64);
            if rollback_steps.is_some() {
                for _ in 0..2048 {
                    let operation = session.operations[usize::from(handle.slot)].as_ref().expect("transfer operation");
                    if operation.cursor.phase == FlowVcsCursorPhase::PublishPage && operation.cursor.visibility_published {
                        break;
                    }
                    session.poll(handle, grant).expect("reach rollback publication boundary");
                }
            } else {
                for _ in 0..2048 {
                    if flow_hostile_cursor_matches(&session, handle, target) {
                        break;
                    }
                    session.poll(handle, grant).expect("reach transfer cursor target");
                }
                assert!(
                    flow_hostile_cursor_matches(&session, handle, target),
                    "transfer boundary {} never reached target {target:?}; cursor stopped at {:?}",
                    boundary.get("boundary").and_then(semio_framework_pack_json::Value::as_str).expect("transfer boundary"),
                    session.operations[usize::from(handle.slot)].as_ref().map(|operation| operation.cursor)
                );
            }

            let before_control = session.resource_fingerprint();
            let control_name = control.get("control").and_then(semio_framework_pack_json::Value::as_str).expect("transfer control");
            let result = match control_name {
                "cancel" => session.cancel(handle, grant),
                "fault" => session.fault(handle, grant),
                value => panic!("unsupported transfer control {value}"),
            };
            let expected = control.get("expected").expect("transfer expected");
            assert_eq!(result.map(|_| "ok").unwrap_or_else(flow_hostile_fault_name), expected.get("result").and_then(semio_framework_pack_json::Value::as_str).expect("transfer control result"));
            if let Some(steps) = rollback_steps {
                for _ in 0..steps {
                    assert!(!matches!(session.close_operation_step(handle, grant).expect("rollback boundary step"),RetainedCloneStep::Complete(_)));
                }
                assert!(
                    flow_hostile_cursor_matches(&session, handle, target),
                    "rollback boundary {} via {control_name} missed target {target:?}; cursor at {:?}",
                    boundary.get("boundary").and_then(semio_framework_pack_json::Value::as_str).expect("rollback boundary"),
                    session.operations[usize::from(handle.slot)].as_ref().map(|operation| operation.cursor)
                );
                let at_boundary = expected.get("atBoundary").expect("rollback expected boundary");
                flow_hostile_assert_rollback_boundary(&session, handle, protocol.get("operation").expect("rollback operation fixture"), target, at_boundary);
                assert_eq!(
                    flow_hostile_actual_state(&session),
                    flow_hostile_expected_state(&lifecycle, &oracle, at_boundary.get("state").and_then(semio_framework_pack_json::Value::as_str).expect("rollback boundary state")),
                    "fixture rollback boundary mismatch at {} via {}",
                    boundary.get("boundary").and_then(semio_framework_pack_json::Value::as_str).expect("rollback boundary"),
                    control_name
                );
                let before_repeat = session.resource_fingerprint();
                let repeat = match control_name {
                    "cancel" => session.cancel(handle, grant),
                    "fault" => session.fault(handle, grant),
                    value => panic!("unsupported repeated transfer control {value}"),
                };
                assert_eq!(repeat.map(|_| "ok").unwrap_or_else(flow_hostile_fault_name), expected.get("repeatResult").and_then(semio_framework_pack_json::Value::as_str).expect("rollback repeat result"));
                assert_eq!(session.resource_fingerprint(), before_repeat);
            } else if expected.get("result").and_then(semio_framework_pack_json::Value::as_str) == Some("duplicateControl") {
                assert_eq!(session.resource_fingerprint(), before_control);
            }
            flow_hostile_close_and_drain(&mut session, handle, grant);
            assert_eq!(
                flow_hostile_actual_state(&session),
                flow_hostile_expected_state(&lifecycle, &oracle, expected.get("finalState").and_then(semio_framework_pack_json::Value::as_str).expect("transfer final state")),
                "fixture transfer result mismatch at {} via {}",
                boundary.get("boundary").and_then(semio_framework_pack_json::Value::as_str).expect("transfer boundary"),
                control_name
            );
            close_to_terminal(&mut session);
        }
    }
}
