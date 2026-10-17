/// 🤖️ Headless agent and pending exchanges: the headless agent operations, abandoned exchange close, pending response grants and faults, the guest fault wire and the probe codec.

/// 🪟️ LAW: a headless agent's transaction carries document operations and owned children only — a verb's config/draft view
/// state beside a document change is omitted with a preview warning, a verb whose whole effect is view state is refused
/// `interactive-job.agent-lane-uncarried` at prepare, and a verb with no view lane passes untouched.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_headless_agent_carries_document_operations_and_leaves_view_state_in_the_shell() {
    let ops = |document: usize, config: usize, draft: usize, children: &[u8]| PreparedOps { document: vec![vec![1]; document], config: vec![vec![2]; config], draft: vec![vec![3]; draft], children: children.to_vec() };
    assert_eq!(headless_agent_ops("p.editor.rename", ops(2, 0, 0, &[])).expect("document only"), (ops(2, 0, 0, &[]), Vec::new()));
    assert_eq!(headless_agent_ops("p.editor.addGeneration", ops(1, 1, 0, &[])).expect("document beside view state"), (ops(1, 0, 0, &[]), vec![VIEW_STATE_OMITTED_WARNING.to_string()]));
    assert_eq!(headless_agent_ops("p.editor.addWidget", ops(0, 0, 1, &[9])).expect("children beside view state"), (ops(0, 0, 0, &[9]), vec![VIEW_STATE_OMITTED_WARNING.to_string()]));
    let refused = headless_agent_ops("p.editor.setShowMode", ops(0, 1, 1, &[])).expect_err("view state alone");
    assert_eq!(refused.code, AGENT_LANE_UNCARRIED_FAULT_CODE);
    assert!(refused.message.contains("config and draft") && refused.message.contains("p.editor.setShowMode"), "{}", refused.message);
}

/// 🧹️ LAW: the exchange an abandoned command left on an instance (its wall budget ran out, or its turn faulted and the
/// instance was discarded) is closed before the next command on that instance is admitted — never met by the next
/// command's own turn, which asserted on the seq and took the whole gateway down (live hub coverage, 7800/p33); the
/// command's own exchange is never touched.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_abandoned_commands_exchange_is_closed_before_the_next_command_on_its_instance() {
    let driver = |seq: u64| {
        let mut owners = semio_framework::kernel::CommandEnvelopeSet::try_new().expect("envelope set");
        let command = ::semio_framework_async::poll::resolve_ready(store::encode_app_command(&store::AppCommand::ReadDocument { seq })).expect("the command encodes");
        assert!(owners.try_push(semio_framework::kernel::CommandEnvelope { instance: 3, seq, command }).is_ok());
        let Ok(batch) = semio_framework::kernel::CommandBatch::try_new(seq, owners) else { panic!("command batch") };
        semio_framework::kernel::CommandBatchDriver::new(seq, batch)
    };
    let mut exchanges = PendingExchangeRegistry::<1>::new();
    let mut closes = semio_framework::kernel::CommandDriverRegistry::<1>::new();
    closes.insert_admitted(3, 5, driver(5));
    exchanges.insert_admitted(PendingExchange { instance: 3, seq: 5, response: PendingResponsePage::Empty, ingress_fault: None });
    begin_closing_abandoned_exchange(&exchanges, &mut closes, 3, 5).expect("the command's own exchange");
    assert!(closes.is_active(3, 5), "a command's own exchange is never closed");
    begin_closing_abandoned_exchange(&exchanges, &mut closes, 3, 6).expect("an abandoned exchange");
    assert!(!closes.is_active(3, 5), "the abandoned exchange is closing");
    let mut retired = false;
    for _ in 0..64 {
        let (response_complete, _) = exchanges.close_response_step(3, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
        if !response_complete {
            continue;
        }
        let (complete, _, _) = closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
        if complete {
            exchanges.remove(3).expect("the abandoned exchange");
            retired = true;
            break;
        }
    }
    assert!(retired && exchanges.can_insert(3) && closes.can_insert(3), "the next command on the instance is admitted");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pending_response_close_releases_one_grant_at_a_time() {
    let mut response = PendingResponsePage::Empty;
    response.admit_frame(&vec![7; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]);
    assert_eq!(response.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1), (false, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1));
    assert_eq!(response.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (true, 1));
    assert!(response.terminal_is_empty());
}

