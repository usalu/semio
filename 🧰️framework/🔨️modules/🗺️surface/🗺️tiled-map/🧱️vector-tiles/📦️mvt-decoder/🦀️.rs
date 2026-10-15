//! 📦️ Mapbox Vector Tile decoder: protobuf cursor, raw layer decode, geometry decode, property helpers and tile-edge geometry tests.
//!
//! Domain-neutral: nothing here knows a map vendor, tile schema or level-of-detail policy. Public paths stay under `vector_tiles` through the facade re-export.

// #region 🔖️MvtDecoder
// #region 🔖️VectorTiles
/// OpenFreeMap planet tiles (OpenMapTiles schema, OSM) — max generated zoom.
pub const MAP_VECTOR_TILE_MAX_Z: u32 = 14;
pub const DEFAULT_MVT_EXTENT: u32 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomType {
    Unknown,
    Point,
    LineString,
    Polygon,
}

#[derive(Clone, Debug, Default)]
pub struct VectorTile {
    pub layers: Vec<VectorLayer>,
}

#[derive(Clone, Debug)]
pub struct VectorLayer {
    pub name: String,
    pub extent: u32,
    pub features: Vec<VectorFeature>,
}

#[derive(Clone, Debug)]
pub struct VectorFeature {
    pub id: Option<u64>,
    pub geom_type: GeomType,
    pub rings: Vec<Vec<(f64, f64)>>,
    pub lines: Vec<Vec<(f64, f64)>>,
    pub points: Vec<(f64, f64)>,
    pub properties: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RawTile {
    version: Option<u32>,
    layers: Vec<RawLayer>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RawLayer {
    version: Option<u32>,
    name: Option<String>,
    features: Vec<RawFeature>,
    keys: Vec<String>,
    values: Vec<RawValue>,
    extent: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RawFeature {
    id: Option<u64>,
    tags: Vec<u64>,
    geom_type: Option<i32>,
    geometry: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
enum RawGeomType {
    Unknown = 0,
    Point = 1,
    LineString = 2,
    Polygon = 3,
}

impl TryFrom<i32> for RawGeomType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Point),
            2 => Ok(Self::LineString),
            3 => Ok(Self::Polygon),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RawValue {
    string_value: Option<String>,
    float_value: Option<f32>,
    double_value: Option<f64>,
    int_value: Option<i64>,
    uint_value: Option<u64>,
    bool_value: Option<bool>,
}

//#region 🧩️OwnedProtobuf
/// 🧩️ A bounded protobuf decode failure with the byte offset at which parsing stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MvtDecodeError {
    offset: usize,
    message: String,
}

impl MvtDecodeError {
    fn new(offset: usize, message: impl Into<String>) -> Self {
        Self { offset, message: message.into() }
    }
}

impl std::fmt::Display for MvtDecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "MVT protobuf decode failed at byte {}: {}", self.offset, self.message)
    }
}

impl std::error::Error for MvtDecodeError {}

