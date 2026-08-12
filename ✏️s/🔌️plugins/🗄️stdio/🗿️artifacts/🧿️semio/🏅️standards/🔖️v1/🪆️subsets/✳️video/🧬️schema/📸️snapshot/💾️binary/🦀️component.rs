//! 💾️ Binary representation grammar surface for `stdio.semio.video` (snapshot): real
//! varint-length-prefixed binary pack frame — `format u8` + `schema` (length-prefixed UTF-8) real
//! and fully described, `streams` an opaque trailing payload (`protocol-array-of-records` gap;
//! video wave, replacing the old `serde_json::to_vec`-in-envelope shortcut) — actual encode/decode
//! lives on `SemioVideoSnapshot`'s `store::ArtifactPack` impl in the facet root
//! `🦀️component.rs`; this leaf carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
