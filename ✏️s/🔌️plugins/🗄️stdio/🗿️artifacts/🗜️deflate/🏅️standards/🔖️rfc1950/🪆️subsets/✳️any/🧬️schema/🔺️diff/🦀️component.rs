//! 🔺️ DeflateDiff — sparse per-field RFC1950 container diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: `DeflateSnapshot` has
//! no keyed/indexed collections (it is five scalar/weak fields), so there is no `XsDiff` triple
//! here -- every field is `Option<T>` (nullable `dict_id` is the tri-state `Option<Option<u32>>`)
//! and absorb is plain last-write-wins per field, exactly as the recipe's "Scalars: LWW" rule
//! prescribes for artifacts with no strong entities.

use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::DeflateSnapshot;
use protocol::MutationDiff;
// 🧭️ `DiffAlgebra` lives at `command::DiffAlgebra` (not re-exported bare at the `protocol` crate
// root the way `MutationDiff` is) -- see `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`.
use protocol::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.deflate`. No `snapshot: Option<DeflateSnapshot>` full-replace slot --
/// even `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate.diff")]
pub struct DeflateDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_method: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_bits: Option<u8>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_level_hint: Option<DeflateLevelHint>,
    /// 🪆️ Tri-state: `None` = unchanged, `Some(None)` = dictionary cleared, `Some(Some(id))` =
    /// dictionary set/changed to `id`.
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<Option<u32>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
pub fn diff_set_snapshot(base: &DeflateSnapshot, snapshot: &DeflateSnapshot) -> DeflateDiff {
    DeflateDiff::between(base, snapshot)
}
/// 🧩 Builds a set-compression-params diff.
pub fn diff_set_compression_params(method: u8, window_bits: u8, level_hint: DeflateLevelHint) -> DeflateDiff {
    DeflateDiff { compression_method: Some(method), window_bits: Some(window_bits), compression_level_hint: Some(level_hint), ..Default::default() }
}
/// 🧩 Builds a set-preset-dictionary diff.
pub fn diff_set_preset_dictionary(dict_id: Option<u32>) -> DeflateDiff {
    DeflateDiff { dict_id: Some(dict_id), ..Default::default() }
}
/// 🧩 Builds a set-payload diff.
pub fn diff_set_payload(payload: Vec<u8>) -> DeflateDiff {
    DeflateDiff { payload: Some(payload), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `DeflateDiff` values -- covers every field, incl. `dict_id`'s
/// tri-state (`Some(None)` = cleared, `Some(Some(_))` = set/changed) and the empty diff. Single
/// source of truth reused by `diff_codec_text_binary_roundtrip_law` (below) AND by
/// `⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests.
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<DeflateDiff> {
    use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

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
///    --> …/🔺️diff/🦀️component.rs:37:17   (pub dict_id: Option<Option<u32>>)
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
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn parse_u8(s: &str) -> Result<u8, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🧭️ Bracket-depth-aware split (tracks `[`/`]` only) — needed even for this small a grammar
/// because `decode_option`'s own `[0]`/`[1,<v>]` payload can itself contain a `,` (none here
/// today, but the primitive is the shared grammar contract every hand-rolled codec in this repo
/// uses, per `f6-recon-report.md` §5 -- kept verbatim rather than hand-simplified).
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
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
fn enc_level_hint(h: DeflateLevelHint) -> char {
    match h {
        DeflateLevelHint::Fastest => 'f',
        DeflateLevelHint::Fast => 'a',
        DeflateLevelHint::Default => 'd',
        DeflateLevelHint::Maximum => 'm',
    }
}
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
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes`
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
mod tests {
    use super::*;
    use crate::artifacts::deflate::schema::mutations::{apply_deflate_mutation, DeflateMutation};
    use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::{decode_deflate_snapshot, encode_deflate_snapshot};
    use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
    use protocol::{DiffCodec, Mutation};

    //#region Fixtures
    /// 🌱 A real RFC1950 zlib stream (CMF=0x78 CINFO=7/CM=8, FLG=0x9c FLEVEL=Default/FDICT=0,
    /// dynamic-Huffman deflate body, real Adler-32 trailer) -- byte-identical to the artifact's
    /// own `📚️examples/🎬️demo/🖼️assets/🗜️example.zz` fixture, duplicated here as a literal so
    /// the test doesn't reach across an emoji-path `include_bytes!` boundary.
    const REAL_FIXTURE_ZLIB: &[u8] = &[
        0x78, 0x9c, 0x2b, 0x2e, 0x49, 0xc9, 0xcc, 0xd7, 0x4b, 0x49, 0x4d, 0xcb, 0x49, 0x2c, 0x49, 0x55, 0x48, 0xce, 0xcf, 0x4b, 0xcb, 0x2f, 0xca, 0x4d, 0xcc, 0x4b, 0x4e, 0x55, 0x48, 0xcb, 0xac, 0x28, 0x29, 0x2d, 0x4a, 0x05, 0x00, 0xda, 0xb1, 0x0c,
        0xf9,
    ];

    fn sweep_a() -> DeflateSnapshot {
        DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fastest, dict_id: None, payload: b"sweep-a-payload".to_vec() }
    }
    fn sweep_b() -> DeflateSnapshot {
        DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 9, window_bits: 6, compression_level_hint: DeflateLevelHint::Maximum, dict_id: Some(0xDEAD_BEEF), payload: b"sweep-b-different-longer-payload".to_vec() }
    }
    //#endregion Fixtures

    //#region field_sweep
    /// 🧪️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable field (incl. the
    /// tri-state `dict_id` exercising `Some(None)` in the b→a direction).
    #[test]
    fn field_sweep_between_covers_every_field() {
        let a = sweep_a();
        let b = sweep_b();

        let ab = DeflateDiff::between(&a, &b);
        assert!(ab.compression_method.is_some());
        assert!(ab.window_bits.is_some());
        assert!(ab.compression_level_hint.is_some());
        assert!(ab.dict_id.is_some());
        assert_eq!(ab.dict_id, Some(Some(0xDEAD_BEEF)));
        assert!(ab.payload.is_some());
        assert_eq!(ab.apply(&a).unwrap(), b);

        let ba = DeflateDiff::between(&b, &a);
        assert!(ba.compression_method.is_some());
        assert!(ba.window_bits.is_some());
        assert!(ba.compression_level_hint.is_some());
        assert!(ba.dict_id.is_some());
        assert_eq!(ba.dict_id, Some(None)); // 🪆️ tri-state Some(None): dictionary cleared
        assert!(ba.payload.is_some());
        assert_eq!(ba.apply(&b).unwrap(), a);

        assert!(DeflateDiff::between(&a, &a).is_empty());
        assert!(DeflateDiff::between(&b, &b).is_empty());
    }
    //#endregion field_sweep

    //#region mutation_diff_law
    #[test]
    fn mutation_diff_law_every_variant() {
        let base = sweep_a();
        let variants = vec![
            DeflateMutation::NoMutation,
            DeflateMutation::SetSnapshot { snapshot: sweep_b() },
            DeflateMutation::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Fast },
            DeflateMutation::SetPresetDictionary { dict_id: Some(7) },
            DeflateMutation::SetPayload { payload: b"mutation-diff-law".to_vec() },
        ];
        for m in variants {
            let mut via_apply = base.clone();
            let returned = apply_deflate_mutation(&mut via_apply, &m);
            let direct = m.diff(&base);
            assert_eq!(direct, returned, "diff mismatch for {m:?}");
            assert_eq!(direct.diff().apply(&base).unwrap(), via_apply, "apply mismatch for {m:?}");
        }
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[test]
    fn inverse_law_mutation_and_diff_level() {
        let base = sweep_a();
        let variants = vec![
            DeflateMutation::NoMutation,
            DeflateMutation::SetSnapshot { snapshot: sweep_b() },
            DeflateMutation::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Fast },
            DeflateMutation::SetPresetDictionary { dict_id: Some(7) },
            DeflateMutation::SetPayload { payload: b"inverse-law".to_vec() },
        ];
        for m in variants {
            // 🔁️ mutation-level: apply then apply every inverse mutation restores base.
            let mut round = base.clone();
            apply_deflate_mutation(&mut round, &m);
            for inv in m.inverse(&base) {
                apply_deflate_mutation(&mut round, &inv);
            }
            assert_eq!(round, base, "mutation-level inverse failed for {m:?}");

            // 🔁️ diff-level: d.diff().inverse(base).apply(&d.diff().apply(base)) == base.
            let d = m.diff(&base);
            let applied = d.diff().apply(&base).unwrap();
            let undone = d.diff().inverse(&base).apply(&applied).unwrap();
            assert_eq!(undone, base, "diff-level inverse failed for {m:?}");
        }
    }
    //#endregion inverse_law

    //#region absorb_law
    /// 🧪️ `DeflateSnapshot` has no keyed collections, so absorb reduces to the recipe's plain
    /// "Scalars: LWW" rule -- these cases cover disjoint-field composition, same-field LWW
    /// override, and associativity over a triple.
    #[test]
    fn absorb_law_scalar_lww_and_associativity() {
        let base = sweep_a();

        // Disjoint fields: both survive.
        let d1 = diff_set_compression_params(8, 5, DeflateLevelHint::Fast);
        let d2 = diff_set_payload(b"absorbed-payload".to_vec());
        let mut absorbed = d1.clone();
        absorbed.absorb(d2.clone());
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        assert_eq!(absorbed.apply(&base).unwrap(), sequential);
        assert_eq!(absorbed.compression_method, Some(8));
        assert_eq!(absorbed.payload, Some(b"absorbed-payload".to_vec()));

        // Same field twice: last write wins.
        let d3 = diff_set_payload(b"first".to_vec());
        let d4 = diff_set_payload(b"second".to_vec());
        let mut lww = d3.clone();
        lww.absorb(d4.clone());
        assert_eq!(lww.payload, Some(b"second".to_vec()));
        assert_eq!(lww.apply(&base).unwrap(), d4.apply(&d3.apply(&base).unwrap()).unwrap());

        // Associativity over a triple: absorb(absorb(d1,d2),d3) == absorb(d1,absorb(d2,d3)).
        let da = diff_set_compression_params(9, 6, DeflateLevelHint::Maximum);
        let db = diff_set_preset_dictionary(Some(11));
        let dc = diff_set_payload(b"triple".to_vec());

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

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law_synthetic_and_real_fixture() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(DeflateDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(DeflateDiff::between(&b, &a).apply(&b).unwrap(), a);

        // 🌱 Real fixture: decode a genuine zlib stream, then round-trip against a variant that
        // changes every field from it.
        let fixture = decode_deflate_snapshot(REAL_FIXTURE_ZLIB).expect("decode real fixture");
        let mut other = fixture.clone();
        other.compression_level_hint = DeflateLevelHint::Maximum;
        other.dict_id = Some(99);
        other.payload = b"real-fixture-variant-payload".to_vec();
        assert_eq!(DeflateDiff::between(&fixture, &other).apply(&fixture).unwrap(), other);
        assert_eq!(DeflateDiff::between(&other, &fixture).apply(&other).unwrap(), fixture);
    }
    //#endregion between_roundtrip_law

    //#region codec_retention_law
    #[test]
    fn codec_retention_law_self_round_trip_is_byte_exact() {
        // 🔁️ Encoding with OUR OWN encoder and decoding back must be exactly byte- and
        // field-preserving (both directions use the same codec, so there is no cross-encoder
        // Huffman-strategy mismatch to normalize away).
        let snap = sweep_a();
        let bytes = encode_deflate_snapshot(&snap);
        let decoded = decode_deflate_snapshot(&bytes).expect("decode self-encoded stream");
        assert_eq!(decoded, snap);
        let bytes2 = encode_deflate_snapshot(&decoded);
        assert_eq!(bytes, bytes2);
    }

    /// 🧪️ Documented normal form for a THIRD-PARTY-encoded fixture: this codec's `deflate_raw`
    /// always emits a single canonical fixed-Huffman block (pre-existing engine behavior this
    /// wave must not touch), so re-encoding a dynamic-Huffman original will not reproduce the
    /// same raw DEFLATE bytes. What must be preserved exactly is the typed header fields and the
    /// decompressed PAYLOAD across a decode -> re-encode -> re-decode cycle.
    #[test]
    fn codec_retention_law_real_fixture_normal_form() {
        let decoded = decode_deflate_snapshot(REAL_FIXTURE_ZLIB).expect("decode real fixture");
        let re_encoded = encode_deflate_snapshot(&decoded);
        let re_decoded = decode_deflate_snapshot(&re_encoded).expect("decode re-encoded stream");
        assert_eq!(re_decoded.compression_method, decoded.compression_method);
        assert_eq!(re_decoded.window_bits, decoded.window_bits);
        assert_eq!(re_decoded.compression_level_hint, decoded.compression_level_hint);
        assert_eq!(re_decoded.dict_id, decoded.dict_id);
        assert_eq!(re_decoded.payload, decoded.payload);
    }
    //#endregion codec_retention_law

    //#region diff_codec_text_binary_roundtrip_law
    /// 🧪️ F6: `DiffCodec::print_diff`/`parse_diff`/`encode_diff`/`decode_diff` round-trip law —
    /// exercises real `between()` results covering every field AND both `dict_id` tri-state
    /// transitions (`Some(None)` = cleared, `Some(Some(_))` = set/changed), plus the empty diff.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        // 🪆️ `a.dict_id` is `None`, `b.dict_id` is `Some(_)` -- `between(a,b)` exercises the
        // Some(Some(_)) arm, `between(b,a)` exercises the Some(None) arm.
        let cases = vec![DeflateDiff::default(), DeflateDiff::between(&a, &b), DeflateDiff::between(&b, &a), diff_set_preset_dictionary(None), diff_set_payload(Vec::new())];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = DeflateDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = DeflateDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion diff_codec_text_binary_roundtrip_law
}
//#endregion 🔖️Tests
