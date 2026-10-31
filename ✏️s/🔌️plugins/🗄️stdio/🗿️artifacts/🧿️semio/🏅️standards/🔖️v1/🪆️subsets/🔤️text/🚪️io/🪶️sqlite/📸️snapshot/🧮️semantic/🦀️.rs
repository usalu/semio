//! 🔤️ Complete Semio text cells are counted from borrowed native primitives before ownership.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::{SemioTextSnapshot,SqliteDatabaseLimits,ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding as native;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use store::os_store::serialization::ArtifactSqliteSnapshotHost;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio text native primitive differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio text semantic extent overflow"))}
/// 🏛️ Admits the exact three-table authored layout before any owner is constructed.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioTextSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_text_document","semio_text_run","semio_text_mark"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioTextSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio text schema exceeds caller bytes"))}
 if limits.max_tables<3||limits.max_columns<5||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio text authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio text complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio text complete SQL cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio text declared rows exceed caller limit"))}Ok(())}
}
/// 🧾️ Validates real binary UTF-8 slices without allocating a text forecast.
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
/// 🔣️ Uses the common fixed scalar validator without allocating forecast text.
fn hex(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{native::hex_text_extent(value,control)}
fn document_kind(value:&str)->Result<usize>{match value{"b"|"c"|"l"=>Ok(4),"i"=>Ok(6),_=>Err(invalid())}}
/// 📃️ Counts required schema and optional run-list syntax without copying hexadecimal text.
pub fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","runs"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,hex(fields[0].ok_or_else(invalid)?,control)?)?)?;
  let mut runs=native::Items::new(fields[1].unwrap_or("[]"))?;let count=runs.count(control,limits.max_rows)?;census.future(count)?;
  control.begin_stage(count)?;while let Some(value)=runs.next(control)?{
   let[language,content,marks]=native::record(value,control)?;census.row(add(add(24,hex(language,control)?)?,hex(content,control)?)?)?;let mut marks=native::Items::new(marks)?;let count=marks.count(control,limits.max_rows)?;census.future(count)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=marks.next(control)?{let[kind,href]=native::record(value,control)?;census.row(add(add(24,document_kind(kind)?)?,hex(href,control)?)?)?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;
  }control.checkpoint()
 })
}
