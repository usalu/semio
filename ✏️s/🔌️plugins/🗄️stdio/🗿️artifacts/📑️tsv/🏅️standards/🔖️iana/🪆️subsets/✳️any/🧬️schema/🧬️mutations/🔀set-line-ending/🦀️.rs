//! 🔀 `set-line-ending` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLineEnding {
    pub(crate) line_ending: LineEnding,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for SetLineEnding {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "line-ending", kind: "set-line-ending", record: "SetLineEnding" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as protocol::Mutation<TsvSnapshot>>::Diff> {
        agg_diff(&TsvMutation::SetLineEnding(self.clone()), base)
    }
    fn inverse(&self, base: &TsvSnapshot) -> Vec<TsvMutation> {
        agg_inverse(&TsvMutation::SetLineEnding(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-line-ending".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
