//! 🏷 Puzzle5d mutation — `RenamePuzzle5d`: changes the document's display label (the closest
//! thing this fixture has to an identity field).
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Mutation
/// 🏷 `rename-puzzle5d` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-puzzle5d")]
pub struct RenamePuzzle5d {
    pub new_label: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_puzzle5d<Op>(new_label: Option<String>) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::RenamePuzzle5d>,
{
    Op::from(RenamePuzzle5d { new_label })
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for RenamePuzzle5d
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<RenamePuzzle5d>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "puzzle5d", kind: "rename-puzzle5d", record: "RenamedPuzzle5d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Rename puzzle5d", "Puzzle 5D umbenennen")
    }
}
//#endregion 🔖️Mutation
