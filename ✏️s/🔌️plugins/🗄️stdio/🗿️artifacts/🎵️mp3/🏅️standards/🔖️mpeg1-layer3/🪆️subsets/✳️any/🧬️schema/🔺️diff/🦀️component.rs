//! 🔺️ Mp3Diff — sparse per-field MPEG1/ID3 container diff. `id3v2`/`id3v1` are independently
//! nullable in `Mp3Snapshot` (a tag may be added or removed entirely), so both are the
//! `DeflateDiff::dict_id`-style tri-state `Option<Option<T>>` (`None` = unchanged, `Some(None)` =
//! tag cleared, `Some(Some(tag))` = tag set/changed); `frames` is a plain `Option<Vec<_>>`
//! "changed or not" slot, same shape as `DeflateDiff::payload`.

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader, Mp3Snapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp3Diff {
    /// 🪆️ Tri-state: `None` = unchanged, `Some(None)` = id3v2 tag cleared, `Some(Some(tag))` =
    /// tag set/changed to `tag`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id3v2: Option<Option<Id3v2Tag>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<Mp3Frame>>,
    /// 🪆️ Tri-state, same shape as `id3v2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id3v1: Option<Option<Id3v1Tag>>,
}

impl MutationDiff<Mp3Snapshot> for Mp3Diff {
    fn apply(&self, base: &Mp3Snapshot) -> protocol::MutationApplyResult<Mp3Snapshot> {
        let mut next = base.clone();
        if let Some(v) = &self.id3v2 {
            next.id3v2 = v.clone();
        }
        if let Some(v) = &self.frames {
            next.frames = v.clone();
        }
        if let Some(v) = &self.id3v1 {
            next.id3v1 = v.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.id3v2.is_some() {
            self.id3v2 = other.id3v2;
        }
        if other.frames.is_some() {
            self.frames = other.frames;
        }
        if other.id3v1.is_some() {
            self.id3v1 = other.id3v1;
        }
    }
}

impl DiffAlgebra<Mp3Snapshot> for Mp3Diff {
    fn between(base: &Mp3Snapshot, other: &Mp3Snapshot) -> Self {
        Mp3Diff { id3v2: (base.id3v2 != other.id3v2).then(|| other.id3v2.clone()), frames: (base.frames != other.frames).then(|| other.frames.clone()), id3v1: (base.id3v1 != other.id3v1).then(|| other.id3v1.clone()) }
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Self {
        Mp3Diff { id3v2: self.id3v2.as_ref().map(|_| base.id3v2.clone()), frames: self.frames.as_ref().map(|_| base.frames.clone()), id3v1: self.id3v1.as_ref().map(|_| base.id3v1.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.id3v2.is_none() && self.frames.is_none() && self.id3v1.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff: the sparse field-by-field delta, never a full-replace slot.
pub fn diff_set_snapshot(base: &Mp3Snapshot, snapshot: &Mp3Snapshot) -> Mp3Diff {
    Mp3Diff::between(base, snapshot)
}
/// 🧩 Builds a set-id3v2 diff (`None` clears the tag).
pub fn diff_set_id3v2(id3v2: Option<Id3v2Tag>) -> Mp3Diff {
    Mp3Diff { id3v2: Some(id3v2), ..Default::default() }
}
/// 🧩 Builds a set-frames diff.
pub fn diff_set_frames(frames: Vec<Mp3Frame>) -> Mp3Diff {
    Mp3Diff { frames: Some(frames), ..Default::default() }
}
/// 🧩 Builds a set-id3v1 diff (`None` clears the tag).
pub fn diff_set_id3v1(id3v1: Option<Id3v1Tag>) -> Mp3Diff {
    Mp3Diff { id3v1: Some(id3v1), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` (per ticket `26/08/11/…-RETIREMENT`'s mandate: no
/// `#[derive(dsl::DslDiff)]` — `Mp3Frame`/`Id3v2Tag` embed nested collections of named structs,
/// the same generic-collection-diff shape `f6-final-summary.md` §4.4 documents as needing a
/// hand-rolled bridge; hand-rolled following `DeflateDiff`'s own tri-state grammar template,
/// `f6-recon-report.md` §5's primitive set copied verbatim). Grammar: one space-separated
/// `name=value` token per changed top-level field; `id3v2`/`id3v1` use the uniform
/// `[0]`=unchanged-inner-None / `[1,<T>]`=inner-Some(T) tri-state tag via `encode_option`/
/// `decode_option`; `frames` is a `[frame1;frame2;…]` list.
//#region 🔖️Primitives
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only) — the shared grammar contract every
/// hand-rolled codec in this repo uses (`f6-recon-report.md` §5), kept verbatim.
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
fn parse_u16(s: &str) -> Result<u16, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
/// 🧭️ `Id3Frame.id` is a 4-char printable ID3 frame id (`TIT2`/`TPE1`/…) — never contains
/// `,`/`[`/`]`/`;` in practice, so it's safe as a bare top-level token.
fn enc_id3_frame(f: &Id3Frame) -> String {
    format!("[{},{},{}]", f.id, f.flags, hex_encode(&f.data))
}
fn dec_id3_frame(s: &str) -> Result<Id3Frame, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 3 {
        return Err(format!("id3 frame: expected 3 fields, got {}", parts.len()));
    }
    Ok(Id3Frame { id: parts[0].to_string(), flags: parse_u16(parts[1])?, data: hex_decode(parts[2])? })
}
fn enc_id3_frames(frames: &[Id3Frame]) -> String {
    format!("[{}]", frames.iter().map(enc_id3_frame).collect::<Vec<_>>().join(";"))
}
fn dec_id3_frames(s: &str) -> Result<Vec<Id3Frame>, String> {
    let inner = strip_brackets(s)?;
    split_top_level(inner, ';').into_iter().filter(|p| !p.is_empty()).map(dec_id3_frame).collect()
}

fn enc_id3v2(tag: &Id3v2Tag) -> String {
    format!("[{},{},{},{}]", tag.major_version, tag.minor_version, tag.flags, enc_id3_frames(&tag.frames))
}
fn dec_id3v2(s: &str) -> Result<Id3v2Tag, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 4 {
        return Err(format!("id3v2: expected 4 fields, got {}", parts.len()));
    }
    Ok(Id3v2Tag { major_version: parse_u8(parts[0])?, minor_version: parse_u8(parts[1])?, flags: parse_u8(parts[2])?, frames: dec_id3_frames(parts[3])? })
}

fn enc_id3v1(tag: &Id3v1Tag) -> String {
    format!("[{}]", hex_encode(&tag.raw))
}
fn dec_id3v1(s: &str) -> Result<Id3v1Tag, String> {
    Ok(Id3v1Tag { raw: hex_decode(strip_brackets(s)?)? })
}

fn enc_mp3_header(h: &Mp3FrameHeader) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{}]",
        h.mpeg_version_id,
        h.layer,
        enc_bool(h.protection_bit),
        h.bitrate_index,
        h.sample_rate_index,
        enc_bool(h.padding),
        enc_bool(h.private_bit),
        h.channel_mode,
        h.mode_extension,
        enc_bool(h.copyright),
        enc_bool(h.original),
        h.emphasis
    )
}
/// 🧭️ `s` is the header's OWN bracketed group (e.g. `[3,1,1,9,0,0,0,3,0,0,1,0]`) — callers strip
/// it from its enclosing token first (see `dec_mp3_frame`).
fn dec_mp3_header(s: &str) -> Result<Mp3FrameHeader, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 12 {
        return Err(format!("mp3 frame header: expected 12 fields, got {}", parts.len()));
    }
    Ok(Mp3FrameHeader {
        mpeg_version_id: parse_u8(parts[0])?,
        layer: parse_u8(parts[1])?,
        protection_bit: parse_bool(parts[2])?,
        bitrate_index: parse_u8(parts[3])?,
        sample_rate_index: parse_u8(parts[4])?,
        padding: parse_bool(parts[5])?,
        private_bit: parse_bool(parts[6])?,
        channel_mode: parse_u8(parts[7])?,
        mode_extension: parse_u8(parts[8])?,
        copyright: parse_bool(parts[9])?,
        original: parse_bool(parts[10])?,
        emphasis: parse_u8(parts[11])?,
    })
}
fn enc_mp3_frame(f: &Mp3Frame) -> String {
    format!("[{},{}]", enc_mp3_header(&f.header), hex_encode(&f.payload))
}
fn dec_mp3_frame(s: &str) -> Result<Mp3Frame, String> {
    let inner = strip_brackets(s)?;
    // 🧭️ The header is itself a bracketed 12-field group, so at depth-0 it is ONE token (same
    // nesting trick `decode_option` relies on) — split at depth 0 gives exactly
    // `["[12 header fields]", "<payload-hex>"]`, 2 top-level tokens, never 13.
    let parts = split_top_level(inner, ',');
    if parts.len() != 2 {
        return Err(format!("mp3 frame: expected header+payload=2 top-level fields, got {}", parts.len()));
    }
    let header = dec_mp3_header(parts[0])?;
    Ok(Mp3Frame { header, payload: hex_decode(parts[1])? })
}
fn enc_mp3_frames(frames: &[Mp3Frame]) -> String {
    format!("[{}]", frames.iter().map(enc_mp3_frame).collect::<Vec<_>>().join(";"))
}
fn dec_mp3_frames(s: &str) -> Result<Vec<Mp3Frame>, String> {
    let inner = strip_brackets(s)?;
    split_top_level(inner, ';').into_iter().filter(|p| !p.is_empty()).map(dec_mp3_frame).collect()
}
//#endregion 🔖️ValueCodecs

//#region 🔖️TopLevel
fn print_mp3_diff(d: &Mp3Diff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.id3v2 {
        tokens.push(format!("id3v2={}", encode_option(v, |t| enc_id3v2(t))));
    }
    if let Some(v) = &d.frames {
        tokens.push(format!("frames={}", enc_mp3_frames(v)));
    }
    if let Some(v) = &d.id3v1 {
        tokens.push(format!("id3v1={}", encode_option(v, |t| enc_id3v1(t))));
    }
    tokens.join(" ")
}
fn parse_mp3_diff(line: &str) -> Result<Mp3Diff, String> {
    let mut d = Mp3Diff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("id3v2=") {
            d.id3v2 = Some(decode_option(rest, dec_id3v2)?);
        } else if let Some(rest) = token.strip_prefix("frames=") {
            d.frames = Some(dec_mp3_frames(rest)?);
        } else if let Some(rest) = token.strip_prefix("id3v1=") {
            d.id3v1 = Some(decode_option(rest, dec_id3v1)?);
        } else {
            return Err(format!("mp3 diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for Mp3Diff {
    fn print_diff(&self) -> String {
        print_mp3_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_mp3_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim (same simplification `DeflateDiff`/`GifDiff`'s
    /// hand-rolled `DiffCodec` impls use).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::DiffCodec;

    fn frame() -> Mp3Frame {
        Mp3Frame {
            header: Mp3FrameHeader { mpeg_version_id: 3, layer: 1, protection_bit: true, bitrate_index: 9, sample_rate_index: 0, padding: false, private_bit: false, channel_mode: 3, mode_extension: 0, copyright: false, original: true, emphasis: 0 },
            payload: vec![0u8; 4],
        }
    }
    fn sweep_a() -> Mp3Snapshot {
        Mp3Snapshot { id3v2: None, frames: vec![frame()], id3v1: None, ..Mp3Snapshot::default() }
    }
    fn sweep_b() -> Mp3Snapshot {
        Mp3Snapshot {
            id3v2: Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![Id3Frame { id: "TIT2".into(), flags: 0, data: vec![0, b'x'] }] }),
            frames: vec![frame(), frame()],
            id3v1: Some(Id3v1Tag { raw: vec![b'T', b'A', b'G'] }),
            ..Mp3Snapshot::default()
        }
    }

    //#region field_sweep
    /// 🧪️ `field_sweep`: `sweep_a`/`sweep_b` differ in EVERY mutable field, exercising both
    /// tri-state directions (`Some(Some(_))` a→b, `Some(None)` b→a).
    #[test]
    fn field_sweep_between_covers_every_field() {
        let a = sweep_a();
        let b = sweep_b();
        let ab = Mp3Diff::between(&a, &b);
        assert!(matches!(ab.id3v2, Some(Some(_))));
        assert!(ab.frames.is_some());
        assert!(matches!(ab.id3v1, Some(Some(_))));
        assert_eq!(ab.apply(&a).unwrap(), b);

        let ba = Mp3Diff::between(&b, &a);
        assert_eq!(ba.id3v2, Some(None));
        assert!(ba.frames.is_some());
        assert_eq!(ba.id3v1, Some(None));
        assert_eq!(ba.apply(&b).unwrap(), a);

        assert!(Mp3Diff::between(&a, &a).is_empty());
    }
    //#endregion field_sweep

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(Mp3Diff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(Mp3Diff::between(&b, &a).apply(&b).unwrap(), a);
    }
    //#endregion between_roundtrip_law

    //#region absorb_law
    #[test]
    fn absorb_law_disjoint_and_lww_and_associativity() {
        let base = sweep_a();
        let d1 = diff_set_frames(vec![frame(), frame(), frame()]);
        let d2 = diff_set_id3v1(Some(Id3v1Tag { raw: vec![1, 2, 3] }));
        let mut absorbed = d1.clone();
        absorbed.absorb(d2.clone());
        assert_eq!(absorbed.apply(&base).unwrap(), d2.apply(&d1.apply(&base).unwrap()).unwrap());

        let d3 = diff_set_id3v2(Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![] }));
        let d4 = diff_set_id3v2(None);
        let mut lww = d3.clone();
        lww.absorb(d4.clone());
        assert_eq!(lww.id3v2, Some(None));

        let da = diff_set_frames(vec![frame()]);
        let db = diff_set_id3v2(None);
        let dc = diff_set_id3v1(None);
        let mut left = da.clone();
        left.absorb(db.clone());
        left.absorb(dc.clone());
        let mut right_tail = db.clone();
        right_tail.absorb(dc.clone());
        let mut right = da.clone();
        right.absorb(right_tail);
        assert_eq!(left, right);
        assert_eq!(left.apply(&base).unwrap(), dc.apply(&db.apply(&da.apply(&base).unwrap()).unwrap()).unwrap());
    }
    //#endregion absorb_law

    //#region inverse_law
    #[test]
    fn inverse_law_diff_level() {
        let base = sweep_a();
        let d = Mp3Diff::between(&base, &sweep_b());
        let applied = d.apply(&base).unwrap();
        let undone = d.inverse(&base).apply(&applied).unwrap();
        assert_eq!(undone, base);
    }
    //#endregion inverse_law

    //#region diff_codec_text_binary_roundtrip_law
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        let cases = vec![Mp3Diff::default(), Mp3Diff::between(&a, &b), Mp3Diff::between(&b, &a), diff_set_id3v2(None), diff_set_id3v1(None), diff_set_frames(vec![])];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = Mp3Diff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = Mp3Diff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion diff_codec_text_binary_roundtrip_law
}
//#endregion 🔖️Tests
