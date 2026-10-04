//! 📝️ Text representation codec surface for `stdio.jpg` (snapshot).

use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
use pack::value::{FromValue, ToValue};

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JpgSnapshotText = String;
//#endregion 🚚️Carrier

const FIELDS:[(u16,&str,semio_framework_dsl_record::Shape,bool);17]=[
    (1,"schema",semio_framework_dsl_record::Shape::Text,false),
    (2,"width",semio_framework_dsl_record::Shape::UInt,false),
    (3,"height",semio_framework_dsl_record::Shape::UInt,false),
    (4,"pixels",semio_framework_dsl_record::Shape::Bytes64,false),
    (5,"reEncodeQuality",semio_framework_dsl_record::Shape::Value,false),
    (6,"jfifVersion",semio_framework_dsl_record::Shape::Value,false),
    (7,"jfifDensityUnits",semio_framework_dsl_record::Shape::Value,false),
    (8,"jfifXDensity",semio_framework_dsl_record::Shape::UInt,false),
    (9,"jfifYDensity",semio_framework_dsl_record::Shape::UInt,false),
    (10,"jfifThumbnail",semio_framework_dsl_record::Shape::Value,false),
    (11,"frame",semio_framework_dsl_record::Shape::Value,false),
    (12,"sofMarker",semio_framework_dsl_record::Shape::UInt,false),
    (13,"arithmetic",semio_framework_dsl_record::Shape::Bool,false),
    (14,"quantTables",semio_framework_dsl_record::Shape::Value,false),
    (15,"huffmanTables",semio_framework_dsl_record::Shape::Value,false),
    (16,"restartInterval",semio_framework_dsl_record::Shape::Value,false),
    (17,"otherSegments",semio_framework_dsl_record::Shape::Value,false),
];
pub fn spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=semio_framework_dsl_record::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub fn spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}


pub fn to_record(snapshot:&JpgSnapshot)->semio_framework_dsl_record::RecordValue {
    use semio_framework_dsl_record::FieldValue as V;
    semio_framework_dsl_record::RecordValue { fields: [
        (1,semio_framework_dsl_record::FieldValue::Text(snapshot.schema.clone())),(2,semio_framework_dsl_record::FieldValue::UInt(snapshot.width.into())),(3,semio_framework_dsl_record::FieldValue::UInt(snapshot.height.into())),(4,semio_framework_dsl_record::FieldValue::Bytes64(snapshot.pixels.clone())),
        (5,semio_framework_dsl_record::FieldValue::Value(snapshot.re_encode_quality.to_value())),(6,semio_framework_dsl_record::FieldValue::Value(snapshot.jfif_version.to_value())),(7,semio_framework_dsl_record::FieldValue::Value(snapshot.jfif_density_units.to_value())),(8,semio_framework_dsl_record::FieldValue::UInt(snapshot.jfif_x_density.into())),(9,semio_framework_dsl_record::FieldValue::UInt(snapshot.jfif_y_density.into())),
        (10,semio_framework_dsl_record::FieldValue::Value(snapshot.jfif_thumbnail.to_value())),(11,semio_framework_dsl_record::FieldValue::Value(snapshot.frame.to_value())),(12,semio_framework_dsl_record::FieldValue::UInt(snapshot.sof_marker.into())),(13,semio_framework_dsl_record::FieldValue::Bool(snapshot.arithmetic)),
        (14,semio_framework_dsl_record::FieldValue::Value(snapshot.quant_tables.to_value())),(15,semio_framework_dsl_record::FieldValue::Value(snapshot.huffman_tables.to_value())),(16,semio_framework_dsl_record::FieldValue::Value(snapshot.restart_interval.to_value())),(17,semio_framework_dsl_record::FieldValue::Value(snapshot.other_segments.to_value())),
    ].into_iter().collect() }
}

