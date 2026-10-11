//! 🚫️ `delete-properties` — clears the object's `properties` CHILD slot. Idempotent; inverse
//! escrows from BASE.

use crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct DeleteProperties {}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for DeleteProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "properties", kind: "delete-properties", record: "DeletedProperties" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete properties child", "Eigenschaftenkind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["properties".to_string()]
    }
}
//#endregion 🔖️Payload
