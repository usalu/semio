//! 📥️ Physical snapshot assembly publishes only a completely admitted semantic host input.
use crate::infinite::board::ports::directed_normal::BoardHost;
use semio_framework_value::NativeDecodeControl;
pub fn load_board_snapshot_json(host:&mut BoardHost,source:&str,control:&mut NativeDecodeControl<'_>)->bool{match super::snapshot::decode_board_snapshot_json(source,control){Ok(snapshot)=>host.load_board_snapshot(snapshot),Err(_)=>false}}
