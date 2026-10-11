//! 🌱 Puzzle3d mutation — `CreateObject`: brings a new id-keyed object into existence.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::{Puzzle3dObject, Puzzle3dSnapshot};

//#region 🔖️Mutation
/// 🌱 `create-object` payload — full initial payload at an optional FINAL-state `index` (`None`
/// appends). A duplicate `object.id` is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-object")]
pub struct CreateObject {
    #[dsl(block)]
    pub object: Puzzle3dObject,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_object(object: Puzzle3dObject, index: Option<usize>) -> Puzzle3dMutation {
    Puzzle3dMutation::CreateObject(CreateObject { object, index })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for CreateObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "object", kind: "create-object", record: "CreatedObject" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create object \"{}\"", self.object.id), &format!("Objekt \"{}\" erstellen", self.object.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object.id.clone()]
    }
}
//#endregion 🔖️Mutation
