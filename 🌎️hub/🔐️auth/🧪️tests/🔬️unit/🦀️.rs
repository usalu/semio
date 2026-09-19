use super::password::{hmac_sha256, pbkdf2_sha256, PasswordCredentialError, PasswordCredentialV1};
use super::rate_limit::{HubRateLimiterV1, RateLimitClassV1, RateLimitClockV1, RateLimitDecisionV1, RateLimitSubjectV1};
use super::*;

/// 🕰️ A hand-stepped clock: every rate-limit law below reads exactly the milliseconds it sets.
struct ManualClock(std::sync::atomic::AtomicI64);

impl ManualClock {
    fn new(now_ms: i64) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self(std::sync::atomic::AtomicI64::new(now_ms)))
    }

    fn advance(&self, milliseconds: i64) {
        self.0.fetch_add(milliseconds, std::sync::atomic::Ordering::SeqCst);
    }
}

impl RateLimitClockV1 for ManualClock {
    fn now_ms(&self) -> i64 {
        self.0.load(std::sync::atomic::Ordering::SeqCst)
    }
}

const VECTORS: &str = include_str!("../../🧫️fixtures/🔑️pbkdf2-sha256-vectors-v1/🔣️.json");

fn decode_hex(encoded: &str) -> Vec<u8> {
    (0..encoded.len() / 2).map(|index| u8::from_str_radix(&encoded[index * 2..index * 2 + 2], 16).expect("fixture hex")).collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn hmac_sha256_matches_third_party_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(VECTORS).expect("pbkdf2 vector fixture");
    let vectors = fixture["hmacSha256"].as_array().expect("hmac vectors");
    assert_eq!(vectors.len(), 3);
    for vector in vectors {
        let key = decode_hex(vector["keyHex"].as_str().expect("key"));
        let message = vector["message"].as_str().expect("message");
        assert_eq!(encode_hex(&hmac_sha256(&key, message.as_bytes())), vector["macHex"].as_str().expect("mac"), "hmac vector {message}");
    }
}

#[test]
fn pbkdf2_sha256_matches_third_party_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(VECTORS).expect("pbkdf2 vector fixture");
    let vectors = fixture["pbkdf2Sha256"].as_array().expect("pbkdf2 vectors");
    assert_eq!(vectors.len(), 4);
    for vector in vectors {
        let name = vector["name"].as_str().expect("name");
        let password = vector["password"].as_str().expect("password");
        let salt = decode_hex(vector["saltHex"].as_str().expect("salt"));
        let iterations = u32::try_from(vector["iterations"].as_u64().expect("iterations")).expect("iteration bound");
        assert_eq!(encode_hex(&pbkdf2_sha256(password.as_bytes(), &salt, iterations)), vector["digestHex"].as_str().expect("digest"), "pbkdf2 vector {name}");
    }
}

#[test]
fn credential_round_trips_through_its_encoding_and_verifies_only_the_right_password() {
    let credential = PasswordCredentialV1::derive("correct horse battery staple", [0x11; 16], 1_000).expect("derive");
    let encoded = credential.encode();
    assert!(encoded.starts_with("pbkdf2-sha256$1000$"));
    let parsed = PasswordCredentialV1::parse(&encoded).expect("parse");
    assert_eq!(parsed, credential);
    assert_eq!(parsed.iterations(), 1_000);
    assert!(parsed.verify("correct horse battery staple"));
    assert!(!parsed.verify("correct horse battery stapl3"));
    assert!(!parsed.verify("short"));
    assert!(!format!("{parsed:?}").contains(&encode_hex(&parsed.digest())));
}

#[test]
fn credential_mint_is_salted_per_credential() {
    let first = PasswordCredentialV1::mint("correct horse battery staple", 1_000).expect("mint");
    let second = PasswordCredentialV1::mint("correct horse battery staple", 1_000).expect("mint");
    assert_ne!(first.salt(), second.salt());
    assert_ne!(first.digest(), second.digest());
    assert!(first.verify("correct horse battery staple") && second.verify("correct horse battery staple"));
}

