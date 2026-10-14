//! 🏠️ Build-tool-neutral workspace identity with caller-owned JSON decoding.

use std::{fs, path::Path};

/// 🪪️ Requires regular build and package markers with an explicit workspace declaration.
pub fn is_workspace_root(root: &Path, declares_workspaces: impl FnOnce(&str) -> Result<bool, String>) -> Result<bool, String> {
    if !["📋️project.json", "package.json"].iter().all(|name| fs::symlink_metadata(root.join(name)).is_ok_and(|metadata| metadata.file_type().is_file())) { return Ok(false); }
    declares_workspaces(&fs::read_to_string(root.join("package.json")).map_err(|error| error.to_string())?)
}
