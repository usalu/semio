//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `archive` module rather than by copying it.
//!
//! CMF/FLG/DICTID framing (`Header`) is RFC1950's own fixed bit arithmetic, computed independently
//! of the subject's own codec — not a competing "implementation" so much as the same deterministic
//! formula every conformant reader/writer performs identically. Neither `windowBits` nor a preset
//! dictionary id actually reconfigures the real DEFLATE window here (RFC1950 leaves them as writer
//! metadata; this subset's own codec documents the same simplification), so `flate2` is used purely
//! for the part that IS hard to get right: the DEFLATE entropy coding and the real Adler-32 trailer.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::{digest, Json};

//#region 🔖️Header
/// 🧮️ The typed RFC1950 CMF/FLG/DICTID fields, independent of the subject's own `DeflateSnapshot`.
#[cfg(feature = "oracles")]
#[derive(Clone, Copy)]
struct Header {
    method: u8,
    window_bits: u8,
    level_hint_bits: u8,
    dict_id: Option<u32>,
}

#[cfg(feature = "oracles")]
impl Header {
    /// 📖️ Parses CMF/FLG and, when FDICT is set, the four-byte dictionary id. Returns the header
    /// plus the byte offset where the DEFLATE stream begins.
    fn parse(data: &[u8]) -> Result<(Header, usize), String> {
        if data.len() < 6 {
            return Err("zlib stream too short".to_string());
        }
        let cmf = data[0];
        let flg = data[1];
        if (cmf & 0x0F) != 8 {
            return Err(format!("unsupported zlib compression method {}", cmf & 0x0F));
        }
        if ((cmf as u16) * 256 + flg as u16) % 31 != 0 {
            return Err("zlib CMF/FLG check failed".to_string());
        }
        let mut pos = 2usize;
        let dict_id = if flg & 0x20 != 0 {
            if data.len() < pos + 4 {
                return Err("truncated preset dictionary id".to_string());
            }
            let id = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            pos += 4;
            Some(id)
        } else {
            None
        };
        Ok((Header { method: cmf & 0x0F, window_bits: (cmf >> 4) & 0x0F, level_hint_bits: flg >> 6, dict_id }, pos))
    }

    /// 🖊️ Packs CMF/FLG (with a freshly computed FCHECK) and the optional DICTID.
    fn write(&self) -> Vec<u8> {
        let cmf = ((self.window_bits & 0x0F) << 4) | (self.method & 0x0F);
        let flg_hi = ((self.level_hint_bits & 0b11) << 6) | (((self.dict_id.is_some()) as u8) << 5);
        let fcheck = (31 - (((cmf as u16) * 256 + flg_hi as u16) % 31)) % 31;
        let mut out = vec![cmf, flg_hi | fcheck as u8];
        if let Some(id) = self.dict_id {
            out.extend_from_slice(&id.to_be_bytes());
        }
        out
    }
}

/// 🎚️ RFC1950 §2.2 FLEVEL bits, independent of `DeflateLevelHint::from_bits`/`to_bits`.
#[cfg(feature = "oracles")]
fn level_hint_name(bits: u8) -> &'static str {
    match bits & 0b11 {
        0 => "fastest",
        1 => "fast",
        2 => "default",
        _ => "maximum",
    }
}

#[cfg(feature = "oracles")]
fn level_hint_bits(name: &str) -> Result<u8, String> {
    match name {
        "fastest" => Ok(0),
        "fast" => Ok(1),
        "default" => Ok(2),
        "maximum" => Ok(3),
        other => Err(format!("unknown levelHint {other:?}")),
    }
}
//#endregion 🔖️Header

