//! 💾️ Binary representation codec surface for `stdio.semio.image` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use framework_schema::ArtifactSchema;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn colorspace_tag(c: SemioColorspace) -> u8 {
    match c {
        SemioColorspace::Rgb => 0,
        SemioColorspace::Rgba => 1,
        SemioColorspace::Grayscale => 2,
        SemioColorspace::GrayscaleAlpha => 3,
        SemioColorspace::Indexed => 4,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn colorspace_from_tag(tag: u8) -> Result<SemioColorspace, String> {
    match tag {
        0 => Ok(SemioColorspace::Rgb),
        1 => Ok(SemioColorspace::Rgba),
        2 => Ok(SemioColorspace::Grayscale),
        3 => Ok(SemioColorspace::GrayscaleAlpha),
        4 => Ok(SemioColorspace::Indexed),
        other => Err(format!("unsupported colorspace tag {other}")),
    }
}

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioImageSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioImageSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioImageSnapshot`] —
/// the binary half of the same bridge, so a caller outside this crate can check the two codecs
/// against each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_image_pack(bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    <SemioImageSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `pack::snapshot::ArtifactPack::encode_pack` inverse of [`decode_semio_image_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_image_pack(snapshot: &SemioImageSnapshot) -> Vec<u8> {
    <SemioImageSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
