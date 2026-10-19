//! 🚫️ Missing-target command preserving the law helper's positive error fixture.

use super::super::{CounterDiff, CounterMutation};
use semio_framework_os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct AddMissingCounter {}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddMissingCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "missing-counter", kind: "add-missing-counter", record: "AddedMissingCounter" };


}

impl crate::os_spr::ColdMutationKind<i64, CounterMutation> for AddMissingCounter {
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> {
        MutationOutcome::error("mutation.target-missing", "target absent", ["thing"])
    }
    fn inverse(&self, _base: &i64) -> Result<Vec<CounterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        Vec::new()
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add to missing counter", "Zum fehlenden Zähler addieren")
    }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddMissingCounter {
    fn print_op(&self) -> String {
        "add-missing-counter".into()
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if line == "add-missing-counter" {
            Ok(Self {})
        } else {
            Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected add-missing-counter", semio_framework_diagnostic::TextSpan::at(1, 1)))
        }
    }
}
//#endregion 📜️Text

//#region 🧪️Contract
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contract
