//! 📦️ `set-payload` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when this field lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, dsl::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-payload")]
pub struct SetPayload {
    #[dsl(base64)]
    pub payload: Vec<u8>,
}

impl protocol::MutationKind<DeflateSnapshot, DeflateMutation> for SetPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "payload", kind: "set-payload", record: "SetPayload" };

    fn diff(&self, base: &DeflateSnapshot) -> protocol::MutationOutcome<<DeflateMutation as protocol::Mutation<DeflateSnapshot>>::Diff> {
        agg_diff(&DeflateMutation::SetPayload(self.clone()), base)
    }
    fn inverse(&self, base: &DeflateSnapshot) -> Vec<DeflateMutation> {
        agg_inverse(&DeflateMutation::SetPayload(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-payload".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
