//! 🧵️ Retains an asynchronous inference worker's terminal result in the public job journal.
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_wire::job_registry::JobRegistry;
use std::panic::{catch_unwind, AssertUnwindSafe};

pub(super) fn run(registry: &JobRegistry, id: &str, work: impl FnOnce()) {
    if catch_unwind(AssertUnwindSafe(work)).is_err() {
        registry.fail(id, GatewayError::new(GatewayErrorCode::Internal, "the inference job thread ended without returning its result"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asynchronous_worker_exit_settles_the_original_job_once() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/worker.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let registry = JobRegistry::new();
            let id = registry.begin("inference.submit");
            match case["before"].as_str().unwrap() {
                "RUNNING" => assert!(registry.report_progress(&id, 0.35, None)),
                "SUCCEEDED" => assert!(registry.succeed(&id, serde_json::json!("retained"))),
                "AWAITING_APPROVAL" => assert!(registry.await_approval(&id, serde_json::json!("proposal"))),
                "CANCELLED" => assert!(registry.mark_cancelled(&id)),
                _ => unreachable!(),
            }
            let count = registry.events(&id,0,64).unwrap().events.len();
            std::thread::scope(|scope| {
                let worker = scope.spawn(|| run(&registry,&id,|| {
                    if case["unwind"] == true { panic!("fixture worker terminated") }
                }));
                let _ = worker.join();
            });
            let snapshot = registry.snapshot(&id).unwrap().to_json();
            assert_eq!(snapshot["jobId"],id);
            assert_eq!(snapshot["status"],case["expected"],"{}",case["name"]);
            let failed = case["expected"]=="FAILED";
            assert_eq!(registry.events(&id,0,64).unwrap().events.len(),count+usize::from(failed));
            if failed { assert_eq!(snapshot["error"]["code"],"INTERNAL"); }
            if case["before"]=="SUCCEEDED" { assert_eq!(snapshot["result"],"retained"); }
        }
    }
}
