//! 🎛️ `set-element` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct SetElement {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) class: Option<ElementClass>,
    #[serde(default)]
    pub(crate) placement: Option<SemioTransform>,
    #[serde(default)]
    pub(crate) geometry: Option<GeometryRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub(crate) spatial_id: Option<Option<String>>,
    #[serde(default)]
    pub(crate) psets: Option<Vec<PropertySet>>,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for SetElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "element", kind: "set-element", record: "SetElement" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::SetElement(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::SetElement(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-element".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
