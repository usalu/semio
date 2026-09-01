//! 🎨️ `set-global-color-table` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetGlobalColorTable {
    #[dsl(block)]
    pub(crate) gct: Option<GifColorTable>,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetGlobalColorTable {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "global-color-table", kind: "set-global-color-table", record: "SetGlobalColorTable" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as protocol::Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::SetGlobalColorTable(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Vec<GifMutation> {
        agg_inverse(&GifMutation::SetGlobalColorTable(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-global-color-table".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
