//! Puzzle5d mutation — `RotatePart3d`: changes a part's 3D-projection orientation quaternion.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// `rotate-part3d` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rotate-part3d")]
pub struct RotatePart3d {
    pub id: String,
    pub new_orientation: Option<[f64; 4]>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for RotatePart3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "part", kind: "rotate-part3d", record: "RotatedPart3d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate part \"{}\" (3d)", self.id), &format!("Bauteil \"{}\" (3D) drehen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_part_3d(id: String, new_orientation: Option<[f64; 4]>) -> Puzzle5dMutation {
    Puzzle5dMutation::RotatePart3d(RotatePart3d { id, new_orientation })
}
