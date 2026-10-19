//! ➕ Puzzle5d mutation — `AddPartGrip`: attaches a new rim grip to a part.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::{Puzzle5dSnapshot};
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGrip;


//#region 🔖️Mutation
/// ➕ `add-part-grip` payload — owner part id + new grip payload at an optional FINAL-state
/// `index` (`None` appends). A duplicate `grip.id` on the same part is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-part-grip")]
pub struct AddPartGrip {
    pub part_id: String,
    #[dsl(block)]
    pub grip: Puzzle5dGrip,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_part_grip<Op>(part_id: String, grip: Puzzle5dGrip, index: Option<usize>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::add_part_grip::AddPartGrip>,
{
    Op::from(AddPartGrip { part_id, grip, index })
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for AddPartGrip
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<AddPartGrip> + From<crate::standards::v1::subsets::any::schema::mutations::remove_part_grip::RemovePartGrip>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "part-grip", kind: "add-part-grip", record: "AddedPartGrip" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add grip \"{}\" to part \"{}\"", self.grip.id, self.part_id), &format!("Griff \"{}\" zu Bauteil \"{}\" hinzufügen", self.grip.id, self.part_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.part_id.clone(), self.grip.id.clone()]
    }
}


impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for AddPartGrip
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<AddPartGrip>,
{
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
