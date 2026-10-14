//! 🌱️ Original intrinsic value retirement remains at its actual type owner.
use crate::RetainedCloneGrant;
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::mem::{ManuallyDrop,size_of};
struct ValueRetirement(ManuallyDrop<Option<crate::DslValue>>);
impl RetirementCursor for ValueRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        match self.0.take() {
            None | Some(crate::DslValue::Null) => RetirementStep::Complete,
            Some(crate::DslValue::Bool(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Number(value)) => RetirementStep::Child(match value {
                crate::Number::UInt(value) => value.retirement(),
                crate::Number::Int(value) => value.retirement(),
                crate::Number::Float(value) => value.retirement(),
            }),
            Some(crate::DslValue::String(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Bytes(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Array(value)) => RetirementStep::Child(value.retirement()),
            Some(crate::DslValue::Object(value)) => RetirementStep::Child(value.retirement()),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        match self.0.as_ref() {
            None | Some(crate::DslValue::Null) => Some(0),
            Some(crate::DslValue::Bool(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Number(value)) => match value {
                crate::Number::UInt(value) => value.retirement_birth_bytes(),
                crate::Number::Int(value) => value.retirement_birth_bytes(),
                crate::Number::Float(value) => value.retirement_birth_bytes(),
            },
            Some(crate::DslValue::String(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Bytes(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Array(value)) => value.retirement_birth_bytes(),
            Some(crate::DslValue::Object(value)) => value.retirement_birth_bytes(),
        }
    }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}
impl Drop for ValueRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.0.is_none(), "dynamic value retired before terminal-empty");
    }
}
impl RetireOwned for crate::DslValue {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(ValueRetirement(ManuallyDrop::new(Some(self))))
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<ValueRetirement>()) }
    fn controlled_retirement_supported() -> bool { true }
}

