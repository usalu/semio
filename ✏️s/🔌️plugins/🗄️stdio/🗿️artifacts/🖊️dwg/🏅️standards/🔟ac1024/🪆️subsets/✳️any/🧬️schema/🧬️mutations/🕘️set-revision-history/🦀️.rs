//! 🕘️ `set-revision-history` — replaces the revision history block of the container. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use crate::schema::snapshot::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetRevisionHistory {
    pub revision_history: DwgRevisionHistory,
}

impl protocol::MutationKind<DwgSnapshot, DwgMutation> for SetRevisionHistory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "revision-history", kind: "set-revision-history", record: "SetRevisionHistory" };

    fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<<DwgMutation as Mutation<DwgSnapshot>>::Diff> {
        protocol::MutationOutcome::new(DwgDiff { revision_history: (base.revision_history != self.revision_history).then(|| self.revision_history.clone()), ..DwgDiff::default() })
    }
    fn inverse(&self, base: &DwgSnapshot) -> Result<Vec<DwgMutation>, semio_framework_value::ValueError> {
        Ok((base.revision_history != self.revision_history).then(|| DwgMutation::SetRevisionHistory(set_revision_history::SetRevisionHistory { revision_history: base.revision_history.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set revision history", "Revisionsverlauf setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
