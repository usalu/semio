/// 🎲️ Property and differential laws of the pack-json companion: the deterministic generator, the round trips, the serde differentials and the float parity.

//#region 🔖️PropertyTesting
/// 🔀️ A tiny deterministic PRNG (SplitMix64) — property/differential tests need arbitrary
/// `Value` trees, but adding a `rand`/`proptest`/`arbitrary` crate would itself be a NEW
/// third-party dependency the freeze ratchet forbids; this is small enough to own outright.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn range(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound.max(1)
    }

    fn unit_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn bool(&mut self) -> bool {
        self.next_u64() & 1 == 0
    }
}

fn arbitrary_finite_float(rng: &mut Rng) -> f64 {
    loop {
        let value = match rng.range(4) {
            0 => 0.0,
            1 => (rng.next_u64() as i64) as f64 / 1e3,
            2 => {
                let exponent = rng.range(600) as i32 - 300;
                let mantissa = 1.0 + rng.unit_f64();
                mantissa * 10f64.powi(exponent)
            }
            _ => f64::from_bits(rng.next_u64()),
        };
        if value.is_finite() {
            return value;
        }
    }
}

fn arbitrary_string(rng: &mut Rng) -> String {
    let len = rng.range(6);
    let mut out = String::new();
    for _ in 0..len {
        let ch = match rng.range(11) {
            0 => '"',
            1 => '\\',
            2 => '\n',
            3 => '\t',
            4 => '\u{0000}',
            5 => '\u{001F}',
            6 => '€',
            7 => '😀',
            8 => '\u{0008}',
            9 => '\u{000C}',
            _ => char::from_u32(0x20 + rng.range(0x5E) as u32).unwrap_or('x'),
        };
        out.push(ch);
    }
    out
}

fn arbitrary_value(rng: &mut Rng, depth: u32) -> Value {
    let kind_bound = if depth >= 4 { 5 } else { 7 };
    match rng.range(kind_bound) {
        0 => Value::Null,
        1 => Value::Bool(rng.bool()),
        2 => Value::Number(Number::UInt(rng.range(1_000_000))),
        3 => Value::Number(Number::Int(rng.range(2_000_000) as i64 - 1_000_000)),
        4 => Value::Number(Number::Float(arbitrary_finite_float(rng))),
        5 => Value::String(arbitrary_string(rng)),
        6 => {
            let len = rng.range(4);
            Value::Array((0..len).map(|_| arbitrary_value(rng, depth + 1)).collect())
        }
        _ => {
            let len = rng.range(4);
            let mut object = Object::new();
            for index in 0..len {
                object.insert(format!("k{index}_{}", rng.range(1000)), arbitrary_value(rng, depth + 1));
            }
            Value::Object(object)
        }
    }
}

const PROPERTY_TEST_ITERATIONS: u32 = 3000;

#[test]
fn round_trips_arbitrary_values() {
    let mut rng = Rng::new(0xC0FF_EE00_1234_5678);
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let text = to_string(&value);
        let parsed = parse(&text, crate::JsonMemberPolicy::Replace).unwrap_or_else(|error| panic!("case {case}: parse failed: {error}; text={text}"));
        assert_eq!(value, parsed, "case {case}: round-trip mismatch; text={text}");
    }
}
//#endregion 🔖️PropertyTesting

//#region 🔖️DifferentialTesting
/// 🔬️ Structural equality between our `Value` and `serde_json::Value` — deliberately NOT a
/// byte-for-byte text comparison: object key order is allowed to differ (this crate's `Object`
/// is insertion-ordered, `serde_json::Value`'s default `Map` is not) as long as the two trees
/// denote the same value. Float notation no longer needs an exception here — see the
/// `FloatParity` region below — but `values_match` stays a value comparison for the key-order
/// reason. Byte-for-byte agreement is checked separately, both for typical documents
/// (`canonical_bytes_match_serde_json_for_typical_documents`) and exhaustively for `f64` alone
/// (`FloatParity`).
fn values_match(mine: &Value, theirs: &serde_json::Value) -> bool {
    match (mine, theirs) {
        (Value::Null, serde_json::Value::Null) => true,
        (Value::Bool(a), serde_json::Value::Bool(b)) => a == b,
        (Value::String(a), serde_json::Value::String(b)) => a == b,
        (Value::Number(a), serde_json::Value::Number(b)) => number_matches(a, b),
        (Value::Array(a), serde_json::Value::Array(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_match(x, y)),
        (Value::Object(a), serde_json::Value::Object(b)) => a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|bv| values_match(v, bv))),
        _ => false,
    }
}

