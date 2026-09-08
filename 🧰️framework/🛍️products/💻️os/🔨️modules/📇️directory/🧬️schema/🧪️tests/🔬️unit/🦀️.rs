use super::*;

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
    let fixture: DirectoryCommandReceiptFixture = crate::os_pack::json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧾️command-receipt-v1.json")).expect("command-receipt fixture decodes");
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
    let fixture: DirectoryEventPageFixture = crate::os_pack::json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/📃️event-page-v1.json")).expect("event-page fixture decodes");
    assert_eq!(fixture.valid.canonical_unsigned_json(), fixture.canonical_unsigned);
    assert_eq!(semio_framework_hash::sha256_hex(fixture.canonical_unsigned.as_bytes()), fixture.expected_receipt_sha256);
    assert_eq!(fixture.valid.validate(), Ok(()));
    let canonical = crate::os_pack::json::to_json_string(&fixture.valid);
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
    let base = crate::os_pack::json::to_json_string(&boundary).len();
    if let DirectoryEventBody::SpaceRenamed { name, .. } = &mut boundary.body {
        *name = "x".repeat(DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
    }
    assert_eq!(crate::os_pack::json::to_json_string(&boundary).len(), DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
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
    let json = crate::os_pack::json::to_json_string(&body);
    assert!(json.contains("\"kind\":\"space.created\""), "got {json}");
    assert!(json.contains("\"spaceKind\":\"studio\""), "got {json}");
    assert!(json.contains("\"visibility\":\"private\""), "got {json}");
    let round: DirectoryEventBody = crate::os_pack::json::from_json_str(&json).expect("deserialize");
    assert_eq!(round, body);
}

#[semio_framework_async_macros::async_test]
async fn command_kind_is_kebab_case() {
    let command = DirectoryCommand::CreateSpace { name: "Atelier".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private };
    let json = crate::os_pack::json::to_json_string(&command);
    assert!(json.contains("\"kind\":\"create-space\""), "got {json}");
    assert!(json.contains("\"spaceKind\":\"atelier\""), "got {json}");
}

#[semio_framework_async_macros::async_test]
async fn stream_message_kinds_round_trip() {
    let heartbeat = DirectoryStreamMessage::Heartbeat { head_seq: 42 };
    let json = crate::os_pack::json::to_json_string(&heartbeat);
    assert!(json.contains("\"kind\":\"heartbeat\""), "got {json}");
    assert!(json.contains("\"headSeq\":42"), "got {json} (must be a bare integer, not 42.0)");
    let round: DirectoryStreamMessage = crate::os_pack::json::from_json_str(&json).expect("deserialize");
    assert_eq!(round, heartbeat);
}

/// 🔢️ The exact scenario `📓️directory-spr-serde-removal.md` declined on: a `u64` field must
/// round-trip as a bare wire integer, never `.0`-suffixed — `DslValue::Number` no longer
/// erases the UInt/Float distinction (`📓️dslvalue-integer-fidelity.md`).
#[semio_framework_async_macros::async_test]
async fn create_invite_ttl_secs_is_a_bare_integer_on_the_wire() {
    let command = DirectoryCommand::CreateInvite { space_id: "sp-1".into(), role: DirectorySpaceRole::Author, ttl_secs: 3600 };
    let json = crate::os_pack::json::to_json_string(&command);
    assert!(json.contains("\"ttlSecs\":3600"), "got {json}");
    assert!(!json.contains("3600.0"), "got {json} — ttl_secs must not collapse to a float");
    let round: DirectoryCommand = crate::os_pack::json::from_json_str(&json).expect("deserialize");
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
    let fixture: DescriptorFixture = crate::os_pack::json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🪪️document-descriptor.json")).expect("descriptor fixture decodes");
    assert_eq!(crate::os_pack::json::to_json_string(&fixture.valid), fixture.canonical);
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct ArtifactAuthorityFixture {
    descriptor: DocumentDescriptor,
    descriptor_encoding_hex: String,
    descriptor_digest_v1: ArtifactHash,
}

#[semio_framework_async_macros::async_test]
async fn document_descriptor_digest_v1_matches_the_language_neutral_binary_vector() {
    let fixture: ArtifactAuthorityFixture = crate::os_pack::json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🛡️artifact-authority.json")).expect("artifact authority fixture decodes");
    assert_eq!(hex_lower(&descriptor_digest_encoding_v1(&fixture.descriptor).expect("descriptor encodes")), fixture.descriptor_encoding_hex);
    assert_eq!(descriptor_digest_v1(&fixture.descriptor).expect("descriptor hashes"), fixture.descriptor_digest_v1);
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
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🔣️.json")).expect("neutral exact integer corpus");
    let fixture: DocumentOpenPlanFixture = crate::os_pack::json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).unwrap();
    let intent_json = crate::os_pack::json::to_json_string(&fixture.intent);
    assert!(intent_json.contains("\"version\":1"));
    for row in corpus["raw"].as_array().unwrap() {
        let raw = row.as_str().unwrap();
        macro_rules! check {
                ($($ty:ty),+ $(,)?) => { $(
                    let reference = serde_json::from_str::<$ty>(raw);
                    let ours = crate::os_pack::json::from_json_str::<$ty>(raw);
                    assert_eq!(ours.is_ok(), reference.is_ok(), "JSON admission {} {raw}", stringify!($ty));
                    if let (Ok(ours), Ok(reference)) = (ours, reference) {
                        assert_eq!(ours, reference, "JSON exact {} {raw}", stringify!($ty));
                    }
                )+ };
            }
        check!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
        let version = serde_json::from_str::<u32>(raw);
        let intent = crate::os_pack::json::from_json_str::<DocumentOpenIntentV1>(&intent_json.replace("\"version\":1", &format!("\"version\":{raw}")));
        assert_eq!(intent.is_ok(), version.is_ok(), "intent integer {raw}");
        if let (Ok(intent), Ok(version)) = (intent, version) {
            assert_eq!(intent.version, version);
            assert_eq!(intent.validate().is_ok(), version == 1, "intent version authority {raw}");
        }
        let expiry = serde_json::from_str::<i64>(raw);
        let grant_json = format!(r#"{{"schema":"fixture","protocol":"fixture","grant":"fixture","actorId":"fixture","expiresAtMs":{raw}}}"#);
        let grant = crate::os_pack::json::from_json_str::<crate::os_directory::client::SocketGrantReceiptV1>(&grant_json);
        assert_eq!(grant.is_ok(), expiry.is_ok(), "socket expiry integer {raw}");
        if let (Ok(grant), Ok(expiry)) = (grant, expiry) {
            assert_eq!(grant.expires_at_ms, expiry, "socket expiry exact {raw}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn document_open_plan_v1_matches_language_neutral_fixture() {
    let fixture: DocumentOpenPlanFixture = crate::os_pack::json::from_json_str(include_str!("../../../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture decodes");
    assert_eq!(hex_lower(&descriptor_digest_v1(&fixture.descriptor).expect("descriptor hashes").0), fixture.descriptor_digest_v1);
    assert_eq!(fixture.intent.validate(), Ok(()));
    assert_eq!(fixture.valid_plan.validate(fixture.now_ms), Ok(()));
    assert_eq!(fixture.valid_plan.parent_dialect.artifact_kind, fixture.valid_plan.artifact.kind);
    assert_eq!(fixture.exchange_intent.validate(), Ok(()));

    let mut overlong = fixture.valid_plan.clone();
    overlong.expires_at_unix_ms = fixture.now_ms + DOCUMENT_OPEN_PLAN_MAX_TTL_MS + 1;
    assert_eq!(overlong.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let encoded = crate::os_pack::json::to_json_string(&fixture.valid_plan);
    let forged = format!("{},\"actor\":\"caller-selected\"}}", encoded.strip_suffix('}').expect("object"));
    assert!(crate::os_pack::json::from_json_str::<DocumentOpenPlanV1>(&forged).is_err());
    let nested_scope = encoded.replace("\"documentId\":\"plan:\u{6771}\u{4eac}\"", "\"documentId\":\"plan:\u{6771}\u{4eac}\",\"actor\":\"caller-selected\"");
    assert!(crate::os_pack::json::from_json_str::<DocumentOpenPlanV1>(&nested_scope).is_err());
    let nested_frontier = encoded.replace("\"headEditOrdinal\":2", "\"headEditOrdinal\":2,\"storageKey\":\"private\"");
    assert!(crate::os_pack::json::from_json_str::<DocumentOpenPlanV1>(&nested_frontier).is_err());

    let mut unicode_control = fixture.valid_plan.clone();
    unicode_control.surface.app_id = "app.\u{85}hidden".into();
    assert_eq!(unicode_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut parent_kind = fixture.valid_plan.clone();
    parent_kind.parent_dialect.artifact_kind = "s.foreign.document".into();
    assert_eq!(parent_kind.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
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
