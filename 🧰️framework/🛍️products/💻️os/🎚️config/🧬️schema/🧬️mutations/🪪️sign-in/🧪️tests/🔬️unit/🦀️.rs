use super::*;
use protocol::Mutation;

fn identity(user_id: &str) -> Identity {
    Identity { user_id: user_id.to_string(), email: format!("{user_id}@example.test"), display_name: user_id.to_string(), hub_base_url: "https://hub.example.test".to_string(), issued_at_ms: 42 }
}

#[test]
fn sign_in_serializes_like_the_typescript_projection() {
    let mutation = sign_in(identity("ada"));
    let json = serde_json::to_value(mutation).expect("sign-in encodes");
    assert_eq!(json["mutation"], "signIn");
    assert_eq!(json["userId"], "ada");
    assert_eq!(json["issuedAtMs"], 42);
}

#[test]
fn replacing_a_session_inverts_to_the_prior_identity() {
    let base = IdentitySetting(Some(identity("prior")));
    let mutation = sign_in(identity("next"));
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse, vec![sign_in(identity("prior"))]);
}
