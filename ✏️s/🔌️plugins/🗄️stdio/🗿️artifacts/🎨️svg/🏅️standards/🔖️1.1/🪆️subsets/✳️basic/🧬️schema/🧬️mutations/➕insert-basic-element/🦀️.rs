//! ➕️ `insert-basic-element` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertBasicElement {
    pub(crate) parent: NodePath,
    pub(crate) index: usize,
    pub(crate) node: XmlNode,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for InsertBasicElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "basic-element", kind: "insert-basic-element", record: "InsertBasicElement" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgBasicMutation as protocol::Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgBasicMutation::InsertBasicElement(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgBasicMutation> {
        agg_inverse(&SvgBasicMutation::InsertBasicElement(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-basic-element".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
