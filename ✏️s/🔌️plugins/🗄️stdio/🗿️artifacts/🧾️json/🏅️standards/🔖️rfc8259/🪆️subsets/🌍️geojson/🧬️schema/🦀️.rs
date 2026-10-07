//! 🌍️ GeoJSON (RFC 7946) — the typed feature model of a JSON text that conforms to RFC 7946, its
//! reader, its writer, and the conformance gate the `rfc8259/geojson` dialect is stamped with. The
//! snapshot is the base subset's `JsonSnapshot` verbatim: GeoJSON is a constraint profile of JSON,
//! exactly as 🛜️i-json is. The model is declared schema-first in `🔣️.json` beside this file.
//!
//! 🧭️ Coordinate reference policy. RFC 7946 §4 fixes WGS 84 longitude/latitude in decimal degrees
//! (an optional third value is the height above the ellipsoid) and removed GJ2008's `crs` member. The
//! reader therefore takes a document without `crs` as WGS 84, and accepts a GJ2008 named `crs` only when
//! it is one this module can honour exactly:
//! - `OGC:CRS84` / `EPSG:4326` in any of their URN spellings — read as longitude/latitude, GJ2008's
//!   mandated axis order;
//! - spherical Web Mercator (`EPSG:3857` and its aliases `900913`, `102100`, `102113`) — inverse
//!   projected to WGS 84 (`lon = x/R`, `lat = 2·atan(e^{y/R}) − π/2`, `R = 6 378 137 m`), heights kept.
//!
//! Any other named CRS, and every linked CRS, is refused instead of guessed. After projection every
//! position must lie in `lon ∈ [−180, 180]`, `lat ∈ [−90, 90]` (projected metres written without a
//! `crs` fail here rather than land in the ocean off Africa). The writer never writes `crs`.

use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonSnapshot, JsonValue};
use std::collections::BTreeMap;

//#region 🔹Model
/// 📍️ One position: longitude, latitude and, when present, the ellipsoidal height (RFC 7946 §3.1.1).
pub type GeoJsonPosition = Vec<f64>;

/// 🔷️ An RFC 7946 §3.1 geometry object.
#[derive(Clone, Debug, PartialEq)]
pub enum GeoJsonGeometry {
    Point(GeoJsonPosition),
    MultiPoint(Vec<GeoJsonPosition>),
    LineString(Vec<GeoJsonPosition>),
    MultiLineString(Vec<Vec<GeoJsonPosition>>),
    Polygon(Vec<Vec<GeoJsonPosition>>),
    MultiPolygon(Vec<Vec<Vec<GeoJsonPosition>>>),
    GeometryCollection(Vec<GeoJsonGeometry>),
}

/// 🏷️ A feature identifier: RFC 7946 §3.2 allows a string or a number.
#[derive(Clone, Debug, PartialEq)]
pub enum GeoJsonId {
    Text(String),
    Number(JsonValue),
}

/// 🗺️ An RFC 7946 §3.2 feature; `properties: None` is the JSON `null` the RFC allows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeoJsonFeature {
    pub id: Option<GeoJsonId>,
    pub geometry: Option<GeoJsonGeometry>,
    pub properties: Option<BTreeMap<String, JsonValue>>,
}

/// 🧭️ The coordinate reference a document declared, after the policy above resolved it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeoJsonSourceCrs {
    Rfc7946,
    DeclaredCrs84,
    WebMercator,
}

/// 📖️ What the reader recovered: every feature in document order (a bare Feature is one, a bare
/// geometry one without id or properties), with every position in WGS 84.
#[derive(Clone, Debug, PartialEq)]
pub struct GeoJsonRead {
    pub features: Vec<GeoJsonFeature>,
    pub source_crs: GeoJsonSourceCrs,
    pub left_handed_rings: usize,
}

/// 🚫️ Why a JSON text is not readable as, or a model not writable as, RFC 7946 GeoJSON — `path` is a
/// JSON pointer to the offending member.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeoJsonError {
    pub path: String,
    pub message: String,
}

impl std::fmt::Display for GeoJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GeoJSON {}: {}", if self.path.is_empty() { "/" } else { &self.path }, self.message)
    }
}

