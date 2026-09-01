//! 🎞️ `set-stream-header` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStreamHeader {
    pub stream_index: usize,
    pub strh: AviStreamHeader,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetStreamHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "stream-header", kind: "set-stream-header", record: "SetStreamHeader" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as protocol::Mutation<AviSnapshot>>::Diff> {
        agg_diff(&AviMutation::SetStreamHeader(self.clone()), base)
    }
    fn inverse(&self, base: &AviSnapshot) -> Vec<AviMutation> {
        agg_inverse(&AviMutation::SetStreamHeader(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-stream-header".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
