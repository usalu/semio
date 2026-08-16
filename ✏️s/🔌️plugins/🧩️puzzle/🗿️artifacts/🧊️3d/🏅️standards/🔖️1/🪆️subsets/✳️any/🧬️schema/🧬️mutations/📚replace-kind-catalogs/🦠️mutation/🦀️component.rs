//! 📚 Puzzle3d mutation — `ReplaceKindCatalogs`: whole-value swap of the fixture-carried typed
//! kind-catalog bundle (`objects`/`vortices`/`cables`/`attractions` catalogs together, one
//! manifest-import gesture).
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::{Puzzle3dKindCatalogs, Puzzle3dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📚 `replace-kind-catalogs` payload — `None` clears the catalogs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-kind-catalogs")]
pub struct ReplaceKindCatalogs {
    pub new_catalogs: Option<Puzzle3dKindCatalogs>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_kind_catalogs(new_catalogs: Option<Puzzle3dKindCatalogs>) -> Puzzle3dMutation {
    Puzzle3dMutation::ReplaceKindCatalogs(ReplaceKindCatalogs { new_catalogs })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ReplaceKindCatalogs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "kind-catalogs", kind: "replace-kind-catalogs", record: "ReplacedKindCatalogs" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Replace kind catalogs".to_string()
    }
}
//#endregion 🔖️Mutation
