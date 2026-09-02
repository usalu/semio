//! 💾️ Binary representation codec surface for `stdio.semio.drawing` (snapshot). The real
//! encode/decode lives on `SemioDrawingSnapshot`'s `store::ArtifactPack` impl
//! (📸️snapshot/🦀️.rs) -- this module exposes the protocol source for tooling.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
