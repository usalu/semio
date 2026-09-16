//! 🧪️ Retained numerical payload projection for native contract tests.

use semio_framework_job::{RetainedJobPayload, StepOutcome, JOB_PAYLOAD_PAGE_BYTES};

/// 📖️ Projects each payload page and returns every admitted page before comparing native fixtures.
pub(crate) fn payload_bytes(mut payload: RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    let mut reader = payload.reader();
    while let Some(page) = reader.read_page(1, JOB_PAYLOAD_PAGE_BYTES) {
        bytes.extend_from_slice(page);
    }
    assert!(reader.terminal_is_empty(), "fixture payload must be fully readable");
    while !payload.terminal_is_empty() {
        payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    bytes
}

/// 🧹️ Releases every retained page an outcome still owns so a test may match on its variant and
/// drop it — `RetainedJobPayload` refuses an ordinary drop while it holds page backing.
pub(crate) fn close_outcome(mut outcome: StepOutcome) -> StepOutcome {
    while !outcome.terminal_is_empty() {
        outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
    outcome
}
