use super::*;
use crate::directory::model::{AgentDelegationRecord, SpaceRole};
use crate::directory::{AgentDelegationCapability, AgentSessionPreflight, agent_session_preflight, prepare_agent_delegation};

fn delegation(audience: AgentAudience, now: i64, ttl_secs: i64) -> (AgentDelegationRecord, AgentDelegationCapability) {
    let issued = prepare_agent_delegation("space-a", "usr_ada", "Drafting agent", audience, ttl_secs, now).expect("a bounded delegation mints");
    (issued.record, issued.capability)
}

#[test]
fn a_create_request_is_admitted_exactly_at_the_schema_bounds() {
    let base = |label: &str, ttl: i64| CreateAgentDelegationRequestV1 { schema: AGENT_DELEGATION_CREATE_SCHEMA.into(), space_id: "space-a".into(), agent_label: label.into(), audience: AgentAudience::Edit, ttl_secs: ttl };
    assert!(base("Drafting agent", DEFAULT_DELEGATION_TTL_SECS).verify().is_ok());
    assert!(base(&"a".repeat(crate::directory::AGENT_LABEL_MAX_BYTES), MIN_DELEGATION_TTL_SECS).verify().is_ok());
    assert!(base(&"a".repeat(crate::directory::AGENT_LABEL_MAX_BYTES + 1), MIN_DELEGATION_TTL_SECS).verify().is_err());
    assert!(base("", DEFAULT_DELEGATION_TTL_SECS).verify().is_err());
    assert!(base("ok\u{7}bell", DEFAULT_DELEGATION_TTL_SECS).verify().is_err());
    assert!(base("ok", MIN_DELEGATION_TTL_SECS - 1).verify().is_err());
    assert!(base("ok", MAX_DELEGATION_TTL_SECS + 1).verify().is_err());
    let mut wrong_schema = base("ok", DEFAULT_DELEGATION_TTL_SECS);
    wrong_schema.schema = "semio.hub.auth.credential-sign-in/v1".into();
    assert_eq!(wrong_schema.verify().unwrap_err(), AgentErrorCodeV1::MalformedRequest);
}

#[test]
fn an_unknown_field_is_refused_by_the_decoder_itself() {
    let body = r#"{"schema":"semio.hub.auth.agent-delegation-create/v1","spaceId":"space-a","agentLabel":"a","audience":"edit","ttlSecs":3600,"extra":1}"#;
    assert!(serde_json::from_str::<CreateAgentDelegationRequestV1>(body).is_err());
    let session = r#"{"schema":"semio.hub.auth.agent-session/v1","audience":"edit","agentInstanceId":"a","extra":1}"#;
    assert!(serde_json::from_str::<AgentSessionRequestV1>(session).is_err());
}

#[test]
fn the_agent_instance_id_charset_matches_the_device_instance_id_charset() {
    let request = |instance: &str| AgentSessionRequestV1 { schema: AGENT_SESSION_REQUEST_SCHEMA.into(), audience: AgentAudience::Read, agent_instance_id: instance.into() };
    assert!(request("mcp.1:a-b_c").verify().is_ok());
    assert!(request("").verify().is_err());
    assert!(request(&"a".repeat(129)).verify().is_err());
    assert!(request("has space").verify().is_err());
}

#[test]
fn an_agent_principal_is_structurally_distinct_from_the_delegating_human() {
    let principal = agent_principal_id("del-1");
    assert_eq!(principal, "agent:del-1");
    assert!(is_agent_principal(&principal));
    assert!(!is_agent_principal("user:usr_ada#sess-1"));
    assert_ne!(principal, "usr_ada");
}

#[test]
fn a_live_delegation_mints_and_every_wrong_credential_is_one_refusal() {
    let now = 1_700_000_000_000;
    let (record, capability) = delegation(AgentAudience::Edit, now, 3600);
    assert_eq!(agent_session_preflight(Some(&record), &capability, AgentAudience::Edit, now + 1), AgentSessionPreflight::Mint);
    assert_eq!(decide_agent_session(AgentSessionPreflight::Mint), AgentSessionDecisionV1::Mint { ttl_secs: AGENT_SESSION_TTL_SECS });

    let (_, other) = delegation(AgentAudience::Edit, now, 3600);
    for refused in [
        agent_session_preflight(None, &capability, AgentAudience::Edit, now + 1),
        agent_session_preflight(Some(&record), &other, AgentAudience::Edit, now + 1),
        agent_session_preflight(Some(&record), &capability, AgentAudience::Read, now + 1),
    ] {
        assert_eq!(refused, AgentSessionPreflight::Denied, "a wrong delegation, a wrong secret and a widened audience are all one refusal");
        assert_eq!(decide_agent_session(refused), AgentSessionDecisionV1::Refuse(AgentErrorCodeV1::InvalidDelegation));
    }
}

