//! 🔎️ Neutral reconcile envelopes; the caller owns transport decoding and recovered payloads.

pub const JOB_RECONCILE_REQUEST_SCHEMA_V1: &str = "semio.framework.job-reconcile/v1";
pub const JOB_RECONCILE_RESULT_SCHEMA_V1: &str = "semio.framework.job-reconcile-result/v1";
pub const JOB_RECONCILE_REQUEST_MAX_BYTES: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JobReconcileRequestV1 {
    pub schema: &'static str,
    pub version: u32,
    pub request_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JobReconcileResultV1<T> {
    Missing { request_id: String },
    Found { request_id: String, job: T },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidJobReconcileV1;

/// 📨️ Validates request fields after the owning transport has decoded its closed object.
pub fn parse_job_reconcile_request_v1(schema: &str, version: u32, request_id: &str) -> Result<JobReconcileRequestV1, InvalidJobReconcileV1> {
    if schema != JOB_RECONCILE_REQUEST_SCHEMA_V1 || version != 1 || request_id.len() != 32 || !request_id.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err(InvalidJobReconcileV1);
    }
    Ok(JobReconcileRequestV1 { schema: JOB_RECONCILE_REQUEST_SCHEMA_V1, version: 1, request_id: request_id.to_owned() })
}

/// 📦️ A missing result cannot hold a job; recovered payloads are owned by the caller.
pub fn parse_job_reconcile_result_v1<T>(schema: &str, version: u32, request_id: &str, found: bool, job: Option<T>) -> Result<JobReconcileResultV1<T>, InvalidJobReconcileV1> {
    parse_job_reconcile_request_v1(JOB_RECONCILE_REQUEST_SCHEMA_V1, version, request_id)?;
    if schema != JOB_RECONCILE_RESULT_SCHEMA_V1 { return Err(InvalidJobReconcileV1); }
    match (found, job) {
        (false, None) => Ok(JobReconcileResultV1::Missing { request_id: request_id.to_owned() }),
        (true, Some(job)) => Ok(JobReconcileResultV1::Found { request_id: request_id.to_owned(), job }),
        _ => Err(InvalidJobReconcileV1),
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️contract/🦀️.rs"]
mod tests;
