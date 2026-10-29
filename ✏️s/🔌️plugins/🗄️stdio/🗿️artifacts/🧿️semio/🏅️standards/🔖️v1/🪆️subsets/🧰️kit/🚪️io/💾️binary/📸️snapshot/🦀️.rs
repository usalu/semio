//! 💾️ Binary representation codec surface for `s.stdio.semio.kit` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

use semio_s_artifact_stdio_semio_base::io::binary::codec::write_ref;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_ref;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_child;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_child;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_child_opt;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_child_opt;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_pin(out: &mut Vec<u8>, p: &store::LinkPin) {
    match p {
        store::LinkPin::Head => out.push(0),
        store::LinkPin::Checkpoint { id } => {
            out.push(1);
            write_str_lp(out, id);
        }
        store::LinkPin::Snapshot { blob } => {
            out.push(2);
            write_str_lp(out, &blob.hash);
            store::pack_rt::write_varint_u64(out, blob.size);
            write_str_lp(out, &blob.media_type);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_pin(reader: &mut store::ByteReader<'_>) -> Result<store::LinkPin, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(store::LinkPin::Head),
        1 => Ok(store::LinkPin::Checkpoint { id: read_str_lp(reader)? }),
        2 => {
            let hash = read_str_lp(reader)?;
            let size = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let media_type = read_str_lp(reader)?;
            Ok(store::LinkPin::Snapshot { blob: store::BlobRef { hash, size, media_type } })
        }
        other => Err(format!("unsupported link pin tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_link(out: &mut Vec<u8>, l: &store::ArtifactLink) {
    write_ref(out, &l.target);
    write_pin(out, &l.pin);
    write_str_lp(out, &l.role);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_link(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactLink, String> {
    let target = read_ref(reader)?;
    let pin = read_pin(reader)?;
    let role = read_str_lp(reader)?;
    Ok(store::ArtifactLink { target, pin, role })
}

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioKitSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioKitSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use crate::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioKitSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_kit_pack(snapshot: &SemioKitSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioKitSnapshot`] — the inverse of
/// [`encode_semio_kit_pack`], reading `../../🖼️assets/🪑️furniture/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_kit_pack(bytes: &[u8]) -> Result<SemioKitSnapshot, String> {
    <SemioKitSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