/// 🛫️ Projects the seventeen owned fields with cumulative allocation and exact stage control.
pub fn to_record_controlled(snapshot:&JpgSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<_,ValueError>{
        use semio_framework_dsl_record::FieldValue as V;
        control.begin_stage(17)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(17,control)?;
        record.insert(1,semio_framework_dsl_record::FieldValue::Text(control.copy_text(&snapshot.schema)?))?;control.step()?;
        record.insert(2,semio_framework_dsl_record::FieldValue::UInt(snapshot.width.into()))?;control.step()?;
        record.insert(3,semio_framework_dsl_record::FieldValue::UInt(snapshot.height.into()))?;control.step()?;
        record.insert(4,semio_framework_dsl_record::FieldValue::Bytes64(control.copy_bytes(&snapshot.pixels)?))?;control.step()?;
        record.insert(5,project(&snapshot.re_encode_quality,control)?)?;control.step()?;
        record.insert(6,project(&snapshot.jfif_version,control)?)?;control.step()?;
        record.insert(7,project(&snapshot.jfif_density_units,control)?)?;control.step()?;
        record.insert(8,semio_framework_dsl_record::FieldValue::UInt(snapshot.jfif_x_density.into()))?;control.step()?;
        record.insert(9,semio_framework_dsl_record::FieldValue::UInt(snapshot.jfif_y_density.into()))?;control.step()?;
        record.insert(10,project(&snapshot.jfif_thumbnail,control)?)?;control.step()?;
        record.insert(11,project(&snapshot.frame,control)?)?;control.step()?;
        record.insert(12,semio_framework_dsl_record::FieldValue::UInt(snapshot.sof_marker.into()))?;control.step()?;
        record.insert(13,semio_framework_dsl_record::FieldValue::Bool(snapshot.arithmetic))?;control.step()?;
        record.insert(14,project(&snapshot.quant_tables,control)?)?;control.step()?;
        record.insert(15,project(&snapshot.huffman_tables,control)?)?;control.step()?;
        record.insert(16,project(&snapshot.restart_interval,control)?)?;control.step()?;
        record.insert(17,project(&snapshot.other_segments,control)?)?;control.step()?;
        Ok(record.take())
    }))
}
fn project<T:ToValue>(value:&T,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(semio_framework_dsl_record::FieldValue::Value)})}

fn error(key:&str)->semio_framework_diagnostic::TextError { semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JPG owned snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)) }
fn value<T:FromValue>(record:&mut semio_framework_dsl_record::RecordValue,id:u16,key:&str)->Result<T,semio_framework_diagnostic::TextError> {
    match record.fields.remove(&id) { Some(semio_framework_dsl_record::FieldValue::Value(value))=>T::from_value(value).map_err(|failure|semio_framework_diagnostic::TextError::from_value_error(failure, semio_framework_diagnostic::TextSpan::at(1,1))),_=>Err(error(key)) }
}
fn unsigned<T:TryFrom<u64>>(record:&mut semio_framework_dsl_record::RecordValue,id:u16,key:&str)->Result<T,semio_framework_diagnostic::TextError> {
    match record.fields.remove(&id) { Some(semio_framework_dsl_record::FieldValue::UInt(value))=>T::try_from(value).map_err(|_|error(key)),_=>Err(error(key)) }
}

pub fn from_record(mut record:semio_framework_dsl_record::RecordValue)->Result<JpgSnapshot,semio_framework_diagnostic::TextError> {
    if record.fields.len()!=17 || record.fields.keys().any(|id|!(1..=17).contains(id)) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JPG owned snapshot requires exactly seventeen declared fields",semio_framework_diagnostic::TextSpan::at(1,1))); }
    let schema=match record.fields.remove(&1){Some(semio_framework_dsl_record::FieldValue::Text(value))=>value,_=>return Err(error("schema"))};
    let width=unsigned(&mut record,2,"width")?;let height=unsigned(&mut record,3,"height")?;
    let pixels=match record.fields.remove(&4){Some(semio_framework_dsl_record::FieldValue::Bytes64(value))=>value,_=>return Err(error("pixels"))};
    let re_encode_quality=value(&mut record,5,"reEncodeQuality")?;let jfif_version=value(&mut record,6,"jfifVersion")?;let jfif_density_units=value(&mut record,7,"jfifDensityUnits")?;
    let jfif_x_density=unsigned(&mut record,8,"jfifXDensity")?;let jfif_y_density=unsigned(&mut record,9,"jfifYDensity")?;let jfif_thumbnail=value(&mut record,10,"jfifThumbnail")?;let frame=value(&mut record,11,"frame")?;let sof_marker=unsigned(&mut record,12,"sofMarker")?;
    let arithmetic=match record.fields.remove(&13){Some(semio_framework_dsl_record::FieldValue::Bool(value))=>value,_=>return Err(error("arithmetic"))};
    let quant_tables=value(&mut record,14,"quantTables")?;let huffman_tables=value(&mut record,15,"huffmanTables")?;let restart_interval=value(&mut record,16,"restartInterval")?;let other_segments=value(&mut record,17,"otherSegments")?;
    Ok(JpgSnapshot{schema,width,height,pixels,re_encode_quality,jfif_version,jfif_density_units,jfif_x_density,jfif_y_density,jfif_thumbnail,frame,sof_marker,arithmetic,quant_tables,huffman_tables,restart_interval,other_segments})
}

