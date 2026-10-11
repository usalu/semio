//! 🏗️ `insert-spatial-node` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertSpatialNode {
    pub node: SpatialNode,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for InsertSpatialNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "spatial-node", kind: "insert-spatial-node", record: "InsertSpatialNode" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as Mutation<SemioModelSnapshot>>::Diff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert spatial node", "Raumknoten einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
