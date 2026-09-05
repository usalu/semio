//! 💾️ Binary representation grammar surface for `stdio.semio.video` (mutations): real binary op
//! frame — `format u8` + `tag u8` (the `SemioVideoMutation` variant ordinal) real and fully
//! described, the variant's own argument text an opaque trailing payload (video wave, replacing
//! the old `print_op().into_bytes()` text-as-binary shortcut) — actual encode/decode lives on
//! `SemioVideoMutation`'s `protocol::OpBinary` impl in the facet root `🦀️.rs`; this leaf
//! carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
