//! 🚦️ Controlled Part21 construction retains every partial original in the caller recipient.
use super::{DslValue,FromValue,Part21Decimal,Part21Document,Part21Header,Part21Instance,Part21Value,ToValue,ValueError};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
#[path="🛬️input/🦀️.rs"]
pub(super) mod input;
#[path="🛫️output/🦀️.rs"]
pub(super) mod output;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn required<'v>(entries:&'v[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<&'v DslValue,ValueError>{DslValue::field_controlled(entries,key,control)?.ok_or_else(||invalid("missing Part21 field"))}
semio_framework_value::artifact_retire_struct!(Part21Decimal{negative,coefficient,scale,exponent});
semio_framework_value::artifact_retire_struct!(Part21Instance{id,entities});
semio_framework_value::artifact_retire_struct!(Part21Header{file_description,file_name,file_schema});
semio_framework_value::artifact_retire_struct!(Part21Document{header,instances});
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
