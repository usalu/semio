//! 🧹️ `strip-non-tiny` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//!
//! `strip` is not itself an approved semantic verb (`protocol::APPROVED_VERBS`); this leaf performs
//! the Full→Tiny down-conversion by DROPPING every excluded element subtree and forbidden
//! presentation attribute, so `SEMANTICS.verb` is `"remove"`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct StripNonTiny {}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for StripNonTiny {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "non-tiny-content", kind: "strip-non-tiny", record: "StripNonTiny" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgTinyMutation as protocol::Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgTinyMutation::StripNonTiny(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgTinyMutation> {
        agg_inverse(&SvgTinyMutation::StripNonTiny(self.clone()), base)
    }
    fn label(&self) -> String {
        "strip-non-tiny".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