//#region 🔖️Codec
/// 🔮️ Wraps `payload` as a real zlib stream with `flate2`, keeping only its DEFLATE bytes and its
/// real Adler-32 trailer — the two-byte header those bytes arrive with is always discarded and
/// replaced by `header.write()`, since the typed CMF/FLG/DICTID fields are this subset's own.
#[cfg(feature = "oracles")]
fn encode(header: &Header, payload: &[u8]) -> Result<Vec<u8>, String> {
    let reference = crate::archive::oracle_zlib_compress(payload)?;
    if reference.len() < 6 {
        return Err("reference zlib wrap produced a truncated stream".to_string());
    }
    let mut out = header.write();
    out.extend_from_slice(&reference[2..]);
    Ok(out)
}

/// 🔮️ Independently inflates the DEFLATE bytes following a stream's own header/DICTID, via a
/// synthetic default zlib prefix `flate2` accepts unconditionally — this subset's codec never
/// primes a real preset dictionary (documented simplification shared with the subject), so the
/// FDICT bit and the CMF/FLG bits actually written are never load-bearing for decompression.
#[cfg(feature = "oracles")]
fn independent_inflate(input: &[u8]) -> Result<Vec<u8>, String> {
    let (_, offset) = Header::parse(input)?;
    if input.len() < offset + 4 {
        return Err("zlib stream too short".to_string());
    }
    let cmf = 0x78u8;
    let flg_hi = 2u8 << 6;
    let fcheck = (31 - (((cmf as u16) * 256 + flg_hi as u16) % 31)) % 31;
    let mut synthetic = vec![cmf, flg_hi | fcheck as u8];
    synthetic.extend_from_slice(&input[offset..]);
    let mut decoder = flate2::read::ZlibDecoder::new(&synthetic[..]);
    let mut out = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut out).map_err(|error| format!("independent reader could not inflate the stream: {error}"))?;
    Ok(out)
}
//#endregion 🔖️Codec

//#region 🔖️Json
#[cfg(feature = "oracles")]
fn json_number(value: &Json, key: &str, default: f64) -> f64 {
    match value.get(key) {
        Some(Json::Number(found)) => *found,
        _ => default,
    }
}

#[cfg(feature = "oracles")]
fn json_optional_u32(value: &Json, key: &str) -> Option<u32> {
    match value.get(key) {
        Some(Json::Number(found)) => Some(*found as u32),
        _ => None,
    }
}

#[cfg(feature = "oracles")]
fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or_else(|| Json::Object(Vec::new()))
}
//#endregion 🔖️Json

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    match kind.as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => {
            let (header, _) = Header::parse(input)?;
            let payload = independent_inflate(input)?;
            encode(&header, &payload)
        }
        "set-snapshot" => {
            let params = params_of(spec);
            let header =
                Header { method: json_number(&params, "method", 8.0) as u8, window_bits: json_number(&params, "windowBits", 7.0) as u8, level_hint_bits: level_hint_bits(&params.str("levelHint"))?, dict_id: json_optional_u32(&params, "dictId") };
            encode(&header, params.str("payload").as_bytes())
        }
        "set-compression-params" => {
            let (original, _) = Header::parse(input)?;
            let payload = independent_inflate(input)?;
            let params = params_of(spec);
            let header = Header {
                method: json_number(&params, "method", original.method as f64) as u8,
                window_bits: json_number(&params, "windowBits", original.window_bits as f64) as u8,
                level_hint_bits: level_hint_bits(&params.str("levelHint"))?,
                dict_id: original.dict_id,
            };
            encode(&header, &payload)
        }
        "set-preset-dictionary" => {
            let (original, _) = Header::parse(input)?;
            let payload = independent_inflate(input)?;
            let params = params_of(spec);
            encode(&Header { dict_id: json_optional_u32(&params, "dictId"), ..original }, &payload)
        }
        "set-payload" => {
            let (header, _) = Header::parse(input)?;
            let params = params_of(spec);
            encode(&header, params.str("payload").as_bytes())
        }
        other => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", other, input.len())),
    }
}

