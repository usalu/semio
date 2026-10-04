//! ☁️ Direct actual LAS fields bind to the authored raw binary64 record words.
use super::super::super::{LasHeader,LasPoint,LasSnapshot,LasVlr};
use semio_framework_dsl_record::{DslField,FieldValue,RecordValue};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl};
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use semio_framework_dsl_record::native_encoding::EncodedRecord;
use semio_framework_value::{FromValue,ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"LAS required declared field is missing or has another shape")}
fn field(record:&RecordValue,id:u16)->Result<&FieldValue,ValueError>{record.get(id).ok_or_else(invalid)}
fn record(value:&FieldValue)->Result<&RecordValue,ValueError>{if let FieldValue::Record(value)=value{Ok(value)}else{Err(invalid())}}
fn read<T:DslField>(value:&RecordValue,id:u16,control:&mut NativeDecodeControl<'_>)->Result<T,ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;T::from_value_controlled(field(value,id)?,control)})}
fn project<T:DslField>(value:&T,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;value.to_value_controlled(control)})}
fn retire_header(value:LasHeader){<String as FromValue>::retire_decoded(value.system_identifier);<String as FromValue>::retire_decoded(value.generating_software)}
pub(super) fn retire(value:LasSnapshot){<String as FromValue>::retire_decoded(value.schema);retire_header(value.header);<Vec<LasVlr> as DslField>::retire_decoded(value.vlrs);drop(value.points)}
fn header_from(value:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<LasHeader,ValueError>{
 let mut output=DecodedFieldOwner::new(LasHeader::default(),retire_header);
 macro_rules! scalar{($id:literal,$name:ident)=>{output.as_mut().$name=read(value,$id,control)?;};}
 scalar!(0,version_major);scalar!(1,version_minor);scalar!(2,system_identifier);scalar!(3,generating_software);
 scalar!(4,creation_day_of_year);scalar!(5,creation_year);scalar!(6,header_size);scalar!(7,offset_to_point_data);
 scalar!(8,number_of_vlrs);scalar!(9,point_data_format_id);scalar!(10,point_data_record_length);scalar!(11,number_of_point_records);
 output.as_mut().points_by_return=control.scoped_stage(|control|{control.begin_stage(6)?;<[u32;5] as DslField>::from_value_controlled(field(value,12)?,control)})?;
 macro_rules! word{($id:literal,$name:ident)=>{output.as_mut().$name=f64::from_bits(read::<u64>(value,$id,control)?);};}
 word!(13,x_scale);word!(14,y_scale);word!(15,z_scale);word!(16,x_offset);word!(17,y_offset);word!(18,z_offset);
 word!(19,max_x);word!(20,min_x);word!(21,max_y);word!(22,min_y);word!(23,max_z);word!(24,min_z);
 control.checkpoint()?;Ok(output.take())
}
fn header_to(value:&LasHeader,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(25)?;let mut output=EncodedRecord::new(25,control)?;
 macro_rules! scalar{($id:literal,$name:ident)=>{output.insert($id,project(&value.$name,control)?)?;control.step()?;};}
 scalar!(0,version_major);scalar!(1,version_minor);scalar!(2,system_identifier);scalar!(3,generating_software);
 scalar!(4,creation_day_of_year);scalar!(5,creation_year);scalar!(6,header_size);scalar!(7,offset_to_point_data);
 scalar!(8,number_of_vlrs);scalar!(9,point_data_format_id);scalar!(10,point_data_record_length);scalar!(11,number_of_point_records);scalar!(12,points_by_return);
 macro_rules! word{($id:literal,$name:ident)=>{output.insert($id,project(&value.$name.to_bits(),control)?)?;control.step()?;};}
 word!(13,x_scale);word!(14,y_scale);word!(15,z_scale);word!(16,x_offset);word!(17,y_offset);word!(18,z_offset);
 word!(19,max_x);word!(20,min_x);word!(21,max_y);word!(22,min_y);word!(23,max_z);word!(24,min_z);
 control.checkpoint()?;Ok(output.take())})
}
fn point_from(value:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<LasPoint,ValueError>{
 let gps_time=match field(value,12)?{FieldValue::Absent=>None,_=>Some(f64::from_bits(read::<u64>(value,12,control)?))};
 let rgb=match field(value,13)?{FieldValue::Absent=>None,value=>{let value=record(value)?;Some((read(value,0,control)?,read(value,1,control)?,read(value,2,control)?))}};
 Ok(LasPoint{x:f64::from_bits(read::<u64>(value,0,control)?),y:f64::from_bits(read::<u64>(value,1,control)?),z:f64::from_bits(read::<u64>(value,2,control)?),
 intensity:read(value,3,control)?,return_number:read(value,4,control)?,number_of_returns:read(value,5,control)?,
 scan_direction_flag:read(value,6,control)?,edge_of_flight_line:read(value,7,control)?,classification:read(value,8,control)?,
 scan_angle_rank:read(value,9,control)?,user_data:read(value,10,control)?,point_source_id:read(value,11,control)?,gps_time,rgb})
}
fn rgb_to(value:(u16,u16,u16),control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(3)?;let mut output=EncodedRecord::new(3,control)?;
 output.insert(0,project(&value.0,control)?)?;control.step()?;output.insert(1,project(&value.1,control)?)?;control.step()?;output.insert(2,project(&value.2,control)?)?;control.step()?;Ok(output.take())})
}
fn point_to(value:&LasPoint,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(14)?;let mut output=EncodedRecord::new(14,control)?;
 output.insert(0,project(&value.x.to_bits(),control)?)?;control.step()?;output.insert(1,project(&value.y.to_bits(),control)?)?;control.step()?;output.insert(2,project(&value.z.to_bits(),control)?)?;control.step()?;
 macro_rules! scalar{($id:literal,$name:ident)=>{output.insert($id,project(&value.$name,control)?)?;control.step()?;};}
 scalar!(3,intensity);scalar!(4,return_number);scalar!(5,number_of_returns);scalar!(6,scan_direction_flag);scalar!(7,edge_of_flight_line);
 scalar!(8,classification);scalar!(9,scan_angle_rank);scalar!(10,user_data);scalar!(11,point_source_id);
 output.insert(12,match value.gps_time{None=>FieldValue::Absent,Some(value)=>project(&value.to_bits(),control)?})?;control.step()?;
 output.insert(13,match value.rgb{None=>FieldValue::Absent,Some(value)=>FieldValue::Record(rgb_to(value,control)?)})?;control.step()?;Ok(output.take())})
}
pub(super) fn from_record(value:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<LasSnapshot,ValueError>{
 let mut output=DecodedFieldOwner::new(LasSnapshot{schema:String::new(),header:LasHeader::default(),vlrs:Vec::new(),points:Vec::new()},retire);
 output.as_mut().schema=read(value,0,control)?;output.as_mut().header=header_from(record(field(value,1)?)?,control)?;
 output.as_mut().vlrs=control.scoped_stage(|control|{control.begin_stage(0)?;<Vec<LasVlr> as DslField>::from_value_controlled(field(value,2)?,control)})?;
 let FieldValue::List(points)=field(value,3)?else{return Err(invalid())};
 output.as_mut().points=control.allocate_vec(points.len())?;
 control.scoped_stage(|control|{control.begin_stage(points.len())?;for point in points{output.as_mut().points.push(point_from(record(point)?,control)?);control.step()?;}Ok::<_,ValueError>(())})?;
 control.checkpoint()?;Ok(output.take())
}
pub(super) fn to_record(value:&LasSnapshot,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(4)?;let mut output=EncodedRecord::new(4,control)?;
 output.insert(0,project(&value.schema,control)?)?;control.step()?;output.insert(1,FieldValue::Record(header_to(&value.header,control)?))?;control.step()?;
 output.insert(2,FieldValue::List(semio_framework_dsl_record::native_encoding::project_list(&value.vlrs,control)?))?;control.step()?;
 let points=control.scoped_stage(|control|{control.begin_stage(value.points.len())?;let mut output=DecodedFieldOwner::new(control.allocate_vec(value.points.len())?,|values:Vec<FieldValue>|{for value in values{semio_framework_dsl_record::native_encoding::retire_field(value);}});
 for point in &value.points{output.as_mut().push(FieldValue::Record(point_to(point,control)?));control.step()?;}Ok::<_,ValueError>(output.take())})?;
 output.insert(3,FieldValue::List(points))?;control.step()?;control.checkpoint()?;Ok(output.take())})
}
