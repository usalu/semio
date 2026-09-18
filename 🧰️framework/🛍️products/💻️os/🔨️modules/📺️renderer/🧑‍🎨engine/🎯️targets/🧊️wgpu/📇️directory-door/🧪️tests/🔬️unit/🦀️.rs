//! 🧪️ The directory door's wire contract, proved without a browser: the request every target
//! encodes, the answer shapes the page may hand back, and the source law that the lanes this seam
//! exists for are no longer compiled out of the browser build.

use super::{decode_directory_door_response, encode_directory_door_request, DIRECTORY_DOOR_OP};
use semio_framework_os_kernel::os_directory::client::{HttpMethod, TransportError};

const SHELL_SOURCE: &str = include_str!("../../../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");

//#region 🧪️Wire
#[test]
fn a_get_request_carries_the_op_verb_url_and_no_body() {
    let encoded = encode_directory_door_request(HttpMethod::Get, "https://hub.example/directory/spaces/space-a", None, None).expect("encodes");
    let value: serde_json::Value = serde_json::from_str(&encoded).expect("valid json");
    assert_eq!(value["op"], DIRECTORY_DOOR_OP);
    assert_eq!(value["method"], "GET");
    assert_eq!(value["url"], "https://hub.example/directory/spaces/space-a");
    assert!(value.get("bearer").is_none(), "an unauthenticated browser call sends no Authorization header");
    assert!(value.get("body").is_none(), "a GET carries no body field at all");
}

#[test]
fn a_command_post_carries_its_sealed_json_body_and_bearer_verbatim() {
    let body = br#"{"schema":"semio.directory.command-request.v1"}"#;
    let encoded = encode_directory_door_request(HttpMethod::Post, "https://hub.example/directory/commands", Some("session.v1.abc"), Some(body)).expect("encodes");
    let value: serde_json::Value = serde_json::from_str(&encoded).expect("valid json");
    assert_eq!(value["method"], "POST");
    assert_eq!(value["bearer"], "session.v1.abc");
    assert_eq!(value["body"], String::from_utf8_lossy(body).as_ref());
}

#[test]
fn a_delete_verb_spells_the_fetch_vocabulary() {
    let encoded = encode_directory_door_request(HttpMethod::Delete, "https://hub.example/directory/spaces/space-a", None, None).expect("encodes");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).expect("valid json")["method"], "DELETE");
}

#[test]
fn a_non_utf8_body_is_refused_rather_than_transcoded() {
    let error = encode_directory_door_request(HttpMethod::Post, "https://hub.example/directory/commands", None, Some(&[0xff, 0xfe])).expect_err("refuses");
    assert!(matches!(error, TransportError::Io(_)));
}

