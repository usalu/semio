//! 📝️ Text representation codec surface for `stdio.jpg` (snapshot).

use super::*;
use pack::value::{FromValue, ToValue};

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JpgSnapshotText = String;
//#endregion 🚚️Carrier

const FIELDS:[(u16,&str,dsl::Shape,bool);17]=[
    (1,"schema",dsl::Shape::Text,false),
    (2,"width",dsl::Shape::UInt,false),
    (3,"height",dsl::Shape::UInt,false),
    (4,"pixels",dsl::Shape::Bytes64,false),
    (5,"reEncodeQuality",dsl::Shape::Value,false),
    (6,"jfifVersion",dsl::Shape::Value,false),
    (7,"jfifDensityUnits",dsl::Shape::Value,false),
    (8,"jfifXDensity",dsl::Shape::UInt,false),
    (9,"jfifYDensity",dsl::Shape::UInt,false),
    (10,"jfifThumbnail",dsl::Shape::Value,false),
    (11,"frame",dsl::Shape::Value,false),
    (12,"sofMarker",dsl::Shape::UInt,false),
    (13,"arithmetic",dsl::Shape::Bool,false),
    (14,"quantTables",dsl::Shape::Value,false),
    (15,"huffmanTables",dsl::Shape::Value,false),
    (16,"restartInterval",dsl::Shape::Value,false),
    (17,"otherSegments",dsl::Shape::Value,false),
];
pub fn spec()->dsl::RecordSpec{
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=dsl::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::RecordSpec,String>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<dsl::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=dsl::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        dsl::schema::producer::record(None,dsl::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub fn spec_producer()->dsl::RecordSpecProducer{dsl::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}


pub fn to_record(snapshot:&JpgSnapshot)->dsl::RecordValue {
    use dsl::FieldValue as V;
    dsl::RecordValue { fields: [
        (1,V::Text(snapshot.schema.clone())),(2,V::UInt(snapshot.width.into())),(3,V::UInt(snapshot.height.into())),(4,V::Bytes64(snapshot.pixels.clone())),
        (5,V::Value(snapshot.re_encode_quality.to_value())),(6,V::Value(snapshot.jfif_version.to_value())),(7,V::Value(snapshot.jfif_density_units.to_value())),(8,V::UInt(snapshot.jfif_x_density.into())),(9,V::UInt(snapshot.jfif_y_density.into())),
        (10,V::Value(snapshot.jfif_thumbnail.to_value())),(11,V::Value(snapshot.frame.to_value())),(12,V::UInt(snapshot.sof_marker.into())),(13,V::Bool(snapshot.arithmetic)),
        (14,V::Value(snapshot.quant_tables.to_value())),(15,V::Value(snapshot.huffman_tables.to_value())),(16,V::Value(snapshot.restart_interval.to_value())),(17,V::Value(snapshot.other_segments.to_value())),
    ].into_iter().collect() }
}

/// 🛫️ Projects the seventeen owned fields with cumulative allocation and exact stage control.
pub fn to_record_controlled(snapshot:&JpgSnapshot,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,store::TextError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<_,String>{
        use dsl::FieldValue as V;
        control.begin_stage(17)?;let mut record=dsl::native_encoding::EncodedRecord::new(17,control)?;
        record.insert(1,V::Text(control.copy_text(&snapshot.schema)?));control.step()?;
        record.insert(2,V::UInt(snapshot.width.into()));control.step()?;
        record.insert(3,V::UInt(snapshot.height.into()));control.step()?;
        record.insert(4,V::Bytes64(control.copy_bytes(&snapshot.pixels)?));control.step()?;
        record.insert(5,project(&snapshot.re_encode_quality,control)?);control.step()?;
        record.insert(6,project(&snapshot.jfif_version,control)?);control.step()?;
        record.insert(7,project(&snapshot.jfif_density_units,control)?);control.step()?;
        record.insert(8,V::UInt(snapshot.jfif_x_density.into()));control.step()?;
        record.insert(9,V::UInt(snapshot.jfif_y_density.into()));control.step()?;
        record.insert(10,project(&snapshot.jfif_thumbnail,control)?);control.step()?;
        record.insert(11,project(&snapshot.frame,control)?);control.step()?;
        record.insert(12,V::UInt(snapshot.sof_marker.into()));control.step()?;
        record.insert(13,V::Bool(snapshot.arithmetic));control.step()?;
        record.insert(14,project(&snapshot.quant_tables,control)?);control.step()?;
        record.insert(15,project(&snapshot.huffman_tables,control)?);control.step()?;
        record.insert(16,project(&snapshot.restart_interval,control)?);control.step()?;
        record.insert(17,project(&snapshot.other_segments,control)?);control.step()?;
        Ok(record.take())
    })).map_err(|message|store::TextError::new(message,dsl::TextSpan::at(1,1)))
}
fn project<T:ToValue>(value:&T,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::FieldValue,String>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(dsl::FieldValue::Value).map_err(|error|error.to_string())})}

