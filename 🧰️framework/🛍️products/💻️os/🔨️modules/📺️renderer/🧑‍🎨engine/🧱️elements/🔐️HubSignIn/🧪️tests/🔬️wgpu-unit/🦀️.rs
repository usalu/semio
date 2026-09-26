//! 🧪️ Laws of the wgpu hub sign-in contract (ticket 26/09/18, slice WG6 / G8 item WG-6).
//!
//! 🤝️ The parity mechanism: every table below is read from the SAME language-agnostic fixture the
//! React twin's vitest suite reads (`📇️directory/🔐️sign-in/🔣️.json`, AU2 §3). Same fixture in →
//! same verdict out. A fixture edit breaks whichever renderer drifted, which is the whole point of
//! having two implementations of one contract rather than two contracts.

use super::*;

const FIXTURE: &str = include_str!("../../../../../../📇️directory/🔐️sign-in/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("the shared sign-in fixture is valid JSON")
}

fn rows(key: &str) -> Vec<serde_json::Value> {
    fixture().get(key).and_then(|value| value.as_array()).cloned().unwrap_or_default()
}

#[test]
fn the_fixture_names_the_routes_bounds_and_schemas_this_module_pins() {
    let fixture = fixture();
    assert_eq!(fixture["mintPath"], HUB_SESSION_MINT_PATH_V1);
    assert_eq!(fixture["sessionPath"], HUB_SESSION_ME_PATH_V1);
    assert_eq!(fixture["signOutPath"], HUB_SESSION_SIGN_OUT_PATH_V1);
    assert_eq!(fixture["requestSchema"], HUB_SIGN_IN_REQUEST_SCHEMA_V1);
    assert_eq!(fixture["errorSchema"], HUB_AUTH_ERROR_SCHEMA_V1);
    assert_eq!(fixture["requestMaxBytes"].as_u64(), Some(HUB_SESSION_MINT_REQUEST_MAX_BYTES as u64));
    assert_eq!(fixture["timeoutMs"].as_u64(), Some(HUB_SIGN_IN_TIMEOUT_MS));
}

#[test]
fn every_status_in_the_shared_table_maps_to_the_same_closed_class_as_the_react_twin() {
    let table = rows("statusCodes");
    assert!(table.len() >= 9, "the shared status table shrank to {}", table.len());
    for row in &table {
        let status = row["status"].as_u64().expect("status") as u16;
        assert_eq!(hub_sign_in_error_from_status(status).as_str(), row["code"].as_str().expect("code"), "status {status}");
    }
}

#[test]
fn the_retry_after_header_grammar_agrees_with_the_shared_table() {
    for row in &rows("retryAfter") {
        let header = row["header"].as_str().expect("header");
        let expected = row["seconds"].as_u64();
        assert_eq!(hub_retry_after_seconds(Some(header)), expected, "header {header:?}");
    }
    assert_eq!(hub_retry_after_seconds(None), None);
}

#[test]
fn the_rate_limit_countdown_is_recovered_from_aus_own_error_body() {
    let body = format!("{{\"schema\":\"{HUB_AUTH_ERROR_SCHEMA_V1}\",\"code\":\"rate-limited\",\"retryAfterMs\":1500}}");
    assert_eq!(hub_auth_error_retry_after_seconds(&body), Some(2), "a partial second rounds UP, never down");
    assert_eq!(hub_auth_error_retry_after_seconds("{\"schema\":\"semio.other/v1\",\"retryAfterMs\":1500}"), None);
    assert_eq!(hub_auth_error_retry_after_seconds("<!doctype html>"), None);
    let over = format!("{{\"schema\":\"{HUB_AUTH_ERROR_SCHEMA_V1}\",\"retryAfterMs\":999999999}}");
    assert_eq!(hub_auth_error_retry_after_seconds(&over), None, "a countdown longer than a day is a hub defect, not a value");
}

#[test]
fn every_credential_vector_gets_the_same_admission_verdict_as_the_react_twin() {
    let table = rows("credentials");
    assert_eq!(table.len(), 5);
    for row in &table {
        let email = row["email"].as_str().expect("email");
        let password = row["password"].as_str().expect("password");
        let admitted = valid_hub_sign_in_email(email) && valid_hub_sign_in_password(password);
        assert_eq!(admitted, row["valid"].as_bool().expect("valid"), "credential {email} / {password}");
    }
}