impl std::error::Error for GeoJsonError {}
//#endregion 🔹Model

//#region 🔖️Reader
/// 🌐️ Web Mercator's sphere radius in metres (EPSG:3857).
const WEB_MERCATOR_RADIUS: f64 = 6_378_137.0;
/// 🪜️ Nesting bound for GeometryCollections (RFC 7946 §3.1.8 advises against nesting at all).
const MAXIMUM_GEOMETRY_DEPTH: usize = 32;













// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn source_crs(root: &JsonValue) -> Result<GeoJsonSourceCrs, GeoJsonError> {
    let Some(crs) = member(root, "crs") else { return Ok(GeoJsonSourceCrs::Rfc7946) };
    if text(member(crs, "type")) != Some("name") {
        return fail("/crs", "only a GJ2008 named crs can be honoured; linked or untyped CRS objects are refused (RFC 7946 §4 is WGS 84)");
    }
    let Some(name) = member(crs, "properties").and_then(|properties| text(member(properties, "name"))) else { return fail("/crs/properties/name", "a named crs carries its name as a string") };
    let code = name.trim().to_ascii_uppercase().replace("URN:OGC:DEF:CRS:", "").replace("::", ":").replace(":1.3:", ":");
    match code.as_str() {
        "OGC:CRS84" | "CRS84" | "EPSG:4326" => Ok(GeoJsonSourceCrs::DeclaredCrs84),
        "EPSG:3857" | "EPSG:900913" | "EPSG:102100" | "EPSG:102113" => Ok(GeoJsonSourceCrs::WebMercator),
        _ => fail("/crs/properties/name", format!("coordinate reference system `{name}` is not WGS 84 and has no exact reprojection here; reproject to WGS 84 (RFC 7946 §4) first")),
    }
}















/// 📖️ Reads an RFC 7946 GeoJSON document (the lexeme-exact `JsonValue` of the base subset's parser)
/// under this module's coordinate reference policy. Foreign members and `bbox` carry nothing into the
/// model; they are dropped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_geojson(root: &JsonValue) -> Result<GeoJsonRead, GeoJsonError> {
    let source_crs = source_crs(root)?;
    let mut left_handed_rings = 0;
    let features = match text(member(root, "type")) {
        Some("FeatureCollection") => {
            let Some(members) = items(member(root, "features")) else { return fail("/features", "a FeatureCollection has a `features` array") };
            members.iter().enumerate().map(|(index, part)| feature(part, source_crs, &format!("/features/{index}"), &mut left_handed_rings)).collect::<Result<_, _>>()?
        }
        Some("Feature") => vec![feature(root, source_crs, "", &mut left_handed_rings)?],
        Some(_) => vec![GeoJsonFeature { id: None, geometry: Some(geometry(root, source_crs, "", 0, &mut left_handed_rings)?), properties: None }],
        None => return fail("/type", "a GeoJSON object has a string `type`"),
    };
    Ok(GeoJsonRead { features, source_crs, left_handed_rings })
}


//#endregion 🔖️Reader

//#region 🔖️Writer











//#endregion 🔖️Writer

//#region 🔖️Conformance
pub const CODE_NOT_GEOJSON: &str = "stdio.json.geojson.not-rfc7946";
pub const CODE_LEGACY_CRS: &str = "stdio.json.geojson.legacy-crs";
pub const CODE_LEFT_HANDED_RING: &str = "stdio.json.geojson.left-handed-ring";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diagnostic(code: &'static str, severity: semio_framework_diagnostic::Severity, message: String) -> semio_framework_diagnostic::Diagnostic {
    semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new(code), severity, span: semio_framework_diagnostic::TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
}

/// 🛡️ Applies the semantic RFC7946 constraints without a storage authority.
pub fn check_geojson_conformance(snapshot:&JsonSnapshot)->Vec<semio_framework_diagnostic::Diagnostic>{check_geojson_conformance_with_control(snapshot,&mut conformance::UnboundedGeoJsonConformance).expect("unbounded semantic GeoJSON conformance")}

