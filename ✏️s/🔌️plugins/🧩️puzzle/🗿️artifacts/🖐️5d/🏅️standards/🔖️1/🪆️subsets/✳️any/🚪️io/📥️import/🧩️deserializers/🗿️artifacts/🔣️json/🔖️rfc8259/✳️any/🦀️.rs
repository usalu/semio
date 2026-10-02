//! puzzle5d <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--puzzle-report.md`): `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's
//! own lexeme-preserving `JsonValue` (own type, `#[serde(tag = "kind")]` — an intentional boundary
//! per that schema's own doc comment, NOT structurally plain JSON) by a concurrent stdio wave,
//! breaking this pre-existing leaf's compile. Fixed as a minimal lagging-call-site update: routes
//! through `JsonSnapshot::to_serde_value` (stdio's own real `JsonValue -> serde_json::Value`
//! bridge) plus stdio's own real `parse_json_text` — no hand-rolled converter here.
//!
//! 🩹️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: no longer
//! routes through `serde_json::from_value` — `Puzzle5dSnapshot` only derives `Deserialize` under
//! `#[cfg(test)]` now. The number lexemes cross into `dsl::DslValue` through stdio's own
//! first-party `to_pack_value()` bridge, then `dsl::FromValue::from_value` hydrates the typed
//! snapshot — same shape the sibling `block5d` leaf already uses. NOT through
//! `to_serde_value()`: `serde_json`'s default (no `float_roundtrip`) number parser reconstructs an
//! f64 as `significand as f64 * 10^exponent`, which is off by one ULP for any 17-significant-digit
//! literal (`0.42839899821678995` came back as `0.4283989982167899`) and made every example whose
//! geometry needs full f64 precision fail its lossless-round-trip oracle.
use crate::Puzzle5dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Puzzle5dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw=crate::standards::v1::subsets::any::io::puzzle5d_json::convert(dsl::json::to_dsl_value(&from.to_pack_value()),true).map_err(|message|store::TextError::new(message,dsl::TextSpan::at(1,1)))?;
    let snap: Puzzle5dSnapshot = dsl::FromValue::from_value(raw).map_err(|e| store::TextError::new(format!("puzzle5d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle5dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

/// 🧪️ Exact native word identity crosses the actual declared JSON serializer and importer.
#[cfg(test)]
#[test]
fn sqlite_snapshot_puzzle5d_declared_json_exact_words() {
    for raw in [0_u64, 0x8000000000000000, 0x3ff0000000000000, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff0000000000001, 0x7ff8000000000042, 0xfff0000000000042] {
        let mut snapshot = Puzzle5dSnapshot::default();
        let mut part = crate::Puzzle5dPart::default();
        part.id = "literal".into();
        part.part_2d.x = f64::from_bits(raw);
        snapshot.parts.push(part);
        let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).expect("declared JSON export");
        let independent: serde_json::Value = serde_json::from_slice(&bytes).expect("independent JSON oracle");
        assert_eq!(independent["parts"][0]["2d"]["x"], serde_json::json!({"bits": format!("{raw:016x}")}));
        assert_eq!(deserialize_bytes(&bytes).expect("declared JSON import").parts[0].part_2d.x.to_bits(), raw);
    }
}
