//! 🗑️️ `remove-design` — removes the DESIGN matching `id` (pieces/connections included).
//! Idempotent no-op if absent; inverse escrows the FULL design (via `add-design` + `edit-design`,
//! a real 2-step inverse — `add-design` alone only creates an empty design).

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveDesign {
    pub id: String,
}

impl<Op> protocol::MutationKind<SemioKitSnapshot, Op> for RemoveDesign
where
    Op: protocol::Mutation<SemioKitSnapshot, Diff = crate::schema::diff::SemioKitDiff> + From<RemoveDesign> + From<crate::schema::mutations::add_design::AddDesign> + From<crate::schema::mutations::edit_design::EditDesign>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "design", kind: "remove-design", record: "RemovedDesign" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove design {}", self.id), &format!("Entwurf {} entfernen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
