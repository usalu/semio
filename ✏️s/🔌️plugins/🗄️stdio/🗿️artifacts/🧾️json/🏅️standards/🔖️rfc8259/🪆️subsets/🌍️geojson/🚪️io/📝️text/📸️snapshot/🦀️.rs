//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_rfc8259::subsets::geojson::schema::*;
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonSnapshot, JsonValue};
use crate::standards::v_rfc8259::subsets::base::io::text::snapshot::{parse_json_text};
use serde_json::{Map, Number, Value};
use derived_construction::*;
use derived_analysis::*;











/// 🌉️ A JSON value as `serde_json` holds it, numbers converted from their lexeme exactly: integers as
/// integers, everything else through the correctly rounded `f64` reading.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn exact_serde_value(value: &JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::Null,
        JsonValue::Bool { value } => Value::Bool(*value),
        JsonValue::Number { lexeme } => lexeme
            .parse::<u64>()
            .map(Number::from)
            .or_else(|_| lexeme.parse::<i64>().map(Number::from))
            .ok()
            .or_else(|| lexeme.parse::<f64>().ok().and_then(Number::from_f64))
            .map_or(Value::Null, Value::Number),
        JsonValue::String { value } => Value::String(value.clone()),
        JsonValue::Array { items } => Value::Array(items.iter().map(exact_serde_value).collect()),
        JsonValue::Object { members } => Value::Object(members.iter().map(|member| (member.key.clone(), exact_serde_value(&member.value))).collect()),
    }
}







/// 📐️ Twice the signed planar area of a closed ring (shoelace) — positive for counter-clockwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ring_signed_area2(ring: &[GeoJsonPosition]) -> f64 {
    ring.windows(2).map(|pair| pair[0][0] * pair[1][1] - pair[1][0] * pair[0][1]).sum()
}











/// 🔁️ A ring as RFC 7946 writes it: closed (the first position repeated when it is not already) and
/// wound by the right-hand rule — counter-clockwise for the exterior, clockwise for holes (§3.1.6).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn right_handed_ring(ring: &[GeoJsonPosition], exterior: bool) -> Vec<GeoJsonPosition> {
    let mut closed = ring.to_vec();
    if let (Some(first), Some(last)) = (ring.first(), ring.last()) {
        if first != last {
            closed.push(first.clone());
        }
    }
    if (ring_signed_area2(&closed) > 0.0) != exterior {
        closed.reverse();
    }
    closed
}





/// 📤️ Writes `features` as one RFC 7946 FeatureCollection: WGS 84 only (no `crs`), rings closed and
/// right-handed, `properties` always present (`null` when absent), `id` only when the feature has one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_geojson(features: &[GeoJsonFeature]) -> Result<Value, GeoJsonError> {
    let mut written = Vec::with_capacity(features.len());
    for (index, feature) in features.iter().enumerate() {
        let path = format!("/features/{index}");
        let mut object = Map::new();
        object.insert("type".into(), Value::String("Feature".into()));
        match &feature.id {
            Some(GeoJsonId::Text(text)) => {
                object.insert("id".into(), Value::String(text.clone()));
            }
            Some(GeoJsonId::Number(number)) => {
                object.insert("id".into(), Value::Number(number.clone()));
            }
            None => {}
        }
        object.insert("geometry".into(), feature.geometry.as_ref().map(|geometry| geometry_value(geometry, &format!("{path}/geometry"))).transpose()?.unwrap_or(Value::Null));
        object.insert("properties".into(), feature.properties.clone().map(Value::Object).unwrap_or(Value::Null));
        written.push(Value::Object(object));
    }
    Ok(serde_json::json!({ "type": "FeatureCollection", "features": written }))
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod reader {
use crate::standards::v_rfc8259::subsets::geojson::schema::*;
use crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
/// 📖️ [`read_geojson`] over GeoJSON text, parsed by the base subset's own lexeme-preserving parser.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_geojson_text(text: &str) -> Result<GeoJsonRead, GeoJsonError> {
    let root = parse_json_text(text).map_err(|error| GeoJsonError { path: String::new(), message: format!("not a JSON text: {error}") })?;
    read_geojson(&root)
}
}
pub use reader::*;
