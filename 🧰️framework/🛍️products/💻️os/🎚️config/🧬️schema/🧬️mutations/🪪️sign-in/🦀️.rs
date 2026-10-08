//! 🪪️ `SignIn` is the authoritative direct Rust leaf for establishing an OS identity session.

use super::super::SettingEdit;
use super::sign_out::SignOut;
use super::IdentityConfigMutation;
use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 🪪️ The OS-wide signed-in session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Identity {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub hub_base_url: String,
    pub issued_at_ms: u64,
}

/// 🪪️ `os.config.identity` — a session or `null` when signed out.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdentitySetting(pub Option<Identity>);

/// 🔀️ Hand-written, not derived: `#[serde(transparent)]` tuple structs are not one of
/// `#[derive(ToValue, FromValue)]`'s supported shapes (named-field structs and internally-/
/// adjacently-tagged enums only) — see the fan-out playbook's attribute-coverage table.
impl ToValue for IdentitySetting {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}
impl FromValue for IdentitySetting {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self(semio_framework_value::FromValue::from_value(value)?))
    }
}

/// 🪪️ The schema id for the identity config facet.
pub const IDENTITY_CONFIG_SCHEMA: &str = "os.config.identity";

/// 🔺️ Sparse diff of [`IdentitySetting`]: the absolute new session, `value: None` signing out.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct IdentityDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub session: Option<SettingEdit<Identity>>,
}

impl MutationDiff<IdentitySetting> for IdentityDiff {
    fn apply(&self, base: &IdentitySetting, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<IdentitySetting> {
        Ok(match &self.session {
            Some(edit) => IdentitySetting(edit.value.clone()),
            None => base.clone(),
        })
    }

    fn absorb(&mut self, other: Self) {
        if other.session.is_some() {
            self.session = other.session;
        }
    }
}

impl protocol::DiffAlgebra<IdentitySetting> for IdentityDiff {
    fn inverse(&self, base: &IdentitySetting) -> Self {
        Self { session: self.session.as_ref().map(|_| SettingEdit::new(base.0.clone())) }
    }

    fn between(base: &IdentitySetting, other: &IdentitySetting) -> Self {
        Self { session: (base != other).then(|| SettingEdit::new(other.0.clone())) }
    }

    fn is_empty(&self) -> bool {
        self.session.is_none()
    }
}
//#endregion 🔖️Schema

//#region 🔖️Mutation
/// 🪪️ Establishes or replaces the OS-wide signed-in session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SignIn {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub hub_base_url: String,
    pub issued_at_ms: u64,
}

/// 🏗️ Wraps a sign-in payload in the identity dispatch enum.
pub fn sign_in(identity: Identity) -> IdentityConfigMutation {
    IdentityConfigMutation::SignIn(SignIn::from(identity))
}

impl From<Identity> for SignIn {
    fn from(identity: Identity) -> Self {
        Self { user_id: identity.user_id, email: identity.email, display_name: identity.display_name, hub_base_url: identity.hub_base_url, issued_at_ms: identity.issued_at_ms }
    }
}

impl From<&SignIn> for Identity {
    fn from(payload: &SignIn) -> Self {
        Self { user_id: payload.user_id.clone(), email: payload.email.clone(), display_name: payload.display_name.clone(), hub_base_url: payload.hub_base_url.clone(), issued_at_ms: payload.issued_at_ms }
    }
}

impl MutationKind<IdentitySetting, IdentityConfigMutation> for SignIn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "identity", kind: "sign-in", record: "Set" };

    fn diff(&self, base: &IdentitySetting) -> MutationOutcome<IdentityDiff> {
        let identity = Identity::from(self);
        if base.0.as_ref() == Some(&identity) {
            return MutationOutcome::empty();
        }
        MutationOutcome::new(IdentityDiff { session: Some(SettingEdit::new(Some(identity))) })
    }

    fn inverse(&self, base: &IdentitySetting) -> Result<Vec<IdentityConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match &base.0 {
            Some(identity) => vec![sign_in(identity.clone())],
            None => vec![IdentityConfigMutation::SignOut(SignOut {})],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Sign in \"{}\"", self.email), &format!("Als \"{}\" anmelden", self.email))
    }

    fn target(&self) -> Vec<String> {
        vec!["identity".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🌉️MutationCodecBridge
/// ↩️ Computes the mutation's inverse steps from the pre-mutation session.
pub fn inverse_identity_config_mutation(snapshot: &IdentitySetting, mutation: &IdentityConfigMutation) -> Result<Vec<IdentityConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(snapshot)?

    })
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

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_identity_config_mutation_steps(mutation: &IdentityConfigMutation, base: &IdentitySetting) -> Result<Vec<IdentityConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(base)?

    })
}
//#endregion 🌉️MutationCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🪪️replaces-the-active-session-with-a-second-account/🦀️.rs"]
mod tests_replaces_the_active_session_with_a_second_account;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
