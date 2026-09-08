
use super::*;

fn operation(code: u16) -> AbiOperation {
    AbiOperation::try_new(code).unwrap()
}

fn bytes(value: Vec<u8>) -> AbiBytes {
    AbiBytes::try_new(value).unwrap()
}

fn handle(slot: u32, generation: u32) -> AbiHandle {
    AbiHandle { slot, generation }
}

fn reply(request_id: u64, generation: u32) -> AbiReply {
    AbiReply { request_id: AbiRequestId(request_id), generation, status: AbiStatus::OK, bytes: bytes(Vec::new()) }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn fixtures() -> Vec<(&'static str, Vec<u8>)> {
    ABI_LEDGER_FIXTURE
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (name, value) = line.split_once('\t').unwrap();
            (name, unhex(value))
        })
        .collect()
}

#[test]
fn schema_and_language_agnostic_fixture_publish_all_owned_contracts() {
    for name in ["AbiRequest", "AbiReply", "AbiEvent", "AbiPage", "AbiControl", "AbiError"] {
        assert!(ABI_SCHEMA_JSON.contains(&format!("\"{name}\"")), "missing {name}");
    }
    for limit in [ABI_MAX_OPERATION_CODE, ABI_MAX_EVENT_CODE] {
        assert!(ABI_SCHEMA_JSON.contains(&limit.to_string()));
    }
    assert_eq!(fixtures().len(), 8);
    let expected_limits = [
        ("operation-code", ABI_MAX_OPERATION_CODE as u64),
        ("event-code", ABI_MAX_EVENT_CODE as u64),
        ("body-bytes", ABI_MAX_BODY_BYTES as u64),
        ("page-bytes", ABI_MAX_PAGE_BYTES as u64),
        ("message-bytes", ABI_MAX_MESSAGE_BYTES as u64),
        ("pages-per-transfer", ABI_MAX_PAGES_PER_TRANSFER as u64),
        ("transfer-bytes", ABI_MAX_TRANSFER_BYTES as u64),
        ("in-flight-handles", ABI_MAX_IN_FLIGHT_HANDLES as u64),
        ("in-flight-requests", ABI_MAX_IN_FLIGHT_REQUESTS as u64),
        ("handle-generation", u32::MAX as u64),
    ];
    let parsed_limits: Vec<_> = ABI_LIMITS_FIXTURE
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let fields: Vec<_> = line.split('\t').collect();
            (fields[0], fields[1].parse::<u64>().unwrap(), fields[2].parse::<u64>().unwrap())
        })
        .collect();
    assert_eq!(parsed_limits.len(), expected_limits.len());
    for ((name, maximum, plus_one), expected) in parsed_limits.into_iter().zip(expected_limits) {
        assert_eq!((name, maximum, plus_one), (expected.0, expected.1, expected.1 + 1));
    }
}

#[test]
fn empty_single_max_and_max_plus_one_preserve_bounds_and_caller_bytes() {
    let empty = AbiMessage::Request(AbiRequest { operation: operation(1), request_id: AbiRequestId(0), generation: 1, bytes: bytes(Vec::new()) });
    assert_eq!(decode_abi_message(&encode_abi_message(&empty)), Ok(empty));
    let single = AbiPage::try_new(handle(1, 1), 0, vec![7]).unwrap();
    assert_eq!(single.bytes.as_slice(), &[7]);
    let maximum = AbiPage::try_new(handle(1, 1), ABI_MAX_PAGES_PER_TRANSFER - 1, vec![3; ABI_MAX_PAGE_BYTES]).unwrap();
    assert_eq!(maximum.bytes.len(), ABI_MAX_PAGE_BYTES);
    let original = vec![9; ABI_MAX_PAGE_BYTES + 1];
    let rejected = AbiPage::try_new(handle(1, 1), 0, original.clone()).unwrap_err();
    assert_eq!(rejected.code, AbiErrorCode::LimitExceeded);
    assert_eq!(rejected.bytes, original);
    assert_eq!(AbiOperation::try_new(ABI_MAX_OPERATION_CODE).unwrap().get(), ABI_MAX_OPERATION_CODE);
    assert_eq!(AbiOperation::try_new(ABI_MAX_OPERATION_CODE + 1), Err(AbiErrorCode::UnknownOperation));
    let maximum_body = AbiBytes::try_new(vec![5; ABI_MAX_BODY_BYTES]).unwrap();
    assert_eq!(maximum_body.len(), ABI_MAX_BODY_BYTES);
    let maximum_message = AbiMessageBytes::try_new(vec![b'm'; ABI_MAX_MESSAGE_BYTES]).unwrap();
    assert_eq!(maximum_message.as_bytes().len(), ABI_MAX_MESSAGE_BYTES);
    let oversized_message = vec![b'm'; ABI_MAX_MESSAGE_BYTES + 1];
    assert_eq!(AbiMessageBytes::try_new(oversized_message.clone()).unwrap_err().bytes, oversized_message);
}

