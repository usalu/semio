
use super::*;
use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
use crate::schema::mutations::{DeflateMutation, apply_deflate_mutation, set_compression_params, set_payload, set_preset_dictionary, set_snapshot};
use crate::standards::v_rfc1950::subsets::any::io::{decode_deflate_snapshot, encode_deflate_snapshot};
use protocol::{DiffCodec, Mutation};

//#region Fixtures
/// 🌱 A real RFC1950 zlib stream (CMF=0x78 CINFO=7/CM=8, FLG=0x9c FLEVEL=Default/FDICT=0,
/// dynamic-Huffman deflate body, real Adler-32 trailer) -- byte-identical to the artifact's
/// own `📚️examples/🎬️demo/🖼️assets/🗜️example.zz` fixture, duplicated here as a literal so
/// the test doesn't reach across an emoji-path `include_bytes!` boundary.
const REAL_FIXTURE_ZLIB: &[u8] = &[
    0x78, 0x9c, 0x2b, 0x2e, 0x49, 0xc9, 0xcc, 0xd7, 0x4b, 0x49, 0x4d, 0xcb, 0x49, 0x2c, 0x49, 0x55, 0x48, 0xce, 0xcf, 0x4b, 0xcb, 0x2f, 0xca, 0x4d, 0xcc, 0x4b, 0x4e, 0x55, 0x48, 0xcb, 0xac, 0x28, 0x29, 0x2d, 0x4a, 0x05, 0x00, 0xda, 0xb1, 0x0c, 0xf9,
];

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> DeflateSnapshot {
    DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Fastest, dict_id: None, payload: b"sweep-a-payload".to_vec() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> DeflateSnapshot {
    DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 9, window_bits: 6, compression_level_hint: DeflateLevelHint::Maximum, dict_id: Some(0xDEAD_BEEF), payload: b"sweep-b-different-longer-payload".to_vec() }
}
//#endregion Fixtures

//#region field_sweep
/// 🧪️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable field (incl. the
/// tri-state `dict_id` exercising `Some(None)` in the b→a direction).
#[semio_framework_async_macros::async_test]
async fn field_sweep_between_covers_every_field() {
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
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_every_variant() {
    let base = sweep_a();
    let variants = vec![
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Fast }),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: Some(7) }),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload: b"mutation-diff-law".to_vec() }),
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
#[semio_framework_async_macros::async_test]
async fn inverse_law_mutation_and_diff_level() {
    let base = sweep_a();
    let variants = vec![
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Fast }),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: Some(7) }),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload: b"inverse-law".to_vec() }),
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
#[semio_framework_async_macros::async_test]
async fn absorb_law_scalar_lww_and_associativity() {
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
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law_synthetic_and_real_fixture() {
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
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_self_round_trip_is_byte_exact() {
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
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_real_fixture_normal_form() {
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
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
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