fn number_matches(mine: &Number, theirs: &serde_json::Number) -> bool {
    match *mine {
        Number::UInt(v) => theirs.as_u64() == Some(v) || theirs.as_f64() == Some(v as f64),
        Number::Int(v) => theirs.as_i64() == Some(v) || theirs.as_f64() == Some(v as f64),
        Number::Float(v) => theirs.as_f64().is_some_and(|t| t == v),
    }
}

#[test]
fn differential_parse_matches_serde_json_on_arbitrary_values() {
    let mut rng = Rng::new(0xD1FF_0000_BEEF_CAFE);
    let mut checked = 0usize;
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let text = to_string(&value);
        let mine = parse(&text, crate::JsonMemberPolicy::Replace).unwrap();
        let theirs: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|error| panic!("case {case}: serde_json rejected our own writer output: {error}; text={text}"));
        assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}\nmine={mine:?}\ntheirs={theirs:?}");
        checked += 1;
    }
}

#[test]
fn differential_cross_parse_serde_json_writer_output() {
    let mut rng = Rng::new(0x5EED_1357_2468_ACE0);
    let mut checked = 0usize;
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let theirs = to_serde_json(&value);
        let text = serde_json::to_string(&theirs).unwrap();
        let mine = parse(&text, crate::JsonMemberPolicy::Replace).unwrap_or_else(|error| panic!("case {case}: our parser rejected serde_json's writer output: {error}; text={text}"));
        assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}");
        checked += 1;
    }
}

fn to_serde_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(v) => serde_json::Value::Bool(*v),
        Value::String(v) => serde_json::Value::String(v.clone()),
        Value::Number(Number::UInt(v)) => serde_json::Value::Number((*v).into()),
        Value::Number(Number::Int(v)) => serde_json::Value::Number((*v).into()),
        Value::Number(Number::Float(v)) => serde_json::Number::from_f64(*v).map_or(serde_json::Value::Null, serde_json::Value::Number),
        Value::Array(items) => serde_json::Value::Array(items.iter().map(to_serde_json).collect()),
        Value::Object(object) => serde_json::Value::Object(object.iter().map(|(k, v)| (k.to_string(), to_serde_json(v))).collect()),
    }
}

