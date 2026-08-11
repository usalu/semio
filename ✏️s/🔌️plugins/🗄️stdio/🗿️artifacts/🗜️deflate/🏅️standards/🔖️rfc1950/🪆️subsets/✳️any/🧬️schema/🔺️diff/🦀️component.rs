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
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.deflate`. No `snapshot: Option<DeflateSnapshot>` full-replace slot --
/// even `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate.diff")]
pub struct DeflateDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_method: Option<u8>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_bits: Option<u8>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_level_hint: Option<DeflateLevelHint>,
    /// 🪆️ Tri-state: `None` = unchanged, `Some(None)` = dictionary cleared, `Some(Some(id))` =
    /// dictionary set/changed to `id`.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<Option<u32>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<u8>>,
}

impl MutationDiff<DeflateSnapshot> for DeflateDiff {
    fn apply(&self, base: &DeflateSnapshot) -> DeflateSnapshot {
        let mut next = base.clone();
        if let Some(v) = self.compression_method { next.compression_method = v; }
        if let Some(v) = self.window_bits { next.window_bits = v; }
        if let Some(v) = self.compression_level_hint { next.compression_level_hint = v; }
        if let Some(v) = self.dict_id { next.dict_id = v; }
        if let Some(v) = &self.payload { next.payload = v.clone(); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.compression_method.is_some() { self.compression_method = other.compression_method; }
        if other.window_bits.is_some() { self.window_bits = other.window_bits; }
        if other.compression_level_hint.is_some() { self.compression_level_hint = other.compression_level_hint; }
        if other.dict_id.is_some() { self.dict_id = other.dict_id; }
        if other.payload.is_some() { self.payload = other.payload; }
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
            compression_method: (base.compression_method != other.compression_method)
                .then_some(other.compression_method),
            window_bits: (base.window_bits != other.window_bits).then_some(other.window_bits),
            compression_level_hint: (base.compression_level_hint != other.compression_level_hint)
                .then_some(other.compression_level_hint),
            dict_id: (base.dict_id != other.dict_id).then_some(other.dict_id),
            payload: (base.payload != other.payload).then_some(other.payload.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.compression_method.is_none()
            && self.window_bits.is_none()
            && self.compression_level_hint.is_none()
            && self.dict_id.is_none()
            && self.payload.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff: the sparse field-by-field delta, never a full-replace slot.
pub fn diff_set_snapshot(base: &DeflateSnapshot, snapshot: &DeflateSnapshot) -> DeflateDiff {
    DeflateDiff::between(base, snapshot)
}
/// 🧩 Builds a set-compression-params diff.
pub fn diff_set_compression_params(method: u8, window_bits: u8, level_hint: DeflateLevelHint) -> DeflateDiff {
    DeflateDiff {
        compression_method: Some(method),
        window_bits: Some(window_bits),
        compression_level_hint: Some(level_hint),
        ..Default::default()
    }
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

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::deflate::schema::mutations::{apply_deflate_mutation, DeflateMutation};
    use crate::artifacts::deflate::engine::{decode_deflate_snapshot, encode_deflate_snapshot};
    use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
    use protocol::Mutation;

    //#region Fixtures
    /// 🌱 A real RFC1950 zlib stream (CMF=0x78 CINFO=7/CM=8, FLG=0x9c FLEVEL=Default/FDICT=0,
    /// dynamic-Huffman deflate body, real Adler-32 trailer) -- byte-identical to the artifact's
    /// own `📚️examples/🎬️demo/🖼️assets/🗜️example.zz` fixture, duplicated here as a literal so
    /// the test doesn't reach across an emoji-path `include_bytes!` boundary.
    const REAL_FIXTURE_ZLIB: &[u8] = &[
        0x78, 0x9c, 0x2b, 0x2e, 0x49, 0xc9, 0xcc, 0xd7, 0x4b, 0x49, 0x4d, 0xcb, 0x49, 0x2c, 0x49,
        0x55, 0x48, 0xce, 0xcf, 0x4b, 0xcb, 0x2f, 0xca, 0x4d, 0xcc, 0x4b, 0x4e, 0x55, 0x48, 0xcb,
        0xac, 0x28, 0x29, 0x2d, 0x4a, 0x05, 0x00, 0xda, 0xb1, 0x0c, 0xf9,
    ];

    fn sweep_a() -> DeflateSnapshot {
        DeflateSnapshot {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            compression_method: 8,
            window_bits: 7,
            compression_level_hint: DeflateLevelHint::Fastest,
            dict_id: None,
            payload: b"sweep-a-payload".to_vec(),
        }
    }
    fn sweep_b() -> DeflateSnapshot {
        DeflateSnapshot {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            compression_method: 9,
            window_bits: 6,
            compression_level_hint: DeflateLevelHint::Maximum,
            dict_id: Some(0xDEAD_BEEF),
            payload: b"sweep-b-different-longer-payload".to_vec(),
        }
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
        assert_eq!(ab.apply(&a), b);

        let ba = DeflateDiff::between(&b, &a);
        assert!(ba.compression_method.is_some());
        assert!(ba.window_bits.is_some());
        assert!(ba.compression_level_hint.is_some());
        assert!(ba.dict_id.is_some());
        assert_eq!(ba.dict_id, Some(None)); // 🪆️ tri-state Some(None): dictionary cleared
        assert!(ba.payload.is_some());
        assert_eq!(ba.apply(&b), a);

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
            assert_eq!(direct.apply(&base), via_apply, "apply mismatch for {m:?}");
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

            // 🔁️ diff-level: d.inverse(base).apply(&d.apply(base)) == base.
            let d = m.diff(&base);
            let applied = d.apply(&base);
            let undone = d.inverse(&base).apply(&applied);
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
        let sequential = d2.apply(&d1.apply(&base));
        assert_eq!(absorbed.apply(&base), sequential);
        assert_eq!(absorbed.compression_method, Some(8));
        assert_eq!(absorbed.payload, Some(b"absorbed-payload".to_vec()));

        // Same field twice: last write wins.
        let d3 = diff_set_payload(b"first".to_vec());
        let d4 = diff_set_payload(b"second".to_vec());
        let mut lww = d3.clone();
        lww.absorb(d4.clone());
        assert_eq!(lww.payload, Some(b"second".to_vec()));
        assert_eq!(lww.apply(&base), d4.apply(&d3.apply(&base)));

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
        assert_eq!(left.apply(&base), dc.apply(&db.apply(&da.apply(&base))));
    }
    //#endregion absorb_law

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law_synthetic_and_real_fixture() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(DeflateDiff::between(&a, &b).apply(&a), b);
        assert_eq!(DeflateDiff::between(&b, &a).apply(&b), a);

        // 🌱 Real fixture: decode a genuine zlib stream, then round-trip against a variant that
        // changes every field from it.
        let fixture = decode_deflate_snapshot(REAL_FIXTURE_ZLIB).expect("decode real fixture");
        let mut other = fixture.clone();
        other.compression_level_hint = DeflateLevelHint::Maximum;
        other.dict_id = Some(99);
        other.payload = b"real-fixture-variant-payload".to_vec();
        assert_eq!(DeflateDiff::between(&fixture, &other).apply(&fixture), other);
        assert_eq!(DeflateDiff::between(&other, &fixture).apply(&other), fixture);
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
}
//#endregion 🔖️Tests
