//! 📐️ Controlled authored CAD layers, blocks and every entity variant.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use crate::schema::snapshot::{SemioCadSnapshot, CadEntityRecord, CadEntity, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioCadSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOCAD_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}
fn f64(reader:&mut store::ByteReader<'_>)->Result<f64,ValueError>{reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))}
fn point(reader:&mut store::ByteReader<'_>)->Result<SemioPoint2,ValueError>{Ok(SemioPoint2{x:f64(reader)?,y:f64(reader)?})}
fn boolean(value:u8)->Result<bool,ValueError>{match value{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native CAD boolean"))}}


/// 🔬️ Selects each binary CAD entity variant in its retained slot before copying its text or vertices.
fn entity_binary_into(output:&mut CadEntity,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 native::entities(entities,1,limits)?;match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{
 0=>{*output=CadEntity::Line{a:point(reader)?,b:point(reader)?};},1=>{*output=CadEntity::Arc{center:point(reader)?,radius:f64(reader)?,start_angle:f64(reader)?,end_angle:f64(reader)?};},2=>{*output=CadEntity::Circle{center:point(reader)?,radius:f64(reader)?};},3=>{*output=CadEntity::Ellipse{center:point(reader)?,major_axis_end:point(reader)?,ratio:f64(reader)?,start_param:f64(reader)?,end_param:f64(reader)?};},
 4=>{*output=CadEntity::Polyline{vertices:Vec::new(),closed:false};let CadEntity::Polyline{vertices,closed}=&mut *output else{unreachable!()};native::binary_list_into(vertices,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=point(reader)?;Ok(())})?;*closed=boolean(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)?;},
 5=>{*output=CadEntity::Text{position:point(reader)?,height:f64(reader)?,rotation:f64(reader)?,content:String::new()};let CadEntity::Text{content,..}=&mut *output else{unreachable!()};native::reader_text_into(content,reader,control)?;},
 6=>{*output=CadEntity::Insert{block_name:String::new(),insertion_point:SemioPoint2{x:0.0,y:0.0},scale:SemioPoint2{x:0.0,y:0.0},rotation:0.0};let CadEntity::Insert{block_name,insertion_point,scale,rotation}=&mut *output else{unreachable!()};native::reader_text_into(block_name,reader,control)?;*insertion_point=point(reader)?;*scale=point(reader)?;*rotation=f64(reader)?;},
 7=>{*output=CadEntity::Solid{p1:point(reader)?,p2:point(reader)?,p3:point(reader)?,p4:point(reader)?};},
 8=>{*output=CadEntity::Dimension{def_point:point(reader)?,text_position:point(reader)?,measurement:f64(reader)?,text:String::new()};let CadEntity::Dimension{text,..}=&mut *output else{unreachable!()};native::reader_text_into(text,reader,control)?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native CAD entity"))}
 Ok(())
}
/// 📇️ Fills one binary entity record in its retained slot.
fn record_binary_into(output:&mut CadEntityRecord,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{native::reader_text_into(&mut output.handle,reader,control)?;native::reader_text_into(&mut output.layer,reader,control)?;entity_binary_into(&mut output.entity,reader,control,limits,entities)}
/// 🧩️ Fills the retained CAD root from borrowed binary octets, inserting every layer, block and entity before its fields.
pub fn binary_into(output:&mut SemioCadSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio CAD native format"))}native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.layers,&mut reader,control,limits,&mut entities,Default::default,|layer,reader,control,_|{native::reader_text_into(&mut layer.name,reader,control)?;let bits=reader.read_varint_u64().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let color_index=bits as i32;if color_index as u64!=bits{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native signed32 layer color"))}layer.color_index=color_index;native::reader_text_into(&mut layer.line_type,reader,control)?;layer.visible=boolean(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)?;Ok(())})?;
 native::binary_list_into(&mut output.blocks,&mut reader,control,limits,&mut entities,Default::default,|block,reader,control,entities|{native::reader_text_into(&mut block.name,reader,control)?;block.base_point=point(reader)?;native::binary_list_into(&mut block.entities,reader,control,limits,entities,Default::default,|record,reader,control,entities|record_binary_into(record,reader,control,limits,entities))})?;
 native::binary_list_into(&mut output.entities,&mut reader,control,limits,&mut entities,Default::default,|record,reader,control,entities|record_binary_into(record,reader,control,limits,entities))?;if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio CAD native trailing bytes"))}Ok(())
}
fn point_text(value:&str,control:&mut NativeDecodeControl<'_>)->Result<SemioPoint2,ValueError>{let[x,y]=native::record(value,control)?;Ok(SemioPoint2{x:native::float(x,control)?,y:native::float(y,control)?})}
fn bool_text(value:&str)->Result<bool,ValueError>{match value{"0"=>Ok(false),"1"=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native CAD boolean"))}}


/// 🧬️ Selects each text CAD entity variant in its retained slot before copying its text or vertices.
fn entity_text_into(output:&mut CadEntity,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 native::entities(entities,1,limits)?;let(tag,value)=value.split_at_checked(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native CAD entity tag missing"))?;match tag{
 "L"=>{let[a,b]=native::record(value,control)?;*output=CadEntity::Line{a:point_text(a,control)?,b:point_text(b,control)?};},
 "A"=>{let[center,radius,start_angle,end_angle]=native::record(value,control)?;*output=CadEntity::Arc{center:point_text(center,control)?,radius:native::float(radius,control)?,start_angle:native::float(start_angle,control)?,end_angle:native::float(end_angle,control)?};},
 "C"=>{let[center,radius]=native::record(value,control)?;*output=CadEntity::Circle{center:point_text(center,control)?,radius:native::float(radius,control)?};},
 "E"=>{let[center,major_axis_end,ratio,start_param,end_param]=native::record(value,control)?;*output=CadEntity::Ellipse{center:point_text(center,control)?,major_axis_end:point_text(major_axis_end,control)?,ratio:native::float(ratio,control)?,start_param:native::float(start_param,control)?,end_param:native::float(end_param,control)?};},
 "P"=>{let[vertex_fields,closed_field]=native::record(value,control)?;*output=CadEntity::Polyline{vertices:Vec::new(),closed:false};let CadEntity::Polyline{vertices,closed}=&mut *output else{unreachable!()};native::text_list_into(vertices,vertex_fields,control,limits,entities,Default::default,|slot,value,control,_|{*slot=point_text(value,control)?;Ok(())})?;*closed=bool_text(closed_field)?;},
 "T"=>{let[position,height,rotation,content]=native::record(value,control)?;*output=CadEntity::Text{position:point_text(position,control)?,height:native::float(height,control)?,rotation:native::float(rotation,control)?,content:String::new()};let CadEntity::Text{content:slot,..}=&mut *output else{unreachable!()};native::hex_text_into(slot,content,control)?;},
 "I"=>{let[block_name,insertion_point,scale,rotation]=native::record(value,control)?;*output=CadEntity::Insert{block_name:String::new(),insertion_point:SemioPoint2{x:0.0,y:0.0},scale:SemioPoint2{x:0.0,y:0.0},rotation:0.0};let CadEntity::Insert{block_name:name_slot,insertion_point:point_slot,scale:extent_slot,rotation:angle_slot}=&mut *output else{unreachable!()};native::hex_text_into(name_slot,block_name,control)?;*point_slot=point_text(insertion_point,control)?;*extent_slot=point_text(scale,control)?;*angle_slot=native::float(rotation,control)?;},
 "S"=>{let[p1,p2,p3,p4]=native::record(value,control)?;*output=CadEntity::Solid{p1:point_text(p1,control)?,p2:point_text(p2,control)?,p3:point_text(p3,control)?,p4:point_text(p4,control)?};},
 "D"=>{let[def_point,text_position,measurement,text]=native::record(value,control)?;*output=CadEntity::Dimension{def_point:point_text(def_point,control)?,text_position:point_text(text_position,control)?,measurement:native::float(measurement,control)?,text:String::new()};let CadEntity::Dimension{text:slot,..}=&mut *output else{unreachable!()};native::hex_text_into(slot,text,control)?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native CAD entity"))}
 Ok(())
}
/// 🗒️ Fills one text entity record in its retained slot.
fn record_text_into(output:&mut CadEntityRecord,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{let[handle,layer,value]=native::record(value,control)?;native::hex_text_into(&mut output.handle,handle,control)?;native::hex_text_into(&mut output.layer,layer,control)?;entity_text_into(&mut output.entity,value,control,limits,entities)}
/// 📝️ Fills the retained CAD root from the borrowed document body, inserting every layer, block and entity before its fields.
pub fn document_into(output:&mut SemioCadSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","layers","blocks","entities"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio CAD native schema missing"))?,control)?;
 native::text_list_into(&mut output.layers,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|layer,value,control,_|{let[name,color_index,line_type,visible]=native::record(value,control)?;native::hex_text_into(&mut layer.name,name,control)?;layer.color_index=control.scoped_stage(|control|{control.begin_stage(color_index.len())?;for _ in color_index.bytes(){control.step()?;}color_index.parse::<i32>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native signed32 layer color"))})?;native::hex_text_into(&mut layer.line_type,line_type,control)?;layer.visible=bool_text(visible)?;Ok(())})?;
 native::text_list_into(&mut output.blocks,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|block,value,control,entities|{let[name,base_point,records]=native::record(value,control)?;native::hex_text_into(&mut block.name,name,control)?;block.base_point=point_text(base_point,control)?;native::text_list_into(&mut block.entities,records,control,limits,entities,Default::default,|record,value,control,entities|record_text_into(record,value,control,limits,entities))})?;
 native::text_list_into(&mut output.entities,fields[3].unwrap_or("[]"),control,limits,&mut entities,Default::default,|record,value,control,entities|record_text_into(record,value,control,limits,entities))
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioCadSnapshot{SemioCadSnapshot{schema:String::new(),layers:Vec::new(),blocks:Vec::new(),entities:Vec::new()}}