fn error(key:&str)->store::TextError { store::TextError::new(format!("JPG owned snapshot field {key} is missing or has a different shape"),dsl::TextSpan::at(1,1)) }
fn value<T:FromValue>(record:&mut dsl::RecordValue,id:u16,key:&str)->Result<T,store::TextError> {
    match record.fields.remove(&id) { Some(dsl::FieldValue::Value(value))=>T::from_value(value).map_err(|failure|store::TextError::new(failure.to_string(),dsl::TextSpan::at(1,1))),_=>Err(error(key)) }
}
fn unsigned<T:TryFrom<u64>>(record:&mut dsl::RecordValue,id:u16,key:&str)->Result<T,store::TextError> {
    match record.fields.remove(&id) { Some(dsl::FieldValue::UInt(value))=>T::try_from(value).map_err(|_|error(key)),_=>Err(error(key)) }
}

pub fn from_record(mut record:dsl::RecordValue)->Result<JpgSnapshot,store::TextError> {
    if record.fields.len()!=17 || record.fields.keys().any(|id|!(1..=17).contains(id)) { return Err(store::TextError::new("JPG owned snapshot requires exactly seventeen declared fields",dsl::TextSpan::at(1,1))); }
    let schema=match record.fields.remove(&1){Some(dsl::FieldValue::Text(value))=>value,_=>return Err(error("schema"))};
    let width=unsigned(&mut record,2,"width")?;let height=unsigned(&mut record,3,"height")?;
    let pixels=match record.fields.remove(&4){Some(dsl::FieldValue::Bytes64(value))=>value,_=>return Err(error("pixels"))};
    let re_encode_quality=value(&mut record,5,"reEncodeQuality")?;let jfif_version=value(&mut record,6,"jfifVersion")?;let jfif_density_units=value(&mut record,7,"jfifDensityUnits")?;
    let jfif_x_density=unsigned(&mut record,8,"jfifXDensity")?;let jfif_y_density=unsigned(&mut record,9,"jfifYDensity")?;let jfif_thumbnail=value(&mut record,10,"jfifThumbnail")?;let frame=value(&mut record,11,"frame")?;let sof_marker=unsigned(&mut record,12,"sofMarker")?;
    let arithmetic=match record.fields.remove(&13){Some(dsl::FieldValue::Bool(value))=>value,_=>return Err(error("arithmetic"))};
    let quant_tables=value(&mut record,14,"quantTables")?;let huffman_tables=value(&mut record,15,"huffmanTables")?;let restart_interval=value(&mut record,16,"restartInterval")?;let other_segments=value(&mut record,17,"otherSegments")?;
    Ok(JpgSnapshot{schema,width,height,pixels,re_encode_quality,jfif_version,jfif_density_units,jfif_x_density,jfif_y_density,jfif_thumbnail,frame,sof_marker,arithmetic,quant_tables,huffman_tables,restart_interval,other_segments})
}

