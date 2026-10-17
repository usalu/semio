/// 🪪️ Laws of typed capabilities and of the identity verifier port.

struct IdentityProbe {
    now: i64,
    cancelled: bool,
    progress: std::sync::Mutex<Vec<IdentityVerificationProgress>>,
}

impl IdentityVerificationControl for IdentityProbe {
    fn now_ms(&self) -> i64 {
        self.now
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    fn report(&self, progress: IdentityVerificationProgress) {
        self.progress.lock().expect("identity progress lock").push(progress);
    }
}

struct TestIdentityVerifier {
    fail: bool,
}

impl IdentityAssertionVerifier for TestIdentityVerifier {
    fn verify<'a>(&'a self, assertion: &'a IdentityAssertion, context: &'a IdentityVerificationContext<'a>) -> IdentityVerificationFuture<'a> {
        Box::pin(async move {
            context.checkpoint(0, 2)?;
            semio_framework_async::yield_once().await;
            context.checkpoint(1, 2)?;
            if self.fail || assertion.as_bytes() != b"signed-test-assertion" {
                return Err(DirectoryError::Unauthorized);
            }
            context.checkpoint(2, 2)?;
            Ok(VerifiedIdentity {
                provider: "test-verifier".into(),
                subject: "test-subject".into(),
                verified_email: Some("verified@example.com".into()),
                display_name: Some("Verified".into()),
                issued_at: 1,
                expires_at: 2,
                assurance: IdentityAssurance::ExternalVerified,
            })
        })
    }
}

#[test]
fn typed_capabilities_match_neutral_sha256_vectors_and_fixed_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🔐️auth/🧪️tests/🧫️fixtures/🔑️capability-v1/🔣️.json")).expect("auth capability fixture");
    let session = SessionCapability::parse(fixture["session"]["capability"].as_str().expect("session capability")).expect("session parser");
    let share = ShareCapability::parse(fixture["share"]["capability"].as_str().expect("share capability")).expect("share parser");
    let invite = InviteCapability::parse(fixture["invite"]["capability"].as_str().expect("invite capability")).expect("invite parser");
    let socket = SocketGrantCapability::parse(fixture["socket"]["capability"].as_str().expect("socket capability")).expect("socket parser");
    assert_eq!(session.selector(), fixture["session"]["selector"].as_str().expect("session selector"));
    assert_eq!(encode_capability_bytes(&session.secret_digest()), fixture["session"]["digestHex"].as_str().expect("session digest"));
    assert_eq!(encode_capability_bytes(&share.secret_digest()), fixture["share"]["digestHex"].as_str().expect("share digest"));
    assert_eq!(encode_capability_bytes(&invite.secret_digest()), fixture["invite"]["digestHex"].as_str().expect("invite digest"));
    assert_eq!(socket.selector(), fixture["socket"]["selector"].as_str().expect("socket selector"));
    assert_eq!(encode_capability_bytes(&socket.secret_digest()), fixture["socket"]["digestHex"].as_str().expect("socket digest"));
    assert_eq!(socket.expose_once().len(), 107);
    assert!(HubCapability::parse(&socket.expose_once()).is_err(), "socket grants are never HTTP hub capabilities");
    let socket_rejections = fixture["socket"]["rejectedCapabilities"].as_array().expect("socket rejection vectors");
    for rejected in &socket_rejections[..4] {
        assert!(SocketGrantCapability::parse(rejected.as_str().expect("socket rejection")).is_err());
    }
    let wrong_secret = SocketGrantCapability::parse(socket_rejections[4].as_str().expect("wrong-secret socket capability")).expect("wrong-secret grammar is valid");
    assert!(!constant_time_digest_eq(&wrong_secret.secret_digest(), &socket.secret_digest()));
    assert!(SessionCapability::parse(fixture["share"]["capability"].as_str().expect("share capability")).is_err());
    assert!(ShareCapability::parse(&share.expose_once().to_uppercase()).is_err());
    assert_eq!(capability_window(1, CAPABILITY_MAX_TTL_SECS).expect("maximum ttl").1, 1 + CAPABILITY_MAX_TTL_SECS * 1_000);
    assert!(capability_window(1, CAPABILITY_MAX_TTL_SECS + 1).is_err());
    assert!(IdentityAssertion::new(vec![0; AUTH_ASSERTION_MAX_BYTES].into_boxed_slice()).is_ok());
    assert!(IdentityAssertion::new(vec![0; AUTH_ASSERTION_MAX_BYTES + 1].into_boxed_slice()).is_err());
    let mut issue = AuthSessionIssue {
        user_id: "fixture-user".into(),
        identity_provider: "oidc.example".into(),
        identity_subject_digest: [7; 32],
        ttl_secs: 60,
        device_instance_id: "d".repeat(DEVICE_INSTANCE_MAX_BYTES),
        session_kind: AuthSessionKind::External,
        correlation_id: "fixture-correlation".into(),
        peer_class: "fixture".into(),
    };
    assert!(prepare_auth_session(&issue, 1).is_ok());
    issue.device_instance_id.push('d');
    assert!(prepare_auth_session(&issue, 1).is_err());
    assert!(constant_time_digest_eq(&[9; 32], &[9; 32]));
    assert!(!constant_time_digest_eq(&[9; 32], &[8; 32]));
    assert_eq!(
        encode_capability_bytes(&identity_subject_digest(fixture["identity"]["provider"].as_str().expect("provider"), fixture["identity"]["subject"].as_str().expect("subject")).expect("identity digest")),
        fixture["identity"]["digestHex"].as_str().expect("identity digest vector"),
    );
}

#[tokio::test]
async fn identity_verifier_port_honors_progress_cancel_deadline_and_provider_error() {
    let assertion = IdentityAssertion::new(b"signed-test-assertion".to_vec().into_boxed_slice()).expect("bounded assertion");
    let success = IdentityProbe { now: 10, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
    let verified = TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &success }).await.expect("verified identity");
    assert_eq!(verified.subject, "test-subject");
    assert_eq!(
        success.progress.into_inner().expect("success progress"),
        vec![IdentityVerificationProgress { completed_units: 0, total_units: 2 }, IdentityVerificationProgress { completed_units: 1, total_units: 2 }, IdentityVerificationProgress { completed_units: 2, total_units: 2 }]
    );

    let cancelled = IdentityProbe { now: 10, cancelled: true, progress: std::sync::Mutex::new(Vec::new()) };
    assert!(matches!(TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &cancelled }).await, Err(DirectoryError::Conflict(_))));
    let expired = IdentityProbe { now: 11, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
    assert!(matches!(TestIdentityVerifier { fail: false }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &expired }).await, Err(DirectoryError::Conflict(_))));
    let provider_error = IdentityProbe { now: 10, cancelled: false, progress: std::sync::Mutex::new(Vec::new()) };
    assert!(matches!(TestIdentityVerifier { fail: true }.verify(&assertion, &IdentityVerificationContext { deadline_ms: 10, control: &provider_error }).await, Err(DirectoryError::Unauthorized)));
}