struct ProtoCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> ProtoCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn is_empty(&self) -> bool {
        self.offset == self.bytes.len()
    }

    fn error(&self, message: impl Into<String>) -> MvtDecodeError {
        MvtDecodeError::new(self.offset, message)
    }

    fn read_byte(&mut self) -> Result<u8, MvtDecodeError> {
        let byte = self.bytes.get(self.offset).copied().ok_or_else(|| self.error("truncated input"))?;
        self.offset += 1;
        Ok(byte)
    }

    fn read_varint(&mut self) -> Result<u64, MvtDecodeError> {
        let start = self.offset;
        let mut value = 0u64;
        for index in 0..10u32 {
            let byte = self.read_byte()?;
            if index == 9 && byte > 1 {
                return Err(MvtDecodeError::new(start, "varint overflows u64"));
            }
            value |= u64::from(byte & 0x7f) << (index * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(MvtDecodeError::new(start, "unterminated varint"))
    }

    fn read_key(&mut self) -> Result<(u32, u8), MvtDecodeError> {
        let start = self.offset;
        let key = self.read_varint()?;
        let tag = u32::try_from(key >> 3).map_err(|_| MvtDecodeError::new(start, "field tag overflows u32"))?;
        if tag == 0 {
            return Err(MvtDecodeError::new(start, "field tag zero is invalid"));
        }
        Ok((tag, (key & 0x07) as u8))
    }

    fn read_exact(&mut self, length: usize) -> Result<&'a [u8], MvtDecodeError> {
        let end = self.offset.checked_add(length).ok_or_else(|| self.error("length overflows address space"))?;
        let bytes = self.bytes.get(self.offset..end).ok_or_else(|| self.error("length-delimited field is truncated"))?;
        self.offset = end;
        Ok(bytes)
    }

    fn read_length_delimited(&mut self) -> Result<&'a [u8], MvtDecodeError> {
        let length = usize::try_from(self.read_varint()?).map_err(|_| self.error("field length overflows usize"))?;
        self.read_exact(length)
    }

    fn read_string(&mut self) -> Result<String, MvtDecodeError> {
        let start = self.offset;
        String::from_utf8(self.read_length_delimited()?.to_vec()).map_err(|_| MvtDecodeError::new(start, "string field is not UTF-8"))
    }

    fn read_fixed32(&mut self) -> Result<u32, MvtDecodeError> {
        Ok(u32::from_le_bytes(self.read_exact(4)?.try_into().expect("fixed32 length")))
    }

    fn read_fixed64(&mut self) -> Result<u64, MvtDecodeError> {
        Ok(u64::from_le_bytes(self.read_exact(8)?.try_into().expect("fixed64 length")))
    }

    fn require_wire(&self, actual: u8, expected: u8) -> Result<(), MvtDecodeError> {
        if actual == expected {
            Ok(())
        } else {
            Err(self.error(format!("wire type {actual} does not match expected type {expected}")))
        }
    }

    fn skip(&mut self, wire: u8) -> Result<(), MvtDecodeError> {
        match wire {
            0 => {
                self.read_varint()?;
            }
            1 => {
                self.read_exact(8)?;
            }
            2 => {
                self.read_length_delimited()?;
            }
            5 => {
                self.read_exact(4)?;
            }
            _ => return Err(self.error(format!("unsupported wire type {wire}"))),
        }
        Ok(())
    }
}

fn decode_packed_u64(bytes: &[u8]) -> Result<Vec<u64>, MvtDecodeError> {
    let mut cursor = ProtoCursor::new(bytes);
    let mut values = Vec::new();
    while !cursor.is_empty() {
        values.push(cursor.read_varint()?);
    }
    Ok(values)
}

fn decode_raw_value(bytes: &[u8]) -> Result<RawValue, MvtDecodeError> {
    let mut cursor = ProtoCursor::new(bytes);
    let mut value = RawValue::default();
    while !cursor.is_empty() {
        let (tag, wire) = cursor.read_key()?;
        match tag {
            1 => {
                cursor.require_wire(wire, 2)?;
                value.string_value = Some(cursor.read_string()?);
            }
            2 => {
                cursor.require_wire(wire, 5)?;
                value.float_value = Some(f32::from_bits(cursor.read_fixed32()?));
            }
            3 => {
                cursor.require_wire(wire, 1)?;
                value.double_value = Some(f64::from_bits(cursor.read_fixed64()?));
            }
            4 => {
                cursor.require_wire(wire, 0)?;
                value.int_value = Some(cursor.read_varint()? as i64);
            }
            5 => {
                cursor.require_wire(wire, 0)?;
                value.uint_value = Some(cursor.read_varint()?);
            }
            7 => {
                cursor.require_wire(wire, 0)?;
                value.bool_value = Some(cursor.read_varint()? != 0);
            }
            _ => cursor.skip(wire)?,
        }
    }
    Ok(value)
}

