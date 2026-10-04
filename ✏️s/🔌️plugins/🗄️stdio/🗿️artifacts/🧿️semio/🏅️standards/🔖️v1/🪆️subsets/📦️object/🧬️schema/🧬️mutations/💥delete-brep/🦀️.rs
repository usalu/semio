//! 💥️ `delete-brep` — clears the object's `brep` CHILD slot. Idempotent (a no-op if already
//! empty); the inverse captures the escrowed handle from BASE so undo restores it exactly.

use crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct DeleteBrep {}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for DeleteBrep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "brep", kind: "delete-brep", record: "DeletedBrep" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete brep child", "B-Rep-Kind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["brep".to_string()]
    }
}
//#endregion 🔖️Payload
