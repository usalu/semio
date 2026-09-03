//! 🩹 Playbook mutation — `UpdateStep`: sets a step's header (`title` + `description`) atomically.
//! Both fields are always submitted together by the step-details form (never independently — no
//! app command edits just one), and `title` is always required, satisfying `update`'s "one cohesive
//! multi-field facet, all fields required" restriction. Deliberately excludes `blocks` (owned by
//! `add-block`/`remove-block`/`move-block`) — the pre-migration kernel payload embedded a whole
//! `PlaybookStep` here but its diff builder silently ignored the `blocks` field, a latent footgun
//! this payload shape removes by construction.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
// 🔬️ `Serialize`/`Deserialize` survive ONLY as a `#[cfg(test)]` differential oracle — committed
// `🧪️tests/<fixture>/🦀️.rs` fixture vectors decode/re-encode through them — never a production
// dependency of this crate.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "update-step")]
pub struct UpdateStep {
    pub step_id: String,
    pub title: String,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 🏗️ Builder.
pub fn update_step_operation(step_id: &str, title: String, description: Option<String>) -> PlaybookMutation {
    PlaybookMutation::UpdateStep(UpdateStep { step_id: step_id.into(), title, description })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for UpdateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "step", kind: "update-step", record: "UpdatedStep" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update step \"{}\"", self.title)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone()]
    }
}
//#endregion 🔖️Mutation