fn decode_raw_feature(bytes: &[u8]) -> Result<RawFeature, MvtDecodeError> {
    let mut cursor = ProtoCursor::new(bytes);
    let mut feature = RawFeature::default();
    while !cursor.is_empty() {
        let (tag, wire) = cursor.read_key()?;
        match tag {
            1 => {
                cursor.require_wire(wire, 0)?;
                feature.id = Some(cursor.read_varint()?);
            }
            2 if wire == 2 => feature.tags.extend(decode_packed_u64(cursor.read_length_delimited()?)?),
            2 if wire == 0 => feature.tags.push(cursor.read_varint()?),
            2 => return Err(cursor.error(format!("wire type {wire} is invalid for feature tags"))),
            3 => {
                cursor.require_wire(wire, 0)?;
                feature.geom_type = Some(cursor.read_varint()? as i32);
            }
            4 if wire == 2 => {
                feature.geometry.extend(decode_packed_u64(cursor.read_length_delimited()?)?.into_iter().map(|value| value as u32));
            }
            4 if wire == 0 => feature.geometry.push(cursor.read_varint()? as u32),
            4 => return Err(cursor.error(format!("wire type {wire} is invalid for feature geometry"))),
            _ => cursor.skip(wire)?,
        }
    }
    Ok(feature)
}

fn decode_raw_layer(bytes: &[u8]) -> Result<RawLayer, MvtDecodeError> {
    let mut cursor = ProtoCursor::new(bytes);
    let mut layer = RawLayer::default();
    while !cursor.is_empty() {
        let (tag, wire) = cursor.read_key()?;
        match tag {
            1 => {
                cursor.require_wire(wire, 2)?;
                layer.name = Some(cursor.read_string()?);
            }
            2 => {
                cursor.require_wire(wire, 2)?;
                layer.features.push(decode_raw_feature(cursor.read_length_delimited()?)?);
            }
            3 => {
                cursor.require_wire(wire, 2)?;
                layer.keys.push(cursor.read_string()?);
            }
            4 => {
                cursor.require_wire(wire, 2)?;
                layer.values.push(decode_raw_value(cursor.read_length_delimited()?)?);
            }
            5 => {
                cursor.require_wire(wire, 0)?;
                layer.extent = Some(cursor.read_varint()? as u32);
            }
            15 => {
                cursor.require_wire(wire, 0)?;
                layer.version = Some(cursor.read_varint()? as u32);
            }
            _ => cursor.skip(wire)?,
        }
    }
    Ok(layer)
}

fn decode_raw_tile(bytes: &[u8]) -> Result<RawTile, MvtDecodeError> {
    let mut cursor = ProtoCursor::new(bytes);
    let mut tile = RawTile::default();
    while !cursor.is_empty() {
        let (tag, wire) = cursor.read_key()?;
        match tag {
            3 => {
                cursor.require_wire(wire, 2)?;
                tile.layers.push(decode_raw_layer(cursor.read_length_delimited()?)?);
            }
            15 => {
                cursor.require_wire(wire, 0)?;
                tile.version = Some(cursor.read_varint()? as u32);
            }
            _ => cursor.skip(wire)?,
        }
    }
    Ok(tile)
}
//#endregion 🧩️OwnedProtobuf

fn zigzag_decode(n: u32) -> i32 {
    ((n >> 1) as i32) ^ (-((n & 1) as i32))
}

fn decode_properties(tags: &[u64], keys: &[String], values: &[RawValue]) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    let mut i = 0usize;
    while i + 1 < tags.len() {
        let key_idx = tags[i] as usize;
        let val_idx = tags[i + 1] as usize;
        i += 2;
        let Some(key) = keys.get(key_idx) else {
            continue;
        };
        let Some(raw) = values.get(val_idx) else {
            continue;
        };
        let val = raw_value_string(raw);
        if !val.is_empty() {
            out.insert(key.clone(), val);
        }
    }
    out
}

