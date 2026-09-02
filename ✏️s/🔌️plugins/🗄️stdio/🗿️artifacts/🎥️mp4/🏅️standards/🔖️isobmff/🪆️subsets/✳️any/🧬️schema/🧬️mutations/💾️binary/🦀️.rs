//! 💾️ Binary representation codec surface for `stdio.mp4` (mutations) — the real op binary
//! codec is `protocol::OpBinary` in ../🦀️.rs (`encode_op`/`decode_op`, shared tagged records).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
