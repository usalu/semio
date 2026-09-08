use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Barrier, OnceLock};

#[test]
fn identity_is_non_secret_and_actor_is_server_subject_derived() {
    let identity = Identity { user_id: "u-amara".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://127.0.0.1:8787".to_string(), issued_at_ms: 0 };
    assert_eq!(actor_id(&identity, "sess-1"), "user:u-amara#sess-1");
    assert!(!crate::os_pack::json::to_json_string(&identity).contains("token"));
}

#[test]
fn credential_claim_is_single_read_class_bound_and_terminal_on_failure() {
    let claimed = Arc::new(OnceLock::new());
    let starts = Arc::new(Barrier::new(3));
    let reads = Arc::new(AtomicUsize::new(0));
    let mut callers = Vec::new();
    for _ in 0..2 {
        let claimed = claimed.clone();
        let starts = starts.clone();
        let reads = reads.clone();
        callers.push(std::thread::spawn(move || {
            starts.wait();
            claim_local_hub_credential_with(&claimed, "native", |_| {
                reads.fetch_add(1, Ordering::SeqCst);
                Ok(LocalHubCredential::test("http://127.0.0.1:8787", &format!("session.v1.{}.{:064}", "a".repeat(32), 1)))
            })
        }));
    }
    starts.wait();
    let first = callers.remove(0).join().unwrap().unwrap();
    let second = callers.remove(0).join().unwrap().unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    assert_eq!(reads.load(Ordering::SeqCst), 1);
    assert!(claim_local_hub_credential_with(&claimed, "mcp", |_| unreachable!()).is_err());

    let failed = OnceLock::new();
    let failed_reads = AtomicUsize::new(0);
    for _ in 0..2 {
        assert!(claim_local_hub_credential_with(&failed, "native", |_| {
            failed_reads.fetch_add(1, Ordering::SeqCst);
            Err(super::super::client::DirectoryClientError::Unauthorized)
        })
        .is_err());
    }
    assert_eq!(failed_reads.load(Ordering::SeqCst), 1);
}
