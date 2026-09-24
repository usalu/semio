//! 🎛️ `set-element` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetElement {
    pub id: String,
    #[value(default)]
    pub class: Option<ElementClass>,
    #[value(default)]
    pub placement: Option<SemioTransform>,
    #[value(default)]
    pub geometry: Option<GeometryRef>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub spatial_id: Option<Option<String>>,
    #[value(default)]
    pub psets: Option<Vec<PropertySet>>,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for SetElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "element", kind: "set-element", record: "SetElement" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::SetElement(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::SetElement(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Set element", "Element setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
