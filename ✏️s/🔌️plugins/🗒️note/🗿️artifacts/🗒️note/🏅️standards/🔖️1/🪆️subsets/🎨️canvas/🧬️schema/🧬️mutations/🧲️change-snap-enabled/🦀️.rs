//! 🧲 Note mutation — `ChangeSnapEnabled`: sets snap-to-grid enabled.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧲 `change-snap-enabled` payload — sets snap-to-grid enabled.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-snap-enabled")]
pub struct ChangeSnapEnabled {
    pub new_enabled: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_snap_enabled(new_enabled: Option<bool>) -> NoteMutation {
    NoteMutation::ChangeSnapEnabled(ChangeSnapEnabled { new_enabled })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ChangeSnapEnabled {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "snap-enabled", kind: "change-snap-enabled", record: "ChangedSnapEnabled" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change snap enabled to {:?}", self.new_enabled)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
