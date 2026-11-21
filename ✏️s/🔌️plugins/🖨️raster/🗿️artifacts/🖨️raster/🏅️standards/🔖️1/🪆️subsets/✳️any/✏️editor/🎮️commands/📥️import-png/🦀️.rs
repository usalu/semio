//! 📥️ Opens the bounded PNG picker and hands its result to the retained Raster editor.
use crate::config::{RasterConfig, RasterConfigMutation};
use semio_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, RequestId};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;

pub const MAX_FILE_BYTES: u32 = 3 * 1024 * 1024 - 1024;
const MAX_BASE64_BYTES: usize = (MAX_FILE_BYTES as usize).div_ceil(3) * 4;
pub const MAX_PAYLOAD_BYTES: usize = MAX_BASE64_BYTES + 37;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-png")]
pub struct ImportPng {}

pub fn handle(_: &ImportPng, _: &ArtifactView<'_, RasterSnapshot>, _: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::RequestFileOpen { req: RequestId(0x5253504e), accept: "image/png,.png".into(), read_as: Some("dataUrl".into()), import_action: "importPngFile".into(), args: None, multiple: false, max_file_bytes: Some(MAX_FILE_BYTES) }))
}

/// 📐️ Finds the extent of the shell's bounded binary data URL header.
pub fn base64_offset(payload: &str) -> Option<usize> {
    const PREFIXES: [&str; 2] = ["data:image/png;base64,", "data:application/octet-stream;base64,"];
    PREFIXES.into_iter().find_map(|prefix| {
        let base64 = payload.strip_prefix(prefix)?;
        if base64.is_empty() || base64.len() > MAX_BASE64_BYTES { return None; }
        let decoded = base64.len().div_ceil(4) * 3 - usize::from(base64.ends_with('=')) - usize::from(base64.ends_with("=="));
        (decoded <= MAX_FILE_BYTES as usize).then_some(prefix.len())
    })
}

#[path = "📄️file/🦀️.rs"]
pub mod file;
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