fn object<'a,const N:usize>(value:&'a semio_framework_value::DslValue,keys:[&str;N],control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<[&'a semio_framework_value::DslValue;N],ValueError>{
    let semio_framework_value::DslValue::Object(entries)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG expected an owned entity"));};if entries.len()!=N{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG entity field count differs from its declared schema"));}let mut fields=[None;N];control.scoped_stage(|control|->Result<(),ValueError>{control.begin_stage(N)?;for(key,value)in entries{control.step()?;let index=keys.iter().position(|expected|*expected==key).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue, "JPG entity names an unknown field"))?;if fields[index].replace(value).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG entity repeats a field"));}}Ok(())})?;Ok(fields.map(|value|value.expect("exact complete field set")))
}
fn number<T:TryFrom<u64>>(value:&semio_framework_value::DslValue)->Result<T,ValueError>{let semio_framework_value::DslValue::Number(value)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG expected an integer"));};T::try_from(value.as_u64().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue, "JPG integer is negative or non-integral"))?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue, "JPG integer is out of its owned domain"))}
fn octets(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{let semio_framework_value::DslValue::Bytes(value)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG expected intrinsic octets"));};control.copy_bytes(value)}
fn list<T>(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>,make:impl Fn(&semio_framework_value::DslValue,&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T,ValueError>)->Result<Vec<T>,ValueError>{let semio_framework_value::DslValue::Array(values)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG expected an ordered list"));};control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=control.allocate_vec::<T>(values.len())?;for value in values{control.step()?;output.push(control.scoped_stage(|control|make(value,control))?);}Ok(output)})}
fn optional<T>(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>,make:impl FnOnce(&semio_framework_value::DslValue,&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T,ValueError>)->Result<Option<T>,ValueError>{if matches!(value,semio_framework_value::DslValue::Null){Ok(None)}else{make(value,control).map(Some)}}
fn fixed<T,const N:usize>(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<[T;N],ValueError>where T:TryFrom<u64>{let semio_framework_value::DslValue::Array(values)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG expected a fixed numeric array"));};if values.len()!=N{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG fixed numeric array arity mismatch"));}list(value,control,|value,_|number(value))?.try_into().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue, "JPG fixed numeric array arity mismatch"))}
fn thumbnail(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<JfifThumbnail,ValueError>{let[width,height,rgb]=object(value,["width","height","rgbData"],control)?;Ok(JfifThumbnail{width:number(width)?,height:number(height)?,rgb_data:octets(rgb,control)?})}
fn component(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<JpgFrameComponent,ValueError>{let[id,h,v,quant]=object(value,["id","hSampling","vSampling","quantTableId"],control)?;Ok(JpgFrameComponent{id:number(id)?,h_sampling:number(h)?,v_sampling:number(v)?,quant_table_id:number(quant)?})}
fn frame(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<JpgFrameHeader,ValueError>{let[precision,width,height,components]=object(value,["precision","width","height","components"],control)?;Ok(JpgFrameHeader{precision:number(precision)?,width:number(width)?,height:number(height)?,components:list(components,control,component)?})}
fn quantization(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<JpgQuantTable,ValueError>{let[id,precision,values]=object(value,["id","precision","values"],control)?;Ok(JpgQuantTable{id:number(id)?,precision:number(precision)?,values:fixed(values,control)?})}
fn huffman(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<JpgHuffmanTable,ValueError>{let[id,class,bits,values]=object(value,["id","class","bits","values"],control)?;let class=match class{semio_framework_value::DslValue::String(value)if value=="dc"=>JpgHuffmanClass::Dc,semio_framework_value::DslValue::String(value)if value=="ac"=>JpgHuffmanClass::Ac,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG Huffman class is outside its declared enum"))};Ok(JpgHuffmanTable{id:number(id)?,class,bits:fixed(bits,control)?,values:octets(values,control)?})}
fn segment(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<JpgSegment,ValueError>{let[marker,data]=object(value,["marker","data"],control)?;Ok(JpgSegment{marker:number(marker)?,data:octets(data,control)?})}
fn field_value(record:&semio_framework_dsl_record::RecordValue,id:u16)->Result<&semio_framework_value::DslValue,ValueError>{match record.get(id){Some(semio_framework_dsl_record::FieldValue::Value(value))=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("JPG root field {id} has no declared value")))}}
fn field_unsigned<T:TryFrom<u64>>(record:&semio_framework_dsl_record::RecordValue,id:u16)->Result<T,ValueError>{match record.get(id){Some(semio_framework_dsl_record::FieldValue::UInt(value))=>T::try_from(*value).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue, format!("JPG root integer {id} is out of range"))),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("JPG root field {id} is not UInt")))}}
pub fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<JpgSnapshot,ValueError>{
    (||->Result<JpgSnapshot,ValueError>{if record.fields.len()!=17||record.fields.keys().any(|id|!(1..=17).contains(id)){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG owned snapshot requires exactly seventeen declared fields"));}control.begin_stage(17)?;
    let schema=match record.get(1){Some(semio_framework_dsl_record::FieldValue::Text(value))=>control.copy_text(value)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG schema is not Text"))};control.step()?;
    let width=field_unsigned(record,2)?;control.step()?;let height=field_unsigned(record,3)?;control.step()?;
    let pixels=match record.get(4){Some(semio_framework_dsl_record::FieldValue::Bytes64(value))=>control.copy_bytes(value)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG pixels are not intrinsic octets"))};control.step()?;
    let re_encode_quality=optional(field_value(record,5)?,control,|value,_|number(value))?;control.step()?;
    let jfif_version=match control.scoped_stage(|control|fixed::<u8,2>(field_value(record,6)?,control))?{[major,minor]=>(major,minor)};control.step()?;
    let jfif_density_units=match field_value(record,7)?{semio_framework_value::DslValue::String(value)if value=="aspect"=>JfifDensityUnits::Aspect,semio_framework_value::DslValue::String(value)if value=="pixelsPerInch"=>JfifDensityUnits::PixelsPerInch,semio_framework_value::DslValue::String(value)if value=="pixelsPerCm"=>JfifDensityUnits::PixelsPerCm,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG density unit is outside its declared enum"))};control.step()?;
    let jfif_x_density=field_unsigned(record,8)?;control.step()?;let jfif_y_density=field_unsigned(record,9)?;control.step()?;
    let jfif_thumbnail=control.scoped_stage(|control|optional(field_value(record,10)?,control,thumbnail))?;control.step()?;
    let frame=control.scoped_stage(|control|optional(field_value(record,11)?,control,frame))?;control.step()?;let sof_marker=field_unsigned(record,12)?;control.step()?;
    let arithmetic=match record.get(13){Some(semio_framework_dsl_record::FieldValue::Bool(value))=>*value,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "JPG arithmetic is not Bool"))};control.step()?;
    let quant_tables=control.scoped_stage(|control|list(field_value(record,14)?,control,quantization))?;control.step()?;
    let huffman_tables=control.scoped_stage(|control|list(field_value(record,15)?,control,huffman))?;control.step()?;
    let restart_interval=optional(field_value(record,16)?,control,|value,_|number(value))?;control.step()?;
    let other_segments=control.scoped_stage(|control|list(field_value(record,17)?,control,segment))?;control.step()?;
    Ok(JpgSnapshot{schema,width,height,pixels,re_encode_quality,jfif_version,jfif_density_units,jfif_x_density,jfif_y_density,jfif_thumbnail,frame,sof_marker,arithmetic,quant_tables,huffman_tables,restart_interval,other_segments})
    })()
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;