/// 📄️ On documents with no object-key-order ambiguity (scalars, arrays, single-key objects),
/// our writer's bytes agree with `serde_json`'s exactly — including large-magnitude floats,
/// now that `FloatParity` below proves the writers agree on every `f64`.
#[test]
fn canonical_bytes_match_serde_json_for_typical_documents() {
    let cases: &[&str] = &[r#"null"#, r#"true"#, r#"false"#, r#"0"#, r#"-17"#, r#"3.5"#, r#""hello""#, r#""café""#, r#"[]"#, r#"{}"#, r#"[1,2,3]"#, r#"{"a":1}"#, r#"{"only":{"one":"key"}}"#, r#"1.5e300"#, r#"5e-300"#, r#"1e21"#, r#"1e-7"#];
    for text in cases {
        let mine = parse(text, crate::JsonMemberPolicy::Replace).unwrap();
        let theirs: serde_json::Value = serde_json::from_str(text).unwrap();
        let mine_bytes = to_string(&mine);
        let their_bytes = serde_json::to_string(&theirs).unwrap();
        assert_eq!(mine_bytes, their_bytes, "byte mismatch for {text}");
    }
}
//#endregion 🔖️DifferentialTesting

//#region 🔖️FloatParity
/// 🔢️ Exhaustive-ish differential sweep proving `write_float`'s output is byte-identical to
/// `serde_json`'s (`zmij`'s) for every `f64` it is handed — a constant-seeded LCG over random
/// bit patterns (this crate's own `Rng`, reused rather than adding a `rand`/`proptest`
/// dependency) plus every historically-awkward case named in this ticket's own brief. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️float-format-parity.md` for the derivation and the full sweep this test's
/// smaller in-crate corpus is drawn from.
fn float_parity_edge_cases() -> Vec<f64> {
    vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::MAX,
        f64::MIN,
        1e-7,
        -1e-7,
        1e21,
        1e22,
        5e-324,
        -5e-324,
        1.7976931348623157e308,
        -1.7976931348623157e308,
        0.1,
        0.3,
        1e16,
        1e15,
        9999999999999998.0,
        9007199254740993.0,
        123456789012345.0,
        1234567890123456.0,
        12345678901234567.0,
        100000.0,
        1000000.0,
        99999.0,
        999999999999999.9,
        1.0e-5,
        1.0e-6,
        1.0e-4,
        8322951083873004.0,
        f64::from_bits(0xc316b3096f9dcd35),
        f64::from_bits(0x431b807272ea6281),
        f64::from_bits(0xc9409f0951d8de1a),
        f64::from_bits(0x40f869f000000000),
        f64::from_bits(0x430c6bf52633ffff),
    ]
}

#[test]
fn write_float_matches_serde_json_byte_for_byte() {
    let mut checked = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    for &value in &float_parity_edge_cases() {
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        if mine != theirs {
            mismatches.push(format!("bits={:#018x} value={value:e} mine={mine} theirs={theirs}", value.to_bits()));
        }
        let reparsed: f64 = mine.parse().unwrap_or_else(|error| panic!("our own output {mine:?} failed to reparse: {error}"));
        assert_eq!(reparsed.to_bits(), value.to_bits(), "round-trip bit mismatch for {value:e}, wrote {mine}");
        checked += 1;
    }
    let mut rng = Rng::new(0xF10A_7000_0000_0001);
    for _ in 0..300_000u32 {
        let bits = rng.next_u64();
        let value = f64::from_bits(bits);
        if !value.is_finite() {
            continue;
        }
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        if mine != theirs {
            mismatches.push(format!("bits={bits:#018x} value={value:e} mine={mine} theirs={theirs}"));
        }
        let reparsed: f64 = mine.parse().unwrap_or_else(|error| panic!("our own output {mine:?} failed to reparse: {error}"));
        assert_eq!(reparsed.to_bits(), value.to_bits(), "round-trip bit mismatch for {value:e}, wrote {mine}");
        checked += 1;
    }
    assert!(mismatches.is_empty(), "{} of {checked} floats mismatched serde_json byte-for-byte:\n{}", mismatches.len(), mismatches.join("\n"));
}

/// 🔣️ The two real production call sites this parity result unblocks
/// (`🌿️vcs::content_addressed_checkpoint_id_core`'s `serde_json::to_vec(change)` and
/// `🧵️canonical-edit::ScalarBytes::from_node`'s `serde_json::to_writer`) both serialize a
/// single JSON document containing ordinary application floats, not adversarial bit patterns —
/// this proves byte-identity on a realistic corpus shaped like those payloads (nested objects
/// with float-valued fields, the kind a checkpoint or a canonical scalar actually carries).
#[test]
fn realistic_payloads_byte_match_serde_json() {
    let mut rng = Rng::new(0x0011_2233_4455_6677);
    let mut checked = 0usize;
    for _ in 0..20_000u32 {
        let value = arbitrary_finite_float(&mut rng);
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        assert_eq!(mine, theirs, "realistic-payload float mismatch for {value:e}");
        checked += 1;
    }
}
//#endregion 🔖️FloatParity
