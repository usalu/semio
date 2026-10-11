//! 📜️ `set-application-history` — replaces the application history block of the container. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use crate::schema::snapshot::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetApplicationHistory {
    pub application_history: DwgApplicationHistory,
}

impl protocol::MutationKind<DwgSnapshot, DwgMutation> for SetApplicationHistory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "application-history", kind: "set-application-history", record: "SetApplicationHistory" };

    fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<<DwgMutation as Mutation<DwgSnapshot>>::Diff> {
        protocol::MutationOutcome::new(DwgDiff { application_history: (base.application_history != self.application_history).then(|| self.application_history.clone()), ..DwgDiff::default() })
    }
    fn inverse(&self, base: &DwgSnapshot) -> Result<Vec<DwgMutation>, semio_framework_value::ValueError> {
        Ok((base.application_history != self.application_history).then(|| DwgMutation::SetApplicationHistory(set_application_history::SetApplicationHistory { application_history: base.application_history.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set application history", "Anwendungsverlauf setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
