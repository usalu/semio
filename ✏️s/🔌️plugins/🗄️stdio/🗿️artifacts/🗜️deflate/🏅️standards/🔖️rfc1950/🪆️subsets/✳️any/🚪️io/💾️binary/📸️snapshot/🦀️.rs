//! 📦️ deflate rep for stdio.deflate 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_rfc1950::subsets::any::schema::snapshot::*;
use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🗜️ Declares the actual cumulative bare RFC1950 producer authority for domain consumers.
use crate::standards::v_rfc1950::subsets::any::io::sqlite::snapshot::native::{compress_zlib,decompress_zlib};

impl pack::snapshot::ArtifactPack for DeflateSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;

        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        let zlib_bytes = encode_deflate_snapshot(self);
        Ok(store::semio_format::wrap_binary(&envelope, &zlib_bytes))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_deflate_snapshot(&inner).map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error)))
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for DeflateSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}

pub use crate::standards::v_rfc1950::subsets::any::io::sqlite::snapshot::native::{compress_zlib,decompress_zlib};

use crate::schema::snapshot::{DeflateLevelHint, DeflateSnapshot};
use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
use crate::standards::v_rfc1950::subsets::any::io::zlib::{adler32, deflate_raw, inflate_raw};

//#region 🔖️SnapshotCodec
/// 🧬️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the typed
/// entry points -- these are what `DeflateSnapshot`'s `ArtifactDsl`/`ArtifactPack` impls call.
/// `zlib_compress`/`zlib_decompress` above stay byte<->byte (they're load-bearing for other
/// artifacts' own internal zlib framing -- PNG IDAT, PDF stream objects -- which have never gone
/// through a `DeflateSnapshot` and must not start doing so here); these two are the RFC1950
/// container<->typed-snapshot pair that actually populates/consumes `cmf`/`flg`/`dict_id` instead
/// of a bare byte blob.
///
/// 🧮️ Encodes a `DeflateSnapshot` into a full zlib (RFC1950) byte stream: CMF/FLG rebuilt from
/// the typed fields (with a freshly computed FCHECK -- it's a pure function of the other header
/// bits, never independently stored), the preset-dictionary id when present, the raw DEFLATE
/// payload, and a freshly computed Adler-32 trailer (never a stale stored checksum).
pub fn encode_deflate_snapshot(snapshot: &DeflateSnapshot) -> Vec<u8> {
    let cmf = ((snapshot.window_bits & 0x0F) << 4) | (snapshot.compression_method & 0x0F);
    let fdict = snapshot.dict_id.is_some();
    let flg_hi = (snapshot.compression_level_hint.to_bits() << 6) | ((fdict as u8) << 5);
    let fcheck = (31 - (((cmf as u16) * 256 + flg_hi as u16) % 31)) % 31;
    let flg = flg_hi | (fcheck as u8);

    let raw = deflate_raw(&snapshot.payload);
    let mut out = Vec::with_capacity(2 + 4 + raw.len() + 4);
    out.push(cmf);
    out.push(flg);
    if let Some(dict_id) = snapshot.dict_id {
        out.extend_from_slice(&dict_id.to_be_bytes());
    }
    out.extend_from_slice(&raw);
    out.extend_from_slice(&adler32(&snapshot.payload).to_be_bytes());
    out
}

/// 🧮️ Decodes a zlib (RFC1950) byte stream into a typed `DeflateSnapshot`: CMF/FLG parsed into
/// typed fields, the preset-dictionary id extracted when FDICT is set, the payload inflated, and
/// the Adler-32 trailer verified against the freshly decompressed payload.
///
/// 📖️ A preset dictionary's ID is retained honestly as typed data, but this codec cannot prime
/// the LZ77 window with actual dictionary content -- that capability doesn't exist in
/// `inflate_raw`/`deflate_raw` (the real LZ77+Huffman engine, untouched per this wave's mandate).
/// Round trips through this codec's own `encode_deflate_snapshot` are unaffected (it never
/// primes a dictionary either); a foreign stream that truly relied on dictionary-primed
/// backreferences would surface as an "invalid backreference" error from `inflate_raw`, same as
/// any other genuinely undecodable stream.
pub fn decode_deflate_snapshot(data: &[u8]) -> Result<DeflateSnapshot, String> {
    if data.len() < 6 {
        return Err("zlib stream too short".into());
    }
    let cmf = data[0];
    let flg = data[1];
    let compression_method = cmf & 0x0F;
    let window_bits = (cmf >> 4) & 0x0F;
    if compression_method != 8 {
        return Err("unsupported zlib compression method".into());
    }
    if !((cmf as u16) * 256 + flg as u16).is_multiple_of(31) {
        return Err("zlib CMF/FLG check failed".into());
    }
    let fdict = flg & 0x20 != 0;
    let compression_level_hint = DeflateLevelHint::from_bits(flg >> 6);

    let mut pos = 2usize;
    let dict_id = if fdict {
        if data.len() < pos + 4 {
            return Err("truncated preset dictionary id".into());
        }
        let id = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;
        Some(id)
    } else {
        None
    };

    if data.len() < pos + 4 {
        return Err("zlib stream too short".into());
    }
    let adler_bytes = &data[data.len() - 4..];
    let expect = u32::from_be_bytes([adler_bytes[0], adler_bytes[1], adler_bytes[2], adler_bytes[3]]);
    let raw = &data[pos..data.len() - 4];
    let payload = inflate_raw(raw)?;
    let got = adler32(&payload);
    if got != expect {
        return Err(format!("adler32 mismatch: expected {expect:#010x}, got {got:#010x}"));
    }

    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method, window_bits, compression_level_hint, dict_id, payload })
}
//#endregion 🔖️SnapshotCodec
