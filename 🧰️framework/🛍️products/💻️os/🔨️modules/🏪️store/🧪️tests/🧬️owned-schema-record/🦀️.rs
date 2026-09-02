//! 🧪️ Strict semantic-key and delimiter laws for owned Store schema cursors.

use super::tests::{drive_owned_schema, owned_schema_test_cursor, OWNED_SCHEMA_TEST_FIELDS};
use super::*;

//#region 🧪️Fixture
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).expect("owned schema record vectors")
}

fn source_case<'a>(row: &'a serde_json::Value) -> &'a str {
    row["source"].as_str().expect("source vector")
}

fn page_case(row: &serde_json::Value) -> [Vec<u8>; 2] {
    assert_eq!(row["pageBytes"].as_u64(), Some(OWNED_SCHEMA_DECODE_PAGE_BYTES as u64));
    let chunks = row["chunks"].as_array().expect("page chunks");
    assert_eq!(chunks.len(), 2);
    let first = chunks[0].as_str().expect("first page suffix").as_bytes();
    let second = chunks[1].as_str().expect("terminal page").as_bytes();
    assert!(first.len() <= OWNED_SCHEMA_DECODE_PAGE_BYTES && second.len() <= OWNED_SCHEMA_DECODE_PAGE_BYTES);
    let mut first_page = vec![b' '; OWNED_SCHEMA_DECODE_PAGE_BYTES - first.len()];
    first_page.extend_from_slice(first);
    [first_page, second.to_vec()]
}

fn close_owned_schema(cursor: &mut OwnedSchemaRecordCursor) {
    for _ in 0..1_000 {
        match cursor.close_step(1) {
            SnapshotRetirementStep::Pending { .. } | SnapshotRetirementStep::Blocked => {}
            SnapshotRetirementStep::Complete => {
                assert!(cursor.terminal_is_empty());
                return;
            }
        }
    }
    panic!("owned schema cursor did not close its admitted pages")
}

fn drive_owned_schema_nested(cursor: &mut OwnedSchemaRecordCursor, nested: &mut OwnedSchemaNestedRecordCursor, fuel: u64) -> Result<(), OwnedSchemaDecodeDiagnostic> {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(fuel, u64::MAX),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
        );
        match cursor.step(&mut context) {
            OwnedSchemaRecordStep::Pending | OwnedSchemaRecordStep::FieldToken { field_id: 2, .. } => {}
            OwnedSchemaRecordStep::FieldToken { field_id: 1, token, .. } => match nested.accept(token, cursor) {
                OwnedSchemaNestedRecordStep::Pending | OwnedSchemaNestedRecordStep::FieldToken { .. } | OwnedSchemaNestedRecordStep::Complete => {}
                OwnedSchemaNestedRecordStep::Fault(diagnostic) => return Err(diagnostic),
            },
            OwnedSchemaRecordStep::FieldToken { field_id, .. } => panic!("unexpected fixed outer field {field_id}"),
            OwnedSchemaRecordStep::Complete => return if nested.terminal_is_complete() { Ok(()) } else { panic!("outer record completed before nested record") },
            OwnedSchemaRecordStep::Fault(diagnostic) => return Err(diagnostic),
            OwnedSchemaRecordStep::Cancelled => panic!("live schema cursor cancelled unexpectedly"),
        }
    }
    panic!("nested schema cursor failed to terminate under repeated fixed fuel")
}

fn close_result(cursor: &mut OwnedSchemaRecordCursor, result: Result<Vec<(u16, OwnedSchemaTokenKind, bool)>, OwnedSchemaDecodeDiagnostic>) -> Result<Vec<(u16, OwnedSchemaTokenKind, bool)>, OwnedSchemaDecodeDiagnostic> {
    close_owned_schema(cursor);
    result
}

fn close_nested_result(cursor: &mut OwnedSchemaRecordCursor, result: Result<(), OwnedSchemaDecodeDiagnostic>) -> Result<(), OwnedSchemaDecodeDiagnostic> {
    close_owned_schema(cursor);
    result
}
//#endregion 🧪️Fixture