#[test]
fn credential_refuses_out_of_bounds_inputs_and_malformed_encodings() {
    assert_eq!(PasswordCredentialV1::derive("short", [0; 16], 1_000), Err(PasswordCredentialError::PasswordOutOfBounds));
    assert_eq!(PasswordCredentialV1::derive(&"x".repeat(257), [0; 16], 1_000), Err(PasswordCredentialError::PasswordOutOfBounds));
    assert_eq!(PasswordCredentialV1::derive("password1", [0; 16], 999), Err(PasswordCredentialError::IterationsOutOfBounds));
    for malformed in ["", "pbkdf2-sha256", "bcrypt$1000$00$00", "pbkdf2-sha256$1000$zz$00", &format!("pbkdf2-sha256$0999${}${}", "0".repeat(32), "0".repeat(64)), &format!("pbkdf2-sha256$1000${}${}", "0".repeat(30), "0".repeat(64)), &format!("pbkdf2-sha256$1000${}${}$x", "0".repeat(32), "0".repeat(64))] {
        assert_eq!(PasswordCredentialV1::parse(malformed), Err(PasswordCredentialError::Malformed), "{malformed} must not parse");
    }
}

fn sign_in_request(password: &str) -> CredentialSignInRequestV1 {
    CredentialSignInRequestV1 { schema: CREDENTIAL_SIGN_IN_SCHEMA.into(), email: "  Person@Example.COM ".into(), password: password.into(), device_instance_id: "device-1".into(), client_class: AuthClientClassV1::Browser }
}

#[test]
fn sign_in_request_bounds_are_the_schema_bounds() {
    let verified = sign_in_request("correct horse battery staple").verify().expect("verify");
    assert_eq!(verified.email(), "person@example.com");
    assert_eq!(verified.client_class(), AuthClientClassV1::Browser);
    assert!(!format!("{verified:?}").contains("correct horse"));
    for mutate in [
        |request: &mut CredentialSignInRequestV1| request.schema = "semio.hub.auth.credential-sign-in/v2".into(),
        |request: &mut CredentialSignInRequestV1| request.email = "no-at-sign".into(),
        |request: &mut CredentialSignInRequestV1| request.email = "a@".into(),
        |request: &mut CredentialSignInRequestV1| request.email = "a@b@c".into(),
        |request: &mut CredentialSignInRequestV1| request.password = "short".into(),
        |request: &mut CredentialSignInRequestV1| request.password = "x".repeat(257),
        |request: &mut CredentialSignInRequestV1| request.password = "with\u{0}null-byte".into(),
        |request: &mut CredentialSignInRequestV1| request.device_instance_id = String::new(),
        |request: &mut CredentialSignInRequestV1| request.device_instance_id = "space here".into(),
        |request: &mut CredentialSignInRequestV1| request.device_instance_id = "d".repeat(129),
    ] {
        let mut request = sign_in_request("correct horse battery staple");
        mutate(&mut request);
        assert_eq!(request.verify().err(), Some(AuthErrorCodeV1::MalformedRequest));
    }
}

