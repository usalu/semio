//! 📄️ `remove-frame` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioImageDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_frame;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveFrame {
    pub index: usize,
}

impl<Op> protocol::MutationKind<SemioImageSnapshot, Op> for RemoveFrame
where
    Op: protocol::Mutation<SemioImageSnapshot, Diff = crate::schema::diff::SemioImageDiff> + From<RemoveFrame> + From<crate::schema::mutations::insert_frame::InsertFrame>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "frame", kind: "remove-frame", record: "RemoveFrame" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove frame", "Einzelbild entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioImageSnapshot, Op> for RemoveFrame
where
    Op: protocol::ColdMutation<SemioImageSnapshot, ReferenceDiff = crate::schema::diff::SemioImageDiff> + From<RemoveFrame> + From<crate::schema::mutations::insert_frame::InsertFrame>,
{
    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioImageSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
