//! 🧫️ Authored tagged pack wire values share the normative operation codec.

use super::*;
use protocol::OpBinary;
#[path="../../🧪️tests/🧰️bank.rs"]
mod bank;

const ORIGINAL_PROTOCOL: &str = include_str!("../../🧪️tests/🧫️fixtures/🏷️protocol.semio");

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive)]
enum TaggedPackValueMutation {
    Remove { text: String },
    Rename { text: String },
}

impl OpBinary for TaggedPackValueMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        tagged_value_binary::encode_op(ORIGINAL_PROTOCOL, tagged_value_binary::VariantTag::Key, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        tagged_value_binary::decode_op(ORIGINAL_PROTOCOL, tagged_value_binary::VariantTag::Key, bytes)
    }
}

crate::original_pack_value_operation_receiving!(TaggedPackValueMutation, tagged_key = ORIGINAL_PROTOCOL);

#[test]
fn original_tagged_pack_value_receives_the_protocol_variant_and_exact_payload_range() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🏷️tagged.json")).unwrap();
    let expected = TaggedPackValueMutation::Rename { text: fixture["text"].as_str().unwrap().into() };
    let encoded: Vec<u8> = serde_json::from_value(fixture["packWire"].clone()).unwrap();
    assert_eq!(expected.encode_op().unwrap(), encoded);
    let mut source = vec![0xff, 0xff];
    let start = source.len();
    source.extend_from_slice(&encoded);
    let end = source.len();
    source.push(fixture["malformed"]["trailingByte"].as_u64().unwrap() as u8);
    let grant = RetainedCloneGrant {
        maximum_items: 64,
        maximum_copy_bytes: 1 << 19,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 2 << 20,
        maximum_depth: 128,
    };
    let stock = ReceivingStock {
        stock: RetainedCloneGrant {
            maximum_items: 1 << 20,
            maximum_copy_bytes: 1 << 24,
            maximum_capacity_bytes: 1 << 24,
            maximum_release_bytes: 1 << 24,
            maximum_depth: grant.maximum_depth,
        },
        quantum: grant,
        fuel: 1 << 20,
    };
    let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
    let refused = TaggedPackValueMutation::admit_receiving(&source, (start, end), denied);
    assert_eq!(refused.work, 0);
    assert!(refused.result.unwrap().is_none());
    let output = receive_to_completion::<TaggedPackValueMutation>(&source, (start, end), stock).unwrap();
    assert_eq!(output, expected);
    assert_eq!(output, TaggedPackValueMutation::decode_op(&encoded).unwrap());
    let malformed = ReceivingStock { fuel: fixture["malformed"]["fuel"].as_u64().unwrap(), ..stock };
    for end in [start + 2, end + 1] {
        let error = receive_to_completion::<TaggedPackValueMutation>(&source, (start, end), malformed).unwrap_err();
        assert_eq!(error.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{error:?}");
    }

    let remove = TaggedPackValueMutation::Remove { text: fixture["text"].as_str().unwrap().into() };
    let wire = remove.encode_op().unwrap();
    assert_eq!(wire[1] as u64, fixture["firstTag"].as_u64().unwrap());
    assert_eq!(receive_to_completion::<TaggedPackValueMutation>(&wire, (0, wire.len()), stock).unwrap(), remove);
}