fn raw_value_string(v: &RawValue) -> String {
    if let Some(s) = &v.string_value {
        return s.clone();
    }
    if let Some(n) = v.int_value {
        return n.to_string();
    }
    if let Some(n) = v.uint_value {
        return n.to_string();
    }
    if let Some(n) = v.double_value {
        return n.to_string();
    }
    if let Some(n) = v.float_value {
        return n.to_string();
    }
    if let Some(b) = v.bool_value {
        return b.to_string();
    }
    String::new()
}

struct DecodedGeometry {
    rings: Vec<Vec<(f64, f64)>>,
    lines: Vec<Vec<(f64, f64)>>,
    points: Vec<(f64, f64)>,
}

fn decode_geometry(geometry: &[u32], geom_type: GeomType) -> DecodedGeometry {
    let mut rings = Vec::new();
    let mut lines = Vec::new();
    let mut points = Vec::new();
    let mut i = 0usize;
    let mut cursor_x = 0i32;
    let mut cursor_y = 0i32;
    let mut current: Vec<(f64, f64)> = Vec::new();
    while i < geometry.len() {
        let cmd_int = geometry[i];
        i += 1;
        let cmd = cmd_int & 0x7;
        let count = (cmd_int >> 3) as usize;
        match cmd {
            1 => {
                for _ in 0..count {
                    if i + 1 >= geometry.len() {
                        break;
                    }
                    cursor_x += zigzag_decode(geometry[i]);
                    cursor_y += zigzag_decode(geometry[i + 1]);
                    i += 2;
                    if matches!(geom_type, GeomType::Polygon) && !current.is_empty() {
                        rings.push(std::mem::take(&mut current));
                    }
                    if matches!(geom_type, GeomType::LineString) && !current.is_empty() {
                        lines.push(std::mem::take(&mut current));
                    }
                    current.push((cursor_x as f64, cursor_y as f64));
                }
            }
            2 => {
                for _ in 0..count {
                    if i + 1 >= geometry.len() {
                        break;
                    }
                    cursor_x += zigzag_decode(geometry[i]);
                    cursor_y += zigzag_decode(geometry[i + 1]);
                    i += 2;
                    current.push((cursor_x as f64, cursor_y as f64));
                }
            }
            7 => {
                if current.len() >= 2 {
                    if matches!(geom_type, GeomType::Polygon) {
                        if let Some(first) = current.first() {
                            if current.last() != Some(first) {
                                current.push(*first);
                            }
                        }
                        rings.push(std::mem::take(&mut current));
                    } else if matches!(geom_type, GeomType::LineString) {
                        lines.push(std::mem::take(&mut current));
                    }
                }
                current.clear();
            }
            _ => break,
        }
    }
    if !current.is_empty() {
        match geom_type {
            GeomType::Polygon => rings.push(current),
            GeomType::LineString => lines.push(current),
            GeomType::Point => {
                if let Some(p) = current.first() {
                    points.push(*p);
                }
            }
            GeomType::Unknown => {}
        }
    }
    DecodedGeometry { rings, lines, points }
}

#[cfg(test)]
include!("../../🧪️tests/🔬️vector-tiles-decode-geometry/🦀️.rs");

pub fn decode_mvt(bytes: &[u8]) -> Result<VectorTile, super::FrameworkSurfaceTiledMapError> {
    let raw = decode_raw_tile(bytes)?;
    let mut layers = Vec::new();
    for layer in raw.layers {
        let extent = layer.extent.filter(|e| *e > 0).unwrap_or(DEFAULT_MVT_EXTENT);
        let name = layer.name.unwrap_or_default();
        let mut features = Vec::new();
        for feat in layer.features {
            let geom_type = match feat.geom_type.and_then(|g| RawGeomType::try_from(g).ok()) {
                Some(RawGeomType::Point) => GeomType::Point,
                Some(RawGeomType::LineString) => GeomType::LineString,
                Some(RawGeomType::Polygon) => GeomType::Polygon,
                _ => GeomType::Unknown,
            };
            let props = decode_properties(&feat.tags, &layer.keys, &layer.values);
            let DecodedGeometry { rings, lines, points } = decode_geometry(&feat.geometry, geom_type);
            features.push(VectorFeature { id: feat.id.filter(|id| *id != 0), geom_type, rings, lines, points, properties: props });
        }
        layers.push(VectorLayer { name, extent, features });
    }
    Ok(VectorTile { layers })
}

