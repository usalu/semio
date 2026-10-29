//! 💾️ Binary representation codec surface for `stdio.semio.drawing` (snapshot). The real
//! encode/decode lives on `SemioDrawingSnapshot`'s `pack::snapshot::ArtifactPack` impl
//! (📸️snapshot/🦀️.rs) -- this module exposes the protocol source for tooling.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::native::NativeF64;
use semio_s_artifact_stdio_semio_base::schema::geometry::native::NativeF32;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;

use semio_s_artifact_stdio_semio_base::io::binary::codec::write_bytes_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_bytes_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_str_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_str_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_point2;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_point2;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_transform;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_transform;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_f32_le;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_rgba;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_rgba;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_bool;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_bool;

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioDrawingSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioDrawingSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::native::NativeF64;
use semio_s_artifact_stdio_semio_base::schema::geometry::native::NativeF32;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use crate::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioDrawingSnapshot`] as a semio pack envelope — the binary twin of the DSL
/// text, produced by a SEPARATE codec, which is what makes the two committed encodings of one
/// document able to contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_drawing_pack(snapshot: &SemioDrawingSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioDrawingSnapshot`] — the inverse of
/// [`encode_semio_drawing_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_drawing_pack(bytes: &[u8]) -> Result<SemioDrawingSnapshot, String> {
    <SemioDrawingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
