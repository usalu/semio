//! 🧫️ Original schema cursor law builders shared by native conformance owners.
use crate::*;
use semio_framework_value::serde_json;
pub const OWNED_SCHEMA_TEST_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: 1, key: "schema", required: true },
    OwnedSchemaFieldSpec { id: 2, key: "id", required: true },
    OwnedSchemaFieldSpec { id: 3, key: "quote\"", required: false },
    OwnedSchemaFieldSpec { id: 4, key: "slash\\", required: false },
    OwnedSchemaFieldSpec { id: 5, key: "control\u{0001}", required: false },
    OwnedSchemaFieldSpec { id: 6, key: "asciiA", required: false },
    OwnedSchemaFieldSpec { id: 7, key: "escaped/\u{0008}\u{000c}\n\r\t", required: false },
];

pub fn owned_schema_test_cursor(chunks: &[&[u8]]) -> OwnedSchemaRecordCursor {
    let byte_count = chunks.iter().map(|chunk| chunk.len()).sum();
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: byte_count }).expect("fixed schema credits");
    for chunk in chunks {
        let page = OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded test page");
        pages.admit_page(page).unwrap_or_else(|_| panic!("pre-admitted test page"));
    }
    pages.seal().expect("seal exact test extent");
    let tokens = OwnedSchemaTokenCursor::try_new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), pages).unwrap_or_else(|_| panic!("sealed token source"));
    OwnedSchemaRecordCursor::try_new(OwnedSchemaRecordSpec { fields: OWNED_SCHEMA_TEST_FIELDS }, tokens).unwrap_or_else(|_| panic!("valid fixed test schema"))
}

pub fn drive_owned_schema(cursor: &mut OwnedSchemaRecordCursor, fuel: u64) -> Result<Vec<(u16, OwnedSchemaTokenKind, bool)>, OwnedSchemaDecodeDiagnostic> {
    let mut events = Vec::new();
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut receipt = RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(fuel, u64::MAX, schema_test_close_grant()),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
            &mut receipt,
        );
        match cursor.step(&mut context) {
            OwnedSchemaRecordStep::Pending => {}
            OwnedSchemaRecordStep::FieldToken { field_id, token, terminal } => events.push((field_id, token.kind, terminal)),
            OwnedSchemaRecordStep::Complete => return Ok(events),
            OwnedSchemaRecordStep::Fault(diagnostic) => return Err(diagnostic),
            OwnedSchemaRecordStep::Cancelled => panic!("live schema cursor cancelled unexpectedly"),
        }
    }
    panic!("schema cursor failed to terminate under repeated fixed fuel")
}

pub fn schema_test_close_grant() -> RetainedCloneGrant {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).expect("plain decode close policy");
    let policy=&fixture["decodeCloseGrant"];
    RetainedCloneGrant { maximum_items:policy["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap() as usize }
}

