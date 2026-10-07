//! 💰️ Direct LAS reconstruction admits every owned sequence and literal field.
use super::*;
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use semio_framework_value::FromValue;
use semio_framework_os_kernel::sqlite_snapshot::{artifact::reconstruct_text,transfer::reserve};
#[path="🔍️rows/🦀️.rs"]
mod rows;
fn retire_header(value:LasHeader){<String as FromValue>::retire_decoded(value.system_identifier);<String as FromValue>::retire_decoded(value.generating_software)}
fn retire(value:LasSnapshot){<String as FromValue>::retire_decoded(value.schema);retire_header(value.header);for vlr in value.vlrs{<String as FromValue>::retire_decoded(vlr.user_id);<String as FromValue>::retire_decoded(vlr.description);<Vec<u8> as FromValue>::retire_decoded(vlr.data);}drop(value.points)}
pub(super) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<LasSnapshot,ValueError>{
 validate_sqlite_database_schema(database,LasSnapshot::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document_row=database.table("las_document")?.single_row()?;if identity(document_row)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"LAS document identity must be 1"))}
 let header_row=database.table("las_header")?.single_row()?;if identity(header_row)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"LAS header identity must be 1"))}document(header_row)?;let h=FloatRow::new(header_row,HEADER_FLOATS)?;
 let histogram=rows::Entities::new(database.table("las_return_histogram")?,4,control)?;if histogram.rows.len()!=5{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"LAS return histogram requires exactly 5 counters"))}
 let mut output=DecodedFieldOwner::new(LasSnapshot{schema:String::new(),header:LasHeader::default(),vlrs:Vec::new(),points:Vec::new()},retire);
 output.as_mut().schema=reconstruct_text(control,document_row.text(1)?)?;
 let header=&mut output.as_mut().header;
 header.system_identifier=reconstruct_text(control,h.text(4)?)?;header.generating_software=reconstruct_text(control,h.text(5)?)?;
 header.version_major=u8_at(header_row,2)?;header.version_minor=u8_at(header_row,3)?;header.creation_day_of_year=u16_at(header_row,6)?;header.creation_year=u16_at(header_row,7)?;
 header.header_size=u16_at(header_row,8)?;header.offset_to_point_data=u32_at(header_row,9)?;header.number_of_vlrs=u32_at(header_row,10)?;
 header.point_data_format_id=u8_at(header_row,11)?;header.point_data_record_length=u16_at(header_row,12)?;header.number_of_point_records=u32_at(header_row,13)?;
 header.points_by_return=[u32_at(histogram.rows[0],3)?,u32_at(histogram.rows[1],3)?,u32_at(histogram.rows[2],3)?,u32_at(histogram.rows[3],3)?,u32_at(histogram.rows[4],3)?];
 header.x_scale=h.real(14)?;header.y_scale=h.real(15)?;header.z_scale=h.real(16)?;header.x_offset=h.real(17)?;header.y_offset=h.real(18)?;header.z_offset=h.real(19)?;
 header.max_x=h.real(20)?;header.min_x=h.real(21)?;header.max_y=h.real(22)?;header.min_y=h.real(23)?;header.max_z=h.real(24)?;header.min_z=h.real(25)?;
 let vlrs=rows::Entities::new(database.table("las_vlr")?,6,control)?;let octets=rows::Octets::new(database.table("las_vlr_octet")?,&vlrs,control)?;
 output.as_mut().vlrs=reserve(vlrs.rows.len(),control)?;
 for(index,row)in vlrs.rows.iter().enumerate(){
  let mut owner=DecodedFieldOwner::new(LasVlr::default(),|value:LasVlr|{<String as FromValue>::retire_decoded(value.user_id);<String as FromValue>::retire_decoded(value.description);<Vec<u8> as FromValue>::retire_decoded(value.data);});
  owner.as_mut().user_id=reconstruct_text(control,row.text(3)?)?;owner.as_mut().record_id=u16_at(row,4)?;owner.as_mut().description=reconstruct_text(control,row.text(5)?)?;
  let children=octets.children(row.rowid,control)?;owner.as_mut().data=reserve(children.len(),control)?;
  for(ordinal,child)in children.iter().enumerate(){owner.as_mut().data.push(u8_at(child,3)?);if ordinal%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,ordinal,children.len())?;}}
  output.as_mut().vlrs.push(owner.take());if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,vlrs.rows.len())?;}
 }
 let points=rows::Entities::new(database.table("las_point")?,21,control)?;
 let gps=rows::Components::new(database.table("las_point_gps")?,4,&points,control)?;let rgb=rows::Components::new(database.table("las_point_rgb")?,4,&points,control)?;
 output.as_mut().points=reserve(points.rows.len(),control)?;
 for(index,row)in points.rows.iter().enumerate(){
  let value=FloatRow::new(row,POINT_FLOATS)?;
  output.as_mut().points.push(LasPoint{x:value.real(3)?,y:value.real(4)?,z:value.real(5)?,intensity:u16_at(row,6)?,return_number:u8_at(row,7)?,number_of_returns:u8_at(row,8)?,
   scan_direction_flag:boolean(row,9)?,edge_of_flight_line:boolean(row,10)?,classification:u8_at(row,11)?,scan_angle_rank:i8::try_from(row.integer(12)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"LAS signed integer exceeds native width"))?,
   user_data:u8_at(row,13)?,point_source_id:u16_at(row,14)?,gps_time:gps.get(row.rowid).map(|row|FloatRow::new(row,GPS_FLOATS)?.real(1)).transpose()?,
   rgb:rgb.get(row.rowid).map(|row|Ok::<_,ValueError>((u16_at(row,1)?,u16_at(row,2)?,u16_at(row,3)?))).transpose()?});
  if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,points.rows.len())?;}
 }
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,points.rows.len(),points.rows.len())?;Ok(output.take())
}
