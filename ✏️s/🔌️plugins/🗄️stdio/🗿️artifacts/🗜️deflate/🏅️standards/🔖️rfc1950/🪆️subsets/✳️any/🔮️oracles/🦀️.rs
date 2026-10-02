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
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

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
    let reference = semio_s_plugin_stdio_archive_test_oracle::oracle_zlib_compress(payload)?;
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
fn json_number(value: &Json, key: &str) -> Result<f64, String> {
    match value.get(key) {
        Some(Json::Number(found)) => Ok(*found),
        _ => Err(format!("expected a numeric field {key:?}")),
    }
}

#[cfg(feature = "oracles")]
fn json_optional_u32(value: &Json, key: &str) -> Option<u32> {
    match value.get(key) {
        Some(Json::Number(found)) => Some(*found as u32),
        _ => None,
    }
}

/// 🔎️ A byte payload as the wire carries it: a plain JSON array of 0-255 numbers.
#[cfg(feature = "oracles")]
fn json_bytes(value: &Json, key: &str) -> Vec<u8> {
    value.array(key).iter().filter_map(|item| if let Json::Number(number) = item { Some(*number as u8) } else { None }).collect()
}

#[cfg(feature = "oracles")]
fn bytes_json(bytes: &[u8]) -> Json {
    Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
}

#[cfg(feature = "oracles")]
fn params_of(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or_else(|| Json::Object(Vec::new()))
}

#[cfg(feature = "oracles")]
fn object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
//#endregion 🔖️Json

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// Every spec's `params` is the leaf's own wire payload (`DeflateMutation`'s `payload_value()`):
/// `set-snapshot` reads `{snapshot: {compressionMethod, windowBits, compressionLevelHint, dictId?,
/// payload}}`, `set-compression-params` reads `{method, window_bits, level_hint}`,
/// `set-preset-dictionary` reads `{dict_id}` and `set-payload` reads `{payload}`. An unrecognised kind
/// is an error, never a silent no-op: a mutation that is quietly skipped reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = params_of(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "set-snapshot" => {
            let snapshot = params.get("snapshot").ok_or("set-snapshot requires a `snapshot` field")?;
            let header = Header {
                method: json_number(snapshot, "compressionMethod")? as u8,
                window_bits: json_number(snapshot, "windowBits")? as u8,
                level_hint_bits: level_hint_bits(&snapshot.str("compressionLevelHint"))?,
                dict_id: json_optional_u32(snapshot, "dictId"),
            };
            encode(&header, &json_bytes(snapshot, "payload"))
        }
        "set-compression-params" => {
            let (original, _) = Header::parse(input)?;
            let payload = independent_inflate(input)?;
            let header = Header { method: json_number(&params, "method")? as u8, window_bits: json_number(&params, "window_bits")? as u8, level_hint_bits: level_hint_bits(&params.str("level_hint"))?, dict_id: original.dict_id };
            encode(&header, &payload)
        }
        "set-preset-dictionary" => {
            let (original, _) = Header::parse(input)?;
            let payload = independent_inflate(input)?;
            encode(&Header { dict_id: json_optional_u32(&params, "dict_id"), ..original }, &payload)
        }
        "set-payload" => {
            let (header, _) = Header::parse(input)?;
            encode(&header, &json_bytes(&params, "payload"))
        }
        other => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", other, input.len())),
    }
}

/// 🔁️ The reference's own decode/re-encode: the header is parsed, the DEFLATE stream is genuinely
/// inflated by `flate2` and re-deflated from the recovered payload alone.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    let (header, _) = Header::parse(input)?;
    encode(&header, &independent_inflate(input)?)
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

/// ↩️ The wire spec of the mutation that undoes `forward`, read out of `base` by the independent
/// reader alone — the same restore-the-prior-value algebra `DeflateMutation::inverse` implements, but
/// never reached through it: this oracle module has no path to the subject's `protocol::Mutation`, and
/// mirroring the algebra from data keeps the two implementations honestly separate.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    let (header, _) = Header::parse(base)?;
    let payload = independent_inflate(base)?;
    let dict_id = header.dict_id.map(|id| Json::Number(id as f64)).unwrap_or(Json::Null);
    let kind = forward.str("kind");
    let params = match kind.as_str() {
        "set-snapshot" => {
            let mut snapshot = vec![
                ("schema", Json::String("stdio.deflate".to_string())),
                ("compressionMethod", Json::Number(header.method as f64)),
                ("windowBits", Json::Number(header.window_bits as f64)),
                ("compressionLevelHint", Json::String(level_hint_name(header.level_hint_bits).to_string())),
            ];
            if let Some(id) = header.dict_id {
                snapshot.push(("dictId", Json::Number(id as f64)));
            }
            snapshot.push(("payload", bytes_json(&payload)));
            object(vec![("snapshot", object(snapshot))])
        }
        "set-compression-params" => object(vec![("method", Json::Number(header.method as f64)), ("window_bits", Json::Number(header.window_bits as f64)), ("level_hint", Json::String(level_hint_name(header.level_hint_bits).to_string()))]),
        "set-preset-dictionary" => object(vec![("dict_id", dict_id)]),
        "set-payload" => object(vec![("payload", bytes_json(&payload))]),
        other => return Err(format!("mutation kind {other:?} has no inverse spec")),
    };
    Ok(object(vec![("kind", Json::String(kind)), ("params", params)]))
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
    pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_deflate(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
    pub fn oracle_inverse_spec(_base: &[u8], _forward: &Json) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_deflate};
//#endregion 🔖️Unavailable