//#endregion 🔖️Conformance

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path="🛡️conformance/🦀️.rs"]
mod conformance;
pub use conformance::{GeoJsonConformanceControl,check_geojson_conformance_with_control};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn fail<T>(path: &str, message: impl Into<String>) -> Result<T, GeoJsonError> {
    Err(GeoJsonError { path: path.into(), message: message.into() })
}

/// 🔑️ The member `key` of an object (the last one when a name repeats, as RFC 8259 readers commonly do).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn member<'a>(value: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    match value {
        JsonValue::Object { members } => members.iter().rev().find(|member| member.key == key).map(|member| &member.value),
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn text(value: Option<&JsonValue>) -> Option<&str> {
    match value {
        Some(JsonValue::String { value }) => Some(value),
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn items(value: Option<&JsonValue>) -> Option<&[JsonValue]> {
    match value {
        Some(JsonValue::Array { items }) => Some(items),
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn geometry(value: &JsonValue, crs: GeoJsonSourceCrs, path: &str, depth: usize, left_handed: &mut usize) -> Result<GeoJsonGeometry, GeoJsonError> {
    if depth > MAXIMUM_GEOMETRY_DEPTH {
        return fail(path, format!("GeometryCollections nest deeper than {MAXIMUM_GEOMETRY_DEPTH}"));
    }
    let Some(kind) = text(member(value, "type")) else { return fail(path, "a geometry has a string `type`") };
    if kind == "GeometryCollection" {
        let Some(members) = items(member(value, "geometries")) else { return fail(&format!("{path}/geometries"), "a GeometryCollection has a `geometries` array") };
        return members.iter().enumerate().map(|(index, part)| geometry(part, crs, &format!("{path}/geometries/{index}"), depth + 1, left_handed)).collect::<Result<_, _>>().map(GeoJsonGeometry::GeometryCollection);
    }
    let coordinates_path = format!("{path}/coordinates");
    let Some(coordinates) = member(value, "coordinates") else { return fail(&coordinates_path, format!("a {kind} has `coordinates`")) };
    let parts = |what: &str| -> Result<&[JsonValue], GeoJsonError> { items(Some(coordinates)).ok_or_else(|| GeoJsonError { path: coordinates_path.clone(), message: format!("{what} coordinates are an array") }) };
    Ok(match kind {
        "Point" => GeoJsonGeometry::Point(position(coordinates, crs, &coordinates_path)?),
        "MultiPoint" => GeoJsonGeometry::MultiPoint(positions(coordinates, crs, &coordinates_path, 0, "a MultiPoint")?),
        "LineString" => GeoJsonGeometry::LineString(positions(coordinates, crs, &coordinates_path, 2, "a LineString")?),
        "MultiLineString" => GeoJsonGeometry::MultiLineString(parts("a MultiLineString")?.iter().enumerate().map(|(index, line)| positions(line, crs, &format!("{coordinates_path}/{index}"), 2, "a LineString")).collect::<Result<_, _>>()?),
        "Polygon" => GeoJsonGeometry::Polygon(polygon(coordinates, crs, &coordinates_path, left_handed)?),
        "MultiPolygon" => GeoJsonGeometry::MultiPolygon(parts("a MultiPolygon")?.iter().enumerate().map(|(index, rings)| polygon(rings, crs, &format!("{coordinates_path}/{index}"), left_handed)).collect::<Result<_, _>>()?),
        other => return fail(&format!("{path}/type"), format!("`{other}` is not an RFC 7946 geometry type")),
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn feature(value: &JsonValue, crs: GeoJsonSourceCrs, path: &str, left_handed: &mut usize) -> Result<GeoJsonFeature, GeoJsonError> {
    if text(member(value, "type")) != Some("Feature") {
        return fail(&format!("{path}/type"), "a feature has `\"type\": \"Feature\"`");
    }
    let id = match member(value, "id") {
        None => None,
        Some(JsonValue::String { value }) => Some(GeoJsonId::Text(value.clone())),
        Some(value @ JsonValue::Number { .. }) if number(value).is_some() => Some(GeoJsonId::Number(value.clone())),
        Some(_) => return fail(&format!("{path}/id"), "a feature id is a string or a number (RFC 7946 §3.2)"),
    };
    let geometry = match member(value, "geometry") {
        None => return fail(&format!("{path}/geometry"), "a feature has a `geometry` member (an object or null)"),
        Some(JsonValue::Null) => None,
        Some(geometry_value) => Some(geometry(geometry_value, crs, &format!("{path}/geometry"), 0, left_handed)?),
    };
    let properties = match member(value, "properties") {
        None => return fail(&format!("{path}/properties"), "a feature has a `properties` member (an object or null)"),
        Some(JsonValue::Null) => None,
        Some(JsonValue::Object { members }) => Some(members.iter().map(|member| (member.key.clone(), member.value.clone())).collect()),
        Some(_) => return fail(&format!("{path}/properties"), "feature properties are an object or null"),
    };
    Ok(GeoJsonFeature { id, geometry, properties })
}

/// 🔢️ A number lexeme read with Rust's correctly rounded decimal conversion — the lexeme is the
/// document's own text, so no intermediate parser can move a coordinate by an ulp.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn number(value: &JsonValue) -> Option<f64> {
    match value {
        JsonValue::Number { lexeme } => lexeme.parse::<f64>().ok().filter(|number| number.is_finite()),
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn position(value: &JsonValue, crs: GeoJsonSourceCrs, path: &str) -> Result<GeoJsonPosition, GeoJsonError> {
    let Some(coordinates) = items(Some(value)) else { return fail(path, "a position is an array of numbers") };
    if coordinates.len() < 2 {
        return fail(path, "a position has at least longitude and latitude");
    }
    let mut numbers = Vec::with_capacity(coordinates.len());
    for (index, coordinate) in coordinates.iter().enumerate() {
        let Some(value) = number(coordinate) else { return fail(&format!("{path}/{index}"), "a coordinate is a finite number") };
        numbers.push(value);
    }
    if crs == GeoJsonSourceCrs::WebMercator {
        numbers[0] = (numbers[0] / WEB_MERCATOR_RADIUS).to_degrees();
        numbers[1] = (2.0 * (numbers[1] / WEB_MERCATOR_RADIUS).exp().atan() - std::f64::consts::FRAC_PI_2).to_degrees();
    }
    wgs84_range(&numbers, path)?;
    Ok(numbers)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn positions(value: &JsonValue, crs: GeoJsonSourceCrs, path: &str, minimum: usize, what: &str) -> Result<Vec<GeoJsonPosition>, GeoJsonError> {
    let Some(list) = items(Some(value)) else { return fail(path, format!("{what} coordinates are an array of positions")) };
    if list.len() < minimum {
        return fail(path, format!("{what} needs at least {minimum} positions"));
    }
    list.iter().enumerate().map(|(index, item)| position(item, crs, &format!("{path}/{index}"))).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn polygon(value: &JsonValue, crs: GeoJsonSourceCrs, path: &str, left_handed: &mut usize) -> Result<Vec<Vec<GeoJsonPosition>>, GeoJsonError> {
    let Some(rings) = items(Some(value)) else { return fail(path, "Polygon coordinates are an array of linear rings") };
    if rings.is_empty() {
        return fail(path, "a Polygon has an exterior ring");
    }
    let mut out = Vec::with_capacity(rings.len());
    for (index, ring) in rings.iter().enumerate() {
        let ring_path = format!("{path}/{index}");
        let ring = positions(ring, crs, &ring_path, 4, "a linear ring")?;
        if ring.first() != ring.last() {
            return fail(&ring_path, "a linear ring is closed: its first and last positions are identical (RFC 7946 §3.1.6)");
        }
        if (ring_signed_area2(&ring) > 0.0) != (index == 0) {
            *left_handed += 1;
        }
        out.push(ring);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wgs84_range(position: &[f64], path: &str) -> Result<(), GeoJsonError> {
    if !(-180.0..=180.0).contains(&position[0]) || !(-90.0..=90.0).contains(&position[1]) {
        return fail(path, format!("[{}, {}] is not a WGS 84 longitude/latitude (lon ∈ [−180, 180], lat ∈ [−90, 90])", position[0], position[1]));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


pub fn ring_signed_area2(ring: &[GeoJsonPosition]) -> f64 {
    ring.windows(2).map(|pair| pair[0][0] * pair[1][1] - pair[1][0] * pair[0][1]).sum()
}
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
