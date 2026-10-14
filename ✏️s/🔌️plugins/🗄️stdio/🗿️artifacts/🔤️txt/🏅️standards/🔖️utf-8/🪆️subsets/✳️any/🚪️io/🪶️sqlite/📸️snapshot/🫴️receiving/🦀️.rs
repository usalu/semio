//! 🫴️ TXT semantic rows and ordered lines remain in original paid receiving custody.
use super::{LineEnding,TxtSnapshot};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteTable,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::receiving::{Direction,Projection,Scalar,AuthoredTable,cell,text_cell,row,push_row,sort,ordinal,validate_schema,check_database,settle}};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
use store::NativeSnapshotBodyWallet;
use std::{cell::{Cell,RefCell},mem::size_of};
type Result<T>=std::result::Result<T,ValueError>;
type Port<'a,'b,'c>=semio_framework_os_kernel::sqlite_snapshot::artifact::receiving::Port<'a,'b,'c,NativeSnapshotBodyWallet>;
const SQL:&str=include_str!("../🗄️.sql");
const NAMES:[&str;2]=["text_document","text_line"];
fn refused(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::literal(kind,message)}
fn invalid(message:&'static str)->ValueError{refused(ValueRefusalKind::InvalidValue,message)}
fn definition(index:usize)->&'static str{SQL.split(';').filter(|word|!word.trim().is_empty()).nth(index).unwrap().trim()}
fn empty_snapshot()->TxtSnapshot{TxtSnapshot{schema:String::new(),lines:Vec::new(),trailing_newline:false,line_ending:LineEnding::Lf}}
fn table<'a>(database:&'a SqliteDatabase,name:&str)->Result<&'a SqliteTable>{database.tables.iter().find(|table|table.name.eq_ignore_ascii_case(name)).ok_or_else(||invalid("TXT authored table is missing"))}
fn integer(row:&SqliteRow,column:usize)->Result<i64>{match row.values.get(column){Some(SqliteValue::Integer(value))=>Ok(*value),_=>Err(invalid("TXT integer cell is invalid"))}}
fn text(row:&SqliteRow,column:usize)->Result<&str>{match row.values.get(column){Some(SqliteValue::Text(value))=>Ok(value),_=>Err(invalid("TXT text cell is invalid"))}}
fn project(source:&TxtSnapshot,frame:&mut Projection,port:&mut Port<'_,'_,'_>)->Result<()>{
 let total=source.lines.len().checked_add(1).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"TXT entity census overflow"))?;port.control.check_rows(total)?;
 let limits=port.control.limits();let schema_bytes=(0..2).map(|index|NAMES[index].len()+definition(index).len()).sum::<usize>();if limits.max_tables<2||limits.max_columns<4||schema_bytes>limits.max_schema_bytes{return Err(refused(ValueRefusalKind::OwnershipLimit,"TXT authored schema exceeds original ceiling"))}
 port.vector(2,&mut frame.database.tables,1)?;
 for(index,count)in[1,source.lines.len()].into_iter().enumerate(){port.work(2,size_of::<SqliteTable>())?;frame.database.tables.push(SqliteTable{name:String::new(),sql:String::new(),rows:Vec::new()});let table=&mut frame.database.tables[index];port.text(NAMES[index],&mut table.name,2)?;port.text(definition(index),&mut table.sql,2)?;port.vector(count,&mut table.rows,2)?;}
 row(frame,1,4,port)?;text_cell(frame,&source.schema,port)?;cell(frame,Scalar::Integer(i64::from(source.trailing_newline)),port)?;text_cell(frame,if source.line_ending==LineEnding::Lf{"lf"}else{"crlf"},port)?;push_row(frame,0,port)?;
 for(index,line)in source.lines.iter().enumerate(){row(frame,ordinal(index.checked_add(1).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"TXT line identity overflow"))?)?,4,port)?;cell(frame,Scalar::Integer(1),port)?;cell(frame,Scalar::Integer(ordinal(index)?),port)?;text_cell(frame,line,port)?;push_row(frame,1,port)?;}
 port.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,total,total)
}
#[derive(semio_framework_value::RetireOwned)]
struct Reconstruction{snapshot:TxtSnapshot,lines:Vec<usize>,identities:Vec<usize>}
impl Reconstruction{fn new()->Self{Self{snapshot:empty_snapshot(),lines:Vec::new(),identities:Vec::new()}}}
fn reconstruct(database:&SqliteDatabase,frame:&mut Reconstruction,port:&mut Port<'_,'_,'_>)->Result<()>{
 let authored=[0,1].map(|index|AuthoredTable{name:NAMES[index],declaration:definition(index),columns:4});validate_schema(database,&authored,port)?;check_database(database,port)?;
 let documents=&table(database,NAMES[0])?.rows;if documents.len()!=1{return Err(invalid("TXT requires one document row"))}let document=&documents[0];if document.values.len()!=4||document.rowid<=0||integer(document,0)?!=document.rowid{return Err(invalid("TXT document identity or width is invalid"))}
 let root=document.rowid;let trailing=match integer(document,2)?{0=>false,1=>true,_=>return Err(invalid("TXT trailing newline requires zero or one"))};let ending=match text(document,3)?{"lf"=>LineEnding::Lf,"crlf"=>LineEnding::CrLf,_=>return Err(invalid("TXT line ending is invalid"))};port.work(1,size_of::<bool>()+size_of::<LineEnding>())?;frame.snapshot.trailing_newline=trailing;frame.snapshot.line_ending=ending;
 let rows=&table(database,NAMES[1])?.rows;port.vector(rows.len(),&mut frame.lines,1)?;port.vector(rows.len(),&mut frame.identities,1)?;
 for(index,row)in rows.iter().enumerate(){port.work(1,0)?;if row.values.len()!=4||row.rowid<=0||integer(row,0)?!=row.rowid||integer(row,1)?!=root{return Err(invalid("TXT line identity, relationship or width is invalid"))}port.work(1,2*size_of::<usize>())?;frame.lines.push(index);frame.identities.push(index);}
 sort(&mut frame.lines,port,|a,b|Ok(integer(&rows[a],2)?.cmp(&integer(&rows[b],2)?)))?;sort(&mut frame.identities,port,|a,b|Ok(rows[a].rowid.cmp(&rows[b].rowid)))?;
 for pair in frame.identities.windows(2){port.work(1,0)?;if rows[pair[0]].rowid==rows[pair[1]].rowid{return Err(invalid("TXT line identity is duplicated"))}}
 let schema=text(document,1)?;port.control.admit_reconstruction_bytes(schema.len())?;port.text(schema,&mut frame.snapshot.schema,1)?;port.vector(rows.len(),&mut frame.snapshot.lines,2)?;
 for(position,index)in frame.lines.iter().copied().enumerate(){port.work(2,0)?;let line=&rows[index];if integer(line,2)?!=ordinal(position)?{return Err(invalid("TXT line ordinals must be contiguous and unique"))}let source=text(line,3)?;port.control.admit_reconstruction_bytes(source.len())?;port.work(2,size_of::<String>())?;frame.snapshot.lines.push(String::new());port.text(source,frame.snapshot.lines.last_mut().unwrap(),3)?;}
 let total=rows.len().checked_add(1).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"TXT entity census overflow"))?;port.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)
}
struct Settlement<'a,'b>{control:RefCell<&'a mut SqliteSnapshotControl<'b>>,before:usize,remaining:usize,phase:SqliteSnapshotPhase,post_body:Cell<bool>,cause:RefCell<Option<ValueError>>}
impl<'a,'b> Settlement<'a,'b>{
 fn new(control:&'a mut SqliteSnapshotControl<'b>,before:usize,phase:SqliteSnapshotPhase)->Self{let remaining=control.allocation_remaining_bytes();Self{control:RefCell::new(control),before,remaining,phase,post_body:Cell::new(false),cause:RefCell::new(None)}}
 fn finish_body(&self,owned:usize)->Result<()>{settle(&mut self.control.borrow_mut(),self.remaining,self.before,owned)?;self.post_body.set(true);Ok(())}
 fn observe(&self,owned:usize,completed:usize,total:usize)->bool{if !self.post_body.get(){return true}let mut control=self.control.borrow_mut();let result=settle(&mut control,self.remaining,self.before,owned).and_then(|_|control.checkpoint(self.phase,completed,total));match result{Ok(())=>true,Err(error)=>{*self.cause.borrow_mut()=Some(error);false}}}
 fn refused(&self,owned:usize,error:ValueError)->ValueError{match settle(&mut self.control.borrow_mut(),self.remaining,self.before,owned){Err(cause)=>cause,Ok(())=>self.cause.borrow_mut().take().unwrap_or(error)}}
}
pub(super)fn project_receiving(source:&TxtSnapshot,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase>{
 let result=(||{owner.native().checkpoint()?;let before=owner.native().owned_bytes();let remaining=control.allocation_remaining_bytes();let maximum=before.checked_add(remaining).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"TXT original SQL allocation ceiling overflow"))?;let settlement=Settlement::new(control,before,SqliteSnapshotPhase::ProjectSnapshot);let mut observer=|event:semio_framework_value::native_decoding::NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
 let result=owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Projection,SqliteDatabase>(|slot,native,body|{
  let mut control=settlement.control.borrow_mut();let mut port=Port{control:&mut control,native:Direction::Decode(native),body};port.work(1,size_of::<Projection>())?;*slot=Some(Projection::new());let frame=slot.as_mut().unwrap();project(source,frame,&mut port)?;port.work(1,size_of::<SqliteDatabase>())?;let owned=port.owned();drop(port);drop(control);settlement.finish_body(owned)?;Ok(std::mem::replace(&mut frame.database,SqliteDatabase{tables:Vec::new()}))
 }));match result{Ok(output)=>Ok(output),Err(error)=>Err(settlement.refused(owner.native().owned_bytes(),error))}})();result.map_err(|error|error.with_retained_progress(owner.progress()))
}
pub(super)fn reconstruct_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<TxtSnapshot>{
 let result=(||{owner.native().checkpoint()?;let before=owner.native().owned_bytes();let remaining=control.allocation_remaining_bytes();let maximum=before.checked_add(remaining).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"TXT original SQL allocation ceiling overflow"))?;let settlement=Settlement::new(control,before,SqliteSnapshotPhase::ReconstructSnapshot);let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
 let result=owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Reconstruction,TxtSnapshot>(|slot,native,body|{
  let mut control=settlement.control.borrow_mut();let mut port=Port{control:&mut control,native:Direction::Encode(native),body};port.work(1,size_of::<Reconstruction>())?;*slot=Some(Reconstruction::new());let frame=slot.as_mut().unwrap();reconstruct(database,frame,&mut port)?;port.work(1,size_of::<TxtSnapshot>())?;let owned=port.owned();drop(port);drop(control);settlement.finish_body(owned)?;Ok(std::mem::replace(&mut frame.snapshot,empty_snapshot()))
 }));match result{Ok(output)=>Ok(output),Err(error)=>Err(settlement.refused(owner.native().owned_bytes(),error))}})();result.map_err(|error|error.with_retained_progress(owner.progress()))
}
pub(super)fn validate_subset(dialect:&semio_framework_artifact_reference::ArtifactDialect,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,progress:RetainedCloneProgress,checkpoint:impl FnOnce()->Result<()>)->store::io_schema::IoResult<()>{
 let result=(||{checkpoint()?;control.checkpoint(phase,0,0)?;if dialect.artifact_kind!="s.stdio.txt"||dialect.standard!="utf-8"||dialect.subset!="*"{return Err(refused(ValueRefusalKind::UnsupportedOwner,"unsupported owned TXT SQLite dialect"))}Ok(())})();result.map(|()|store::io_schema::IoOutcome::clean(())).map_err(|error|store::io_schema::IoError::from_value_error(error.with_retained_progress(progress)))
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