pub fn property_str<'a>(properties: &'a std::collections::BTreeMap<String, String>, key: &str) -> Option<&'a str> {
    properties.get(key).map(|s| s.as_str()).filter(|s| !s.is_empty())
}

pub fn property_class(properties: &std::collections::BTreeMap<String, String>) -> &str {
    property_str(properties, "class").unwrap_or("")
}

pub fn property_u64(properties: &std::collections::BTreeMap<String, String>, key: &str) -> Option<u64> {
    properties.get(key)?.parse().ok()
}

pub fn feature_label(properties: &std::collections::BTreeMap<String, String>) -> Option<String> {
    for key in ["name:en", "name_en", "name_int", "name", "name:latin", "NAME", "Name", "title", "TITLE", "ref"] {
        if let Some(v) = properties.get(key) {
            let t = v.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    None
}

/// 📍️ Tile-local anchor for a label — point geometry first, then the midpoint of the longest line
/// (`transportation_name` / `water_name` are line strings, not points).
pub fn feature_label_anchor(feature: &VectorFeature, extent: u32) -> (f64, f64) {
    if let Some(point) = feature.points.first() {
        return *point;
    }
    if let Some(line) = feature.lines.iter().max_by_key(|line| line.len()).filter(|line| !line.is_empty()) {
        return line[line.len() / 2];
    }
    if let Some(ring) = feature.rings.first().and_then(|ring| ring.first()) {
        return *ring;
    }
    (f64::from(extent) / 2.0, f64::from(extent) / 2.0)
}

pub fn property_flag(properties: &std::collections::BTreeMap<String, String>, key: &str) -> bool {
    property_str(properties, key).is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

const TILE_BBOX_EPS: f64 = 1.0;

pub fn mvt_point_on_tile_bbox_edge(extent: u32, x: f64, y: f64) -> bool {
    let eps = TILE_BBOX_EPS;
    let e = f64::from(extent);
    x <= eps || y <= eps || x >= e - eps || y >= e - eps
}

pub fn mvt_segment_is_tile_seam(extent: u32, a: (f64, f64), b: (f64, f64)) -> bool {
    if !mvt_point_on_tile_bbox_edge(extent, a.0, a.1) || !mvt_point_on_tile_bbox_edge(extent, b.0, b.1) {
        return false;
    }
    let eps = TILE_BBOX_EPS;
    let e = f64::from(extent);
    (a.1 <= eps && b.1 <= eps) || (a.1 >= e - eps && b.1 >= e - eps) || (a.0 <= eps && b.0 <= eps) || (a.0 >= e - eps && b.0 >= e - eps)
}

pub fn mvt_segment_touches_tile_bbox(extent: u32, a: (f64, f64), b: (f64, f64)) -> bool {
    mvt_point_on_tile_bbox_edge(extent, a.0, a.1) || mvt_point_on_tile_bbox_edge(extent, b.0, b.1)
}

pub fn mvt_polyline_is_tile_bbox_artifact(extent: u32, line: &[(f64, f64)]) -> bool {
    line.len() >= 2 && line.iter().all(|&(x, y)| mvt_point_on_tile_bbox_edge(extent, x, y))
}

pub fn mvt_ring_is_tile_bbox_cover(extent: u32, ring: &[(f64, f64)]) -> bool {
    if ring.len() < 4 {
        return false;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for &(x, y) in ring {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let eps = TILE_BBOX_EPS;
    let e = f64::from(extent);
    min_x <= eps && min_y <= eps && max_x >= e - eps && max_y >= e - eps
}
// #endregion 🔖️VectorTiles
// #endregion 🔖️MvtDecoder
