use crate::*;
use crate::testing::*;
#[test]
fn owned_schema_pages_reject_capacity_plus_one_with_the_exact_page_untouched() {
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: OWNED_SCHEMA_DECODE_PAGE_BYTES }).expect("one page credits");
    let first = OwnedSchemaDecodePage::try_from_slice(&[b'a'; OWNED_SCHEMA_DECODE_PAGE_BYTES]).expect("maximum page");
    pages.admit_page(first).expect("maximum page admitted");
    let rejected = OwnedSchemaDecodePage::try_from_slice(b"tail").expect("bounded rejected page");
    let (fault, rejected) = pages.admit_page(rejected).expect_err("capacity plus one rejected before adoption");
    assert_eq!(fault, OwnedSchemaDecodeAdmissionFault::PageCapacity);
    assert_eq!(rejected.len(), 4, "the exact rejected owner is returned untouched");
    assert_eq!(pages.page_count(), 1);

    let mut byte_limited = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).expect("three-byte credits");
    let rejected = OwnedSchemaDecodePage::try_from_slice(b"four").expect("bounded page owner");
    let (fault, rejected) = byte_limited.admit_page(rejected).expect_err("aggregate byte plus one rejected before adoption");
    assert_eq!(fault, OwnedSchemaDecodeAdmissionFault::ByteCapacity);
    assert_eq!(rejected.len(), 4);
    assert!(!byte_limited.terminal_is_empty(), "the admitted directory backing is retained until its funded release");
    assert!(matches!(byte_limited.close_backing_step(schema_test_close_grant()), Ok(RetainedCloneStep::Complete(_))));
    assert!(byte_limited.terminal_is_empty());
    while let Some(page) = pages.close_take_page() { drop(page); }
    assert!(matches!(pages.close_backing_step(schema_test_close_grant()), Ok(RetainedCloneStep::Complete(_))));
    assert!(pages.terminal_is_empty());
}

#[test]
fn owned_schema_record_resumes_across_a_partial_terminal_page_and_preserves_field_order() {
    let mut first = br#"{"schema":""#.to_vec();
    first.resize(OWNED_SCHEMA_DECODE_PAGE_BYTES, b'a');
    let second = br#"","id":"document"}"#;
    let mut cursor = owned_schema_test_cursor(&[&first, second]);
    let events = drive_owned_schema(&mut cursor, 7).expect("valid cross-page record");
    assert_eq!(events.iter().filter(|(field, _, terminal)| *field == 1 && *terminal).count(), 1);
    assert_eq!(events.iter().filter(|(field, _, terminal)| *field == 2 && *terminal).count(), 1);
    assert_eq!(events.first().map(|event| event.0), Some(1));
    assert_eq!(events.last().map(|event| event.0), Some(2));
}