#[test]
fn an_agent_can_never_widen_its_own_audience_at_exchange_time() {
    let now = 1_700_000_000_000;
    let (record, capability) = delegation(AgentAudience::Read, now, 3600);
    assert_eq!(agent_session_preflight(Some(&record), &capability, AgentAudience::Read, now + 1), AgentSessionPreflight::Mint);
    assert_eq!(agent_session_preflight(Some(&record), &capability, AgentAudience::Edit, now + 1), AgentSessionPreflight::Denied);
}

#[test]
fn revocation_and_expiry_are_reported_only_to_a_caller_that_proved_the_secret() {
    let now = 1_700_000_000_000;
    let (mut record, capability) = delegation(AgentAudience::Edit, now, 3600);
    record.revoked_at = Some(now + 10);
    assert_eq!(agent_session_preflight(Some(&record), &capability, AgentAudience::Edit, now + 20), AgentSessionPreflight::Revoked);
    assert_eq!(decide_agent_session(AgentSessionPreflight::Revoked), AgentSessionDecisionV1::Refuse(AgentErrorCodeV1::DelegationRevoked));

    let (expired, expired_capability) = delegation(AgentAudience::Edit, now, 60);
    assert_eq!(agent_session_preflight(Some(&expired), &expired_capability, AgentAudience::Edit, expired.expires_at), AgentSessionPreflight::Expired);
    assert_eq!(decide_agent_session(AgentSessionPreflight::Expired), AgentSessionDecisionV1::Refuse(AgentErrorCodeV1::DelegationExpired));
}

#[test]
fn an_agent_audience_never_exceeds_author_and_read_never_exceeds_spectator() {
    assert_eq!(AgentAudience::Read.space_role(), SpaceRole::Spectator);
    assert_eq!(AgentAudience::Edit.space_role(), SpaceRole::Author);
    assert!(AgentAudience::ALL.iter().all(|audience| matches!(audience.space_role(), SpaceRole::Author | SpaceRole::Spectator)));
}

#[test]
fn every_error_code_carries_exactly_one_status_and_one_wire_spelling() {
    for (code, status, spelling) in [
        (AgentErrorCodeV1::MalformedRequest, 400, "malformed-request"),
        (AgentErrorCodeV1::InvalidDelegation, 401, "invalid-delegation"),
        (AgentErrorCodeV1::DelegationRevoked, 403, "delegation-revoked"),
        (AgentErrorCodeV1::DelegationExpired, 403, "delegation-expired"),
        (AgentErrorCodeV1::Forbidden, 403, "forbidden"),
        (AgentErrorCodeV1::RateLimited, 429, "rate-limited"),
        (AgentErrorCodeV1::DirectoryUnavailable, 503, "directory-unavailable"),
    ] {
        assert_eq!(code.status(), status);
        assert_eq!(code.as_str(), spelling);
        assert_eq!(serde_json::to_string(&code).expect("code serializes"), format!("\"{spelling}\""));
    }
    let body = serde_json::to_value(AgentErrorV1::new(AgentErrorCodeV1::InvalidDelegation)).expect("error body serializes");
    assert_eq!(body, serde_json::json!({ "schema": AGENT_ERROR_SCHEMA, "error": "invalid-delegation" }));
}

#[test]
fn a_delegation_receipt_exposes_the_token_once_and_a_summary_never_does() {
    let now = 1_700_000_000_000;
    let issued = prepare_agent_delegation("space-a", "usr_ada", "Drafting agent", AgentAudience::Edit, 3600, now).expect("a bounded delegation mints");
    let token = issued.capability.expose_once();
    assert!(token.starts_with("delegation.v1."), "the delegation capability has its own prefix: {token}");
    assert_ne!(token, issued.record.selector, "the token is never the selector alone");

    let summary = AgentDelegationSummaryV1::from_row(crate::directory::model::AgentDelegationRow {
        delegation_id: issued.record.id.clone(),
        space_id: issued.record.space_id.clone(),
        delegating_user_id: issued.record.delegating_user_id.clone(),
        agent_label: issued.record.agent_label.clone(),
        audience: issued.record.audience,
        created_at_ms: issued.record.created_at,
        expires_at_ms: issued.record.expires_at,
        revoked: false,
        last_used_at_ms: None,
    });
    let encoded = serde_json::to_string(&summary).expect("summary serializes");
    assert!(!encoded.contains(&token), "a listing never carries the token");
    assert!(!encoded.contains(&issued.record.selector), "a listing never carries the selector");
    assert_eq!(summary.agent_principal_id, agent_principal_id(&issued.record.id));
}

