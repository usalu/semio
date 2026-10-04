//! 🧊 Block5d mutation — `UpdatePart3d`: the whole 2-field 3D-projection pose facet atomically (orientation quaternion + scale vector, always edited together in a 3D pose gizmo).

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🧊 `update-part-3d` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-part-3d")]
pub struct UpdatePart3d {
    pub new_orientation: Option<[f64; 4]>,
    pub new_scale: Option<[f64; 3]>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_part_3d(new_orientation: Option<[f64; 4]>, new_scale: Option<[f64; 3]>) -> Block5dMutation {
    Block5dMutation::UpdatePart3d(UpdatePart3d { new_orientation, new_scale })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for UpdatePart3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "part-3d", kind: "update-part3d", record: "UpdatedPart3d" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update part 3D pose", "3D-Lage des Bauteils aktualisieren")
    }
}
//#endregion 🔖️Mutation
