//! 📚 Puzzle2d mutation — `ReplaceKindCatalogs`: whole-value swap of the fixture-carried typed
//! kind-catalog bundle (`nodes`/`handles`/`edges`/`wires` catalogs together, one manifest-import
//! gesture).
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::{Puzzle2dKindCatalogs, Puzzle2dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📚 `replace-kind-catalogs` payload — `None` clears the catalogs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-kind-catalogs")]
pub struct ReplaceKindCatalogs {
    pub new_catalogs: Option<Puzzle2dKindCatalogs>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_kind_catalogs(new_catalogs: Option<Puzzle2dKindCatalogs>) -> Puzzle2dMutation {
    Puzzle2dMutation::ReplaceKindCatalogs(ReplaceKindCatalogs { new_catalogs })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ReplaceKindCatalogs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "kind-catalogs", kind: "replace-kind-catalogs", record: "ReplacedKindCatalogs" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Replace kind catalogs".to_string()
    }
}
//#endregion 🔖️Mutation
