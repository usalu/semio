//! 💾️ Binary representation codec surface for `stdio.semio` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

const PACK_BINARY_FORMAT: u8 = 1;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot;
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;



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
        SemioSubsetSnapshot::Brep(s) => <SemioBrepSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Mesh(s) => <SemioMeshSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Model(s) => <SemioModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Value(s) => <SemioValueSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Document(s) => <SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Cad(s) => <SemioCadSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Drawing(s) => <SemioDrawingSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Image(s) => <SemioImageSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Video(s) => <SemioVideoSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Audio(s) => <SemioAudioSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Animation(s) => <SemioAnimationSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Presentation(s) => <SemioPresentationSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Flow(s) => <SemioFlowSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Text(s) => <SemioTextSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Table(s) => <SemioTableSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Graph(s) => <SemioGraphSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Object(s) => <SemioObjectSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
        SemioSubsetSnapshot::Kit(s) => <SemioKitSnapshot as pack::snapshot::ArtifactPack>::encode_pack(s),
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
        0 => SemioSubsetSnapshot::Brep(<SemioBrepSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        1 => SemioSubsetSnapshot::Mesh(<SemioMeshSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        2 => SemioSubsetSnapshot::Model(<SemioModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        3 => SemioSubsetSnapshot::Value(<SemioValueSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        4 => SemioSubsetSnapshot::Document(<SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        5 => SemioSubsetSnapshot::Cad(<SemioCadSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        6 => SemioSubsetSnapshot::Drawing(<SemioDrawingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        7 => SemioSubsetSnapshot::Image(<SemioImageSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        8 => SemioSubsetSnapshot::Video(<SemioVideoSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        9 => SemioSubsetSnapshot::Audio(<SemioAudioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        10 => SemioSubsetSnapshot::Animation(<SemioAnimationSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        11 => SemioSubsetSnapshot::Presentation(<SemioPresentationSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        12 => SemioSubsetSnapshot::Flow(<SemioFlowSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        13 => SemioSubsetSnapshot::Text(<SemioTextSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        14 => SemioSubsetSnapshot::Table(<SemioTableSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        15 => SemioSubsetSnapshot::Graph(<SemioGraphSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        16 => SemioSubsetSnapshot::Object(<SemioObjectSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        17 => SemioSubsetSnapshot::Kit(<SemioKitSnapshot as pack::snapshot::ArtifactPack>::decode_pack(payload).map_err(|e| e.to_string())?),
        other => return Err(format!("semio snapshot: unknown subset tag {other}")),
    };
    Ok(SemioSnapshot { schema, subset })
}

impl pack::snapshot::ArtifactPack for SemioSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_semio_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_semio_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot;
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
/// 📥️ Decodes the envelope's own committed `.pack.semio` bytes into a real [`SemioSnapshot`] — the
/// binary half of the same bridge, so a caller outside this crate can check the two codecs against
/// each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_envelope_pack(bytes: &[u8]) -> Result<SemioSnapshot, String> {
    <SemioSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `pack::snapshot::ArtifactPack::encode_pack` inverse of [`decode_semio_envelope_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_envelope_pack(snapshot: &SemioSnapshot) -> Vec<u8> {
    <SemioSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
