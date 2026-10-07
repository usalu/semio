//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_rfc8259::subsets::geojson::schema::*;
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonSnapshot, JsonValue};
use crate::standards::v_rfc8259::subsets::base::io::text::snapshot::{parse_json_text};
use serde_json::{Map, Number, Value};











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












/// 🔁️ A ring as RFC 7946 writes it: closed (the first position repeated when it is not already) and
/// wound by the right-hand rule — counter-clockwise for the exterior, clockwise for holes (§3.1.6).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9






/// 📤️ Writes `features` as one RFC 7946 FeatureCollection: WGS 84 only (no `crs`), rings closed and
/// right-handed, `properties` always present (`null` when absent), `id` only when the feature has one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn geometry_value(geometry: &GeoJsonGeometry, path: &str) -> Result<Value, GeoJsonError> {
    let coordinates_path = format!("{path}/coordinates");
    let (kind, coordinates) = match geometry {
        GeoJsonGeometry::Point(position) => ("Point", position_value(position, &coordinates_path)?),
        GeoJsonGeometry::MultiPoint(positions) => ("MultiPoint", positions_value(positions, &coordinates_path, 0, "a MultiPoint")?),
        GeoJsonGeometry::LineString(positions) => ("LineString", positions_value(positions, &coordinates_path, 2, "a LineString")?),
        GeoJsonGeometry::MultiLineString(lines) => ("MultiLineString", lines.iter().enumerate().map(|(index, line)| positions_value(line, &format!("{coordinates_path}/{index}"), 2, "a LineString")).collect::<Result<Vec<_>, _>>().map(Value::Array)?),
        GeoJsonGeometry::Polygon(rings) => ("Polygon", polygon_value(rings, &coordinates_path)?),
        GeoJsonGeometry::MultiPolygon(polygons) => ("MultiPolygon", polygons.iter().enumerate().map(|(index, rings)| polygon_value(rings, &format!("{coordinates_path}/{index}"))).collect::<Result<Vec<_>, _>>().map(Value::Array)?),
        GeoJsonGeometry::GeometryCollection(members) => {
            let geometries = members.iter().enumerate().map(|(index, member)| geometry_value(member, &format!("{path}/geometries/{index}"))).collect::<Result<Vec<_>, _>>()?;
            return Ok(serde_json::json!({ "type": "GeometryCollection", "geometries": geometries }));
        }
    };
    Ok(serde_json::json!({ "type": kind, "coordinates": coordinates }))
}
pub(crate) fn position_value(position: &GeoJsonPosition, path: &str) -> Result<Value, GeoJsonError> {
    if position.len() < 2 {
        return fail(path, "a position has at least longitude and latitude");
    }
    let numbers = position.iter().map(|number| Number::from_f64(*number).map(Value::Number)).collect::<Option<Vec<_>>>();
    let Some(numbers) = numbers else { return fail(path, "a coordinate is a finite number") };
    wgs84_range(position, path)?;
    Ok(Value::Array(numbers))
}
pub(crate) fn positions_value(positions: &[GeoJsonPosition], path: &str, minimum: usize, what: &str) -> Result<Value, GeoJsonError> {
    if positions.len() < minimum {
        return fail(path, format!("{what} needs at least {minimum} positions"));
    }
    positions.iter().enumerate().map(|(index, position)| position_value(position, &format!("{path}/{index}"))).collect::<Result<Vec<_>, _>>().map(Value::Array)
}
pub(crate) fn polygon_value(rings: &[Vec<GeoJsonPosition>], path: &str) -> Result<Value, GeoJsonError> {
    if rings.is_empty() {
        return fail(path, "a Polygon has an exterior ring");
    }
    rings.iter().enumerate().map(|(index, ring)| positions_value(&right_handed_ring(ring, index == 0), &format!("{path}/{index}"), 4, "a linear ring")).collect::<Result<Vec<_>, _>>().map(Value::Array)
}

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
                object.insert("id".into(), exact_serde_value(number));
            }
            None => {}
        }
        object.insert("geometry".into(), feature.geometry.as_ref().map(|geometry| geometry_value(geometry, &format!("{path}/geometry"))).transpose()?.unwrap_or(Value::Null));
        object.insert("properties".into(), feature.properties.as_ref().map(|members|Value::Object(members.iter().map(|(name,value)|(name.clone(),exact_serde_value(value))).collect())).unwrap_or(Value::Null));
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
