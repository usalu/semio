//! 📇️ kind-catalog bundle (`nodes`/`🐙️handles`/`edges`/`wires` catalogs together, one manifest-import
//! 📚 Puzzle2d mutation — `ReplaceKindCatalogs`: whole-value swap of the snapshot-carried typed
//! gesture).


use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use {crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs, crate::Puzzle2dSnapshot};

//#region 🔖️Mutation
/// 📚 `replace-kind-catalogs` payload — `None` clears the catalogs.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-kind-catalogs")]
pub struct ReplaceKindCatalogs {
    pub new_catalogs: Option<Puzzle2dKindCatalogs>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_kind_catalogs<Op: From<ReplaceKindCatalogs>>(new_catalogs: Option<Puzzle2dKindCatalogs>) -> Op {
    Op::from(ReplaceKindCatalogs { new_catalogs })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for ReplaceKindCatalogs
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<ReplaceKindCatalogs>,
 {

    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "kind-catalogs", kind: "replace-kind-catalogs", record: "ReplacedKindCatalogs" };



    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace kind catalogs", "Artkataloge ersetzen")
    }

}
impl<Op> protocol::ColdMutationKind<Puzzle2dSnapshot, Op> for ReplaceKindCatalogs
where
    Op: protocol::ColdMutation<Puzzle2dSnapshot, ReferenceDiff = Puzzle2dDiff> + From<ReplaceKindCatalogs>,
 {
fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
