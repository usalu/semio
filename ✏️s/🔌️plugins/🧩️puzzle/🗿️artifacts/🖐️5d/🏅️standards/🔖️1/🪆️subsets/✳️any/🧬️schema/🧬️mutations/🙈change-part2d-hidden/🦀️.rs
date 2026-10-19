//! Puzzle5d mutation — `ChangePart2dHidden`: changes a part's 2D-projection hidden flag.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// `change-part2d-hidden` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part2d-hidden")]
pub struct ChangePart2dHidden {
    pub id: String,
    pub new_hidden: Option<bool>,
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ChangePart2dHidden
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ChangePart2dHidden>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part", kind: "change-part2d-hidden", record: "ChangedPart2dHidden" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change part \"{}\" 2d hidden", self.id), &format!("2D-Ausblendung von Bauteil \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl protocol::ColdMutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ChangePart2dHidden {
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

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_2d_hidden<Op>(id: String, new_hidden: Option<bool>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::ChangePart2dHidden>,
{
    Op::from(ChangePart2dHidden { id, new_hidden })
}
