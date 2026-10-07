//! 🧬️ Authoritative change-restart-interval mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeRestartIntervalMutation {
    pub restart_interval: Option<u16>,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ChangeRestartIntervalMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "restart-interval", kind: "change-restart-interval", record: "ChangeRestartInterval" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { restart_interval } = self;
        protocol::MutationOutcome::new(contribute(base, *restart_interval))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Result<Vec<JpgMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![JpgMutation::ChangeRestartInterval(ChangeRestartIntervalMutation { restart_interval: base.restart_interval })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change restart interval", "Neustartintervall ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-restart-interval".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, restart_interval: Option<u16>) -> JpgDiff {
    JpgDiff { restart_interval: (base.restart_interval != restart_interval).then_some(restart_interval), ..Default::default() }
}
//#endregion Semantics


