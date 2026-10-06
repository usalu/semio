//! 💰️ Exact backing for the complete byte-authoritative BMP semantic relation.
use super::BmpSnapshot;
use crate::standards::v_v3::subsets::any::io::BmpLayout;
use crate::schema::snapshot::BmpRowOrder;
use crate::standards::v_v3::subsets::any::io::layout::{self,LayoutFailure};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteTable,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,validate_sqlite_database_schema_controlled};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{Cell,Projection,reconstruct_text,ordered_row_refs};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl};
use std::fmt::Write;
type Result<T>=std::result::Result<T,ValueError>;
const SQL:&str=include_str!("../🗄️.sql");
const PROJECT:SqliteSnapshotPhase=SqliteSnapshotPhase::ProjectSnapshot;
const RECONSTRUCT:SqliteSnapshotPhase=SqliteSnapshotPhase::ReconstructSnapshot;
const CHANNELS:[&str;3]=["red","green","blue"];
#[path="📤️projection/🦀️.rs"] mod projection;
#[path="📥️reconstruction/🦀️.rs"] mod reconstruction;
pub(super) use projection::project;
pub(super) use reconstruction::reconstruct;
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn ownership(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::OwnershipLimit,message)}
fn invariant(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,message)}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ownership("BMP native length overflow"))}
fn mul(a:usize,b:usize)->Result<usize>{a.checked_mul(b).ok_or_else(||ownership("BMP native length overflow"))}
fn ordinal(value:usize)->Result<i64>{i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"BMP occurrence ordinal exceeds INTEGER"))}
fn integer(row:&SqliteRow,column:usize,min:i64,max:i64)->Result<i64>{let value=row.integer(column)?;if value<min||value>max{return Err(invalid("BMP INTEGER is outside its native field width"));}Ok(value)}
fn number(row:&SqliteRow,column:usize,max:u32)->Result<u32>{Ok(integer(row,column,0,i64::from(max))? as u32)}
fn scoped(row:&SqliteRow)->Result<()>{if row.integer(1)?!=1{return Err(invalid("BMP occurrence belongs to another document"));}Ok(())}
fn count(table:&SqliteTable,expected:usize)->Result<()>{if table.rows.len()!=expected{return Err(invalid("BMP semantic occurrence cardinality is incomplete"));}Ok(())}
fn singleton(table:&SqliteTable)->Result<&SqliteRow>{let row=table.single_row()?;if row.rowid!=1||row.integer(0)?!=1{return Err(invalid("BMP singleton identity must be one"));}Ok(row)}
fn word(bytes:&[u8],at:usize,width:usize)->u32{bytes[at..at+width].iter().enumerate().fold(0,|value,(index,byte)|value|u32::from(*byte)<<(index*8))}
fn store(bytes:&mut[u8],at:usize,width:usize,value:u32){for index in 0..width{bytes[at+index]=(value>>(index*8))as u8;}}
fn masks(layout:&BmpLayout)->[u32;3]{if layout.bits_per_pixel==24{[0xff0000,0xff00,0xff]}else{[layout.masks[0],layout.masks[1],layout.masks[2]]}}
fn physical(layout:&BmpLayout,y:usize)->usize{layout.data_offset+(if layout.row_order==BmpRowOrder::TopDown{y}else{layout.height as usize-1-y})*layout.row_stride}
fn capture(bytes:&[u8],control:&mut SqliteSnapshotControl<'_>)->Result<Vec<u8>>{
 control.check_value_bytes(bytes.len())?;
 control.allocation_stage(PROJECT,|remaining,progress|{
  let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
  let mut native=NativeEncodeControl::new(remaining,&mut callback);let result=native.copy_bytes(bytes);(result,native.owned_bytes())
 })?
}
fn zeros(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<u8>>{
 control.allocation_stage(RECONSTRUCT,|remaining,progress|{
  let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|progress(event.completed,event.total);
  let mut native=NativeDecodeControl::new(remaining,&mut callback);
  let result=(||{native.begin_stage(count)?;let mut bytes=native.allocate_vec(count)?;while bytes.len()<count{let end=bytes.len().saturating_add(65536).min(count);let added=end-bytes.len();bytes.resize(end,0);native.advance(added)?;}Ok(bytes)})();
  (result,native.owned_bytes())
 })?
}
struct DiagnosticLength(usize);
impl std::fmt::Write for DiagnosticLength{fn write_str(&mut self,text:&str)->std::fmt::Result{self.0=self.0.checked_add(text.len()).ok_or(std::fmt::Error)?;Ok(())}}
struct DiagnosticBytes(Vec<u8>);
impl std::fmt::Write for DiagnosticBytes{fn write_str(&mut self,text:&str)->std::fmt::Result{if text.len()>self.0.capacity()-self.0.len(){return Err(std::fmt::Error);}self.0.extend_from_slice(text.as_bytes());Ok(())}}
struct DiagnosticMatch<'a>{expected:&'a str,position:usize}
impl std::fmt::Write for DiagnosticMatch<'_>{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.position.checked_add(text.len()).ok_or(std::fmt::Error)?;if self.expected.as_bytes().get(self.position..end)!=Some(text.as_bytes()){return Err(std::fmt::Error);}self.position=end;Ok(())}}
fn diagnostic_matches(failure:LayoutFailure,expected:&str)->bool{let mut matcher=DiagnosticMatch{expected,position:0};write!(&mut matcher,"{failure}").is_ok()&&matcher.position==expected.len()}
fn diagnostic(failure:LayoutFailure,control:&mut SqliteSnapshotControl<'_>)->Result<String>{
 let mut length=DiagnosticLength(0);write!(&mut length,"{failure}").map_err(|_|ownership("BMP diagnostic length overflow"))?;control.check_value_bytes(length.0)?;
 control.allocation_stage(PROJECT,|remaining,progress|{
  let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
  let mut native=NativeEncodeControl::new(remaining,&mut callback);
  let result=(||{native.begin_stage(length.0)?;let mut writer=DiagnosticBytes(native.allocate_vec(length.0)?);write!(&mut writer,"{failure}").map_err(|_|invariant("BMP diagnostic exceeded its counted backing"))?;native.advance(writer.0.len())?;String::from_utf8(writer.0).map_err(|_|invariant("BMP diagnostic formatter emitted invalid UTF-8"))})();
  (result,native.owned_bytes())
 })?
}
fn ordered<'a>(table:&'a SqliteTable,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>>{let rows=ordered_row_refs(table,2,control)?;for(index,row)in rows.iter().enumerate(){scoped(row)?;if(index+1)%256==0{control.checkpoint(RECONSTRUCT,index+1,rows.len())?;}}Ok(rows)}
