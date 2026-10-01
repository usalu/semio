use super::*;

#[test]
fn credential_exchange_path_follows_the_portable_schema() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🤖️agent-credential/🧫️fixtures/🔣️.json")).unwrap();
    for vector in corpus["vectors"].as_array().unwrap() {
        assert_eq!(CredentialExchangeRequestV1::new(vector["path"].as_str().unwrap().into(), Vec::new()).is_ok(), vector["valid"].as_bool().unwrap());
    }
    assert!(CredentialExchangeRequestV1::new("/identity/exchanges".into(), vec![0; 16385]).is_err());
}

#[test]
fn delegated_secret_is_bounded_and_redacted_without_an_owner_protocol_default() {
    let credential = DelegatedCredentialV1::new("https://fixture.invalid".into(), "scope".into(), "use".into(), "fixture-secret".into()).unwrap();
    assert!(!format!("{credential:?}").contains("fixture-secret"));
    assert!(decode_delegated_credential_v1(br#"{"schema":"fixture.credential/v1"}"#, &[]).is_err());
    assert!(DelegatedCredentialV1::new(String::new(), "scope".into(), "use".into(), "secret".into()).is_err());
}