#[test]
fn native_and_wasm_ledger_is_fixed_little_endian_and_deterministic() {
    let records = [
        AbiMessage::Request(AbiRequest { operation: operation(7), request_id: AbiRequestId(1), generation: 2, bytes: bytes(vec![b'A']) }),
        AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 2, status: AbiStatus::OK, bytes: bytes(vec![b'B']) }),
        AbiMessage::Event(AbiEvent { request_id: AbiRequestId(1), generation: 2, sequence: 3, event: AbiEventCode::try_new(4).unwrap(), status: AbiStatus::OK, bytes: bytes(vec![b'C']) }),
        AbiMessage::Page(AbiPage::try_new(handle(5, 6), 7, vec![b'D']).unwrap()),
        AbiMessage::Page(AbiPage::try_new(handle(5, 6), 0, Vec::new()).unwrap()),
        AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(8), generation: 9 }),
        AbiMessage::Control(AbiControl::Close { handle: handle(5, 6) }),
        AbiMessage::Control(AbiControl::Acknowledge { handle: handle(5, 6), index: 7 }),
    ];
    for ((name, expected), record) in fixtures().into_iter().zip(records) {
        let encoded = encode_abi_message(&record);
        assert_eq!(hex(&encoded), hex(&expected), "{name}");
        assert_eq!(decode_abi_message(&encoded), Ok(record), "{name}");
    }
}

#[test]
fn malformed_tag_length_utf8_and_missing_optional_fail_closed() {
    assert_eq!(decode_abi_message(&[ABI_VERSION, 255]), Err(AbiErrorCode::MalformedTag));
    let mut unknown_operation = fixtures()[0].1.clone();
    unknown_operation[2..4].copy_from_slice(&0u16.to_le_bytes());
    assert_eq!(decode_abi_message(&unknown_operation), Err(AbiErrorCode::UnknownOperation));
    let mut malformed_length = encode_abi_message(&AbiMessage::Request(AbiRequest { operation: operation(1), request_id: AbiRequestId(1), generation: 1, bytes: bytes(vec![1]) }));
    malformed_length[16..20].copy_from_slice(&2u32.to_le_bytes());
    assert_eq!(decode_abi_message(&malformed_length), Err(AbiErrorCode::MalformedLength));
    let mut invalid_utf8 = vec![ABI_VERSION, 2];
    invalid_utf8.extend_from_slice(&1u64.to_le_bytes());
    invalid_utf8.extend_from_slice(&1u32.to_le_bytes());
    invalid_utf8.extend_from_slice(&(AbiStatusCode::Failed as u16).to_le_bytes());
    invalid_utf8.push(1);
    invalid_utf8.extend_from_slice(&(AbiErrorCode::Interrupted as u16).to_le_bytes());
    invalid_utf8.extend_from_slice(&1u16.to_le_bytes());
    invalid_utf8.push(0xff);
    invalid_utf8.extend_from_slice(&0u32.to_le_bytes());
    assert_eq!(decode_abi_message(&invalid_utf8), Err(AbiErrorCode::InvalidUtf8));
    let mut missing_optional = vec![ABI_VERSION, 2];
    missing_optional.extend_from_slice(&1u64.to_le_bytes());
    missing_optional.extend_from_slice(&1u32.to_le_bytes());
    missing_optional.extend_from_slice(&(AbiStatusCode::Ok as u16).to_le_bytes());
    assert_eq!(decode_abi_message(&missing_optional), Err(AbiErrorCode::MissingField));
    let mut zero_handle = fixtures()[3].1.clone();
    zero_handle[2..6].copy_from_slice(&0u32.to_le_bytes());
    assert_eq!(decode_abi_message(&zero_handle), Err(AbiErrorCode::UnknownHandle));
}