fn object<'a,const N:usize>(value:&'a dsl::DslValue,keys:[&str;N],control:&mut dsl::NativeDecodeControl<'_>)->Result<[&'a dsl::DslValue;N],String>{
    let dsl::DslValue::Object(entries)=value else{return Err("JPG expected an owned entity".into());};if entries.len()!=N{return Err("JPG entity field count differs from its declared schema".into());}let mut fields=[None;N];control.scoped_stage(|control|->Result<(),String>{control.begin_stage(N)?;for(key,value)in entries{control.step()?;let index=keys.iter().position(|expected|*expected==key).ok_or("JPG entity names an unknown field")?;if fields[index].replace(value).is_some(){return Err("JPG entity repeats a field".into());}}Ok(())})?;Ok(fields.map(|value|value.expect("exact complete field set")))
}
fn number<T:TryFrom<u64>>(value:&dsl::DslValue)->Result<T,String>{let dsl::DslValue::Number(value)=value else{return Err("JPG expected an integer".into());};T::try_from(value.as_u64().ok_or("JPG integer is negative or non-integral")?).map_err(|_|"JPG integer is out of its owned domain".into())}
fn octets(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Vec<u8>,String>{let dsl::DslValue::Bytes(value)=value else{return Err("JPG expected intrinsic octets".into());};control.copy_bytes(value)}
fn list<T>(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>,make:impl Fn(&dsl::DslValue,&mut dsl::NativeDecodeControl<'_>)->Result<T,String>)->Result<Vec<T>,String>{let dsl::DslValue::Array(values)=value else{return Err("JPG expected an ordered list".into());};control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=control.allocate_vec::<T>(values.len())?;for value in values{control.step()?;output.push(control.scoped_stage(|control|make(value,control))?);}Ok(output)})}
fn optional<T>(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>,make:impl FnOnce(&dsl::DslValue,&mut dsl::NativeDecodeControl<'_>)->Result<T,String>)->Result<Option<T>,String>{if matches!(value,dsl::DslValue::Null){Ok(None)}else{make(value,control).map(Some)}}
fn fixed<T,const N:usize>(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<[T;N],String>where T:TryFrom<u64>{let dsl::DslValue::Array(values)=value else{return Err("JPG expected a fixed numeric array".into());};if values.len()!=N{return Err("JPG fixed numeric array arity mismatch".into());}list(value,control,|value,_|number(value))?.try_into().map_err(|_|"JPG fixed numeric array arity mismatch".into())}
fn thumbnail(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JfifThumbnail,String>{let[width,height,rgb]=object(value,["width","height","rgbData"],control)?;Ok(JfifThumbnail{width:number(width)?,height:number(height)?,rgb_data:octets(rgb,control)?})}
fn component(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgFrameComponent,String>{let[id,h,v,quant]=object(value,["id","hSampling","vSampling","quantTableId"],control)?;Ok(JpgFrameComponent{id:number(id)?,h_sampling:number(h)?,v_sampling:number(v)?,quant_table_id:number(quant)?})}
fn frame(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgFrameHeader,String>{let[precision,width,height,components]=object(value,["precision","width","height","components"],control)?;Ok(JpgFrameHeader{precision:number(precision)?,width:number(width)?,height:number(height)?,components:list(components,control,component)?})}
fn quantization(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgQuantTable,String>{let[id,precision,values]=object(value,["id","precision","values"],control)?;Ok(JpgQuantTable{id:number(id)?,precision:number(precision)?,values:fixed(values,control)?})}
fn huffman(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgHuffmanTable,String>{let[id,class,bits,values]=object(value,["id","class","bits","values"],control)?;let class=match class{dsl::DslValue::String(value)if value=="dc"=>JpgHuffmanClass::Dc,dsl::DslValue::String(value)if value=="ac"=>JpgHuffmanClass::Ac,_=>return Err("JPG Huffman class is outside its declared enum".into())};Ok(JpgHuffmanTable{id:number(id)?,class,bits:fixed(bits,control)?,values:octets(values,control)?})}
fn segment(value:&dsl::DslValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgSegment,String>{let[marker,data]=object(value,["marker","data"],control)?;Ok(JpgSegment{marker:number(marker)?,data:octets(data,control)?})}
fn field_value(record:&dsl::RecordValue,id:u16)->Result<&dsl::DslValue,String>{match record.get(id){Some(dsl::FieldValue::Value(value))=>Ok(value),_=>Err(format!("JPG root field {id} has no declared value"))}}
fn field_unsigned<T:TryFrom<u64>>(record:&dsl::RecordValue,id:u16)->Result<T,String>{match record.get(id){Some(dsl::FieldValue::UInt(value))=>T::try_from(*value).map_err(|_|format!("JPG root integer {id} is out of range")),_=>Err(format!("JPG root field {id} is not UInt"))}}
pub fn from_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<JpgSnapshot,store::TextError>{
    (||->Result<JpgSnapshot,String>{if record.fields.len()!=17||record.fields.keys().any(|id|!(1..=17).contains(id)){return Err("JPG owned snapshot requires exactly seventeen declared fields".into());}control.begin_stage(17)?;
    let schema=match record.get(1){Some(dsl::FieldValue::Text(value))=>control.copy_text(value)?,_=>return Err("JPG schema is not Text".into())};control.step()?;
    let width=field_unsigned(record,2)?;control.step()?;let height=field_unsigned(record,3)?;control.step()?;
    let pixels=match record.get(4){Some(dsl::FieldValue::Bytes64(value))=>control.copy_bytes(value)?,_=>return Err("JPG pixels are not intrinsic octets".into())};control.step()?;
    let re_encode_quality=optional(field_value(record,5)?,control,|value,_|number(value))?;control.step()?;
    let jfif_version=match control.scoped_stage(|control|fixed::<u8,2>(field_value(record,6)?,control))?{[major,minor]=>(major,minor)};control.step()?;
    let jfif_density_units=match field_value(record,7)?{dsl::DslValue::String(value)if value=="aspect"=>JfifDensityUnits::Aspect,dsl::DslValue::String(value)if value=="pixelsPerInch"=>JfifDensityUnits::PixelsPerInch,dsl::DslValue::String(value)if value=="pixelsPerCm"=>JfifDensityUnits::PixelsPerCm,_=>return Err("JPG density unit is outside its declared enum".into())};control.step()?;
    let jfif_x_density=field_unsigned(record,8)?;control.step()?;let jfif_y_density=field_unsigned(record,9)?;control.step()?;
    let jfif_thumbnail=control.scoped_stage(|control|optional(field_value(record,10)?,control,thumbnail))?;control.step()?;
    let frame=control.scoped_stage(|control|optional(field_value(record,11)?,control,frame))?;control.step()?;let sof_marker=field_unsigned(record,12)?;control.step()?;
    let arithmetic=match record.get(13){Some(dsl::FieldValue::Bool(value))=>*value,_=>return Err("JPG arithmetic is not Bool".into())};control.step()?;
    let quant_tables=control.scoped_stage(|control|list(field_value(record,14)?,control,quantization))?;control.step()?;
    let huffman_tables=control.scoped_stage(|control|list(field_value(record,15)?,control,huffman))?;control.step()?;
    let restart_interval=optional(field_value(record,16)?,control,|value,_|number(value))?;control.step()?;
    let other_segments=control.scoped_stage(|control|list(field_value(record,17)?,control,segment))?;control.step()?;
    Ok(JpgSnapshot{schema,width,height,pixels,re_encode_quality,jfif_version,jfif_density_units,jfif_x_density,jfif_y_density,jfif_thumbnail,frame,sof_marker,arithmetic,quant_tables,huffman_tables,restart_interval,other_segments})
    })().map_err(|message|store::TextError::new(message,dsl::TextSpan::at(1,1)))
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;
