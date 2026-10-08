//! 👁️ Physical scene admission resolves authored visibility before typed publication.
use crate::infinite::board::{BoardVisibility,board_visible_option};
use crate::infinite::board::ports::directed::SceneDescriptor;
use semio_framework_value::{DslValue,FromValue,NativeDecodeControl,ValueError};
pub fn decode_board_scene_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<SceneDescriptor,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    let mut scene=SceneDescriptor::from_value_controlled(raw.get(),control)?.guard_decoded();
    macro_rules! normalize {($field:ident)=>{if let Some(rows)=raw.get().get(stringify!($field)).and_then(|value|value.as_array()){for (entry,row)in scene.get_mut().$field.iter_mut().zip(rows){control.step()?;entry.visible=board_visible_option(&BoardVisibility{hidden:row.get("hidden").and_then(|value|value.as_bool()),visible:row.get("visible").and_then(|value|value.as_bool()),locked:None});}}};}
    normalize!(nodes);normalize!(handles);normalize!(edges);normalize!(wires);
    Ok(scene.take())
}
