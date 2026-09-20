use super::*;

fn token() -> String {
    format!("delegation.v1.{}.{}", "0123456789abcdef".repeat(2), "0123456789abcdef".repeat(4))
}

fn document(overrides: &[(&str, serde_json::Value)]) -> Vec<u8> {
    let mut value = serde_json::json!({
        "schema": AGENT_CREDENTIAL_SCHEMA,
        "hubOrigin": "http://127.0.0.1:7501",
        "spaceId": "space-a",
        "audience": "edit",
        "token": token(),
    });
    for (key, replacement) in overrides {
        value[*key] = replacement.clone();
    }
    serde_json::to_vec(&value).expect("the credential fixture serializes")
}

#[test]
fn a_well_formed_credential_decodes_and_never_prints_its_token() {
    let credential = AgentCredentialV1::decode(&document(&[])).expect("a well-formed credential decodes");
    assert_eq!(credential.hub_origin(), "http://127.0.0.1:7501");
    assert_eq!(credential.space_id(), "space-a");
    assert_eq!(credential.audience(), "edit");
    assert_eq!(credential.expose_for_exchange(), token());
    let printed = format!("{credential:?}");
    assert!(printed.contains("<redacted>"), "Debug redacts: {printed}");
    assert!(!printed.contains(&token()), "Debug never prints the token: {printed}");
}

#[test]
fn every_malformed_credential_is_refused_at_startup() {
    for (what, overrides) in [
        ("wrong schema", vec![("schema", serde_json::json!("semio.local.consumer-credential/v1"))]),
        ("empty origin", vec![("hubOrigin", serde_json::json!(""))]),
        ("empty space", vec![("spaceId", serde_json::json!(""))]),
        ("unknown audience", vec![("audience", serde_json::json!("admin"))]),
        ("session token in place of a delegation", vec![("token", serde_json::json!(format!("session.v1.{}.{}", "0".repeat(32), "0".repeat(64))))]),
        ("truncated token", vec![("token", serde_json::json!("delegation.v1.abc"))]),
        ("uppercase hex", vec![("token", serde_json::json!(format!("delegation.v1.{}.{}", "A".repeat(32), "0".repeat(64))))]),
        ("non-string token", vec![("token", serde_json::json!(7))]),
    ] {
        assert!(AgentCredentialV1::decode(&document(&overrides)).is_err(), "{what} must be refused");
    }
    assert!(AgentCredentialV1::decode(b"").is_err(), "an empty file is refused");
    assert!(AgentCredentialV1::decode(b"not json").is_err(), "a non-JSON file is refused");
}

#[test]
fn the_delegation_token_shape_is_exactly_the_capability_the_hub_mints() {
    assert!(valid_delegation_token(&token()));
    assert!(!valid_delegation_token(&format!("{}x", token())), "one byte too long is refused");
    assert!(!valid_delegation_token(&token()[..110]), "one byte too short is refused");
    assert!(!valid_delegation_token(&token().replace("delegation.v1.", "invite.v1.")), "another capability kind is refused");
}

#[cfg(unix)]
#[test]
fn a_group_or_world_readable_credential_file_is_refused() {
    use std::os::unix::fs::PermissionsExt;
    let directory = std::env::temp_dir().join(format!("m6-agent-credential-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join("credential.json");
    std::fs::write(&path, document(&[])).expect("credential written");

    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).expect("mode 644");
    let refused = AgentCredentialV1::read_file(&path).expect_err("a world-readable credential is refused");
    assert!(refused.message.contains("chmod 600"), "the refusal names the remedy: {}", refused.message);

    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).expect("mode 640");
    assert!(AgentCredentialV1::read_file(&path).is_err(), "a group-readable credential is refused");

    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("mode 600");
    let credential = AgentCredentialV1::read_file(&path).expect("a 0600 credential is admitted");
    assert_eq!(credential.space_id(), "space-a");

    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn the_exchange_request_body_is_the_hubs_own_schema() {
    let body = agent_session_request_body("edit", "mcp.instance-1");
    let decoded: serde_json::Value = serde_json::from_slice(&body).expect("the request body is JSON");
    assert_eq!(decoded, serde_json::json!({ "schema": "semio.hub.auth.agent-session/v1", "audience": "edit", "agentInstanceId": "mcp.instance-1" }));
}

#[test]
fn a_grant_decodes_and_a_refusal_never_reads_as_one() {
    let grant = serde_json::json!({
        "schema": "semio.hub.auth.agent-session/v1",
        "token": format!("session.v1.{}.{}", "0".repeat(32), "0".repeat(64)),
        "agentPrincipalId": "agent:del-1",
        "agentLabel": "Drafting agent",
        "spaceId": "space-a",
        "audience": "edit",
        "expiresAtMs": 1_700_000_000_000i64,
    });
    let decoded = decode_agent_session_grant(&serde_json::to_vec(&grant).expect("grant serializes")).expect("a grant decodes");
    assert_eq!(decoded.agent_principal_id, "agent:del-1");
    assert_eq!(decoded.space_id, "space-a");
    assert_eq!(decoded.expires_at_ms, 1_700_000_000_000);

    let refusal = serde_json::to_vec(&serde_json::json!({ "schema": "semio.hub.auth.agent-error/v1", "error": "delegation-revoked" })).expect("refusal serializes");
    assert!(decode_agent_session_grant(&refusal).is_err(), "a refusal body never decodes as a grant");
}

#[test]
fn every_hub_refusal_becomes_a_typed_gateway_error_that_names_the_code() {
    for (code, expected_retryable) in [("delegation-revoked", false), ("delegation-expired", false), ("forbidden", false), ("rate-limited", true), ("directory-unavailable", true)] {
        let body = serde_json::to_vec(&serde_json::json!({ "schema": "semio.hub.auth.agent-error/v1", "error": code })).expect("refusal serializes");
        let error = agent_exchange_error(403, &body);
        assert!(error.message.contains(code), "the operator reads the hub's own code: {}", error.message);
        assert_eq!(error.retryable, expected_retryable, "{code} retryability");
    }
    let opaque = agent_exchange_error(500, b"<html>");
    assert!(opaque.message.contains("http-500"), "an unparseable refusal still names its status: {}", opaque.message);
}
