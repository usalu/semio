
use super::*;

#[test]
fn authenticated_hub_workspace_cli_contains_no_hub_credential_carrier() {
    let stdio = parse_stdio_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a"].into_iter().map(str::to_string)).unwrap();
    assert_eq!(stdio.hub, Some(HubOptions { base_url: "http://127.0.0.1:8787".into(), space_id: "space-a".into() }));
    assert!(parse_stdio_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a", "--token", "forbidden"].into_iter().map(str::to_string)).is_err());

    let options = parse_http_args(&mut ["--hub", "http://127.0.0.1:8787", "--space", "space-a"].into_iter().map(str::to_string)).unwrap();
    let hub = options.hub.expect("authenticated hub binding");
    assert_eq!(hub, HubOptions { base_url: "http://127.0.0.1:8787".into(), space_id: "space-a".into() });
    assert!(parse_http_args(&mut ["--token", "forbidden"].into_iter().map(str::to_string)).is_err());
    assert!(parse_http_args(&mut ["--bridge-token-file", "forbidden"].into_iter().map(str::to_string)).is_err());
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
