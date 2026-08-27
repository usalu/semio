//! ℹ️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-info`.

use super::PdfMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{diff::{self, PdfDiff}, snapshot::{PdfInfo, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInfo {
    pub info: PdfInfo,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetInfo {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info", kind: "set-info", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_info(self.info.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        vec![PdfMutation::SetInfo(SetInfo { info: base.info.clone() })]
    }

    fn label(&self) -> String {
        "Set document info".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["Info".to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_identity_is_owned_by_this_leaf() {
        assert_eq!(<SetInfo as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-info");
    }
}
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