#[test]
fn sign_in_request_decoder_refuses_unknown_and_missing_fields() {
    assert!(serde_json::from_str::<CredentialSignInRequestV1>(r#"{"schema":"semio.hub.auth.credential-sign-in/v1","email":"a@b.c","password":"password1","deviceInstanceId":"d","clientClass":"browser"}"#).is_ok());
    assert!(serde_json::from_str::<CredentialSignInRequestV1>(r#"{"schema":"semio.hub.auth.credential-sign-in/v1","email":"a@b.c","password":"password1","deviceInstanceId":"d","clientClass":"browser","extra":1}"#).is_err());
    assert!(serde_json::from_str::<CredentialSignInRequestV1>(r#"{"schema":"semio.hub.auth.credential-sign-in/v1","email":"a@b.c","password":"password1","deviceInstanceId":"d"}"#).is_err());
    assert!(serde_json::from_str::<CredentialSignInRequestV1>(r#"{"schema":"semio.hub.auth.credential-sign-in/v1","email":"a@b.c","password":"password1","deviceInstanceId":"d","clientClass":"robot"}"#).is_err());
}

#[test]
fn every_refusal_is_one_status_and_the_mint_response_stays_snake_case() {
    assert_eq!(AuthErrorCodeV1::MalformedRequest.status(), 400);
    assert_eq!(AuthErrorCodeV1::InvalidCredentials.status(), 401);
    assert_eq!(AuthErrorCodeV1::CredentialSignInDisabled.status(), 403);
    assert_eq!(AuthErrorCodeV1::RateLimited.status(), 429);
    assert_eq!(AuthErrorCodeV1::DirectoryUnavailable.status(), 503);
    let encoded = serde_json::to_string(&AuthErrorV1::rate_limited(4_200)).expect("error json");
    assert_eq!(encoded, r#"{"schema":"semio.hub.auth.error/v1","error":"rate-limited","retryAfterMs":4200}"#);
    assert_eq!(serde_json::to_string(&AuthErrorV1::new(AuthErrorCodeV1::InvalidCredentials)).expect("error json"), r#"{"schema":"semio.hub.auth.error/v1","error":"invalid-credentials"}"#);
    let minted = serde_json::to_string(&SessionMintResponseV1 { token: "session.v1.0.0".into(), user_id: "usr".into() }).expect("mint json");
    assert_eq!(minted, r#"{"token":"session.v1.0.0","user_id":"usr"}"#);
}

#[test]
fn a_disabled_policy_refuses_before_any_credential_is_read() {
    let request = sign_in_request("correct horse battery staple").verify().expect("verify");
    let credential = PasswordCredentialV1::derive("correct horse battery staple", [7; 16], 1_000).expect("derive");
    let subject = CredentialSubjectV1 { user_id: "usr-1".into(), password_hash: Some(credential.encode()) };
    assert_eq!(decide_credential_sign_in(CredentialSignInPolicyV1::default(), &request, Some(&subject)), CredentialSignInDecisionV1::Refuse(AuthErrorCodeV1::CredentialSignInDisabled));
}

#[test]
fn an_unknown_account_a_credential_free_account_and_a_wrong_password_are_indistinguishable() {
    let policy = CredentialSignInPolicyV1::enabled(3_600, 1_000).expect("policy");
    let request = sign_in_request("correct horse battery staple").verify().expect("verify");
    let credential = PasswordCredentialV1::derive("a different password", [3; 16], 1_000).expect("derive");
    let refusals = [
        decide_credential_sign_in(policy, &request, None),
        decide_credential_sign_in(policy, &request, Some(&CredentialSubjectV1 { user_id: "usr-1".into(), password_hash: None })),
        decide_credential_sign_in(policy, &request, Some(&CredentialSubjectV1 { user_id: "usr-1".into(), password_hash: Some("not-a-credential".into()) })),
        decide_credential_sign_in(policy, &request, Some(&CredentialSubjectV1 { user_id: "usr-1".into(), password_hash: Some(credential.encode()) })),
    ];
    for refusal in &refusals {
        assert_eq!(refusal, &CredentialSignInDecisionV1::Refuse(AuthErrorCodeV1::InvalidCredentials));
    }
}

#[test]
fn a_matching_credential_mints_at_the_policy_lifetime() {
    let policy = CredentialSignInPolicyV1::enabled(3_600, 1_000).expect("policy");
    let request = sign_in_request("correct horse battery staple").verify().expect("verify");
    let credential = PasswordCredentialV1::derive("correct horse battery staple", [9; 16], 1_000).expect("derive");
    let subject = CredentialSubjectV1 { user_id: "usr-1".into(), password_hash: Some(credential.encode()) };
    assert_eq!(decide_credential_sign_in(policy, &request, Some(&subject)), CredentialSignInDecisionV1::Mint { user_id: "usr-1".into(), ttl_secs: 3_600 });
    assert!(CredentialSignInPolicyV1::enabled(59, 1_000).is_err());
    assert!(CredentialSignInPolicyV1::enabled(31_536_001, 1_000).is_err());
    assert!(CredentialSignInPolicyV1::enabled(3_600, 999).is_err());
}

#[test]
fn a_bucket_admits_its_burst_then_refuses_with_the_exact_wait() {
    let clock = ManualClock::new(1_000);
    let limiter = HubRateLimiterV1::new(clock.clone());
    let subject = RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([127, 0, 0, 1]));
    let policy = RateLimitClassV1::Auth.policy();
    for attempt in 0..policy.burst {
        assert_eq!(limiter.admit(RateLimitClassV1::Auth, &[subject]), RateLimitDecisionV1::Admitted, "attempt {attempt}");
    }
    let refusal = limiter.admit(RateLimitClassV1::Auth, &[subject]);
    assert_eq!(refusal, RateLimitDecisionV1::Refused { retry_after_ms: u64::from(policy.cost_ms) });
    assert_eq!(refusal.retry_after_secs(), 6);
    clock.advance(i64::from(policy.cost_ms) - 1);
    assert_eq!(limiter.admit(RateLimitClassV1::Auth, &[subject]), RateLimitDecisionV1::Refused { retry_after_ms: 1 });
    clock.advance(1);
    assert_eq!(limiter.admit(RateLimitClassV1::Auth, &[subject]), RateLimitDecisionV1::Admitted);
}

#[test]
fn buckets_are_per_subject_and_per_class() {
    let clock = ManualClock::new(0);
    let limiter = HubRateLimiterV1::new(clock);
    let first = RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([10, 0, 0, 1]));
    let second = RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([10, 0, 0, 2]));
    for _ in 0..RateLimitClassV1::Auth.policy().burst {
        assert!(limiter.admit(RateLimitClassV1::Auth, &[first]).is_admitted());
    }
    assert!(!limiter.admit(RateLimitClassV1::Auth, &[first]).is_admitted());
    assert!(limiter.admit(RateLimitClassV1::Auth, &[second]).is_admitted());
    assert!(limiter.admit(RateLimitClassV1::DirectoryCommand, &[first]).is_admitted());
    assert_ne!(RateLimitSubjectV1::principal("selector"), RateLimitSubjectV1::claimed_identity("selector"));
    assert_eq!(RateLimitSubjectV1::claimed_identity("Person@Example.com"), RateLimitSubjectV1::claimed_identity("person@example.com"));
}

#[test]
fn a_request_is_charged_to_every_subject_and_a_refusal_charges_none() {
    let clock = ManualClock::new(0);
    let limiter = HubRateLimiterV1::new(clock.clone());
    let address = RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([10, 0, 0, 3]));
    let identity = RateLimitSubjectV1::claimed_identity("person@example.com");
    let burst = RateLimitClassV1::Auth.policy().burst;
    for _ in 0..burst {
        assert!(limiter.admit(RateLimitClassV1::Auth, &[address]).is_admitted());
    }
    assert!(!limiter.admit(RateLimitClassV1::Auth, &[address, identity]).is_admitted());
    for _ in 0..burst {
        assert!(limiter.admit(RateLimitClassV1::Auth, &[identity]).is_admitted(), "the refused attempt never charged the identity bucket");
    }
}

#[test]
fn the_subject_map_stays_bounded_and_sheds_only_recovered_subjects() {
    let clock = ManualClock::new(0);
    let limiter = HubRateLimiterV1::with_capacity(clock.clone(), 4);
    let held = RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([172, 16, 0, 1]));
    for _ in 0..RateLimitClassV1::Auth.policy().burst {
        assert!(limiter.admit(RateLimitClassV1::Auth, &[held]).is_admitted());
    }
    for octet in 2..5u8 {
        assert!(limiter.admit(RateLimitClassV1::Auth, &[RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([172, 16, 0, octet]))]).is_admitted());
    }
    assert_eq!(limiter.tracked_subjects(), 4);
    assert!(!limiter.admit(RateLimitClassV1::Auth, &[RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([172, 16, 0, 5]))]).is_admitted(), "a full limiter fails closed rather than growing");
    clock.advance(i64::from(RateLimitClassV1::Auth.policy().cost_ms));
    assert!(limiter.admit(RateLimitClassV1::Auth, &[RateLimitSubjectV1::remote_address(&std::net::IpAddr::from([172, 16, 0, 6]))]).is_admitted(), "recovered subjects are shed to make room");
    assert!(limiter.tracked_subjects() <= 4);
    assert!(limiter.admit(RateLimitClassV1::Auth, &[held]).is_admitted(), "the held subject recovered exactly the one token the clock paid for");
    assert!(!limiter.admit(RateLimitClassV1::Auth, &[held]).is_admitted(), "and not one more — an exhausted subject is never silently reset to full");
}

const SCHEMA_MODULE: &str = include_str!("../../🧬️schema/🔣️.json");

/// 📥 Compiles one `$defs` export of the `hub.auth` module through the owned draft-07 validator.
fn structural(export: &str) -> semio_framework_schema::OwnedJsonSchemaValidator {
    let mut document: serde_json::Value = serde_json::from_str(SCHEMA_MODULE).expect("hub.auth schema module");
    document["$ref"] = serde_json::Value::String(format!("#/$defs/{export}"));
    semio_framework_schema::OwnedJsonSchemaValidator::compile(&document.to_string()).unwrap_or_else(|error| panic!("{export} does not compile: {error:?}"))
}

#[test]
fn the_rust_decoders_and_the_json_schema_admit_exactly_the_same_wire() {
    let request = serde_json::json!({ "schema": CREDENTIAL_SIGN_IN_SCHEMA, "email": "person@example.com", "password": "correct horse battery staple", "deviceInstanceId": "device-1", "clientClass": "browser" });
    let request_schema = structural("CredentialSignInRequestV1");
    assert!(request_schema.is_valid_json(&request.to_string()));
    assert!(serde_json::from_value::<CredentialSignInRequestV1>(request.clone()).expect("decode").verify().is_ok());
    for (name, mutate) in [
        ("unknown field", serde_json::json!({ "schema": CREDENTIAL_SIGN_IN_SCHEMA, "email": "person@example.com", "password": "correct horse battery staple", "deviceInstanceId": "device-1", "clientClass": "browser", "extra": 1 })),
        ("short password", serde_json::json!({ "schema": CREDENTIAL_SIGN_IN_SCHEMA, "email": "person@example.com", "password": "short", "deviceInstanceId": "device-1", "clientClass": "browser" })),
        ("no at sign", serde_json::json!({ "schema": CREDENTIAL_SIGN_IN_SCHEMA, "email": "person.example.com", "password": "correct horse battery staple", "deviceInstanceId": "device-1", "clientClass": "browser" })),
        ("spaced device", serde_json::json!({ "schema": CREDENTIAL_SIGN_IN_SCHEMA, "email": "person@example.com", "password": "correct horse battery staple", "deviceInstanceId": "device 1", "clientClass": "browser" })),
        ("unknown class", serde_json::json!({ "schema": CREDENTIAL_SIGN_IN_SCHEMA, "email": "person@example.com", "password": "correct horse battery staple", "deviceInstanceId": "device-1", "clientClass": "robot" })),
        ("wrong schema", serde_json::json!({ "schema": "semio.hub.auth.credential-sign-in/v2", "email": "person@example.com", "password": "correct horse battery staple", "deviceInstanceId": "device-1", "clientClass": "browser" })),
    ] {
        assert!(!request_schema.is_valid_json(&mutate.to_string()), "{name} must fail the schema");
        let decoded = serde_json::from_value::<CredentialSignInRequestV1>(mutate).map_err(|_| AuthErrorCodeV1::MalformedRequest).and_then(CredentialSignInRequestV1::verify);
        assert!(decoded.is_err(), "{name} must fail the Rust decoder");
    }

    let minted = SessionMintResponseV1 { token: format!("session.v1.{}.{}", "0".repeat(32), "a".repeat(64)), user_id: "usr-1".into() };
    assert!(structural("SessionMintResponseV1").is_valid_json(&serde_json::to_string(&minted).expect("mint json")));
    let error_schema = structural("AuthErrorV1");
    assert!(error_schema.is_valid_json(&serde_json::to_string(&AuthErrorV1::new(AuthErrorCodeV1::InvalidCredentials)).expect("error json")));
    assert!(error_schema.is_valid_json(&serde_json::to_string(&AuthErrorV1::rate_limited(6_000)).expect("error json")));
    let credential_schema = structural("CredentialHashV1");
    assert!(credential_schema.is_valid_json(&serde_json::to_string(&PasswordCredentialV1::derive("correct horse battery staple", [0x2a; 16], 210_000).expect("derive").encode()).expect("credential json")));
    assert!(!credential_schema.is_valid_json(&serde_json::to_string("pbkdf2-sha512$1000$00$00").expect("credential json")));
}