#[test]
fn owned_schema_record_reports_exact_invalid_utf8_unknown_and_duplicate_diagnostics() {
    let mut first = br#"{"schema":""#.to_vec();
    first.resize(OWNED_SCHEMA_DECODE_PAGE_BYTES - 1, b'a');
    first.push(0xe2);
    let invalid_tail = br#"!","id":"document"}"#;
    let invalid = drive_owned_schema(&mut owned_schema_test_cursor(&[&first, invalid_tail]), 11).expect_err("invalid continuation faults");
    assert_eq!(invalid.code, "schema-json.invalid-utf8");
    assert_eq!(invalid.offset, OWNED_SCHEMA_DECODE_PAGE_BYTES as u64);

    let unknown = drive_owned_schema(&mut owned_schema_test_cursor(&[br#"{"schema":"v1","unexpected":1,"id":"document"}"#]), 64).expect_err("unknown field faults");
    assert_eq!(unknown.code, "schema-json.unknown-field");
    assert_eq!(unknown.path.as_str(), "$");

    let duplicate = drive_owned_schema(&mut owned_schema_test_cursor(&[br#"{"schema":"v1","schema":"v2","id":"document"}"#]), 64).expect_err("duplicate field faults");
    assert_eq!(duplicate.code, "schema-json.duplicate-field");
    assert_eq!(duplicate.path.as_str(), "$.schema");

    let truncated = drive_owned_schema(&mut owned_schema_test_cursor(&[br#"{"schema":"unterminated"#]), 9).expect_err("truncated token faults");
    assert_eq!(truncated.code, "schema-json.truncated-token");
}

#[test]
fn owned_schema_record_rejects_stale_operation_and_generation_before_consuming_input() {
    let mut cursor = owned_schema_test_cursor(&[br#"{"schema":"v1","id":"document"}"#]);
    let mut preview_sequence = 0;
    let mut original_retained_progress_0=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(2),
        semio_framework_job::StepBudget::new(64, u64::MAX, schema_test_close_grant()),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut preview_sequence,
     &mut original_retained_progress_0);
    let OwnedSchemaRecordStep::Fault(diagnostic) = cursor.step(&mut context) else { panic!("stale generation must fault before syntax work") };
    assert_eq!(diagnostic.code, "schema-json.stale-authority");
    assert_eq!(diagnostic.offset, 0);
    let grant=schema_test_close_grant();
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(grant).unwrap());
    assert_eq!((allocated,released),(0,0));assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress().released_bytes,0);assert!(step.progress().fits(grant));assert!(!cursor.terminal_is_empty());
    let backing=cursor.next_close_release_byte_demand().unwrap();let pointer=cursor.tokens.pages.slots.as_ptr();
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(RetainedCloneGrant { maximum_release_bytes:backing-1,..grant }).unwrap());
    assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(cursor.tokens.pages.slots.as_ptr(),pointer);
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(grant).unwrap());
    assert_eq!((allocated,released),(0,backing));assert_eq!(step.progress().released_bytes,backing);assert!(step.progress().fits(grant));assert!(matches!(step,RetainedCloneStep::Complete(_)));assert!(cursor.terminal_is_empty());
}

fn next_owned_schema_field_token(cursor: &mut OwnedSchemaRecordCursor, expected_field: u16) -> OwnedSchemaToken {
    let mut preview_sequence = 0;
    for _ in 0..1_000 {
        let mut original_retained_progress_1=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(32, u64::MAX, schema_test_close_grant()),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
         &mut original_retained_progress_1);
        match cursor.step(&mut context) {
            OwnedSchemaRecordStep::Pending => {}
            OwnedSchemaRecordStep::FieldToken { field_id, token, terminal: true } if field_id == expected_field => return token,
            OwnedSchemaRecordStep::FieldToken { .. } => {}
            other => panic!("expected field token, got {other:?}"),
        }
    }
    panic!("schema field token did not arrive under repeated grants")
}

#[test]
fn owned_schema_string_unescapes_into_fixed_storage_and_rejects_semantic_byte_plus_one() {
    let mut cursor = owned_schema_test_cursor(&[br#"{"schema":"line\n\uD83D\uDE00","id":"document"}"#]);
    let token = next_owned_schema_field_token(&mut cursor, 1);
    let mut string = OwnedSchemaStringAuthority::<32>::try_new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), token, OwnedSchemaPath::field("schema").expect("bounded field path")).expect("schema token is a string");
    let mut preview_sequence = 0;
    for _ in 0..100 {
        let mut original_retained_progress_2=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(3, u64::MAX, schema_test_close_grant()),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
         &mut original_retained_progress_2);
        match string.step(&cursor, &mut context) {
            OwnedSchemaStringStep::Pending => {}
            OwnedSchemaStringStep::Complete => break,
            other => panic!("bounded escaped string failed: {other:?}"),
        }
    }
    assert_eq!(string.as_str(), Some("line\n😀"));
    assert_eq!(string.take_string().as_deref(), Some("line\n😀"));
    assert!(string.take_string().is_none(), "semantic string publishes exactly once");
    assert!(string.terminal_is_empty());

    let mut cursor = owned_schema_test_cursor(&[br#"{"schema":"four","id":"document"}"#]);
    let token = next_owned_schema_field_token(&mut cursor, 1);
    let mut limited = OwnedSchemaStringAuthority::<3>::try_new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), token, OwnedSchemaPath::field("schema").expect("bounded field path")).expect("schema token is a string");
    let mut preview_sequence = 0;
    let mut original_retained_progress_3=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(32, u64::MAX, schema_test_close_grant()),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut preview_sequence,
     &mut original_retained_progress_3);
    let OwnedSchemaStringStep::Fault(diagnostic) = limited.step(&cursor, &mut context) else { panic!("semantic byte plus one must fault") };
    assert_eq!(diagnostic.code, "schema-json.string-byte-capacity");
    assert_eq!(diagnostic.path.as_str(), "$.schema");
    assert!(limited.terminal_is_empty(), "fixed inline rejection has no deep owner to retire");
}

#[test]
fn cancelled_owned_schema_cursor_retires_one_real_page_per_close_grant() {
    let first = [b' '; OWNED_SCHEMA_DECODE_PAGE_BYTES];
    let mut cursor = owned_schema_test_cursor(&[&first, br#"{"schema":"v1","id":"document"}"#]);
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut preview_sequence = 0;
    let mut original_retained_progress_4=semio_framework_value::RetainedCloneProgress::default();
        let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(2), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX, schema_test_close_grant()), cancel, semio_framework_job::default_now_us, &mut preview_sequence, &mut original_retained_progress_4);
    assert_eq!(cursor.step(&mut context), OwnedSchemaRecordStep::Cancelled);
    let grant=schema_test_close_grant();let pointer=cursor.tokens.pages.slots.as_ptr();
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(RetainedCloneGrant { maximum_copy_bytes:0,..grant }).unwrap());assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(cursor.tokens.pages.slots.as_ptr(),pointer);
    for _ in 0..2 {
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(grant).unwrap());
        assert_eq!((allocated,released),(0,0));assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress().released_bytes,0);assert!(step.progress().fits(grant));assert_eq!(cursor.tokens.pages.slots.as_ptr(),pointer);
    }
    assert!(!cursor.terminal_is_empty());let backing=cursor.next_close_release_byte_demand().unwrap();assert_eq!(backing,2*std::mem::size_of::<OwnedSchemaDecodePage>());assert_eq!(std::mem::size_of::<OwnedSchemaDecodePage>(),OWNED_SCHEMA_DECODE_PAGE_BYTES+std::mem::size_of::<u16>());
    for denied in [RetainedCloneGrant { maximum_items:0,..grant },RetainedCloneGrant { maximum_release_bytes:backing-1,..grant },RetainedCloneGrant { maximum_depth:0,..grant }] {
        let(result,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(denied));assert_eq!((allocated,released),(0,0));
        if denied.maximum_depth==0 { assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit); } else { assert_eq!(result.unwrap().progress(),RetainedCloneProgress::default()); }
        assert!(!cursor.terminal_is_empty());assert_eq!(cursor.tokens.pages.slots.as_ptr(),pointer);
    }
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.close_step(grant).unwrap());assert_eq!((allocated,released),(0,backing));assert_eq!(step.progress().released_bytes,backing);assert!(step.progress().fits(grant));assert!(matches!(step,RetainedCloneStep::Complete(_)));assert!(cursor.terminal_is_empty());
    eprintln!("[DEBUG] schema cancel preserves original directory pointer under denied work/release/depth; inline page work releases0 and final directory releases exact{backing} bytes");
}

