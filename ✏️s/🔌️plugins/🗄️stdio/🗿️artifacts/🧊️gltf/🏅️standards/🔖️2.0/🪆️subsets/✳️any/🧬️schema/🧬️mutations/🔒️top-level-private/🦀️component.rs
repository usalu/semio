//! 🔒 Pure mechanics private to executable document-level glTF leaves.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfTopLevelMutationRejection { pub code: String, pub path: String, pub detail: String }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn reject(code: impl Into<String>, path: impl Into<String>, detail: impl Into<String>) -> GltfTopLevelMutationRejection { GltfTopLevelMutationRejection { code: code.into(), path: path.into(), detail: detail.into() } }
