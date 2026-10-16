//! 🔏️ Sealed: operations that refuse to touch sealed content.

use super::{clean_relative, fail, MoveError, Workspace};
use std::collections::BTreeSet;

//#region 🔏️Sealed

/// 🔏️ Repository-relative path of the taxonomy that registers every digest-sealed evidence document.
pub const SEALED_TAXONOMY_PATH: &str = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";

/// 🔏️ Every live digest-sealed evidence document the workspace taxonomy registers; a path-renaming plan never rewrites,
/// moves or relocates one. A workspace without that taxonomy seals nothing.
pub fn sealed_document_paths(workspace: &Workspace) -> Result<BTreeSet<String>, MoveError> {
    let Some(content) = workspace.file(SEALED_TAXONOMY_PATH) else {
        return Ok(BTreeSet::new());
    };
    let taxonomy: serde_json::Value = serde_json::from_str(content).map_err(|error| MoveError(error.to_string()))?;
    let mut sealed = BTreeSet::new();
    for key in ["frozenCoordinateEvidenceContracts", "frozenMarkdownCoordinateEvidenceContracts"] {
        let Some(contracts) = taxonomy.get(key).and_then(serde_json::Value::as_object) else {
            continue;
        };
        for contract in contracts.values().filter(|contract| contract.get("retired").is_none()) {
            if let Some(path) = contract.get("path").and_then(serde_json::Value::as_str).filter(|path| !path.is_empty()) {
                sealed.insert(path.to_string());
            }
        }
    }
    Ok(sealed)
}

/// 🛡️ The sealed documents at `path` or beneath it, byte ordered.
pub(super) fn sealed_within(sealed: &BTreeSet<String>, path: &str) -> Vec<String> {
    let prefix = format!("{path}/");
    sealed.iter().filter(|candidate| candidate.as_str() == path || candidate.starts_with(&prefix)).cloned().collect()
}

/// 🚫️ Refuses a file or folder move whose source holds digest-sealed evidence.
pub(super) fn refuse_sealed_move(workspace: &Workspace, source: &str) -> Result<(), MoveError> {
    let found = sealed_within(&sealed_document_paths(workspace)?, &clean_relative(source));
    if found.is_empty() {
        Ok(())
    } else {
        fail(format!("Refusing to move digest-sealed evidence: {}", found.join(", ")))
    }
}

//#endregion 🔏️Sealed
