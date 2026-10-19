//! 👁️ Deliberately nondeterministic command retaining shared, nonserialized observation state.

use super::super::{CounterDiff, CounterMutation};
use semio_framework_os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct AddObservedCounter {
    #[serde(skip)]
    #[value(skip)]
    calls: std::sync::Arc<std::sync::atomic::AtomicI64>,
}
impl PartialEq for AddObservedCounter{fn eq(&self,other:&Self)->bool{std::sync::Arc::ptr_eq(&self.calls,&other.calls)||self.calls.load(std::sync::atomic::Ordering::Relaxed)==other.calls.load(std::sync::atomic::Ordering::Relaxed)}}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddObservedCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "observed-counter", kind: "add-observed-counter", record: "AddedObservedCounter" };


}

impl crate::os_spr::ColdMutationKind<i64, CounterMutation> for AddObservedCounter {
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> {
        let count = self.calls.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
        MutationOutcome::new(CounterDiff::delta(count))
    }
    fn inverse(&self, _base: &i64) -> Result<Vec<CounterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        Vec::new()
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add observed counter", "Beobachteten Zähler hinzufügen")
    }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddObservedCounter {
    fn print_op(&self) -> String {
        "add-observed-counter".into()
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if line == "add-observed-counter" {
            Ok(Self::default())
        } else {
            Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected add-observed-counter", semio_framework_diagnostic::TextSpan::at(1, 1)))
        }
    }
}
//#endregion 📜️Text

//#region 🧪️Contract
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contract
