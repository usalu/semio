//! 🗃️ Symbol tables: table controls, table records, complex colors and dimension-style parts.

use semio_framework_value::{ValueError, ValueRefusalKind};

//#region 🔖️DrawingModel
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgTableControlEntry {
    #[value(default)]
    pub handle: Option<u64>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
dwg_metadata!(dwg_table_control_entry_spec, dwg_table_control_entry_spec_controlled, dwg_table_control_entry_spec_producer => DwgTableControlEntry;
    0, "has_handle", false, [primitive Bool];
    1, "handle", true, [primitive UInt];
);
impl semio_framework_dsl_record::DslField for DwgTableControlEntry {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{
        let count=if self.handle.is_some(){2}else{1};control.begin_stage(count)?;
        let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(count,control)?;
        record.insert(0,semio_framework_dsl_record::FieldValue::Bool(self.handle.is_some()))?;control.step()?;
        if let Some(handle)=self.handle{record.insert(1,semio_framework_dsl_record::FieldValue::UInt(handle))?;control.step()?;}
        Ok(semio_framework_dsl_record::FieldValue::Record(record.take()))
    })}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG table-control entry requires a record"))};Self::from_record_controlled(record,control)}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(record.fields.len())?;let has=control.scoped_stage(|control|{control.begin_stage(0)?;<bool as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(0).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG handle presence missing"))?,control)})?;
        for(id,value)in &record.fields{if *id!=0&&!(has&&*id==1)&&!matches!(value,semio_framework_dsl_record::FieldValue::Absent){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG table-control entry has an unowned field"))}control.step()?;}
        Ok(Self{handle:if has{Some(control.scoped_stage(|control|{control.begin_stage(0)?;<u64 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG handle missing"))?,control)})?)}else{None}})
    })}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_table_control_entry_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_table_control_entry_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Bool(self.handle.is_some()));
        if let Some(handle) = self.handle {
            record.fields.insert(1, <u64 as semio_framework_dsl_record::DslField>::to_value(&handle));
        }
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected table-control entry record".into()) };
        match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Bool(false)) => Ok(Self { handle: None }),
            Some(semio_framework_dsl_record::FieldValue::Bool(true)) => Ok(Self { handle: Some(<u64 as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("table-control handle missing")?)?) }),
            other => Err(format!("invalid table-control handle presence {other:?}")),
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgTableControlEntries {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockTableControl {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    #[value(default)]
    pub model_space_handle: Option<u64>,
    #[value(default)]
    pub paper_space_handle: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLinetypeTableControl {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    pub by_block_handle: u64,
    pub by_layer_handle: u64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDimensionStyleTableControl {
    #[value(default)]
    pub entry_handles: Vec<DwgTableControlEntry>,
    #[value(default)]
    pub additional_handles: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
dwg_metadata!(table_control_body_spec, table_control_body_spec_controlled, table_control_body_spec_producer => DwgTableControlBody;
    1, "kind", false, [enum ("block", 0), ("layer", 1), ("textStyle", 2), ("linetype", 3), ("view", 4), ("ucs", 5), ("viewport", 6), ("registeredApplication", 7), ("dimensionStyle", 8)];
    2, "entries", true, [typed DwgTableControlEntries];
    3, "block", true, [typed DwgBlockTableControl];
    4, "linetype", true, [typed DwgLinetypeTableControl];
    5, "dimensionStyle", true, [typed DwgDimensionStyleTableControl];
);
impl semio_framework_dsl_record::DslField for DwgTableControlBody {
    dwg_controlled_payloads!(1; units[];
        0,3,Block,DwgBlockTableControl;
        1,2,Layer,DwgTableControlEntries;
        2,2,TextStyle,DwgTableControlEntries;
        3,4,Linetype,DwgLinetypeTableControl;
        4,2,View,DwgTableControlEntries;
        5,2,Ucs,DwgTableControlEntries;
        6,2,Viewport,DwgTableControlEntries;
        7,2,RegisteredApplication,DwgTableControlEntries;
        8,5,DimensionStyle,DwgDimensionStyleTableControl;);

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(table_control_body_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(table_control_body_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        let (kind, field, value) = match self {
            Self::Block(value) => (0, 3, <DwgBlockTableControl as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Layer(value) => (1, 2, <DwgTableControlEntries as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::TextStyle(value) => (2, 2, <DwgTableControlEntries as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Linetype(value) => (3, 4, <DwgLinetypeTableControl as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::View(value) => (4, 2, <DwgTableControlEntries as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Ucs(value) => (5, 2, <DwgTableControlEntries as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Viewport(value) => (6, 2, <DwgTableControlEntries as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::RegisteredApplication(value) => (7, 2, <DwgTableControlEntries as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::DimensionStyle(value) => (8, 5, <DwgDimensionStyleTableControl as semio_framework_dsl_record::DslField>::to_value(value)),
        };
        record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(kind));
        record.fields.insert(field, value);
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected table-control body record".into()) };
        match record.get(1) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) => Ok(Self::Block(<DwgBlockTableControl as semio_framework_dsl_record::DslField>::from_value(record.get(3).ok_or("block control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(1)) => Ok(Self::Layer(<DwgTableControlEntries as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("layer control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(2)) => Ok(Self::TextStyle(<DwgTableControlEntries as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("text-style control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(3)) => Ok(Self::Linetype(<DwgLinetypeTableControl as semio_framework_dsl_record::DslField>::from_value(record.get(4).ok_or("linetype control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(4)) => Ok(Self::View(<DwgTableControlEntries as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("view control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(5)) => Ok(Self::Ucs(<DwgTableControlEntries as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("UCS control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(6)) => Ok(Self::Viewport(<DwgTableControlEntries as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("viewport control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(7)) => Ok(Self::RegisteredApplication(<DwgTableControlEntries as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("registered-application control missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(8)) => Ok(Self::DimensionStyle(<DwgDimensionStyleTableControl as semio_framework_dsl_record::DslField>::from_value(record.get(5).ok_or("dimension-style control missing")?)?)),
            other => Err(format!("unknown table-control kind {other:?}")),
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgTableRecordCommon {
    pub name: String,
    pub xref_resolution: u16,
    #[value(default)]
    pub xref_handle: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgRegisteredApplicationTableRecord {
    pub common: DwgTableRecordCommon,
    pub group_71: u8,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
dwg_metadata!(dwg_complex_color_value_spec, dwg_complex_color_value_spec_controlled, dwg_complex_color_value_spec_producer => DwgComplexColorValue;
    0, "kind", false, [enum ("none", 0), ("byLayer", 1), ("byBlock", 2), ("byColor", 3), ("byAci", 4), ("byPen", 5), ("foreground", 6), ("layerOff", 7), ("layerFrozen", 8)];
    1, "red", true, [typed u8];
    2, "green", true, [typed u8];
    3, "blue", true, [typed u8];
    4, "index", true, [typed u16];
);
impl semio_framework_dsl_record::DslField for DwgComplexColorValue {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{
        let count=match self{Self::ByColor{..}=>4,Self::ByAci{..}|Self::ByPen{..}=>2,_=>1};control.begin_stage(count)?;
        let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(count,control)?;
        let kind=match self{
            Self::None=>0,Self::ByLayer=>1,Self::ByBlock=>2,
            Self::ByColor{red,green,blue}=>{record.insert(1,semio_framework_dsl_record::FieldValue::UInt(u64::from(*red)))?;control.step()?;record.insert(2,semio_framework_dsl_record::FieldValue::UInt(u64::from(*green)))?;control.step()?;record.insert(3,semio_framework_dsl_record::FieldValue::UInt(u64::from(*blue)))?;control.step()?;3},
            Self::ByAci{index}=>{record.insert(4,semio_framework_dsl_record::FieldValue::UInt(u64::from(*index)))?;control.step()?;4},
            Self::ByPen{index}=>{record.insert(4,semio_framework_dsl_record::FieldValue::UInt(u64::from(*index)))?;control.step()?;5},
            Self::Foreground=>6,Self::LayerOff=>7,Self::LayerFrozen=>8,
        };record.insert(0,semio_framework_dsl_record::FieldValue::Enum(kind))?;control.step()?;Ok(semio_framework_dsl_record::FieldValue::Record(record.take()))
    })}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG complex color requires a record"))};Self::from_record_controlled(record,control)}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(record.fields.len())?;let kind=match record.get(0){Some(semio_framework_dsl_record::FieldValue::Enum(kind))=>*kind,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG complex-color kind missing"))};
        let allowed:&[u16]=match kind{0|1|2|6|7|8=>&[],3=>&[1,2,3],4|5=>&[4],_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG complex-color kind unknown"))};
        for(id,value)in &record.fields{if *id!=0&&!allowed.contains(id)&&!matches!(value,semio_framework_dsl_record::FieldValue::Absent){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG complex color has an unowned field"))}control.step()?;}
        control.scoped_stage(|control|{control.begin_stage(0)?;match kind{
            0=>Ok(Self::None),1=>Ok(Self::ByLayer),2=>Ok(Self::ByBlock),
            3=>Ok(Self::ByColor{red:<u8 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG red missing"))?,control)?,green:<u8 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(2).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG green missing"))?,control)?,blue:<u8 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(3).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG blue missing"))?,control)?}),
            4=>Ok(Self::ByAci{index:<u16 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(4).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG ACI missing"))?,control)?}),
            5=>Ok(Self::ByPen{index:u8::try_from(<u16 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(4).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG pen missing"))?,control)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"DWG pen exceeds u8"))?}),
            6=>Ok(Self::Foreground),7=>Ok(Self::LayerOff),8=>Ok(Self::LayerFrozen),_=>unreachable!(),
        }})
    })}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_complex_color_value_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_complex_color_value_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        let kind = match self {
            Self::None => 0,
            Self::ByLayer => 1,
            Self::ByBlock => 2,
            Self::ByColor { red, green, blue } => {
                record.fields.insert(1, <u8 as semio_framework_dsl_record::DslField>::to_value(red));
                record.fields.insert(2, <u8 as semio_framework_dsl_record::DslField>::to_value(green));
                record.fields.insert(3, <u8 as semio_framework_dsl_record::DslField>::to_value(blue));
                3
            }
            Self::ByAci { index } => {
                record.fields.insert(4, <u16 as semio_framework_dsl_record::DslField>::to_value(index));
                4
            }
            Self::ByPen { index } => {
                record.fields.insert(4, <u16 as semio_framework_dsl_record::DslField>::to_value(&u16::from(*index)));
                5
            }
            Self::Foreground => 6,
            Self::LayerOff => 7,
            Self::LayerFrozen => 8,
        };
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(kind));
        semio_framework_dsl_record::FieldValue::Record(record)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected complex-color value record".into()) };
        let no_extra = |allowed: &[u16]| record.fields.iter().all(|(field, value)| *field == 0 || allowed.contains(field) || matches!(value, semio_framework_dsl_record::FieldValue::Absent));
        match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) if no_extra(&[]) => Ok(Self::None),
            Some(semio_framework_dsl_record::FieldValue::Enum(1)) if no_extra(&[]) => Ok(Self::ByLayer),
            Some(semio_framework_dsl_record::FieldValue::Enum(2)) if no_extra(&[]) => Ok(Self::ByBlock),
            Some(semio_framework_dsl_record::FieldValue::Enum(3)) if no_extra(&[1, 2, 3]) => Ok(Self::ByColor {
                red: <u8 as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("red missing")?)?,
                green: <u8 as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("green missing")?)?,
                blue: <u8 as semio_framework_dsl_record::DslField>::from_value(record.get(3).ok_or("blue missing")?)?,
            }),
            Some(semio_framework_dsl_record::FieldValue::Enum(4)) if no_extra(&[4]) => Ok(Self::ByAci { index: <u16 as semio_framework_dsl_record::DslField>::from_value(record.get(4).ok_or("ACI index missing")?)? }),
            Some(semio_framework_dsl_record::FieldValue::Enum(5)) if no_extra(&[4]) => Ok(Self::ByPen { index: u8::try_from(<u16 as semio_framework_dsl_record::DslField>::from_value(record.get(4).ok_or("pen index missing")?)?).map_err(|_| "pen index exceeds u8")? }),
            Some(semio_framework_dsl_record::FieldValue::Enum(6)) if no_extra(&[]) => Ok(Self::Foreground),
            Some(semio_framework_dsl_record::FieldValue::Enum(7)) if no_extra(&[]) => Ok(Self::LayerOff),
            Some(semio_framework_dsl_record::FieldValue::Enum(8)) if no_extra(&[]) => Ok(Self::LayerFrozen),
            other => Err(format!("invalid complex-color value {other:?}")),
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgComplexColor {
    pub index: u16,
    pub value: DwgComplexColorValue,
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub book_name: Option<String>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLinetypeTableRecord {
    pub common: DwgTableRecordCommon,
    pub description: String,
    pub pattern_length: f64,
    pub alignment: u8,
    #[value(default)]
    pub dashes: Vec<DwgLinetypeDash>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
dwg_metadata!(table_record_body_spec, table_record_body_spec_controlled, table_record_body_spec_producer => DwgTableRecordBody;
    1, "kind", false, [enum ("registeredApplication", 0), ("textStyle", 1), ("layer", 2), ("linetype", 3), ("blockHeader", 4), ("viewport", 5), ("dimensionStyle", 6)];
    2, "registeredApplication", true, [typed DwgRegisteredApplicationTableRecord];
    3, "textStyle", true, [typed DwgTextStyleTableRecord];
    4, "layer", true, [typed DwgLayerTableRecord];
    5, "linetype", true, [typed DwgLinetypeTableRecord];
    6, "blockHeader", true, [typed DwgBlockHeaderTableRecord];
    7, "viewport", true, [typed DwgViewportTableRecord];
    8, "dimensionStyle", true, [typed DwgDimensionStyleTableRecord];
);
impl semio_framework_dsl_record::DslField for DwgTableRecordBody {
    dwg_controlled_payloads!(1; units[];
        0,2,RegisteredApplication,DwgRegisteredApplicationTableRecord;
        1,3,TextStyle,DwgTextStyleTableRecord;
        2,4,Layer,DwgLayerTableRecord;
        3,5,Linetype,DwgLinetypeTableRecord;
        4,6,BlockHeader,DwgBlockHeaderTableRecord;
        5,7,Viewport,DwgViewportTableRecord;
        6,8,DimensionStyle,DwgDimensionStyleTableRecord;);

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(table_record_body_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(table_record_body_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        match self {
            Self::RegisteredApplication(value) => {
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(0));
                record.fields.insert(2, <DwgRegisteredApplicationTableRecord as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::TextStyle(value) => {
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(1));
                record.fields.insert(3, <DwgTextStyleTableRecord as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Layer(value) => {
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(2));
                record.fields.insert(4, <DwgLayerTableRecord as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Linetype(value) => {
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(3));
                record.fields.insert(5, <DwgLinetypeTableRecord as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::BlockHeader(value) => {
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(4));
                record.fields.insert(6, <DwgBlockHeaderTableRecord as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Viewport(value) => {
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(5));
                record.fields.insert(7, <DwgViewportTableRecord as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::DimensionStyle(value) => {
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Enum(6));
                record.fields.insert(8, <DwgDimensionStyleTableRecord as semio_framework_dsl_record::DslField>::to_value(value));
            }
        }
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err("expected table-record body record".into());
        };
        match record.get(1) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) => Ok(Self::RegisteredApplication(<DwgRegisteredApplicationTableRecord as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("registered-application record missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(1)) => Ok(Self::TextStyle(<DwgTextStyleTableRecord as semio_framework_dsl_record::DslField>::from_value(record.get(3).ok_or("text-style record missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(2)) => Ok(Self::Layer(<DwgLayerTableRecord as semio_framework_dsl_record::DslField>::from_value(record.get(4).ok_or("layer record missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(3)) => Ok(Self::Linetype(<DwgLinetypeTableRecord as semio_framework_dsl_record::DslField>::from_value(record.get(5).ok_or("linetype record missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(4)) => Ok(Self::BlockHeader(<DwgBlockHeaderTableRecord as semio_framework_dsl_record::DslField>::from_value(record.get(6).ok_or("block-header record missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(5)) => Ok(Self::Viewport(<DwgViewportTableRecord as semio_framework_dsl_record::DslField>::from_value(record.get(7).ok_or("viewport record missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(6)) => Ok(Self::DimensionStyle(<DwgDimensionStyleTableRecord as semio_framework_dsl_record::DslField>::from_value(record.get(8).ok_or("dimension-style record missing")?)?)),
            other => Err(format!("unknown table-record kind {other:?}")),
        }
    }
}
//#endregion 🔖️DrawingModel