//#region 🧪️Outer
#[test]
fn owned_schema_record_accepts_every_valid_semantic_key_and_small_page_fixture() {
    let fixture = fixture();
    let mut failures = Vec::new();
    for row in fixture["outer"]["valid"].as_array().expect("outer valid vectors") {
        let mut cursor = owned_schema_test_cursor(&[source_case(row).as_bytes()]);
        let result = drive_owned_schema(&mut cursor, 3);
        if let Err(diagnostic) = close_result(&mut cursor, result) {
            failures.push(format!("{}: {}", row["name"], diagnostic.code));
        }
    }
    for row in fixture["outer"]["pages"].as_array().expect("outer page vectors") {
        let pages = page_case(row);
        let chunks = pages.iter().map(Vec::as_slice).collect::<Vec<_>>();
        let mut cursor = owned_schema_test_cursor(&chunks);
        let result = drive_owned_schema(&mut cursor, 3);
        if let Err(diagnostic) = close_result(&mut cursor, result) {
            failures.push(format!("{}: {}", row["name"], diagnostic.code));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn owned_schema_record_rejects_every_invalid_fixture_with_its_native_diagnostic_and_retires_pages() {
    let fixture = fixture();
    let mut failures = Vec::new();
    for row in fixture["outer"]["invalid"].as_array().expect("outer invalid vectors") {
        let mut cursor = owned_schema_test_cursor(&[source_case(row).as_bytes()]);
        let result = drive_owned_schema(&mut cursor, 7);
        let expected = row["nativeCode"].as_str().expect("native code");
        match close_result(&mut cursor, result) {
            Err(diagnostic) if diagnostic.code == expected => {}
            Err(diagnostic) => failures.push(format!("{}: expected {expected}, actual {}", row["name"], diagnostic.code)),
            Ok(_) => failures.push(format!("{}: expected {expected}, record completed", row["name"])),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn owned_schema_record_cancellation_fixture_retires_every_owned_page() {
    let fixture = fixture();
    for row in fixture["outer"]["cancellation"].as_array().expect("cancellation vectors") {
        let pages = page_case(row);
        let chunks = pages.iter().map(Vec::as_slice).collect::<Vec<_>>();
        let mut cursor = owned_schema_test_cursor(&chunks);
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut preview_sequence = 0;
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
        let result = cursor.step(&mut context);
        close_owned_schema(&mut cursor);
        assert_eq!(result, OwnedSchemaRecordStep::Cancelled, "{}", row["name"]);
    }
}
//#endregion 🧪️Outer

//#region 🧪️Nested
#[test]
fn owned_schema_nested_record_accepts_and_rejects_every_fixture_with_the_same_semantic_key_rules() {
    let fixture = fixture();
    let mut failures = Vec::new();
    let spec = OwnedSchemaRecordSpec { fields: OWNED_SCHEMA_TEST_FIELDS };
    for row in fixture["nested"]["valid"].as_array().expect("nested valid vectors") {
        let mut cursor = owned_schema_test_cursor(&[source_case(row).as_bytes()]);
        let mut nested = OwnedSchemaNestedRecordCursor::try_new(spec).expect("fixed nested schema");
        let result = drive_owned_schema_nested(&mut cursor, &mut nested, 3);
        if let Err(diagnostic) = close_nested_result(&mut cursor, result) {
            failures.push(format!("{}: {}", row["name"], diagnostic.code));
        }
    }
    for row in fixture["nested"]["invalid"].as_array().expect("nested invalid vectors") {
        let mut cursor = owned_schema_test_cursor(&[source_case(row).as_bytes()]);
        let mut nested = OwnedSchemaNestedRecordCursor::try_new(spec).expect("fixed nested schema");
        let result = drive_owned_schema_nested(&mut cursor, &mut nested, 7);
        let expected = row["nativeCode"].as_str().expect("native code");
        match close_nested_result(&mut cursor, result) {
            Err(diagnostic) if diagnostic.code == expected => {}
            Err(diagnostic) => failures.push(format!("{}: expected {expected}, actual {}", row["name"], diagnostic.code)),
            Ok(()) => failures.push(format!("{}: expected {expected}, record completed", row["name"])),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
//#endregion 🧪️Nested
