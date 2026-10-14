//! 🏷️ Extended data: xrecord values and bodies, dictionaries, extended entity data and named references.

use semio_framework_value::{ValueError, ValueRefusalKind};

//#region 🔖️DrawingModel
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgNamedReference {
    pub name: String,
    pub handle: u64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
dwg_metadata!(dwg_xrecord_value_spec, dwg_xrecord_value_spec_controlled, dwg_xrecord_value_spec_producer => DwgXRecordValue;
    0, "kind", false, [enum ("string", 0), ("real", 1), ("boolean", 2), ("integer8", 3), ("integer16", 4), ("integer32", 5), ("integer64", 6), ("point3d", 7), ("binary", 8), ("handle", 9), ("objectId", 10)];
    1, "group_code", false, [primitive Int];
    2, "string_value", true, [primitive Text];
    3, "real_value", true, [primitive Float];
    4, "boolean_value", true, [primitive Bool];
    5, "integer_value", true, [primitive Int];
    6, "point_value", true, [tuple Float 3];
    7, "binary_octets", true, [primitive Bytes64];
    8, "handle_value", true, [primitive UInt];
);
impl semio_framework_dsl_record::DslField for DwgXRecordValue {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(3)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(3,control)?;
        let (kind,group_code,payload_id,payload)=match self{
            Self::String{group_code,value}=>(0,*group_code,2,control.scoped_stage(|control|{control.begin_stage(0)?;<String as semio_framework_dsl_record::DslField>::to_value_controlled(value,control)})?),
            Self::Real{group_code,value}=>(1,*group_code,3,semio_framework_dsl_record::FieldValue::Float(*value)),
            Self::Boolean{group_code,value}=>(2,*group_code,4,semio_framework_dsl_record::FieldValue::Bool(*value)),
            Self::Integer8{group_code,value}=>(3,*group_code,5,semio_framework_dsl_record::FieldValue::Int(i64::from(*value))),
            Self::Integer16{group_code,value}=>(4,*group_code,5,semio_framework_dsl_record::FieldValue::Int(i64::from(*value))),
            Self::Integer32{group_code,value}=>(5,*group_code,5,semio_framework_dsl_record::FieldValue::Int(i64::from(*value))),
            Self::Integer64{group_code,value}=>(6,*group_code,5,semio_framework_dsl_record::FieldValue::Int(*value)),
            Self::Point3d{group_code,value}=>(7,*group_code,6,control.scoped_stage(|control|{control.begin_stage(0)?;<[f64;3] as semio_framework_dsl_record::DslField>::to_value_controlled(value,control)})?),
            Self::Binary{group_code,octets}=>(8,*group_code,7,semio_framework_dsl_record::FieldValue::Bytes64(control.copy_bytes(octets)?)),
            Self::Handle{group_code,value}=>(9,*group_code,8,semio_framework_dsl_record::FieldValue::UInt(*value)),
            Self::ObjectId{group_code,absolute_value}=>(10,*group_code,8,semio_framework_dsl_record::FieldValue::UInt(*absolute_value)),
        };record.insert(payload_id,payload)?;control.step()?;record.insert(0,semio_framework_dsl_record::FieldValue::Enum(kind))?;control.step()?;record.insert(1,semio_framework_dsl_record::FieldValue::Int(i64::from(group_code)))?;control.step()?;
        Ok(semio_framework_dsl_record::FieldValue::Record(record.take()))
    })}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRECORD value requires a record"))};Self::from_record_controlled(record,control)}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(record.fields.len())?;let kind=match record.get(0){Some(semio_framework_dsl_record::FieldValue::Enum(kind))=>*kind,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRECORD kind missing"))};
        let payload_id=match kind{0=>2,1=>3,2=>4,3..=6=>5,7=>6,8=>7,9|10=>8,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRECORD kind unknown"))};
        for(id,value)in &record.fields{if *id!=0&&*id!=1&&*id!=payload_id&&!matches!(value,semio_framework_dsl_record::FieldValue::Absent){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRECORD has an unowned field"))}control.step()?;}
        let group_code=control.scoped_stage(|control|{control.begin_stage(0)?;<i16 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRECORD group code missing"))?,control)})?;
        let payload=record.get(payload_id).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRECORD payload missing"))?;
        control.scoped_stage(|control|{control.begin_stage(0)?;match kind{
            0=>Ok(Self::String{group_code,value:<String as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            1=>Ok(Self::Real{group_code,value:<f64 as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            2=>Ok(Self::Boolean{group_code,value:<bool as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            3=>Ok(Self::Integer8{group_code,value:<i8 as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            4=>Ok(Self::Integer16{group_code,value:<i16 as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            5=>Ok(Self::Integer32{group_code,value:<i32 as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            6=>Ok(Self::Integer64{group_code,value:<i64 as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            7=>Ok(Self::Point3d{group_code,value:<[f64;3] as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            8=>{let semio_framework_dsl_record::FieldValue::Bytes64(bytes)=payload else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRECORD octets missing"))};Ok(Self::Binary{group_code,octets:control.copy_bytes(bytes)?})},
            9=>Ok(Self::Handle{group_code,value:<u64 as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),
            10=>Ok(Self::ObjectId{group_code,absolute_value:<u64 as semio_framework_dsl_record::DslField>::from_value_controlled(payload,control)?}),_=>unreachable!(),
        }})
    })}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_xrecord_value_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_xrecord_value_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        let (kind, group_code, payload_id, payload) = match self {
            Self::String { group_code, value } => (0, *group_code, 2, semio_framework_dsl_record::FieldValue::Text(value.clone())),
            Self::Real { group_code, value } => (1, *group_code, 3, semio_framework_dsl_record::FieldValue::Float(*value)),
            Self::Boolean { group_code, value } => (2, *group_code, 4, semio_framework_dsl_record::FieldValue::Bool(*value)),
            Self::Integer8 { group_code, value } => (3, *group_code, 5, semio_framework_dsl_record::FieldValue::Int(i64::from(*value))),
            Self::Integer16 { group_code, value } => (4, *group_code, 5, semio_framework_dsl_record::FieldValue::Int(i64::from(*value))),
            Self::Integer32 { group_code, value } => (5, *group_code, 5, semio_framework_dsl_record::FieldValue::Int(i64::from(*value))),
            Self::Integer64 { group_code, value } => (6, *group_code, 5, semio_framework_dsl_record::FieldValue::Int(*value)),
            Self::Point3d { group_code, value } => (7, *group_code, 6, semio_framework_dsl_record::FieldValue::Tuple(value.iter().copied().map(semio_framework_dsl_record::FieldValue::Float).collect())),
            Self::Binary { group_code, octets } => (8, *group_code, 7, semio_framework_dsl_record::FieldValue::Bytes64(octets.clone())),
            Self::Handle { group_code, value } => (9, *group_code, 8, semio_framework_dsl_record::FieldValue::UInt(*value)),
            Self::ObjectId { group_code, absolute_value, .. } => (10, *group_code, 8, semio_framework_dsl_record::FieldValue::UInt(*absolute_value)),
        };
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(kind));
        record.fields.insert(1, semio_framework_dsl_record::FieldValue::Int(i64::from(group_code)));
        record.fields.insert(payload_id, payload);
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err(format!("expected XRECORD value record, found {value:?}"));
        };
        if record.fields.values().filter(|value| !matches!(value, semio_framework_dsl_record::FieldValue::Absent)).count() != 3 {
            return Err("XRECORD value must contain exactly kind, group code, and one payload".into());
        }
        let kind = match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Enum(value)) => *value,
            other => return Err(format!("expected XRECORD value kind, found {other:?}")),
        };
        let group_code = match record.get(1) {
            Some(semio_framework_dsl_record::FieldValue::Int(value)) => i16::try_from(*value).map_err(|_| format!("XRECORD group code {value} exceeds i16"))?,
            other => return Err(format!("expected XRECORD group code, found {other:?}")),
        };
        let result = match kind {
            0 => Self::String {
                group_code,
                value: match record.get(2) {
                    Some(semio_framework_dsl_record::FieldValue::Text(value)) => value.clone(),
                    other => return Err(format!("expected XRECORD string, found {other:?}")),
                },
            },
            1 => Self::Real {
                group_code,
                value: match record.get(3) {
                    Some(semio_framework_dsl_record::FieldValue::Float(value)) => *value,
                    other => return Err(format!("expected XRECORD real, found {other:?}")),
                },
            },
            2 => Self::Boolean {
                group_code,
                value: match record.get(4) {
                    Some(semio_framework_dsl_record::FieldValue::Bool(value)) => *value,
                    other => return Err(format!("expected XRECORD boolean, found {other:?}")),
                },
            },
            3 => Self::Integer8 {
                group_code,
                value: match record.get(5) {
                    Some(semio_framework_dsl_record::FieldValue::Int(value)) => i8::try_from(*value).map_err(|_| format!("XRECORD integer8 {value} is out of range"))?,
                    other => return Err(format!("expected XRECORD integer8, found {other:?}")),
                },
            },
            4 => Self::Integer16 {
                group_code,
                value: match record.get(5) {
                    Some(semio_framework_dsl_record::FieldValue::Int(value)) => i16::try_from(*value).map_err(|_| format!("XRECORD integer16 {value} is out of range"))?,
                    other => return Err(format!("expected XRECORD integer16, found {other:?}")),
                },
            },
            5 => Self::Integer32 {
                group_code,
                value: match record.get(5) {
                    Some(semio_framework_dsl_record::FieldValue::Int(value)) => i32::try_from(*value).map_err(|_| format!("XRECORD integer32 {value} is out of range"))?,
                    other => return Err(format!("expected XRECORD integer32, found {other:?}")),
                },
            },
            6 => Self::Integer64 {
                group_code,
                value: match record.get(5) {
                    Some(semio_framework_dsl_record::FieldValue::Int(value)) => *value,
                    other => return Err(format!("expected XRECORD integer64, found {other:?}")),
                },
            },
            7 => Self::Point3d {
                group_code,
                value: match record.get(6) {
                    Some(semio_framework_dsl_record::FieldValue::Tuple(values)) if values.len() == 3 => [0, 1, 2]
                        .map(|index| match values.get(index) {
                            Some(semio_framework_dsl_record::FieldValue::Float(value)) => Ok(*value),
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
                    Some(semio_framework_dsl_record::FieldValue::Bytes64(value)) => value.clone(),
                    other => return Err(format!("expected XRECORD binary value, found {other:?}")),
                },
            },
            9 => Self::Handle {
                group_code,
                value: match record.get(8) {
                    Some(semio_framework_dsl_record::FieldValue::UInt(value)) => *value,
                    other => return Err(format!("expected XRECORD handle, found {other:?}")),
                },
            },
            10 => Self::ObjectId {
                group_code,
                absolute_value: match record.get(8) {
                    Some(semio_framework_dsl_record::FieldValue::UInt(value)) => *value,
                    other => return Err(format!("expected XRECORD absolute object id, found {other:?}")),
                },
            },
            other => return Err(format!("unknown XRECORD value kind ordinal {other}")),
        };
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDictionaryBody {
    #[value(default)]
    pub entries: Vec<DwgNamedReference>,
    pub cloning_flag: u16,
    pub hard_owner: bool,
    #[value(default)]
    pub default_entry_handle: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgXRecordBody {
    #[value(default)]
    pub values: Vec<DwgXRecordValue>,
    #[value(default)]
    pub object_id_handles: Vec<u64>,
    pub cloning_flag: u16,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgExtendedEntityData {
    pub application_handle: u64,
    #[value(default)]
    pub values: Vec<DwgXRecordValue>,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDictionaryVariable {
    pub value: String,
}
//#endregion 🔖️DrawingModel
