//! 🧬️ DwgSnapshot schema — persistent fields + real codecs. Byte/bit-level decode logic (file
//! header decrypt, R2004+ LZ77-variant decompression, section/page directory walk) lives in
//! `⚙️engine` (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION 🖊️dwg
//! D1-D2 wave); this module owns the typed persisted model and glues `decode_dwg`/`encode_dwg`
//! to it.

use crate::artifacts::dwg::standards::v_ac1024::engine as dwg_engine;
use crate::artifacts::dwg::STDIO_DWG_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

//#region 🔖️DrawingModel
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgLogicalGeometryKind {
    #[default]
    Point,
    Line,
    Circle,
    Arc,
    Ellipse,
    LwPolyline,
    Spline,
    Text,
    Face3d,
    Polyline3d,
    PolyfaceMesh,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalGeometry {
    pub kind: DwgLogicalGeometryKind,
    #[serde(default)]
    pub values: Vec<f64>,
    #[serde(default)]
    pub indices: Vec<i32>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub closed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalLayer {
    pub name: String,
    pub color: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalEntity {
    pub layer: usize,
    pub color: i16,
    pub geometry: DwgLogicalGeometry,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgObjectCategory {
    Entity,
    TableControl,
    TableRecord,
    Dictionary,
    #[default]
    Object,
    Custom,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgNamedReference {
    pub name: String,
    pub handle: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DwgXRecordValue {
    String { group_code: i16, value: String },
    Real { group_code: i16, value: f64 },
    Boolean { group_code: i16, value: bool },
    Integer8 { group_code: i16, value: i8 },
    Integer16 { group_code: i16, value: i16 },
    Integer32 { group_code: i16, value: i32 },
    Integer64 { group_code: i16, value: i64 },
    Point3d { group_code: i16, value: [f64; 3] },
    Binary { group_code: i16, octets: Vec<u8> },
    Handle { group_code: i16, value: u64 },
    ObjectId { group_code: i16, absolute_value: u64 },
}

impl Default for DwgXRecordValue {
    fn default() -> Self {
        Self::String { group_code: 1, value: String::new() }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_xrecord_value_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("string".into(), 0),
                    ("real".into(), 1),
                    ("boolean".into(), 2),
                    ("integer8".into(), 3),
                    ("integer16".into(), 4),
                    ("integer32".into(), 5),
                    ("integer64".into(), 6),
                    ("point3d".into(), 7),
                    ("binary".into(), 8),
                    ("handle".into(), 9),
                    ("objectId".into(), 10),
                ]),
            ),
            dsl::FieldSpec::new(1, "group_code", dsl::Shape::Int),
            dsl::FieldSpec::new(2, "string_value", dsl::Shape::Text).optional(),
            dsl::FieldSpec::new(3, "real_value", dsl::Shape::Float).optional(),
            dsl::FieldSpec::new(4, "boolean_value", dsl::Shape::Bool).optional(),
            dsl::FieldSpec::new(5, "integer_value", dsl::Shape::Int).optional(),
            dsl::FieldSpec::new(6, "point_value", dsl::Shape::Tuple(Box::new(dsl::Shape::Float), Some(3))).optional(),
            dsl::FieldSpec::new(7, "binary_octets", dsl::Shape::Bytes64).optional(),
            dsl::FieldSpec::new(8, "handle_value", dsl::Shape::UInt).optional(),
        ],
    )
}

impl dsl::DslField for DwgXRecordValue {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_xrecord_value_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let (kind, group_code, payload_id, payload) = match self {
            Self::String { group_code, value } => (0, *group_code, 2, dsl::FieldValue::Text(value.clone())),
            Self::Real { group_code, value } => (1, *group_code, 3, dsl::FieldValue::Float(*value)),
            Self::Boolean { group_code, value } => (2, *group_code, 4, dsl::FieldValue::Bool(*value)),
            Self::Integer8 { group_code, value } => (3, *group_code, 5, dsl::FieldValue::Int(i64::from(*value))),
            Self::Integer16 { group_code, value } => (4, *group_code, 5, dsl::FieldValue::Int(i64::from(*value))),
            Self::Integer32 { group_code, value } => (5, *group_code, 5, dsl::FieldValue::Int(i64::from(*value))),
            Self::Integer64 { group_code, value } => (6, *group_code, 5, dsl::FieldValue::Int(*value)),
            Self::Point3d { group_code, value } => (7, *group_code, 6, dsl::FieldValue::Tuple(value.iter().copied().map(dsl::FieldValue::Float).collect())),
            Self::Binary { group_code, octets } => (8, *group_code, 7, dsl::FieldValue::Bytes64(octets.clone())),
            Self::Handle { group_code, value } => (9, *group_code, 8, dsl::FieldValue::UInt(*value)),
            Self::ObjectId { group_code, absolute_value, .. } => (10, *group_code, 8, dsl::FieldValue::UInt(*absolute_value)),
        };
        record.fields.insert(0, dsl::FieldValue::Enum(kind));
        record.fields.insert(1, dsl::FieldValue::Int(i64::from(group_code)));
        record.fields.insert(payload_id, payload);
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else {
            return Err(format!("expected XRECORD value record, found {value:?}"));
        };
        if record.fields.values().filter(|value| !matches!(value, dsl::FieldValue::Absent)).count() != 3 {
            return Err("XRECORD value must contain exactly kind, group code, and one payload".into());
        }
        let kind = match record.get(0) {
            Some(dsl::FieldValue::Enum(value)) => *value,
            other => return Err(format!("expected XRECORD value kind, found {other:?}")),
        };
        let group_code = match record.get(1) {
            Some(dsl::FieldValue::Int(value)) => i16::try_from(*value).map_err(|_| format!("XRECORD group code {value} exceeds i16"))?,
            other => return Err(format!("expected XRECORD group code, found {other:?}")),
        };
        let result = match kind {
            0 => Self::String {
                group_code,
                value: match record.get(2) {
                    Some(dsl::FieldValue::Text(value)) => value.clone(),
                    other => return Err(format!("expected XRECORD string, found {other:?}")),
                },
            },
            1 => Self::Real {
                group_code,
                value: match record.get(3) {
                    Some(dsl::FieldValue::Float(value)) => *value,
                    other => return Err(format!("expected XRECORD real, found {other:?}")),
                },
            },
            2 => Self::Boolean {
                group_code,
                value: match record.get(4) {
                    Some(dsl::FieldValue::Bool(value)) => *value,
                    other => return Err(format!("expected XRECORD boolean, found {other:?}")),
                },
            },
            3 => Self::Integer8 {
                group_code,
                value: match record.get(5) {
                    Some(dsl::FieldValue::Int(value)) => i8::try_from(*value).map_err(|_| format!("XRECORD integer8 {value} is out of range"))?,
                    other => return Err(format!("expected XRECORD integer8, found {other:?}")),
                },
            },
            4 => Self::Integer16 {
                group_code,
                value: match record.get(5) {
                    Some(dsl::FieldValue::Int(value)) => i16::try_from(*value).map_err(|_| format!("XRECORD integer16 {value} is out of range"))?,
                    other => return Err(format!("expected XRECORD integer16, found {other:?}")),
                },
            },
            5 => Self::Integer32 {
                group_code,
                value: match record.get(5) {
                    Some(dsl::FieldValue::Int(value)) => i32::try_from(*value).map_err(|_| format!("XRECORD integer32 {value} is out of range"))?,
                    other => return Err(format!("expected XRECORD integer32, found {other:?}")),
                },
            },
            6 => Self::Integer64 {
                group_code,
                value: match record.get(5) {
                    Some(dsl::FieldValue::Int(value)) => *value,
                    other => return Err(format!("expected XRECORD integer64, found {other:?}")),
                },
            },
            7 => Self::Point3d {
                group_code,
                value: match record.get(6) {
                    Some(dsl::FieldValue::Tuple(values)) if values.len() == 3 => [0, 1, 2]
                        .map(|index| match values.get(index) {
                            Some(dsl::FieldValue::Float(value)) => Ok(*value),
                            other => Err(format!("expected XRECORD point coordinate, found {other:?}")),
                        })
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()?
                        .try_into()
                        .map_err(|_| "XRECORD point must contain three coordinates")?,
                    other => return Err(format!("expected XRECORD point, found {other:?}")),
                },
            },
            8 => Self::Binary {
                group_code,
                octets: match record.get(7) {
                    Some(dsl::FieldValue::Bytes64(value)) => value.clone(),
                    other => return Err(format!("expected XRECORD binary value, found {other:?}")),
                },
            },
            9 => Self::Handle {
                group_code,
                value: match record.get(8) {
                    Some(dsl::FieldValue::UInt(value)) => *value,
                    other => return Err(format!("expected XRECORD handle, found {other:?}")),
                },
            },
            10 => Self::ObjectId {
                group_code,
                absolute_value: match record.get(8) {
                    Some(dsl::FieldValue::UInt(value)) => *value,
                    other => return Err(format!("expected XRECORD absolute object id, found {other:?}")),
                },
            },
            other => return Err(format!("unknown XRECORD value kind ordinal {other}")),
        };
        result.validate()?;
        Ok(result)
    }
}

impl DwgXRecordValue {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn group_code(&self) -> i16 {
        match self {
            Self::String { group_code, .. }
            | Self::Real { group_code, .. }
            | Self::Boolean { group_code, .. }
            | Self::Integer8 { group_code, .. }
            | Self::Integer16 { group_code, .. }
            | Self::Integer32 { group_code, .. }
            | Self::Integer64 { group_code, .. }
            | Self::Point3d { group_code, .. }
            | Self::Binary { group_code, .. }
            | Self::Handle { group_code, .. }
            | Self::ObjectId { group_code, .. } => *group_code,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate(&self) -> Result<(), String> {
        let code = self.group_code();
        let valid = match self {
            Self::String { value, .. } => matches!(code, 0..=4 | 6..=9 | 100..=104 | 300..=309 | 410..=419 | 430..=439 | 470..=479 | 999 | 1000..=1002) && value.encode_utf16().count() <= usize::from(u16::MAX),
            Self::Real { .. } => matches!(code, 38..=59 | 140..=149 | 460..=469 | 1040..=1042),
            Self::Boolean { .. } => matches!(code, 290..=299),
            Self::Integer8 { .. } => matches!(code, 280..=289),
            Self::Integer16 { .. } => matches!(code, 60..=79 | 170..=179 | 270..=279 | 370..=389 | 400..=409 | 1070),
            Self::Integer32 { .. } => matches!(code, 90..=99 | 420..=429 | 440..=459 | 1071),
            Self::Integer64 { .. } => matches!(code, 160..=169),
            Self::Point3d { .. } => matches!(code, 10..=37 | 110..=139 | 210..=269 | 1010..=1015),
            Self::Binary { octets, .. } => matches!(code, 310..=319 | 1004) && octets.len() <= usize::from(u8::MAX),
            Self::Handle { .. } => matches!(code, 5 | 105 | 320..=329 | 390..=399 | 1003 | 1005),
            Self::ObjectId { .. } => matches!(code, 330..=369),
        };
        valid.then_some(()).ok_or_else(|| format!("XRECORD group code {code} does not match its typed value"))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDictionaryBody {
    #[serde(default)]
    pub entries: Vec<DwgNamedReference>,
    pub cloning_flag: u16,
    pub hard_owner: bool,
    #[serde(default)]
    pub default_entry_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DwgTableControlEntry {
    #[serde(default)]
    pub handle: Option<u64>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_table_control_entry_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "has_handle", dsl::Shape::Bool), dsl::FieldSpec::new(1, "handle", dsl::Shape::UInt).optional()])
}

impl dsl::DslField for DwgTableControlEntry {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_table_control_entry_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        record.fields.insert(0, dsl::FieldValue::Bool(self.handle.is_some()));
        if let Some(handle) = self.handle {
            record.fields.insert(1, <u64 as dsl::DslField>::to_value(&handle).await);
        }
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected table-control entry record".into()) };
        match record.get(0) {
            Some(dsl::FieldValue::Bool(false)) => Ok(Self { handle: None }),
            Some(dsl::FieldValue::Bool(true)) => Ok(Self { handle: Some(<u64 as dsl::DslField>::from_value(record.get(1).ok_or("table-control handle missing")?).await?) }),
            other => Err(format!("invalid table-control handle presence {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgTableControlEntries {
    #[serde(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockTableControl {
    #[serde(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    #[serde(default)]
    pub model_space_handle: Option<u64>,
    #[serde(default)]
    pub paper_space_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLinetypeTableControl {
    #[serde(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    pub by_block_handle: u64,
    pub by_layer_handle: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionStyleTableControl {
    #[serde(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    #[serde(default)]
    pub additional_handles: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgTableControlBody {
    Block(DwgBlockTableControl),
    Layer(DwgTableControlEntries),
    TextStyle(DwgTableControlEntries),
    Linetype(DwgLinetypeTableControl),
    View(DwgTableControlEntries),
    Ucs(DwgTableControlEntries),
    Viewport(DwgTableControlEntries),
    RegisteredApplication(DwgTableControlEntries),
    DimensionStyle(DwgDimensionStyleTableControl),
}

impl Default for DwgTableControlBody {
    fn default() -> Self {
        Self::Layer(DwgTableControlEntries::default())
    }
}

impl DwgTableControlBody {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn entry_handles(&self) -> &[DwgTableControlEntry] {
        match self {
            Self::Block(value) => &value.entry_handles,
            Self::Layer(value) | Self::TextStyle(value) | Self::View(value) | Self::Ucs(value) | Self::Viewport(value) | Self::RegisteredApplication(value) => &value.entry_handles,
            Self::Linetype(value) => &value.entry_handles,
            Self::DimensionStyle(value) => &value.entry_handles,
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn table_control_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                1,
                "kind",
                dsl::Shape::Enum(vec![
                    ("block".into(), 0),
                    ("layer".into(), 1),
                    ("textStyle".into(), 2),
                    ("linetype".into(), 3),
                    ("view".into(), 4),
                    ("ucs".into(), 5),
                    ("viewport".into(), 6),
                    ("registeredApplication".into(), 7),
                    ("dimensionStyle".into(), 8),
                ]),
            ),
            dsl::FieldSpec::new(2, "entries", <DwgTableControlEntries as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "block", <DwgBlockTableControl as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "linetype", <DwgLinetypeTableControl as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "dimensionStyle", <DwgDimensionStyleTableControl as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgTableControlBody {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(table_control_body_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let (kind, field, value) = match self {
            Self::Block(value) => (0, 3, <DwgBlockTableControl as dsl::DslField>::to_value(value)),
            Self::Layer(value) => (1, 2, <DwgTableControlEntries as dsl::DslField>::to_value(value)),
            Self::TextStyle(value) => (2, 2, <DwgTableControlEntries as dsl::DslField>::to_value(value)),
            Self::Linetype(value) => (3, 4, <DwgLinetypeTableControl as dsl::DslField>::to_value(value)),
            Self::View(value) => (4, 2, <DwgTableControlEntries as dsl::DslField>::to_value(value)),
            Self::Ucs(value) => (5, 2, <DwgTableControlEntries as dsl::DslField>::to_value(value)),
            Self::Viewport(value) => (6, 2, <DwgTableControlEntries as dsl::DslField>::to_value(value)),
            Self::RegisteredApplication(value) => (7, 2, <DwgTableControlEntries as dsl::DslField>::to_value(value)),
            Self::DimensionStyle(value) => (8, 5, <DwgDimensionStyleTableControl as dsl::DslField>::to_value(value)),
        };
        record.fields.insert(1, dsl::FieldValue::Enum(kind));
        record.fields.insert(field, value);
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected table-control body record".into()) };
        match record.get(1) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Block(<DwgBlockTableControl as dsl::DslField>::from_value(record.get(3).ok_or("block control missing")?).await?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::Layer(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("layer control missing")?).await?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::TextStyle(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("text-style control missing")?).await?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::Linetype(<DwgLinetypeTableControl as dsl::DslField>::from_value(record.get(4).ok_or("linetype control missing")?).await?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::View(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("view control missing")?).await?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Ucs(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("UCS control missing")?).await?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::Viewport(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("viewport control missing")?).await?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::RegisteredApplication(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("registered-application control missing")?).await?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::DimensionStyle(<DwgDimensionStyleTableControl as dsl::DslField>::from_value(record.get(5).ok_or("dimension-style control missing")?).await?)),
            other => Err(format!("unknown table-control kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgTableRecordCommon {
    pub name: String,
    pub xref_resolution: u16,
    #[serde(default)]
    pub xref_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgRegisteredApplicationTableRecord {
    pub common: DwgTableRecordCommon,
    pub group_71: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgTextStyleTableRecord {
    pub common: DwgTableRecordCommon,
    pub is_shape: bool,
    pub is_vertical: bool,
    pub text_size: f64,
    pub width_factor: f64,
    pub oblique_angle: f64,
    pub generation: u8,
    pub last_height: f64,
    pub font_file: String,
    pub big_font_file: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DwgComplexColorValue {
    #[default]
    None,
    ByLayer,
    ByBlock,
    ByColor {
        red: u8,
        green: u8,
        blue: u8,
    },
    ByAci {
        index: u16,
    },
    ByPen {
        index: u8,
    },
    Foreground,
    LayerOff,
    LayerFrozen,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_complex_color_value_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![("none".into(), 0), ("byLayer".into(), 1), ("byBlock".into(), 2), ("byColor".into(), 3), ("byAci".into(), 4), ("byPen".into(), 5), ("foreground".into(), 6), ("layerOff".into(), 7), ("layerFrozen".into(), 8)]),
            ),
            dsl::FieldSpec::new(1, "red", <u8 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "green", <u8 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "blue", <u8 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "index", <u16 as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgComplexColorValue {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_complex_color_value_spec)
    }
    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let kind = match self {
            Self::None => 0,
            Self::ByLayer => 1,
            Self::ByBlock => 2,
            Self::ByColor { red, green, blue } => {
                record.fields.insert(1, <u8 as dsl::DslField>::to_value(red).await);
                record.fields.insert(2, <u8 as dsl::DslField>::to_value(green).await);
                record.fields.insert(3, <u8 as dsl::DslField>::to_value(blue).await);
                3
            }
            Self::ByAci { index } => {
                record.fields.insert(4, <u16 as dsl::DslField>::to_value(index).await);
                4
            }
            Self::ByPen { index } => {
                record.fields.insert(4, <u16 as dsl::DslField>::to_value(&u16::from(*index)).await);
                5
            }
            Self::Foreground => 6,
            Self::LayerOff => 7,
            Self::LayerFrozen => 8,
        };
        record.fields.insert(0, dsl::FieldValue::Enum(kind));
        dsl::FieldValue::Record(record)
    }
    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected complex-color value record".into()) };
        let no_extra = |allowed: &[u16]| record.fields.iter().all(|(field, value)| *field == 0 || allowed.contains(field) || matches!(value, dsl::FieldValue::Absent));
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) if no_extra(&[]) => Ok(Self::None),
            Some(dsl::FieldValue::Enum(1)) if no_extra(&[]) => Ok(Self::ByLayer),
            Some(dsl::FieldValue::Enum(2)) if no_extra(&[]) => Ok(Self::ByBlock),
            Some(dsl::FieldValue::Enum(3)) if no_extra(&[1, 2, 3]) => Ok(Self::ByColor {
                red: <u8 as dsl::DslField>::from_value(record.get(1).ok_or("red missing")?).await?,
                green: <u8 as dsl::DslField>::from_value(record.get(2).ok_or("green missing")?).await?,
                blue: <u8 as dsl::DslField>::from_value(record.get(3).ok_or("blue missing")?).await?,
            }),
            Some(dsl::FieldValue::Enum(4)) if no_extra(&[4]) => Ok(Self::ByAci { index: <u16 as dsl::DslField>::from_value(record.get(4).ok_or("ACI index missing")?).await? }),
            Some(dsl::FieldValue::Enum(5)) if no_extra(&[4]) => Ok(Self::ByPen { index: u8::try_from(<u16 as dsl::DslField>::from_value(record.get(4).ok_or("pen index missing")?).await?).map_err(|_| "pen index exceeds u8")? }),
            Some(dsl::FieldValue::Enum(6)) if no_extra(&[]) => Ok(Self::Foreground),
            Some(dsl::FieldValue::Enum(7)) if no_extra(&[]) => Ok(Self::LayerOff),
            Some(dsl::FieldValue::Enum(8)) if no_extra(&[]) => Ok(Self::LayerFrozen),
            other => Err(format!("invalid complex-color value {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgComplexColor {
    pub index: u16,
    pub value: DwgComplexColorValue,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub book_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLayerTableRecord {
    pub common: DwgTableRecordCommon,
    pub frozen: bool,
    pub off: bool,
    pub frozen_in_new_viewports: bool,
    pub locked: bool,
    pub plottable: bool,
    pub lineweight: u8,
    pub color: DwgComplexColor,
    #[serde(default)]
    pub plot_style_handle: Option<u64>,
    #[serde(default)]
    pub material_handle: Option<u64>,
    #[serde(default)]
    pub linetype_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLinetypeDash {
    pub length: f64,
    pub complex_shape_code: u16,
    #[serde(default)]
    pub style_handle: Option<u64>,
    pub x_offset: f64,
    pub y_offset: f64,
    pub scale: f64,
    pub rotation: f64,
    pub shape_flags: u16,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLinetypeTableRecord {
    pub common: DwgTableRecordCommon,
    pub description: String,
    pub pattern_length: f64,
    pub alignment: u8,
    #[serde(default)]
    pub dashes: Vec<DwgLinetypeDash>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockHeaderTableRecord {
    pub common: DwgTableRecordCommon,
    pub anonymous: bool,
    pub has_attributes: bool,
    pub is_xref: bool,
    pub xref_overlaid: bool,
    pub xref_loaded: bool,
    #[serde(default)]
    pub owned_entity_handles: Vec<u64>,
    pub base_point: [f64; 3],
    pub xref_path: String,
    #[serde(default)]
    pub insert_backreference_handles: Vec<u64>,
    pub description: String,
    pub insert_units: u16,
    pub explodable: bool,
    pub block_scaling: u8,
    pub block_entity_handle: u64,
    pub end_block_entity_handle: u64,
    #[serde(default)]
    pub layout_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgViewportTableRecord {
    pub common: DwgTableRecordCommon,
    pub view_height: f64,
    pub view_width: f64,
    pub center: [f64; 2],
    pub target: [f64; 3],
    pub direction: [f64; 3],
    pub twist: f64,
    pub lens_length: f64,
    pub front_clipping: f64,
    pub back_clipping: f64,
    pub view_mode: [bool; 4],
    pub render_mode: u8,
    pub use_default_lights: bool,
    pub default_lighting_type: u8,
    pub brightness: f64,
    pub contrast: f64,
    pub ambient_color: DwgComplexColor,
    pub lower_left: [f64; 2],
    pub upper_right: [f64; 2],
    pub ucs_follow: bool,
    pub circle_zoom: u16,
    pub fast_zoom: bool,
    pub ucs_icon: u8,
    pub grid_mode: bool,
    pub grid_unit: [f64; 2],
    pub snap_mode: bool,
    pub snap_style: bool,
    pub snap_isopair: u16,
    pub snap_angle: f64,
    pub snap_base: [f64; 2],
    pub snap_unit: [f64; 2],
    pub ucs_at_origin: bool,
    pub ucs_viewport: bool,
    pub ucs_origin: [f64; 3],
    pub ucs_x_axis: [f64; 3],
    pub ucs_y_axis: [f64; 3],
    pub ucs_elevation: f64,
    pub ucs_orthographic_view: u16,
    pub grid_flags: u16,
    pub grid_major: u16,
    #[serde(default)]
    pub background_handle: Option<u64>,
    #[serde(default)]
    pub visual_style_handle: Option<u64>,
    #[serde(default)]
    pub sun_handle: Option<u64>,
    #[serde(default)]
    pub named_ucs_handle: Option<u64>,
    #[serde(default)]
    pub base_ucs_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionGeometry {
    pub scale: f64,
    pub arrow_size: f64,
    pub extension_origin_offset: f64,
    pub dimension_line_increment: f64,
    pub extension_line_extension: f64,
    pub rounding: f64,
    pub dimension_line_extension: f64,
    pub plus_tolerance: f64,
    pub minus_tolerance: f64,
    pub fixed_extension_length: f64,
    pub jog_angle: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionBehavior {
    pub tolerance: bool,
    pub limits: bool,
    pub text_inside_horizontal: bool,
    pub text_outside_horizontal: bool,
    pub suppress_extension_1: bool,
    pub suppress_extension_2: bool,
    pub text_vertical_alignment: u16,
    pub zero_suppression: u16,
    pub angular_zero_suppression: u16,
    pub arc_symbol: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionText {
    pub height: f64,
    pub center_mark_size: f64,
    pub tick_size: f64,
    pub alternate_scale: f64,
    pub linear_scale: f64,
    pub vertical_position: f64,
    pub tolerance_scale: f64,
    pub gap: f64,
    pub alternate_rounding: f64,
    pub alternate_enabled: bool,
    pub alternate_decimals: u16,
    pub text_outside_extensions: bool,
    pub separate_arrowheads: bool,
    pub force_text_inside: bool,
    pub suppress_outside_extensions: bool,
    pub dimension_line_color: DwgComplexColor,
    pub extension_line_color: DwgComplexColor,
    pub text_color: DwgComplexColor,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionUnits {
    pub alternate_decimal_places: u16,
    pub decimal_places: u16,
    pub tolerance_decimal_places: u16,
    pub alternate_units: u16,
    pub alternate_tolerance_decimal_places: u16,
    pub angular_units: u16,
    pub fraction_format: u16,
    pub linear_units: u16,
    pub decimal_separator: u16,
    pub text_movement: u16,
    pub text_horizontal_alignment: u16,
    pub suppress_dimension_line_1: bool,
    pub suppress_dimension_line_2: bool,
    pub tolerance_vertical_alignment: u16,
    pub tolerance_zero_suppression: u16,
    pub alternate_zero_suppression: u16,
    pub alternate_tolerance_zero_suppression: u16,
    pub user_positioned_text: bool,
    pub arrow_text_fit: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionR2010 {
    pub fixed_extension_enabled: bool,
    pub text_direction: bool,
    pub alternate_measurement_factor: f64,
    pub alternate_measurement_suffix: String,
    pub measurement_factor: f64,
    pub measurement_suffix: String,
    pub dimension_lineweight: u16,
    pub extension_lineweight: u16,
    pub flag: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionStyleTableRecord {
    pub common: DwgTableRecordCommon,
    pub dimension_postfix: String,
    pub alternate_postfix: String,
    pub geometry: DwgDimensionGeometry,
    pub fill_mode: u16,
    pub fill_color: DwgComplexColor,
    pub behavior: DwgDimensionBehavior,
    pub text: DwgDimensionText,
    pub units: DwgDimensionUnits,
    pub r2010: DwgDimensionR2010,
    pub text_style_handle: Option<u64>,
    pub leader_arrow_handle: Option<u64>,
    pub arrow_handle: Option<u64>,
    pub arrow_1_handle: Option<u64>,
    pub arrow_2_handle: Option<u64>,
    pub dimension_linetype_handle: Option<u64>,
    pub extension_1_linetype_handle: Option<u64>,
    pub extension_2_linetype_handle: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgTableRecordBody {
    RegisteredApplication(DwgRegisteredApplicationTableRecord),
    TextStyle(DwgTextStyleTableRecord),
    Layer(DwgLayerTableRecord),
    Linetype(DwgLinetypeTableRecord),
    BlockHeader(DwgBlockHeaderTableRecord),
    Viewport(DwgViewportTableRecord),
    DimensionStyle(DwgDimensionStyleTableRecord),
}

impl Default for DwgTableRecordBody {
    fn default() -> Self {
        Self::RegisteredApplication(DwgRegisteredApplicationTableRecord::default())
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn table_record_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                1,
                "kind",
                dsl::Shape::Enum(vec![("registeredApplication".into(), 0), ("textStyle".into(), 1), ("layer".into(), 2), ("linetype".into(), 3), ("blockHeader".into(), 4), ("viewport".into(), 5), ("dimensionStyle".into(), 6)]),
            ),
            dsl::FieldSpec::new(2, "registeredApplication", <DwgRegisteredApplicationTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "textStyle", <DwgTextStyleTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "layer", <DwgLayerTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "linetype", <DwgLinetypeTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "blockHeader", <DwgBlockHeaderTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "viewport", <DwgViewportTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(8, "dimensionStyle", <DwgDimensionStyleTableRecord as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgTableRecordBody {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(table_record_body_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        match self {
            Self::RegisteredApplication(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(0));
                record.fields.insert(2, <DwgRegisteredApplicationTableRecord as dsl::DslField>::to_value(value).await);
            }
            Self::TextStyle(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(1));
                record.fields.insert(3, <DwgTextStyleTableRecord as dsl::DslField>::to_value(value).await);
            }
            Self::Layer(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(2));
                record.fields.insert(4, <DwgLayerTableRecord as dsl::DslField>::to_value(value).await);
            }
            Self::Linetype(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(3));
                record.fields.insert(5, <DwgLinetypeTableRecord as dsl::DslField>::to_value(value).await);
            }
            Self::BlockHeader(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(4));
                record.fields.insert(6, <DwgBlockHeaderTableRecord as dsl::DslField>::to_value(value).await);
            }
            Self::Viewport(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(5));
                record.fields.insert(7, <DwgViewportTableRecord as dsl::DslField>::to_value(value).await);
            }
            Self::DimensionStyle(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(6));
                record.fields.insert(8, <DwgDimensionStyleTableRecord as dsl::DslField>::to_value(value).await);
            }
        }
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else {
            return Err("expected table-record body record".into());
        };
        match record.get(1) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::RegisteredApplication(<DwgRegisteredApplicationTableRecord as dsl::DslField>::from_value(record.get(2).ok_or("registered-application record missing")?).await?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::TextStyle(<DwgTextStyleTableRecord as dsl::DslField>::from_value(record.get(3).ok_or("text-style record missing")?).await?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::Layer(<DwgLayerTableRecord as dsl::DslField>::from_value(record.get(4).ok_or("layer record missing")?).await?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::Linetype(<DwgLinetypeTableRecord as dsl::DslField>::from_value(record.get(5).ok_or("linetype record missing")?).await?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::BlockHeader(<DwgBlockHeaderTableRecord as dsl::DslField>::from_value(record.get(6).ok_or("block-header record missing")?).await?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Viewport(<DwgViewportTableRecord as dsl::DslField>::from_value(record.get(7).ok_or("viewport record missing")?).await?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::DimensionStyle(<DwgDimensionStyleTableRecord as dsl::DslField>::from_value(record.get(8).ok_or("dimension-style record missing")?).await?)),
            other => Err(format!("unknown table-record kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgXRecordBody {
    #[serde(default)]
    pub values: Vec<DwgXRecordValue>,
    #[serde(default)]
    pub object_id_handles: Vec<u64>,
    pub cloning_flag: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgExtendedEntityData {
    pub application_handle: u64,
    #[serde(default)]
    pub values: Vec<DwgXRecordValue>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgEntityMode {
    ExplicitOwner,
    PaperSpace,
    #[default]
    ModelSpace,
    Reserved,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgEntityReferenceMode {
    #[default]
    ByLayer,
    ByBlock,
    Continuous,
    Explicit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgEntityColorKind {
    #[default]
    ByLayer,
    ByBlock,
    Index,
    TrueColor,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgEntityColor {
    pub kind: DwgEntityColorKind,
    pub index: u16,
    pub rgb: u32,
    #[serde(default)]
    pub transparency: Option<u32>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub book_name: Option<String>,
    #[serde(default)]
    pub color_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgEntityCommon {
    pub mode: DwgEntityMode,
    pub color: DwgEntityColor,
    pub linetype_scale: f64,
    pub linetype: DwgEntityReferenceMode,
    pub plot_style: DwgEntityReferenceMode,
    pub material: DwgEntityReferenceMode,
    pub shadow: u8,
    pub invisible: u16,
    pub lineweight: u8,
    pub layer_handle: u64,
    #[serde(default)]
    pub linetype_handle: Option<u64>,
    #[serde(default)]
    pub material_handle: Option<u64>,
    #[serde(default)]
    pub shadow_handle: Option<u64>,
    #[serde(default)]
    pub plot_style_handle: Option<u64>,
    #[serde(default)]
    pub full_visual_style_handle: Option<u64>,
    #[serde(default)]
    pub face_visual_style_handle: Option<u64>,
    #[serde(default)]
    pub edge_visual_style_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLineEntity {
    pub common: DwgEntityCommon,
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgArcEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub radius: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    pub start_angle: f64,
    pub end_angle: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLwPolylineVertex {
    pub point: Vec<f64>,
    pub bulge: f64,
    #[serde(default)]
    pub vertex_id: Option<u32>,
    #[serde(default)]
    pub start_width: Option<f64>,
    #[serde(default)]
    pub end_width: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLwPolylineEntity {
    pub common: DwgEntityCommon,
    pub closed: bool,
    #[serde(default)]
    pub constant_width: Option<f64>,
    pub elevation: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    #[serde(default)]
    pub vertices: Vec<DwgLwPolylineVertex>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockBeginEntity {
    pub common: DwgEntityCommon,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockEndEntity {
    pub common: DwgEntityCommon,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgInsertEntity {
    pub common: DwgEntityCommon,
    pub insertion: Vec<f64>,
    pub scale: Vec<f64>,
    pub rotation: f64,
    pub extrusion: Vec<f64>,
    pub block_header_handle: u64,
    #[serde(default)]
    pub attribute_handles: Vec<u64>,
    #[serde(default)]
    pub sequence_end_handle: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgDimensionTextAttachment {
    #[default]
    TopCenter,
    TopLeft,
    TopRight,
    MiddleCenter,
    MiddleLeft,
    MiddleRight,
    BottomCenter,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgDimensionLineSpacingStyle {
    #[default]
    AtLeast,
    Exact,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionStatus {
    pub block_reference_is_exclusive: bool,
    pub user_positioned_text: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionEntityCommon {
    pub common: DwgEntityCommon,
    pub extrusion: Vec<f64>,
    pub text_midpoint: Vec<f64>,
    pub elevation: f64,
    pub status: DwgDimensionStatus,
    pub user_text: String,
    pub text_rotation: f64,
    pub horizontal_direction: f64,
    pub insertion_scale: Vec<f64>,
    pub insertion_rotation: f64,
    pub attachment: DwgDimensionTextAttachment,
    pub line_spacing_style: DwgDimensionLineSpacingStyle,
    pub line_spacing_factor: f64,
    pub actual_measurement: f64,
    pub flip_arrow_1: bool,
    pub flip_arrow_2: bool,
    pub clone_insertion_point: Vec<f64>,
    pub dimension_style_handle: u64,
    #[serde(default)]
    pub dimension_block_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLinearDimensionEntity {
    pub dimension: DwgDimensionEntityCommon,
    pub extension_line_1: Vec<f64>,
    pub extension_line_2: Vec<f64>,
    pub definition_point: Vec<f64>,
    pub oblique_angle: f64,
    pub dimension_rotation: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgViewportStatusFlag {
    Perspective,
    FrontClipping,
    BackClipping,
    UcsFollow,
    FrontClipNotAtEye,
    UcsIconVisible,
    UcsIconAtOrigin,
    FastZoom,
    Snap,
    Grid,
    IsometricSnap,
    HidePlot,
    IsoPairTop,
    IsoPairRight,
    ZoomLock,
    AlwaysEnabled,
    NonRectangularClipping,
    ViewportOff,
    GridBeyondDrawingLimits,
    AdaptiveGrid,
    AdaptiveSubdivision,
    GridFollowsWorkplane,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgViewportRenderMode {
    #[default]
    Optimized2d,
    Wireframe,
    HiddenLine,
    FlatShaded,
    GouraudShaded,
    FlatShadedWithWireframe,
    GouraudShadedWithWireframe,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgOrthographicView {
    #[default]
    None,
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgShadePlotMode {
    #[default]
    AsDisplayed,
    Wireframe,
    Hidden,
    Rendered,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgDefaultLightingType {
    OneDistantLight,
    #[default]
    TwoDistantLights,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgViewportEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub width: f64,
    pub height: f64,
    pub view_target: Vec<f64>,
    pub view_direction: Vec<f64>,
    pub twist_angle: f64,
    pub view_height: f64,
    pub lens_length: f64,
    pub front_clip: f64,
    pub back_clip: f64,
    pub snap_angle: f64,
    pub view_center: Vec<f64>,
    pub snap_base: Vec<f64>,
    pub snap_unit: Vec<f64>,
    pub grid_unit: Vec<f64>,
    pub circle_zoom_percent: u16,
    pub grid_major: u16,
    pub frozen_layer_handles: Vec<u64>,
    pub status: Vec<DwgViewportStatusFlag>,
    pub style_sheet: String,
    pub render_mode: DwgViewportRenderMode,
    pub ucs_at_origin: bool,
    pub ucs_per_viewport: bool,
    pub ucs_origin: Vec<f64>,
    pub ucs_x_axis: Vec<f64>,
    pub ucs_y_axis: Vec<f64>,
    pub ucs_elevation: f64,
    pub orthographic_view: DwgOrthographicView,
    pub shade_plot_mode: DwgShadePlotMode,
    pub use_default_lights: bool,
    pub default_lighting_type: DwgDefaultLightingType,
    pub brightness: f64,
    pub contrast: f64,
    pub ambient_color: DwgComplexColor,
    pub clip_boundary_handle: Option<u64>,
    pub named_ucs_handle: Option<u64>,
    pub base_ucs_handle: Option<u64>,
    pub background_handle: Option<u64>,
    pub visual_style_handle: Option<u64>,
    pub shade_plot_handle: Option<u64>,
    pub sun_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgGeometryEntity {
    pub common: DwgEntityCommon,
    pub geometry: DwgLogicalGeometry,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEntityBody {
    Line(DwgLineEntity),
    Arc(DwgArcEntity),
    LwPolyline(DwgLwPolylineEntity),
    BlockBegin(DwgBlockBeginEntity),
    BlockEnd(DwgBlockEndEntity),
    Insert(DwgInsertEntity),
    DimensionLinear(DwgLinearDimensionEntity),
    Viewport(DwgViewportEntity),
    Geometry(DwgGeometryEntity),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgAssociativeDependencyStatus {
    #[default]
    UpToDate,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssociativeDependency {
    pub status: DwgAssociativeDependencyStatus,
    pub is_read_dependency: bool,
    pub is_write_dependency: bool,
    pub is_attached_to_object: bool,
    pub is_delegating_to_owning_action: bool,
    pub order: i32,
    pub dependent_on_object_handle: u64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub read_dependency_handle: Option<u64>,
    #[serde(default)]
    pub dependency_node_handle: Option<u64>,
    #[serde(default)]
    pub dependency_body_handle: Option<u64>,
    pub dependency_body_id: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEvaluationVariant {
    Integer32(i32),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_evaluation_variant_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "kind", dsl::Shape::Enum(vec![("integer32".into(), 0)])), dsl::FieldSpec::new(1, "integer32", <i32 as dsl::DslField>::shape()).optional()])
}

impl dsl::DslField for DwgEvaluationVariant {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_evaluation_variant_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        match self {
            Self::Integer32(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(0));
                record.fields.insert(1, <i32 as dsl::DslField>::to_value(value).await);
            }
        }
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected evaluation-variant record".into()) };
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Integer32(<i32 as dsl::DslField>::from_value(record.get(1).ok_or("evaluation integer32 missing")?).await?)),
            other => Err(format!("unknown evaluation-variant kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssociativeValueDependency {
    pub dependency: DwgAssociativeDependency,
    pub cached_value: DwgEvaluationVariant,
    pub value_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssociativeGeometryDependency {
    pub dependency: DwgAssociativeDependency,
    pub enabled: bool,
    pub persistent_subentity_class_name: String,
    pub dependent_on_compound_object: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEvaluationExpressionValue {
    Empty,
    Double(f64),
    PointGroup10(Vec<f64>),
    PointGroup11(Vec<f64>),
    String(String),
    Integer32(i32),
    ObjectReference(u64),
    Integer16(i16),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_evaluation_expression_value_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![("empty".into(), 0), ("double".into(), 1), ("pointGroup10".into(), 2), ("pointGroup11".into(), 3), ("string".into(), 4), ("integer32".into(), 5), ("objectReference".into(), 6), ("integer16".into(), 7)]),
            ),
            dsl::FieldSpec::new(1, "double", <f64 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "point_group_10", <Vec<f64> as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "point_group_11", <Vec<f64> as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "string", <String as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "integer32", <i32 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "object_reference", <u64 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "integer16", <i16 as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgEvaluationExpressionValue {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_evaluation_expression_value_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let (kind, field, value) = match self {
            Self::Empty => (0, None, None),
            Self::Double(value) => (1, Some(1), Some(<f64 as dsl::DslField>::to_value(value))),
            Self::PointGroup10(value) => (2, Some(2), Some(<Vec<f64> as dsl::DslField>::to_value(value))),
            Self::PointGroup11(value) => (3, Some(3), Some(<Vec<f64> as dsl::DslField>::to_value(value))),
            Self::String(value) => (4, Some(4), Some(<String as dsl::DslField>::to_value(value))),
            Self::Integer32(value) => (5, Some(5), Some(<i32 as dsl::DslField>::to_value(value))),
            Self::ObjectReference(value) => (6, Some(6), Some(<u64 as dsl::DslField>::to_value(value))),
            Self::Integer16(value) => (7, Some(7), Some(<i16 as dsl::DslField>::to_value(value))),
        };
        record.fields.insert(0, dsl::FieldValue::Enum(kind));
        if let (Some(field), Some(value)) = (field, value) {
            record.fields.insert(field, value);
        }
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected evaluation-expression value record".into()) };
        let present = record.fields.values().filter(|value| !matches!(value, dsl::FieldValue::Absent)).count();
        let expected = if matches!(record.get(0), Some(dsl::FieldValue::Enum(0))) { 1 } else { 2 };
        if present != expected {
            return Err("evaluation-expression value must contain exactly its tagged payload".into());
        }
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Empty),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::Double(<f64 as dsl::DslField>::from_value(record.get(1).ok_or("evaluation-expression double missing")?).await?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::PointGroup10(<Vec<f64> as dsl::DslField>::from_value(record.get(2).ok_or("evaluation-expression group-10 point missing")?).await?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::PointGroup11(<Vec<f64> as dsl::DslField>::from_value(record.get(3).ok_or("evaluation-expression group-11 point missing")?).await?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::String(<String as dsl::DslField>::from_value(record.get(4).ok_or("evaluation-expression string missing")?).await?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Integer32(<i32 as dsl::DslField>::from_value(record.get(5).ok_or("evaluation-expression integer32 missing")?).await?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::ObjectReference(<u64 as dsl::DslField>::from_value(record.get(6).ok_or("evaluation-expression object reference missing")?).await?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::Integer16(<i16 as dsl::DslField>::from_value(record.get(7).ok_or("evaluation-expression integer16 missing")?).await?)),
            other => Err(format!("unknown evaluation-expression value kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgEvaluationExpression {
    pub parent_id: i32,
    pub major_version: u32,
    pub minor_version: u32,
    pub value: DwgEvaluationExpressionValue,
    pub node_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockGripLocationComponent {
    pub evaluation_expression: DwgEvaluationExpression,
    pub grip_type: u32,
    pub grip_expression: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDynamicBlockProxyNode {
    pub evaluation_expression: DwgEvaluationExpression,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgAssociativeActionStatus {
    #[default]
    UpToDate,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssociativeActionDependency {
    pub owned: bool,
    pub dependency_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssociativeAction {
    pub status: DwgAssociativeActionStatus,
    #[serde(default)]
    pub owning_network_handle: Option<u64>,
    #[serde(default)]
    pub action_body_handle: Option<u64>,
    pub action_index: i32,
    pub maximum_dependency_index: i32,
    pub dependencies: Vec<DwgAssociativeActionDependency>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssociativeVariable {
    pub action: DwgAssociativeAction,
    pub name: String,
    pub expression: String,
    pub evaluator_id: String,
    pub description: String,
    pub evaluated_value: DwgEvaluationVariant,
    pub mergeable: bool,
    #[serde(default)]
    pub mergeable_variable_name: Option<String>,
    pub must_merge: bool,
    pub referenced_value_dependency_handles: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssociativeDimensionDependencyBody {
    pub name: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgVisualStylePropertyOperation {
    Inherit,
    #[default]
    Set,
    Disable,
    Enable,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DwgVisualStyleProperty<T> {
    pub value: T,
    pub operation: DwgVisualStylePropertyOperation,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_visual_style_property_spec<T: dsl::DslField>() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "value", T::shape()), dsl::FieldSpec::new(1, "operation", <DwgVisualStylePropertyOperation as dsl::DslField>::shape())])
}

impl<T: dsl::DslField> dsl::DslField for DwgVisualStyleProperty<T> {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_visual_style_property_spec::<T>)
    }
    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        record.fields.insert(0, T::to_value(&self.value).await);
        record.fields.insert(1, <DwgVisualStylePropertyOperation as dsl::DslField>::to_value(&self.operation).await);
        dsl::FieldValue::Record(record)
    }
    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected visual-style property record".into()) };
        Ok(Self { value: T::from_value(record.get(0).ok_or("visual-style property value missing")?).await?, operation: <DwgVisualStylePropertyOperation as dsl::DslField>::from_value(record.get(1).ok_or("visual-style property operation missing")?).await? })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgVisualStyleProperties {
    pub face_lighting_model: DwgVisualStyleProperty<u32>,
    pub face_lighting_quality: DwgVisualStyleProperty<u32>,
    pub face_color_mode: DwgVisualStyleProperty<u32>,
    pub face_modifiers: DwgVisualStyleProperty<u16>,
    pub face_opacity: DwgVisualStyleProperty<f64>,
    pub face_specular_amount: DwgVisualStyleProperty<f64>,
    pub face_monochrome_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_model: DwgVisualStyleProperty<u32>,
    pub edge_styles: DwgVisualStyleProperty<u32>,
    pub edge_intersection_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_obscured_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_obscured_line_pattern: DwgVisualStyleProperty<u32>,
    pub edge_intersection_line_pattern: DwgVisualStyleProperty<u32>,
    pub edge_crease_angle: DwgVisualStyleProperty<f64>,
    pub edge_modifiers: DwgVisualStyleProperty<u32>,
    pub edge_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_opacity: DwgVisualStyleProperty<f64>,
    pub edge_width: DwgVisualStyleProperty<u32>,
    pub edge_overhang: DwgVisualStyleProperty<u32>,
    pub edge_jitter: DwgVisualStyleProperty<u32>,
    pub edge_silhouette_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_silhouette_width: DwgVisualStyleProperty<u32>,
    pub edge_halo_gap: DwgVisualStyleProperty<u32>,
    pub edge_isolines: DwgVisualStyleProperty<u32>,
    pub hidden_edge_precision: DwgVisualStyleProperty<bool>,
    pub display_settings: DwgVisualStyleProperty<u32>,
    pub display_brightness: DwgVisualStyleProperty<f64>,
    pub display_shadow_type: DwgVisualStyleProperty<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgVisualStyle {
    pub description: String,
    pub style_type: u32,
    pub extension_lighting_model: u16,
    pub internal_only: bool,
    pub properties: DwgVisualStyleProperties,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockParameterDependencyBody {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockRepresentationData {
    pub represented_block_header_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDynamicBlockPurgePreventer {
    pub protected_block_header_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgEvaluationGraphNode {
    pub id: u32,
    pub expression_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgEvaluationGraphEdge {
    pub from_node_id: u32,
    pub to_node_id: u32,
    pub reference_count: u32,
    pub invertible: bool,
    pub suppressed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgEvaluationGraph {
    pub nodes: Vec<DwgEvaluationGraphNode>,
    pub edges: Vec<DwgEvaluationGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockParameterConnection {
    pub code: u32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockParameterProperty {
    pub connections: Vec<DwgBlockParameterConnection>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgBlockParameterBaseLocation {
    #[default]
    StartPoint,
    Midpoint,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockFlipValueSet {
    pub base_label: String,
    pub flipped_label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgNamedEvaluationNodeReference {
    pub node_id: u32,
    pub expression_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockFlipParameter {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_base: Vec<f64>,
    pub definition_end: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
    pub base_location: DwgBlockParameterBaseLocation,
    pub label: String,
    pub description: String,
    pub value_set: DwgBlockFlipValueSet,
    pub label_point: Vec<f64>,
    pub updated_flip: DwgNamedEvaluationNodeReference,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgVisibilityEvaluationHistory {
    #[default]
    Stateless,
    Required,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgVisibilityState {
    pub name: String,
    pub visible_entity_handles: Vec<u64>,
    pub controlled_expression_handles: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockVisibilityParameter {
    pub evaluation_expression: DwgEvaluationExpression,
    pub element_name: String,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_point: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
    pub updated_visibility_node_id: u32,
    pub initialized: bool,
    pub name: String,
    pub description: String,
    pub evaluation_history: DwgVisibilityEvaluationHistory,
    pub eligible_entity_handles: Vec<u64>,
    pub states: Vec<DwgVisibilityState>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockElement {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockGrip {
    pub element: DwgBlockElement,
    pub location: Vec<f64>,
    pub insertion_cycling: bool,
    pub insertion_cycling_weight: i32,
    pub updated_x: DwgNamedEvaluationNodeReference,
    pub updated_y: DwgNamedEvaluationNodeReference,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgPropertyExpressionReference {
    pub property_index: u32,
    pub node_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockTwoPointParameter {
    pub element: DwgBlockElement,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_base: Vec<f64>,
    pub definition_end: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
    pub property_expression_references: Vec<DwgPropertyExpressionReference>,
    pub base_location: DwgBlockParameterBaseLocation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockLinearParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub distance_name: String,
    pub distance_description: String,
    pub label_offset: f64,
    pub allowed_values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockLinearGrip {
    pub grip: DwgBlockGrip,
    pub orientation: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockFlipGrip {
    pub grip: DwgBlockGrip,
    pub updated_flip: DwgNamedEvaluationNodeReference,
    pub orientation: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockVisibilityGrip {
    pub grip: DwgBlockGrip,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgPlaceholder {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDictionaryVariable {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAnnotationScale {
    pub name: String,
    pub paper_units: f64,
    pub drawing_units: f64,
    pub is_unit_scale: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDrawOrderEntry {
    pub entity_handle: u64,
    pub sort_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSortEntitiesTable {
    pub block_header_handle: u64,
    pub entries: Vec<DwgDrawOrderEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgCellContentFormat {
    pub property_override_flags: u32,
    pub property_flags: u32,
    pub value_data_type: u32,
    pub value_unit_type: u32,
    pub value_format_string: String,
    pub rotation: f64,
    pub block_scale: f64,
    pub alignment: u32,
    pub content_color: DwgComplexColor,
    pub text_style_handle: Option<u64>,
    pub text_height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgCellMargins {
    pub vertical: f64,
    pub horizontal: f64,
    pub bottom: f64,
    pub right: f64,
    pub horizontal_spacing: f64,
    pub vertical_spacing: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgCellBorder {
    pub override_flags: u32,
    pub border_type: u32,
    pub color: DwgComplexColor,
    pub lineweight: i32,
    pub linetype_handle: Option<u64>,
    pub visible: u32,
    pub double_line_spacing: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgCellBorders {
    pub top: Option<DwgCellBorder>,
    pub horizontal_inside: Option<DwgCellBorder>,
    pub bottom: Option<DwgCellBorder>,
    pub left: Option<DwgCellBorder>,
    pub vertical_inside: Option<DwgCellBorder>,
    pub right: Option<DwgCellBorder>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgCellStyle {
    pub property_override_flags: u32,
    pub merge_flags: u32,
    pub background_color: DwgComplexColor,
    pub content_layout: u32,
    pub content_format: DwgCellContentFormat,
    pub margins: DwgCellMargins,
    pub borders: DwgCellBorders,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgTableStyle {
    pub description: String,
    pub bit_flags: u32,
    pub template_style_handle: Option<u64>,
    pub table: DwgCellStyle,
    pub title: DwgCellStyle,
    pub header: DwgCellStyle,
    pub data: DwgCellStyle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMlineLinetype {
    #[default]
    ByLayer,
    ByBlock,
    Continuous,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMlineCaps {
    pub square: bool,
    pub inner_arcs: bool,
    pub round_outer_arcs: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMlineStyleElement {
    pub offset: f64,
    pub color: DwgComplexColor,
    pub linetype: DwgMlineLinetype,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMlineStyle {
    pub name: String,
    pub description: String,
    pub fill_enabled: bool,
    pub display_miters: bool,
    pub start_caps: DwgMlineCaps,
    pub end_caps: DwgMlineCaps,
    pub fill_color: DwgComplexColor,
    pub start_angle: f64,
    pub end_angle: f64,
    pub elements: Vec<DwgMlineStyleElement>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderContentType {
    None,
    Block,
    #[default]
    MText,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderDrawOrder {
    #[default]
    LeaderFirst,
    ContentFirst,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderLeaderOrder {
    #[default]
    HeadFirst,
    TailFirst,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderKind {
    Invisible,
    #[default]
    Straight,
    Spline,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderTextAttachment {
    TopOfTop,
    #[default]
    MiddleOfTop,
    Middle,
    MiddleOfBottom,
    BottomOfBottom,
    BottomLine,
    BottomOfTop,
    BottomOfTopUnderline,
    BottomOfTopNoUnderline,
    Center,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderTextAngle {
    #[default]
    Horizontal,
    Aligned,
    AlwaysRightReading,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderTextAlignment {
    #[default]
    Left,
    Center,
    Right,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderAttachmentDirection {
    #[default]
    Horizontal,
    Vertical,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMLeaderBlockConnection {
    #[default]
    Extents,
    BasePoint,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMLeaderLeaderStyle {
    pub kind: DwgMLeaderKind,
    pub color: DwgComplexColor,
    pub linetype_style_handle: u64,
    pub lineweight: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMLeaderLanding {
    pub enabled: bool,
    pub gap: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMLeaderDogleg {
    pub enabled: bool,
    pub length: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMLeaderArrow {
    pub symbol_handle: Option<u64>,
    pub size: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMLeaderTextStyle {
    pub default_content: String,
    pub style_handle: u64,
    pub left_attachment: DwgMLeaderTextAttachment,
    pub right_attachment: DwgMLeaderTextAttachment,
    pub angle: DwgMLeaderTextAngle,
    pub alignment: DwgMLeaderTextAlignment,
    pub color: DwgComplexColor,
    pub height: f64,
    pub frame: bool,
    pub always_left: bool,
    pub alignment_space: f64,
    pub attachment_direction: DwgMLeaderAttachmentDirection,
    pub top_attachment: DwgMLeaderTextAttachment,
    pub bottom_attachment: DwgMLeaderTextAttachment,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMLeaderBlockStyle {
    pub content_handle: Option<u64>,
    pub color: DwgComplexColor,
    pub scale: Vec<f64>,
    pub use_scale: bool,
    pub rotation: f64,
    pub use_rotation: bool,
    pub connection: DwgMLeaderBlockConnection,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMLeaderStyle {
    pub content_type: DwgMLeaderContentType,
    pub draw_order: DwgMLeaderDrawOrder,
    pub leader_order: DwgMLeaderLeaderOrder,
    pub maximum_segment_points: u32,
    pub first_segment_angle: f64,
    pub second_segment_angle: f64,
    pub leader: DwgMLeaderLeaderStyle,
    pub landing: DwgMLeaderLanding,
    pub dogleg: DwgMLeaderDogleg,
    pub description: String,
    pub arrow: DwgMLeaderArrow,
    pub text: DwgMLeaderTextStyle,
    pub block: DwgMLeaderBlockStyle,
    pub overall_scale: f64,
    pub property_overrides_changed: bool,
    pub annotative: bool,
    pub break_size: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMaterialProjection {
    Inherit,
    Planar,
    #[default]
    Box,
    Cylinder,
    Sphere,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMaterialTiling {
    Inherit,
    #[default]
    Tile,
    Crop,
    Clamp,
    Mirror,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMaterialMapSource {
    #[default]
    None,
    CurrentScene,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMaterialColor {
    pub factor: f64,
    pub override_rgb: Option<u32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMaterialMap {
    pub blend_factor: f64,
    pub projection: DwgMaterialProjection,
    pub tiling: DwgMaterialTiling,
    pub scale_to_entity: bool,
    pub use_current_block_transform: bool,
    pub transform: Vec<f64>,
    pub source: DwgMaterialMapSource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMaterialChannels {
    pub diffuse: bool,
    pub specular: bool,
    pub reflection: bool,
    pub opacity: bool,
    pub bump: bool,
    pub refraction: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgMaterial {
    pub name: String,
    pub description: String,
    pub ambient: DwgMaterialColor,
    pub diffuse: DwgMaterialColor,
    pub diffuse_map: DwgMaterialMap,
    pub specular: DwgMaterialColor,
    pub specular_map: DwgMaterialMap,
    pub specular_gloss: f64,
    pub reflection_map: DwgMaterialMap,
    pub opacity: f64,
    pub opacity_map: DwgMaterialMap,
    pub bump_map: DwgMaterialMap,
    pub refraction_index: f64,
    pub refraction_map: DwgMaterialMap,
    pub translucence: f64,
    pub self_illumination: f64,
    pub reflectivity: f64,
    pub enabled_channels: DwgMaterialChannels,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockActionConnection {
    pub node_id: u32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockActionDependency {
    pub object_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockAction {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
    pub display_location: Vec<f64>,
    pub dependencies: Vec<DwgBlockActionDependency>,
    pub action_node_ids: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgBlockMoveCoordinateMode {
    #[default]
    CartesianXy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockMoveAction {
    pub action: DwgBlockAction,
    pub x_connection: DwgBlockActionConnection,
    pub y_connection: DwgBlockActionConnection,
    pub distance_multiplier: f64,
    pub angle_offset: f64,
    pub coordinate_mode: DwgBlockMoveCoordinateMode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockAlignmentParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub updated_grip_node_id: u32,
    pub align_perpendicular: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockAlignmentGrip {
    pub grip: DwgBlockGrip,
    pub first_location_node_id: u32,
    pub second_location_node_id: u32,
    pub orientation: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgStretchSelection {
    pub object_handle: u64,
    pub vertex_indices: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgStretchSelector {
    pub node_id: u32,
    pub point_indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgBlockActionCoordinateMode {
    #[default]
    CartesianXy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockStretchAction {
    pub action: DwgBlockAction,
    pub x_connection: DwgBlockActionConnection,
    pub y_connection: DwgBlockActionConnection,
    pub points: Vec<Vec<f64>>,
    pub selections: Vec<DwgStretchSelection>,
    pub selectors: Vec<DwgStretchSelector>,
    pub distance_multiplier: f64,
    pub angle_offset: f64,
    pub coordinate_mode: DwgBlockActionCoordinateMode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockActionWithBasePoint {
    pub action: DwgBlockAction,
    pub offset: Vec<f64>,
    pub x_base_connection: DwgBlockActionConnection,
    pub y_base_connection: DwgBlockActionConnection,
    pub dependent: bool,
    pub base_point: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgBlockScaleMode {
    #[default]
    Xy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockScaleAction {
    pub base: DwgBlockActionWithBasePoint,
    pub uniform_scale_connection: DwgBlockActionConnection,
    pub x_scale_connection: DwgBlockActionConnection,
    pub y_scale_connection: DwgBlockActionConnection,
    pub mode: DwgBlockScaleMode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockFlipAction {
    pub action: DwgBlockAction,
    pub flip_connection: DwgBlockActionConnection,
    pub updated_flip_connection: DwgBlockActionConnection,
    pub updated_base_connection: DwgBlockActionConnection,
    pub updated_end_connection: DwgBlockActionConnection,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockOnePointParameter {
    pub element: DwgBlockElement,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_point: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockBasePointParameter {
    pub parameter: DwgBlockOnePointParameter,
    pub point: Vec<f64>,
    pub base_point: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockParameterAllowedValues {
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgBlockLinearConstraintParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub displacement_grip_node_id: u32,
    pub dependency_handle: u64,
    pub expression_name: String,
    pub expression_description: String,
    pub value: f64,
    pub allowed_values: DwgBlockParameterAllowedValues,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgPlotOptions {
    pub use_standard_scale: bool,
    pub plot_viewport_borders: bool,
    pub plot_with_lineweights: bool,
    pub draw_viewports_first: bool,
    pub model_type: bool,
    pub update_paper: bool,
    pub initializing: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgPlotPaperUnit {
    #[default]
    Inches,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgPlotRotation {
    #[default]
    QuarterTurn,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgPlotArea {
    #[default]
    Display,
    Layout,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgStandardScale {
    #[default]
    Custom,
    OneToOne,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgShadePlot {
    #[default]
    AsDisplayed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgShadePlotResolution {
    #[default]
    Normal,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLayoutOptions {
    pub paper_space_linetype_scaling: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLayout {
    pub page_setup_name: String,
    pub printer_configuration: String,
    pub canonical_media_name: String,
    pub stylesheet: String,
    pub name: String,
    pub plot_options: DwgPlotOptions,
    pub margins: Vec<f64>,
    pub paper_size: Vec<f64>,
    pub plot_origin: Vec<f64>,
    pub paper_unit: DwgPlotPaperUnit,
    pub rotation: DwgPlotRotation,
    pub plot_area: DwgPlotArea,
    pub plot_window_lower_left: Vec<f64>,
    pub plot_window_upper_right: Vec<f64>,
    pub paper_units: f64,
    pub drawing_units: f64,
    pub standard_scale: DwgStandardScale,
    pub standard_scale_factor: f64,
    pub paper_image_origin: Vec<f64>,
    pub shade_plot: DwgShadePlot,
    pub shade_plot_resolution: DwgShadePlotResolution,
    pub shade_plot_dpi: u16,
    pub tab_order: u16,
    pub options: DwgLayoutOptions,
    pub insertion_base: Vec<f64>,
    pub limits_minimum: Vec<f64>,
    pub limits_maximum: Vec<f64>,
    pub ucs_origin: Vec<f64>,
    pub ucs_x_axis: Vec<f64>,
    pub ucs_y_axis: Vec<f64>,
    pub ucs_elevation: f64,
    pub orthographic_view: DwgOrthographicView,
    pub extents_minimum: Vec<f64>,
    pub extents_maximum: Vec<f64>,
    pub plot_view_handle: Option<u64>,
    pub visual_style_handle: Option<u64>,
    pub block_header_handle: u64,
    pub active_viewport_handle: Option<u64>,
    pub base_ucs_handle: Option<u64>,
    pub named_ucs_handle: Option<u64>,
    pub viewport_handles: Vec<u64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgAssocNetworkMemberKind {
    Network,
    #[default]
    Action,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssocNetworkMember {
    pub handle: u64,
    pub kind: DwgAssocNetworkMemberKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssocNetwork {
    pub action: DwgAssociativeAction,
    pub network_action_index: i32,
    pub actions: Vec<DwgAssocNetworkMember>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgConstraintNodeCore {
    pub id: i32,
    pub connected_node_ids: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgGeometricConstraint {
    pub node: DwgConstraintNodeCore,
    pub owner_node_id: u32,
    pub implied: bool,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgConstraintGeometry {
    pub node: DwgConstraintNodeCore,
    pub geometry_dependency_handle: Option<u64>,
    pub geometry_node_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgExplicitConstraint {
    pub geometric: DwgGeometricConstraint,
    pub value_dependency_handle: u64,
    pub dimension_dependency_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgConstrainedImplicitPoint {
    pub geometry: DwgConstraintGeometry,
    pub point: Option<Vec<f64>>,
    pub point_kind: u8,
    pub point_index: i32,
    pub curve_node_id: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgConstrainedBoundedLine {
    pub geometry: DwgConstraintGeometry,
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
    pub ray: bool,
    pub bounded: bool,
    pub start_point: Vec<f64>,
    pub end_point: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDistanceConstraint {
    pub explicit: DwgExplicitConstraint,
    pub direction_kind: u8,
    pub direction: Option<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAxisConstraint {
    pub geometric: DwgGeometricConstraint,
    pub datum_line_index: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgConstrainedDatumLine {
    pub geometry: DwgConstraintGeometry,
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgConstraintNode {
    ConstrainedImplicitPoint(DwgConstrainedImplicitPoint),
    PointCurveConstraint(DwgGeometricConstraint),
    ConstrainedBoundedLine(DwgConstrainedBoundedLine),
    PointCoincidenceConstraint(DwgGeometricConstraint),
    DistanceConstraint(DwgDistanceConstraint),
    PerpendicularConstraint(DwgGeometricConstraint),
    HorizontalConstraint(DwgAxisConstraint),
    ParallelConstraint(DwgGeometricConstraint),
    MidPointConstraint(DwgGeometricConstraint),
    EqualLengthConstraint(DwgGeometricConstraint),
    ColinearConstraint(DwgGeometricConstraint),
    ConstrainedDatumLine(DwgConstrainedDatumLine),
    FixedConstraint(DwgGeometricConstraint),
    VerticalConstraint(DwgAxisConstraint),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_constraint_node_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("constrainedImplicitPoint".into(), 0),
                    ("pointCurveConstraint".into(), 1),
                    ("constrainedBoundedLine".into(), 2),
                    ("pointCoincidenceConstraint".into(), 3),
                    ("distanceConstraint".into(), 4),
                    ("perpendicularConstraint".into(), 5),
                    ("horizontalConstraint".into(), 6),
                    ("parallelConstraint".into(), 7),
                    ("midPointConstraint".into(), 8),
                    ("equalLengthConstraint".into(), 9),
                    ("colinearConstraint".into(), 10),
                    ("constrainedDatumLine".into(), 11),
                    ("fixedConstraint".into(), 12),
                    ("verticalConstraint".into(), 13),
                ]),
            ),
            dsl::FieldSpec::new(1, "constrained_implicit_point", <DwgConstrainedImplicitPoint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "geometric_constraint", <DwgGeometricConstraint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "constrained_bounded_line", <DwgConstrainedBoundedLine as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "distance_constraint", <DwgDistanceConstraint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "axis_constraint", <DwgAxisConstraint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "constrained_datum_line", <DwgConstrainedDatumLine as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgConstraintNode {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_constraint_node_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let (kind, field, value) = match self {
            Self::ConstrainedImplicitPoint(value) => (0, 1, value.to_value()),
            Self::PointCurveConstraint(value) => (1, 2, value.to_value()),
            Self::ConstrainedBoundedLine(value) => (2, 3, value.to_value()),
            Self::PointCoincidenceConstraint(value) => (3, 2, value.to_value()),
            Self::DistanceConstraint(value) => (4, 4, value.to_value()),
            Self::PerpendicularConstraint(value) => (5, 2, value.to_value()),
            Self::HorizontalConstraint(value) => (6, 5, value.to_value()),
            Self::ParallelConstraint(value) => (7, 2, value.to_value()),
            Self::MidPointConstraint(value) => (8, 2, value.to_value()),
            Self::EqualLengthConstraint(value) => (9, 2, value.to_value()),
            Self::ColinearConstraint(value) => (10, 2, value.to_value()),
            Self::ConstrainedDatumLine(value) => (11, 6, value.to_value()),
            Self::FixedConstraint(value) => (12, 2, value.to_value()),
            Self::VerticalConstraint(value) => (13, 5, value.to_value()),
        };
        record.fields.insert(0, dsl::FieldValue::Enum(kind));
        record.fields.insert(field, value);
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected constraint-node record".into()) };
        if record.fields.values().filter(|value| !matches!(value, dsl::FieldValue::Absent)).count() != 2 {
            return Err("constraint node must contain exactly its tagged body".into());
        }
        let geometric = || DwgGeometricConstraint::from_value(record.get(2).ok_or("geometric constraint missing")?);
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::ConstrainedImplicitPoint(DwgConstrainedImplicitPoint::from_value(record.get(1).ok_or("implicit point missing")?).await?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::PointCurveConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::ConstrainedBoundedLine(DwgConstrainedBoundedLine::from_value(record.get(3).ok_or("bounded line missing")?).await?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::PointCoincidenceConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::DistanceConstraint(DwgDistanceConstraint::from_value(record.get(4).ok_or("distance constraint missing")?).await?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::PerpendicularConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::HorizontalConstraint(DwgAxisConstraint::from_value(record.get(5).ok_or("horizontal constraint missing")?).await?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::ParallelConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::MidPointConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(9)) => Ok(Self::EqualLengthConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(10)) => Ok(Self::ColinearConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(11)) => Ok(Self::ConstrainedDatumLine(DwgConstrainedDatumLine::from_value(record.get(6).ok_or("datum line missing")?).await?)),
            Some(dsl::FieldValue::Enum(12)) => Ok(Self::FixedConstraint(geometric().await?)),
            Some(dsl::FieldValue::Enum(13)) => Ok(Self::VerticalConstraint(DwgAxisConstraint::from_value(record.get(5).ok_or("vertical constraint missing")?).await?)),
            other => Err(format!("unknown constraint-node kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAssoc2dConstraintGroup {
    pub action: DwgAssociativeAction,
    pub do_not_check_newly_added_constraints: bool,
    pub work_plane: Vec<Vec<f64>>,
    pub member_action_handles: Vec<u64>,
    pub nodes: Vec<DwgConstraintNode>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_entity_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("line".into(), 0),
                    ("arc".into(), 1),
                    ("lwPolyline".into(), 2),
                    ("blockBegin".into(), 3),
                    ("blockEnd".into(), 4),
                    ("insert".into(), 5),
                    ("dimensionLinear".into(), 6),
                    ("viewport".into(), 7),
                    ("geometry".into(), 8),
                ]),
            ),
            dsl::FieldSpec::new(1, "line", <DwgLineEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "arc", <DwgArcEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "lw_polyline", <DwgLwPolylineEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "block_begin", <DwgBlockBeginEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "block_end", <DwgBlockEndEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "insert", <DwgInsertEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "dimension_linear", <DwgLinearDimensionEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(8, "viewport", <DwgViewportEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(9, "geometry", <DwgGeometryEntity as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgEntityBody {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_entity_body_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        match self {
            Self::Line(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(0));
                record.fields.insert(1, <DwgLineEntity as dsl::DslField>::to_value(value).await);
            }
            Self::Arc(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(1));
                record.fields.insert(2, <DwgArcEntity as dsl::DslField>::to_value(value).await);
            }
            Self::LwPolyline(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(2));
                record.fields.insert(3, <DwgLwPolylineEntity as dsl::DslField>::to_value(value).await);
            }
            Self::BlockBegin(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(3));
                record.fields.insert(4, <DwgBlockBeginEntity as dsl::DslField>::to_value(value).await);
            }
            Self::BlockEnd(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(4));
                record.fields.insert(5, <DwgBlockEndEntity as dsl::DslField>::to_value(value).await);
            }
            Self::Insert(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(5));
                record.fields.insert(6, <DwgInsertEntity as dsl::DslField>::to_value(value).await);
            }
            Self::DimensionLinear(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(6));
                record.fields.insert(7, <DwgLinearDimensionEntity as dsl::DslField>::to_value(value).await);
            }
            Self::Viewport(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(7));
                record.fields.insert(8, <DwgViewportEntity as dsl::DslField>::to_value(value).await);
            }
            Self::Geometry(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(8));
                record.fields.insert(9, <DwgGeometryEntity as dsl::DslField>::to_value(value).await);
            }
        }
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected entity-body record".into()) };
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Line(<DwgLineEntity as dsl::DslField>::from_value(record.get(1).ok_or("LINE body missing")?).await?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::Arc(<DwgArcEntity as dsl::DslField>::from_value(record.get(2).ok_or("ARC body missing")?).await?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::LwPolyline(<DwgLwPolylineEntity as dsl::DslField>::from_value(record.get(3).ok_or("LWPOLYLINE body missing")?).await?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::BlockBegin(<DwgBlockBeginEntity as dsl::DslField>::from_value(record.get(4).ok_or("BLOCK body missing")?).await?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::BlockEnd(<DwgBlockEndEntity as dsl::DslField>::from_value(record.get(5).ok_or("ENDBLK body missing")?).await?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Insert(<DwgInsertEntity as dsl::DslField>::from_value(record.get(6).ok_or("INSERT body missing")?).await?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::DimensionLinear(<DwgLinearDimensionEntity as dsl::DslField>::from_value(record.get(7).ok_or("DIMENSION_LINEAR body missing")?).await?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::Viewport(<DwgViewportEntity as dsl::DslField>::from_value(record.get(8).ok_or("VIEWPORT body missing")?).await?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::Geometry(<DwgGeometryEntity as dsl::DslField>::from_value(record.get(9).ok_or("GEOMETRY body missing")?).await?)),
            other => Err(format!("unknown entity-body kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgLogicalObjectBody {
    Dictionary(DwgDictionaryBody),
    TableControl(DwgTableControlBody),
    TableRecord(DwgTableRecordBody),
    XRecord(DwgXRecordBody),
    Entity(DwgEntityBody),
    AssociativeDependency(DwgAssociativeDependency),
    AssociativeValueDependency(DwgAssociativeValueDependency),
    AssociativeGeometryDependency(DwgAssociativeGeometryDependency),
    BlockGripLocationComponent(DwgBlockGripLocationComponent),
    DynamicBlockProxyNode(DwgDynamicBlockProxyNode),
    AssociativeVariable(DwgAssociativeVariable),
    AssociativeDimensionDependencyBody(DwgAssociativeDimensionDependencyBody),
    VisualStyle(DwgVisualStyle),
    BlockParameterDependencyBody(DwgBlockParameterDependencyBody),
    BlockRepresentationData(DwgBlockRepresentationData),
    DynamicBlockPurgePreventer(DwgDynamicBlockPurgePreventer),
    EvaluationGraph(DwgEvaluationGraph),
    BlockFlipParameter(DwgBlockFlipParameter),
    BlockVisibilityParameter(DwgBlockVisibilityParameter),
    Placeholder(DwgPlaceholder),
    DictionaryVariable(DwgDictionaryVariable),
    AnnotationScale(DwgAnnotationScale),
    SortEntitiesTable(DwgSortEntitiesTable),
    TableStyle(DwgTableStyle),
    MlineStyle(DwgMlineStyle),
    MLeaderStyle(DwgMLeaderStyle),
    Material(DwgMaterial),
    BlockMoveAction(DwgBlockMoveAction),
    AssocNetwork(DwgAssocNetwork),
    Assoc2dConstraintGroup(DwgAssoc2dConstraintGroup),
    BlockLinearParameter(DwgBlockLinearParameter),
    BlockLinearGrip(DwgBlockLinearGrip),
    BlockFlipGrip(DwgBlockFlipGrip),
    BlockVisibilityGrip(DwgBlockVisibilityGrip),
    BlockAlignmentParameter(DwgBlockAlignmentParameter),
    BlockAlignmentGrip(DwgBlockAlignmentGrip),
    BlockStretchAction(DwgBlockStretchAction),
    BlockScaleAction(DwgBlockScaleAction),
    BlockFlipAction(DwgBlockFlipAction),
    BlockBasePointParameter(DwgBlockBasePointParameter),
    BlockVerticalConstraintParameter(DwgBlockLinearConstraintParameter),
    BlockHorizontalConstraintParameter(DwgBlockLinearConstraintParameter),
    Layout(DwgLayout),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_logical_object_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("dictionary".into(), 0),
                    ("tableControl".into(), 1),
                    ("tableRecord".into(), 2),
                    ("xrecord".into(), 3),
                    ("entity".into(), 4),
                    ("associativeDependency".into(), 5),
                    ("associativeValueDependency".into(), 6),
                    ("associativeGeometryDependency".into(), 7),
                    ("blockGripLocationComponent".into(), 8),
                    ("dynamicBlockProxyNode".into(), 9),
                    ("associativeVariable".into(), 10),
                    ("associativeDimensionDependencyBody".into(), 11),
                    ("visualStyle".into(), 12),
                    ("blockParameterDependencyBody".into(), 13),
                    ("blockRepresentationData".into(), 14),
                    ("dynamicBlockPurgePreventer".into(), 15),
                    ("evaluationGraph".into(), 16),
                    ("blockFlipParameter".into(), 17),
                    ("blockVisibilityParameter".into(), 18),
                    ("placeholder".into(), 19),
                    ("dictionaryVariable".into(), 20),
                    ("annotationScale".into(), 21),
                    ("sortEntitiesTable".into(), 22),
                    ("tableStyle".into(), 23),
                    ("mlineStyle".into(), 24),
                    ("mLeaderStyle".into(), 25),
                    ("material".into(), 26),
                    ("blockMoveAction".into(), 27),
                    ("assocNetwork".into(), 28),
                    ("assoc2dConstraintGroup".into(), 29),
                    ("blockLinearParameter".into(), 30),
                    ("blockLinearGrip".into(), 31),
                    ("blockFlipGrip".into(), 32),
                    ("blockVisibilityGrip".into(), 33),
                    ("blockAlignmentParameter".into(), 34),
                    ("blockAlignmentGrip".into(), 35),
                    ("blockStretchAction".into(), 36),
                    ("blockScaleAction".into(), 37),
                    ("blockFlipAction".into(), 38),
                    ("blockBasePointParameter".into(), 39),
                    ("blockVerticalConstraintParameter".into(), 40),
                    ("blockHorizontalConstraintParameter".into(), 41),
                    ("layout".into(), 42),
                ]),
            ),
            dsl::FieldSpec::new(1, "dictionary", <DwgDictionaryBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "table_control", <DwgTableControlBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "table_record", <DwgTableRecordBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "xrecord", <DwgXRecordBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "entity", <DwgEntityBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "associative_dependency", <DwgAssociativeDependency as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "associative_value_dependency", <DwgAssociativeValueDependency as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(8, "associative_geometry_dependency", <DwgAssociativeGeometryDependency as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(9, "block_grip_location_component", <DwgBlockGripLocationComponent as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(10, "dynamic_block_proxy_node", <DwgDynamicBlockProxyNode as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(11, "associative_variable", <DwgAssociativeVariable as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(12, "associative_dimension_dependency_body", <DwgAssociativeDimensionDependencyBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(13, "visual_style", <DwgVisualStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(14, "block_parameter_dependency_body", <DwgBlockParameterDependencyBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(15, "block_representation_data", <DwgBlockRepresentationData as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(16, "dynamic_block_purge_preventer", <DwgDynamicBlockPurgePreventer as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(17, "evaluation_graph", <DwgEvaluationGraph as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(18, "block_flip_parameter", <DwgBlockFlipParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(19, "block_visibility_parameter", <DwgBlockVisibilityParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(20, "placeholder", <DwgPlaceholder as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(21, "dictionary_variable", <DwgDictionaryVariable as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(22, "annotation_scale", <DwgAnnotationScale as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(23, "sort_entities_table", <DwgSortEntitiesTable as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(24, "table_style", <DwgTableStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(25, "mline_style", <DwgMlineStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(26, "m_leader_style", <DwgMLeaderStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(27, "material", <DwgMaterial as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(28, "block_move_action", <DwgBlockMoveAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(29, "assoc_network", <DwgAssocNetwork as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(30, "assoc_2d_constraint_group", <DwgAssoc2dConstraintGroup as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(31, "block_linear_parameter", <DwgBlockLinearParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(32, "block_linear_grip", <DwgBlockLinearGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(33, "block_flip_grip", <DwgBlockFlipGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(34, "block_visibility_grip", <DwgBlockVisibilityGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(35, "block_alignment_parameter", <DwgBlockAlignmentParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(36, "block_alignment_grip", <DwgBlockAlignmentGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(37, "block_stretch_action", <DwgBlockStretchAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(38, "block_scale_action", <DwgBlockScaleAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(39, "block_flip_action", <DwgBlockFlipAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(40, "block_base_point_parameter", <DwgBlockBasePointParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(41, "block_vertical_constraint_parameter", <DwgBlockLinearConstraintParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(42, "block_horizontal_constraint_parameter", <DwgBlockLinearConstraintParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(43, "layout", <DwgLayout as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgLogicalObjectBody {
    async fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_logical_object_body_spec)
    }

    async fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let (kind, payload_id, payload) = match self {
            Self::Dictionary(value) => (0, 1, <DwgDictionaryBody as dsl::DslField>::to_value(value)),
            Self::TableControl(value) => (1, 2, <DwgTableControlBody as dsl::DslField>::to_value(value)),
            Self::TableRecord(value) => (2, 3, <DwgTableRecordBody as dsl::DslField>::to_value(value)),
            Self::XRecord(value) => (3, 4, <DwgXRecordBody as dsl::DslField>::to_value(value)),
            Self::Entity(value) => (4, 5, <DwgEntityBody as dsl::DslField>::to_value(value)),
            Self::AssociativeDependency(value) => (5, 6, <DwgAssociativeDependency as dsl::DslField>::to_value(value)),
            Self::AssociativeValueDependency(value) => (6, 7, <DwgAssociativeValueDependency as dsl::DslField>::to_value(value)),
            Self::AssociativeGeometryDependency(value) => (7, 8, <DwgAssociativeGeometryDependency as dsl::DslField>::to_value(value)),
            Self::BlockGripLocationComponent(value) => (8, 9, <DwgBlockGripLocationComponent as dsl::DslField>::to_value(value)),
            Self::DynamicBlockProxyNode(value) => (9, 10, <DwgDynamicBlockProxyNode as dsl::DslField>::to_value(value)),
            Self::AssociativeVariable(value) => (10, 11, <DwgAssociativeVariable as dsl::DslField>::to_value(value)),
            Self::AssociativeDimensionDependencyBody(value) => (11, 12, <DwgAssociativeDimensionDependencyBody as dsl::DslField>::to_value(value)),
            Self::VisualStyle(value) => (12, 13, <DwgVisualStyle as dsl::DslField>::to_value(value)),
            Self::BlockParameterDependencyBody(value) => (13, 14, <DwgBlockParameterDependencyBody as dsl::DslField>::to_value(value)),
            Self::BlockRepresentationData(value) => (14, 15, <DwgBlockRepresentationData as dsl::DslField>::to_value(value)),
            Self::DynamicBlockPurgePreventer(value) => (15, 16, <DwgDynamicBlockPurgePreventer as dsl::DslField>::to_value(value)),
            Self::EvaluationGraph(value) => (16, 17, <DwgEvaluationGraph as dsl::DslField>::to_value(value)),
            Self::BlockFlipParameter(value) => (17, 18, <DwgBlockFlipParameter as dsl::DslField>::to_value(value)),
            Self::BlockVisibilityParameter(value) => (18, 19, <DwgBlockVisibilityParameter as dsl::DslField>::to_value(value)),
            Self::Placeholder(value) => (19, 20, <DwgPlaceholder as dsl::DslField>::to_value(value)),
            Self::DictionaryVariable(value) => (20, 21, <DwgDictionaryVariable as dsl::DslField>::to_value(value)),
            Self::AnnotationScale(value) => (21, 22, <DwgAnnotationScale as dsl::DslField>::to_value(value)),
            Self::SortEntitiesTable(value) => (22, 23, <DwgSortEntitiesTable as dsl::DslField>::to_value(value)),
            Self::TableStyle(value) => (23, 24, <DwgTableStyle as dsl::DslField>::to_value(value)),
            Self::MlineStyle(value) => (24, 25, <DwgMlineStyle as dsl::DslField>::to_value(value)),
            Self::MLeaderStyle(value) => (25, 26, <DwgMLeaderStyle as dsl::DslField>::to_value(value)),
            Self::Material(value) => (26, 27, <DwgMaterial as dsl::DslField>::to_value(value)),
            Self::BlockMoveAction(value) => (27, 28, <DwgBlockMoveAction as dsl::DslField>::to_value(value)),
            Self::AssocNetwork(value) => (28, 29, <DwgAssocNetwork as dsl::DslField>::to_value(value)),
            Self::Assoc2dConstraintGroup(value) => (29, 30, <DwgAssoc2dConstraintGroup as dsl::DslField>::to_value(value)),
            Self::BlockLinearParameter(value) => (30, 31, <DwgBlockLinearParameter as dsl::DslField>::to_value(value)),
            Self::BlockLinearGrip(value) => (31, 32, <DwgBlockLinearGrip as dsl::DslField>::to_value(value)),
            Self::BlockFlipGrip(value) => (32, 33, <DwgBlockFlipGrip as dsl::DslField>::to_value(value)),
            Self::BlockVisibilityGrip(value) => (33, 34, <DwgBlockVisibilityGrip as dsl::DslField>::to_value(value)),
            Self::BlockAlignmentParameter(value) => (34, 35, <DwgBlockAlignmentParameter as dsl::DslField>::to_value(value)),
            Self::BlockAlignmentGrip(value) => (35, 36, <DwgBlockAlignmentGrip as dsl::DslField>::to_value(value)),
            Self::BlockStretchAction(value) => (36, 37, <DwgBlockStretchAction as dsl::DslField>::to_value(value)),
            Self::BlockScaleAction(value) => (37, 38, <DwgBlockScaleAction as dsl::DslField>::to_value(value)),
            Self::BlockFlipAction(value) => (38, 39, <DwgBlockFlipAction as dsl::DslField>::to_value(value)),
            Self::BlockBasePointParameter(value) => (39, 40, <DwgBlockBasePointParameter as dsl::DslField>::to_value(value)),
            Self::BlockVerticalConstraintParameter(value) => (40, 41, <DwgBlockLinearConstraintParameter as dsl::DslField>::to_value(value)),
            Self::BlockHorizontalConstraintParameter(value) => (41, 42, <DwgBlockLinearConstraintParameter as dsl::DslField>::to_value(value)),
            Self::Layout(value) => (42, 43, <DwgLayout as dsl::DslField>::to_value(value)),
        };
        record.fields.insert(0, dsl::FieldValue::Enum(kind));
        record.fields.insert(payload_id, payload);
        dsl::FieldValue::Record(record)
    }

    async fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else {
            return Err(format!("expected DWG object-body record, found {value:?}"));
        };
        if record.fields.values().filter(|value| !matches!(value, dsl::FieldValue::Absent)).count() != 2 {
            return Err("DWG object body must contain exactly one tagged payload".into());
        }
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Dictionary(<DwgDictionaryBody as dsl::DslField>::from_value(record.get(1).ok_or("dictionary body missing")?).await?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::TableControl(<DwgTableControlBody as dsl::DslField>::from_value(record.get(2).ok_or("table-control body missing")?).await?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::TableRecord(<DwgTableRecordBody as dsl::DslField>::from_value(record.get(3).ok_or("table-record body missing")?).await?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::XRecord(<DwgXRecordBody as dsl::DslField>::from_value(record.get(4).ok_or("XRECORD body missing")?).await?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::Entity(<DwgEntityBody as dsl::DslField>::from_value(record.get(5).ok_or("entity body missing")?).await?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::AssociativeDependency(<DwgAssociativeDependency as dsl::DslField>::from_value(record.get(6).ok_or("associative-dependency body missing")?).await?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::AssociativeValueDependency(<DwgAssociativeValueDependency as dsl::DslField>::from_value(record.get(7).ok_or("associative-value-dependency body missing")?).await?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::AssociativeGeometryDependency(<DwgAssociativeGeometryDependency as dsl::DslField>::from_value(record.get(8).ok_or("associative-geometry-dependency body missing")?).await?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::BlockGripLocationComponent(<DwgBlockGripLocationComponent as dsl::DslField>::from_value(record.get(9).ok_or("block-grip-location-component body missing")?).await?)),
            Some(dsl::FieldValue::Enum(9)) => Ok(Self::DynamicBlockProxyNode(<DwgDynamicBlockProxyNode as dsl::DslField>::from_value(record.get(10).ok_or("dynamic-block-proxy-node body missing")?).await?)),
            Some(dsl::FieldValue::Enum(10)) => Ok(Self::AssociativeVariable(<DwgAssociativeVariable as dsl::DslField>::from_value(record.get(11).ok_or("associative-variable body missing")?).await?)),
            Some(dsl::FieldValue::Enum(11)) => Ok(Self::AssociativeDimensionDependencyBody(<DwgAssociativeDimensionDependencyBody as dsl::DslField>::from_value(record.get(12).ok_or("associative-dimension-dependency body missing")?).await?)),
            Some(dsl::FieldValue::Enum(12)) => Ok(Self::VisualStyle(<DwgVisualStyle as dsl::DslField>::from_value(record.get(13).ok_or("visual-style body missing")?).await?)),
            Some(dsl::FieldValue::Enum(13)) => Ok(Self::BlockParameterDependencyBody(<DwgBlockParameterDependencyBody as dsl::DslField>::from_value(record.get(14).ok_or("block-parameter-dependency body missing")?).await?)),
            Some(dsl::FieldValue::Enum(14)) => Ok(Self::BlockRepresentationData(<DwgBlockRepresentationData as dsl::DslField>::from_value(record.get(15).ok_or("block-representation data missing")?).await?)),
            Some(dsl::FieldValue::Enum(15)) => Ok(Self::DynamicBlockPurgePreventer(<DwgDynamicBlockPurgePreventer as dsl::DslField>::from_value(record.get(16).ok_or("dynamic-block purge-preventer body missing")?).await?)),
            Some(dsl::FieldValue::Enum(16)) => Ok(Self::EvaluationGraph(<DwgEvaluationGraph as dsl::DslField>::from_value(record.get(17).ok_or("evaluation-graph body missing")?).await?)),
            Some(dsl::FieldValue::Enum(17)) => Ok(Self::BlockFlipParameter(<DwgBlockFlipParameter as dsl::DslField>::from_value(record.get(18).ok_or("block-flip-parameter body missing")?).await?)),
            Some(dsl::FieldValue::Enum(18)) => Ok(Self::BlockVisibilityParameter(<DwgBlockVisibilityParameter as dsl::DslField>::from_value(record.get(19).ok_or("block-visibility-parameter body missing")?).await?)),
            Some(dsl::FieldValue::Enum(19)) => Ok(Self::Placeholder(<DwgPlaceholder as dsl::DslField>::from_value(record.get(20).ok_or("placeholder body missing")?).await?)),
            Some(dsl::FieldValue::Enum(20)) => Ok(Self::DictionaryVariable(<DwgDictionaryVariable as dsl::DslField>::from_value(record.get(21).ok_or("dictionary-variable body missing")?).await?)),
            Some(dsl::FieldValue::Enum(21)) => Ok(Self::AnnotationScale(<DwgAnnotationScale as dsl::DslField>::from_value(record.get(22).ok_or("annotation-scale body missing")?).await?)),
            Some(dsl::FieldValue::Enum(22)) => Ok(Self::SortEntitiesTable(<DwgSortEntitiesTable as dsl::DslField>::from_value(record.get(23).ok_or("sort-entities-table body missing")?).await?)),
            Some(dsl::FieldValue::Enum(23)) => Ok(Self::TableStyle(<DwgTableStyle as dsl::DslField>::from_value(record.get(24).ok_or("table-style body missing")?).await?)),
            Some(dsl::FieldValue::Enum(24)) => Ok(Self::MlineStyle(<DwgMlineStyle as dsl::DslField>::from_value(record.get(25).ok_or("MLINESTYLE body missing")?).await?)),
            Some(dsl::FieldValue::Enum(25)) => Ok(Self::MLeaderStyle(<DwgMLeaderStyle as dsl::DslField>::from_value(record.get(26).ok_or("MLEADERSTYLE body missing")?).await?)),
            Some(dsl::FieldValue::Enum(26)) => Ok(Self::Material(<DwgMaterial as dsl::DslField>::from_value(record.get(27).ok_or("MATERIAL body missing")?).await?)),
            Some(dsl::FieldValue::Enum(27)) => Ok(Self::BlockMoveAction(<DwgBlockMoveAction as dsl::DslField>::from_value(record.get(28).ok_or("BLOCKMOVEACTION body missing")?).await?)),
            Some(dsl::FieldValue::Enum(28)) => Ok(Self::AssocNetwork(<DwgAssocNetwork as dsl::DslField>::from_value(record.get(29).ok_or("ACDBASSOCNETWORK body missing")?).await?)),
            Some(dsl::FieldValue::Enum(29)) => Ok(Self::Assoc2dConstraintGroup(<DwgAssoc2dConstraintGroup as dsl::DslField>::from_value(record.get(30).ok_or("ACDBASSOC2DCONSTRAINTGROUP body missing")?).await?)),
            Some(dsl::FieldValue::Enum(30)) => Ok(Self::BlockLinearParameter(<DwgBlockLinearParameter as dsl::DslField>::from_value(record.get(31).ok_or("BLOCKLINEARPARAMETER body missing")?).await?)),
            Some(dsl::FieldValue::Enum(31)) => Ok(Self::BlockLinearGrip(<DwgBlockLinearGrip as dsl::DslField>::from_value(record.get(32).ok_or("BLOCKLINEARGRIP body missing")?).await?)),
            Some(dsl::FieldValue::Enum(32)) => Ok(Self::BlockFlipGrip(<DwgBlockFlipGrip as dsl::DslField>::from_value(record.get(33).ok_or("BLOCKFLIPGRIP body missing")?).await?)),
            Some(dsl::FieldValue::Enum(33)) => Ok(Self::BlockVisibilityGrip(<DwgBlockVisibilityGrip as dsl::DslField>::from_value(record.get(34).ok_or("BLOCKVISIBILITYGRIP body missing")?).await?)),
            Some(dsl::FieldValue::Enum(34)) => Ok(Self::BlockAlignmentParameter(<DwgBlockAlignmentParameter as dsl::DslField>::from_value(record.get(35).ok_or("BLOCKALIGNMENTPARAMETER body missing")?).await?)),
            Some(dsl::FieldValue::Enum(35)) => Ok(Self::BlockAlignmentGrip(<DwgBlockAlignmentGrip as dsl::DslField>::from_value(record.get(36).ok_or("BLOCKALIGNMENTGRIP body missing")?).await?)),
            Some(dsl::FieldValue::Enum(36)) => Ok(Self::BlockStretchAction(<DwgBlockStretchAction as dsl::DslField>::from_value(record.get(37).ok_or("BLOCKSTRETCHACTION body missing")?).await?)),
            Some(dsl::FieldValue::Enum(37)) => Ok(Self::BlockScaleAction(<DwgBlockScaleAction as dsl::DslField>::from_value(record.get(38).ok_or("BLOCKSCALEACTION body missing")?).await?)),
            Some(dsl::FieldValue::Enum(38)) => Ok(Self::BlockFlipAction(<DwgBlockFlipAction as dsl::DslField>::from_value(record.get(39).ok_or("BLOCKFLIPACTION body missing")?).await?)),
            Some(dsl::FieldValue::Enum(39)) => Ok(Self::BlockBasePointParameter(<DwgBlockBasePointParameter as dsl::DslField>::from_value(record.get(40).ok_or("BLOCKBASEPOINTPARAMETER body missing")?).await?)),
            Some(dsl::FieldValue::Enum(40)) => Ok(Self::BlockVerticalConstraintParameter(<DwgBlockLinearConstraintParameter as dsl::DslField>::from_value(record.get(41).ok_or("BLOCKVERTICALCONSTRAINTPARAMETER body missing")?).await?)),
            Some(dsl::FieldValue::Enum(41)) => Ok(Self::BlockHorizontalConstraintParameter(<DwgBlockLinearConstraintParameter as dsl::DslField>::from_value(record.get(42).ok_or("BLOCKHORIZONTALCONSTRAINTPARAMETER body missing")?).await?)),
            Some(dsl::FieldValue::Enum(42)) => Ok(Self::Layout(<DwgLayout as dsl::DslField>::from_value(record.get(43).ok_or("LAYOUT body missing")?).await?)),
            other => Err(format!("expected DWG object-body kind, found {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalObject {
    pub handle: u64,
    pub type_code: u16,
    pub class_name: String,
    pub category: DwgObjectCategory,
    #[serde(default)]
    pub owner_handle: Option<u64>,
    #[serde(default)]
    pub reactor_handles: Vec<u64>,
    #[serde(default)]
    pub extension_dictionary_handle: Option<u64>,
    #[serde(default)]
    pub referenced_handles: Vec<u64>,
    #[serde(default)]
    pub extended_data: Vec<DwgExtendedEntityData>,
    #[serde(default)]
    pub body: Option<DwgLogicalObjectBody>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgLogicalDrawing {
    /// 🧭 Handle-keyed objects are the sole persisted entity authority; use `entities()` for a derived view.
    #[serde(default)]
    pub layers: Vec<DwgLogicalLayer>,
    #[serde(default)]
    pub objects: Vec<DwgLogicalObject>,
    #[serde(default)]
    pub extmin: Vec<f64>,
    #[serde(default)]
    pub extmax: Vec<f64>,
}

impl DwgLogicalDrawing {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_native(drawing: &dwg_engine::DwgDrawing) -> Result<Self, String> {
        let mut objects = drawing
            .layers
            .iter()
            .enumerate()
            .map(|(index, layer)| {
                let handle = u64::try_from(index).map_err(|_| "DWG layer index exceeds u64")?.checked_add(1).ok_or("DWG layer handle overflow")?;
                let color_index = u16::from(layer.color);
                Ok(DwgLogicalObject {
                    handle,
                    type_code: 51,
                    class_name: "LAYER".into(),
                    category: DwgObjectCategory::TableRecord,
                    body: Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(DwgLayerTableRecord {
                        common: DwgTableRecordCommon { name: layer.name.clone(), ..Default::default() },
                        plottable: true,
                        lineweight: 29,
                        color: DwgComplexColor { index: color_index, value: DwgComplexColorValue::ByAci { index: color_index }, ..Default::default() },
                        ..Default::default()
                    }))),
                    ..Default::default()
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let entity_handle_offset = u64::try_from(objects.len()).map_err(|_| "DWG layer count exceeds u64")?;
        let entity_objects = drawing
            .entities
            .iter()
            .enumerate()
            .map(|(index, entity)| {
                if entity.layer >= drawing.layers.len() {
                    return Err(format!("DWG entity {index} references missing layer index {}", entity.layer));
                }
                let (kind, color_index) = match entity.color {
                    dwg_engine::DwgColor::ByLayer => (DwgEntityColorKind::ByLayer, 256),
                    dwg_engine::DwgColor::ByBlock => (DwgEntityColorKind::ByBlock, 0),
                    dwg_engine::DwgColor::Index(value) => (DwgEntityColorKind::Index, u16::from(value)),
                };
                let layer_handle = u64::try_from(entity.layer).map_err(|_| format!("DWG entity {index} layer index exceeds u64"))?.checked_add(1).ok_or_else(|| format!("DWG entity {index} layer handle overflow"))?;
                let common = DwgEntityCommon { mode: DwgEntityMode::ModelSpace, color: DwgEntityColor { kind, index: color_index, ..Default::default() }, linetype_scale: 1.0, lineweight: 29, layer_handle, ..Default::default() };
                let (type_code, class_name, body) = match &entity.geometry {
                    dwg_engine::DwgGeometry::Line { start, end } => (19, "LINE", DwgEntityBody::Line(DwgLineEntity { common, start: start.to_vec(), end: end.to_vec(), thickness: 0.0, extrusion: vec![0.0, 0.0, 1.0] })),
                    dwg_engine::DwgGeometry::Arc { center, radius, start_angle, end_angle, normal } => {
                        (17, "ARC", DwgEntityBody::Arc(DwgArcEntity { common, center: center.to_vec(), radius: *radius, thickness: 0.0, extrusion: normal.to_vec(), start_angle: *start_angle, end_angle: *end_angle }))
                    }
                    dwg_engine::DwgGeometry::LwPolyline { closed, elevation, vertices, bulges } => (
                        77,
                        "LWPOLYLINE",
                        DwgEntityBody::LwPolyline(DwgLwPolylineEntity {
                            common,
                            closed: *closed,
                            elevation: *elevation,
                            thickness: 0.0,
                            extrusion: vec![0.0, 0.0, 1.0],
                            vertices: vertices.iter().enumerate().map(|(vertex_index, point)| DwgLwPolylineVertex { point: point.to_vec(), bulge: bulges.get(vertex_index).copied().unwrap_or_default(), ..Default::default() }).collect(),
                            ..Default::default()
                        }),
                    ),
                    geometry => {
                        let (type_code, class_name) = match geometry {
                            dwg_engine::DwgGeometry::Point { .. } => (27, "POINT"),
                            dwg_engine::DwgGeometry::Circle { .. } => (18, "CIRCLE"),
                            dwg_engine::DwgGeometry::Ellipse { .. } => (35, "ELLIPSE"),
                            dwg_engine::DwgGeometry::Spline { .. } => (36, "SPLINE"),
                            dwg_engine::DwgGeometry::Text { .. } => (1, "TEXT"),
                            dwg_engine::DwgGeometry::Face3d { .. } => (28, "3DFACE"),
                            dwg_engine::DwgGeometry::Polyline3d { .. } => (16, "POLYLINE3D"),
                            dwg_engine::DwgGeometry::PolyfaceMesh { .. } => (29, "POLYFACEMESH"),
                            dwg_engine::DwgGeometry::Line { .. } | dwg_engine::DwgGeometry::Arc { .. } | dwg_engine::DwgGeometry::LwPolyline { .. } => unreachable!(),
                        };
                        (type_code, class_name, DwgEntityBody::Geometry(DwgGeometryEntity { common, geometry: DwgLogicalGeometry::from_native(geometry) }))
                    }
                };
                let handle = entity_handle_offset.checked_add(u64::try_from(index).map_err(|_| "DWG entity index exceeds u64")?).and_then(|value| value.checked_add(1)).ok_or("DWG entity handle overflow")?;
                Ok(DwgLogicalObject { handle, type_code, class_name: class_name.into(), category: DwgObjectCategory::Entity, body: Some(DwgLogicalObjectBody::Entity(body)), ..Default::default() })
            })
            .collect::<Result<Vec<_>, String>>()?;
        objects.extend(entity_objects);
        Ok(Self { layers: drawing.layers.iter().map(|layer| DwgLogicalLayer { name: layer.name.clone(), color: layer.color }).collect(), objects, extmin: drawing.extmin.to_vec(), extmax: drawing.extmax.to_vec() })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn entities(&self) -> Vec<DwgLogicalEntity> {
        self.objects
            .iter()
            .filter_map(|object| {
                let Some(DwgLogicalObjectBody::Entity(body)) = object.body.as_ref() else { return None };
                let (common, geometry) = match body {
                    DwgEntityBody::Line(line) => (&line.common, DwgLogicalGeometry { kind: DwgLogicalGeometryKind::Line, values: line.start.iter().chain(&line.end).copied().collect(), ..Default::default() }),
                    DwgEntityBody::Arc(arc) => (
                        &arc.common,
                        DwgLogicalGeometry { kind: DwgLogicalGeometryKind::Arc, values: arc.center.iter().copied().chain([arc.radius, arc.start_angle, arc.end_angle]).chain(arc.extrusion.iter().copied()).collect(), ..Default::default() },
                    ),
                    DwgEntityBody::LwPolyline(polyline) => (
                        &polyline.common,
                        DwgLogicalGeometry {
                            kind: DwgLogicalGeometryKind::LwPolyline,
                            values: std::iter::once(polyline.elevation).chain(polyline.vertices.iter().flat_map(|vertex| vertex.point.iter().copied())).chain(polyline.vertices.iter().map(|vertex| vertex.bulge)).collect(),
                            indices: vec![polyline.vertices.len() as i32],
                            closed: polyline.closed,
                            ..Default::default()
                        },
                    ),
                    DwgEntityBody::Geometry(value) => (&value.common, value.geometry.clone()),
                    DwgEntityBody::BlockBegin(_) | DwgEntityBody::BlockEnd(_) | DwgEntityBody::Insert(_) | DwgEntityBody::DimensionLinear(_) | DwgEntityBody::Viewport(_) => return None,
                };
                let color = match common.color.kind {
                    DwgEntityColorKind::ByLayer => -1,
                    DwgEntityColorKind::ByBlock => -2,
                    DwgEntityColorKind::Index => common.color.index as i16,
                    DwgEntityColorKind::TrueColor => return None,
                };
                let layer = self
                    .objects
                    .iter()
                    .filter_map(|candidate| match candidate.body.as_ref() {
                        Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(_))) => Some(candidate.handle),
                        _ => None,
                    })
                    .position(|handle| handle == common.layer_handle)
                    .unwrap_or(0);
                Some(DwgLogicalEntity { layer, color, geometry })
            })
            .collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_native(&self) -> Result<dwg_engine::DwgDrawing, String> {
        Ok(dwg_engine::DwgDrawing {
            layers: self.layers.iter().map(|layer| dwg_engine::DwgLayer { name: layer.name.clone(), color: layer.color }).collect(),
            entities: self.entities().iter().map(DwgLogicalEntity::to_native).collect::<Result<_, _>>()?,
            extmin: vec3(&self.extmin)?,
            extmax: vec3(&self.extmax)?,
        })
    }
}

impl DwgLogicalEntity {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn to_native(&self) -> Result<dwg_engine::DwgEntity, String> {
        let color = match self.color {
            -1 => dwg_engine::DwgColor::ByLayer,
            -2 => dwg_engine::DwgColor::ByBlock,
            value if (0..=255).contains(&value) => dwg_engine::DwgColor::Index(value as u8),
            value => return Err(format!("invalid logical DWG color {value}")),
        };
        Ok(dwg_engine::DwgEntity { layer: self.layer, color, geometry: self.geometry.to_native()? })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vec2(values: &[f64]) -> Result<[f64; 2], String> {
    values.try_into().map_err(|_| format!("expected 2 values, got {}", values.len()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vec3(values: &[f64]) -> Result<[f64; 3], String> {
    values.try_into().map_err(|_| format!("expected 3 values, got {}", values.len()))
}

impl DwgLogicalGeometry {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from_native(geometry: &dwg_engine::DwgGeometry) -> Self {
        use dwg_engine::DwgGeometry::*;
        match geometry {
            Point { at } => Self { kind: DwgLogicalGeometryKind::Point, values: at.to_vec(), ..Default::default() },
            Line { start, end } => Self { kind: DwgLogicalGeometryKind::Line, values: start.iter().chain(end).copied().collect(), ..Default::default() },
            Circle { center, radius, normal } => Self { kind: DwgLogicalGeometryKind::Circle, values: center.iter().chain([radius]).chain(normal).copied().collect(), ..Default::default() },
            Arc { center, radius, start_angle, end_angle, normal } => Self { kind: DwgLogicalGeometryKind::Arc, values: center.iter().chain([radius, start_angle, end_angle]).chain(normal).copied().collect(), ..Default::default() },
            Ellipse { center, major_axis, ratio, start_param, end_param, normal } => {
                Self { kind: DwgLogicalGeometryKind::Ellipse, values: center.iter().chain(major_axis).chain([ratio, start_param, end_param]).chain(normal).copied().collect(), ..Default::default() }
            }
            LwPolyline { closed, elevation, vertices, bulges } => Self {
                kind: DwgLogicalGeometryKind::LwPolyline,
                values: std::iter::once(*elevation).chain(vertices.iter().flatten().copied()).chain(bulges.iter().copied()).collect(),
                indices: vec![vertices.len() as i32],
                closed: *closed,
                ..Default::default()
            },
            Spline { degree, control_points, knots, weights } => Self {
                kind: DwgLogicalGeometryKind::Spline,
                values: control_points.iter().flatten().copied().chain(knots.iter().copied()).chain(weights.iter().copied()).collect(),
                indices: vec![*degree as i32, control_points.len() as i32, knots.len() as i32],
                ..Default::default()
            },
            Text { at, height, rotation, content } => Self { kind: DwgLogicalGeometryKind::Text, values: at.iter().copied().chain([*height, *rotation]).collect(), text: content.clone(), ..Default::default() },
            Face3d { corners } => Self { kind: DwgLogicalGeometryKind::Face3d, values: corners.iter().flatten().copied().collect(), ..Default::default() },
            Polyline3d { closed, vertices } => Self { kind: DwgLogicalGeometryKind::Polyline3d, values: vertices.iter().flatten().copied().collect(), closed: *closed, ..Default::default() },
            PolyfaceMesh { vertices, faces } => {
                Self { kind: DwgLogicalGeometryKind::PolyfaceMesh, values: vertices.iter().flatten().copied().collect(), indices: std::iter::once(vertices.len() as i32).chain(faces.iter().flatten().copied()).collect(), ..Default::default() }
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn to_native(&self) -> Result<dwg_engine::DwgGeometry, String> {
        use DwgLogicalGeometryKind::*;
        Ok(match self.kind {
            Point => dwg_engine::DwgGeometry::Point { at: vec3(&self.values)? },
            Line => dwg_engine::DwgGeometry::Line { start: vec3(&self.values[0..3])?, end: vec3(&self.values[3..6])? },
            Circle => dwg_engine::DwgGeometry::Circle { center: vec3(&self.values[0..3])?, radius: self.values[3], normal: vec3(&self.values[4..7])? },
            Arc => dwg_engine::DwgGeometry::Arc { center: vec3(&self.values[0..3])?, radius: self.values[3], start_angle: self.values[4], end_angle: self.values[5], normal: vec3(&self.values[6..9])? },
            Ellipse => dwg_engine::DwgGeometry::Ellipse { center: vec3(&self.values[0..3])?, major_axis: vec3(&self.values[3..6])?, ratio: self.values[6], start_param: self.values[7], end_param: self.values[8], normal: vec3(&self.values[9..12])? },
            LwPolyline => {
                let count = *self.indices.first().ok_or("polyline vertex count missing")? as usize;
                let vertices = self.values[1..1 + count * 2].chunks_exact(2).map(vec2).collect::<Result<_, _>>()?;
                dwg_engine::DwgGeometry::LwPolyline { closed: self.closed, elevation: self.values[0], vertices, bulges: self.values[1 + count * 2..].to_vec() }
            }
            Spline => {
                let degree = self.indices[0] as u32;
                let point_count = self.indices[1] as usize;
                let knot_count = self.indices[2] as usize;
                let point_end = point_count * 3;
                let control_points = self.values[..point_end].chunks_exact(3).map(vec3).collect::<Result<_, _>>()?;
                dwg_engine::DwgGeometry::Spline { degree, control_points, knots: self.values[point_end..point_end + knot_count].to_vec(), weights: self.values[point_end + knot_count..].to_vec() }
            }
            Text => dwg_engine::DwgGeometry::Text { at: vec3(&self.values[0..3])?, height: self.values[3], rotation: self.values[4], content: self.text.clone() },
            Face3d => dwg_engine::DwgGeometry::Face3d { corners: [vec3(&self.values[0..3])?, vec3(&self.values[3..6])?, vec3(&self.values[6..9])?, vec3(&self.values[9..12])?] },
            Polyline3d => dwg_engine::DwgGeometry::Polyline3d { closed: self.closed, vertices: self.values.chunks_exact(3).map(vec3).collect::<Result<_, _>>()? },
            PolyfaceMesh => {
                let vertex_count = self.indices[0] as usize;
                let vertices = self.values.chunks_exact(3).take(vertex_count).map(vec3).collect::<Result<_, _>>()?;
                let faces = self.indices[1..].chunks_exact(4).map(|face| face.try_into().unwrap()).collect();
                dwg_engine::DwgGeometry::PolyfaceMesh { vertices, faces }
            }
        })
    }
}
//#endregion 🔖️DrawingModel

//#region 🔖️DocumentModel
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderUnits {
    pub unit1_conversion: f64,
    pub unit2_conversion: f64,
    pub unit3_conversion: f64,
    pub unit4_conversion: f64,
    pub unit1_name: String,
    pub unit2_name: String,
    pub unit3_name: String,
    pub unit4_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderModes {
    pub dimension_associative: bool,
    pub dimension_show: bool,
    pub polyline_generation: bool,
    pub orthographic_mode: bool,
    pub regeneration_mode: bool,
    pub fill_mode: bool,
    pub quick_text_mode: bool,
    pub paper_space_linetype_scale: bool,
    pub limits_check: bool,
    pub user_timer: bool,
    pub sketch_polyline: bool,
    pub angle_direction: bool,
    pub spline_frame: bool,
    pub mirror_text: bool,
    pub world_view: bool,
    pub tile_mode: bool,
    pub paper_limits_check: bool,
    pub visual_retain: bool,
    pub display_silhouette: bool,
    pub polyline_ellipse: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderIntegerSettings {
    pub proxy_graphics: u16,
    pub tree_depth: i16,
    pub linear_units: u16,
    pub linear_precision: u16,
    pub angular_units: u16,
    pub angular_precision: u16,
    pub attribute_mode: u16,
    pub point_display_mode: u16,
    pub user_integer1: i16,
    pub user_integer2: i16,
    pub user_integer3: i16,
    pub user_integer4: i16,
    pub user_integer5: i16,
    pub spline_segments: u16,
    pub surface_u: u16,
    pub surface_v: u16,
    pub surface_type: u16,
    pub surface_tab1: u16,
    pub surface_tab2: u16,
    pub spline_type: u16,
    pub shade_edge: u16,
    pub shade_difference: u16,
    pub unit_mode: u16,
    pub maximum_active_viewports: u16,
    pub isolines: u16,
    pub multiline_justification: u16,
    pub text_quality: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderScalars {
    pub linetype_scale: f64,
    pub text_size: f64,
    pub trace_width: f64,
    pub sketch_increment: f64,
    pub fillet_radius: f64,
    pub thickness: f64,
    pub angle_base: f64,
    pub point_display_size: f64,
    pub polyline_width: f64,
    pub user_real1: f64,
    pub user_real2: f64,
    pub user_real3: f64,
    pub user_real4: f64,
    pub user_real5: f64,
    pub chamfer_a: f64,
    pub chamfer_b: f64,
    pub chamfer_c: f64,
    pub chamfer_d: f64,
    pub facet_resolution: f64,
    pub multiline_scale: f64,
    pub current_entity_linetype_scale: f64,
    pub current_entity_color_index: u16,
    pub paper_space_viewport_scale: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderTimeState {
    pub created_at: DwgJulianDate,
    pub updated_at: DwgJulianDate,
    pub editing_duration: DwgJulianDate,
    pub user_timer_duration: DwgJulianDate,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderSpaceGeometry {
    pub insertion_base: Vec<f64>,
    pub extents_minimum: Vec<f64>,
    pub extents_maximum: Vec<f64>,
    pub limits_minimum: Vec<f64>,
    pub limits_maximum: Vec<f64>,
    pub elevation: f64,
    pub ucs_origin: Vec<f64>,
    pub ucs_x_axis: Vec<f64>,
    pub ucs_y_axis: Vec<f64>,
    pub ucs_orthographic_view: u16,
    pub ucs_origin_top: Vec<f64>,
    pub ucs_origin_bottom: Vec<f64>,
    pub ucs_origin_left: Vec<f64>,
    pub ucs_origin_right: Vec<f64>,
    pub ucs_origin_front: Vec<f64>,
    pub ucs_origin_back: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDimensionSettings {
    pub scale: f64,
    pub arrow_size: f64,
    pub extension_offset: f64,
    pub line_increment: f64,
    pub extension: f64,
    pub rounding: f64,
    pub line_extension: f64,
    pub tolerance_plus: f64,
    pub tolerance_minus: f64,
    pub fixed_extension_length: f64,
    pub jog_angle: f64,
    pub text_fill: u16,
    pub text_fill_color_index: u16,
    pub tolerance: bool,
    pub limits: bool,
    pub text_inside_horizontal: bool,
    pub text_outside_horizontal: bool,
    pub suppress_extension1: bool,
    pub suppress_extension2: bool,
    pub text_above: u16,
    pub zero_suppression: u16,
    pub angular_zero_suppression: u16,
    pub arc_symbol: u16,
    pub text_height: f64,
    pub center_mark: f64,
    pub tick_size: f64,
    pub alternate_scale: f64,
    pub linear_factor: f64,
    pub text_vertical_position: f64,
    pub text_factor: f64,
    pub gap: f64,
    pub alternate_rounding: f64,
    pub alternate_units: bool,
    pub alternate_decimal_places: u16,
    pub text_outside_force_line: bool,
    pub separate_arrows: bool,
    pub text_inside: bool,
    pub suppress_outside: bool,
    pub line_color_index: u16,
    pub extension_color_index: u16,
    pub text_color_index: u16,
    pub angular_decimal_places: u16,
    pub decimal_places: u16,
    pub tolerance_decimal_places: u16,
    pub alternate_units_format: u16,
    pub alternate_tolerance_decimal_places: u16,
    pub angular_unit_format: u16,
    pub fractional_format: u16,
    pub linear_unit_format: u16,
    pub decimal_separator: u16,
    pub text_movement: u16,
    pub justification: u16,
    pub suppress_dimension1: bool,
    pub suppress_dimension2: bool,
    pub tolerance_justification: u16,
    pub tolerance_zero_suppression: u16,
    pub alternate_zero_suppression: u16,
    pub alternate_tolerance_zero_suppression: u16,
    pub user_positioned_text: bool,
    pub fit: u16,
    pub fixed_extension_enabled: bool,
    pub text_direction: bool,
    pub alternate_measurement_scale: f64,
    pub measurement_scale: f64,
    pub dimension_line_weight: i16,
    pub extension_line_weight: i16,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDrawingPolicy {
    pub text_stack_alignment: u16,
    pub text_stack_size: u16,
    pub current_entity_lineweight: i16,
    pub end_caps: u8,
    pub join_style: u8,
    pub lineweight_display: bool,
    pub external_reference_editing: bool,
    pub extended_names: bool,
    pub plot_style_mode: bool,
    pub ole_startup: bool,
    pub insertion_units: u16,
    pub current_plot_style_type: u16,
    pub sort_entities: u8,
    pub index_control: u8,
    pub hide_text: u8,
    pub xclip_frame: u8,
    pub dimension_association: u8,
    pub halo_gap: u8,
    pub obscured_color: u16,
    pub intersection_color: u16,
    pub obscured_linetype: u8,
    pub intersection_display: u8,
    pub camera_display: bool,
    pub steps_per_second: f64,
    pub step_size: f64,
    pub dwf_3d_precision: f64,
    pub lens_length: f64,
    pub camera_height: f64,
    pub solid_history: u8,
    pub show_history: u8,
    pub polysolid_width: f64,
    pub polysolid_height: f64,
    pub loft_angle1: f64,
    pub loft_angle2: f64,
    pub loft_magnitude1: f64,
    pub loft_magnitude2: f64,
    pub loft_parameter: u16,
    pub loft_normals: u8,
    pub latitude: f64,
    pub longitude: f64,
    pub north_direction: f64,
    pub timezone: i32,
    pub light_glyph_display: u8,
    pub tile_mode_light_sync: u8,
    pub dwf_frame: u8,
    pub dgn_frame: u8,
    pub real_world_scale: bool,
    pub interfere_color_index: u16,
    pub shadow_mode: u8,
    pub shadow_plane_location: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderStrings {
    pub menu: String,
    pub dimension_postfix: String,
    pub dimension_alternate_postfix: String,
    pub dimension_alternate_measurement_zero_suffix: String,
    pub dimension_measurement_zero_suffix: String,
    pub hyperlink_base: String,
    pub stylesheet: String,
    pub fingerprint_guid: String,
    pub version_guid: String,
    pub project_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderRelations {
    pub handle_seed: u64,
    pub current_layer: u64,
    pub text_style: u64,
    pub current_linetype: u64,
    pub current_material: u64,
    pub dimension_style: u64,
    pub multiline_style: u64,
    pub paper_ucs_name: Option<u64>,
    pub paper_ucs_orthographic_reference: Option<u64>,
    pub paper_ucs_base: Option<u64>,
    pub model_ucs_name: Option<u64>,
    pub model_ucs_orthographic_reference: Option<u64>,
    pub model_ucs_base: Option<u64>,
    pub dimension_text_style: u64,
    pub dimension_leader_block: Option<u64>,
    pub dimension_block: Option<u64>,
    pub dimension_block1: Option<u64>,
    pub dimension_block2: Option<u64>,
    pub dimension_linetype: Option<u64>,
    pub dimension_extension_linetype1: Option<u64>,
    pub dimension_extension_linetype2: Option<u64>,
    pub block_control: u64,
    pub layer_control: u64,
    pub style_control: u64,
    pub linetype_control: u64,
    pub view_control: u64,
    pub ucs_control: u64,
    pub viewport_control: u64,
    pub appid_control: u64,
    pub dimension_style_control: u64,
    pub group_dictionary: u64,
    pub multiline_style_dictionary: u64,
    pub named_objects_dictionary: u64,
    pub layout_dictionary: u64,
    pub plot_settings_dictionary: u64,
    pub plot_style_name_dictionary: u64,
    pub material_dictionary: u64,
    pub color_dictionary: u64,
    pub visual_style_dictionary: u64,
    pub paper_space_block_record: u64,
    pub model_space_block_record: u64,
    pub by_layer_linetype: u64,
    pub by_block_linetype: u64,
    pub continuous_linetype: u64,
    pub interfere_object_visual_style: Option<u64>,
    pub interfere_viewport_visual_style: Option<u64>,
    pub drag_visual_style: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgHeaderVariables {
    pub units: DwgHeaderUnits,
    pub modes: DwgHeaderModes,
    pub integers: DwgHeaderIntegerSettings,
    pub scalars: DwgHeaderScalars,
    pub time: DwgHeaderTimeState,
    pub paper_space: DwgHeaderSpaceGeometry,
    pub model_space: DwgHeaderSpaceGeometry,
    pub dimensions: DwgDimensionSettings,
    pub policy: DwgDrawingPolicy,
    pub strings: DwgHeaderStrings,
    pub relations: DwgHeaderRelations,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgClass {
    pub number: u16,
    pub proxy_flags: u32,
    pub application_name: String,
    pub cpp_class_name: String,
    pub dxf_name: String,
    #[serde(default)]
    pub was_zombie: bool,
    #[serde(default)]
    pub item_class_id: u16,
    #[serde(default)]
    pub object_count: u32,
    #[serde(default)]
    pub dwg_version: u32,
    #[serde(default)]
    pub maintenance_version: u32,
    #[serde(default)]
    pub reserved_values: Vec<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgDependency {
    pub feature: String,
    pub full_path: String,
    pub relative_path: String,
    pub fingerprint: String,
    pub version: String,
    #[serde(default)]
    pub timestamp: u32,
    #[serde(default)]
    pub file_size: u32,
    #[serde(default)]
    pub affects_graphics: bool,
    #[serde(default)]
    pub reference_count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgJulianDate {
    pub days: u32,
    pub milliseconds: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgVersionStamp {
    pub version: u16,
    pub maintenance: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgCompatibilityProfile {
    #[default]
    Autocad2009,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgAuxiliaryHeader {
    pub total_saves: u32,
    pub save_partition_one: u16,
    pub save_partition_two: u16,
    pub save_generation: u32,
    pub legacy_stamp_one: DwgVersionStamp,
    pub legacy_stamp_two: DwgVersionStamp,
    pub compatibility_profile: DwgCompatibilityProfile,
    pub created_at: DwgJulianDate,
    pub updated_at: DwgJulianDate,
    pub handle_seed: u64,
    pub terminal_save_generation: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgRevisionHistory {
    pub format_major: u32,
    pub format_minor: u32,
    #[serde(default)]
    pub revisions: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgPreviewOrigin {
    #[default]
    BottomUp,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgRgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgIndexedPreview {
    pub width: u32,
    pub height: u32,
    pub origin: DwgPreviewOrigin,
    #[serde(default)]
    pub palette: Vec<DwgRgba>,
    #[serde(default)]
    pub pixel_indices: Vec<u8>,
    pub background_palette_index: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgApplicationPropertyKind {
    #[default]
    String,
    DateTime,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgApplicationProperty {
    pub id: u32,
    pub kind: DwgApplicationPropertyKind,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgProductInformation {
    pub name: String,
    pub build_version: String,
    pub registry_version: String,
    pub install_id: String,
    pub locale_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgApplicationHistory {
    pub history_identifier_one: String,
    pub history_identifier_two: String,
    pub class_version: u32,
    pub application_version_digest: String,
    pub application_version: String,
    pub trust_comment_digest: String,
    pub trust_comment: String,
    pub property_set_digest: String,
    pub property_format_identifier: String,
    #[serde(default)]
    pub properties: Vec<DwgApplicationProperty>,
    pub product_digest: String,
    pub product: DwgProductInformation,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgCustomProperty {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgSummaryInfo {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub keywords: String,
    pub comments: String,
    pub last_saved_by: String,
    pub revision_number: String,
    pub hyperlink_base: String,
    #[serde(default)]
    pub total_editing_time: u64,
    #[serde(default)]
    pub created_at: DwgJulianDate,
    #[serde(default)]
    pub modified_at: DwgJulianDate,
    #[serde(default)]
    pub custom_properties: Vec<DwgCustomProperty>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgApplicationInfo {
    pub name: String,
    pub version_checksum: String,
    pub version: String,
    pub comment_checksum: String,
    pub comment: String,
    pub product_checksum: String,
    pub product: String,
    pub application_version: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum DwgMeasurement {
    #[default]
    English,
    Metric,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DwgTemplate {
    pub description: String,
    pub measurement: DwgMeasurement,
}
//#endregion 🔖️DocumentModel

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub version: String,
    /// 🗓️ `maint_version` (RC, plain preamble byte 0x12) — cross-checked against LibreDWG's
    /// `header.spec` (`FIELD_RC (maint_version, 0);` right after `dwg_version` at 0x11) and
    /// verified on the real `architectural.dwg` fixture (byte 0x12 == 0x02).
    #[state(artifact)]
    #[serde(default)]
    pub maintenance_version: u8,
    /// 🌐 `codepage` (RS, plain preamble bytes 0x13-0x14, little-endian) — LibreDWG's
    /// `header.spec` documents this exact offset with `//@0x13: 29/30 for ANSI_1252`; the real
    /// `architectural.dwg` fixture reads `30` there, an exact match.
    #[state(artifact)]
    #[serde(default)]
    pub codepage: u16,
    #[state(artifact)]
    #[serde(default)]
    pub drawing: DwgLogicalDrawing,
    #[state(artifact)]
    #[serde(default)]
    pub header: DwgHeaderVariables,
    #[state(artifact)]
    #[serde(default)]
    pub classes: Vec<DwgClass>,
    #[state(artifact)]
    #[serde(default)]
    pub dependencies: Vec<DwgDependency>,
    #[state(artifact)]
    #[serde(default)]
    pub summary: DwgSummaryInfo,
    #[state(artifact)]
    #[serde(default)]
    pub application: DwgApplicationInfo,
    #[state(artifact)]
    #[serde(default)]
    pub template: DwgTemplate,
    #[state(artifact)]
    #[serde(default)]
    pub auxiliary_header: DwgAuxiliaryHeader,
    #[state(artifact)]
    #[serde(default)]
    pub revision_history: DwgRevisionHistory,
    #[state(artifact)]
    #[serde(default)]
    pub preview: DwgIndexedPreview,
    #[state(artifact)]
    #[serde(default)]
    pub application_history: DwgApplicationHistory,
}

impl Default for DwgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
            version: String::new(),
            maintenance_version: 0,
            codepage: 0,
            drawing: DwgLogicalDrawing::default(),
            header: DwgHeaderVariables::default(),
            classes: Vec::new(),
            dependencies: Vec::new(),
            summary: DwgSummaryInfo::default(),
            application: DwgApplicationInfo::default(),
            template: DwgTemplate::default(),
            auxiliary_header: DwgAuxiliaryHeader::default(),
            revision_history: DwgRevisionHistory::default(),
            preview: DwgIndexedPreview::default(),
            application_history: DwgApplicationHistory::default(),
        }
    }
}

impl DwgSnapshot {
    /// 🪞️ Clones the deterministic logical projection.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn projection(&self) -> Self {
        self.clone()
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DwgCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_version_sentinel(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() < 6 {
        return Err("DWG too short for AC10xx header".into());
    }
    let head = &bytes[0..6];
    if head[0] != b'A' || head[1] != b'C' || !head[2].is_ascii_digit() || !head[3].is_ascii_digit() {
        return Err("missing AC10xx DWG version sentinel".into());
    }
    if !head[4].is_ascii_digit() || !head[5].is_ascii_digit() {
        return Err("invalid AC10xx version digits".into());
    }
    Ok(String::from_utf8_lossy(head).into_owned())
}

/// 🗓️🌐 Reads `maint_version` (offset 0x12) and `codepage` (offset 0x13-0x14 LE) from the plain
/// file-header preamble shared by every AC1015+ DWG file, per LibreDWG's own
/// `header.spec` field order (`zero_one_or_three@0x0B`, `thumbnail_address@0x0D`,
/// `dwg_version@0x11`, `maint_version@0x12`, `codepage@0x13`). Truncated headers are rejected.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_version_header_fields(bytes: &[u8]) -> Result<(u8, u16), String> {
    let maintenance_version = *bytes.get(0x12).ok_or("DWG header is too short for maintenance version")?;
    let codepage = bytes.get(0x13..0x15).ok_or("DWG header is too short for codepage")?;
    Ok((maintenance_version, u16::from_le_bytes([codepage[0], codepage[1]])))
}

/// 🗺️ Materializes section pages only while deserializing and projects their standard objects.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_drawing(bytes: &[u8]) -> Result<DwgLogicalDrawing, String> {
    DwgLogicalDrawing::from_native(&dwg_engine::decode_r2004_drawing(bytes)?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let (maintenance_version, codepage) = parse_version_header_fields(bytes)?;
    if version == "AC1015" {
        let drawing = DwgLogicalDrawing::from_native(&dwg_engine::dwg_from_bytes(bytes)?)?;
        return Ok(DwgSnapshot { schema: STDIO_DWG_DOCUMENT_SCHEMA.into(), version, maintenance_version, codepage, drawing, ..Default::default() });
    }
    let mut drawing = decode_drawing(bytes)?;
    let classes = dwg_engine::decode_r2004_classes(bytes)?;
    drawing.objects = dwg_engine::decode_r2004_object_identities(bytes, &classes)?;
    let document = dwg_engine::decode_r2004_document_sections(bytes)?;
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version,
        maintenance_version,
        codepage,
        drawing,
        header: document.header,
        classes,
        dependencies: document.dependencies,
        summary: document.summary,
        application: document.application,
        template: document.template,
        auxiliary_header: document.auxiliary_header,
        revision_history: document.revision_history,
        preview: document.preview,
        application_history: document.application_history,
        ..Default::default()
    })
}

/// 🚫 Typed DWG export failures distinguish invalid logical state from writer failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DwgExportError {
    InvalidLogical(String),
    InvalidVersion(String),
    HeaderMismatch(String),
    Writer(String),
}

impl fmt::Display for DwgExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLogical(message) => write!(formatter, "invalid logical DWG: {message}"),
            Self::InvalidVersion(message) => write!(formatter, "invalid DWG version: {message}"),
            Self::HeaderMismatch(message) => write!(formatter, "DWG header mismatch: {message}"),
            Self::Writer(message) => write!(formatter, "DWG writer failed: {message}"),
        }
    }
}

impl std::error::Error for DwgExportError {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_export_header(bytes: &[u8], snapshot: &DwgSnapshot) -> Result<(), DwgExportError> {
    let version = dwg_version_sentinel(bytes).map_err(DwgExportError::Writer)?;
    if snapshot.version.len() != 6 {
        return Err(DwgExportError::InvalidVersion("AC10xx sentinel must contain six ASCII bytes".into()));
    }
    if version != snapshot.version {
        return Err(DwgExportError::HeaderMismatch(format!("version {} != {}", version, snapshot.version)));
    }
    let (maintenance_version, codepage) = parse_version_header_fields(bytes).map_err(DwgExportError::Writer)?;
    if maintenance_version != snapshot.maintenance_version {
        return Err(DwgExportError::HeaderMismatch(format!("maintenance version {maintenance_version} != {}", snapshot.maintenance_version)));
    }
    if codepage != snapshot.codepage {
        return Err(DwgExportError::HeaderMismatch(format!("codepage {codepage} != {}", snapshot.codepage)));
    }
    Ok(())
}

/// 🔄 Updates supported typed header fields.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn synchronize_version_info(snapshot: &mut DwgSnapshot, version: &str, maintenance_version: u8, codepage: u16) -> Result<(), DwgExportError> {
    dwg_version_sentinel(version.as_bytes()).map_err(DwgExportError::InvalidVersion)?;
    snapshot.version = version.to_string();
    snapshot.maintenance_version = maintenance_version;
    snapshot.codepage = codepage;
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_dwg(snapshot: &DwgSnapshot) -> Result<Vec<u8>, DwgExportError> {
    if snapshot.schema != STDIO_DWG_DOCUMENT_SCHEMA {
        return Err(DwgExportError::InvalidLogical("schema identity changed".into()));
    }
    let bytes = if snapshot.version == "AC1015" {
        dwg_engine::dwg_to_bytes(&snapshot.drawing.to_native().map_err(DwgExportError::InvalidLogical)?).map_err(DwgExportError::Writer)?
    } else {
        dwg_engine::encode_r2004_snapshot(snapshot).map_err(DwgExportError::Writer)?
    };
    validate_export_header(&bytes, snapshot)?;
    Ok(bytes)
}
//#endregion 🔖️DwgCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DwgSnapshot {
    const EXTENSION: &'static str = "dwg";
    async fn envelope_id() -> &'static str {
        "stdio.dwg"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document }).await?;
        Self::__dsl_from_record(&record).await
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DwgSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options).await?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options).await?;
        Self::__dsl_from_record(&record).await.map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
