//! 🌳️ Exact persistent local interaction roots compose defining typed physical retirement.

use super::{DomainSelection,LocalInteractionState,SelectionMode};
use semio_framework_value::ordered::OrderedMap;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::ValueError;

#[path="🩹️update/🦀️.rs"]
mod update;
pub use update::{LocalInteractionRootPatch,LocalInteractionRootUpdate,LocalInteractionWork};

#[derive(Clone,Default,Debug,PartialEq,Eq)]
pub struct LocalInteractionRoot {
    selection:OrderedMap<DomainSelection>,
    active_mode:OrderedMap<SelectionMode>,
    active_granularity:OrderedMap<String>,
}

impl semio_framework_value::ToValue for LocalInteractionRoot {
    fn to_value(&self)->semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("selection".to_string(),semio_framework_value::ToValue::to_value(&self.selection)),
            ("activeMode".to_string(),semio_framework_value::ToValue::to_value(&self.active_mode)),
            ("activeGranularity".to_string(),semio_framework_value::ToValue::to_value(&self.active_granularity)),
        ])
    }
}

semio_framework_value::artifact_retire_struct!(LocalInteractionRoot {selection,active_mode,active_granularity});

impl LocalInteractionRoot {
    /// 🧊️ Explicit cold transport construction provides no retained-work credit.
    pub fn from_cold(state:LocalInteractionState)->Self {
        Self {selection:state.selection.into_iter().collect(),active_mode:state.active_mode.into_iter().collect(),active_granularity:state.active_granularity.into_iter().collect()}
    }
    pub fn selection(&self)->&OrderedMap<DomainSelection>{&self.selection}
    pub fn active_mode(&self)->&OrderedMap<SelectionMode>{&self.active_mode}
    pub fn active_granularity(&self)->&OrderedMap<String>{&self.active_granularity}
    /// ♻️ Transfers the exact root inline; the defining driver admits every later work, birth and physical release.
    pub fn retire(self)->Result<ControlledRetirement<Self>,(ValueError,Self)>{ControlledRetirement::new(self)}
}
