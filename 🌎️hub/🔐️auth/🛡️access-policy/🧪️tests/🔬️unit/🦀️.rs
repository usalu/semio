use super::*;

/// 🧫️ The declared policy decides every language-neutral truth-table vector exactly as declared,
/// round-trips as data, and refuses a document with an unknown space kind or an empty grant.
#[test]
fn declared_access_policy_matches_the_language_neutral_truth_table() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("access policy vectors");
    let policy = HubAccessPolicyV1::declared();
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let roles: Vec<HubAccessRoleV1> = serde_json::from_value(vector["roles"].clone()).expect("roles");
        let action: HubAccessActionV1 = serde_json::from_value(vector["action"].clone()).expect("action");
        let space_kind = vector["spaceKind"].as_str();
        assert_eq!(policy.permits(&roles, action, space_kind), vector["permitted"].as_bool().expect("permitted"), "{}", vector["name"]);
    }
    let reparsed = HubAccessPolicyV1::parse(&serde_json::to_string(policy).expect("policy serializes")).expect("policy round-trips");
    assert_eq!(&reparsed, policy);
    assert!(HubAccessPolicyV1::parse(r#"{"schema":"semio.hub.access-policy/v1","grants":[{"effect":"allow","roles":["author"],"actions":["document.write"],"spaceKinds":["garden"]}]}"#).is_err());
    assert!(HubAccessPolicyV1::parse(r#"{"schema":"semio.hub.access-policy/v1","grants":[{"effect":"allow","roles":[],"actions":["document.write"]}]}"#).is_err());
    assert!(HubAccessPolicyV1::parse(r#"{"schema":"semio.hub.access-policy/v1","grants":[{"effect":"allow","roles":["author"],"actions":["document.write"],"extra":1}]}"#).is_err());
}

/// 🚫️ Closed by default and deny overrides allow, whatever order the grants were declared in.
#[test]
fn access_policy_is_closed_by_default_and_deny_overrides_allow() {
    let deny_first = HubAccessPolicyV1::parse(r#"{"schema":"semio.hub.access-policy/v1","grants":[{"effect":"deny","roles":["author"],"actions":["document.write"],"spaceKinds":["archive"]},{"effect":"allow","roles":["author"],"actions":["document.write"]}]}"#).expect("policy");
    assert!(deny_first.permits(&[HubAccessRoleV1::Author], HubAccessActionV1::DocumentWrite, Some("studio")));
    assert!(!deny_first.permits(&[HubAccessRoleV1::Author], HubAccessActionV1::DocumentWrite, Some("archive")));
    assert!(!deny_first.permits(&[HubAccessRoleV1::Author], HubAccessActionV1::DocumentRead, Some("studio")), "nothing granted is denied");
    assert!(!deny_first.permits(&[], HubAccessActionV1::DocumentWrite, Some("studio")));
}