#[test]
fn retained_writer_is_credit_deadline_and_interruption_aware() {
    let identity = handle(2, 4);
    let mut writer = AbiPageWriter::new(identity);
    writer.offer(AbiPage::try_new(identity, 0, b"abcd".to_vec()).unwrap()).unwrap();
    assert_eq!(writer.write_step(AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(4) }), Err(AbiErrorCode::Interrupted));
    assert!(writer.bytes().is_empty());
    assert_eq!(writer.write_step(AbiWorkBudget { now_ms: 7, deadline_ms: Some(7), ..AbiWorkBudget::credits(4) }), Err(AbiErrorCode::DeadlineExceeded));
    assert!(writer.bytes().is_empty());
    assert_eq!(writer.write_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::Advanced(2)));
    assert_eq!(writer.write_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::PageComplete(0)));
    assert_eq!(writer.bytes(), b"abcd");
}

#[test]
fn exact_ack_duplicate_ack_and_generation_errors_are_distinct() {
    let identity = handle(3, 9);
    let mut reader = AbiPageReader::try_new(identity, b"page".to_vec()).unwrap();
    assert_eq!(reader.read_step(AbiWorkBudget::credits(4)), Ok(AbiCursorStep::PageComplete(0)));
    assert_eq!(reader.acknowledge(AbiControl::Acknowledge { handle: handle(4, 9), index: 0 }), Err(AbiErrorCode::UnknownHandle));
    assert_eq!(reader.acknowledge(AbiControl::Acknowledge { handle: handle(3, 8), index: 0 }), Err(AbiErrorCode::AbaHandle));
    assert_eq!(reader.acknowledge(AbiControl::Acknowledge { handle: handle(3, 10), index: 0 }), Err(AbiErrorCode::StaleGeneration));
    let ack = AbiControl::Acknowledge { handle: identity, index: 0 };
    assert_eq!(reader.acknowledge(ack), Ok(()));
    assert_eq!(reader.acknowledge(ack), Err(AbiErrorCode::DuplicateAcknowledgement));
    assert_eq!(reader.read_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
}

#[test]
fn handle_table_rejects_unknown_stale_and_aba_reuse() {
    let mut table = AbiHandleTable::new();
    assert_eq!(table.get(handle(7, 1)), Err(AbiErrorCode::UnknownHandle));
    let first = table.open("first").unwrap();
    assert_eq!(table.get(handle(first.slot, first.generation + 1)), Err(AbiErrorCode::StaleGeneration));
    assert_eq!(table.close(first), Ok("first"));
    let second = table.open("second").unwrap();
    assert_eq!(second.slot, first.slot);
    assert_ne!(second.generation, first.generation);
    assert_eq!(table.get(first), Err(AbiErrorCode::AbaHandle));
    assert_eq!(table.get(second), Ok(&"second"));
}

#[test]
fn handle_generation_exhaustion_quarantines_slots_without_aliasing() {
    let mut one = AbiHandleTable::new();
    let first = one.open("near-max").unwrap();
    one.slots[0].generation = u32::MAX - 1;
    let near_max = handle(first.slot, u32::MAX - 1);
    assert_eq!(one.close(near_max), Ok("near-max"));
    let maximum = one.open("maximum").unwrap();
    assert_eq!(maximum, handle(first.slot, u32::MAX));
    assert_eq!(one.close(maximum), Ok("maximum"));
    let replacement = one.open("fresh-slot").unwrap();
    assert_ne!(replacement.slot, maximum.slot);
    assert_eq!(one.get(maximum), Err(AbiErrorCode::UnknownHandle));

    let mut exhausted = AbiHandleTable::new();
    let mut handles = Vec::new();
    for value in 0..ABI_MAX_IN_FLIGHT_HANDLES {
        let opened = exhausted.open(value).unwrap();
        exhausted.slots[value].generation = u32::MAX;
        handles.push(handle(opened.slot, u32::MAX));
    }
    for (value, handle) in handles.into_iter().enumerate() {
        assert_eq!(exhausted.close(handle), Ok(value));
    }
    assert_eq!(exhausted.open(999), Err((AbiErrorCode::GenerationExhausted, 999)));
}

#[test]
fn cancel_before_and_after_seal_are_terminal_and_non_advancing() {
    let identity = handle(1, 1);
    let mut before = AbiPageWriter::new(identity);
    assert_eq!(before.cancel(), AbiCancelOutcome { page: None, admitted_byte_credits: 0, copied_bytes: 0 });
    let rejected = before.offer(AbiPage::try_new(identity, 0, vec![1]).unwrap()).unwrap_err();
    assert_eq!(rejected.code, AbiErrorCode::Cancelled);
    assert_eq!(rejected.bytes, vec![1]);
    let mut after = AbiPageWriter::new(identity);
    after.seal().unwrap();
    assert_eq!(after.cancel(), AbiCancelOutcome { page: None, admitted_byte_credits: 0, copied_bytes: 0 });
    assert!(after.is_sealed() && after.is_cancelled());
    assert_eq!(after.write_step(AbiWorkBudget::credits(1)), Err(AbiErrorCode::Cancelled));

    let mut admitted = AbiPageWriter::new(identity);
    admitted.offer(AbiPage::try_new(identity, 0, b"credits".to_vec()).unwrap()).unwrap();
    assert_eq!(admitted.write_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::Advanced(2)));
    let outcome = admitted.cancel();
    assert_eq!((outcome.admitted_byte_credits, outcome.copied_bytes), (7, 2));
    let handback = outcome.page.expect("admitted page handback");
    assert_eq!((handback.handle, handback.index, handback.bytes.as_slice()), (identity, 0, b"credits".as_slice()));
    assert_eq!(admitted.write_step(AbiWorkBudget::credits(5)), Err(AbiErrorCode::Cancelled));
    assert_eq!(admitted.bytes(), b"cr");
    assert_eq!(admitted.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Advanced(1)));
    assert_eq!(admitted.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
    assert!(admitted.terminal_is_empty());
    assert_eq!(admitted.cancel(), AbiCancelOutcome { page: None, admitted_byte_credits: 0, copied_bytes: 0 });
}

