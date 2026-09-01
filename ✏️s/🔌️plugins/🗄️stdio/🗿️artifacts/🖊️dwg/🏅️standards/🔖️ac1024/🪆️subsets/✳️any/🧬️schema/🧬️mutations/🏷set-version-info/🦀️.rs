//! 🏷️ `set-version-info` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by construction
//! rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetVersionInfo {
    pub(crate) version: String,
    pub(crate) maintenance_version: u8,
    pub(crate) codepage: u16,
}

impl protocol::MutationKind<DwgSnapshot, DwgMutation> for SetVersionInfo {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "version-info", kind: "set-version-info", record: "SetVersionInfo" };

    fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<<DwgMutation as protocol::Mutation<DwgSnapshot>>::Diff> {
        agg_diff(&DwgMutation::SetVersionInfo(self.clone()), base)
    }
    fn inverse(&self, base: &DwgSnapshot) -> Vec<DwgMutation> {
        agg_inverse(&DwgMutation::SetVersionInfo(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-version-info".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
