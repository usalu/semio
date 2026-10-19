//! (kind/2d/3d together, one property-panel gesture).
//! 🔌 Puzzle5d mutation — `ReplacePartGrip`: whole-value swap of one grip's presentation fields
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip;


//#region 🔖️Mutation
/// 🔌 `replace-part-grip` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-part-grip")]
pub struct ReplacePartGrip {
    pub part_id: String,
    pub grip_id: String,
    #[dsl(block)]
    pub new_grip: Puzzle5dGrip,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_part_grip<Op>(part_id: String, grip_id: String, new_grip: Puzzle5dGrip) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_part_grip::ReplacePartGrip>,
{
    Op::from(ReplacePartGrip { part_id, grip_id, new_grip })
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ReplacePartGrip
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ReplacePartGrip>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "part-grip", kind: "replace-part-grip", record: "ReplacedPartGrip" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace grip \"{}\" on part \"{}\"", self.grip_id, self.part_id), &format!("Griff \"{}\" an Bauteil \"{}\" ersetzen", self.grip_id, self.part_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.part_id.clone(), self.grip_id.clone()]
    }
}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ReplacePartGrip {


}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ReplacePartGrip {
    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
