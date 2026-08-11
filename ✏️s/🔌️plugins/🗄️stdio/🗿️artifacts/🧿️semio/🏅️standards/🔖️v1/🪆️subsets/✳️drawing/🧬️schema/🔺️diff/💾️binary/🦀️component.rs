//! 💾️ Binary representation codec surface for `stdio.semio.drawing` (diff). The real
//! encode/decode is `SemioDrawingDiff`'s hand-rolled `protocol::DiffCodec` impl
//! (../🦀️component.rs) -- no separate envelope, the bytes ARE the text-facet grammar verbatim.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
