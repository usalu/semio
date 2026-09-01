//! ✏️ Playbook mutation — `ChangeTitle`: sets the playbook document's own `title` scalar (nullable —
//! clearing the title is a legal edit). Whole-document scope, no address. Was
//! `PlaybookMutation::UpdatePlaybook` pre-migration; renamed because `update` requires a cohesive
//! multi-field facet and `title` is the document's only mutable root scalar.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-title")]
pub struct ChangeTitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub new_title: Option<String>,
}

/// 🏗️ Builder.
pub fn change_title_operation(new_title: Option<String>) -> PlaybookMutation {
    PlaybookMutation::ChangeTitle(ChangeTitle { new_title })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for ChangeTitle {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "playbook", kind: "change-title", record: "ChangedPlaybookTitle" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change playbook title to \"{}\"", self.new_title.clone().unwrap_or_default())
    }
}
//#endregion 🔖️Mutation
