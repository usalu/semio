//#region 🧒️SetSlotChildren
use super::super::{TestDiff, TestMutation, TestSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧒️ Sets the exact membership of this document's one owned-child slot, `"slot"`, as canonical
/// `ArtifactRef` uris. Registering a member is the runtime's job; DECLARING it on the parent
/// snapshot is the app's, and only a declared member is admitted back by `ChildRestoreProjection`
/// when the parent document is reloaded.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SetSlotChildren {
    pub children: Vec<String>,
}

impl MutationKind<TestSnapshot, TestMutation> for SetSlotChildren {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "slot-children", kind: "set-slot-children", record: "SetSlotChildren" };

    fn diff(&self, _: &TestSnapshot) -> MutationOutcome<TestDiff> {
        MutationOutcome::new(TestDiff { count: None, label: None, slot: Some(self.children.clone()) })
    }

    fn inverse(&self, base: &TestSnapshot) -> Vec<TestMutation> {
        vec![Self { children: base.slot.iter().map(|child| child.target.to_uri()).collect() }.into()]
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &match self.children.len() {
                0 => "Clear the declared slot children".to_string(),
                1 => "Set 1 slot child reference".to_string(),
                count => format!("Set {count} slot child references"),
            },
            &match self.children.len() {
                0 => "Deklarierte Slot-Kinder leeren".to_string(),
                1 => "1 Slot-Kindverweis deklarieren".to_string(),
                count => format!("{count} Slot-Kindverweise deklarieren"),
            },
        )
    }
}

#[cfg(test)]
#[path = "../../../../../🧪️tests/🧒️test-app-document-set-slot-children-unit/🦀️.rs"]
mod tests;
//#endregion 🧒️SetSlotChildren
