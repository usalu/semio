//! 🏭️ IFC2x3 exact decimal syntax, instance types and EDM production fields.
use crate::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3Snapshot,Ifc2x3EdmPreamble};
use semio_s_artifact_stdio_contract::part21::{Part21Document,Part21Header,Part21Instance,Part21Value,Part21Decimal};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter},*}};

#[path="🔎️mvd/💰️diagnostics/🦀️.rs"]
mod mvd_diagnostics;
pub use mvd_diagnostics::MvdDiagnostics;
#[path="🛬️reconstruction/🦀️.rs"]
mod reconstruction;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
#[path="📏️encoding/💰️backing/🦀️.rs"]
mod native_backing;
struct UnsignedWord{bytes:[u8;20],start:usize}
impl UnsignedWord{
    fn new(mut value:u64)->Self{let mut out=Self{bytes:[0;20],start:20};loop{out.start-=1;out.bytes[out.start]=b'0'+(value%10)as u8;value/=10;if value==0{return out;}}}
    fn text(&self)->&str{std::str::from_utf8(&self.bytes[self.start..]).expect("decimal digits are UTF-8")}
}
fn sort_paid<T>(values:&mut[T],mut compare:impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{
    fn sift<T>(values:&mut[T],mut root:usize,end:usize,compare:&mut impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{loop{let Some(child)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};let mut next=child;if child+1<end&&compare(&values[child],&values[child+1])?.is_lt(){next=child+1;}if !compare(&values[root],&values[next])?.is_lt(){return Ok(())}values.swap(root,next);root=next;}}
    for root in(0..values.len()/2).rev(){sift(values,root,values.len(),&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,&mut compare)?;}Ok(())
}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid IFC2x3 relational identity or columns"))}else{Ok(())}}
fn null(row:&SqliteRow,index:usize)->Result<(),ValueError>{if row.values.get(index)==Some(&SqliteValue::Null){Ok(())}else{Err(ValueError::new(ValueRefusalKind::InvalidValue,"unexpected IFC2x3 value variant field"))}}
fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"IFC ordinal exceeds signed64"))}
fn word(text:&str)->Result<u64,ValueError>{if text.len()>20{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"IFC2x3 identity exceeds unsigned64"));}let value=text.parse::<u64>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"IFC identity is not an unsigned64 decimal word"))?;if UnsignedWord::new(value).text()!=text{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"IFC2x3 identity must be canonical unsigned64 decimal text"));}Ok(value)}
/// 🏷️ Controlled scans of the exact IFC2x3 schema and MVD header positions.
pub fn mvd_header(snapshot:&Ifc2x3Snapshot,view:&str,control:&mut SqliteSnapshotControl<'_>)->Result<(bool,bool),ValueError>{if view.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"IFC view name must not be empty"));}control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;let mut schema=false;let mut found=false;let mut count=0usize;for value in &snapshot.document.header.file_schema{if let Part21Value::List(items)=value{for item in items{schema|=item.as_str()==Some("IFC2X3");count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 header count overflow"))?;control.check_rows(count)?;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}}}count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 header count overflow"))?;control.check_rows(count)?;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}}
    if let Some(Part21Value::List(items))=snapshot.document.header.file_description.first(){for item in items{if let Part21Value::Str(text)=item{control.check_value_bytes(text.len())?;let bytes=text.as_bytes();for start in (0..bytes.len()).step_by(65536){control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,start,bytes.len())?;let end=(start+65536+view.len().saturating_sub(1)).min(bytes.len());if bytes[start..end].windows(view.len()).any(|candidate|candidate==view.as_bytes()){found=true;break;}}}count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 view count overflow"))?;control.check_rows(count)?;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}}}Ok((schema,found))
}
/// 🔎️ Borrows matching IFC2x3 instances and checkpoints every retained entity type.
pub fn mvd_instances<'a>(snapshot:&'a Ifc2x3Snapshot,name:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a Part21Instance>,ValueError>{
    control.check_rows(snapshot.document.instances.len())?;let maximum=control.limits().max_rows;
    control.allocation_stage(SqliteSnapshotPhase::ProjectSnapshot,|remaining,progress|{
        let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
        let mut native=semio_framework_value::NativeEncodeControl::new(remaining,&mut callback);
        let result=(||{native.begin_stage(0)?;let mut result=native.allocate_vec(snapshot.document.instances.len())?;let mut count=0usize;for instance in &snapshot.document.instances{let mut matched=false;for(entity,_)in &instance.entities{matched|=entity.eq_ignore_ascii_case(name);count=count.checked_add(1).filter(|count|*count<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 MVD count exceeds limit"))?;native.step()?;}if matched{result.push(instance);}count=count.checked_add(1).filter(|count|*count<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 MVD count exceeds limit"))?;native.step()?;}Ok(result)})();(result,native.owned_bytes())
    })?
}
/// 🧭️ Paid identity ordering preserves the first actual native instance for each literal word.
pub struct MvdIdentityIndex<'a>{snapshot:&'a Ifc2x3Snapshot,entries:Vec<(u64,usize)>}
impl<'a>MvdIdentityIndex<'a>{
    /// 🔗️ Resolves a literal identity through the paid borrowed ordering.
    pub fn resolve(&self,id:u64)->Option<&'a Part21Instance>{self.entries.binary_search_by_key(&id,|entry|entry.0).ok().map(|index|&self.snapshot.document.instances[self.entries[index].1])}
}
/// 🗂️ Admits concrete borrowed identity backing before controlled sorting and first-owner selection.
pub fn mvd_identity_index<'a>(snapshot:&'a Ifc2x3Snapshot,control:&mut SqliteSnapshotControl<'_>)->Result<MvdIdentityIndex<'a>,ValueError>{
    control.check_rows(snapshot.document.instances.len())?;
    control.allocation_stage(SqliteSnapshotPhase::ProjectSnapshot,|remaining,progress|{
        let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
        let mut native=semio_framework_value::NativeEncodeControl::new(remaining,&mut callback);
        let result=(||{native.begin_stage(0)?;let mut entries=native.allocate_vec(snapshot.document.instances.len())?;for(index,instance)in snapshot.document.instances.iter().enumerate(){entries.push((instance.id,index));native.step()?;}sort_paid(&mut entries,|left,right|{native.step()?;Ok(left.cmp(right))})?;let mut kept=0;for index in 0..entries.len(){if kept==0||entries[kept-1].0!=entries[index].0{entries[kept]=entries[index];kept+=1;}native.step()?;}entries.truncate(kept);Ok(MvdIdentityIndex{snapshot,entries})})();(result,native.owned_bytes())
    })?
}
/// 🧩️ Borrows one exact matching complex instance type with controlled traversal.
pub fn mvd_entity<'a>(instance:&'a Part21Instance,name:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Option<&'a [Part21Value]>,ValueError>{control.check_rows(instance.entities.len())?;for(index,(entity,args))in instance.entities.iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index,instance.entities.len())?;}if entity.eq_ignore_ascii_case(name){return Ok(Some(args));}}Ok(None)}
/// 🏠️ Checks COBie's actual Name field without an uninterruptible whitespace scan.
pub fn mvd_nonempty_name(text:&str,control:&mut SqliteSnapshotControl<'_>)->Result<bool,ValueError>{control.check_value_bytes(text.len())?;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,text.len())?;for(index,c)in text.chars().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index,text.len())?;}if !c.is_whitespace(){return Ok(true);}}Ok(false)}
enum Owner{Header(&'static str,usize),Argument(i64,usize),List(i64,usize),Typed(i64,usize)}
fn project_value(root:&Part21Value,owner:Owner,identities:&[(u64,i64)],p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let mut pending=p.allocate_frontier(1)?;pending.push((root,owner));while let Some((value,owner))=pending.pop(){p.checkpoint()?;let mut cells=[Cell::Null;11];let reference;cells[0]=Cell::Text(match value{Part21Value::Unset=>"unset",Part21Value::Derived=>"derived",Part21Value::Int(_)=>"int",Part21Value::Real(_)=>"real",Part21Value::Str(_)=>"str",Part21Value::Enum(_)=>"enum",Part21Value::Ref(_)=>"ref",Part21Value::List(_)=>"list",Part21Value::Typed{..}=>"typed"});match value{Part21Value::Int(v)=>cells[1]=Cell::Integer(*v),Part21Value::Real(v)=>{cells[2]=Cell::Integer(i64::from(v.negative));cells[3]=Cell::Text(&v.coefficient);cells[4]=Cell::Integer(i64::from(v.scale));cells[5]=v.exponent.map_or(Cell::Null,|v|Cell::Integer(i64::from(v)));},Part21Value::Str(v)=>cells[6]=Cell::Text(v),Part21Value::Enum(v)=>cells[7]=Cell::Text(v),Part21Value::Ref(v)=>{reference=UnsignedWord::new(*v);cells[8]=Cell::Text(reference.text());cells[9]=identities.binary_search_by_key(v,|entry|entry.0).ok().map(|index|identities[index].1).map_or(Cell::Null,Cell::Integer);},Part21Value::Typed{name,..}=>cells[10]=Cell::Text(name),_=>{}}let id=p.insert("ifc2x3_value",&cells)?;match owner{Owner::Header(table,index)=>{p.insert(table,&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;},Owner::Argument(parent,index)=>{p.insert("ifc2x3_entity_argument",&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;},Owner::List(parent,index)=>{p.insert("ifc2x3_list_element",&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;},Owner::Typed(parent,index)=>{p.insert("ifc2x3_typed_argument",&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;}}match value{Part21Value::List(items)|Part21Value::Typed{items,..}=>{p.check_rows(pending.len().checked_add(items.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 pending count overflow"))?)?;for(index,item)in items.iter().enumerate().rev(){p.push_frontier(&mut pending,(item,if matches!(value,Part21Value::List(_)){Owner::List(id,index)}else{Owner::Typed(id,index)}))?;if index%256==0{p.checkpoint()?;}}},_=>{}}}Ok(())}
fn visit_rows(snapshot:&Ifc2x3Snapshot,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let h=&snapshot.document.header;let mut predicted=2usize+usize::from(snapshot.edm_preamble.is_some());for size in [snapshot.document.instances.len(),h.file_description.len(),h.file_name.len(),h.file_schema.len()]{predicted=predicted.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 projected count overflow"))?;}p.check_rows(predicted)?;p.checkpoint()?;let mut identities=p.allocate_frontier(snapshot.document.instances.len())?;
        for(index,instance)in snapshot.document.instances.iter().enumerate(){identities.push((instance.id,ordinal(index+1)?));p.checkpoint()?;}
        sort_paid(&mut identities,|left,right|{p.checkpoint()?;Ok(left.0.cmp(&right.0))})?;
        for pair in identities.windows(2){if pair[0].0==pair[1].0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate IFC2x3 instance identifier"));}p.checkpoint()?;}
        p.insert_key("ifc2x3_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(1),if snapshot.edm_preamble.is_some(){Cell::Integer(1)}else{Cell::Null}])?;p.insert_key("ifc2x3_header",1,&[])?;if let Some(e)=&snapshot.edm_preamble{p.insert_key("ifc2x3_edm_preamble",1,&[Cell::Text(&e.producer),Cell::Text(&e.module),Cell::Text(&e.creation_date),Cell::Text(&e.host),Cell::Text(&e.database),Cell::Text(&e.database_version),Cell::Text(&e.database_creation_date),Cell::Text(&e.schema),Cell::Text(&e.model),Cell::Text(&e.model_creation_date),Cell::Text(&e.header_model),Cell::Text(&e.header_model_creation_date),Cell::Text(&e.user),Cell::Text(&e.group),Cell::Text(&e.license),Cell::Text(&e.options)])?;}
        for(table,values)in [("ifc2x3_file_description_argument",&h.file_description),("ifc2x3_file_name_argument",&h.file_name),("ifc2x3_file_schema_argument",&h.file_schema)]{for(index,value)in values.iter().enumerate(){project_value(value,Owner::Header(table,index),&identities,p)?;}}
        for(index,instance)in snapshot.document.instances.iter().enumerate(){let id=identities[identities.binary_search_by_key(&instance.id,|entry|entry.0).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"missing IFC2x3 identity"))?].1;let word=UnsignedWord::new(instance.id);p.insert_key("ifc2x3_instance",id,&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Text(word.text())])?;p.check_rows(instance.entities.len())?;for(index,(name,args))in instance.entities.iter().enumerate(){let entity=p.insert("ifc2x3_entity_type",&[Cell::Integer(id),Cell::Integer(ordinal(index)?),Cell::Text(name)])?;p.check_rows(args.len())?;for(index,value)in args.iter().enumerate(){project_value(value,Owner::Argument(entity,index),&identities,p)?;}}}Ok(())
    }
impl Ifc2x3Snapshot{pub(crate)fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{semantic::extent(control.limits())?;let mut writer=RowWriter::borrowed(control,phase)?;visit_rows(self,&mut writer)?;writer.finish_borrowed()}pub(crate)fn admit_sqlite_record(value:&semio_framework_dsl_record::RecordValue,limits:SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{semantic::admit_record(value,limits,native)}}
impl ArtifactSqliteSnapshot for Ifc2x3Snapshot{
    fn retire_sqlite_snapshot(self){crate::standards::v2x3::subsets::base::io::sqlite::snapshot::native::close(self)}
    fn decode_sqlite_snapshot_native(payload:&semio_framework_os_kernel::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v2x3::subsets::base::io::sqlite::snapshot::native::decode(payload,control)}
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_os_kernel::io_schema::IoPayload,ValueError>{crate::standards::v2x3::subsets::base::io::sqlite::snapshot::native::encode(self,encoding,control)}
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.ifc"||dialect.standard!="2x3"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown owned IFC2x3 snapshot dialect"));}let root=db.table("ifc2x3_document")?.single_row()?;if root.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"IFC2x3 document identity differs from its projection"));}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cv20"=>crate::standards::v2x3::subsets::cv20::schema::check_cv20_conformance_controlled(self,control)?,"sav"=>crate::standards::v2x3::subsets::sav::schema::check_sav_conformance_controlled(self,control)?,"cobie"=>crate::standards::v2x3::subsets::cobie::schema::check_cobie_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown owned IFC2x3 snapshot subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    })().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{native_backing::preflight(self,control)}
    const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
    fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::extent(control.limits())?;let mut p=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut p)?;p.finish()}
    fn from_sqlite_database(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{validate_sqlite_database_schema(db,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(db,SqliteSnapshotPhase::ReconstructSnapshot)?;reconstruction::read(db,control)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
