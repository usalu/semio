//! 💾️ Binary representation grammar surface for `stdio.semio.video` (diff): real binary diff
//! frame — `format u8` + `presence u8` (bit0 = `streams`) real and fully described, the optional
//! `streams` blob an opaque trailing payload (video wave, replacing the old
//! `print_diff().into_bytes()` text-as-binary shortcut) — actual encode/decode lives on
//! `SemioVideoDiff`'s `protocol::DiffCodec` impl in the facet root `🦀️.rs`; this leaf
//! carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
