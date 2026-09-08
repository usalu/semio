
use super::*;
use protocol::Mutation;

#[test]
fn sign_out_serializes_like_the_typescript_projection() {
    let json = serde_json::to_value(sign_out()).expect("sign-out encodes");
    assert_eq!(json, serde_json::json!({ "mutation": "signOut" }));
}

#[test]
fn signed_out_state_has_no_inverse_step() {
    assert!(sign_out().inverse(&IdentitySetting::default()).is_empty());
}