#[test]
fn both_password_byte_edges_are_inclusive() {
    assert!(valid_hub_sign_in_password(&"a".repeat(HUB_SIGN_IN_PASSWORD_MIN_BYTES)));
    assert!(!valid_hub_sign_in_password(&"a".repeat(HUB_SIGN_IN_PASSWORD_MIN_BYTES - 1)));
    assert!(valid_hub_sign_in_password(&"a".repeat(HUB_SIGN_IN_PASSWORD_MAX_BYTES)));
    assert!(!valid_hub_sign_in_password(&"a".repeat(HUB_SIGN_IN_PASSWORD_MAX_BYTES + 1)));
    assert!(!valid_hub_sign_in_password("passwo\u{0007}rd"), "a control byte is refused, not stripped");
}

#[test]
fn every_token_vector_gets_the_same_verdict_as_the_hubs_own_capability_pattern() {
    let table = rows("tokens");
    assert_eq!(table.len(), 5);
    for row in &table {
        let token = row["token"].as_str().expect("token");
        assert_eq!(valid_hub_session_token(token), row["valid"].as_bool().expect("valid"), "token {token}");
    }
}

#[test]
fn every_origin_vector_normalizes_exactly_as_the_react_twin_does() {
    let table = rows("origins");
    assert_eq!(table.len(), 10);
    for row in &table {
        let typed = row["typed"].as_str().expect("typed");
        let expected = row["origin"].as_str().map(str::to_string);
        assert_eq!(parse_hub_origin(typed), expected, "typed {typed:?}");
    }
}

#[test]
fn the_mint_body_carries_the_shared_field_order_and_lowercases_the_email() {
    let credential = HubSignInCredential { email: "  Ada@Example.ORG ".to_string(), password: "correct horse".to_string(), device_instance_id: "device-1".to_string(), client_class: HubSignInClientClass::Native };
    let body = hub_session_mint_request_json(&credential).expect("a valid credential seals");
    let order: Vec<String> = rows("requestFieldOrder").iter().map(|value| value.as_str().expect("field").to_string()).collect();
    let mut cursor = 0usize;
    for field in &order {
        let needle = format!("\"{field}\":");
        let at = body[cursor..].find(&needle).unwrap_or_else(|| panic!("field {field} missing or out of order in {body}"));
        cursor += at + needle.len();
    }
    assert!(body.contains("\"email\":\"ada@example.org\""), "the email is trimmed and lowercased: {body}");
    assert!(body.len() <= HUB_SESSION_MINT_REQUEST_MAX_BYTES);
}

#[test]
fn a_malformed_credential_never_reaches_the_wire() {
    let bad = HubSignInCredential { email: "ada".into(), password: "correct horse".into(), device_instance_id: "device-1".into(), client_class: HubSignInClientClass::Browser };
    assert_eq!(hub_session_mint_request_json(&bad), Err(HubSignInErrorCode::MalformedRequest));
    let bad_device = HubSignInCredential { email: "ada@example.org".into(), password: "correct horse".into(), device_instance_id: "device 1".into(), client_class: HubSignInClientClass::Browser };
    assert_eq!(hub_session_mint_request_json(&bad_device), Err(HubSignInErrorCode::MalformedRequest));
}

#[test]
fn the_widest_admissible_credential_still_fits_the_hubs_own_body_limit() {
    let email = format!("{}@{}", "a".repeat(126), "a".repeat(127));
    assert_eq!(email.len(), HUB_SIGN_IN_EMAIL_MAX_BYTES);
    let widest = HubSignInCredential { email, password: "a".repeat(HUB_SIGN_IN_PASSWORD_MAX_BYTES), device_instance_id: "d".repeat(HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES), client_class: HubSignInClientClass::Browser };
    let body = hub_session_mint_request_json(&widest).expect("every in-bounds credential seals");
    assert!(
        body.len() <= HUB_SESSION_MINT_REQUEST_MAX_BYTES,
        "the field bounds already keep every admissible body under axum's {HUB_SESSION_MINT_REQUEST_MAX_BYTES}-byte limit — the widest is {} bytes, so the local cap is a belt-and-braces check, never the binding one",
        body.len()
    );
}

