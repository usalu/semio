//! 🔺️ DeflateDiff — sparse per-field RFC1950 container diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: `DeflateSnapshot` has
//! no keyed/indexed collections (it is five scalar/weak fields), so there is no `XsDiff` triple
//! here -- every field is `Option<T>` (nullable `dict_id` is the tri-state `Option<Option<u32>>`)
//! and absorb is plain last-write-wins per field, exactly as the recipe's "Scalars: LWW" rule
//! prescribes for artifacts with no strong entities.

use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use protocol::MutationDiff;
// 🧭️ `DiffAlgebra` lives at `command::DiffAlgebra` (not re-exported bare at the `protocol` crate
// root the way `MutationDiff` is) -- see `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`.
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.deflate`. No `snapshot: Option<DeflateSnapshot>` full-replace slot --
/// even `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate.diff")]
pub struct DeflateDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub compression_method: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window_bits: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub compression_level_hint: Option<DeflateLevelHint>,
    /// 🪆️ Tri-state: `None` = unchanged, `Some(None)` = dictionary cleared, `Some(Some(id))` =
    /// dictionary set/changed to `id`.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<Option<u32>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<u8>>,
}

impl MutationDiff<DeflateSnapshot> for DeflateDiff {
    fn apply(&self, base: &DeflateSnapshot) -> protocol::MutationApplyResult<DeflateSnapshot> {
        let mut next = base.clone();
        if let Some(v) = self.compression_method {
            next.compression_method = v;
        }
        if let Some(v) = self.window_bits {
            next.window_bits = v;
        }
        if let Some(v) = self.compression_level_hint {
            next.compression_level_hint = v;
        }
        if let Some(v) = self.dict_id {
            next.dict_id = v;
        }
        if let Some(v) = &self.payload {
            next.payload = v.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.compression_method.is_some() {
            self.compression_method = other.compression_method;
        }
        if other.window_bits.is_some() {
            self.window_bits = other.window_bits;
        }
        if other.compression_level_hint.is_some() {
            self.compression_level_hint = other.compression_level_hint;
        }
        if other.dict_id.is_some() {
            self.dict_id = other.dict_id;
        }
        if other.payload.is_some() {
            self.payload = other.payload;
        }
    }
}

impl DiffAlgebra<DeflateSnapshot> for DeflateDiff {
    fn inverse(&self, base: &DeflateSnapshot) -> Self {
        DeflateDiff {
            compression_method: self.compression_method.map(|_| base.compression_method),
            window_bits: self.window_bits.map(|_| base.window_bits),
            compression_level_hint: self.compression_level_hint.map(|_| base.compression_level_hint),
            dict_id: self.dict_id.map(|_| base.dict_id),
            payload: self.payload.as_ref().map(|_| base.payload.clone()),
        }
    }

