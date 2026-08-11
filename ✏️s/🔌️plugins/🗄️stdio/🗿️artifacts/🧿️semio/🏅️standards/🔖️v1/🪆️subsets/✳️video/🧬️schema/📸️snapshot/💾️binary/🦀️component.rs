//! 💾️ Binary representation grammar surface for `stdio.semio.video` (snapshot): magic + version +
//! length-prefixed JSON body — actual encode/decode lives on `SemioVideoSnapshot`'s
//! `store::ArtifactPack` impl in the facet root `🦀️component.rs`; this leaf carries the
//! normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
