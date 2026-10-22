//! 🏌️ `set-block-entity-layer` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetBlockEntityLayer {
    pub block_name: String,
    pub handle: String,
    pub layer: String,
}

impl<Op> protocol::MutationKind<SemioCadSnapshot, Op> for SetBlockEntityLayer
where
    Op: protocol::Mutation<SemioCadSnapshot, Diff = crate::schema::diff::SemioCadDiff> + From<SetBlockEntityLayer>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "block-entity-layer", kind: "set-block-entity-layer", record: "SetBlockEntityLayer" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set block entity layer", "Ebene der Blockentität setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioCadSnapshot, Op> for SetBlockEntityLayer
where
    Op: protocol::ColdMutation<SemioCadSnapshot, ReferenceDiff = crate::schema::diff::SemioCadDiff> + From<SetBlockEntityLayer>,
{
    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioCadSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
