//! 💾️ Binary representation codec surface for `stdio.semio` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::base::schema::snapshot::*;
use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;

/// 🔢️ The binary sibling of [`subset_tag`] — a real, individually protocol-walkable `u8` ordinal
/// (0-13, enum declaration order), used by the binary pack header instead of a length-prefixed name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn subset_ordinal(s: &SemioSubsetSnapshot) -> u8 {
    match s {
        SemioSubsetSnapshot::Brep(_) => 0,
        SemioSubsetSnapshot::Mesh(_) => 1,
        SemioSubsetSnapshot::Model(_) => 2,
        SemioSubsetSnapshot::Value(_) => 3,
        SemioSubsetSnapshot::Document(_) => 4,
        SemioSubsetSnapshot::Cad(_) => 5,
        SemioSubsetSnapshot::Drawing(_) => 6,
        SemioSubsetSnapshot::Image(_) => 7,
        SemioSubsetSnapshot::Video(_) => 8,
        SemioSubsetSnapshot::Audio(_) => 9,
        SemioSubsetSnapshot::Animation(_) => 10,
        SemioSubsetSnapshot::Presentation(_) => 11,
        SemioSubsetSnapshot::Flow(_) => 12,
        SemioSubsetSnapshot::Text(_) => 13,
        SemioSubsetSnapshot::Table(_) => 14,
        SemioSubsetSnapshot::Graph(_) => 15,
        SemioSubsetSnapshot::Object(_) => 16,
        SemioSubsetSnapshot::Kit(_) => 17,
    }
}

/// 🧪️ Real varint-length-prefixed binary envelope: `format u8` + `tag u8` (real
/// [`subset_ordinal`]) + varint-length-prefixed `schema` UTF-8, then the WRAPPED subset's own
/// full, already-real `ArtifactPack::encode_pack()` bytes as one opaque trailing payload — that
/// call already applies THAT subset's own `semio_format` envelope internally, so this is a real,
/// honest double-envelope (delegation, not a re-derivation of any subset's own binary layout).
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
pub(crate) fn encode_semio_snapshot_binary(snap: &SemioSnapshot) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    out.push(subset_ordinal(&snap.subset));
    write_bytes_lp(&mut out, snap.schema.as_bytes());
    let payload = match &snap.subset {
        SemioSubsetSnapshot::Brep(s) => <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Mesh(s) => <SemioMeshSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Model(s) => <SemioModelSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Value(s) => <SemioValueSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Document(s) => <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Cad(s) => <SemioCadSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Drawing(s) => <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Image(s) => <SemioImageSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Video(s) => <SemioVideoSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Audio(s) => <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Animation(s) => <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Presentation(s) => <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Flow(s) => <SemioFlowSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Text(s) => <SemioTextSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Table(s) => <SemioTableSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Graph(s) => <SemioGraphSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Object(s) => <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Kit(s) => <SemioKitSnapshot as store::ArtifactPack>::encode_pack(s),
    };
    out.extend_from_slice(&payload);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_semio_snapshot_binary(bytes: &[u8]) -> Result<SemioSnapshot, String> {
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let schema = String::from_utf8(read_bytes_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let payload = reader.read_bytes(reader.remaining()).map_err(|e| e.to_string())?;
    let subset = match tag {
        0 => SemioSubsetSnapshot::Brep(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        1 => SemioSubsetSnapshot::Mesh(<SemioMeshSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        2 => SemioSubsetSnapshot::Model(<SemioModelSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        3 => SemioSubsetSnapshot::Value(<SemioValueSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        4 => SemioSubsetSnapshot::Document(<SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        5 => SemioSubsetSnapshot::Cad(<SemioCadSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        6 => SemioSubsetSnapshot::Drawing(<SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        7 => SemioSubsetSnapshot::Image(<SemioImageSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        8 => SemioSubsetSnapshot::Video(<SemioVideoSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        9 => SemioSubsetSnapshot::Audio(<SemioAudioSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        10 => SemioSubsetSnapshot::Animation(<SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        11 => SemioSubsetSnapshot::Presentation(<SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        12 => SemioSubsetSnapshot::Flow(<SemioFlowSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        13 => SemioSubsetSnapshot::Text(<SemioTextSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        14 => SemioSubsetSnapshot::Table(<SemioTableSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        15 => SemioSubsetSnapshot::Graph(<SemioGraphSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        16 => SemioSubsetSnapshot::Object(<SemioObjectSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        17 => SemioSubsetSnapshot::Kit(<SemioKitSnapshot as store::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        other => return Err(format!("semio snapshot: unknown subset tag {other}")),
    };
    Ok(SemioSnapshot { schema, subset })
}

impl store::ArtifactPack for SemioSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_semio_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_semio_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::base::schema::snapshot::*;
use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::base::io::text::snapshot::*;
/// 📥️ Decodes the envelope's own committed `.pack.semio` bytes into a real [`SemioSnapshot`] — the
/// binary half of the same bridge, so a caller outside this crate can check the two codecs against
/// each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_envelope_pack(bytes: &[u8]) -> Result<SemioSnapshot, String> {
    <SemioSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `store::ArtifactPack::encode_pack` inverse of [`decode_semio_envelope_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_envelope_pack(snapshot: &SemioSnapshot) -> Vec<u8> {
    <SemioSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