#[test]
fn lost_handle_late_reply_and_duplicate_reply_cannot_cross_generations() {
    let mut handles = AbiHandleTable::new();
    let lost = handles.open(7u8).unwrap();
    assert_eq!(handles.lose(lost), Ok(7));
    assert_eq!(handles.get(lost), Err(AbiErrorCode::UnknownHandle));
    let mut ledger = AbiReplyLedger::new();
    ledger.admit(AbiRequestId(11), 3).unwrap();
    ledger.lose(AbiRequestId(11), 3).unwrap();
    assert_eq!(ledger.accept(&reply(11, 3)), Err(AbiErrorCode::LateReply));
    ledger.admit(AbiRequestId(11), 4).unwrap();
    assert_eq!(ledger.accept(&reply(11, 3)), Err(AbiErrorCode::LateReply));
    assert_eq!(ledger.accept(&reply(11, 4)), Ok(()));
    assert_eq!(ledger.accept(&reply(11, 4)), Err(AbiErrorCode::DuplicateReply));
    ledger.admit(AbiRequestId(267), 1).unwrap();
    assert_eq!(ledger.admit(AbiRequestId(523), 1), Err(AbiErrorCode::Busy));
    ledger.lose(AbiRequestId(267), 1).unwrap();
    ledger.admit(AbiRequestId(523), 1).unwrap();
    assert_eq!(ledger.accept(&reply(267, 1)), Err(AbiErrorCode::LateReply));
}

#[test]
fn reader_preflights_every_rejection_before_allocation_or_copy() {
    let identity = handle(4, 2);
    let mut reader = AbiPageReader::try_new(identity, vec![1; ABI_MAX_PAGE_BYTES]).unwrap();
    let rejected = [
        (AbiWorkBudget::credits(0), AbiErrorCode::NoCredit),
        (AbiWorkBudget { cancelled: true, ..AbiWorkBudget::credits(1) }, AbiErrorCode::Cancelled),
        (AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }, AbiErrorCode::Interrupted),
        (AbiWorkBudget { now_ms: 9, deadline_ms: Some(9), ..AbiWorkBudget::credits(1) }, AbiErrorCode::DeadlineExceeded),
    ];
    for (budget, code) in rejected {
        assert_eq!(reader.read_step(budget), Err(code));
        assert_eq!((reader.source_cursor, reader.target_len, reader.staging.len(), reader.staging.capacity()), (0, 0, 0, 0));
    }
    assert_eq!(reader.read_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Advanced(1)));
    assert_eq!((reader.source_cursor, reader.target_len, reader.staging.len()), (1, ABI_MAX_PAGE_BYTES, 1));
}

