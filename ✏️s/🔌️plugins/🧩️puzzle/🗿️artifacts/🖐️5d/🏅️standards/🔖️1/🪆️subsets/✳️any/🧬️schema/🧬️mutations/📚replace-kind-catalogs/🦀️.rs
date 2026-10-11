//! 📚 Puzzle5d mutation — `ReplaceKindCatalogs`: whole-value swap of the fixture-carried typed
//! kind-catalog bundle (`parts`/`grips`/`fasteners`/`ropes` catalogs together, one manifest-import
//! gesture).
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::{Puzzle5dKindCatalogs, Puzzle5dSnapshot};

//#region 🔖️Mutation
/// 📚 `replace-kind-catalogs` payload — `None` clears the catalogs.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-kind-catalogs")]
pub struct ReplaceKindCatalogs {
    pub new_catalogs: Option<Puzzle5dKindCatalogs>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_kind_catalogs(new_catalogs: Option<Puzzle5dKindCatalogs>) -> Puzzle5dMutation {
    Puzzle5dMutation::ReplaceKindCatalogs(ReplaceKindCatalogs { new_catalogs })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ReplaceKindCatalogs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "kind-catalogs", kind: "replace-kind-catalogs", record: "ReplacedKindCatalogs" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace kind catalogs", "Artkataloge ersetzen")
    }
}
//#endregion 🔖️Mutation
