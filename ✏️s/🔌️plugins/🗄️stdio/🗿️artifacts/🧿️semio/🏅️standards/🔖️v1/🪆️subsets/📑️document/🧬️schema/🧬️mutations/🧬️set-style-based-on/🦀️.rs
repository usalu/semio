//! 🔗 `set-style-based-on` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioDocumentDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStyleBasedOn {
    pub id: String,
    pub based_on: Option<String>,
}

impl<Op> protocol::MutationKind<SemioDocumentSnapshot, Op> for SetStyleBasedOn
where
    Op: protocol::Mutation<SemioDocumentSnapshot, Diff = crate::schema::diff::SemioDocumentDiff> + From<SetStyleBasedOn>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "style-based-on", kind: "set-style-based-on", record: "SetStyleBasedOn" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set style based on", "Basis des Stils setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioDocumentSnapshot, Op> for SetStyleBasedOn
where
    Op: protocol::ColdMutation<SemioDocumentSnapshot, ReferenceDiff = crate::schema::diff::SemioDocumentDiff> + From<SetStyleBasedOn>,
{
    fn diff(&self, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioDocumentSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
