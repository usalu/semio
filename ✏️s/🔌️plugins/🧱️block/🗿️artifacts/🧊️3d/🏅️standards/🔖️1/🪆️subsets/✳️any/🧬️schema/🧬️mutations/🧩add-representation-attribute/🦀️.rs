//! 🧩 Block3d mutation — `AddRepresentationAttribute`: a member of a representation's nested `attributes`.

use crate::BlockAttribute;
use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Mutation
/// 🧩 `add-representation-attribute` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-representation-attribute")]
pub struct AddRepresentationAttribute {
    pub id: String,
    #[dsl(block)]
    pub attribute: BlockAttribute,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_representation_attribute(id: String, attribute: BlockAttribute) -> Block3dMutation {
    Block3dMutation::AddRepresentationAttribute(AddRepresentationAttribute { id, attribute, index: None })
}

/// 📍️ Builder — like [`add_representation_attribute`] but inserts the row at `index`.
pub fn add_representation_attribute_at(id: String, attribute: BlockAttribute, index: u32) -> Block3dMutation {
    Block3dMutation::AddRepresentationAttribute(AddRepresentationAttribute { id, attribute, index: Some(index) })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddRepresentationAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "representation-attribute", kind: "add-representation-attribute", record: "AddedRepresentationAttribute" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add attribute \"{}\" to representation \"{}\"", self.attribute.key, self.id), &format!("Attribut \"{}\" zu Repräsentation \"{}\" hinzufügen", self.attribute.key, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
