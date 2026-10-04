//! 🚦️ Controlled Part21 field construction preserves the canonical owned model and partial retirement.
use super::{DslValue, FromValue, Part21Decimal, Part21Document, Part21Header, Part21Instance, Part21Value, ToValue, ValueError};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::{DecodedValue, NativeDecodeControl, NativeEncodeControl, ValueRefusalKind};

#[path = "🛬️input/🦀️.rs"]
pub(super) mod input;
#[path = "🛫️output/🦀️.rs"]
pub(super) mod output;

fn invalid(message: &'static str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}

fn required<'v>(entries: &'v [(String, DslValue)], key: &str, control: &mut NativeDecodeControl<'_>) -> Result<&'v DslValue, ValueError> {
    DslValue::field_controlled(entries, key, control)?.ok_or_else(|| invalid("missing Part21 field"))
}

fn retire_values(values: Vec<Part21Value>) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(values);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 cold retirement preserves its grant");
    }
}

pub(super) fn retire_decimal(decimal: Part21Decimal) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(decimal);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 decimal cold retirement preserves its grant");
    }
}

pub(super) fn retire_value(value: Part21Value) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(value);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 cold retirement preserves its grant");
    }
}

pub(super) fn retire_instance(instance: Part21Instance) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(instance);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 instance cold retirement preserves its grant");
    }
}

fn retire_entities(entities: Vec<(String, Vec<Part21Value>)>) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(entities);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 entities cold retirement preserves its grant");
    }
}

fn retire_name(name: String) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(name);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 name cold retirement preserves its grant");
    }
}

semio_framework_value::artifact_retire_struct!(Part21Decimal { negative, coefficient, scale, exponent });
semio_framework_value::artifact_retire_struct!(Part21Instance { id, entities });
semio_framework_value::artifact_retire_struct!(Part21Header { file_description, file_name, file_schema });
semio_framework_value::artifact_retire_struct!(Part21Document { header, instances });

impl RetireOwned for Part21Value {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Ref(value) => value.retirement(),
            Self::Str(value) | Self::Enum(value) => value.retirement(),
            Self::Int(value) => value.retirement(),
            Self::Real(value) => value.retirement(),
            Self::List(values) => values.retirement(),
            Self::Typed { name, items } => semio_framework_value::artifact_retirement_sequence![name, items],
            Self::Unset | Self::Derived => ().retirement(),
        }
    }
}

pub(super) fn retire_header(header: Part21Header) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(header);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 header cold retirement preserves its grant");
    }
}

pub(super) fn retire_document(document: Part21Document) {
    let mut retirement = semio_framework_value::retirement::owned_retirement(document);
    while !retirement.terminal_is_empty() {
        retirement.close_step(256, 65_536).expect("Part21 document cold retirement preserves its grant");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