#[test]
fn page_and_transfer_max_plus_one_return_exact_allocations() {
    let identity = handle(1, 2);
    let mut writer = AbiPageWriter::new(identity);
    let page = AbiPage::try_new(identity, 1, vec![4]).unwrap();
    let rejected = writer.offer(page).unwrap_err();
    assert_eq!(rejected.code, AbiErrorCode::OutOfOrderPage);
    assert_eq!((rejected.handle, rejected.index, rejected.bytes), (identity, 1, vec![4]));
    let original = vec![8; ABI_MAX_TRANSFER_BYTES + 1];
    let rejected = AbiPageReader::try_new(identity, original.clone()).err().unwrap();
    assert_eq!(rejected.bytes, original);

    let mut page_count = AbiPageWriter::new(identity);
    for index in 0..ABI_MAX_PAGES_PER_TRANSFER {
        page_count.offer(AbiPage::try_new(identity, index, Vec::new()).unwrap()).unwrap();
        assert_eq!(page_count.write_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::PageComplete(index)));
    }
    let rejected = page_count.offer(AbiPage { handle: identity, index: ABI_MAX_PAGES_PER_TRANSFER, bytes: AbiPageBytes::default() }).unwrap_err();
    assert_eq!(rejected.code, AbiErrorCode::LimitExceeded);
    assert!(rejected.bytes.is_empty());
}

#[test]
fn interrupted_port_callback_returns_the_exact_owned_message() {
    struct FixturePort;

    impl AbiPort for FixturePort {
        fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), AbiPortRejection> {
            if let Err(code) = budget.permit(1) { Err(AbiPortRejection { code, message }) } else { Ok(()) }
        }

        fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
            budget.permit(1).map(|_| AbiPortPoll::Pending)
        }
    }

    let message = AbiMessage::Request(AbiRequest { operation: operation(1), request_id: AbiRequestId(19), generation: 3, bytes: bytes(vec![1, 2, 3]) });
    let rejection = FixturePort.try_send(message.clone(), AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }).unwrap_err();
    assert_eq!(rejection, AbiPortRejection { code: AbiErrorCode::Interrupted, message });
}

#[test]
fn interrupted_close_retains_state_and_terminal_empty_is_idempotent() {
    let identity = handle(5, 7);
    let mut writer = AbiPageWriter::new(identity);
    writer.offer(AbiPage::try_new(identity, 0, b"close".to_vec()).unwrap()).unwrap();
    writer.write_step(AbiWorkBudget::credits(5)).unwrap();
    assert_eq!(writer.close_step(AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(5) }), Err(AbiErrorCode::Interrupted));
    assert_eq!(writer.bytes(), b"close");
    assert_eq!(writer.close_step(AbiWorkBudget::credits(2)), Ok(AbiCursorStep::Advanced(2)));
    assert_eq!(writer.close_step(AbiWorkBudget::credits(3)), Ok(AbiCursorStep::Complete));
    assert!(writer.terminal_is_empty());
    assert_eq!(writer.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
    assert!(writer.terminal_is_empty());

    let mut reader = AbiPageReader::try_new(identity, b"reader".to_vec()).unwrap();
    reader.read_step(AbiWorkBudget::credits(2)).unwrap();
    assert_eq!(reader.close_step(AbiWorkBudget { cancelled: true, ..AbiWorkBudget::credits(8) }), Err(AbiErrorCode::Cancelled));
    while !reader.terminal_is_empty() {
        reader.close_step(AbiWorkBudget::credits(2)).unwrap();
    }
    assert_eq!(reader.close_step(AbiWorkBudget::credits(1)), Ok(AbiCursorStep::Complete));
}
