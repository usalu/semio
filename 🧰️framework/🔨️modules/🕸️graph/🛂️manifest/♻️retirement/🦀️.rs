//! ♻️ Typed property-tree retirement releases one nested owner per granted step.

use super::PropertyValue;
use dsl_core::os_store::retirement::{RetireOwned, RetirementCursor, RetirementStep};

struct PropertyRetirement(std::mem::ManuallyDrop<Option<PropertyValue>>);
impl RetirementCursor for PropertyRetirement {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        match self.0.take() {
            None | Some(PropertyValue::Null) => RetirementStep::Complete,
            Some(PropertyValue::Bool(value)) => RetirementStep::Child(value.retirement()),
            Some(PropertyValue::Number(value)) => RetirementStep::Child(value.retirement()),
            Some(PropertyValue::String(value)) => RetirementStep::Child(value.retirement()),
            Some(PropertyValue::Array(value)) => RetirementStep::Child(value.retirement()),
            Some(PropertyValue::Object(value)) => RetirementStep::Child(value.retirement()),
        }
    }
    fn terminal_is_empty(&self) -> bool { self.0.is_none() }
}
impl Drop for PropertyRetirement {
    fn drop(&mut self) { assert!(self.0.is_none(), "property tree retired before terminal-empty"); }
}
impl RetireOwned for PropertyValue {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(PropertyRetirement(std::mem::ManuallyDrop::new(Some(self)))) }
}
