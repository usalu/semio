//! 🚪️ block2d ← zip — foreign `Deserializer<Block2dSnapshot>` on the framework's `io_mechanism`
//! channel: decodes a REAL zip 2.0 container (stdio's own `decode_zip`) and rebuilds the snapshot
//! from its `snapshot.block2d.semio` member, falling back to `snapshot.json`. Exact inverse of the sibling
//! `📤️export` leaf, so `IoFidelity::Exact`.
//!
//! 🐛️ Repaired here (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): this file used to ignore its
//! `bytes` argument entirely and return `Ok(Block2dSnapshot::default())` — silent, total data loss on
//! every import.

use crate::artifacts::block2d::io::export::serializers::artifacts::zip::v2_0::any::{ZIP_DSL_ENTRY, ZIP_JSON_ENTRY};
use crate::artifacts::block2d::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
use crate::artifacts::block2d::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;
use crate::artifacts::block2d::Block2dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::zip::io::decode_zip;

/// 🎯️ The foreign dialect this leaf reads.
pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

/// 🎒️ Local-file-header magic every zip 2.0 archive opens with (APPNOTE §4.3.7).
pub const ZIP_MAGIC: &[u8] = b"PK\x03\x04";

/// 🎒️ Rebuilds this subset's snapshot from real zip 2.0 container bytes.
pub fn from_zip_bytes(bytes: &[u8]) -> Result<Block2dSnapshot, IoError> {
    let archive = decode_zip(bytes).map_err(|error| IoError { message: format!("zip→block2d: {error}"), diagnostics: Vec::new() })?;
    for (name, parse) in [(ZIP_DSL_ENTRY, from_dsl_text as fn(&str) -> Result<Block2dSnapshot, IoError>), (ZIP_JSON_ENTRY, from_json_text as fn(&str) -> Result<Block2dSnapshot, IoError>)] {
        let Some(entry) = archive.entries.iter().find(|entry| entry.name == name) else {
            continue;
        };
        let text = std::str::from_utf8(&entry.data).map_err(|error| IoError { message: format!("zip→block2d: `{name}` is not utf-8: {error}"), diagnostics: Vec::new() })?;
        return parse(text);
    }
    Err(IoError { message: format!("zip→block2d: archive carries neither `{ZIP_DSL_ENTRY}` nor `{ZIP_JSON_ENTRY}`"), diagnostics: Vec::new() })
}

/// 🧩️ `s.stdio.zip@2.0/*` → `s.block.block2d@1/*`.
pub struct ZipIntoBlock2d;

impl Deserializer<Block2dSnapshot> for ZipIntoBlock2d {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(ZIP_MAGIC) => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Block2dSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "zip→block2d: expected a binary zip payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_zip_bytes(bytes)?))
    }
}
