//! ➕️ `add-type` — appends a new TYPE to the kit's catalog (id-keyed, no positional meaning).

use crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddType {
    pub id: String,
    pub name: String,
    pub category: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for AddType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "type", kind: "add-type", record: "AddedType" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add type {}", self.id), &format!("Typ {} hinzufügen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
