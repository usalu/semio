
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
    serde_json::from_str(include_str!("../../🧫️fixtures/🚇️pipe-v1/🔣️.json")).expect("fixture")
}

fn admission_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/⏳️idle-admission-v1/🔣️.json")).expect("admission fixture")
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
    assert!(validate_frame(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 2, 2, exchange_id).is_ok());
    assert!(validate_window(1_000, 16_000, 16_000).is_ok());
    assert!(validate_window(1_000, 16_001, 16_000).is_err());
    assert!(validate_frame(LOCAL_BOOTSTRAP_SCHEMA, "issue", "issue", run_id, run_id, 1, 2, exchange_id).is_err());
    assert!(validate_window(1_000, 16_000, 16_001).is_err());
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

/// 🧪️ The launcher's end of one in-process pipe: frames in and out, signed with the fixture key.
#[cfg(unix)]
struct LauncherEnd {
    stream: tokio::net::UnixStream,
    key: [u8; 32],
    run_id: String,
    sequence: u64,
}

#[cfg(unix)]
impl LauncherEnd {
    async fn send(&mut self, value: &Value) {
        let bytes = serde_json::to_vec(value).expect("launcher frame");
        self.stream.write_all(&(bytes.len() as u32).to_be_bytes()).await.expect("launcher prefix");
        self.stream.write_all(&bytes).await.expect("launcher frame bytes");
    }

    async fn receive(&mut self) -> Value {
        let mut prefix = [0u8; 4];
        tokio::time::timeout(std::time::Duration::from_secs(5), self.stream.read_exact(&mut prefix)).await.expect("hub answers within 5 s").expect("hub prefix");
        let mut bytes = vec![0; u32::from_be_bytes(prefix) as usize];
        self.stream.read_exact(&mut bytes).await.expect("hub frame");
        serde_json::from_slice(&bytes).expect("hub frame JSON")
    }

    fn issue(&mut self, exchange_id: &str, issued_at: i64, profile_id: &str) -> Value {
        self.sequence += 1;
        let unsigned = IssueUnsigned {
            schema: LOCAL_BOOTSTRAP_SCHEMA,
            kind: "issue",
            run_id: &self.run_id,
            sequence: self.sequence,
            exchange_id,
            issued_at,
            expires_at: issued_at + LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS,
            profile_id,
            device_instance_id: "native-window-device",
            client_class: LocalBootstrapClientClass::Native,
        };
        let proof = hex_lower(&hmac_sha256(&self.key, &serde_json::to_vec(&unsigned).expect("canonical issue")));
        let mut wire = serde_json::to_value(&unsigned).expect("issue value");
        wire["proof"] = Value::String(proof);
        wire
    }
}

/// 🧪️ Opens a transport over one end of a socket pair, the fixture's initialize + hello on the other.
#[cfg(unix)]
async fn open_fixture_pipe(clock: Arc<AdmissionClock>) -> (Arc<InheritedLocalBootstrapTransport>, LauncherEnd) {
    use std::os::fd::OwnedFd;
    let fixture = fixture();
    let (hub, launcher) = std::os::unix::net::UnixStream::pair().expect("socket pair");
    launcher.set_nonblocking(true).expect("nonblocking launcher");
    let mut launcher = LauncherEnd { stream: tokio::net::UnixStream::from_std(launcher).expect("tokio launcher"), key: decode_hex::<32>(fixture["channelKey"].as_str().unwrap()).unwrap(), run_id: fixture["initialize"]["runId"].as_str().unwrap().to_owned(), sequence: 1 };
    launcher.send(&fixture["initialize"]).await;
    launcher.send(&fixture["hello"]).await;
    let context = IdentityVerificationContext { deadline_ms: checked_deadline(clock.now_ms()).unwrap(), control: clock.as_ref() };
    let transport = InheritedLocalBootstrapTransport::open_over(std::fs::File::from(OwnedFd::from(hub)), &context, clock.clone()).await.expect("transport over the fixture pipe");
    assert_eq!(launcher.receive().await["kind"], "hello-accepted");
    (transport, launcher)
}