/// 📦️ The decoded payload alone, independent of header framing — the `flate2` half of
/// `project_deflate`, exposed separately so a caller building an inverse spec (which needs the
/// ORIGINAL payload as text, not just its digest) never has to parse RFC1950 header bytes itself.
#[cfg(feature = "oracles")]
pub fn independent_payload(input: &[u8]) -> Result<Vec<u8>, String> {
    independent_inflate(input)
}

/// 👁️ Projects zlib bytes with the INDEPENDENT `flate2` reader onto this subset's own semantic
/// shape: the typed header fields plus the recovered payload's size and digest. Never the raw
/// compressed bytes themselves — this repository's own encoder and `flate2` choose different block
/// splits and Huffman tables for the same payload, so only the DECODED content is normative.
#[cfg(feature = "oracles")]
pub fn project_deflate(input: &[u8]) -> Result<Json, String> {
    let (header, _) = Header::parse(input)?;
    let payload = independent_inflate(input)?;
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("zlib".to_string())),
        ("compressionMethod".to_string(), Json::Number(header.method as f64)),
        ("windowBits".to_string(), Json::Number(header.window_bits as f64)),
        ("compressionLevelHint".to_string(), Json::String(level_hint_name(header.level_hint_bits).to_string())),
        ("presetDictionaryId".to_string(), header.dict_id.map(|id| Json::Number(id as f64)).unwrap_or(Json::Null)),
        ("payloadSize".to_string(), Json::Number(payload.len() as f64)),
        ("payloadDigest".to_string(), Json::String(digest(&payload))),
    ]))
}

/// ↩️ The spec for the mutation that undoes `kind`, computed from the ORIGINAL typed fields the
/// same way `DeflateMutation::inverse` does (restore the prior value), but independently: this
/// oracle module has no reachable path to the subject's own `protocol::Mutation` trait impl, and
/// mirroring its algebra from data rather than calling it keeps the two implementations honestly
/// separate.
#[cfg(feature = "oracles")]
pub fn inverse_mutation_spec(kind: &str, method: u8, window_bits: u8, level_hint_bits: u8, dict_id: Option<u32>, payload: &[u8]) -> Result<Json, String> {
    let dict_id_json = dict_id.map(|id| Json::Number(id as f64)).unwrap_or(Json::Null);
    let payload_text = String::from_utf8(payload.to_vec()).map_err(|error| format!("original payload is not UTF-8 text: {error}"))?;
    let params = match kind {
        "no-mutation" => Json::Object(Vec::new()),
        "set-snapshot" => Json::Object(vec![
            ("method".to_string(), Json::Number(method as f64)),
            ("windowBits".to_string(), Json::Number(window_bits as f64)),
            ("levelHint".to_string(), Json::String(level_hint_name(level_hint_bits).to_string())),
            ("dictId".to_string(), dict_id_json),
            ("payload".to_string(), Json::String(payload_text)),
        ]),
        "set-compression-params" => {
            Json::Object(vec![("method".to_string(), Json::Number(method as f64)), ("windowBits".to_string(), Json::Number(window_bits as f64)), ("levelHint".to_string(), Json::String(level_hint_name(level_hint_bits).to_string()))])
        }
        "set-preset-dictionary" => Json::Object(vec![("dictId".to_string(), dict_id_json)]),
        "set-payload" => Json::Object(vec![("payload".to_string(), Json::String(payload_text))]),
        other => return Err(format!("mutation kind {other:?} has no inverse spec")),
    };
    Ok(Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)]))
}
//#endregion 🔖️Dispatch

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
mod unavailable {
    use super::Json;
    const MESSAGE: &str = "the `oracles` feature is disabled — this host was not built with the registered reference implementations";

    pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn independent_payload(_input: &[u8]) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_deflate(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
    pub fn inverse_mutation_spec(_kind: &str, _method: u8, _window_bits: u8, _level_hint_bits: u8, _dict_id: Option<u32>, _payload: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{independent_payload, inverse_mutation_spec, oracle_apply_mutation, project_deflate};
//#endregion 🔖️Unavailable
