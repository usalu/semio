//! 🌐 Block5d mutation — `ChangeRepresentationMeshUrl`: a representation's `meshUrl`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🌐 `change-representation-mesh-url` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-representation-mesh-url")]
pub struct ChangeRepresentationMeshUrl {
    pub id: String,
    pub new_mesh_url: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_representation_mesh_url(id: String, new_mesh_url: Option<String>) -> Block5dMutation {
    Block5dMutation::ChangeRepresentationMeshUrl(ChangeRepresentationMeshUrl { id, new_mesh_url })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeRepresentationMeshUrl {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-mesh-url", record: "ChangedRepresentationMeshUrl" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change representation \"{}\" mesh URL", self.id), &format!("Netz-URL von Repräsentation \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
