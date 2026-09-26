
use super::*;

#[test]
fn authenticated_hub_workspace_cli_contains_no_hub_credential_carrier() {
    let stdio = parse_stdio_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a"].into_iter().map(str::to_string)).unwrap();
    assert_eq!(stdio.hub, Some(HubOptions { base_url: "http://127.0.0.1:8787".into(), space_id: "space-a".into(), credential: None }));
    assert!(parse_stdio_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a", "--token", "forbidden"].into_iter().map(str::to_string)).is_err());

    let options = parse_http_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a"].into_iter().map(str::to_string)).unwrap();
    let hub = options.hub.expect("authenticated hub binding");
    assert_eq!(hub, HubOptions { base_url: "http://127.0.0.1:8787".into(), space_id: "space-a".into(), credential: None });
    assert!(parse_http_args(&mut ["--token", "forbidden"].into_iter().map(str::to_string)).is_err());
    assert!(parse_http_args(&mut ["--bridge-token-file", "forbidden"].into_iter().map(str::to_string)).is_err());
}

/// 🤖️ The delegated-agent flags carry a LOCATION, never a secret: `--credential-file` is the only
/// shape an MCP client configuration can express, and argv is world-readable through `ps`, so a
/// token-shaped flag stays rejected.
///
/// 🔢️ `http` mode does not use stdio for MCP framing, so descriptor 0 is admissible there.
#[test]
fn the_delegated_agent_credential_is_a_path_and_never_a_secret_in_argv() {
    let stdio = parse_stdio_args(&mut ["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-file", "/home/ada/.semio/agent.json"].into_iter().map(str::to_string)).unwrap();
    assert_eq!(
        stdio.hub,
        Some(HubOptions { base_url: "http://127.0.0.1:7501".into(), space_id: "space-a".into(), credential: Some(AgentCredentialSource::File("/home/ada/.semio/agent.json".into())) })
    );
    assert!(format!("{:?}", stdio.hub).contains("agent.json"), "the path is printable — it is not the secret");

    let http = parse_http_args(&mut ["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-fd", "5"].into_iter().map(str::to_string)).unwrap();
    assert_eq!(http.hub.expect("hub binding").credential, Some(AgentCredentialSource::Descriptor(5)));

    for refused in [
        vec!["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-file", "/a", "--credential-fd", "5"],
        vec!["--credential-file", "/a"],
        vec!["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-token", "delegation.v1.deadbeef"],
        vec!["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-fd", "3"],
        vec!["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-fd", "0"],
        vec!["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-fd", "not-a-number"],
    ] {
        assert!(parse_stdio_args(&mut refused.clone().into_iter().map(str::to_string)).is_err(), "{refused:?} must be refused");
    }

    assert!(parse_http_args(&mut ["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-fd", "0"].into_iter().map(str::to_string)).is_ok());
    assert!(parse_http_args(&mut ["--hub", "http://127.0.0.1:7501", "--space", "space-a", "--credential-fd", "3"].into_iter().map(str::to_string)).is_err());
}

#[test]
fn the_process_entry_seal_rejects_hub_carriers_and_admits_host_harness_variables() {
    for carrier in ["S_USER", "VITE_S_USER", "S_HUB_URL", "VITE_S_HUB_URL", "S_SESSION", "S_BRIDGE_TOKEN", "VITE_S_CAPABILITY_GRANT", "AUTHORIZATION", "COOKIE"] {
        assert!(protected_credential_environment_name(carrier), "{carrier} carries hub authority and must fail the process-entry seal");
    }
    for benign in [
        "CLAUDE_CODE_SESSION_ID",
        "CLAUDE_CODE_HOST_SESSION_ID",
        "CLAUDE_CODE_MESSAGING_TOKEN",
        "CLAUDE_CODE_SESSION_ATTENDED",
        "VSCODE_SESSION_ID",
        "GH_TOKEN",
        "NPM_TOKEN",
        "SESSION_MANAGER",
        "SEMIO_DIRECT_CHILD_BENIGN",
        "SEMIO_PLAYGROUND_SESSION_OUTPUT_ROOT",
        "S_DATA_DIR",
        "S_OS_MCP_PORT",
        "PATH",
    ] {
        assert!(!protected_credential_environment_name(benign), "{benign} carries no hub credential and must not fail the process-entry seal");
    }
}

//#region 🔖️AutoApprove
// 🎫️ slice M4 (audit §6 P0.2 item 4): `AutoApprovePolicy` existed as a type with no way to reach it
// from the command line, so every live server hardcoded `Never` and a destructive capability was
// permanently unapprovable. These assert the real argv surface.
#[test]
fn auto_approve_reaches_both_transports_from_argv() {
    let stdio = parse_stdio_args(&mut ["--auto-approve", "readonly"].into_iter().map(str::to_string)).unwrap();
    assert_eq!(stdio.auto_approve, AutoApprovePolicy::ReadonlyOnly);
    let http = parse_http_args(&mut ["--auto-approve", "all"].into_iter().map(str::to_string)).unwrap();
    assert_eq!(http.auto_approve, AutoApprovePolicy::All);
}

#[test]
fn auto_approve_defaults_to_never_and_rejects_an_unknown_policy() {
    assert_eq!(parse_stdio_args(&mut std::iter::empty()).unwrap().auto_approve, AutoApprovePolicy::Never);
    assert_eq!(parse_http_args(&mut std::iter::empty()).unwrap().auto_approve, AutoApprovePolicy::Never);
    let rejected = parse_stdio_args(&mut ["--auto-approve", "sometimes"].into_iter().map(str::to_string)).unwrap_err();
    assert!(rejected.contains("never|readonly|all"), "an unknown policy is an argv error, never a silent downgrade: {rejected}");
    assert!(parse_stdio_args(&mut ["--auto-approve"].into_iter().map(str::to_string)).is_err());
}

#[test]
fn the_stdio_bridge_can_be_opted_out_of_but_is_on_by_default() {
    assert!(!parse_stdio_args(&mut std::iter::empty()).unwrap().no_bridge);
    assert!(parse_stdio_args(&mut ["--no-bridge"].into_iter().map(str::to_string)).unwrap().no_bridge);
    assert!(parse_http_args(&mut ["--no-bridge"].into_iter().map(str::to_string)).is_err(), "--no-bridge is stdio-only: http always serves /bridge on its own socket");
}
//#endregion 🔖️AutoApprove
