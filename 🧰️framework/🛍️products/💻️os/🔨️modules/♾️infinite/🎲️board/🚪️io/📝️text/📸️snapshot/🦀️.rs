//! 📸️ Physical snapshot JSON binds owned semantic records under caller control.
use crate::infinite::board::ports::directed::BoardSnapshot;
use semio_framework_value::{NativeDecodeControl,ValueError};
pub fn decode_board_snapshot_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<BoardSnapshot,ValueError>{semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)}
