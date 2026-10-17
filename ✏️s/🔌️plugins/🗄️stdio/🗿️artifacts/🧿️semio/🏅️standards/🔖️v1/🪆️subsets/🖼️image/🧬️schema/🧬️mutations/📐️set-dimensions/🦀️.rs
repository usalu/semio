//! 📐️️ `set-dimensions` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioImageDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDimensions {
    pub width: u32,
    pub height: u32,
}

impl<Op> protocol::MutationKind<SemioImageSnapshot, Op> for SetDimensions
where
    Op: protocol::Mutation<SemioImageSnapshot, Diff = crate::schema::diff::SemioImageDiff> + From<SetDimensions>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "dimensions", kind: "set-dimensions", record: "SetDimensions" };

    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<Op as Mutation<SemioImageSnapshot>>::Diff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set dimensions", "Abmessungen setzen")
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
