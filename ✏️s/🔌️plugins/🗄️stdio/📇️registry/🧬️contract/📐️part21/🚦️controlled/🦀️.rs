//! 🚦️ Controlled Part21 construction retains every partial original in the caller recipient.
use super::{DslValue,FromValue,Part21Decimal,Part21Document,Part21Header,Part21Instance,Part21Value,ToValue,ValueError};
use semio_framework_value::{NativeEncodeControl, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
#[path="🛬️input/🦀️.rs"]
pub(super) mod input;
#[path="🛫️output/🦀️.rs"]
pub(super) mod output;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn required<'v>(entries:&'v[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<&'v DslValue,ValueError>{DslValue::field_controlled(entries,key,control)?.ok_or_else(||invalid("missing Part21 field"))}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
