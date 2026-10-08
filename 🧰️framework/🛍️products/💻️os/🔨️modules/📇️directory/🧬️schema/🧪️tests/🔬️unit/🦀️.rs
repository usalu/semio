use crate::os_directory::io::binary::artifact_hash::hex_lower;
use crate::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use crate::os_directory::io::text::directory_command_sha256;
use crate::os_directory::io::text::valid_user_preference_record_v1;
use crate::os_directory::io::text::validate_directory_event_page_event;
use super::*;

#[test]
fn intrinsic_bytes_are_refused_by_the_directory_json_event_mirror() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let bytes: Vec<u8> = serde_json::from_value(row["octets"].clone()).unwrap();
        assert!(directory_event_page_has_control(&crate::DslValue::Bytes(bytes.clone())));
        assert!(!directory_event_page_has_control(&crate::ToValue::to_value(&bytes)));
    }
}

#[test]
fn directory_session_authority_v1_matches_neutral_corpus_and_binding_goldens() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🪪️session-authority-v1/🔣️.json")).expect("session authority fixture");
    for row in fixture["rows"].as_array().expect("rows") {
        let parsed = serde_json::from_value::<DirectorySessionAuthorityV1>(row["value"].clone()).ok().filter(DirectorySessionAuthorityV1::validate);
        assert_eq!(parsed.is_some(), row["accepted"].as_bool().expect("accepted"), "{}", row["id"]);
    }
    for row in fixture["raw"].as_array().expect("raw") {
        let source = row["source"].as_str().expect("source");
        assert_eq!(DirectorySessionAuthorityV1::parse_canonical_json(source).is_some(), row["accepted"].as_bool().expect("accepted"), "{}", row["id"]);
    }
    for row in fixture["bindingGoldens"].as_array().expect("binding goldens") {
        let session_id = row["sessionId"].as_str().expect("session id");
        let user_id = row["userId"].as_str().expect("user id");
        let generation = row["authorizationGeneration"].as_u64().expect("generation");
        let expires_at = row["expiresAt"].as_i64().expect("expiry");
        let mut encoded = b"semio/hub/directory-event-page/session-binding/v1\0".to_vec();
        encoded.extend_from_slice(&u32::try_from(session_id.len()).expect("session id length").to_be_bytes());
        encoded.extend_from_slice(session_id.as_bytes());
        encoded.extend_from_slice(&u32::try_from(user_id.len()).expect("user id length").to_be_bytes());
        encoded.extend_from_slice(user_id.as_bytes());
        encoded.extend_from_slice(&generation.to_be_bytes());
        encoded.extend_from_slice(&expires_at.to_be_bytes());
        assert_eq!(semio_framework_hash::sha256_hex(&encoded), row["sessionBindingSha256"].as_str().expect("binding"), "{}", row["id"]);
    }
    for row in fixture["authorityLifecycle"].as_array().expect("authority lifecycle") {
        let outcome = row["outcome"].as_str().expect("outcome");
        assert!(matches!(outcome, "installed" | "retained" | "replaced" | "ignored" | "retired"), "{}", row["id"]);
        assert_eq!(row["epochDelta"] == 1, row["retireMountedAuthorities"] == true, "{}", row["id"]);
        if matches!(outcome, "replaced" | "retired") {
            assert_eq!(row["retainIndeterminateJobs"], true, "{}", row["id"]);
        }
        if row["brokerAdmissionCurrent"] == false {
            assert_eq!(outcome, "ignored", "{}", row["id"]);
        }
    }
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DirectoryEventPageFixture {
    valid: DirectoryEventPageV1,
    canonical_unsigned: String,
    expected_receipt_sha256: String,
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DirectoryCommandRequestVector {
    name: String,
    request_id: String,
    command: DirectoryCommand,
    canonical: String,
    command_sha256: String,
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DirectoryCommandReceiptVector {
    name: String,
    request_name: String,
    outcome: DirectoryCommandOutcomeV1,
    canonical: String,
    receipt_sha256: String,
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DirectoryCommandRejectedRequestVector {
    name: String,
    source: String,
    code: String,
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DirectoryCommandRejectedReceiptVector {
    name: String,
    request_name: String,
    source: String,
    code: String,
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DirectoryCommandReceiptFixture {
    requests: Vec<DirectoryCommandRequestVector>,
    receipts: Vec<DirectoryCommandReceiptVector>,
    rejected_requests: Vec<DirectoryCommandRejectedRequestVector>,
    rejected_receipts: Vec<DirectoryCommandRejectedReceiptVector>,
}

#[semio_framework_async_macros::async_test]
async fn directory_command_receipt_v1_matches_language_neutral_vectors_and_rejects_hostiles() {
    let fixture: DirectoryCommandReceiptFixture = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧾️command-receipt-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("command-receipt fixture decodes");
    let request_of = |name: &str| -> DirectoryCommandRequestV1 {
        let vector = fixture.requests.iter().find(|request| request.name == name).expect("request vector");
        DirectoryCommandRequestV1::new(vector.request_id.clone(), vector.command.clone())
    };
    for vector in &fixture.requests {
        let request = DirectoryCommandRequestV1::new(vector.request_id.clone(), vector.command.clone());
        assert_eq!(request.canonical_json(), vector.canonical, "{} canonical request", vector.name);
        assert_eq!(directory_command_sha256(&vector.command), vector.command_sha256, "{} command digest", vector.name);
        assert_eq!(DirectoryCommandRequestV1::parse_canonical_json(&vector.canonical), Ok(request), "{} round trip", vector.name);
    }
    let mut delivered = 0usize;
    for vector in &fixture.receipts {
        let request = request_of(&vector.request_name);
        let receipt = DirectoryCommandReceiptV1::parse_canonical_json(&vector.canonical, &request).unwrap_or_else(|error| panic!("{} canonical receipt: {error:?}", vector.name));
        assert_eq!(receipt.outcome, vector.outcome, "{} outcome", vector.name);
        assert_eq!(receipt.receipt_sha256, vector.receipt_sha256, "{} receipt digest", vector.name);
        assert_eq!(DirectoryCommandReceiptV1::seal(receipt.request_id.clone(), receipt.command_sha256.clone(), receipt.outcome, receipt.events.clone(), receipt.result.clone()), receipt, "{} seal", vector.name);
        if let DirectoryCommandResultV1::Invite { invite_token } = &receipt.result {
            assert_eq!(receipt.outcome, DirectoryCommandOutcomeV1::Accepted, "{} only a live acceptance carries a capability", vector.name);
            assert!(invite_token.len() <= DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES);
            delivered += 1;
        } else if receipt.outcome != DirectoryCommandOutcomeV1::Accepted {
            assert!(receipt.events.is_empty(), "{} a redacted outcome delivers no events", vector.name);
        }
    }
    assert_eq!(delivered, 1, "exactly one neutral vector proves live one-shot capability delivery");
    for vector in &fixture.rejected_requests {
        let expected = if vector.code == "too-large" { DirectoryCommandErrorCodeV1::TooLarge } else { DirectoryCommandErrorCodeV1::Invalid };
        assert_eq!(DirectoryCommandRequestV1::parse_canonical_json(&vector.source), Err(expected), "{} rejected request", vector.name);
    }
    for vector in &fixture.rejected_receipts {
        let expected = if vector.code == "too-large" { DirectoryCommandErrorCodeV1::TooLarge } else { DirectoryCommandErrorCodeV1::Invalid };
        assert_eq!(DirectoryCommandReceiptV1::parse_canonical_json(&vector.source, &request_of(&vector.request_name)), Err(expected), "{} rejected receipt", vector.name);
    }
    let minted = mint_directory_command_request_id();
    assert!(minted.len() == DIRECTORY_COMMAND_REQUEST_ID_LEN && minted.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')) && minted != "0".repeat(DIRECTORY_COMMAND_REQUEST_ID_LEN));
    assert_ne!(minted, mint_directory_command_request_id());
    for (status, code) in [
        (401u16, DirectoryCommandErrorCodeV1::Unauthorized),
        (403, DirectoryCommandErrorCodeV1::Forbidden),
        (409, DirectoryCommandErrorCodeV1::RequestConflict),
        (410, DirectoryCommandErrorCodeV1::StaleSession),
        (413, DirectoryCommandErrorCodeV1::TooLarge),
        (503, DirectoryCommandErrorCodeV1::Overloaded),
        (500, DirectoryCommandErrorCodeV1::Invalid),
    ] {
        assert_eq!(DirectoryCommandErrorCodeV1::from_status(status), code);
        assert_eq!(code.is_transient(), matches!(code, DirectoryCommandErrorCodeV1::Overloaded));
    }
    assert!(DirectoryCommandErrorCodeV1::Transport.is_transient() && !DirectoryCommandErrorCodeV1::Cancelled.is_transient() && !DirectoryCommandErrorCodeV1::Capacity.is_transient() && !DirectoryCommandErrorCodeV1::Closed.is_transient());
}

#[semio_framework_async_macros::async_test]
async fn directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles() {
    let fixture: DirectoryEventPageFixture = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/📃️event-page-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("event-page fixture decodes");
    assert_eq!(fixture.valid.canonical_unsigned_json(), fixture.canonical_unsigned);
    assert_eq!(semio_framework_hash::sha256_hex(fixture.canonical_unsigned.as_bytes()), fixture.expected_receipt_sha256);
    assert_eq!(fixture.valid.validate(), Ok(()));
    let canonical = semio_framework_pack_json::to_json_string(&fixture.valid);
    assert_eq!(DirectoryEventPageV1::parse_canonical_json(&canonical), Ok(fixture.valid.clone()));

    let mut hostile = fixture.valid.clone();
    hostile.session_binding_sha256.make_ascii_uppercase();
    assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
    hostile = fixture.valid.clone();
    hostile.authorization_generation = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
    assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
    hostile = fixture.valid.clone();
    hostile.after_seq_exclusive = hostile.events[0].seq;
    assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
    hostile = fixture.valid.clone();
    hostile.events[0].seq = hostile.after_seq_exclusive;
    assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));
    hostile = fixture.valid.clone();
    hostile.receipt_sha256 = "b".repeat(64);
    assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::ReceiptMismatch));
    hostile = fixture.valid.clone();
    if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut hostile.events[0].body {
        name.push('\u{1}');
    }
    assert_eq!(hostile.validate(), Err(DirectoryEventPageErrorV1::Invalid));

    let mut boundary = fixture.valid.events[0].clone();
    if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut boundary.body {
        name.clear();
    }
    let base = semio_framework_pack_json::to_json_string(&boundary).len();
    if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut boundary.body {
        *name = "x".repeat(DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
    }
    assert_eq!(semio_framework_pack_json::to_json_string(&boundary).len(), DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
    assert_eq!(validate_directory_event_page_event(&boundary), Ok(()));
    if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut boundary.body {
        name.push('x');
    }
    assert_eq!(validate_directory_event_page_event(&boundary), Err(DirectoryEventPageErrorV1::Invalid));

    assert_eq!(DirectoryEventPageV1::parse_canonical_json(&format!("{canonical} ")), Err(DirectoryEventPageErrorV1::Invalid));
    assert_eq!(DirectoryEventPageV1::parse_canonical_json(&canonical.replacen("{\"schema\":", "{\"schema\":\"duplicate\",\"schema\":", 1)), Err(DirectoryEventPageErrorV1::Invalid));
    assert_eq!(DirectoryEventPageV1::parse_canonical_json(&canonical.replacen("{\"schema\":", "{\"unexpected\":true,\"schema\":", 1)), Err(DirectoryEventPageErrorV1::Invalid));
}

#[semio_framework_async_macros::async_test]
async fn event_body_kind_is_the_dotted_wire_string() {
    let body = DirectoryEventBody::SpaceCreated { space_id: "sp-1".into(), name: "Studio".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private, owner_user_id: "u-1".into() };
    let json = semio_framework_pack_json::to_json_string(&body);
    assert!(json.contains("\"kind\":\"space.created\""), "got {json}");
    assert!(json.contains("\"spaceKind\":\"studio\""), "got {json}");
    assert!(json.contains("\"visibility\":\"private\""), "got {json}");
    let round: DirectoryEventBody = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("deserialize");
    assert_eq!(round, body);
}

#[semio_framework_async_macros::async_test]
async fn command_kind_is_kebab_case() {
    let command = DirectoryCommand::CreateSpace { name: "Atelier".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private };
    let json = semio_framework_pack_json::to_json_string(&command);
    assert!(json.contains("\"kind\":\"create-space\""), "got {json}");
    assert!(json.contains("\"spaceKind\":\"atelier\""), "got {json}");
}

#[semio_framework_async_macros::async_test]
async fn stream_message_kinds_round_trip() {
    let heartbeat = DirectoryStreamMessage::Heartbeat { head_seq: 42 };
    let json = semio_framework_pack_json::to_json_string(&heartbeat);
    assert!(json.contains("\"kind\":\"heartbeat\""), "got {json}");
    assert!(json.contains("\"headSeq\":42"), "got {json} (must be a bare integer, not 42.0)");
    let round: DirectoryStreamMessage = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("deserialize");
    assert_eq!(round, heartbeat);
}

/// 🔢️ The exact scenario `📓️directory-spr-serde-removal.md` declined on: a `u64` field must
/// round-trip as a bare wire integer, never `.0`-suffixed — `DslValue::Number` no longer
/// erases the UInt/Float distinction (`📓️dslvalue-integer-fidelity.md`).
#[semio_framework_async_macros::async_test]
async fn create_invite_ttl_secs_is_a_bare_integer_on_the_wire() {
    let command = DirectoryCommand::CreateInvite { space_id: "sp-1".into(), role: DirectorySpaceRole::Author, ttl_secs: 3600 };
    let json = semio_framework_pack_json::to_json_string(&command);
    assert!(json.contains("\"ttlSecs\":3600"), "got {json}");
    assert!(!json.contains("3600.0"), "got {json} — ttl_secs must not collapse to a float");
    let round: DirectoryCommand = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("deserialize");
    assert_eq!(round, command);
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DescriptorFixture {
    valid: DocumentDescriptor,
    canonical: String,
}

#[semio_framework_async_macros::async_test]
async fn document_descriptor_matches_the_language_neutral_fixture() {
    let fixture: DescriptorFixture = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🪪️document-descriptor.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("descriptor fixture decodes");
    assert_eq!(semio_framework_pack_json::to_json_string(&fixture.valid), fixture.canonical);
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DocumentOpenPlanFixture {
    now_ms: u64,
    descriptor: DocumentDescriptor,
    descriptor_digest_v1: String,
    intent: DocumentOpenIntentV1,
    valid_plan: DocumentOpenPlanV1,
    exchange_intent: DocumentPlanSocketGrantIntentV1,
}

#[test]
fn document_authority_json_integer_tokens_never_coerce() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🌱️value/🔁️codec/🧫️fixtures/🔣️.json")).expect("neutral exact integer corpus");
    let fixture: DocumentOpenPlanFixture = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let intent_json = semio_framework_pack_json::to_json_string(&fixture.intent);
    assert!(intent_json.contains("\"version\":1"));
    for row in corpus["raw"].as_array().unwrap() {
        let raw = row.as_str().unwrap();
        macro_rules! check {
                ($($ty:ty),+ $(,)?) => { $(
                    let reference = serde_json::from_str::<$ty>(raw);
                    let ours = semio_framework_pack_json::from_json_str::<$ty>(raw, semio_framework_pack_json::JsonMemberPolicy::Reject);
                    assert_eq!(ours.is_ok(), reference.is_ok(), "JSON admission {} {raw}", stringify!($ty));
                    if let (Ok(ours), Ok(reference)) = (ours, reference) {
                        assert_eq!(ours, reference, "JSON exact {} {raw}", stringify!($ty));
                    }
                )+ };
            }
        check!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
        let version = serde_json::from_str::<u32>(raw);
        let intent = semio_framework_pack_json::from_json_str::<DocumentOpenIntentV1>(&intent_json.replace("\"version\":1", &format!("\"version\":{raw}")), semio_framework_pack_json::JsonMemberPolicy::Reject);
        assert_eq!(intent.is_ok(), version.is_ok(), "intent integer {raw}");
        if let (Ok(intent), Ok(version)) = (intent, version) {
            assert_eq!(intent.version, version);
            assert_eq!(intent.validate().is_ok(), version == 1, "intent version authority {raw}");
        }
        let expiry = serde_json::from_str::<i64>(raw);
        let grant_json = format!(r#"{{"schema":"fixture","protocol":"fixture","grant":"fixture","actorId":"fixture","expiresAtMs":{raw}}}"#);
        let grant = semio_framework_pack_json::from_json_str::<crate::os_directory::client::SocketGrantReceiptV1>(&grant_json, semio_framework_pack_json::JsonMemberPolicy::Reject);
        assert_eq!(grant.is_ok(), expiry.is_ok(), "socket expiry integer {raw}");
        if let (Ok(grant), Ok(expiry)) = (grant, expiry) {
            assert_eq!(grant.expires_at_ms, expiry, "socket expiry exact {raw}");
        }
        let document_grant = semio_framework_pack_json::from_json_str::<crate::os_directory::client::DocumentSocketGrantReceiptV1>(&format!(r#"{{"schema":"fixture","protocol":"fixture","actorId":"fixture","expiresAtMs":{raw}}}"#), semio_framework_pack_json::JsonMemberPolicy::Reject);
        assert_eq!(document_grant.as_ref().map(|grant| grant.expires_at_ms).ok(), serde_json::from_str::<i64>(raw).ok(), "document socket expiry exact {raw}");
    }
}

#[semio_framework_async_macros::async_test]
async fn document_open_plan_v1_matches_language_neutral_fixture() {
    let fixture: DocumentOpenPlanFixture = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture decodes");
    assert_eq!(hex_lower(&descriptor_digest_v1(&fixture.descriptor).expect("descriptor hashes").0), fixture.descriptor_digest_v1);
    assert_eq!(fixture.intent.validate(), Ok(()));
    assert_eq!(fixture.valid_plan.validate(fixture.now_ms), Ok(()));
    let mut two_space = fixture.valid_plan.clone();
    two_space.parent_dialect.artifact_kind = "s.note.note".into();
    two_space.artifact.kind = "2d.note".into();
    assert_eq!(two_space.validate(fixture.now_ms), Ok(()));
    let mut unbounded_dialect_kind = fixture.valid_plan.clone();
    unbounded_dialect_kind.parent_dialect.artifact_kind = String::new();
    assert_eq!(unbounded_dialect_kind.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    assert_eq!(fixture.exchange_intent.validate(), Ok(()));

    let mut overlong = fixture.valid_plan.clone();
    overlong.expires_at_unix_ms = fixture.now_ms + DOCUMENT_OPEN_PLAN_MAX_TTL_MS + 1;
    assert_eq!(overlong.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let encoded = semio_framework_pack_json::to_json_string(&fixture.valid_plan);
    let forged = format!("{},\"actor\":\"caller-selected\"}}", encoded.strip_suffix('}').expect("object"));
    assert!(semio_framework_pack_json::from_json_str::<DocumentOpenPlanV1>(&forged, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    let nested_scope = encoded.replace("\"documentId\":\"plan:\u{6771}\u{4eac}\"", "\"documentId\":\"plan:\u{6771}\u{4eac}\",\"actor\":\"caller-selected\"");
    assert!(semio_framework_pack_json::from_json_str::<DocumentOpenPlanV1>(&nested_scope, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    let nested_frontier = encoded.replace("\"headEditOrdinal\":2", "\"headEditOrdinal\":2,\"storageKey\":\"private\"");
    assert!(semio_framework_pack_json::from_json_str::<DocumentOpenPlanV1>(&nested_frontier, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());

    let mut unicode_control = fixture.valid_plan.clone();
    unicode_control.surface.app_id = "app.\u{85}hidden".into();
    assert_eq!(unicode_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    // 🪢 A parent dialect naming another app's artifact kind is ADMITTED here: `artifact.kind` and
    // `parent_dialect.artifact_kind` are separate id spaces, bounded separately and never against
    // each other, and their binding is pinned by the trusted catalog's `validate_descriptor_open_target`.
    let mut parent_kind = fixture.valid_plan.clone();
    parent_kind.parent_dialect.artifact_kind = "s.foreign.document".into();
    assert_eq!(parent_kind.validate(fixture.now_ms), Ok(()));
    let mut parent_control = fixture.valid_plan.clone();
    parent_control.parent_dialect.standard.push('\u{85}');
    assert_eq!(parent_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut parent_trim = fixture.valid_plan.clone();
    parent_trim.parent_dialect.subset = " * ".into();
    assert_eq!(parent_trim.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut parent_kind_trim = fixture.valid_plan.clone();
    parent_kind_trim.artifact.kind = " s.gis:gismap ".into();
    parent_kind_trim.parent_dialect.artifact_kind = parent_kind_trim.artifact.kind.clone();
    assert_eq!(parent_kind_trim.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut noncanonical_receipt = fixture.valid_plan.clone();
    noncanonical_receipt.receipt = "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB".into();
    assert_eq!(noncanonical_receipt.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut frontier_control = fixture.valid_plan.clone();
    frontier_control.checkpoint.baseline_frontier.head_edit_id = "edit:\u{85}".into();
    assert_eq!(frontier_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut frontier_overlong = fixture.valid_plan.clone();
    frontier_overlong.checkpoint.baseline_frontier.head_edit_id = "a".repeat(DOCUMENT_OPEN_ID_MAX_BYTES + 1);
    assert_eq!(frontier_overlong.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut unsafe_expiry = fixture.valid_plan.clone();
    unsafe_expiry.expires_at_unix_ms = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
    assert_eq!(unsafe_expiry.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut unsafe_frontier = fixture.valid_plan.clone();
    unsafe_frontier.checkpoint.baseline_frontier.head_edit_ordinal = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
    assert_eq!(unsafe_frontier.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut unsafe_revalidation = fixture.valid_plan;
    unsafe_revalidation.revalidation.directory_revision = DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1;
    assert_eq!(unsafe_revalidation.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
}

#[test]
fn user_preference_record_v1_matches_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️user-preference-record/🔣️.json")).expect("user preference record fixture");
    let rows = fixture["rows"].as_array().expect("rows");
    assert!(rows.len() >= 15);
    for row in rows {
        let schema = row["schema"].as_str().expect("schema");
        let mutation = row["mutation"].as_str().expect("mutation");
        assert_eq!(valid_user_preference_record_v1(schema, mutation), row["valid"].as_bool().expect("valid"), "{}", row["id"]);
    }
    let event = |user_id: &str, space_id: Option<&str>, owner: Option<&str>| DirectoryEvent {
        seq: 1,
        id: "e1".into(),
        hlc: Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::User, id: user_id.into() },
        space_id: space_id.map(Into::into),
        user_id: owner.map(Into::into),
        body: DirectoryEventBody::UserPreferenceRecorded { user_id: user_id.into(), schema: "os.config.ui-preferences.v1".into(), mutation: "{}".into() },
        recorded_at_ms: 1,
    };
    assert!(validate_directory_event_page_event(&event("u1", None, Some("u1"))).is_ok());
    assert!(validate_directory_event_page_event(&event("u1", None, Some("u2"))).is_err(), "the event's owner is the preference's user");
    assert!(validate_directory_event_page_event(&event("u1", Some("s1"), Some("u1"))).is_err(), "a preference belongs to no space");
}
