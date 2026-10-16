//! 📌️ Cache: the content-digest tree cache envelope and its canonical encoding.
//!
//! Holds the `📌️Cache`, `🔣️Encoding` regions of the crate root; the root re-exports its items.

use super::*;

//#region 📌️Cache

/// 💿️ The metadata a cached tree carries; a build is reused only when all of it still matches.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeCacheMeta {
    #[serde(rename = "SchemaVersion")]
    pub schema_version: i64,
    #[serde(rename = "Fingerprint")]
    pub fingerprint: String,
    #[serde(rename = "IncludeSections")]
    pub include_sections: bool,
    #[serde(rename = "ContentDigest", default, skip_serializing_if = "String::is_empty")]
    pub content_digest: String,
}

impl TreeCacheMeta {
    /// 🆕️ Builds the metadata of a freshly computed tree.
    pub fn of(tree: &TreeNode, fingerprint: &str, include_sections: bool) -> TreeCacheMeta {
        TreeCacheMeta {
            schema_version: TREE_CACHE_SCHEMA_VERSION,
            fingerprint: fingerprint.to_string(),
            include_sections,
            content_digest: tree_content_digest(tree),
        }
    }
}

/// ♻️ Whether a cached tree may be reused for a fingerprint and section choice.
pub fn tree_cache_is_valid(meta: &TreeCacheMeta, fingerprint: &str, include_sections: bool) -> bool {
    meta.schema_version == TREE_CACHE_SCHEMA_VERSION && meta.fingerprint == fingerprint && meta.include_sections == include_sections
}

/// 🔏️ The content digest of a tree: the SHA-256 of its canonical JSON encoding.
pub fn tree_content_digest(tree: &TreeNode) -> String {
    sha256_hex(canonical_tree_json(tree).as_bytes())
}

/// 🔣️ The canonical JSON encoding of a tree, with every map key in sorted order.
pub fn canonical_tree_json(tree: &TreeNode) -> String {
    serde_json::to_string(tree).unwrap_or_default()
}

/// 🗜️ The gzip-wrapped canonical JSON of a tree: the payload half of the cache envelope, the
/// same shape `github.com/usalu/semio/repo/tree` writes to `tree.json.gz`.
pub fn encode_tree_cache(tree: &TreeNode) -> Vec<u8> {
    gzip_encode(canonical_tree_json(tree).as_bytes())
}

/// 🗜️ The tree read back out of a gzip-wrapped cache payload.
pub fn decode_tree_cache(payload: &[u8]) -> Result<TreeNode, String> {
    let plain = gzip_decode(payload)?;
    let text = String::from_utf8(plain).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

//#endregion 📌️Cache

//#region 🔣️Encoding

/// 🔣️ Encodes any tree value as JSON, the shape both implementations project through.
pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}

//#endregion 🔣️Encoding
