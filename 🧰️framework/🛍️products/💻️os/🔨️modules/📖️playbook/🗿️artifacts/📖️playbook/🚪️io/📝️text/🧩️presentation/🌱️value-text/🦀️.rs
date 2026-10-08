//! 🔤️ Controlled JSON text emission at the value-input presentation boundary.
use semio_framework_value::{DslValue,NativeEncodeControl,ValueError};
/// 🛫️ Emits owned semantic input under the caller's progress, cancellation and allocation control.
pub fn generation_value_input_text_v1(value:&DslValue,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{
    semio_framework_pack_json::to_json_string_controlled(value,control)
}
