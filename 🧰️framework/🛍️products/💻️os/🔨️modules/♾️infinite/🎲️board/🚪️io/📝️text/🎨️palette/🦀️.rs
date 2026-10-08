//! 🔤️ Physical palette JSON admission with integer spelling normalization under caller ownership control.
use crate::infinite::board::BoardPaletteOverlay;
use semio_framework_value::{DslValue,FromValue,Number,NativeDecodeControl,ValueError};
pub fn decode_board_palette_overlay_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<BoardPaletteOverlay,ValueError>{
    let mut value=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    if let DslValue::Object(fields)=value.get_mut(){for (_,field)in fields{control.step()?;if let DslValue::Array(components)=field{for component in components{control.step()?;if let DslValue::Number(Number::Float(number))=component{if number.is_finite()&&number.fract()==0.0&&*number>=0.0&&*number<=255.0{*component=DslValue::Number(Number::UInt(*number as u64));}}}}}}
    BoardPaletteOverlay::from_value_controlled(value.get(),control)
}
