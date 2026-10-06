//! binary rep note.diff.pack
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

semio_framework_os_kernel::diff_binary!(crate::standards::v1::subsets::any::schema::diff::PlaybookDiff);