/// ⚖️ LAW: an answer past the declared host-answer ceiling is a typed fault, a second answer for the
/// same sequence is a typed fault, and a command that settled silently answers a stamped `Done` —
/// the reply-stamp rule the React host applies (`commandIngressNeedsReplyStampV1`), without which
/// every verb whose guest has nothing to say would hang on an answer that is never coming.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pending_response_faults_oversize_and_duplicate_and_stamps_a_silent_settlement() {
    let mut oversized = PendingResponsePage::Empty;
    oversized.admit_frame(&vec![1; semio_framework::kernel::COMMAND_MAXIMUM_BYTES + 1]);
    assert_eq!(oversized.take(9).unwrap_err().code, "channel.not-wired");
    let mut duplicate = PendingResponsePage::Empty;
    duplicate.admit_frame(&[2]);
    duplicate.admit_frame(&[3]);
    assert!(duplicate.take(10).unwrap_err().message.contains("more than one frame"));
    let mut silent = PendingResponsePage::Empty;
    silent.stamp_settled();
    assert_eq!(silent.take(11).expect("a settled command answers"), store::AppFrame::Done { in_reply_to: 11 });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn guest_fault_wire_decodes_through_the_first_party_value_codec() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text("../../🧪️tests/🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    let wire = semio_framework_value::DslValue::from(&fixture["guestFault"]["wire"]);
    let fault = decode_guest_fault(&store::pack_rt::encode_wire_value(&wire));
    assert_eq!(fault.code, fixture["guestFault"]["expected"]["code"].as_str().expect("fixture fault code"));
    assert_eq!(fault.message, fixture["guestFault"]["expected"]["message"].as_str().expect("fixture fault message"));
}

/// 🪲️ Regression law for the two-frame `to_value` → `to_dsl_value` → `to_value` recursion that
/// aborted every probe-committing test with `fatal runtime error: stack overflow`: reaching an
/// assertion at all proves the cycle is gone, and the fixture pins the exact wire shape so the
/// cure cannot silently change the encoding. `serde_json` is the independent third-party oracle
/// — the first-party `ToValue` tree must equal what it serializes for the same values.
#[test]
fn probe_codec_encodes_the_fixture_shape_and_agrees_with_the_third_party_serializer() {
    use semio_framework_value::{FromValue as _, ToValue as _};
    let fixture: serde_json::Value = serde_json::from_str(fixture_text("../../🧪️tests/🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    let probe = &fixture["probeCodec"];
    let value = probe["value"].clone();
    let snapshot = ProbeSnapshot(value.clone().into());
    let diff = ProbeDiff(value.clone().into());
    let mutation = ProbeMutation::SetValue(value.clone().into());

    assert_eq!(serde_json::Value::from(snapshot.to_value()), probe["snapshotEncoding"]);
    assert_eq!(serde_json::Value::from(diff.to_value()), probe["diffEncoding"]);
    assert_eq!(serde_json::Value::from(mutation.to_value()), probe["mutationEncoding"]);
    assert_eq!(serde_json::to_value(&snapshot).expect("serde oracle"), probe["snapshotEncoding"]);
    assert_eq!(serde_json::to_value(&diff).expect("serde oracle"), probe["diffEncoding"]);
    assert_eq!(serde_json::to_value(&mutation).expect("serde oracle"), probe["mutationEncoding"]);

    assert_eq!(ProbeSnapshot::from_value(snapshot.to_value()).expect("snapshot round trip"), snapshot);
    assert_eq!(ProbeDiff::from_value(diff.to_value()).expect("diff round trip"), diff);
    assert_eq!(ProbeMutation::from_value(mutation.to_value()).expect("mutation round trip"), mutation);

    for rejected in probe["rejectedMutationEncodings"].as_array().expect("fixture rejection cases") {
        assert!(ProbeMutation::from_value(semio_framework_value::DslValue::from(rejected)).is_err(), "must reject {rejected}");
    }
}