#[test]
fn original_tagged_pack_value_resumes_a_large_body_after_copy_denial() {
    use bank::OperationBank;
    use semio_framework_value_native_owned::OriginalSourceTurnIssuer;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🏷️tagged.json")).unwrap();
    let profile = &fixture["large"];
    let body = profile["bodyBytes"].as_u64().unwrap() as usize;
    let refused_copy = profile["refusedCopyBytes"].as_u64().unwrap() as usize;
    let text = fixture["text"].as_str().unwrap().repeat(profile["repetitions"].as_u64().unwrap() as usize);
    assert!(text.len() > refused_copy && refused_copy > body);
    let expected = TaggedPackValueMutation::Rename { text };
    let wire = expected.encode_op().unwrap();
    assert_eq!(wire.len(), profile["wireBytes"].as_u64().unwrap() as usize);
    let inline_prefix: Vec<u8> = serde_json::from_value(profile["inlinePrefix"].clone()).unwrap();
    assert!(wire.starts_with(&inline_prefix));
    let quantum = RetainedCloneGrant {
        maximum_items: 64,
        maximum_copy_bytes: 1 << 19,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 2 << 20,
        maximum_depth: 128,
    };
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 20,
        maximum_copy_bytes: 1 << 24,
        maximum_capacity_bytes: 1 << 24,
        maximum_release_bytes: 1 << 24,
        ..quantum
    };
    let issuer = OriginalSourceTurnIssuer::new();
    let dispatch = issuer.issue().unwrap();
    let mut bank = OperationBank::new(quantum, stock, 1 << 20, dispatch.identity());
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(stock.maximum_capacity_bytes, &mut checkpoint);
    native.with_source_witness(Some(dispatch.witness()), |native| {
        let admission = TaggedPackValueMutation::admit_receiving(&wire, (0, wire.len()), bank.grant());
        let (mut cursor, progress) = admission.result.unwrap().unwrap();
        bank.debit(progress, admission.work);
        let (mut denied_copy, mut body_copy_turns) = (false, 0);
        while !cursor.is_ready() {
            let grant = bank.grant();
            let quote = cursor.demands(&wire, body).unwrap();
            let work = cursor.step_work_demand(&wire, body).unwrap();
            if !denied_copy && quote.copy_bytes > refused_copy {
                let turn_work = work.saturating_add(semio_framework_value::ORIGINAL_INVOCATION_WORK);
                let denied = bank.native(native, RetainedCloneGrant { maximum_copy_bytes: refused_copy, ..grant }, turn_work, |native, grant| {
                    cursor.step(&wire, native, body, grant)
                });
                assert_eq!((denied.result.unwrap(), denied.work), (Default::default(), 0));
                assert_eq!(cursor.demands(&wire, body).unwrap(), quote);
                assert_eq!(cursor.step_work_demand(&wire, body).unwrap(), work);
                denied_copy = true;
            }
            let before = NativeDecodeControl::transitions();
            let advance = bank.normal(native, work, |native, grant| cursor.step(&wire, native, body, grant));
            let progress = advance.result.unwrap();
            assert!(advance.work != 0 || progress != Default::default() || NativeDecodeControl::transitions() != before, "stalled at {quote:?}");
            if ((body / 2) as u64..=(body + std::mem::size_of::<usize>()) as u64).contains(&work) {
                assert!(progress.copied_bytes >= body / 2, "body copy quote {quote:?}");
                body_copy_turns += 1;
            }
        }
        let mut output = None;
        let take = cursor.take_into(&mut output, bank.grant());
        bank.debit(take.result.unwrap(), take.work);
        for _ in 0..1 << 20 {
            if cursor.terminal_is_empty() { break }
            let close = bank.close_native(native, |effect| cursor.close_step(effect));
            close.result.unwrap();
        }
        assert!(cursor.terminal_is_empty());
        assert!(denied_copy);
        assert!(body_copy_turns >= wire.len() / body);
        assert_eq!(output, Some(expected));
    });
}

#[test]
fn original_tagged_pack_value_closes_after_selected_prefixes() {
    use bank::OperationBank;
    use semio_framework_value_native_owned::OriginalSourceTurnIssuer;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🏷️tagged.json")).unwrap();
    let source: Vec<u8> = serde_json::from_value(fixture["packWire"].clone()).unwrap();
    let quantum = RetainedCloneGrant {
        maximum_items: 64,
        maximum_copy_bytes: 1 << 19,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 2 << 20,
        maximum_depth: 128,
    };
    let physical = RetainedCloneGrant {
        maximum_items: 1 << 20,
        maximum_copy_bytes: 1 << 24,
        maximum_capacity_bytes: 1 << 24,
        maximum_release_bytes: 1 << 24,
        ..quantum
    };
    for prefix in fixture["cancelAfterPaidTurns"].as_array().unwrap() {
        let issuer = OriginalSourceTurnIssuer::new();
        let dispatch = issuer.issue().unwrap();
        let mut bank = OperationBank::new(quantum, physical, 1 << 20, dispatch.identity());
        let mut checkpoint = |_| true;
        let mut native = NativeDecodeControl::new(physical.maximum_capacity_bytes, &mut checkpoint);
        native.with_source_witness(Some(dispatch.witness()), |native| {
            let admission = TaggedPackValueMutation::admit_receiving(&source, (0, source.len()), bank.grant());
            let (mut cursor, progress) = admission.result.unwrap().unwrap();
            bank.debit(progress, admission.work);
            for _ in 0..prefix.as_u64().unwrap() {
                if cursor.is_ready() { break }
                let grant = bank.grant();
                let body = grant.maximum_copy_bytes;
                let quote = cursor.demands(&source, body).unwrap();
                let denied = bank.native(native, RetainedCloneGrant { maximum_items: 0, ..grant }, 0, |native, grant| {
                    cursor.step(&source, native, body, grant)
                });
                assert_eq!((denied.result.unwrap(), denied.work), (Default::default(), 0));
                assert_eq!(cursor.demands(&source, body).unwrap(), quote);
                let work = cursor.step_work_demand(&source, body).unwrap();
                bank.normal(native, work, |native, grant| cursor.step(&source, native, body, grant)).result.unwrap();
            }
            for _ in 0..4096 {
                if cursor.terminal_is_empty() { break }
                let retired = bank.close_native(native, |effect| cursor.close_step(effect));
                if let Err(error) = retired.result {
                    std::mem::forget(cursor);
                    panic!("prefix {prefix} failed original close {error:?}");
                }
            }
            assert!(cursor.terminal_is_empty(), "prefix {prefix}");
            assert_eq!(
                physical.maximum_capacity_bytes - bank.left.maximum_capacity_bytes,
                physical.maximum_release_bytes - bank.left.maximum_release_bytes,
                "prefix {prefix}",
            );
        });
    }
}
