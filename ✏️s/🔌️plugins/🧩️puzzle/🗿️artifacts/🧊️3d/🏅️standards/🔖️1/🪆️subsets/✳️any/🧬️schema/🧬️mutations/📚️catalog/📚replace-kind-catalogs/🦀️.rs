//! 📚 Puzzle3d mutation — `ReplaceKindCatalogs`: whole-value swap of the scene_snapshot-carried typed
//! kind-catalog bundle (`objects`/`vortices`/`cables`/`attractions` catalogs together, one
//! manifest-import gesture).
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use {semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs, semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot};

//#region 🔖️Mutation
/// 📚 `replace-kind-catalogs` payload — `None` clears the catalogs.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-kind-catalogs")]
pub struct ReplaceKindCatalogs {
    pub new_catalogs: Option<Puzzle3dKindCatalogs>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_kind_catalogs<Op: From<ReplaceKindCatalogs>>(new_catalogs: Option<Puzzle3dKindCatalogs>) -> Op {
    Op::from(ReplaceKindCatalogs { new_catalogs })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ReplaceKindCatalogs
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ReplaceKindCatalogs>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "kind-catalogs", kind: "replace-kind-catalogs", record: "ReplacedKindCatalogs" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace kind catalogs", "Artkataloge ersetzen")
    }
}
//#endregion 🔖️Mutation
