//! 🧩️ Puzzle5d mutation — `ChangePart2dIcon`: changes a part's 2D-projection icon.
use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🔩️ `change-part2d-icon` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-part2d-icon")]
pub struct ChangePart2dIcon {
    pub id: String,
    pub new_icon_kind: Option<String>,
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for ChangePart2dIcon
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<ChangePart2dIcon>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part", kind: "change-part2d-icon", record: "ChangedPart2dIcon" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change part \"{}\" 2d icon", self.id), &format!("2D-Symbol von Bauteil \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for ChangePart2dIcon
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<ChangePart2dIcon>,
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

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_part_2d_icon<Op>(id: String, new_icon_kind: Option<String>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::ChangePart2dIcon>,
{
    Op::from(ChangePart2dIcon { id, new_icon_kind })
}
