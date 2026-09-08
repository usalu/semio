
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
