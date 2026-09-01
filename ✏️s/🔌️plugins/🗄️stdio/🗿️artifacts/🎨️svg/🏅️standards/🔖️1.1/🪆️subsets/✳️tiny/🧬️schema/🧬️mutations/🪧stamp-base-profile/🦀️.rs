//! 🪧️ `stamp-base-profile` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//!
//! `stamp` is not itself an approved semantic verb (`protocol::APPROVED_VERBS`); the operation it
//! performs is a plain attribute assignment on the root, so `SEMANTICS.verb` is `"set"`, matching
//! what `attributes_diff_at_path` actually does underneath.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct StampBaseProfile {
    pub(crate) base_profile: Option<String>,
    pub(crate) version: Option<String>,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for StampBaseProfile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "base-profile", kind: "stamp-base-profile", record: "StampBaseProfile" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgTinyMutation as protocol::Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgTinyMutation::StampBaseProfile(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgTinyMutation> {
        agg_inverse(&SvgTinyMutation::StampBaseProfile(self.clone()), base)
    }
    fn label(&self) -> String {
        "stamp-base-profile".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