/// 🪟️ A long-lived launcher exchanges for ever: the replay set bounds one validity window, not the
/// run. A burst above the window, a replayed exchange id, an expired frame and an unknown profile
/// are each answered with a signed `reject` carrying its code while the pipe stays ready; only a
/// frame that is not authentic ends the pipe.
#[cfg(unix)]
#[tokio::test]
async fn local_bootstrap_refusals_are_answers_and_the_replay_window_frees_with_time() {
    let fixture = fixture();
    assert_eq!(fixture["limits"]["replayWindowExchangesMax"].as_u64(), Some(LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX as u64));
    let start = fixture["hello"]["issuedAt"].as_i64().unwrap() + 10;
    let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(start), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
    let (transport, mut launcher) = open_fixture_pipe(clock.clone()).await;
    let context = IdentityVerificationContext { deadline_ms: i64::MAX, control: clock.as_ref() };
    for index in 0..LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX {
        let exchange_id = format!("{index:032x}");
        let frame = launcher.issue(&exchange_id, start, "developer");
        launcher.send(&frame).await;
        let request = transport.accept(clock.as_ref()).await.expect("admitted issue").expect("request");
        assert_eq!(request.request_id, exchange_id);
        transport.reject(&request.request_id, LocalBootstrapRejectCode::Unavailable, &context).await.expect("finish the request");
        assert_eq!(launcher.receive().await["exchangeId"], exchange_id.as_str());
    }
    let accepting = {
        let transport = transport.clone();
        let clock = clock.clone();
        tokio::spawn(async move { transport.accept(clock.as_ref()).await })
    };
    let refusals = [
        (launcher.issue("f0000000000000000000000000000001", start, "developer"), "resource-limit"),
        (launcher.issue(&format!("{:032x}", 0), start, "developer"), "denied"),
        (launcher.issue("f0000000000000000000000000000002", start - 2 * LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS, "developer"), "expired"),
        (launcher.issue("f0000000000000000000000000000003", start, "nobody"), "denied"),
    ];
    for (frame, code) in refusals {
        launcher.send(&frame).await;
        let answer = launcher.receive().await;
        assert_eq!((answer["kind"].as_str(), answer["code"].as_str(), answer["exchangeId"].as_str()), (Some("reject"), Some(code), frame["exchangeId"].as_str()));
        let proof = answer["proof"].as_str().unwrap().to_owned();
        let unsigned = RejectUnsigned {
            schema: LOCAL_BOOTSTRAP_SCHEMA,
            kind: "reject",
            run_id: &launcher.run_id,
            sequence: answer["sequence"].as_u64().unwrap(),
            exchange_id: answer["exchangeId"].as_str().unwrap(),
            issued_at: answer["issuedAt"].as_i64().unwrap(),
            expires_at: answer["expiresAt"].as_i64().unwrap(),
            code,
        };
        assert_eq!(proof, hex_lower(&hmac_sha256(&launcher.key, &serde_json::to_vec(&unsigned).unwrap())), "a refusal is a signed answer");
        assert!(transport.is_ready(), "a refusal never ends the pipe");
    }
    assert!(!accepting.is_finished(), "refusals are answered without surfacing a request");
    let later = start + LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS + 1;
    clock.now_ms.store(later, Ordering::Release);
    let frame = launcher.issue("f0000000000000000000000000000004", later, "developer");
    launcher.send(&frame).await;
    let admitted = tokio::time::timeout(std::time::Duration::from_secs(5), accepting).await.expect("window freed").expect("accept task").expect("accept").expect("request");
    assert_eq!(admitted.request_id, "f0000000000000000000000000000004", "expired exchanges left the window, so the 70th exchange of the run is admitted");
    transport.reject(&admitted.request_id, LocalBootstrapRejectCode::Unavailable, &context).await.expect("finish");
    launcher.receive().await;
    let mut forged = launcher.issue("f0000000000000000000000000000005", later, "developer");
    forged["proof"] = Value::String("00".repeat(32));
    launcher.send(&forged).await;
    assert!(transport.accept(clock.as_ref()).await.is_err(), "an unauthentic frame is the one refusal that ends the pipe");
    assert!(!transport.is_ready());
}

/// 🚪️ The launcher's end closing is observed while nobody accepts — the phase a hub spends loading
/// its catalog — so that hub stops with its launcher instead of outliving it.
#[cfg(unix)]
#[tokio::test]
async fn local_bootstrap_closed_resolves_when_the_launcher_leaves_before_anyone_accepts() {
    let fixture = fixture();
    let clock = Arc::new(AdmissionClock { now_ms: AtomicI64::new(fixture["hello"]["issuedAt"].as_i64().unwrap() + 10), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) });
    let (transport, launcher) = open_fixture_pipe(clock).await;
    let closed = tokio::spawn({
        let transport = transport.clone();
        async move { transport.closed().await }
    });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    assert!(!closed.is_finished(), "an open pipe is not closed");
    drop(launcher);
    tokio::time::timeout(std::time::Duration::from_secs(2), closed).await.expect("closed within 2 s of EOF").expect("closed task").expect("closed");
    assert!(transport.accept(&*Arc::new(AdmissionClock { now_ms: AtomicI64::new(0), admitted_units: AtomicU8::new(0), cancelled: AtomicBool::new(false) })).await.expect("EOF accept").is_none());
    assert!(!transport.is_ready());
}

