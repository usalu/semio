//! 📖️ Decoder of the AVI codec: the RIFF walk that builds the snapshot of the main header, the streams and the chunks. See [`super`].

use super::*;

//#region 🔖️Decode
/// 📥️ Real RIFF/AVI decode: `hdrl` (`avih` + every `strl`'s `strh`/`strf`, plus any nested
/// `hdrl`/`strl` auxiliary children such as `vprp`/`JUNK` typed-raw retained in `hdrl_extra`/
/// `strl_extra`), `movi` (every chunk, assigned to its owning stream by the leading 2-digit stream
/// number in its fourcc), `idx1` (positionally matched to `movi` chunks for the keyframe flag —
/// see module doc comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_avi(bytes: &[u8]) -> Result<AviSnapshot, String> {
    if !sniff_real_bytes(bytes) {
        return Err("avi: missing RIFF/AVI magic".into());
    }
    let body = &bytes[12..bytes.len().min(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize + 8)];

    let mut main_header = None;
    let mut hdrl_extra: Vec<RiffChunk> = Vec::new();
    let mut stream_headers: Vec<(AviStreamHeader, AviStreamFormat, Vec<RiffChunk>)> = Vec::new();
    let mut movi_chunks: Vec<(String, Vec<u8>)> = Vec::new();
    let mut idx1_entries: Vec<u32> = Vec::new();
    let mut idx1_present = false;
    let mut unknown_chunks = Vec::new();

    for item in iter_riff(body) {
        let entry = item?;
        if &entry.fourcc == b"LIST" {
            let list_type = &entry.payload[0..4];
            let list_body = &entry.payload[4..];
            match list_type {
                b"hdrl" => {
                    for hitem in iter_riff(list_body) {
                        let h = hitem?;
                        if &h.fourcc == b"avih" {
                            main_header = Some(parse_avih(h.payload)?);
                        } else if &h.fourcc == b"LIST" && &h.payload[0..4] == b"strl" {
                            let strl_body = &h.payload[4..];
                            let mut strh = None;
                            let mut strf_bytes: Option<&[u8]> = None;
                            let mut strl_extra: Vec<RiffChunk> = Vec::new();
                            for sitem in iter_riff(strl_body) {
                                let s = sitem?;
                                if &s.fourcc == b"strh" {
                                    strh = Some(parse_strh(s.payload)?);
                                } else if &s.fourcc == b"strf" {
                                    strf_bytes = Some(s.payload);
                                } else if &s.fourcc == b"LIST" {
                                    let sub_type = fourcc_str(&s.payload[0..4].try_into().unwrap());
                                    strl_extra.push(RiffChunk { fourcc: format!("LIST:{sub_type}"), data: s.payload[4..].to_vec() });
                                } else {
                                    // 📦 e.g. a real `vprp` (video properties) or `JUNK` padding chunk — no
                                    // typed slot of its own, retained verbatim (see snapshot's module doc).
                                    strl_extra.push(RiffChunk { fourcc: fourcc_str(&s.fourcc), data: s.payload.to_vec() });
                                }
                            }
                            let strh = strh.ok_or("avi: strl missing strh")?;
                            let strf = parse_strf(&strh.fcc_type, strf_bytes.ok_or("avi: strl missing strf")?);
                            stream_headers.push((strh, strf, strl_extra));
                        } else if &h.fourcc == b"LIST" {
                            let sub_type = fourcc_str(&h.payload[0..4].try_into().unwrap());
                            hdrl_extra.push(RiffChunk { fourcc: format!("LIST:{sub_type}"), data: h.payload[4..].to_vec() });
                        } else {
                            // 📦 e.g. a real `JUNK` padding chunk directly inside `hdrl` — no typed slot
                            // of its own, retained verbatim (see snapshot's module doc).
                            hdrl_extra.push(RiffChunk { fourcc: fourcc_str(&h.fourcc), data: h.payload.to_vec() });
                        }
                    }
                }
                b"movi" => {
                    for mitem in iter_riff(list_body) {
                        let m = mitem?;
                        movi_chunks.push((fourcc_str(&m.fourcc), m.payload.to_vec()));
                    }
                }
                other => unknown_chunks.push(RiffChunk { fourcc: format!("LIST:{}", String::from_utf8_lossy(other)), data: list_body.to_vec() }),
            }
        } else if &entry.fourcc == b"idx1" {
            idx1_present = true;
            let mut r = entry.payload;
            while r.len() >= 16 {
                idx1_entries.push(u32::from_le_bytes(r[4..8].try_into().unwrap()));
                r = &r[16..];
            }
        } else {
            unknown_chunks.push(RiffChunk { fourcc: fourcc_str(&entry.fourcc), data: entry.payload.to_vec() });
        }
    }

    let mut streams: Vec<AviStream> = stream_headers.into_iter().map(|(strh, strf, strl_extra)| AviStream { strh, strf, chunks: Vec::new(), strl_extra }).collect();
    let idx1_matches_by_position = idx1_present && idx1_entries.len() == movi_chunks.len();
    for (i, (fourcc, data)) in movi_chunks.into_iter().enumerate() {
        let stream_index: usize = fourcc.get(0..2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let keyframe = if idx1_matches_by_position { idx1_entries[i] & 0x10 != 0 } else { true };
        if let Some(stream) = streams.get_mut(stream_index) {
            stream.chunks.push(AviChunk { fourcc, data, keyframe });
        } else {
            unknown_chunks.push(RiffChunk { fourcc: format!("movi:{fourcc}"), data });
        }
    }

    Ok(AviSnapshot { schema: STDIO_AVI_DOCUMENT_SCHEMA.into(), main_header: main_header.ok_or("avi: hdrl missing avih")?, streams, idx1_present, unknown_chunks, hdrl_extra })
}
//#endregion 🔖️Decode
