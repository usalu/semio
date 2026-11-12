//! 📊️ Controlled native columns and rows retain independent declarations and every typed cell.
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use crate::schema::snapshot::SemioTableSnapshot;
use crate::schema::snapshot::SemioTableCellKind;
use crate::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::native_decoding as values;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<native::Output<SemioTableSnapshot>>)->native::Outcome{
 if let Err(error)=crate::io::sqlite::snapshot::admit_layout(control.limits()){return native::refused(error)}
 native::decode_receiving(payload,STDIO_SEMIOTABLE_DOCUMENT_SCHEMA,control,native_owner,output,empty,|body,output,decoder,limits|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)})
}
fn kind(tag:u8)->Result<SemioTableCellKind,ValueError>{Ok(match tag{0=>SemioTableCellKind::Null,1=>SemioTableCellKind::Bool,2=>SemioTableCellKind::Int,3=>SemioTableCellKind::Float,4=>SemioTableCellKind::Str,5=>SemioTableCellKind::Bytes,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native column kind"))})}
/// 🧩️ Fills the retained table root from borrowed binary octets, inserting every column, row and cell before its fields.
pub fn binary_into(output:&mut SemioTableSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::record_binary_into(output,body,control,limits)}
/// 📝️ Fills the retained table root from the borrowed document body, inserting every column, row and cell before its fields.
pub fn document_into(output:&mut SemioTableSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?; let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","columns","rows"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio table native schema missing"))?,control)?;
 native::text_list_into(&mut output.columns,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|column,item,control,_|{let[name,tag]=native::record(item,control)?;let tag=match tag{"n"=>0,"b"=>1,"i"=>2,"f"=>3,"s"=>4,"y"=>5,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native column kind"))};native::hex_text_into(&mut column.name,name,control)?;column.kind=kind(tag)?;Ok(())})?;
 native::text_list_into(&mut output.rows,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|row,item,control,entities|{
  let mut cell_items=native::Items::new(item)?;let count=cell_items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;if count>limits.max_rows.saturating_sub(*entities){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio table native cells exceed remaining row limit"))}row.cells=control.allocate_vec::<SemioValue>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=cell_items.next(control)?{row.cells.push(SemioValue::Null);values::value_text_into(row.cells.last_mut().unwrap(),item,control,limits,entities)?;control.step()?;}Ok::<_,ValueError>(())})
 })
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioTableSnapshot{SemioTableSnapshot{schema:String::new(),columns:Vec::new(),rows:Vec::new()}}