#[test]
fn an_answer_decodes_its_status_and_body_bytes() {
    let response = decode_directory_door_response(r#"{"status":200,"body":"{\"schema\":\"x\"}"}"#).expect("decodes");
    assert_eq!(response.status, 200);
    assert_eq!(String::from_utf8(response.body).expect("utf-8"), r#"{"schema":"x"}"#);
}

#[test]
fn a_status_without_a_body_decodes_as_empty_not_as_a_failure() {
    let response = decode_directory_door_response(r#"{"status":204}"#).expect("decodes");
    assert_eq!(response.status, 204);
    assert!(response.body.is_empty());
}

#[test]
fn a_server_error_status_stays_a_response_so_the_client_can_retry_it() {
    let response = decode_directory_door_response(r#"{"status":503,"body":""}"#).expect("decodes");
    assert_eq!(response.status, 503, "a reachable hub answering 5xx is a response, never a transport fault");
}

#[test]
fn a_fetch_refusal_decodes_as_a_transport_fault_not_a_synthetic_status() {
    let error = decode_directory_door_response(r#"{"error":"NetworkError"}"#).expect_err("refuses");
    assert_eq!(error, TransportError::Io("NetworkError".into()));
}

#[test]
fn an_answer_with_neither_status_nor_error_is_refused() {
    assert!(decode_directory_door_response("{}").is_err());
}

#[test]
fn an_unreadable_answer_is_refused() {
    assert!(decode_directory_door_response("not json").is_err());
}

#[test]
fn a_round_trip_preserves_every_field() {
    let encoded = encode_directory_door_request(HttpMethod::Post, "https://hub.example/directory/commands", Some("bearer"), Some(b"{}")).expect("encodes");
    let decoded: super::DirectoryDoorRequestV1 = serde_json::from_str(&encoded).expect("decodes");
    assert_eq!(decoded, super::DirectoryDoorRequestV1 { op: DIRECTORY_DOOR_OP.into(), method: "POST".into(), url: "https://hub.example/directory/commands".into(), bearer: Some("bearer".into()), body: Some("{}".into()) });
}
//#endregion 🧪️Wire

//#region ⚖️BrowserParityLaw
/// ⚖️ The law this packet exists to hold: the Space-Administration region, the pure check-in
/// reducers, and the directory command FIFO must be compiled into the BROWSER build too. A
/// `#[cfg(not(target_arch = "wasm32"))]` anywhere inside them is exactly the regression that made
/// this chrome native-only, and it is invisible to a native `cargo check`.
fn region_of(source: &str, region: &str) -> String {
    let start = source.find(&format!("//#region {region}")).unwrap_or_else(|| panic!("region {region} is present"));
    let end = source[start..].find(&format!("//#endregion {region}")).unwrap_or_else(|| panic!("region {region} is closed"));
    source[start..start + end].to_string()
}

#[test]
fn the_space_administration_region_compiles_on_the_browser_target() {
    assert!(!region_of(SHELL_SOURCE, "🏛️SpaceAdministration").contains(r#"cfg(not(target_arch = "wasm32"))"#), "Space Administration is gated out of the browser build again");
}

#[test]
fn the_pure_check_in_reducers_compile_on_the_browser_target() {
    assert!(!region_of(SHELL_SOURCE, "🔖️CheckInPure").contains(r#"cfg(not(target_arch = "wasm32"))"#), "the check-in reducers are gated out of the browser build again");
}

#[test]
fn the_directory_command_queue_compiles_on_the_browser_target() {
    assert!(!region_of(SHELL_SOURCE, "🎮️DirectoryCommandQueue").contains(r#"cfg(not(target_arch = "wasm32"))"#), "the directory command FIFO is gated out of the browser build again");
}

/// ⚖️ The shared lane may still branch INSIDE a method on the two halves the browser has no door for
/// (the opening relay's document backbone), and it may carry a cfg PAIR where the two targets read a
/// different platform boundary (the deadline clock). What it may never do again is gate a method out
/// with no browser answer at all, which is what made identity, the command FIFO and
/// `pump_space_administration` native-only in the first place.
#[test]
fn no_method_of_the_shared_directory_lane_is_left_without_a_browser_answer() {
    let lane = region_of(SHELL_SOURCE, "📇️DirectoryLane");
    let signature = |tail: &str| -> Option<String> {
        let head = tail.trim_start().lines().next()?;
        head.split_once('(').map(|(name, _)| name.trim_start_matches("pub ").trim_start_matches("async ").trim_start_matches("fn ").to_string())
    };
    let twins: Vec<String> = lane.split(r#"#[cfg(target_arch = "wasm32")]"#).skip(1).filter_map(signature).collect();
    let orphans: Vec<String> = lane
        .split(r#"#[cfg(not(target_arch = "wasm32"))]"#)
        .skip(1)
        .filter(|tail| {
            let head = tail.trim_start();
            head.starts_with("fn ") || head.starts_with("pub fn ") || head.starts_with("async fn ") || head.starts_with("pub async fn ")
        })
        .filter_map(signature)
        .filter(|name| !twins.contains(name))
        .collect();
    assert!(orphans.is_empty(), "the shared directory lane gates methods out of the browser build with no wasm32 twin: {orphans:?}");
}

#[test]
fn the_shared_directory_lane_declares_the_space_administration_entry_points() {
    let lane = region_of(SHELL_SOURCE, "📇️DirectoryLane");
    for entry in ["pub fn open_space_administration(", "pub fn close_space_administration(", "pub fn acknowledge_space_administration_capability(", "pub async fn pump_space_administration(", "async fn dispatch_directory_command(", "async fn flush_pending_directory_commands("] {
        assert!(lane.contains(entry), "{entry} left the shared directory lane");
    }
}
//#endregion ⚖️BrowserParityLaw