    fn between(base: &DeflateSnapshot, other: &DeflateSnapshot) -> Self {
        DeflateDiff {
            compression_method: (base.compression_method != other.compression_method).then_some(other.compression_method),
            window_bits: (base.window_bits != other.window_bits).then_some(other.window_bits),
            compression_level_hint: (base.compression_level_hint != other.compression_level_hint).then_some(other.compression_level_hint),
            dict_id: (base.dict_id != other.dict_id).then_some(other.dict_id),
            payload: (base.payload != other.payload).then_some(other.payload.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.compression_method.is_none() && self.window_bits.is_none() && self.compression_level_hint.is_none() && self.dict_id.is_none() && self.payload.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff: the sparse field-by-field delta, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &DeflateSnapshot, snapshot: &DeflateSnapshot) -> DeflateDiff {
    DeflateDiff::between(base, snapshot)
}
/// 🧩 Builds a set-compression-params diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_compression_params(method: u8, window_bits: u8, level_hint: DeflateLevelHint) -> DeflateDiff {
    DeflateDiff { compression_method: Some(method), window_bits: Some(window_bits), compression_level_hint: Some(level_hint), ..Default::default() }
}
/// 🧩 Builds a set-preset-dictionary diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_preset_dictionary(dict_id: Option<u32>) -> DeflateDiff {
    DeflateDiff { dict_id: Some(dict_id), ..Default::default() }
}
/// 🧩 Builds a set-payload diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_payload(payload: Vec<u8>) -> DeflateDiff {
    DeflateDiff { payload: Some(payload), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `DeflateDiff` values -- covers every field, incl. `dict_id`'s
/// tri-state (`Some(None)` = cleared, `Some(Some(_))` = set/changed) and the empty diff. Single
/// source of truth reused by `diff_codec_text_binary_roundtrip_law` (below) AND by
/// `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<DeflateDiff> {
    use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

    let a = DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fastest, dict_id: None, payload: b"demo-cases-a-payload".to_vec() };
    let b = DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        compression_method: 9,
        window_bits: 6,
        compression_level_hint: DeflateLevelHint::Maximum,
        dict_id: Some(0xDEAD_BEEF),
        payload: b"demo-cases-b-different-longer-payload".to_vec(),
    };
    vec![DeflateDiff::default(), DeflateDiff::between(&a, &b), DeflateDiff::between(&b, &a), diff_set_preset_dictionary(None), diff_set_payload(Vec::new())]
}
//#endregion 🔖️DemoCases

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `DeflateDiff` — the derive path
/// (`#[derive(dsl::DslDiff)]`) is NOT usable here: `dict_id: Option<Option<u32>>` is a tri-state
/// field (`f6-recon-report.md` §3b — `dsl_derive::classify_field` peels exactly one `Option<..>`
/// layer before binding, and there is no `impl<T: DslField> DslField for Option<T>` anywhere in
/// the `dsl` crate, so the REMAINING `Option<u32>` after that one peel is structurally
/// unbindable). Confirmed via real `cargo check`:
/// ```text
/// error[E0277]: the trait bound `std::option::Option<u32>: DslField` is not satisfied
///    --> …/🔺️diff/🦀️.rs:37:17   (pub dict_id: Option<Option<u32>>)
/// ```
/// This is the SAME hand-rolled path `GifDiff` uses for its own tri-state fields (`gct`,
/// `loop_count`, `GifFrameDiff`'s `lct`/`transparent_index`/`plain_text`) — the primitive set
/// below (`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`/`encode_option`/
/// `decode_option`) is copied verbatim from that pilot's grammar template
/// (`f6-recon-report.md` §5), since `DeflateDiff` needs no enum-tag or collection-triple
/// machinery (no data-carrying enum, no keyed collection anywhere in this artifact).
///
/// **Grammar**: one space-separated `name=value` token per changed top-level field (a field
/// absent from the line = unchanged). Bytes (`payload`) are lowercase hex — same local idiom
/// `DeflateSnapshot`'s own `ArtifactDsl` impl above already uses, and the same reason `GifDiff`
/// gives (no external base64 dep, no escaping needed at this grammar layer). `compression_level_hint`
/// uses a single-letter tag (`f`/`a`/`d`/`m`, mirroring `GifDisposal`'s `enc_disposal` pattern).
/// The tri-state `dict_id` and the plain-optional `payload` both use the uniform
/// `[0]`=unchanged-inner-None / `[1,<T>]`=inner-Some(T) tag via `encode_option`/`decode_option`
/// (note: `payload`'s own `Option<Vec<u8>>` is the DIFF's "field changed at all" wrapper, not a
/// second tri-state — `DeflateSnapshot::payload` itself is a bare, never-nullable `Vec<u8>`, so
/// `payload`'s token is only present when the field changed, and its value is always hex, never
/// itself optional).
///
/// Worked example: `compression-method=9 window-bits=6 level=m dict-id=[1,3735928559] payload=` (empty
/// payload prints as a zero-length hex string after `=`).
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only) — needed even for this small a grammar
/// because `decode_option`'s own `[0]`/`[1,<v>]` payload can itself contain a `,` (none here
/// today, but the primitive is the shared grammar contract every hand-rolled codec in this repo
/// uses, per `f6-recon-report.md` §5 -- kept verbatim rather than hand-simplified).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_level_hint(h: DeflateLevelHint) -> char {
    match h {
        DeflateLevelHint::Fastest => 'f',
        DeflateLevelHint::Fast => 'a',
        DeflateLevelHint::Default => 'd',
        DeflateLevelHint::Maximum => 'm',
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_level_hint(s: &str) -> Result<DeflateLevelHint, String> {
    match s {
        "f" => Ok(DeflateLevelHint::Fastest),
        "a" => Ok(DeflateLevelHint::Fast),
        "d" => Ok(DeflateLevelHint::Default),
        "m" => Ok(DeflateLevelHint::Maximum),
        other => Err(format!("bad level hint {other:?}")),
    }
}
//#endregion 🔖️ValueCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_deflate_diff(d: &DeflateDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.compression_method {
        tokens.push(format!("compression-method={v}"));
    }
    if let Some(v) = d.window_bits {
        tokens.push(format!("window-bits={v}"));
    }
    if let Some(v) = d.compression_level_hint {
        tokens.push(format!("level={}", enc_level_hint(v)));
    }
    if let Some(v) = &d.dict_id {
        tokens.push(format!("dict-id={}", encode_option(v, |x| x.to_string())));
    }
    if let Some(v) = &d.payload {
        tokens.push(format!("payload={}", hex_encode(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_deflate_diff(line: &str) -> Result<DeflateDiff, String> {
    let mut d = DeflateDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("compression-method=") {
            d.compression_method = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("window-bits=") {
            d.window_bits = Some(parse_u8(rest)?);
        } else if let Some(rest) = token.strip_prefix("level=") {
            d.compression_level_hint = Some(dec_level_hint(rest)?);
        } else if let Some(rest) = token.strip_prefix("dict-id=") {
            d.dict_id = Some(decode_option(rest, parse_u32)?);
        } else if let Some(rest) = token.strip_prefix("payload=") {
            d.payload = Some(hex_decode(rest)?);
        } else {
            return Err(format!("deflate diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for DeflateDiff {
    fn print_diff(&self) -> String {
        print_deflate_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_deflate_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG2: REAL binary frame (`format u8 | flags u8 | [compression_method][window_bits]
    /// [compression_level_hint][dict_id][payload]`), matching
    /// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes`
    /// shape -- upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of
    /// stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
    /// 0-4 mark `compression_method`/`window_bits`/`compression_level_hint`/`dict_id`/`payload`
    /// presence in that fixed order; each present field's own (possibly tri-state) payload
    /// follows in the same order, `payload` last so it can be bare "rest of buffer" bytes with
    /// no length prefix (it is the only opaque, unbounded field in the frame).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.compression_method.is_some() {
            flags |= 0b0_0001;
        }
        if self.window_bits.is_some() {
            flags |= 0b0_0010;
        }
        if self.compression_level_hint.is_some() {
            flags |= 0b0_0100;
        }
        if self.dict_id.is_some() {
            flags |= 0b0_1000;
        }
        if self.payload.is_some() {
            flags |= 0b1_0000;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(v) = self.compression_method {
            out.push(v);
        }
        if let Some(v) = self.window_bits {
            out.push(v);
        }
        if let Some(v) = self.compression_level_hint {
            out.push(v.to_bits());
        }
        if let Some(dict_id) = &self.dict_id {
            out.push(if dict_id.is_some() { 1 } else { 0 });
            if let Some(id) = dict_id {
                out.extend_from_slice(&id.to_le_bytes());
            }
        }
        if let Some(payload) = &self.payload {
            out.extend_from_slice(payload);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let compression_method = if flags & 0b0_0001 != 0 { Some(reader.read_u8().map_err(|e| malformed("diff compression_method", reader.position(), e.to_string()))?) } else { None };
        let window_bits = if flags & 0b0_0010 != 0 { Some(reader.read_u8().map_err(|e| malformed("diff window_bits", reader.position(), e.to_string()))?) } else { None };
        let compression_level_hint = if flags & 0b0_0100 != 0 {
            let bits = reader.read_u8().map_err(|e| malformed("diff compression_level_hint", reader.position(), e.to_string()))?;
            Some(DeflateLevelHint::from_bits(bits))
        } else {
            None
        };
        let dict_id = if flags & 0b0_1000 != 0 {
            let has = reader.read_u8().map_err(|e| malformed("diff dict_id presence", reader.position(), e.to_string()))?;
            Some(if has != 0 {
                let bytes4 = reader.read_bytes(4).map_err(|e| malformed("diff dict_id", reader.position(), e.to_string()))?;
                Some(u32::from_le_bytes([bytes4[0], bytes4[1], bytes4[2], bytes4[3]]))
            } else {
                None
            })
        } else {
            None
        };
        let payload = if flags & 0b1_0000 != 0 {
            let rest = reader.read_bytes(reader.remaining()).map_err(|e| malformed("diff payload", reader.position(), e.to_string()))?;
            Some(rest.to_vec())
        } else {
            None
        };
        Ok(DeflateDiff { compression_method, window_bits, compression_level_hint, dict_id, payload })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
