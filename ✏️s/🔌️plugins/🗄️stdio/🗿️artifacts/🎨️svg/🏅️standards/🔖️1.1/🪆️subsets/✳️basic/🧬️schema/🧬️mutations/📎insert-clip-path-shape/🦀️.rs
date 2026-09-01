//! 📎️ `insert-clip-path-shape` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by construction
//! rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertClipPathShape {
    pub(crate) clip_path_id: String,
    pub(crate) index: usize,
    pub(crate) node: XmlNode,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for InsertClipPathShape {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "clip-path-shape", kind: "insert-clip-path-shape", record: "InsertClipPathShape" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgBasicMutation as protocol::Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgBasicMutation::InsertClipPathShape(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Vec<SvgBasicMutation> {
        agg_inverse(&SvgBasicMutation::InsertClipPathShape(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-clip-path-shape".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
