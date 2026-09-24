//! 🧬️ DwgSnapshot schema — persistent fields + real codecs. Byte/bit-level decode logic (file
//! header decrypt, R2004+ LZ77-variant decompression, section/page directory walk) lives in
//! `⚙️engine` (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION 🖊️dwg
//! D1-D2 wave); this module owns the typed persisted model and glues `decode_dwg`/`encode_dwg`
//! to it.

use crate::standards::v_ac1024::engine as dwg_engine;
use crate::STDIO_DWG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use std::fmt;

//#region 🔖️DrawingModel
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalGeometry {
    pub kind: DwgLogicalGeometryKind,
    #[value(default)]
    pub values: Vec<f64>,
    #[value(default)]
    pub indices: Vec<i32>,
    #[value(default)]
    pub text: String,
    #[value(default)]
    pub closed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalLayer {
    pub name: String,
    pub color: u8,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalEntity {
    pub layer: usize,
    pub color: i16,
    pub geometry: DwgLogicalGeometry,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgObjectCategory {
    Entity,
    TableControl,
    TableRecord,
    Dictionary,
    #[default]
    Object,
    Custom,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgNamedReference {
    pub name: String,
    pub handle: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_xrecord_value_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
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

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDictionaryBody {
    #[value(default)]
    pub entries: Vec<DwgNamedReference>,
    pub cloning_flag: u16,
    pub hard_owner: bool,
    #[value(default)]
    pub default_entry_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DwgTableControlEntry {
    #[value(default)]
    pub handle: Option<u64>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_table_control_entry_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "has_handle", dsl::Shape::Bool), dsl::FieldSpec::new(1, "handle", dsl::Shape::UInt).optional()])
}

impl dsl::DslField for DwgTableControlEntry {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_table_control_entry_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        record.fields.insert(0, dsl::FieldValue::Bool(self.handle.is_some()));
        if let Some(handle) = self.handle {
            record.fields.insert(1, <u64 as dsl::DslField>::to_value(&handle));
        }
        dsl::FieldValue::Record(record)
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected table-control entry record".into()) };
        match record.get(0) {
            Some(dsl::FieldValue::Bool(false)) => Ok(Self { handle: None }),
            Some(dsl::FieldValue::Bool(true)) => Ok(Self { handle: Some(<u64 as dsl::DslField>::from_value(record.get(1).ok_or("table-control handle missing")?)?) }),
            other => Err(format!("invalid table-control handle presence {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgTableControlEntries {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockTableControl {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    #[value(default)]
    pub model_space_handle: Option<u64>,
    #[value(default)]
    pub paper_space_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLinetypeTableControl {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    pub by_block_handle: u64,
    pub by_layer_handle: u64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDimensionStyleTableControl {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    #[value(default)]
    pub additional_handles: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(table_control_body_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
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

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected table-control body record".into()) };
        match record.get(1) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Block(<DwgBlockTableControl as dsl::DslField>::from_value(record.get(3).ok_or("block control missing")?)?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::Layer(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("layer control missing")?)?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::TextStyle(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("text-style control missing")?)?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::Linetype(<DwgLinetypeTableControl as dsl::DslField>::from_value(record.get(4).ok_or("linetype control missing")?)?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::View(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("view control missing")?)?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Ucs(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("UCS control missing")?)?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::Viewport(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("viewport control missing")?)?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::RegisteredApplication(<DwgTableControlEntries as dsl::DslField>::from_value(record.get(2).ok_or("registered-application control missing")?)?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::DimensionStyle(<DwgDimensionStyleTableControl as dsl::DslField>::from_value(record.get(5).ok_or("dimension-style control missing")?)?)),
            other => Err(format!("unknown table-control kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgTableRecordCommon {
    pub name: String,
    pub xref_resolution: u16,
    #[value(default)]
    pub xref_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgRegisteredApplicationTableRecord {
    pub common: DwgTableRecordCommon,
    pub group_71: u8,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_complex_color_value_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let kind = match self {
            Self::None => 0,
            Self::ByLayer => 1,
            Self::ByBlock => 2,
            Self::ByColor { red, green, blue } => {
                record.fields.insert(1, <u8 as dsl::DslField>::to_value(red));
                record.fields.insert(2, <u8 as dsl::DslField>::to_value(green));
                record.fields.insert(3, <u8 as dsl::DslField>::to_value(blue));
                3
            }
            Self::ByAci { index } => {
                record.fields.insert(4, <u16 as dsl::DslField>::to_value(index));
                4
            }
            Self::ByPen { index } => {
                record.fields.insert(4, <u16 as dsl::DslField>::to_value(&u16::from(*index)));
                5
            }
            Self::Foreground => 6,
            Self::LayerOff => 7,
            Self::LayerFrozen => 8,
        };
        record.fields.insert(0, dsl::FieldValue::Enum(kind));
        dsl::FieldValue::Record(record)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected complex-color value record".into()) };
        let no_extra = |allowed: &[u16]| record.fields.iter().all(|(field, value)| *field == 0 || allowed.contains(field) || matches!(value, dsl::FieldValue::Absent));
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) if no_extra(&[]) => Ok(Self::None),
            Some(dsl::FieldValue::Enum(1)) if no_extra(&[]) => Ok(Self::ByLayer),
            Some(dsl::FieldValue::Enum(2)) if no_extra(&[]) => Ok(Self::ByBlock),
            Some(dsl::FieldValue::Enum(3)) if no_extra(&[1, 2, 3]) => Ok(Self::ByColor {
                red: <u8 as dsl::DslField>::from_value(record.get(1).ok_or("red missing")?)?,
                green: <u8 as dsl::DslField>::from_value(record.get(2).ok_or("green missing")?)?,
                blue: <u8 as dsl::DslField>::from_value(record.get(3).ok_or("blue missing")?)?,
            }),
            Some(dsl::FieldValue::Enum(4)) if no_extra(&[4]) => Ok(Self::ByAci { index: <u16 as dsl::DslField>::from_value(record.get(4).ok_or("ACI index missing")?)? }),
            Some(dsl::FieldValue::Enum(5)) if no_extra(&[4]) => Ok(Self::ByPen { index: u8::try_from(<u16 as dsl::DslField>::from_value(record.get(4).ok_or("pen index missing")?)?).map_err(|_| "pen index exceeds u8")? }),
            Some(dsl::FieldValue::Enum(6)) if no_extra(&[]) => Ok(Self::Foreground),
            Some(dsl::FieldValue::Enum(7)) if no_extra(&[]) => Ok(Self::LayerOff),
            Some(dsl::FieldValue::Enum(8)) if no_extra(&[]) => Ok(Self::LayerFrozen),
            other => Err(format!("invalid complex-color value {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgComplexColor {
    pub index: u16,
    pub value: DwgComplexColorValue,
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub book_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLayerTableRecord {
    pub common: DwgTableRecordCommon,
    pub frozen: bool,
    pub off: bool,
    pub frozen_in_new_viewports: bool,
    pub locked: bool,
    pub plottable: bool,
    pub lineweight: u8,
    pub color: DwgComplexColor,
    #[value(default)]
    pub plot_style_handle: Option<u64>,
    #[value(default)]
    pub material_handle: Option<u64>,
    #[value(default)]
    pub linetype_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLinetypeDash {
    pub length: f64,
    pub complex_shape_code: u16,
    #[value(default)]
    pub style_handle: Option<u64>,
    pub x_offset: f64,
    pub y_offset: f64,
    pub scale: f64,
    pub rotation: f64,
    pub shape_flags: u16,
    #[value(default)]
    pub text: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLinetypeTableRecord {
    pub common: DwgTableRecordCommon,
    pub description: String,
    pub pattern_length: f64,
    pub alignment: u8,
    #[value(default)]
    pub dashes: Vec<DwgLinetypeDash>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockHeaderTableRecord {
    pub common: DwgTableRecordCommon,
    pub anonymous: bool,
    pub has_attributes: bool,
    pub is_xref: bool,
    pub xref_overlaid: bool,
    pub xref_loaded: bool,
    #[value(default)]
    pub owned_entity_handles: Vec<u64>,
    pub base_point: [f64; 3],
    pub xref_path: String,
    #[value(default)]
    pub insert_backreference_handles: Vec<u64>,
    pub description: String,
    pub insert_units: u16,
    pub explodable: bool,
    pub block_scaling: u8,
    pub block_entity_handle: u64,
    pub end_block_entity_handle: u64,
    #[value(default)]
    pub layout_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
    #[value(default)]
    pub background_handle: Option<u64>,
    #[value(default)]
    pub visual_style_handle: Option<u64>,
    #[value(default)]
    pub sun_handle: Option<u64>,
    #[value(default)]
    pub named_ucs_handle: Option<u64>,
    #[value(default)]
    pub base_ucs_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(table_record_body_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        match self {
            Self::RegisteredApplication(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(0));
                record.fields.insert(2, <DwgRegisteredApplicationTableRecord as dsl::DslField>::to_value(value));
            }
            Self::TextStyle(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(1));
                record.fields.insert(3, <DwgTextStyleTableRecord as dsl::DslField>::to_value(value));
            }
            Self::Layer(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(2));
                record.fields.insert(4, <DwgLayerTableRecord as dsl::DslField>::to_value(value));
            }
            Self::Linetype(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(3));
                record.fields.insert(5, <DwgLinetypeTableRecord as dsl::DslField>::to_value(value));
            }
            Self::BlockHeader(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(4));
                record.fields.insert(6, <DwgBlockHeaderTableRecord as dsl::DslField>::to_value(value));
            }
            Self::Viewport(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(5));
                record.fields.insert(7, <DwgViewportTableRecord as dsl::DslField>::to_value(value));
            }
            Self::DimensionStyle(value) => {
                record.fields.insert(1, dsl::FieldValue::Enum(6));
                record.fields.insert(8, <DwgDimensionStyleTableRecord as dsl::DslField>::to_value(value));
            }
        }
        dsl::FieldValue::Record(record)
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else {
            return Err("expected table-record body record".into());
        };
        match record.get(1) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::RegisteredApplication(<DwgRegisteredApplicationTableRecord as dsl::DslField>::from_value(record.get(2).ok_or("registered-application record missing")?)?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::TextStyle(<DwgTextStyleTableRecord as dsl::DslField>::from_value(record.get(3).ok_or("text-style record missing")?)?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::Layer(<DwgLayerTableRecord as dsl::DslField>::from_value(record.get(4).ok_or("layer record missing")?)?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::Linetype(<DwgLinetypeTableRecord as dsl::DslField>::from_value(record.get(5).ok_or("linetype record missing")?)?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::BlockHeader(<DwgBlockHeaderTableRecord as dsl::DslField>::from_value(record.get(6).ok_or("block-header record missing")?)?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Viewport(<DwgViewportTableRecord as dsl::DslField>::from_value(record.get(7).ok_or("viewport record missing")?)?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::DimensionStyle(<DwgDimensionStyleTableRecord as dsl::DslField>::from_value(record.get(8).ok_or("dimension-style record missing")?)?)),
            other => Err(format!("unknown table-record kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgXRecordBody {
    #[value(default)]
    pub values: Vec<DwgXRecordValue>,
    #[value(default)]
    pub object_id_handles: Vec<u64>,
    pub cloning_flag: u16,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgExtendedEntityData {
    pub application_handle: u64,
    #[value(default)]
    pub values: Vec<DwgXRecordValue>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgEntityMode {
    ExplicitOwner,
    PaperSpace,
    #[default]
    ModelSpace,
    Reserved,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgEntityReferenceMode {
    #[default]
    ByLayer,
    ByBlock,
    Continuous,
    Explicit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgEntityColorKind {
    #[default]
    ByLayer,
    ByBlock,
    Index,
    TrueColor,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgEntityColor {
    pub kind: DwgEntityColorKind,
    pub index: u16,
    pub rgb: u32,
    #[value(default)]
    pub transparency: Option<u32>,
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub book_name: Option<String>,
    #[value(default)]
    pub color_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
    #[value(default)]
    pub linetype_handle: Option<u64>,
    #[value(default)]
    pub material_handle: Option<u64>,
    #[value(default)]
    pub shadow_handle: Option<u64>,
    #[value(default)]
    pub plot_style_handle: Option<u64>,
    #[value(default)]
    pub full_visual_style_handle: Option<u64>,
    #[value(default)]
    pub face_visual_style_handle: Option<u64>,
    #[value(default)]
    pub edge_visual_style_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLineEntity {
    pub common: DwgEntityCommon,
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgArcEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub radius: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    pub start_angle: f64,
    pub end_angle: f64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLwPolylineVertex {
    pub point: Vec<f64>,
    pub bulge: f64,
    #[value(default)]
    pub vertex_id: Option<u32>,
    #[value(default)]
    pub start_width: Option<f64>,
    #[value(default)]
    pub end_width: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLwPolylineEntity {
    pub common: DwgEntityCommon,
    pub closed: bool,
    #[value(default)]
    pub constant_width: Option<f64>,
    pub elevation: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    #[value(default)]
    pub vertices: Vec<DwgLwPolylineVertex>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockBeginEntity {
    pub common: DwgEntityCommon,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockEndEntity {
    pub common: DwgEntityCommon,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgInsertEntity {
    pub common: DwgEntityCommon,
    pub insertion: Vec<f64>,
    pub scale: Vec<f64>,
    pub rotation: f64,
    pub extrusion: Vec<f64>,
    pub block_header_handle: u64,
    #[value(default)]
    pub attribute_handles: Vec<u64>,
    #[value(default)]
    pub sequence_end_handle: Option<u64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgDimensionLineSpacingStyle {
    #[default]
    AtLeast,
    Exact,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDimensionStatus {
    pub block_reference_is_exclusive: bool,
    pub user_positioned_text: bool,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
    #[value(default)]
    pub dimension_block_handle: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLinearDimensionEntity {
    pub dimension: DwgDimensionEntityCommon,
    pub extension_line_1: Vec<f64>,
    pub extension_line_2: Vec<f64>,
    pub definition_point: Vec<f64>,
    pub oblique_angle: f64,
    pub dimension_rotation: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgShadePlotMode {
    #[default]
    AsDisplayed,
    Wireframe,
    Hidden,
    Rendered,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgDefaultLightingType {
    OneDistantLight,
    #[default]
    TwoDistantLights,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgPointEntity {
    pub common: DwgEntityCommon,
    pub point: Vec<f64>,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    pub x_axis_angle: f64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgCircleEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub radius: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgEllipseEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub major_axis: Vec<f64>,
    pub extrusion: Vec<f64>,
    pub axis_ratio: f64,
    pub start_parameter: f64,
    pub end_parameter: f64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgTextEntity {
    pub common: DwgEntityCommon,
    pub elevation: f64,
    pub insertion: Vec<f64>,
    #[value(default)]
    pub alignment: Option<Vec<f64>>,
    pub extrusion: Vec<f64>,
    pub thickness: f64,
    pub oblique_angle: f64,
    pub rotation: f64,
    pub height: f64,
    pub width_factor: f64,
    pub value: String,
    pub generation: u16,
    pub horizontal_alignment: u16,
    pub vertical_alignment: u16,
    pub style_handle: u64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgSplineEntity {
    pub common: DwgEntityCommon,
    pub degree: u32,
    pub rational: bool,
    pub closed: bool,
    pub periodic: bool,
    pub knot_tolerance: f64,
    pub control_tolerance: f64,
    #[value(default)]
    pub knots: Vec<f64>,
    #[value(default)]
    pub control_points: Vec<f64>,
    #[value(default)]
    pub weights: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgFace3dEntity {
    pub common: DwgEntityCommon,
    pub corners: Vec<f64>,
    pub invisible_edges: u16,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgPolyline3dEntity {
    pub common: DwgEntityCommon,
    pub curve_type: u8,
    pub flags: u8,
    #[value(default)]
    pub vertex_handles: Vec<u64>,
    pub sequence_end_handle: u64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgPolyfaceMeshEntity {
    pub common: DwgEntityCommon,
    pub vertex_count: u16,
    pub face_count: u16,
    #[value(default)]
    pub vertex_handles: Vec<u64>,
    pub sequence_end_handle: u64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgVertexEntity {
    pub common: DwgEntityCommon,
    pub flags: u8,
    pub point: Vec<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgPolyfaceFaceEntity {
    pub common: DwgEntityCommon,
    pub indices: Vec<i16>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgSequenceEndEntity {
    pub common: DwgEntityCommon,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEntityBody {
    Line(DwgLineEntity),
    Arc(DwgArcEntity),
    LwPolyline(DwgLwPolylineEntity),
    BlockBegin(DwgBlockBeginEntity),
    BlockEnd(DwgBlockEndEntity),
    Insert(DwgInsertEntity),
    DimensionLinear(DwgLinearDimensionEntity),
    Viewport(DwgViewportEntity),
    Point(DwgPointEntity),
    Circle(DwgCircleEntity),
    Ellipse(DwgEllipseEntity),
    Text(DwgTextEntity),
    Spline(DwgSplineEntity),
    Face3d(DwgFace3dEntity),
    Polyline3d(DwgPolyline3dEntity),
    PolyfaceMesh(DwgPolyfaceMeshEntity),
    Vertex(DwgVertexEntity),
    PolyfaceFace(DwgPolyfaceFaceEntity),
    SequenceEnd(DwgSequenceEndEntity),
}

impl DwgEntityBody {
    /// 🧬️ The common entity data every body carries.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn common_mut(&mut self) -> &mut DwgEntityCommon {
        match self {
            Self::Line(value) => &mut value.common,
            Self::Arc(value) => &mut value.common,
            Self::LwPolyline(value) => &mut value.common,
            Self::BlockBegin(value) => &mut value.common,
            Self::BlockEnd(value) => &mut value.common,
            Self::Insert(value) => &mut value.common,
            Self::DimensionLinear(value) => &mut value.dimension.common,
            Self::Viewport(value) => &mut value.common,
            Self::Point(value) => &mut value.common,
            Self::Circle(value) => &mut value.common,
            Self::Ellipse(value) => &mut value.common,
            Self::Text(value) => &mut value.common,
            Self::Spline(value) => &mut value.common,
            Self::Face3d(value) => &mut value.common,
            Self::Polyline3d(value) => &mut value.common,
            Self::PolyfaceMesh(value) => &mut value.common,
            Self::Vertex(value) => &mut value.common,
            Self::PolyfaceFace(value) => &mut value.common,
            Self::SequenceEnd(value) => &mut value.common,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgAssociativeDependencyStatus {
    #[default]
    UpToDate,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeDependency {
    pub status: DwgAssociativeDependencyStatus,
    pub is_read_dependency: bool,
    pub is_write_dependency: bool,
    pub is_attached_to_object: bool,
    pub is_delegating_to_owning_action: bool,
    pub order: i32,
    pub dependent_on_object_handle: u64,
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub read_dependency_handle: Option<u64>,
    #[value(default)]
    pub dependency_node_handle: Option<u64>,
    #[value(default)]
    pub dependency_body_handle: Option<u64>,
    pub dependency_body_id: i32,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEvaluationVariant {
    Integer32(i32),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_evaluation_variant_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "kind", dsl::Shape::Enum(vec![("integer32".into(), 0)])), dsl::FieldSpec::new(1, "integer32", <i32 as dsl::DslField>::shape()).optional()])
}

impl dsl::DslField for DwgEvaluationVariant {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_evaluation_variant_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        match self {
            Self::Integer32(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(0));
                record.fields.insert(1, <i32 as dsl::DslField>::to_value(value));
            }
        }
        dsl::FieldValue::Record(record)
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected evaluation-variant record".into()) };
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Integer32(<i32 as dsl::DslField>::from_value(record.get(1).ok_or("evaluation integer32 missing")?)?)),
            other => Err(format!("unknown evaluation-variant kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeValueDependency {
    pub dependency: DwgAssociativeDependency,
    pub cached_value: DwgEvaluationVariant,
    pub value_name: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeGeometryDependency {
    pub dependency: DwgAssociativeDependency,
    pub enabled: bool,
    pub persistent_subentity_class_name: String,
    pub dependent_on_compound_object: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_evaluation_expression_value_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
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

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected evaluation-expression value record".into()) };
        let present = record.fields.values().filter(|value| !matches!(value, dsl::FieldValue::Absent)).count();
        let expected = if matches!(record.get(0), Some(dsl::FieldValue::Enum(0))) { 1 } else { 2 };
        if present != expected {
            return Err("evaluation-expression value must contain exactly its tagged payload".into());
        }
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Empty),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::Double(<f64 as dsl::DslField>::from_value(record.get(1).ok_or("evaluation-expression double missing")?)?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::PointGroup10(<Vec<f64> as dsl::DslField>::from_value(record.get(2).ok_or("evaluation-expression group-10 point missing")?)?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::PointGroup11(<Vec<f64> as dsl::DslField>::from_value(record.get(3).ok_or("evaluation-expression group-11 point missing")?)?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::String(<String as dsl::DslField>::from_value(record.get(4).ok_or("evaluation-expression string missing")?)?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Integer32(<i32 as dsl::DslField>::from_value(record.get(5).ok_or("evaluation-expression integer32 missing")?)?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::ObjectReference(<u64 as dsl::DslField>::from_value(record.get(6).ok_or("evaluation-expression object reference missing")?)?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::Integer16(<i16 as dsl::DslField>::from_value(record.get(7).ok_or("evaluation-expression integer16 missing")?)?)),
            other => Err(format!("unknown evaluation-expression value kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationExpression {
    pub parent_id: i32,
    pub major_version: u32,
    pub minor_version: u32,
    pub value: DwgEvaluationExpressionValue,
    pub node_id: u32,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockGripLocationComponent {
    pub evaluation_expression: DwgEvaluationExpression,
    pub grip_type: u32,
    pub grip_expression: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDynamicBlockProxyNode {
    pub evaluation_expression: DwgEvaluationExpression,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgAssociativeActionStatus {
    #[default]
    UpToDate,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeActionDependency {
    pub owned: bool,
    pub dependency_handle: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeAction {
    pub status: DwgAssociativeActionStatus,
    #[value(default)]
    pub owning_network_handle: Option<u64>,
    #[value(default)]
    pub action_body_handle: Option<u64>,
    pub action_index: i32,
    pub maximum_dependency_index: i32,
    pub dependencies: Vec<DwgAssociativeActionDependency>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeVariable {
    pub action: DwgAssociativeAction,
    pub name: String,
    pub expression: String,
    pub evaluator_id: String,
    pub description: String,
    pub evaluated_value: DwgEvaluationVariant,
    pub mergeable: bool,
    #[value(default)]
    pub mergeable_variable_name: Option<String>,
    pub must_merge: bool,
    pub referenced_value_dependency_handles: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeDimensionDependencyBody {
    pub name: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgVisualStylePropertyOperation {
    Inherit,
    #[default]
    Set,
    Disable,
    Enable,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DwgVisualStyleProperty<T> {
    pub value: T,
    pub operation: DwgVisualStylePropertyOperation,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_visual_style_property_spec<T: dsl::DslField>() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "value", T::shape()), dsl::FieldSpec::new(1, "operation", <DwgVisualStylePropertyOperation as dsl::DslField>::shape())])
}

impl<T: dsl::DslField> dsl::DslField for DwgVisualStyleProperty<T> {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_visual_style_property_spec::<T>)
    }
    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        record.fields.insert(0, T::to_value(&self.value));
        record.fields.insert(1, <DwgVisualStylePropertyOperation as dsl::DslField>::to_value(&self.operation));
        dsl::FieldValue::Record(record)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected visual-style property record".into()) };
        Ok(Self { value: T::from_value(record.get(0).ok_or("visual-style property value missing")?)?, operation: <DwgVisualStylePropertyOperation as dsl::DslField>::from_value(record.get(1).ok_or("visual-style property operation missing")?)? })
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgVisualStyle {
    pub description: String,
    pub style_type: u32,
    pub extension_lighting_model: u16,
    pub internal_only: bool,
    pub properties: DwgVisualStyleProperties,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterDependencyBody {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockRepresentationData {
    pub represented_block_header_handle: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDynamicBlockPurgePreventer {
    pub protected_block_header_handle: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationGraphNode {
    pub id: u32,
    pub expression_handle: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationGraphEdge {
    pub from_node_id: u32,
    pub to_node_id: u32,
    pub reference_count: u32,
    pub invertible: bool,
    pub suppressed: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationGraph {
    pub nodes: Vec<DwgEvaluationGraphNode>,
    pub edges: Vec<DwgEvaluationGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterConnection {
    pub code: u32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterProperty {
    pub connections: Vec<DwgBlockParameterConnection>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockParameterBaseLocation {
    #[default]
    StartPoint,
    Midpoint,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockFlipValueSet {
    pub base_label: String,
    pub flipped_label: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgNamedEvaluationNodeReference {
    pub node_id: u32,
    pub expression_name: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgVisibilityEvaluationHistory {
    #[default]
    Stateless,
    Required,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgVisibilityState {
    pub name: String,
    pub visible_entity_handles: Vec<u64>,
    pub controlled_expression_handles: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockElement {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockGrip {
    pub element: DwgBlockElement,
    pub location: Vec<f64>,
    pub insertion_cycling: bool,
    pub insertion_cycling_weight: i32,
    pub updated_x: DwgNamedEvaluationNodeReference,
    pub updated_y: DwgNamedEvaluationNodeReference,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgPropertyExpressionReference {
    pub property_index: u32,
    pub node_id: u32,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockLinearParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub distance_name: String,
    pub distance_description: String,
    pub label_offset: f64,
    pub allowed_values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockLinearGrip {
    pub grip: DwgBlockGrip,
    pub orientation: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockFlipGrip {
    pub grip: DwgBlockGrip,
    pub updated_flip: DwgNamedEvaluationNodeReference,
    pub orientation: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockVisibilityGrip {
    pub grip: DwgBlockGrip,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgPlaceholder {}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDictionaryVariable {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAnnotationScale {
    pub name: String,
    pub paper_units: f64,
    pub drawing_units: f64,
    pub is_unit_scale: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDrawOrderEntry {
    pub entity_handle: u64,
    pub sort_handle: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgSortEntitiesTable {
    pub block_header_handle: u64,
    pub entries: Vec<DwgDrawOrderEntry>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgCellMargins {
    pub vertical: f64,
    pub horizontal: f64,
    pub bottom: f64,
    pub right: f64,
    pub horizontal_spacing: f64,
    pub vertical_spacing: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgCellBorder {
    pub override_flags: u32,
    pub border_type: u32,
    pub color: DwgComplexColor,
    pub lineweight: i32,
    pub linetype_handle: Option<u64>,
    pub visible: u32,
    pub double_line_spacing: f64,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgCellBorders {
    pub top: Option<DwgCellBorder>,
    pub horizontal_inside: Option<DwgCellBorder>,
    pub bottom: Option<DwgCellBorder>,
    pub left: Option<DwgCellBorder>,
    pub vertical_inside: Option<DwgCellBorder>,
    pub right: Option<DwgCellBorder>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgCellStyle {
    pub property_override_flags: u32,
    pub merge_flags: u32,
    pub background_color: DwgComplexColor,
    pub content_layout: u32,
    pub content_format: DwgCellContentFormat,
    pub margins: DwgCellMargins,
    pub borders: DwgCellBorders,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgTableStyle {
    pub description: String,
    pub bit_flags: u32,
    pub template_style_handle: Option<u64>,
    pub table: DwgCellStyle,
    pub title: DwgCellStyle,
    pub header: DwgCellStyle,
    pub data: DwgCellStyle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMlineLinetype {
    #[default]
    ByLayer,
    ByBlock,
    Continuous,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMlineCaps {
    pub square: bool,
    pub inner_arcs: bool,
    pub round_outer_arcs: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMlineStyleElement {
    pub offset: f64,
    pub color: DwgComplexColor,
    pub linetype: DwgMlineLinetype,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderContentType {
    None,
    Block,
    #[default]
    MText,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderDrawOrder {
    #[default]
    LeaderFirst,
    ContentFirst,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderLeaderOrder {
    #[default]
    HeadFirst,
    TailFirst,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderKind {
    Invisible,
    #[default]
    Straight,
    Spline,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderTextAngle {
    #[default]
    Horizontal,
    Aligned,
    AlwaysRightReading,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderTextAlignment {
    #[default]
    Left,
    Center,
    Right,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderAttachmentDirection {
    #[default]
    Horizontal,
    Vertical,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderBlockConnection {
    #[default]
    Extents,
    BasePoint,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderLeaderStyle {
    pub kind: DwgMLeaderKind,
    pub color: DwgComplexColor,
    pub linetype_style_handle: u64,
    pub lineweight: i32,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderLanding {
    pub enabled: bool,
    pub gap: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderDogleg {
    pub enabled: bool,
    pub length: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderArrow {
    pub symbol_handle: Option<u64>,
    pub size: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderBlockStyle {
    pub content_handle: Option<u64>,
    pub color: DwgComplexColor,
    pub scale: Vec<f64>,
    pub use_scale: bool,
    pub rotation: f64,
    pub use_rotation: bool,
    pub connection: DwgMLeaderBlockConnection,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMaterialProjection {
    Inherit,
    Planar,
    #[default]
    Box,
    Cylinder,
    Sphere,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMaterialTiling {
    Inherit,
    #[default]
    Tile,
    Crop,
    Clamp,
    Mirror,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMaterialMapSource {
    #[default]
    None,
    CurrentScene,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMaterialColor {
    pub factor: f64,
    pub override_rgb: Option<u32>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMaterialMap {
    pub blend_factor: f64,
    pub projection: DwgMaterialProjection,
    pub tiling: DwgMaterialTiling,
    pub scale_to_entity: bool,
    pub use_current_block_transform: bool,
    pub transform: Vec<f64>,
    pub source: DwgMaterialMapSource,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgMaterialChannels {
    pub diffuse: bool,
    pub specular: bool,
    pub reflection: bool,
    pub opacity: bool,
    pub bump: bool,
    pub refraction: bool,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockActionConnection {
    pub node_id: u32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockActionDependency {
    pub object_handle: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockAction {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
    pub display_location: Vec<f64>,
    pub dependencies: Vec<DwgBlockActionDependency>,
    pub action_node_ids: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockMoveCoordinateMode {
    #[default]
    CartesianXy,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockMoveAction {
    pub action: DwgBlockAction,
    pub x_connection: DwgBlockActionConnection,
    pub y_connection: DwgBlockActionConnection,
    pub distance_multiplier: f64,
    pub angle_offset: f64,
    pub coordinate_mode: DwgBlockMoveCoordinateMode,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockAlignmentParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub updated_grip_node_id: u32,
    pub align_perpendicular: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockAlignmentGrip {
    pub grip: DwgBlockGrip,
    pub first_location_node_id: u32,
    pub second_location_node_id: u32,
    pub orientation: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgStretchSelection {
    pub object_handle: u64,
    pub vertex_indices: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgStretchSelector {
    pub node_id: u32,
    pub point_indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockActionCoordinateMode {
    #[default]
    CartesianXy,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockActionWithBasePoint {
    pub action: DwgBlockAction,
    pub offset: Vec<f64>,
    pub x_base_connection: DwgBlockActionConnection,
    pub y_base_connection: DwgBlockActionConnection,
    pub dependent: bool,
    pub base_point: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockScaleMode {
    #[default]
    Xy,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockScaleAction {
    pub base: DwgBlockActionWithBasePoint,
    pub uniform_scale_connection: DwgBlockActionConnection,
    pub x_scale_connection: DwgBlockActionConnection,
    pub y_scale_connection: DwgBlockActionConnection,
    pub mode: DwgBlockScaleMode,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockFlipAction {
    pub action: DwgBlockAction,
    pub flip_connection: DwgBlockActionConnection,
    pub updated_flip_connection: DwgBlockActionConnection,
    pub updated_base_connection: DwgBlockActionConnection,
    pub updated_end_connection: DwgBlockActionConnection,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockOnePointParameter {
    pub element: DwgBlockElement,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_point: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockBasePointParameter {
    pub parameter: DwgBlockOnePointParameter,
    pub point: Vec<f64>,
    pub base_point: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterAllowedValues {
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockLinearConstraintParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub displacement_grip_node_id: u32,
    pub dependency_handle: u64,
    pub expression_name: String,
    pub expression_description: String,
    pub value: f64,
    pub allowed_values: DwgBlockParameterAllowedValues,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgPlotOptions {
    pub use_standard_scale: bool,
    pub plot_viewport_borders: bool,
    pub plot_with_lineweights: bool,
    pub draw_viewports_first: bool,
    pub model_type: bool,
    pub update_paper: bool,
    pub initializing: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgPlotPaperUnit {
    #[default]
    Inches,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgPlotRotation {
    #[default]
    QuarterTurn,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgPlotArea {
    #[default]
    Display,
    Layout,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgStandardScale {
    #[default]
    Custom,
    OneToOne,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgShadePlot {
    #[default]
    AsDisplayed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgShadePlotResolution {
    #[default]
    Normal,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLayoutOptions {
    pub paper_space_linetype_scaling: bool,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgAssocNetworkMemberKind {
    Network,
    #[default]
    Action,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssocNetworkMember {
    pub handle: u64,
    pub kind: DwgAssocNetworkMemberKind,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAssocNetwork {
    pub action: DwgAssociativeAction,
    pub network_action_index: i32,
    pub actions: Vec<DwgAssocNetworkMember>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgConstraintNodeCore {
    pub id: i32,
    pub connected_node_ids: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgGeometricConstraint {
    pub node: DwgConstraintNodeCore,
    pub owner_node_id: u32,
    pub implied: bool,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgConstraintGeometry {
    pub node: DwgConstraintNodeCore,
    pub geometry_dependency_handle: Option<u64>,
    pub geometry_node_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgExplicitConstraint {
    pub geometric: DwgGeometricConstraint,
    pub value_dependency_handle: u64,
    pub dimension_dependency_handle: u64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgConstrainedImplicitPoint {
    pub geometry: DwgConstraintGeometry,
    pub point: Option<Vec<f64>>,
    pub point_kind: u8,
    pub point_index: i32,
    pub curve_node_id: i32,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgConstrainedBoundedLine {
    pub geometry: DwgConstraintGeometry,
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
    pub ray: bool,
    pub bounded: bool,
    pub start_point: Vec<f64>,
    pub end_point: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDistanceConstraint {
    pub explicit: DwgExplicitConstraint,
    pub direction_kind: u8,
    pub direction: Option<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgAxisConstraint {
    pub geometric: DwgGeometricConstraint,
    pub datum_line_index: i32,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgConstrainedDatumLine {
    pub geometry: DwgConstraintGeometry,
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_constraint_node_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
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

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected constraint-node record".into()) };
        if record.fields.values().filter(|value| !matches!(value, dsl::FieldValue::Absent)).count() != 2 {
            return Err("constraint node must contain exactly its tagged body".into());
        }
        let geometric = || DwgGeometricConstraint::from_value(record.get(2).ok_or("geometric constraint missing")?);
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::ConstrainedImplicitPoint(DwgConstrainedImplicitPoint::from_value(record.get(1).ok_or("implicit point missing")?)?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::PointCurveConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::ConstrainedBoundedLine(DwgConstrainedBoundedLine::from_value(record.get(3).ok_or("bounded line missing")?)?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::PointCoincidenceConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::DistanceConstraint(DwgDistanceConstraint::from_value(record.get(4).ok_or("distance constraint missing")?)?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::PerpendicularConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::HorizontalConstraint(DwgAxisConstraint::from_value(record.get(5).ok_or("horizontal constraint missing")?)?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::ParallelConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::MidPointConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(9)) => Ok(Self::EqualLengthConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(10)) => Ok(Self::ColinearConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(11)) => Ok(Self::ConstrainedDatumLine(DwgConstrainedDatumLine::from_value(record.get(6).ok_or("datum line missing")?)?)),
            Some(dsl::FieldValue::Enum(12)) => Ok(Self::FixedConstraint(geometric()?)),
            Some(dsl::FieldValue::Enum(13)) => Ok(Self::VerticalConstraint(DwgAxisConstraint::from_value(record.get(5).ok_or("vertical constraint missing")?)?)),
            other => Err(format!("unknown constraint-node kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
                    ("point".into(), 8),
                    ("circle".into(), 9),
                    ("ellipse".into(), 10),
                    ("text".into(), 11),
                    ("spline".into(), 12),
                    ("face3d".into(), 13),
                    ("polyline3d".into(), 14),
                    ("polyfaceMesh".into(), 15),
                    ("vertex".into(), 16),
                    ("polyfaceFace".into(), 17),
                    ("sequenceEnd".into(), 18),
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
            dsl::FieldSpec::new(9, "point", <DwgPointEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(10, "circle", <DwgCircleEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(11, "ellipse", <DwgEllipseEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(12, "text", <DwgTextEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(13, "spline", <DwgSplineEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(14, "face3d", <DwgFace3dEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(15, "polyline3d", <DwgPolyline3dEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(16, "polyface_mesh", <DwgPolyfaceMeshEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(17, "vertex", <DwgVertexEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(18, "polyface_face", <DwgPolyfaceFaceEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(19, "sequence_end", <DwgSequenceEndEntity as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for DwgEntityBody {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_entity_body_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        match self {
            Self::Line(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(0));
                record.fields.insert(1, <DwgLineEntity as dsl::DslField>::to_value(value));
            }
            Self::Arc(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(1));
                record.fields.insert(2, <DwgArcEntity as dsl::DslField>::to_value(value));
            }
            Self::LwPolyline(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(2));
                record.fields.insert(3, <DwgLwPolylineEntity as dsl::DslField>::to_value(value));
            }
            Self::BlockBegin(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(3));
                record.fields.insert(4, <DwgBlockBeginEntity as dsl::DslField>::to_value(value));
            }
            Self::BlockEnd(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(4));
                record.fields.insert(5, <DwgBlockEndEntity as dsl::DslField>::to_value(value));
            }
            Self::Insert(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(5));
                record.fields.insert(6, <DwgInsertEntity as dsl::DslField>::to_value(value));
            }
            Self::DimensionLinear(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(6));
                record.fields.insert(7, <DwgLinearDimensionEntity as dsl::DslField>::to_value(value));
            }
            Self::Viewport(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(7));
                record.fields.insert(8, <DwgViewportEntity as dsl::DslField>::to_value(value));
            }
            Self::Point(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(8));
                record.fields.insert(9, <DwgPointEntity as dsl::DslField>::to_value(value));
            }
            Self::Circle(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(9));
                record.fields.insert(10, <DwgCircleEntity as dsl::DslField>::to_value(value));
            }
            Self::Ellipse(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(10));
                record.fields.insert(11, <DwgEllipseEntity as dsl::DslField>::to_value(value));
            }
            Self::Text(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(11));
                record.fields.insert(12, <DwgTextEntity as dsl::DslField>::to_value(value));
            }
            Self::Spline(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(12));
                record.fields.insert(13, <DwgSplineEntity as dsl::DslField>::to_value(value));
            }
            Self::Face3d(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(13));
                record.fields.insert(14, <DwgFace3dEntity as dsl::DslField>::to_value(value));
            }
            Self::Polyline3d(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(14));
                record.fields.insert(15, <DwgPolyline3dEntity as dsl::DslField>::to_value(value));
            }
            Self::PolyfaceMesh(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(15));
                record.fields.insert(16, <DwgPolyfaceMeshEntity as dsl::DslField>::to_value(value));
            }
            Self::Vertex(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(16));
                record.fields.insert(17, <DwgVertexEntity as dsl::DslField>::to_value(value));
            }
            Self::PolyfaceFace(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(17));
                record.fields.insert(18, <DwgPolyfaceFaceEntity as dsl::DslField>::to_value(value));
            }
            Self::SequenceEnd(value) => {
                record.fields.insert(0, dsl::FieldValue::Enum(18));
                record.fields.insert(19, <DwgSequenceEndEntity as dsl::DslField>::to_value(value));
            }
        }
        dsl::FieldValue::Record(record)
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else { return Err("expected entity-body record".into()) };
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Line(<DwgLineEntity as dsl::DslField>::from_value(record.get(1).ok_or("LINE body missing")?)?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::Arc(<DwgArcEntity as dsl::DslField>::from_value(record.get(2).ok_or("ARC body missing")?)?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::LwPolyline(<DwgLwPolylineEntity as dsl::DslField>::from_value(record.get(3).ok_or("LWPOLYLINE body missing")?)?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::BlockBegin(<DwgBlockBeginEntity as dsl::DslField>::from_value(record.get(4).ok_or("BLOCK body missing")?)?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::BlockEnd(<DwgBlockEndEntity as dsl::DslField>::from_value(record.get(5).ok_or("ENDBLK body missing")?)?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::Insert(<DwgInsertEntity as dsl::DslField>::from_value(record.get(6).ok_or("INSERT body missing")?)?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::DimensionLinear(<DwgLinearDimensionEntity as dsl::DslField>::from_value(record.get(7).ok_or("DIMENSION_LINEAR body missing")?)?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::Viewport(<DwgViewportEntity as dsl::DslField>::from_value(record.get(8).ok_or("VIEWPORT body missing")?)?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::Point(<DwgPointEntity as dsl::DslField>::from_value(record.get(9).ok_or("POINT body missing")?)?)),
            Some(dsl::FieldValue::Enum(9)) => Ok(Self::Circle(<DwgCircleEntity as dsl::DslField>::from_value(record.get(10).ok_or("CIRCLE body missing")?)?)),
            Some(dsl::FieldValue::Enum(10)) => Ok(Self::Ellipse(<DwgEllipseEntity as dsl::DslField>::from_value(record.get(11).ok_or("ELLIPSE body missing")?)?)),
            Some(dsl::FieldValue::Enum(11)) => Ok(Self::Text(<DwgTextEntity as dsl::DslField>::from_value(record.get(12).ok_or("TEXT body missing")?)?)),
            Some(dsl::FieldValue::Enum(12)) => Ok(Self::Spline(<DwgSplineEntity as dsl::DslField>::from_value(record.get(13).ok_or("SPLINE body missing")?)?)),
            Some(dsl::FieldValue::Enum(13)) => Ok(Self::Face3d(<DwgFace3dEntity as dsl::DslField>::from_value(record.get(14).ok_or("3DFACE body missing")?)?)),
            Some(dsl::FieldValue::Enum(14)) => Ok(Self::Polyline3d(<DwgPolyline3dEntity as dsl::DslField>::from_value(record.get(15).ok_or("POLYLINE_3D body missing")?)?)),
            Some(dsl::FieldValue::Enum(15)) => Ok(Self::PolyfaceMesh(<DwgPolyfaceMeshEntity as dsl::DslField>::from_value(record.get(16).ok_or("POLYLINE_PFACE body missing")?)?)),
            Some(dsl::FieldValue::Enum(16)) => Ok(Self::Vertex(<DwgVertexEntity as dsl::DslField>::from_value(record.get(17).ok_or("VERTEX body missing")?)?)),
            Some(dsl::FieldValue::Enum(17)) => Ok(Self::PolyfaceFace(<DwgPolyfaceFaceEntity as dsl::DslField>::from_value(record.get(18).ok_or("VERTEX_PFACE_FACE body missing")?)?)),
            Some(dsl::FieldValue::Enum(18)) => Ok(Self::SequenceEnd(<DwgSequenceEndEntity as dsl::DslField>::from_value(record.get(19).ok_or("SEQEND body missing")?)?)),
            other => Err(format!("unknown entity-body kind {other:?}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
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
    TableStyle(Box<DwgTableStyle>),
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
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(dwg_logical_object_body_spec)
    }

    fn to_value(&self) -> dsl::FieldValue {
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

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else {
            return Err(format!("expected DWG object-body record, found {value:?}"));
        };
        if record.fields.values().filter(|value| !matches!(value, dsl::FieldValue::Absent)).count() != 2 {
            return Err("DWG object body must contain exactly one tagged payload".into());
        }
        match record.get(0) {
            Some(dsl::FieldValue::Enum(0)) => Ok(Self::Dictionary(<DwgDictionaryBody as dsl::DslField>::from_value(record.get(1).ok_or("dictionary body missing")?)?)),
            Some(dsl::FieldValue::Enum(1)) => Ok(Self::TableControl(<DwgTableControlBody as dsl::DslField>::from_value(record.get(2).ok_or("table-control body missing")?)?)),
            Some(dsl::FieldValue::Enum(2)) => Ok(Self::TableRecord(<DwgTableRecordBody as dsl::DslField>::from_value(record.get(3).ok_or("table-record body missing")?)?)),
            Some(dsl::FieldValue::Enum(3)) => Ok(Self::XRecord(<DwgXRecordBody as dsl::DslField>::from_value(record.get(4).ok_or("XRECORD body missing")?)?)),
            Some(dsl::FieldValue::Enum(4)) => Ok(Self::Entity(<DwgEntityBody as dsl::DslField>::from_value(record.get(5).ok_or("entity body missing")?)?)),
            Some(dsl::FieldValue::Enum(5)) => Ok(Self::AssociativeDependency(<DwgAssociativeDependency as dsl::DslField>::from_value(record.get(6).ok_or("associative-dependency body missing")?)?)),
            Some(dsl::FieldValue::Enum(6)) => Ok(Self::AssociativeValueDependency(<DwgAssociativeValueDependency as dsl::DslField>::from_value(record.get(7).ok_or("associative-value-dependency body missing")?)?)),
            Some(dsl::FieldValue::Enum(7)) => Ok(Self::AssociativeGeometryDependency(<DwgAssociativeGeometryDependency as dsl::DslField>::from_value(record.get(8).ok_or("associative-geometry-dependency body missing")?)?)),
            Some(dsl::FieldValue::Enum(8)) => Ok(Self::BlockGripLocationComponent(<DwgBlockGripLocationComponent as dsl::DslField>::from_value(record.get(9).ok_or("block-grip-location-component body missing")?)?)),
            Some(dsl::FieldValue::Enum(9)) => Ok(Self::DynamicBlockProxyNode(<DwgDynamicBlockProxyNode as dsl::DslField>::from_value(record.get(10).ok_or("dynamic-block-proxy-node body missing")?)?)),
            Some(dsl::FieldValue::Enum(10)) => Ok(Self::AssociativeVariable(<DwgAssociativeVariable as dsl::DslField>::from_value(record.get(11).ok_or("associative-variable body missing")?)?)),
            Some(dsl::FieldValue::Enum(11)) => Ok(Self::AssociativeDimensionDependencyBody(<DwgAssociativeDimensionDependencyBody as dsl::DslField>::from_value(record.get(12).ok_or("associative-dimension-dependency body missing")?)?)),
            Some(dsl::FieldValue::Enum(12)) => Ok(Self::VisualStyle(<DwgVisualStyle as dsl::DslField>::from_value(record.get(13).ok_or("visual-style body missing")?)?)),
            Some(dsl::FieldValue::Enum(13)) => Ok(Self::BlockParameterDependencyBody(<DwgBlockParameterDependencyBody as dsl::DslField>::from_value(record.get(14).ok_or("block-parameter-dependency body missing")?)?)),
            Some(dsl::FieldValue::Enum(14)) => Ok(Self::BlockRepresentationData(<DwgBlockRepresentationData as dsl::DslField>::from_value(record.get(15).ok_or("block-representation data missing")?)?)),
            Some(dsl::FieldValue::Enum(15)) => Ok(Self::DynamicBlockPurgePreventer(<DwgDynamicBlockPurgePreventer as dsl::DslField>::from_value(record.get(16).ok_or("dynamic-block purge-preventer body missing")?)?)),
            Some(dsl::FieldValue::Enum(16)) => Ok(Self::EvaluationGraph(<DwgEvaluationGraph as dsl::DslField>::from_value(record.get(17).ok_or("evaluation-graph body missing")?)?)),
            Some(dsl::FieldValue::Enum(17)) => Ok(Self::BlockFlipParameter(<DwgBlockFlipParameter as dsl::DslField>::from_value(record.get(18).ok_or("block-flip-parameter body missing")?)?)),
            Some(dsl::FieldValue::Enum(18)) => Ok(Self::BlockVisibilityParameter(<DwgBlockVisibilityParameter as dsl::DslField>::from_value(record.get(19).ok_or("block-visibility-parameter body missing")?)?)),
            Some(dsl::FieldValue::Enum(19)) => Ok(Self::Placeholder(<DwgPlaceholder as dsl::DslField>::from_value(record.get(20).ok_or("placeholder body missing")?)?)),
            Some(dsl::FieldValue::Enum(20)) => Ok(Self::DictionaryVariable(<DwgDictionaryVariable as dsl::DslField>::from_value(record.get(21).ok_or("dictionary-variable body missing")?)?)),
            Some(dsl::FieldValue::Enum(21)) => Ok(Self::AnnotationScale(<DwgAnnotationScale as dsl::DslField>::from_value(record.get(22).ok_or("annotation-scale body missing")?)?)),
            Some(dsl::FieldValue::Enum(22)) => Ok(Self::SortEntitiesTable(<DwgSortEntitiesTable as dsl::DslField>::from_value(record.get(23).ok_or("sort-entities-table body missing")?)?)),
            Some(dsl::FieldValue::Enum(23)) => Ok(Self::TableStyle(Box::new(<DwgTableStyle as dsl::DslField>::from_value(record.get(24).ok_or("table-style body missing")?)?))),
            Some(dsl::FieldValue::Enum(24)) => Ok(Self::MlineStyle(<DwgMlineStyle as dsl::DslField>::from_value(record.get(25).ok_or("MLINESTYLE body missing")?)?)),
            Some(dsl::FieldValue::Enum(25)) => Ok(Self::MLeaderStyle(<DwgMLeaderStyle as dsl::DslField>::from_value(record.get(26).ok_or("MLEADERSTYLE body missing")?)?)),
            Some(dsl::FieldValue::Enum(26)) => Ok(Self::Material(<DwgMaterial as dsl::DslField>::from_value(record.get(27).ok_or("MATERIAL body missing")?)?)),
            Some(dsl::FieldValue::Enum(27)) => Ok(Self::BlockMoveAction(<DwgBlockMoveAction as dsl::DslField>::from_value(record.get(28).ok_or("BLOCKMOVEACTION body missing")?)?)),
            Some(dsl::FieldValue::Enum(28)) => Ok(Self::AssocNetwork(<DwgAssocNetwork as dsl::DslField>::from_value(record.get(29).ok_or("ACDBASSOCNETWORK body missing")?)?)),
            Some(dsl::FieldValue::Enum(29)) => Ok(Self::Assoc2dConstraintGroup(<DwgAssoc2dConstraintGroup as dsl::DslField>::from_value(record.get(30).ok_or("ACDBASSOC2DCONSTRAINTGROUP body missing")?)?)),
            Some(dsl::FieldValue::Enum(30)) => Ok(Self::BlockLinearParameter(<DwgBlockLinearParameter as dsl::DslField>::from_value(record.get(31).ok_or("BLOCKLINEARPARAMETER body missing")?)?)),
            Some(dsl::FieldValue::Enum(31)) => Ok(Self::BlockLinearGrip(<DwgBlockLinearGrip as dsl::DslField>::from_value(record.get(32).ok_or("BLOCKLINEARGRIP body missing")?)?)),
            Some(dsl::FieldValue::Enum(32)) => Ok(Self::BlockFlipGrip(<DwgBlockFlipGrip as dsl::DslField>::from_value(record.get(33).ok_or("BLOCKFLIPGRIP body missing")?)?)),
            Some(dsl::FieldValue::Enum(33)) => Ok(Self::BlockVisibilityGrip(<DwgBlockVisibilityGrip as dsl::DslField>::from_value(record.get(34).ok_or("BLOCKVISIBILITYGRIP body missing")?)?)),
            Some(dsl::FieldValue::Enum(34)) => Ok(Self::BlockAlignmentParameter(<DwgBlockAlignmentParameter as dsl::DslField>::from_value(record.get(35).ok_or("BLOCKALIGNMENTPARAMETER body missing")?)?)),
            Some(dsl::FieldValue::Enum(35)) => Ok(Self::BlockAlignmentGrip(<DwgBlockAlignmentGrip as dsl::DslField>::from_value(record.get(36).ok_or("BLOCKALIGNMENTGRIP body missing")?)?)),
            Some(dsl::FieldValue::Enum(36)) => Ok(Self::BlockStretchAction(<DwgBlockStretchAction as dsl::DslField>::from_value(record.get(37).ok_or("BLOCKSTRETCHACTION body missing")?)?)),
            Some(dsl::FieldValue::Enum(37)) => Ok(Self::BlockScaleAction(<DwgBlockScaleAction as dsl::DslField>::from_value(record.get(38).ok_or("BLOCKSCALEACTION body missing")?)?)),
            Some(dsl::FieldValue::Enum(38)) => Ok(Self::BlockFlipAction(<DwgBlockFlipAction as dsl::DslField>::from_value(record.get(39).ok_or("BLOCKFLIPACTION body missing")?)?)),
            Some(dsl::FieldValue::Enum(39)) => Ok(Self::BlockBasePointParameter(<DwgBlockBasePointParameter as dsl::DslField>::from_value(record.get(40).ok_or("BLOCKBASEPOINTPARAMETER body missing")?)?)),
            Some(dsl::FieldValue::Enum(40)) => Ok(Self::BlockVerticalConstraintParameter(<DwgBlockLinearConstraintParameter as dsl::DslField>::from_value(record.get(41).ok_or("BLOCKVERTICALCONSTRAINTPARAMETER body missing")?)?)),
            Some(dsl::FieldValue::Enum(41)) => Ok(Self::BlockHorizontalConstraintParameter(<DwgBlockLinearConstraintParameter as dsl::DslField>::from_value(record.get(42).ok_or("BLOCKHORIZONTALCONSTRAINTPARAMETER body missing")?)?)),
            Some(dsl::FieldValue::Enum(42)) => Ok(Self::Layout(<DwgLayout as dsl::DslField>::from_value(record.get(43).ok_or("LAYOUT body missing")?)?)),
            other => Err(format!("expected DWG object-body kind, found {other:?}")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalObject {
    pub handle: u64,
    pub type_code: u16,
    pub class_name: String,
    pub category: DwgObjectCategory,
    #[value(default)]
    pub owner_handle: Option<u64>,
    #[value(default)]
    pub reactor_handles: Vec<u64>,
    #[value(default)]
    pub extension_dictionary_handle: Option<u64>,
    #[value(default)]
    pub referenced_handles: Vec<u64>,
    #[value(default)]
    pub extended_data: Vec<DwgExtendedEntityData>,
    #[value(default)]
    pub body: Option<DwgLogicalObjectBody>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalDrawing {
    /// 🧭 Handle-keyed objects are the sole persisted entity authority; use `entities()` for a derived view.
    #[value(default)]
    pub layers: Vec<DwgLogicalLayer>,
    #[value(default)]
    pub objects: Vec<DwgLogicalObject>,
    #[value(default)]
    pub extmin: Vec<f64>,
    #[value(default)]
    pub extmax: Vec<f64>,
}

impl DwgLogicalDrawing {
    /// 🏗️ The complete AC1024 object graph of a new document carrying `drawing`: the nine symbol-table
    /// controls with their standard records (layer `0` plus the drawing's layers, the ByBlock/ByLayer/
    /// Continuous linetypes, the Standard text/dimension styles, the ACAD application, the `*Active`
    /// viewport), the model- and paper-space block records with their BLOCK/ENDBLK pairs and layouts,
    /// the named-object dictionary with the dictionaries AutoCAD requires, and every entity owned by
    /// model space. Handles follow [`new_document_handles`]; entities start at its `FIRST_FREE`.
    /// See <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> §20.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_native(drawing: &dwg_engine::DwgDrawing) -> Result<Self, String> {
        use new_document_handles as h;
        let mut layers = drawing.layers.clone();
        if !layers.iter().any(|layer| layer.name == "0") {
            layers.push(dwg_engine::DwgLayer::default());
        }
        let mut next = h::FIRST_FREE;
        let mut allocate = || {
            let handle = next;
            next += 1;
            handle
        };
        let layer_handles: Vec<u64> = layers.iter().map(|_| allocate()).collect();
        let layer_zero = layers.iter().zip(&layer_handles).find_map(|(layer, handle)| (layer.name == "0").then_some(*handle)).ok_or("layer 0 is missing")?;
        let common = |layer: usize, color: dwg_engine::DwgColor, mode: DwgEntityMode| DwgEntityCommon {
            mode,
            color: match color {
                dwg_engine::DwgColor::ByLayer => DwgEntityColor { kind: DwgEntityColorKind::ByLayer, index: 256, ..Default::default() },
                dwg_engine::DwgColor::ByBlock => DwgEntityColor { kind: DwgEntityColorKind::ByBlock, index: 0, ..Default::default() },
                dwg_engine::DwgColor::Index(index) => DwgEntityColor { kind: DwgEntityColorKind::Index, index: u16::from(index), ..Default::default() },
            },
            linetype_scale: 1.0,
            lineweight: 29,
            layer_handle: layer_handles[layer],
            ..Default::default()
        };
        let entity = |handle: u64, type_code: u16, class_name: &str, owner: Option<u64>, body: DwgEntityBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::Entity,
            owner_handle: owner,
            body: Some(DwgLogicalObjectBody::Entity(body)),
            ..Default::default()
        };
        let mut entity_objects = Vec::new();
        let mut model_space_entities = Vec::new();
        for (index, source) in drawing.entities.iter().enumerate() {
            let layer = Some(source.layer).filter(|layer| *layer < drawing.layers.len()).ok_or_else(|| format!("DWG entity {index} references missing layer index {}", source.layer))?;
            let top = common(layer, source.color, DwgEntityMode::ModelSpace);
            let owned = common(layer, source.color, DwgEntityMode::ExplicitOwner);
            let handle = allocate();
            model_space_entities.push(handle);
            let z = [0.0, 0.0, 1.0].to_vec();
            match &source.geometry {
                dwg_engine::DwgGeometry::Line { start, end } => entity_objects.push(entity(handle, 19, "LINE", None, DwgEntityBody::Line(DwgLineEntity { common: top, start: start.to_vec(), end: end.to_vec(), thickness: 0.0, extrusion: z }))),
                dwg_engine::DwgGeometry::Point { at } => entity_objects.push(entity(handle, 27, "POINT", None, DwgEntityBody::Point(DwgPointEntity { common: top, point: at.to_vec(), thickness: 0.0, extrusion: z, x_axis_angle: 0.0 }))),
                dwg_engine::DwgGeometry::Circle { center, radius, normal } => entity_objects.push(entity(handle, 18, "CIRCLE", None, DwgEntityBody::Circle(DwgCircleEntity { common: top, center: center.to_vec(), radius: *radius, thickness: 0.0, extrusion: normal.to_vec() }))),
                dwg_engine::DwgGeometry::Arc { center, radius, start_angle, end_angle, normal } => entity_objects.push(entity(
                    handle,
                    17,
                    "ARC",
                    None,
                    DwgEntityBody::Arc(DwgArcEntity { common: top, center: center.to_vec(), radius: *radius, thickness: 0.0, extrusion: normal.to_vec(), start_angle: *start_angle, end_angle: *end_angle }),
                )),
                dwg_engine::DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal } => entity_objects.push(entity(
                    handle,
                    35,
                    "ELLIPSE",
                    None,
                    DwgEntityBody::Ellipse(DwgEllipseEntity { common: top, center: center.to_vec(), major_axis: major_axis.to_vec(), extrusion: normal.to_vec(), axis_ratio: *ratio, start_parameter: *start_param, end_parameter: *end_param }),
                )),
                dwg_engine::DwgGeometry::LwPolyline { closed, elevation, vertices, bulges } => entity_objects.push(entity(
                    handle,
                    77,
                    "LWPOLYLINE",
                    None,
                    DwgEntityBody::LwPolyline(DwgLwPolylineEntity {
                        common: top,
                        closed: *closed,
                        elevation: *elevation,
                        thickness: 0.0,
                        extrusion: z,
                        vertices: vertices.iter().enumerate().map(|(vertex, point)| DwgLwPolylineVertex { point: point.to_vec(), bulge: bulges.get(vertex).copied().unwrap_or_default(), ..Default::default() }).collect(),
                        ..Default::default()
                    }),
                )),
                dwg_engine::DwgGeometry::Spline { degree, control_points, knots, weights } => entity_objects.push(entity(
                    handle,
                    36,
                    "SPLINE",
                    None,
                    DwgEntityBody::Spline(DwgSplineEntity {
                        common: top,
                        degree: *degree,
                        rational: !weights.is_empty(),
                        closed: false,
                        periodic: false,
                        knot_tolerance: 1e-10,
                        control_tolerance: 1e-10,
                        knots: knots.clone(),
                        control_points: control_points.iter().flatten().copied().collect(),
                        weights: weights.clone(),
                    }),
                )),
                dwg_engine::DwgGeometry::Text { at, height, rotation, content } => entity_objects.push(entity(
                    handle,
                    1,
                    "TEXT",
                    None,
                    DwgEntityBody::Text(DwgTextEntity {
                        common: top,
                        elevation: at[2],
                        insertion: vec![at[0], at[1]],
                        alignment: None,
                        extrusion: z,
                        thickness: 0.0,
                        oblique_angle: 0.0,
                        rotation: *rotation,
                        height: *height,
                        width_factor: 1.0,
                        value: content.clone(),
                        generation: 0,
                        horizontal_alignment: 0,
                        vertical_alignment: 0,
                        style_handle: h::STANDARD_TEXT_STYLE,
                    }),
                )),
                dwg_engine::DwgGeometry::Face3d { corners } => entity_objects.push(entity(handle, 28, "3DFACE", None, DwgEntityBody::Face3d(DwgFace3dEntity { common: top, corners: corners.iter().flatten().copied().collect(), invisible_edges: 0 }))),
                dwg_engine::DwgGeometry::Polyline3d { closed, vertices } => {
                    let vertex_handles: Vec<u64> = vertices.iter().map(|_| allocate()).collect();
                    let sequence_end_handle = allocate();
                    entity_objects.push(entity(handle, 16, "POLYLINE_3D", None, DwgEntityBody::Polyline3d(DwgPolyline3dEntity { common: top, curve_type: 0, flags: u8::from(*closed), vertex_handles: vertex_handles.clone(), sequence_end_handle })));
                    for (vertex, point) in vertex_handles.iter().zip(vertices) {
                        entity_objects.push(entity(*vertex, 11, "VERTEX_3D", Some(handle), DwgEntityBody::Vertex(DwgVertexEntity { common: owned.clone(), flags: 32, point: point.to_vec() })));
                    }
                    entity_objects.push(entity(sequence_end_handle, 6, "SEQEND", Some(handle), DwgEntityBody::SequenceEnd(DwgSequenceEndEntity { common: owned })));
                }
                dwg_engine::DwgGeometry::PolyfaceMesh { vertices, faces } => {
                    let vertex_count = u16::try_from(vertices.len()).map_err(|_| format!("DWG polyface mesh {index} exceeds {} vertices", u16::MAX))?;
                    let face_count = u16::try_from(faces.len()).map_err(|_| format!("DWG polyface mesh {index} exceeds {} faces", u16::MAX))?;
                    let vertex_handles: Vec<u64> = vertices.iter().chain(faces.iter().map(|_| &[0.0; 3])).map(|_| allocate()).collect();
                    let sequence_end_handle = allocate();
                    entity_objects.push(entity(handle, 29, "POLYLINE_PFACE", None, DwgEntityBody::PolyfaceMesh(DwgPolyfaceMeshEntity { common: top, vertex_count, face_count, vertex_handles: vertex_handles.clone(), sequence_end_handle })));
                    for (vertex, point) in vertex_handles.iter().zip(vertices) {
                        entity_objects.push(entity(*vertex, 13, "VERTEX_PFACE", Some(handle), DwgEntityBody::Vertex(DwgVertexEntity { common: owned.clone(), flags: 192, point: point.to_vec() })));
                    }
                    for (face_handle, face) in vertex_handles[vertices.len()..].iter().zip(faces) {
                        let indices = face.iter().map(|value| i16::try_from(*value).map_err(|_| format!("DWG polyface mesh {index} face index {value} exceeds i16"))).collect::<Result<Vec<_>, _>>()?;
                        entity_objects.push(entity(*face_handle, 14, "VERTEX_PFACE_FACE", Some(handle), DwgEntityBody::PolyfaceFace(DwgPolyfaceFaceEntity { common: owned.clone(), indices })));
                    }
                    entity_objects.push(entity(sequence_end_handle, 6, "SEQEND", Some(handle), DwgEntityBody::SequenceEnd(DwgSequenceEndEntity { common: owned })));
                }
            }
        }
        let record = |handle: u64, type_code: u16, class_name: &str, owner: u64, body: DwgTableRecordBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::TableRecord,
            owner_handle: Some(owner),
            body: Some(DwgLogicalObjectBody::TableRecord(body)),
            ..Default::default()
        };
        let control = |handle: u64, type_code: u16, class_name: &str, body: DwgTableControlBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::TableControl,
            body: Some(DwgLogicalObjectBody::TableControl(body)),
            ..Default::default()
        };
        let entries = |handles: &[u64]| DwgTableControlEntries { entry_handles: handles.iter().map(|handle| DwgTableControlEntry { handle: Some(*handle) }).collect() };
        let dictionary = |handle: u64, owner: Option<u64>, entries: &[(&str, u64)]| DwgLogicalObject {
            handle,
            type_code: 42,
            class_name: "DICTIONARY".into(),
            category: DwgObjectCategory::Dictionary,
            owner_handle: owner,
            reactor_handles: owner.into_iter().collect(),
            body: Some(DwgLogicalObjectBody::Dictionary(DwgDictionaryBody { entries: entries.iter().map(|(name, handle)| DwgNamedReference { name: (*name).into(), handle: *handle }).collect(), cloning_flag: 1, hard_owner: false, default_entry_handle: None })),
            ..Default::default()
        };
        let owned_object = |handle: u64, type_code: u16, class_name: &str, owner: u64, extension_dictionary: Option<u64>, body: DwgLogicalObjectBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::Object,
            owner_handle: Some(owner),
            reactor_handles: vec![owner],
            extension_dictionary_handle: extension_dictionary,
            body: Some(body),
            ..Default::default()
        };
        let block_common = |mode: DwgEntityMode| DwgEntityCommon { mode, linetype_scale: 1.0, lineweight: 29, layer_handle: layer_zero, color: DwgEntityColor { kind: DwgEntityColorKind::ByLayer, index: 256, ..Default::default() }, ..Default::default() };
        let block_header = |name: &str, owned_entity_handles: Vec<u64>, block_entity_handle: u64, end_block_entity_handle: u64, layout: u64| DwgBlockHeaderTableRecord {
            common: DwgTableRecordCommon { name: name.into(), ..Default::default() },
            owned_entity_handles,
            explodable: true,
            block_entity_handle,
            end_block_entity_handle,
            layout_handle: Some(layout),
            ..Default::default()
        };
        let layout = |name: &str, model: bool, block_header_handle: u64, active_viewport_handle: Option<u64>| DwgLayout {
            page_setup_name: String::new(),
            printer_configuration: "none_device".into(),
            canonical_media_name: "ANSI_A_(8.50_x_11.00_Inches)".into(),
            stylesheet: String::new(),
            name: name.into(),
            plot_options: DwgPlotOptions { use_standard_scale: true, plot_viewport_borders: true, plot_with_lineweights: true, draw_viewports_first: true, model_type: model, update_paper: false, initializing: false },
            margins: vec![6.35, 6.35, 6.35, 6.35],
            paper_size: vec![215.9, 279.4],
            plot_origin: vec![0.0, 0.0],
            paper_unit: DwgPlotPaperUnit::Inches,
            rotation: DwgPlotRotation::QuarterTurn,
            plot_area: if model { DwgPlotArea::Display } else { DwgPlotArea::Layout },
            plot_window_lower_left: vec![0.0, 0.0],
            plot_window_upper_right: vec![0.0, 0.0],
            paper_units: 1.0,
            drawing_units: 1.0,
            standard_scale: DwgStandardScale::OneToOne,
            standard_scale_factor: 1.0,
            paper_image_origin: vec![0.0, 0.0],
            shade_plot: DwgShadePlot::AsDisplayed,
            shade_plot_resolution: DwgShadePlotResolution::Normal,
            shade_plot_dpi: 300,
            tab_order: u16::from(!model),
            options: DwgLayoutOptions { paper_space_linetype_scaling: true },
            insertion_base: vec![0.0; 3],
            limits_minimum: vec![0.0, 0.0],
            limits_maximum: vec![12.0, 9.0],
            ucs_origin: vec![0.0; 3],
            ucs_x_axis: vec![1.0, 0.0, 0.0],
            ucs_y_axis: vec![0.0, 1.0, 0.0],
            ucs_elevation: 0.0,
            orthographic_view: DwgOrthographicView::None,
            extents_minimum: vec![0.0; 3],
            extents_maximum: vec![0.0; 3],
            plot_view_handle: None,
            visual_style_handle: None,
            block_header_handle,
            active_viewport_handle,
            base_ucs_handle: None,
            named_ucs_handle: None,
            viewport_handles: Vec::new(),
        };
        let by_block = DwgComplexColor { index: 0, value: DwgComplexColorValue::ByBlock, ..Default::default() };
        let linetype = |name: &str, description: &str| DwgLinetypeTableRecord { common: DwgTableRecordCommon { name: name.into(), ..Default::default() }, description: description.into(), pattern_length: 0.0, alignment: b'A', dashes: Vec::new() };
        let extent_width = (drawing.extmax[0] - drawing.extmin[0]).abs().max(1.0);
        let extent_height = (drawing.extmax[1] - drawing.extmin[1]).abs().max(1.0);
        let mut objects = vec![
            control(h::BLOCK_CONTROL, 48, "BLOCK_CONTROL", DwgTableControlBody::Block(DwgBlockTableControl { entry_handles: Vec::new(), model_space_handle: Some(h::MODEL_SPACE), paper_space_handle: Some(h::PAPER_SPACE) })),
            control(h::LAYER_CONTROL, 50, "LAYER_CONTROL", DwgTableControlBody::Layer(entries(&layer_handles))),
            control(h::STYLE_CONTROL, 52, "STYLE_CONTROL", DwgTableControlBody::TextStyle(entries(&[h::STANDARD_TEXT_STYLE]))),
            control(h::LINETYPE_CONTROL, 56, "LTYPE_CONTROL", DwgTableControlBody::Linetype(DwgLinetypeTableControl { entry_handles: entries(&[h::CONTINUOUS]).entry_handles, by_block_handle: h::BY_BLOCK, by_layer_handle: h::BY_LAYER })),
            control(h::VIEW_CONTROL, 60, "VIEW_CONTROL", DwgTableControlBody::View(entries(&[]))),
            control(h::UCS_CONTROL, 62, "UCS_CONTROL", DwgTableControlBody::Ucs(entries(&[]))),
            control(h::VIEWPORT_CONTROL, 64, "VPORT_CONTROL", DwgTableControlBody::Viewport(entries(&[h::ACTIVE_VIEWPORT]))),
            control(h::APPID_CONTROL, 66, "APPID_CONTROL", DwgTableControlBody::RegisteredApplication(entries(&[h::ACAD_APPLICATION]))),
            control(h::DIMSTYLE_CONTROL, 68, "DIMSTYLE_CONTROL", DwgTableControlBody::DimensionStyle(DwgDimensionStyleTableControl { entry_handles: entries(&[h::STANDARD_DIMSTYLE]).entry_handles, additional_handles: Vec::new() })),
            dictionary(
                h::NAMED_OBJECTS,
                None,
                &[
                    ("ACAD_COLOR", h::COLOR_DICTIONARY),
                    ("ACAD_GROUP", h::GROUP_DICTIONARY),
                    ("ACAD_LAYOUT", h::LAYOUT_DICTIONARY),
                    ("ACAD_MATERIAL", h::MATERIAL_DICTIONARY),
                    ("ACAD_MLINESTYLE", h::MLINE_STYLE_DICTIONARY),
                    ("ACAD_PLOTSETTINGS", h::PLOT_SETTINGS_DICTIONARY),
                    ("ACAD_PLOTSTYLENAME", h::PLOT_STYLE_NAME_DICTIONARY),
                    ("ACAD_VISUALSTYLE", h::VISUAL_STYLE_DICTIONARY),
                ],
            ),
            dictionary(h::GROUP_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            DwgLogicalObject {
                handle: h::PLOT_STYLE_NAME_DICTIONARY,
                type_code: 500,
                class_name: "ACDBDICTIONARYWDFLT".into(),
                category: DwgObjectCategory::Custom,
                owner_handle: Some(h::NAMED_OBJECTS),
                reactor_handles: vec![h::NAMED_OBJECTS],
                body: Some(DwgLogicalObjectBody::Dictionary(DwgDictionaryBody {
                    entries: vec![DwgNamedReference { name: "Normal".into(), handle: h::NORMAL_PLOT_STYLE }],
                    cloning_flag: 1,
                    hard_owner: false,
                    default_entry_handle: Some(h::NORMAL_PLOT_STYLE),
                })),
                ..Default::default()
            },
            owned_object(h::NORMAL_PLOT_STYLE, 80, "ACDBPLACEHOLDER", h::PLOT_STYLE_NAME_DICTIONARY, None, DwgLogicalObjectBody::Placeholder(DwgPlaceholder {})),
            record(h::STANDARD_TEXT_STYLE, 53, "STYLE", h::STYLE_CONTROL, DwgTableRecordBody::TextStyle(DwgTextStyleTableRecord { common: DwgTableRecordCommon { name: "Standard".into(), ..Default::default() }, width_factor: 1.0, last_height: 0.2, font_file: "txt".into(), ..Default::default() })),
            record(h::ACAD_APPLICATION, 67, "APPID", h::APPID_CONTROL, DwgTableRecordBody::RegisteredApplication(DwgRegisteredApplicationTableRecord { common: DwgTableRecordCommon { name: "ACAD".into(), ..Default::default() }, group_71: 0 })),
            record(h::BY_BLOCK, 57, "LTYPE", h::LINETYPE_CONTROL, DwgTableRecordBody::Linetype(linetype("ByBlock", ""))),
            record(h::BY_LAYER, 57, "LTYPE", h::LINETYPE_CONTROL, DwgTableRecordBody::Linetype(linetype("ByLayer", ""))),
            record(h::CONTINUOUS, 57, "LTYPE", h::LINETYPE_CONTROL, DwgTableRecordBody::Linetype(linetype("Continuous", "Solid line"))),
            dictionary(h::MLINE_STYLE_DICTIONARY, Some(h::NAMED_OBJECTS), &[("Standard", h::STANDARD_MLINE_STYLE)]),
            owned_object(
                h::STANDARD_MLINE_STYLE,
                73,
                "MLINESTYLE",
                h::MLINE_STYLE_DICTIONARY,
                None,
                DwgLogicalObjectBody::MlineStyle(DwgMlineStyle {
                    name: "Standard".into(),
                    description: String::new(),
                    fill_enabled: false,
                    display_miters: false,
                    start_caps: DwgMlineCaps::default(),
                    end_caps: DwgMlineCaps::default(),
                    fill_color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByLayer, ..Default::default() },
                    start_angle: std::f64::consts::FRAC_PI_2,
                    end_angle: std::f64::consts::FRAC_PI_2,
                    elements: [0.5, -0.5].into_iter().map(|offset| DwgMlineStyleElement { offset, color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByLayer, ..Default::default() }, linetype: DwgMlineLinetype::ByLayer }).collect(),
                }),
            ),
            dictionary(h::PLOT_SETTINGS_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            dictionary(h::LAYOUT_DICTIONARY, Some(h::NAMED_OBJECTS), &[("Layout1", h::PAPER_LAYOUT), ("Model", h::MODEL_LAYOUT)]),
            record(
                h::STANDARD_DIMSTYLE,
                69,
                "DIMSTYLE",
                h::DIMSTYLE_CONTROL,
                DwgTableRecordBody::DimensionStyle(DwgDimensionStyleTableRecord {
                    common: DwgTableRecordCommon { name: "Standard".into(), ..Default::default() },
                    dimension_postfix: String::new(),
                    alternate_postfix: String::new(),
                    geometry: DwgDimensionGeometry { scale: 1.0, arrow_size: 0.18, extension_origin_offset: 0.0625, dimension_line_increment: 0.38, extension_line_extension: 0.18, rounding: 0.0, dimension_line_extension: 0.0, plus_tolerance: 0.0, minus_tolerance: 0.0, fixed_extension_length: 1.0, jog_angle: std::f64::consts::FRAC_PI_4 },
                    fill_mode: 0,
                    fill_color: by_block.clone(),
                    behavior: DwgDimensionBehavior { text_inside_horizontal: true, text_outside_horizontal: true, ..Default::default() },
                    text: DwgDimensionText {
                        height: 0.18,
                        center_mark_size: 0.09,
                        tick_size: 0.0,
                        alternate_scale: 25.4,
                        linear_scale: 1.0,
                        vertical_position: 0.0,
                        tolerance_scale: 1.0,
                        gap: 0.09,
                        alternate_rounding: 0.0,
                        alternate_enabled: false,
                        alternate_decimals: 2,
                        text_outside_extensions: false,
                        separate_arrowheads: false,
                        force_text_inside: false,
                        suppress_outside_extensions: false,
                        dimension_line_color: by_block.clone(),
                        extension_line_color: by_block.clone(),
                        text_color: by_block.clone(),
                    },
                    units: DwgDimensionUnits { decimal_places: 4, tolerance_decimal_places: 4, alternate_units: 2, alternate_tolerance_decimal_places: 2, linear_units: 2, decimal_separator: 46, tolerance_vertical_alignment: 1, arrow_text_fit: 3, ..Default::default() },
                    r2010: DwgDimensionR2010 { alternate_measurement_factor: 100.0, measurement_factor: 100.0, dimension_lineweight: 0xfffe, extension_lineweight: 0xfffe, ..Default::default() },
                    text_style_handle: Some(h::STANDARD_TEXT_STYLE),
                    ..Default::default()
                }),
            ),
            record(
                h::ACTIVE_VIEWPORT,
                65,
                "VPORT",
                h::VIEWPORT_CONTROL,
                DwgTableRecordBody::Viewport(DwgViewportTableRecord {
                    common: DwgTableRecordCommon { name: "*Active".into(), ..Default::default() },
                    view_height: extent_height,
                    view_width: extent_width,
                    center: [(drawing.extmin[0] + drawing.extmax[0]) / 2.0, (drawing.extmin[1] + drawing.extmax[1]) / 2.0],
                    target: [0.0; 3],
                    direction: [0.0, 0.0, 1.0],
                    twist: 0.0,
                    lens_length: 50.0,
                    front_clipping: 0.0,
                    back_clipping: 0.0,
                    view_mode: [false, false, false, true],
                    render_mode: 0,
                    use_default_lights: true,
                    default_lighting_type: 1,
                    brightness: 0.0,
                    contrast: 0.0,
                    ambient_color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByColor { red: 51, green: 51, blue: 51 }, ..Default::default() },
                    lower_left: [0.0, 0.0],
                    upper_right: [1.0, 1.0],
                    ucs_follow: false,
                    circle_zoom: 1000,
                    fast_zoom: true,
                    ucs_icon: 3,
                    grid_mode: false,
                    grid_unit: [0.5, 0.5],
                    snap_mode: false,
                    snap_style: false,
                    snap_isopair: 0,
                    snap_angle: 0.0,
                    snap_base: [0.0, 0.0],
                    snap_unit: [0.5, 0.5],
                    ucs_at_origin: false,
                    ucs_viewport: true,
                    ucs_origin: [0.0; 3],
                    ucs_x_axis: [1.0, 0.0, 0.0],
                    ucs_y_axis: [0.0, 1.0, 0.0],
                    ucs_elevation: 0.0,
                    ucs_orthographic_view: 0,
                    grid_flags: 2,
                    grid_major: 5,
                    ..Default::default()
                }),
            ),
            dictionary(h::MATERIAL_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            dictionary(h::COLOR_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            dictionary(h::VISUAL_STYLE_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            record(h::MODEL_SPACE, 49, "BLOCK_HEADER", h::BLOCK_CONTROL, DwgTableRecordBody::BlockHeader(block_header("*Model_Space", model_space_entities, h::MODEL_SPACE_BLOCK, h::MODEL_SPACE_END, h::MODEL_LAYOUT))),
            entity(h::MODEL_SPACE_BLOCK, 4, "BLOCK", None, DwgEntityBody::BlockBegin(DwgBlockBeginEntity { common: block_common(DwgEntityMode::ModelSpace) })),
            entity(h::MODEL_SPACE_END, 5, "ENDBLK", None, DwgEntityBody::BlockEnd(DwgBlockEndEntity { common: block_common(DwgEntityMode::ModelSpace) })),
            owned_object(h::MODEL_LAYOUT, 82, "LAYOUT", h::LAYOUT_DICTIONARY, Some(h::MODEL_LAYOUT_DICTIONARY), DwgLogicalObjectBody::Layout(layout("Model", true, h::MODEL_SPACE, Some(h::ACTIVE_VIEWPORT)))),
            dictionary(h::MODEL_LAYOUT_DICTIONARY, Some(h::MODEL_LAYOUT), &[]),
            record(h::PAPER_SPACE, 49, "BLOCK_HEADER", h::BLOCK_CONTROL, DwgTableRecordBody::BlockHeader(block_header("*Paper_Space", Vec::new(), h::PAPER_SPACE_BLOCK, h::PAPER_SPACE_END, h::PAPER_LAYOUT))),
            entity(h::PAPER_SPACE_BLOCK, 4, "BLOCK", None, DwgEntityBody::BlockBegin(DwgBlockBeginEntity { common: block_common(DwgEntityMode::PaperSpace) })),
            entity(h::PAPER_SPACE_END, 5, "ENDBLK", None, DwgEntityBody::BlockEnd(DwgBlockEndEntity { common: block_common(DwgEntityMode::PaperSpace) })),
            owned_object(h::PAPER_LAYOUT, 82, "LAYOUT", h::LAYOUT_DICTIONARY, Some(h::PAPER_LAYOUT_DICTIONARY), DwgLogicalObjectBody::Layout(layout("Layout1", false, h::PAPER_SPACE, None))),
            dictionary(h::PAPER_LAYOUT_DICTIONARY, Some(h::PAPER_LAYOUT), &[]),
        ];
        for (layer, handle) in layers.iter().zip(&layer_handles) {
            objects.push(record(
                *handle,
                51,
                "LAYER",
                h::LAYER_CONTROL,
                DwgTableRecordBody::Layer(DwgLayerTableRecord {
                    common: DwgTableRecordCommon { name: layer.name.clone(), ..Default::default() },
                    plottable: true,
                    lineweight: 31,
                    color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByAci { index: u16::from(layer.color) }, ..Default::default() },
                    plot_style_handle: Some(h::NORMAL_PLOT_STYLE),
                    linetype_handle: Some(h::CONTINUOUS),
                    ..Default::default()
                }),
            ));
        }
        objects.extend(entity_objects);
        objects.sort_by_key(|object| object.handle);
        Ok(Self { layers: layers.iter().map(|layer| DwgLogicalLayer { name: layer.name.clone(), color: layer.color }).collect(), objects, extmin: drawing.extmin.to_vec(), extmax: drawing.extmax.to_vec() })
    }

    /// 0️⃣ The handle of layer `0`, the layer a new document's header makes current.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn layer_zero_handle(&self) -> Result<u64, String> {
        self.objects
            .iter()
            .find_map(|object| match object.body.as_ref() {
                Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(layer))) if layer.common.name == "0" => Some(object.handle),
                _ => None,
            })
            .ok_or_else(|| "layer 0 is missing".into())
    }

    /// 🔢️ The next handle a new object of this drawing would take (the header's `HANDSEED`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn handle_seed(&self) -> u64 {
        self.objects.iter().map(|object| object.handle).max().unwrap_or_default() + 1
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn entities(&self) -> Vec<DwgLogicalEntity> {
        let by_handle: std::collections::HashMap<u64, &DwgEntityBody> = self
            .objects
            .iter()
            .filter_map(|object| match object.body.as_ref() {
                Some(DwgLogicalObjectBody::Entity(body)) => Some((object.handle, body)),
                _ => None,
            })
            .collect();
        let point = |values: &[f64]| -> [f64; 3] { [values.first().copied().unwrap_or_default(), values.get(1).copied().unwrap_or_default(), values.get(2).copied().unwrap_or_default()] };
        let vertex_points = |handles: &[u64]| -> Vec<[f64; 3]> {
            handles
                .iter()
                .filter_map(|handle| match by_handle.get(handle) {
                    Some(DwgEntityBody::Vertex(vertex)) => Some(point(&vertex.point)),
                    _ => None,
                })
                .collect()
        };
        let layer_handles: Vec<u64> = self
            .objects
            .iter()
            .filter_map(|candidate| match candidate.body.as_ref() {
                Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(_))) => Some(candidate.handle),
                _ => None,
            })
            .collect();
        self.objects
            .iter()
            .filter_map(|object| {
                let Some(DwgLogicalObjectBody::Entity(body)) = object.body.as_ref() else { return None };
                let (common, geometry) = match body {
                    DwgEntityBody::Line(line) => (&line.common, dwg_engine::DwgGeometry::Line { start: point(&line.start), end: point(&line.end) }),
                    DwgEntityBody::Point(value) => (&value.common, dwg_engine::DwgGeometry::Point { at: point(&value.point) }),
                    DwgEntityBody::Circle(circle) => (&circle.common, dwg_engine::DwgGeometry::Circle { center: point(&circle.center), radius: circle.radius, normal: point(&circle.extrusion) }),
                    DwgEntityBody::Arc(arc) => (&arc.common, dwg_engine::DwgGeometry::Arc { center: point(&arc.center), radius: arc.radius, start_angle: arc.start_angle, end_angle: arc.end_angle, normal: point(&arc.extrusion) }),
                    DwgEntityBody::Ellipse(ellipse) => (
                        &ellipse.common,
                        dwg_engine::DwgGeometry::Ellipse { center: point(&ellipse.center), major_axis: point(&ellipse.major_axis), ratio: ellipse.axis_ratio, start_param: ellipse.start_parameter, end_param: ellipse.end_parameter, normal: point(&ellipse.extrusion) },
                    ),
                    DwgEntityBody::LwPolyline(polyline) => (
                        &polyline.common,
                        dwg_engine::DwgGeometry::LwPolyline {
                            closed: polyline.closed,
                            elevation: polyline.elevation,
                            vertices: polyline.vertices.iter().map(|vertex| [vertex.point.first().copied().unwrap_or_default(), vertex.point.get(1).copied().unwrap_or_default()]).collect(),
                            bulges: polyline.vertices.iter().map(|vertex| vertex.bulge).collect(),
                        },
                    ),
                    DwgEntityBody::Spline(spline) => (
                        &spline.common,
                        dwg_engine::DwgGeometry::Spline { degree: spline.degree, control_points: spline.control_points.as_chunks::<3>().0.iter().map(|chunk| *chunk).collect(), knots: spline.knots.clone(), weights: spline.weights.clone() },
                    ),
                    DwgEntityBody::Text(text) => (
                        &text.common,
                        dwg_engine::DwgGeometry::Text { at: [text.insertion.first().copied().unwrap_or_default(), text.insertion.get(1).copied().unwrap_or_default(), text.elevation], height: text.height, rotation: text.rotation, content: text.value.clone() },
                    ),
                    DwgEntityBody::Face3d(face) => {
                        let corners = face.corners.as_chunks::<3>().0;
                        (&face.common, dwg_engine::DwgGeometry::Face3d { corners: [0, 1, 2, 3].map(|corner| corners.get(corner).copied().unwrap_or_default()) })
                    }
                    DwgEntityBody::Polyline3d(polyline) => (&polyline.common, dwg_engine::DwgGeometry::Polyline3d { closed: polyline.flags & 1 != 0, vertices: vertex_points(&polyline.vertex_handles) }),
                    DwgEntityBody::PolyfaceMesh(mesh) => (
                        &mesh.common,
                        dwg_engine::DwgGeometry::PolyfaceMesh {
                            vertices: vertex_points(&mesh.vertex_handles),
                            faces: mesh
                                .vertex_handles
                                .iter()
                                .filter_map(|handle| match by_handle.get(handle) {
                                    Some(DwgEntityBody::PolyfaceFace(face)) => Some([0, 1, 2, 3].map(|slot| i32::from(face.indices.get(slot).copied().unwrap_or_default()))),
                                    _ => None,
                                })
                                .collect(),
                        },
                    ),
                    DwgEntityBody::BlockBegin(_) | DwgEntityBody::BlockEnd(_) | DwgEntityBody::Insert(_) | DwgEntityBody::DimensionLinear(_) | DwgEntityBody::Viewport(_) | DwgEntityBody::Vertex(_) | DwgEntityBody::PolyfaceFace(_) | DwgEntityBody::SequenceEnd(_) => return None,
                };
                let color = match common.color.kind {
                    DwgEntityColorKind::ByLayer => -1,
                    DwgEntityColorKind::ByBlock => -2,
                    DwgEntityColorKind::Index => common.color.index as i16,
                    DwgEntityColorKind::TrueColor => return None,
                };
                let layer = layer_handles.iter().position(|handle| *handle == common.layer_handle).unwrap_or(0);
                Some(DwgLogicalEntity { layer, color, geometry: DwgLogicalGeometry::from_native(&geometry) })
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

/// 🗂️ Handles of the standard objects [`DwgLogicalDrawing::from_native`] gives a new document,
/// following AutoCAD 2010's numbering for the ones it has a fixed number for.
pub mod new_document_handles {
    pub const BLOCK_CONTROL: u64 = 0x01;
    pub const LAYER_CONTROL: u64 = 0x02;
    pub const STYLE_CONTROL: u64 = 0x03;
    pub const LINETYPE_CONTROL: u64 = 0x05;
    pub const VIEW_CONTROL: u64 = 0x06;
    pub const UCS_CONTROL: u64 = 0x07;
    pub const VIEWPORT_CONTROL: u64 = 0x08;
    pub const APPID_CONTROL: u64 = 0x09;
    pub const DIMSTYLE_CONTROL: u64 = 0x0a;
    pub const NAMED_OBJECTS: u64 = 0x0c;
    pub const GROUP_DICTIONARY: u64 = 0x0d;
    pub const PLOT_STYLE_NAME_DICTIONARY: u64 = 0x0e;
    pub const NORMAL_PLOT_STYLE: u64 = 0x0f;
    pub const STANDARD_TEXT_STYLE: u64 = 0x11;
    pub const ACAD_APPLICATION: u64 = 0x12;
    pub const BY_BLOCK: u64 = 0x14;
    pub const BY_LAYER: u64 = 0x15;
    pub const CONTINUOUS: u64 = 0x16;
    pub const MLINE_STYLE_DICTIONARY: u64 = 0x17;
    pub const STANDARD_MLINE_STYLE: u64 = 0x18;
    pub const PLOT_SETTINGS_DICTIONARY: u64 = 0x19;
    pub const LAYOUT_DICTIONARY: u64 = 0x1a;
    pub const STANDARD_DIMSTYLE: u64 = 0x1b;
    pub const ACTIVE_VIEWPORT: u64 = 0x1c;
    pub const MATERIAL_DICTIONARY: u64 = 0x1d;
    pub const COLOR_DICTIONARY: u64 = 0x1e;
    pub const MODEL_SPACE: u64 = 0x1f;
    pub const MODEL_SPACE_BLOCK: u64 = 0x20;
    pub const MODEL_SPACE_END: u64 = 0x21;
    pub const MODEL_LAYOUT: u64 = 0x22;
    pub const MODEL_LAYOUT_DICTIONARY: u64 = 0x23;
    pub const PAPER_SPACE: u64 = 0x24;
    pub const PAPER_SPACE_BLOCK: u64 = 0x25;
    pub const PAPER_SPACE_END: u64 = 0x26;
    pub const PAPER_LAYOUT: u64 = 0x27;
    pub const PAPER_LAYOUT_DICTIONARY: u64 = 0x28;
    pub const VISUAL_STYLE_DICTIONARY: u64 = 0x29;
    pub const FIRST_FREE: u64 = 0x2a;
}

impl DwgSnapshot {
    /// 🆕️ A new AC1024 document holding `drawing`: the object graph of [`DwgLogicalDrawing::from_native`],
    /// header variables with AutoCAD 2010's defaults (their relations pointing at that graph, extents
    /// recomputed from the geometry), the classes of its three class-numbered objects, and empty
    /// file-level sections. Deterministic: the same drawing always yields the same document.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_drawing(drawing: &dwg_engine::DwgDrawing) -> Result<Self, String> {
        use new_document_handles as h;
        let mut drawing = drawing.clone();
        drawing.recompute_extents();
        let logical = DwgLogicalDrawing::from_native(&drawing)?;
        let handle_seed = logical.handle_seed();
        let epoch = DwgJulianDate { days: 2_440_588, milliseconds: 0 };
        let space = |extents_minimum: Vec<f64>, extents_maximum: Vec<f64>| DwgHeaderSpaceGeometry {
            insertion_base: vec![0.0; 3],
            extents_minimum,
            extents_maximum,
            limits_minimum: vec![0.0, 0.0],
            limits_maximum: vec![12.0, 9.0],
            elevation: 0.0,
            ucs_origin: vec![0.0; 3],
            ucs_x_axis: vec![1.0, 0.0, 0.0],
            ucs_y_axis: vec![0.0, 1.0, 0.0],
            ucs_orthographic_view: 0,
            ucs_origin_top: vec![0.0; 3],
            ucs_origin_bottom: vec![0.0; 3],
            ucs_origin_left: vec![0.0; 3],
            ucs_origin_right: vec![0.0; 3],
            ucs_origin_front: vec![0.0; 3],
            ucs_origin_back: vec![0.0; 3],
        };
        let digest = semio_framework_hash::Sha256::digest(format!("{drawing:?}").as_bytes());
        let guid = |bytes: &[u8]| format!("{{{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}", bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]);
        let class = |number: u16, cpp_class_name: &str, dxf_name: &str, object_count: u32| DwgClass {
            number,
            proxy_flags: 0,
            application_name: "ObjectDBX Classes".into(),
            cpp_class_name: cpp_class_name.into(),
            dxf_name: dxf_name.into(),
            was_zombie: false,
            item_class_id: 499,
            object_count,
            dwg_version: 22,
            maintenance_version: 42,
            reserved_values: vec![0, 0],
        };
        let header = DwgHeaderVariables {
            units: DwgHeaderUnits { unit1_conversion: 412_148_564_080.0, unit2_conversion: 1.0, unit3_conversion: 1.0, unit4_conversion: 1.0, unit1_name: "m".into(), ..Default::default() },
            modes: DwgHeaderModes { dimension_associative: true, dimension_show: true, regeneration_mode: true, fill_mode: true, paper_space_linetype_scale: true, user_timer: true, world_view: true, tile_mode: true, visual_retain: true, ..Default::default() },
            integers: DwgHeaderIntegerSettings {
                proxy_graphics: 1,
                tree_depth: 3020,
                linear_units: 2,
                linear_precision: 4,
                angular_units: 0,
                angular_precision: 0,
                attribute_mode: 1,
                spline_segments: 8,
                surface_u: 6,
                surface_v: 6,
                surface_type: 6,
                surface_tab1: 6,
                surface_tab2: 6,
                spline_type: 6,
                shade_edge: 3,
                shade_difference: 70,
                maximum_active_viewports: 64,
                isolines: 4,
                text_quality: 50,
                ..Default::default()
            },
            scalars: DwgHeaderScalars { linetype_scale: 1.0, text_size: 0.2, trace_width: 0.05, sketch_increment: 0.1, facet_resolution: 0.5, multiline_scale: 1.0, current_entity_linetype_scale: 1.0, current_entity_color_index: 256, ..Default::default() },
            time: DwgHeaderTimeState { created_at: epoch.clone(), updated_at: epoch.clone(), ..Default::default() },
            paper_space: space(vec![0.0; 3], vec![0.0; 3]),
            model_space: space(drawing.extmin.to_vec(), drawing.extmax.to_vec()),
            dimensions: DwgDimensionSettings {
                scale: 1.0,
                arrow_size: 0.18,
                extension_offset: 0.0625,
                line_increment: 0.38,
                extension: 0.18,
                fixed_extension_length: 1.0,
                jog_angle: std::f64::consts::FRAC_PI_4,
                text_inside_horizontal: true,
                text_outside_horizontal: true,
                text_height: 0.18,
                center_mark: 0.09,
                alternate_scale: 25.4,
                linear_factor: 1.0,
                text_factor: 1.0,
                gap: 0.09,
                alternate_decimal_places: 2,
                decimal_places: 4,
                tolerance_decimal_places: 4,
                alternate_units_format: 2,
                alternate_tolerance_decimal_places: 2,
                linear_unit_format: 2,
                decimal_separator: 46,
                tolerance_justification: 1,
                fit: 3,
                alternate_measurement_scale: 100.0,
                measurement_scale: 100.0,
                dimension_line_weight: -2,
                extension_line_weight: -2,
                ..Default::default()
            },
            policy: DwgDrawingPolicy {
                text_stack_alignment: 1,
                text_stack_size: 70,
                current_entity_lineweight: -1,
                external_reference_editing: true,
                extended_names: true,
                plot_style_mode: true,
                sort_entities: 127,
                hide_text: 1,
                dimension_association: 2,
                obscured_color: 257,
                intersection_color: 257,
                steps_per_second: 2.0,
                step_size: 6.0,
                dwf_3d_precision: 2.0,
                lens_length: 50.0,
                solid_history: 1,
                show_history: 1,
                polysolid_width: 0.25,
                polysolid_height: 4.0,
                loft_angle1: std::f64::consts::FRAC_PI_2,
                loft_angle2: std::f64::consts::FRAC_PI_2,
                loft_parameter: 7,
                loft_normals: 1,
                latitude: 37.795,
                longitude: -122.394,
                timezone: -8000,
                light_glyph_display: 1,
                tile_mode_light_sync: 1,
                dwf_frame: 2,
                real_world_scale: true,
                interfere_color_index: 256,
                ..Default::default()
            },
            strings: DwgHeaderStrings { menu: "acad".into(), fingerprint_guid: guid(&digest[..16]), version_guid: guid(&digest[16..]), ..Default::default() },
            relations: DwgHeaderRelations {
                handle_seed,
                current_layer: logical.layer_zero_handle()?,
                text_style: h::STANDARD_TEXT_STYLE,
                current_linetype: h::BY_LAYER,
                current_material: 0,
                dimension_style: h::STANDARD_DIMSTYLE,
                multiline_style: h::STANDARD_MLINE_STYLE,
                dimension_text_style: h::STANDARD_TEXT_STYLE,
                block_control: h::BLOCK_CONTROL,
                layer_control: h::LAYER_CONTROL,
                style_control: h::STYLE_CONTROL,
                linetype_control: h::LINETYPE_CONTROL,
                view_control: h::VIEW_CONTROL,
                ucs_control: h::UCS_CONTROL,
                viewport_control: h::VIEWPORT_CONTROL,
                appid_control: h::APPID_CONTROL,
                dimension_style_control: h::DIMSTYLE_CONTROL,
                group_dictionary: h::GROUP_DICTIONARY,
                multiline_style_dictionary: h::MLINE_STYLE_DICTIONARY,
                named_objects_dictionary: h::NAMED_OBJECTS,
                layout_dictionary: h::LAYOUT_DICTIONARY,
                plot_settings_dictionary: h::PLOT_SETTINGS_DICTIONARY,
                plot_style_name_dictionary: h::PLOT_STYLE_NAME_DICTIONARY,
                material_dictionary: h::MATERIAL_DICTIONARY,
                color_dictionary: h::COLOR_DICTIONARY,
                visual_style_dictionary: h::VISUAL_STYLE_DICTIONARY,
                paper_space_block_record: h::PAPER_SPACE,
                model_space_block_record: h::MODEL_SPACE,
                by_layer_linetype: h::BY_LAYER,
                by_block_linetype: h::BY_BLOCK,
                continuous_linetype: h::CONTINUOUS,
                ..Default::default()
            },
        };
        let zero_digest = "0".repeat(32);
        let zero_checksum = "00000000-0000-0000-0000-000000000000".to_string();
        Ok(Self {
            schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
            version: "AC1024".into(),
            maintenance_version: DWG_NEW_DOCUMENT_MAINTENANCE_VERSION,
            codepage: 30,
            drawing: logical,
            header,
            classes: vec![class(500, "AcDbDictionaryWithDefault", "ACDBDICTIONARYWDFLT", 1), class(501, "AcDbPlaceHolder", "ACDBPLACEHOLDER", 1), class(502, "AcDbLayout", "LAYOUT", 2)],
            dependencies: Vec::new(),
            summary: DwgSummaryInfo { created_at: epoch.clone(), modified_at: epoch.clone(), ..Default::default() },
            application: DwgApplicationInfo { name: "AppInfoDataList".into(), version_checksum: zero_checksum.clone(), comment_checksum: zero_checksum.clone(), product_checksum: zero_checksum, ..Default::default() },
            template: DwgTemplate::default(),
            auxiliary_header: DwgAuxiliaryHeader { created_at: epoch.clone(), updated_at: epoch, handle_seed, ..Default::default() },
            revision_history: DwgRevisionHistory::default(),
            preview: DwgIndexedPreview { palette: vec![DwgRgba::default(); 256], ..Default::default() },
            application_history: DwgApplicationHistory {
                history_identifier_one: zero_digest.clone(),
                history_identifier_two: zero_digest.clone(),
                class_version: 3,
                application_version_digest: zero_digest.clone(),
                trust_comment_digest: zero_digest.clone(),
                property_set_digest: zero_digest.clone(),
                property_format_identifier: "f29f85e0-4ff9-1068-ab91-08002b27b3d9".into(),
                product_digest: zero_digest,
                ..Default::default()
            },
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
                let vertices = self.values[1..1 + count * 2].as_chunks::<2>().0.iter().map(|chunk| vec2(chunk)).collect::<Result<_, _>>()?;
                dwg_engine::DwgGeometry::LwPolyline { closed: self.closed, elevation: self.values[0], vertices, bulges: self.values[1 + count * 2..].to_vec() }
            }
            Spline => {
                let degree = self.indices[0] as u32;
                let point_count = self.indices[1] as usize;
                let knot_count = self.indices[2] as usize;
                let point_end = point_count * 3;
                let control_points = self.values[..point_end].as_chunks::<3>().0.iter().map(|chunk| vec3(chunk)).collect::<Result<_, _>>()?;
                dwg_engine::DwgGeometry::Spline { degree, control_points, knots: self.values[point_end..point_end + knot_count].to_vec(), weights: self.values[point_end + knot_count..].to_vec() }
            }
            Text => dwg_engine::DwgGeometry::Text { at: vec3(&self.values[0..3])?, height: self.values[3], rotation: self.values[4], content: self.text.clone() },
            Face3d => dwg_engine::DwgGeometry::Face3d { corners: [vec3(&self.values[0..3])?, vec3(&self.values[3..6])?, vec3(&self.values[6..9])?, vec3(&self.values[9..12])?] },
            Polyline3d => dwg_engine::DwgGeometry::Polyline3d { closed: self.closed, vertices: self.values.as_chunks::<3>().0.iter().map(|chunk| vec3(chunk)).collect::<Result<_, _>>()? },
            PolyfaceMesh => {
                let vertex_count = self.indices[0] as usize;
                let vertices = self.values.as_chunks::<3>().0.iter().take(vertex_count).map(|chunk| vec3(chunk)).collect::<Result<_, _>>()?;
                let faces = self.indices[1..].as_chunks::<4>().0.to_vec();
                dwg_engine::DwgGeometry::PolyfaceMesh { vertices, faces }
            }
        })
    }
}
//#endregion 🔖️DrawingModel

//#region 🔖️DocumentModel
/// 🔧️ The AC1024 maintenance release (preamble byte 0x12) new documents carry — the one the committed
/// AutoCAD drawing and this codec's AC1024 header layout were verified against.
pub const DWG_NEW_DOCUMENT_MAINTENANCE_VERSION: u8 = 2;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderTimeState {
    pub created_at: DwgJulianDate,
    pub updated_at: DwgJulianDate,
    pub editing_duration: DwgJulianDate,
    pub user_timer_duration: DwgJulianDate,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgClass {
    pub number: u16,
    pub proxy_flags: u32,
    pub application_name: String,
    pub cpp_class_name: String,
    pub dxf_name: String,
    #[value(default)]
    pub was_zombie: bool,
    #[value(default)]
    pub item_class_id: u16,
    #[value(default)]
    pub object_count: u32,
    #[value(default)]
    pub dwg_version: u32,
    #[value(default)]
    pub maintenance_version: u32,
    #[value(default)]
    pub reserved_values: Vec<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgDependency {
    pub feature: String,
    pub full_path: String,
    pub relative_path: String,
    pub fingerprint: String,
    pub version: String,
    #[value(default)]
    pub timestamp: u32,
    #[value(default)]
    pub file_size: u32,
    #[value(default)]
    pub affects_graphics: bool,
    #[value(default)]
    pub reference_count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgJulianDate {
    pub days: u32,
    pub milliseconds: u32,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgVersionStamp {
    pub version: u16,
    pub maintenance: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgCompatibilityProfile {
    #[default]
    Autocad2009,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgRevisionHistory {
    pub format_major: u32,
    pub format_minor: u32,
    #[value(default)]
    pub revisions: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgPreviewOrigin {
    #[default]
    BottomUp,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgRgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgIndexedPreview {
    pub width: u32,
    pub height: u32,
    pub origin: DwgPreviewOrigin,
    #[value(default)]
    pub palette: Vec<DwgRgba>,
    #[value(default)]
    pub pixel_indices: Vec<u8>,
    pub background_palette_index: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgApplicationPropertyKind {
    #[default]
    String,
    DateTime,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgApplicationProperty {
    pub id: u32,
    pub kind: DwgApplicationPropertyKind,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgProductInformation {
    pub name: String,
    pub build_version: String,
    pub registry_version: String,
    pub install_id: String,
    pub locale_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
    #[value(default)]
    pub properties: Vec<DwgApplicationProperty>,
    pub product_digest: String,
    pub product: DwgProductInformation,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgCustomProperty {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgSummaryInfo {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub keywords: String,
    pub comments: String,
    pub last_saved_by: String,
    pub revision_number: String,
    pub hyperlink_base: String,
    #[value(default)]
    pub total_editing_time: u64,
    #[value(default)]
    pub created_at: DwgJulianDate,
    #[value(default)]
    pub modified_at: DwgJulianDate,
    #[value(default)]
    pub custom_properties: Vec<DwgCustomProperty>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DwgMeasurement {
    #[default]
    English,
    Metric,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DwgTemplate {
    pub description: String,
    pub measurement: DwgMeasurement,
}
//#endregion 🔖️DocumentModel

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
    #[value(default)]
    pub maintenance_version: u8,
    /// 🌐 `codepage` (RS, plain preamble bytes 0x13-0x14, little-endian) — LibreDWG's
    /// `header.spec` documents this exact offset with `//@0x13: 29/30 for ANSI_1252`; the real
    /// `architectural.dwg` fixture reads `30` there, an exact match.
    #[state(artifact)]
    #[value(default)]
    pub codepage: u16,
    #[state(artifact)]
    #[value(default)]
    pub drawing: DwgLogicalDrawing,
    #[state(artifact)]
    #[value(default)]
    pub header: DwgHeaderVariables,
    #[state(artifact)]
    #[value(default)]
    pub classes: Vec<DwgClass>,
    #[state(artifact)]
    #[value(default)]
    pub dependencies: Vec<DwgDependency>,
    #[state(artifact)]
    #[value(default)]
    pub summary: DwgSummaryInfo,
    #[state(artifact)]
    #[value(default)]
    pub application: DwgApplicationInfo,
    #[state(artifact)]
    #[value(default)]
    pub template: DwgTemplate,
    #[state(artifact)]
    #[value(default)]
    pub auxiliary_header: DwgAuxiliaryHeader,
    #[state(artifact)]
    #[value(default)]
    pub revision_history: DwgRevisionHistory,
    #[state(artifact)]
    #[value(default)]
    pub preview: DwgIndexedPreview,
    #[state(artifact)]
    #[value(default)]
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

/// 📏️ The plain file-header PREAMBLE every AC1015+ DWG shares — `0x00`..=`0x15`: six ASCII version
/// characters, the application maintenance-release byte at `0x12`, the codepage `RS` at
/// `0x13`-`0x14`. A document that is EXACTLY this and nothing else is this artifact's canonical
/// EMPTY DWG, and it is not a hypothetical: `📚️examples/🎬️demo/🖼️assets/🖊️example.dwg` is committed
/// as those 22 bytes (`AC1024` followed by sixteen zeros).
const DWG_PREAMBLE_LEN: usize = 0x16;

/// 🫙️ Whether `snapshot` carries the empty document — the preamble triple and nothing else. Compared
/// against a freshly defaulted snapshot wearing the same triple rather than by inspecting fields one
/// at a time, so a field added to `DwgSnapshot` later cannot quietly fall out of the question.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_preamble_only_document(snapshot: &DwgSnapshot) -> bool {
    let bare = DwgSnapshot { schema: snapshot.schema.clone(), version: snapshot.version.clone(), maintenance_version: snapshot.maintenance_version, codepage: snapshot.codepage, ..DwgSnapshot::default() };
    *snapshot == bare
}

/// 🖨️ The empty document's bytes: the preamble region, version stamp and preamble triple written in,
/// every other byte zero — byte for byte what the committed demo example already is.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_preamble_only_document(snapshot: &DwgSnapshot) -> Vec<u8> {
    let mut bytes = vec![0u8; DWG_PREAMBLE_LEN];
    bytes[0..6].copy_from_slice(snapshot.version.as_bytes());
    bytes[0x12] = snapshot.maintenance_version;
    bytes[0x13..0x15].copy_from_slice(&snapshot.codepage.to_le_bytes());
    bytes
}

/// 🗺️ Materializes section pages only while deserializing and projects their standard objects.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_drawing(bytes: &[u8]) -> Result<DwgLogicalDrawing, String> {
    DwgLogicalDrawing::from_native(&dwg_engine::dwg_from_bytes(bytes)?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let (maintenance_version, codepage) = parse_version_header_fields(bytes)?;
    // 🫙️ The empty document. A stream that stops at the end of the preamble carries no section map,
    // no page map and no encrypted file header, so there is nothing behind it to decode — and the
    // R2004 reader's "file too short for encrypted header" is the wrong answer to give about a
    // document this artifact itself writes and commits as an example.
    if bytes.len() == DWG_PREAMBLE_LEN {
        return Ok(DwgSnapshot { schema: STDIO_DWG_DOCUMENT_SCHEMA.into(), version, maintenance_version, codepage, ..Default::default() });
    }
    if version != "AC1024" {
        return DwgSnapshot::from_drawing(&dwg_engine::dwg_from_bytes(bytes)?);
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
    // 🫙️ The empty document is written as the preamble and nothing else — the inverse of the read
    // above, and the only representation an R2004 container HAS for a snapshot that carries no
    // Header, no Classes and no object sections to build one from.
    let bytes = if is_preamble_only_document(snapshot) {
        encode_preamble_only_document(snapshot)
    } else if snapshot.version != "AC1024" {
        return Err(DwgExportError::InvalidVersion(format!("{} documents are read as AC1024 documents; this writer emits AC1024", snapshot.version)));
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
    fn envelope_id() -> &'static str {
        "stdio.dwg"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DwgSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
