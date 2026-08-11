//! 💾️ Binary representation codec surface for `stdio.semio.drawing` (mutation). The real
//! encode/decode is `SemioDrawingMutation`'s hand-rolled `protocol::OpBinary` impl
//! (../🦀️component.rs) -- `serde_json::to_vec`, no separate envelope.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
