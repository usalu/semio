//! 🪪️ `SignIn` is the authoritative direct Rust leaf for establishing an OS identity session.

use super::sign_out::SignOut;
use super::IdentityConfigMutation;
use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 🪪️ The OS-wide signed-in session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub hub_base_url: String,
    pub session_token: String,
    pub issued_at_ms: u64,
}

/// 🪪️ `os.config.identity` — a session or `null` when signed out.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdentitySetting(pub Option<Identity>);

/// 🪪️ The schema id for the identity config facet.
pub const IDENTITY_CONFIG_SCHEMA: &str = "os.config.identity";

impl MutationDiff<IdentitySetting> for IdentitySetting {
    fn apply(&self, _base: &IdentitySetting) -> protocol::MutationApplyResult<IdentitySetting> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Schema

//#region 🔖️Mutation
/// 🪪️ Establishes or replaces the OS-wide signed-in session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub hub_base_url: String,
    pub session_token: String,
    pub issued_at_ms: u64,
}

/// 🏗️ Wraps a sign-in payload in the identity dispatch enum.
pub fn sign_in(identity: Identity) -> IdentityConfigMutation {
    IdentityConfigMutation::SignIn(SignIn::from(identity))
}

impl From<Identity> for SignIn {
    fn from(identity: Identity) -> Self {
        Self {
            user_id: identity.user_id,
            email: identity.email,
            display_name: identity.display_name,
            hub_base_url: identity.hub_base_url,
            session_token: identity.session_token,
            issued_at_ms: identity.issued_at_ms,
        }
    }
}

impl From<&SignIn> for Identity {
    fn from(payload: &SignIn) -> Self {
        Self {
            user_id: payload.user_id.clone(),
            email: payload.email.clone(),
            display_name: payload.display_name.clone(),
            hub_base_url: payload.hub_base_url.clone(),
            session_token: payload.session_token.clone(),
            issued_at_ms: payload.issued_at_ms,
        }
    }
}

impl MutationKind<IdentitySetting, IdentityConfigMutation> for SignIn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "identity", kind: "sign-in", record: "Set" };

    fn diff(&self, _base: &IdentitySetting) -> MutationOutcome<IdentitySetting> {
        MutationOutcome::new(IdentitySetting(Some(self.into())))
    }

    fn inverse(&self, base: &IdentitySetting) -> Vec<IdentityConfigMutation> {
        match &base.0 {
            Some(identity) => vec![sign_in(identity.clone())],
            None => vec![IdentityConfigMutation::SignOut(SignOut {})],
        }
    }

    fn label(&self) -> String {
        format!("Sign in \"{}\"", self.email)
    }

    fn target(&self) -> Vec<String> {
        vec!["identity".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🌉️MutationCodecBridge
/// 🧮️ Applies one identity mutation through its whole-record diff.
pub fn apply_identity_config_mutation(snapshot: &mut IdentitySetting, mutation: &IdentityConfigMutation) -> protocol::MutationApplyResult<()> {
    use protocol::{Mutation as _, MutationDiff as _};
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation session.
pub fn inverse_identity_config_mutation(snapshot: &IdentitySetting, mutation: &IdentityConfigMutation) -> Vec<IdentityConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(snapshot)
}

/// 📥️ Decodes the internally tagged identity mutation JSON projection.
pub fn decode_identity_config_mutation_json(text: &str) -> Result<IdentityConfigMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ Encodes the identity setting to its canonical camel-case JSON projection — a bare
/// session object, or the bare literal `null` when signed out.
pub fn encode_identity_setting_json(snapshot: &IdentitySetting) -> String {
    serde_json::to_string(snapshot).expect("IdentitySetting serialization is infallible")
}

/// 📥️ Decodes the canonical identity setting JSON projection.
pub fn decode_identity_setting_json(text: &str) -> Result<IdentitySetting, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_identity_config_mutation_reporting(snapshot: &mut IdentitySetting, mutation: &IdentityConfigMutation) -> Vec<(String, String)> {
    use protocol::Mutation as _;
    let outcome = mutation.diff(snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_identity_config_mutation_steps(mutation: &IdentityConfigMutation, base: &IdentitySetting) -> Vec<IdentityConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(base)
}
//#endregion 🌉️MutationCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/replaces-the-active-session-with-a-second-account/🦀️component.rs"]
mod tests_replaces_the_active_session_with_a_second_account;

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    fn identity(user_id: &str) -> Identity {
        Identity {
            user_id: user_id.to_string(),
            email: format!("{user_id}@example.test"),
            display_name: user_id.to_string(),
            hub_base_url: "https://hub.example.test".to_string(),
            session_token: format!("token-{user_id}"),
            issued_at_ms: 42,
        }
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
}
//#endregion 🧪️Tests