#[test]
fn the_mint_answer_is_parsed_only_in_its_exact_two_field_shape() {
    let token = "session.v1.0123456789abcdef0123456789abcdef.0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let good = format!("{{\"token\":\"{token}\",\"user_id\":\"u1\"}}");
    assert_eq!(parse_hub_session_mint_result(&good), Some(HubSessionMintResult { token: token.to_string(), user_id: "u1".to_string() }));
    assert_eq!(parse_hub_session_mint_result(&format!("{{\"token\":\"{token}\",\"user_id\":\"u1\",\"role\":\"author\"}}")), None, "an extra field is a refusal");
    assert_eq!(parse_hub_session_mint_result(&format!("{{\"token\":\"{token}\",\"userId\":\"u1\"}}")), None, "this route is snake_case and only snake_case");
    assert_eq!(parse_hub_session_mint_result("<!doctype html>"), None);
}

#[test]
fn the_session_authority_record_is_the_only_shape_that_installs_an_identity() {
    let good = format!("{{\"schema\":\"{HUB_SESSION_AUTHORITY_SCHEMA_V1}\",\"userId\":\"u1\",\"email\":\"ada@example.org\",\"displayName\":\"Ada\",\"expiresAt\":1893456000000,\"sessionKind\":\"external\",\"authorizationGeneration\":1}}");
    let authority = parse_hub_session_authority(&good).expect("the hub's own record decodes");
    assert_eq!(authority.expires_at_ms, 1_893_456_000_000);
    assert_eq!(authority.session_kind, HubSessionKind::External);
    let zero = good.replace("1893456000000", "0");
    assert_eq!(parse_hub_session_authority(&zero), None, "a zeroed deadline is refused, not treated as now");
    let unknown = good.replace("external", "root");
    assert_eq!(parse_hub_session_authority(&unknown), None);
    assert_eq!(parse_hub_session_authority(&"a".repeat(HUB_SESSION_AUTHORITY_MAX_BYTES + 1)), None);
}

#[test]
fn both_locales_are_complete_and_agree_with_the_shared_fixture_string_for_string() {
    let fixture = fixture();
    for (locale, key) in [(Locale::En, "en"), (Locale::De, "de")] {
        let expected = &fixture["presentation"]["text"][key];
        let table = hub_sign_in_text(locale);
        assert_eq!(expected["invalidCredentials"].as_str(), Some(table.invalid_credentials));
        assert_eq!(expected["rateLimited"].as_str(), Some(table.rate_limited));
        assert_eq!(expected["unreachable"].as_str(), Some(table.unreachable));
        assert_eq!(expected["hubRefused"].as_str(), Some(table.hub_refused));
        assert_eq!(expected["invalidResponse"].as_str(), Some(table.invalid_response));
        assert_eq!(expected["cancelled"].as_str(), Some(table.cancelled));
        assert_eq!(expected["expired"].as_str(), Some(table.expired));
        assert_eq!(expected["invalidOrigin"].as_str(), Some(table.invalid_origin));
        assert_eq!(expected["passwordDisabled"].as_str(), Some(table.password_disabled));
        assert_eq!(expected["malformedRequest"].as_str(), Some(table.malformed_request));
        assert_eq!(expected["shortPassword"].as_str(), Some(table.short_password));
        assert_eq!(expected["invalidEmail"].as_str(), Some(table.invalid_email));
    }
}

#[test]
fn no_chrome_label_is_shared_between_the_two_languages_by_accident() {
    for key in [
        HubSignInLabel::Title,
        HubSignInLabel::Email,
        HubSignInLabel::Password,
        HubSignInLabel::Submit,
        HubSignInLabel::Cancel,
        HubSignInLabel::SignOut,
        HubSignInLabel::AddHub,
        HubSignInLabel::ForgetHub,
        HubSignInLabel::HubAddress,
        HubSignInLabel::LocalOnly,
        HubSignInLabel::SigningIn,
        HubSignInLabel::SigningOut,
        HubSignInLabel::SignedInAs,
        HubSignInLabel::ThisDevice,
    ] {
        assert_ne!(hub_sign_in_label(key, Locale::En), hub_sign_in_label(key, Locale::De), "{key:?} was left untranslated");
        assert!(!hub_sign_in_label(key, Locale::De).is_empty());
    }
}

#[test]
fn the_rate_limit_text_substitutes_the_countdown_and_no_other_class_does() {
    let text = hub_sign_in_error_text(Locale::En, HubSignInErrorCode::RateLimited, Some(45));
    assert!(text.contains("45"), "{text}");
    assert!(!text.contains("{{seconds}}"));
    assert_eq!(hub_sign_in_error_text(Locale::De, HubSignInErrorCode::RateLimited, None), hub_sign_in_text(Locale::De).rate_limited.replace("{{seconds}}", "0"));
    assert_eq!(hub_sign_in_error_text(Locale::En, HubSignInErrorCode::Unreachable, Some(45)), hub_sign_in_text(Locale::En).unreachable);
}

#[test]
fn the_book_persists_hub_identity_and_never_a_secret() {
    let fixture = fixture();
    let persisted: Vec<&str> = fixture["persistedFields"].as_array().expect("persistedFields").iter().map(|value| value.as_str().expect("field")).collect();
    let never: Vec<&str> = fixture["neverPersisted"].as_array().expect("neverPersisted").iter().map(|value| value.as_str().expect("field")).collect();
    let book = upsert_hub_connection(
        &parse_hub_connection_book(None, "http://127.0.0.1:7777", Locale::En),
        HubConnection { id: hub_connection_id_for_origin("https://hub.example.org"), kind: HubConnectionKind::Remote, label: "Studio".into(), last_user_id: Some("u1".into()), origin: "https://hub.example.org".into() },
    );
    let serialized = serialize_hub_connection_book(&book);
    for field in &persisted {
        assert!(serialized.contains(&format!("\"{field}\":")), "the book must persist {field}: {serialized}");
    }
    for field in &never {
        assert!(!serialized.contains(field), "the book leaked {field}: {serialized}");
    }
    assert!(!serialized.contains("session.v1."), "no capability may reach the profile store");
    assert!(!serialized.contains(LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1), "the bootstrap entry is derived, never written");
}

#[test]
fn a_corrupt_or_absent_book_repairs_to_the_bootstrap_only_book() {
    for source in [None, Some("{"), Some("[]"), Some("null"), Some("{\"schema\":\"semio.other\"}"), Some("{\"schema\":\"semio.os.hub-connection-book.v1\",\"connections\":{},\"selectedId\":\"x\"}")] {
        let book = parse_hub_connection_book(source, "http://127.0.0.1:7777", Locale::En);
        assert_eq!(book.connections.len(), 1, "source {source:?}");
        assert_eq!(book.selected_id, LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1);
        assert_eq!(book.connections[0].origin, "http://127.0.0.1:7777");
    }
}

#[test]
fn the_book_is_capped_and_a_re_added_hub_updates_rather_than_duplicates() {
    let mut book = parse_hub_connection_book(None, "http://127.0.0.1:7777", Locale::En);
    for index in 0..40 {
        let origin = format!("https://hub{index}.example.org");
        book = upsert_hub_connection(&book, HubConnection { id: hub_connection_id_for_origin(&origin), kind: HubConnectionKind::Remote, label: format!("Hub {index}"), last_user_id: None, origin });
    }
    assert_eq!(book.connections.len(), HUB_CONNECTION_BOOK_MAX_ENTRIES);
    let before = book.connections.len();
    let origin = "https://hub39.example.org".to_string();
    book = upsert_hub_connection(&book, HubConnection { id: hub_connection_id_for_origin(&origin), kind: HubConnectionKind::Remote, label: "Renamed".into(), last_user_id: None, origin });
    assert_eq!(book.connections.len(), before, "re-adding one hub must not grow the book");
    assert_eq!(book.connections.iter().filter(|entry| entry.label == "Renamed").count(), 1);
    assert_eq!(selected_hub_connection(&book).label, "Renamed");
}

#[test]
fn forgetting_the_selected_hub_falls_back_to_the_local_bootstrap_which_can_never_be_removed() {
    let origin = "https://hub.example.org".to_string();
    let book = upsert_hub_connection(
        &parse_hub_connection_book(None, "http://127.0.0.1:7777", Locale::En),
        HubConnection { id: hub_connection_id_for_origin(&origin), kind: HubConnectionKind::Remote, label: "Studio".into(), last_user_id: None, origin: origin.clone() },
    );
    let forgotten = remove_hub_connection(&book, &hub_connection_id_for_origin(&origin));
    assert_eq!(forgotten.selected_id, LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1);
    assert_eq!(remove_hub_connection(&forgotten, LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1), forgotten);
    assert_eq!(select_hub_connection(&forgotten, "remote:https://nowhere.example.org"), forgotten, "an unknown id leaves the book untouched");
}

#[test]
fn losing_the_network_never_signs_a_live_session_out() {
    let signed_in = reduce_hub_session(&hub_session_initial_state(LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1), &HubSessionEvent::Minted { user_id: "u1".into(), expires_at_ms: Some(10) });
    let offline = reduce_hub_session(&signed_in, &HubSessionEvent::Connectivity { offline: true });
    assert_eq!(offline.phase, HubSessionPhase::SignedIn);
    assert!(offline.offline);
    let refused = reduce_hub_session(&offline, &HubSessionEvent::Failed { code: HubSignInErrorCode::Unreachable, retry_after_seconds: None });
    assert_eq!(refused.phase, HubSessionPhase::SignedIn, "a refusal while signed in is reported, never a sign-out");
    assert_eq!(refused.user_id.as_deref(), Some("u1"));
    assert_eq!(refused.error, Some(HubSignInErrorCode::Unreachable));
    assert!(hub_session_allows_local_work(&refused), "no hub phase ever disables local work");
}

#[test]
fn the_reducer_walks_submit_to_minted_to_expired_to_re_authentication() {
    let state = hub_session_initial_state(LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1);
    let submitting = reduce_hub_session(&state, &HubSessionEvent::Submit);
    assert_eq!(submitting.phase, HubSessionPhase::SigningIn);
    let minted = reduce_hub_session(&submitting, &HubSessionEvent::Minted { user_id: "u1".into(), expires_at_ms: Some(1_000) });
    assert!(!hub_session_needs_reauthentication(&minted, 999));
    assert!(hub_session_needs_reauthentication(&minted, 1_000), "the deadline is inclusive");
    let expired = reduce_hub_session(&minted, &HubSessionEvent::Expired);
    assert_eq!(expired.phase, HubSessionPhase::Expired);
    assert!(hub_session_needs_reauthentication(&expired, 0));
    let refused_again = reduce_hub_session(&expired, &HubSessionEvent::Failed { code: HubSignInErrorCode::InvalidCredentials, retry_after_seconds: None });
    assert_eq!(refused_again.phase, HubSessionPhase::Expired, "a wrong password during re-auth stays in the expired phase");
}

#[test]
fn switching_hub_resets_identity_but_keeps_connectivity() {
    let signed_in = reduce_hub_session(&hub_session_initial_state("local-bootstrap"), &HubSessionEvent::Minted { user_id: "u1".into(), expires_at_ms: None });
    let offline = reduce_hub_session(&signed_in, &HubSessionEvent::Connectivity { offline: true });
    let switched = reduce_hub_session(&offline, &HubSessionEvent::SelectConnection { connection_id: "remote:https://hub.example.org".into() });
    assert_eq!(switched.phase, HubSessionPhase::SignedOut);
    assert_eq!(switched.user_id, None);
    assert!(switched.offline, "connectivity is a property of the device, not of the hub");
    assert_eq!(reduce_hub_session(&offline, &HubSessionEvent::SelectConnection { connection_id: "local-bootstrap".into() }), offline, "re-selecting the same hub is a no-op");
}

#[test]
fn a_hub_that_refused_password_sign_in_offers_no_password_form_at_all() {
    let state = hub_session_initial_state(LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1);
    assert!(hub_sign_in_form_offered(&state));
    let refused = reduce_hub_session(&state, &HubSessionEvent::Failed { code: HubSignInErrorCode::PasswordSignInDisabled, retry_after_seconds: None });
    assert!(!hub_sign_in_form_offered(&refused), "the field is structurally absent, never offered and then refused");
    let signed_in = reduce_hub_session(&state, &HubSessionEvent::Minted { user_id: "u1".into(), expires_at_ms: None });
    assert!(!hub_sign_in_form_offered(&signed_in));
}

#[test]
fn signing_out_clears_the_identity_and_keeps_the_hub_selection() {
    let signed_in = reduce_hub_session(&hub_session_initial_state("remote:https://hub.example.org"), &HubSessionEvent::Minted { user_id: "u1".into(), expires_at_ms: Some(10) });
    let leaving = reduce_hub_session(&signed_in, &HubSessionEvent::SignOut);
    assert_eq!(leaving.phase, HubSessionPhase::SigningOut);
    let gone = reduce_hub_session(&leaving, &HubSessionEvent::SignedOut);
    assert_eq!(gone.phase, HubSessionPhase::SignedOut);
    assert_eq!(gone.user_id, None);
    assert_eq!(gone.expires_at_ms, None);
    assert_eq!(gone.connection_id, "remote:https://hub.example.org");
}
