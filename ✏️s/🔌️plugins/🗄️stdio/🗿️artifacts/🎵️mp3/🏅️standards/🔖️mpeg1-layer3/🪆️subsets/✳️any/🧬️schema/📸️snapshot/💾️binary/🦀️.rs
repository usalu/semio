//! 🎵️ Complete MP3 logical snapshot Binary protocol.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub const BINARY_MAGIC: &str = "stdio.mp3.snapshot";
