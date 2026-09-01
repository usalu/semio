//! 🚪️ `SignOut` is the authoritative direct Rust leaf for clearing the OS identity session.

use super::sign_in::{sign_in, IdentitySetting};
use super::IdentityConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚪️ Clears the OS-wide signed-in session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SignOut {}

/// 🏗️ Wraps the empty sign-out payload in the identity dispatch enum.
pub fn sign_out() -> IdentityConfigMutation {
    IdentityConfigMutation::SignOut(SignOut {})
}

impl MutationKind<IdentitySetting, IdentityConfigMutation> for SignOut {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "identity", kind: "sign-out", record: "Cleared" };

    fn diff(&self, _base: &IdentitySetting) -> MutationOutcome<IdentitySetting> {
        MutationOutcome::new(IdentitySetting(None))
    }

    fn inverse(&self, base: &IdentitySetting) -> Vec<IdentityConfigMutation> {
        base.0.clone().map(sign_in).into_iter().collect()
    }

    fn label(&self) -> String {
        "Sign out".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["identity".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/clears-the-active-session/🦀️component.rs"]
mod tests_clears_the_active_session;

#[cfg(test)]
mod tests {
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
}
//#endregion 🧪️Tests