const SCHEMA_MODULE: &str = include_str!("../../🧬️schema/🔣️.json");

/// 📥 Compiles one `$defs` export of the `hub.auth` module through the owned draft-07 validator —
/// the same helper the credential laws use, so the agent scope is held to the same authority.
fn structural(export: &str) -> semio_framework_schema::OwnedJsonSchemaValidator {
    let mut document: serde_json::Value = serde_json::from_str(SCHEMA_MODULE).expect("hub.auth schema module");
    document["$ref"] = serde_json::Value::String(format!("#/$defs/{export}"));
    semio_framework_schema::OwnedJsonSchemaValidator::compile(&document.to_string()).unwrap_or_else(|error| panic!("{export} does not compile: {error:?}"))
}

/// 🧬️ The agent scope's Rust decoders and its JSON Schema admit EXACTLY the same wire. Without this
/// the `$defs` would be documentation; with it they are authority, and the decoder cannot drift.
#[test]
fn the_agent_decoders_and_the_json_schema_admit_exactly_the_same_wire() {
    let create = serde_json::json!({ "schema": AGENT_DELEGATION_CREATE_SCHEMA, "spaceId": "space-a", "agentLabel": "Drafting agent", "audience": "edit", "ttlSecs": DEFAULT_DELEGATION_TTL_SECS });
    let create_schema = structural("CreateAgentDelegationRequestV1");
    assert!(create_schema.is_valid_json(&create.to_string()));
    assert!(serde_json::from_value::<CreateAgentDelegationRequestV1>(create).expect("decode").verify().is_ok());
    for (name, body) in [
        ("unknown field", serde_json::json!({ "schema": AGENT_DELEGATION_CREATE_SCHEMA, "spaceId": "space-a", "agentLabel": "a", "audience": "edit", "ttlSecs": 3600, "extra": 1 })),
        ("empty label", serde_json::json!({ "schema": AGENT_DELEGATION_CREATE_SCHEMA, "spaceId": "space-a", "agentLabel": "", "audience": "edit", "ttlSecs": 3600 })),
        ("label over 64", serde_json::json!({ "schema": AGENT_DELEGATION_CREATE_SCHEMA, "spaceId": "space-a", "agentLabel": "a".repeat(65), "audience": "edit", "ttlSecs": 3600 })),
        ("administrative audience", serde_json::json!({ "schema": AGENT_DELEGATION_CREATE_SCHEMA, "spaceId": "space-a", "agentLabel": "a", "audience": "admin", "ttlSecs": 3600 })),
        ("ttl under floor", serde_json::json!({ "schema": AGENT_DELEGATION_CREATE_SCHEMA, "spaceId": "space-a", "agentLabel": "a", "audience": "edit", "ttlSecs": MIN_DELEGATION_TTL_SECS - 1 })),
        ("ttl over ceiling", serde_json::json!({ "schema": AGENT_DELEGATION_CREATE_SCHEMA, "spaceId": "space-a", "agentLabel": "a", "audience": "edit", "ttlSecs": MAX_DELEGATION_TTL_SECS + 1 })),
        ("wrong schema", serde_json::json!({ "schema": "semio.hub.auth.agent-delegation-create/v2", "spaceId": "space-a", "agentLabel": "a", "audience": "edit", "ttlSecs": 3600 })),
    ] {
        assert!(!create_schema.is_valid_json(&body.to_string()), "{name} must fail the schema");
        let decoded = serde_json::from_value::<CreateAgentDelegationRequestV1>(body).map_err(|_| AgentErrorCodeV1::MalformedRequest).and_then(CreateAgentDelegationRequestV1::verify);
        assert!(decoded.is_err(), "{name} must fail the Rust decoder");
    }

    let session_schema = structural("AgentSessionRequestV1");
    let session = serde_json::json!({ "schema": AGENT_SESSION_REQUEST_SCHEMA, "audience": "read", "agentInstanceId": "mcp.1" });
    assert!(session_schema.is_valid_json(&session.to_string()));
    assert!(serde_json::from_value::<AgentSessionRequestV1>(session).expect("decode").verify().is_ok());
    for (name, body) in [
        ("spaced instance", serde_json::json!({ "schema": AGENT_SESSION_REQUEST_SCHEMA, "audience": "read", "agentInstanceId": "mcp 1" })),
        ("empty instance", serde_json::json!({ "schema": AGENT_SESSION_REQUEST_SCHEMA, "audience": "read", "agentInstanceId": "" })),
        ("unknown field", serde_json::json!({ "schema": AGENT_SESSION_REQUEST_SCHEMA, "audience": "read", "agentInstanceId": "mcp.1", "extra": 1 })),
    ] {
        assert!(!session_schema.is_valid_json(&body.to_string()), "{name} must fail the schema");
        let decoded = serde_json::from_value::<AgentSessionRequestV1>(body).map_err(|_| AgentErrorCodeV1::MalformedRequest).and_then(|request| request.verify());
        assert!(decoded.is_err(), "{name} must fail the Rust decoder");
    }

    let issued = prepare_agent_delegation("space-a", "usr_ada", "Drafting agent", AgentAudience::Edit, 3600, 1_700_000_000_000).expect("delegation mints");
    let receipt = AgentDelegationReceiptV1 {
        schema: AGENT_DELEGATION_RECEIPT_SCHEMA.into(),
        agent_principal_id: agent_principal_id(&issued.record.id),
        delegation_id: issued.record.id.clone(),
        agent_label: issued.record.agent_label.clone(),
        space_id: issued.record.space_id.clone(),
        audience: issued.record.audience,
        expires_at_ms: issued.record.expires_at,
        token: issued.capability.expose_once(),
    };
    assert!(structural("AgentDelegationReceiptV1").is_valid_json(&serde_json::to_string(&receipt).expect("receipt json")));
    assert!(structural("AgentDelegationCapabilityV1").is_valid_json(&serde_json::to_string(&receipt.token).expect("token json")));
    assert!(!structural("AgentDelegationCapabilityV1").is_valid_json(&serde_json::to_string(&format!("session.v1.{}.{}", "0".repeat(32), "0".repeat(64))).expect("token json")), "a session capability is not a delegation");

    let listing = AgentDelegationListV1::new(vec![AgentDelegationSummaryV1::from_row(crate::directory::model::AgentDelegationRow {
        delegation_id: issued.record.id.clone(),
        space_id: issued.record.space_id.clone(),
        delegating_user_id: issued.record.delegating_user_id.clone(),
        agent_label: issued.record.agent_label.clone(),
        audience: issued.record.audience,
        created_at_ms: issued.record.created_at,
        expires_at_ms: issued.record.expires_at,
        revoked: true,
        last_used_at_ms: Some(issued.record.created_at + 1),
    })]);
    assert!(structural("AgentDelegationListV1").is_valid_json(&serde_json::to_string(&listing).expect("listing json")));

    let error_schema = structural("AgentErrorV1");
    assert!(error_schema.is_valid_json(&serde_json::to_string(&AgentErrorV1::new(AgentErrorCodeV1::DelegationRevoked)).expect("error json")));

    // 🧾️ Every fact kind this scope appends is declared in the module's durable-event enum.
    let kinds = structural("AuthDurableEventKindV1");
    for kind in [crate::directory::AGENT_DELEGATED_EVENT, crate::directory::AGENT_DELEGATION_REVOKED_EVENT, crate::directory::AGENT_SESSION_ISSUED_EVENT] {
        assert!(kinds.is_valid_json(&serde_json::to_string(kind).expect("kind json")), "{kind} must be a declared durable event kind");
    }

    // 🤖️ The credential FILE the MCP reads is the same schema authority, not a second dialect.
    let credential_schema = structural("AgentCredentialFileV1");
    let credential = serde_json::json!({ "schema": "semio.hub.agent-credential/v1", "hubOrigin": "http://127.0.0.1:7501", "spaceId": "space-a", "audience": "edit", "token": receipt.token });
    assert!(credential_schema.is_valid_json(&credential.to_string()));
    assert!(!credential_schema.is_valid_json(&serde_json::json!({ "schema": "semio.hub.agent-credential/v1", "hubOrigin": "http://127.0.0.1:7501", "spaceId": "space-a", "audience": "admin", "token": receipt.token }).to_string()));
}
