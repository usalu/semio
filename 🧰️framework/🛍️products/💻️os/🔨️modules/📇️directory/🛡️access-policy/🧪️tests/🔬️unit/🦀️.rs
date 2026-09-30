use super::*;

/// 🧫️ The declared policy decides every language-neutral truth-table vector exactly as declared,
/// round-trips as data, and refuses a document with an unknown space kind or an empty grant.
#[test]
fn declared_access_policy_matches_the_language_neutral_truth_table() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("access policy vectors");
    let policy = DirectoryAccessPolicyV1::declared();
    for row in fixture["policies"].as_array().expect("policies") {
        let source = serde_json::to_string(&row["value"]).expect("policy source");
        assert_eq!(DirectoryAccessPolicyV1::parse(&source).is_ok(), row["valid"].as_bool().expect("valid"), "{}", row["name"]);
    }
    println!("[access-policy] policies={} decisions={}", fixture["policies"].as_array().expect("policies").len(), fixture["vectors"].as_array().expect("vectors").len());
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let roles: Vec<DirectoryAccessRoleV1> = serde_json::from_value(vector["roles"].clone()).expect("roles");
        let action: DirectoryAccessActionV1 = serde_json::from_value(vector["action"].clone()).expect("action");
        let space_kind = vector["spaceKind"].as_str();
        assert_eq!(policy.permits(&roles, action, space_kind), vector["permitted"].as_bool().expect("permitted"), "{}", vector["name"]);
    }
    let reparsed = DirectoryAccessPolicyV1::parse(&serde_json::to_string(policy).expect("policy serializes")).expect("policy round-trips");
    assert_eq!(&reparsed, policy);
    assert!(DirectoryAccessPolicyV1::parse(r#"{"schema":"semio.os.directory.access-policy/v1","grants":[{"effect":"allow","roles":["author"],"actions":["document.write"],"spaceKinds":["garden"]}]}"#).is_err());
    assert!(DirectoryAccessPolicyV1::parse(r#"{"schema":"semio.os.directory.access-policy/v1","grants":[{"effect":"allow","roles":[],"actions":["document.write"]}]}"#).is_err());
    assert!(DirectoryAccessPolicyV1::parse(r#"{"schema":"semio.os.directory.access-policy/v1","grants":[{"effect":"allow","roles":["author"],"actions":["document.write"],"extra":1}]}"#).is_err());
}

/// 🚫️ Closed by default and deny overrides allow, whatever order the grants were declared in.
#[test]
fn access_policy_is_closed_by_default_and_deny_overrides_allow() {
    let deny_first = DirectoryAccessPolicyV1::parse(r#"{"schema":"semio.os.directory.access-policy/v1","grants":[{"effect":"deny","roles":["author"],"actions":["document.write"],"spaceKinds":["archive"]},{"effect":"allow","roles":["author"],"actions":["document.write"]}]}"#).expect("policy");
    assert!(deny_first.permits(&[DirectoryAccessRoleV1::Author], DirectoryAccessActionV1::DocumentWrite, Some("studio")));
    assert!(!deny_first.permits(&[DirectoryAccessRoleV1::Author], DirectoryAccessActionV1::DocumentWrite, Some("archive")));
    assert!(!deny_first.permits(&[DirectoryAccessRoleV1::Author], DirectoryAccessActionV1::DocumentRead, Some("studio")), "nothing granted is denied");
    assert!(!deny_first.permits(&[], DirectoryAccessActionV1::DocumentWrite, Some("studio")));
}
