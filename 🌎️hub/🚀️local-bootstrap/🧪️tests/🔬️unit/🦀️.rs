
use super::*;
use serde_json::Value;
use std::sync::atomic::{AtomicI64, AtomicU8};

struct CancelAtCommit {
    cancelled: AtomicBool,
    now_ms: i64,
}

struct AdmissionClock {
    now_ms: AtomicI64,
    admitted_units: AtomicU8,
    cancelled: AtomicBool,
}

impl IdentityVerificationControl for AdmissionClock {
    fn now_ms(&self) -> i64 {
        self.now_ms.load(Ordering::Acquire)
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn report(&self, progress: crate::directory::IdentityVerificationProgress) {
        self.admitted_units.store(progress.completed_units, Ordering::Release);
    }
}

impl IdentityVerificationControl for CancelAtCommit {
    fn now_ms(&self) -> i64 {
        self.now_ms
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn report(&self, progress: crate::directory::IdentityVerificationProgress) {
        if progress.completed_units == progress.total_units {
            self.cancelled.store(true, Ordering::Release);
        }
    }
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../🧪️fixtures/🚇️pipe-v1/🔣️.json")).expect("fixture")
}

fn admission_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧪️fixtures/⏳️idle-admission-v1/🔣️.json")).expect("admission fixture")
}

#[tokio::test]
async fn local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded() {
    let fixture = admission_fixture();
    let exchange_deadline_ms = fixture["exchangeDeadlineMs"].as_i64().expect("exchange deadline");
    let idle_before_admission_ms = fixture["idleBeforeAdmissionMs"].as_i64().expect("idle duration");
    assert_eq!(exchange_deadline_ms, LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS);
    assert!(idle_before_admission_ms > exchange_deadline_ms);
    let frame = decode_hex_bytes(fixture["frameHex"].as_str().expect("frame"));
    let payload = decode_hex_bytes(fixture["payloadHex"].as_str().expect("payload"));

    let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(0), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
    let (mut reader, mut writer) = tokio::io::duplex(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX);
    let read_clock = clock.clone();
    let read = tokio::spawn(async move { read_admitted_frame(&mut reader, read_clock.as_ref()).await });
    tokio::task::yield_now().await;
    clock.now_ms.store(idle_before_admission_ms, Ordering::Release);
    writer.write_all(&frame).await.expect("write admitted frame");
    let (actual, deadline_ms) = read.await.expect("idle reader task").expect("idle reader result").expect("admitted frame");
    assert_eq!(&*actual, payload.as_ref());
    assert_eq!(deadline_ms, idle_before_admission_ms + exchange_deadline_ms);
    assert_eq!(clock.admitted_units.load(Ordering::Acquire), 2);

    let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(0), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
    let (mut reader, mut writer) = tokio::io::duplex(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX);
    let read_clock = clock.clone();
    let read = tokio::spawn(async move { read_admitted_frame(&mut reader, read_clock.as_ref()).await });
    writer.write_all(&frame[..1]).await.expect("write admission byte");
    tokio::time::timeout(std::time::Duration::from_secs(1), async {
        while clock.admitted_units.load(Ordering::Acquire) == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("admission progress");
    clock.now_ms.store(exchange_deadline_ms + 1, Ordering::Release);
    writer.write_all(&frame[1..4]).await.expect("write late prefix");
    let error = read.await.expect("bounded reader task").expect_err("late admitted frame");
    assert!(matches!(error, DirectoryError::Conflict(message) if message.contains("deadline")));

    let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(0), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
    let (mut reader, _writer) = tokio::io::duplex(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX);
    let read_clock = clock.clone();
    let read = tokio::spawn(async move { read_admitted_frame(&mut reader, read_clock.as_ref()).await });
    clock.cancelled.store(true, Ordering::Release);
    let error = tokio::time::timeout(std::time::Duration::from_secs(1), read).await.expect("idle cancellation deadline").expect("idle cancellation task").expect_err("idle cancellation result");
    assert!(matches!(error, DirectoryError::Conflict(message) if message.contains("cancellation")));
}

fn decode_hex_bytes(encoded: &str) -> Box<[u8]> {
    assert_eq!(encoded.len() % 2, 0);
    encoded.as_bytes().chunks_exact(2).map(|pair| hex_nibble(pair[0]) << 4 | hex_nibble(pair[1])).collect::<Vec<_>>().into_boxed_slice()
}

#[test]
fn local_bootstrap_hmac_matches_neutral_node_oracle_and_rejects_boundaries() {
    let fixture = fixture();
    let key = decode_hex::<32>(fixture["channelKey"].as_str().expect("key")).expect("key bytes");
    let hello: HelloWire = serde_json::from_value(fixture["hello"].clone()).expect("hello fixture");
    let hello_unsigned =
        HelloUnsigned { schema: &hello.schema, kind: &hello.kind, run_id: &hello.run_id, sequence: hello.sequence, exchange_id: &hello.exchange_id, issued_at: hello.issued_at, expires_at: hello.expires_at, launcher_nonce: &hello.launcher_nonce };
    assert_eq!(hex_lower(&hmac_sha256(&key, &serde_json::to_vec(&hello_unsigned).expect("hello canonical"))), hello.proof);
    let issue: IssueWire = serde_json::from_value(fixture["issue"].clone()).expect("issue fixture");
    let issue_unsigned = IssueUnsigned {
        schema: &issue.schema,
        kind: &issue.kind,
        run_id: &issue.run_id,
        sequence: issue.sequence,
        exchange_id: &issue.exchange_id,
        issued_at: issue.issued_at,
        expires_at: issue.expires_at,
        profile_id: &issue.profile_id,
        device_instance_id: &issue.device_instance_id,
        client_class: issue.client_class,
    };
    assert_eq!(hex_lower(&hmac_sha256(&key, &serde_json::to_vec(&issue_unsigned).expect("issue canonical"))), issue.proof);
    let credential = &fixture["credential"];
    let credential_unsigned = CredentialUnsigned {
        schema: credential["schema"].as_str().expect("credential schema"),
        run_id: credential["runId"].as_str().expect("credential run"),
        sequence: credential["sequence"].as_u64().expect("credential sequence"),
        exchange_id: credential["exchangeId"].as_str().expect("credential exchange"),
        issued_at: credential["issuedAt"].as_i64().expect("credential issued"),
        expires_at: credential["expiresAt"].as_i64().expect("credential expires"),
        profile_id: credential["profileId"].as_str().expect("credential profile"),
        client_class: serde_json::from_value(credential["clientClass"].clone()).expect("credential class"),
        session_id: credential["sessionId"].as_str().expect("credential session"),
        session_kind: serde_json::from_value(credential["sessionKind"].clone()).expect("credential session kind"),
        authorization_generation: credential["authorizationGeneration"].as_u64().expect("credential generation"),
        capability: credential["capability"].as_str().expect("credential capability"),
    };
    assert_eq!(hex_lower(&hmac_sha256(&key, &serde_json::to_vec(&credential_unsigned).expect("credential canonical"))), credential["proof"]);
    assert_ne!(fixture["hostile"]["wrongProof"], fixture["hello"]["proof"]);
    assert_eq!(LOCAL_BOOTSTRAP_FRAME_BYTES_MAX, fixture["limits"]["frameBytesMax"].as_u64().expect("frame cap") as usize);
    assert_eq!(LOCAL_BOOTSTRAP_OUTSTANDING_MAX, fixture["limits"]["outstandingRequestsMax"].as_u64().expect("request cap") as usize);
    assert_eq!(LOCAL_BOOTSTRAP_PROFILES_MAX, fixture["limits"]["profilesMax"].as_u64().expect("profile cap") as usize);
    assert_eq!(LOCAL_BOOTSTRAP_SESSION_TTL_SECS, fixture["limits"]["clientTtlSeconds"].as_i64().expect("ttl"));
    assert!(decode_hex::<32>(fixture["hostile"]["wrongProof"].as_str().expect("wrong proof")).is_ok());
    assert!(decode_hex::<16>("AA112233445566778899aabbccddeeff").is_err());
    assert!(validate_identifier(&"a".repeat(64)).is_ok());
    assert!(validate_identifier(&"a".repeat(65)).is_err());
    assert!(validate_bounded_auth_text(&"d".repeat(DEVICE_INSTANCE_MAX_BYTES), "device instance", DEVICE_INSTANCE_MAX_BYTES).is_ok());
    assert!(validate_bounded_auth_text(&"d".repeat(DEVICE_INSTANCE_MAX_BYTES + 1), "device instance", DEVICE_INSTANCE_MAX_BYTES).is_err());
    let run_id = fixture["hello"]["runId"].as_str().expect("run");
    let exchange_id = fixture["hello"]["exchangeId"].as_str().expect("exchange");
    assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 2, 2, exchange_id, 1_000, 16_000, 16_000).is_ok());
    assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 2, 2, exchange_id, 1_000, 16_001, 16_000).is_err());
    assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 1, 2, exchange_id, 1_000, 16_000, 2_000).is_err());
    assert!(validate_common(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 2, 2, exchange_id, 1_000, 16_000, 16_001).is_err());
    let mut slots = ExchangeSlots::<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>::new();
    for index in 0..LOCAL_BOOTSTRAP_OUTSTANDING_MAX {
        assert!(slots.insert(&format!("{index:032x}")));
    }
    assert!(!slots.insert("ffffffffffffffffffffffffffffffff"));
    assert!(!slots.insert("00000000000000000000000000000000"));
    assert!(slots.remove("00000000000000000000000000000000"));
    assert!(slots.insert("ffffffffffffffffffffffffffffffff"));
}

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn local_session_commit_survives_cancellation_observed_by_final_progress() {
    let sqlite = crate::directory::sqlite::SqliteDirectory::connect(":memory:").await.expect("sqlite");
    let directory = Arc::new(HubDirectories::from(sqlite));
    directory.create_user("local-commit@bootstrap.invalid", "Local Commit", None, Some("commit-subject"), Some(LOCAL_BOOTSTRAP_IDENTITY_PROVIDER)).await.expect("profile user");
    let control = CancelAtCommit { cancelled: AtomicBool::new(false), now_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("clock").as_millis() as i64 };
    let context = IdentityVerificationContext { deadline_ms: checked_deadline(control.now_ms()).expect("deadline"), control: &control };
    let request = VerifiedLocalBootstrapRequest {
        request_id: "00112233445566778899aabbccddeeff".into(),
        run_id: "ffeeddccbbaa99887766554433221100".into(),
        profile_id: "developer".into(),
        identity_provider: LOCAL_BOOTSTRAP_IDENTITY_PROVIDER.into(),
        identity_subject: "commit-subject".into(),
        display_name: "Local Commit".into(),
        device_instance_id: "native-commit-device".into(),
        client_class: LocalBootstrapClientClass::Native,
    };

    let issued = issue_local_session(directory.clone(), &request, &context).await.expect("committed session");

    assert!(control.is_cancelled());
    assert!(directory.authenticate_session(&issued.capability).await.expect("authenticate").is_some());
}
