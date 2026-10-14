//! 🔤️ Complete authored text SQL work remains in the original retained receiving frame.
use semio_framework_os_kernel::sqlite_snapshot::receiving::{SqliteSnapshotDecodeReceiving,SqliteSnapshotEncodeReceiving};
use super::{SemioTextSnapshot,SemioTextRun,SemioTextMark,SemioTextMarkKind};
use semio_framework_os_kernel::{io::control::{NativeSnapshotBodyWallet,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner},sqlite_snapshot::{SqliteDatabase,SqliteTable,SqliteRow,SqliteValue,SqliteSnapshotControl,artifact::receiving::{Port,Projection,Scalar,row,cell,text_cell,push_row,sort,integer,ordinal,identity_position,parent_range,validate_schema_sql,check_database}}};
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::mem::size_of;
type Result<T>=std::result::Result<T,ValueError>;
type Paid<'a,'b,'c>=Port<'a,'b,'c,NativeSnapshotBodyWallet>;
const SQL:&str=include_str!("../🗄️.sql");
const TABLES:[(&str,usize);3]=[("semio_text_document",2),("semio_text_run",5),("semio_text_mark",5)];
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Semio text complete semantic state differs")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Semio text extent overflow"))}
fn empty()->SemioTextSnapshot{SemioTextSnapshot{schema:String::new(),runs:Vec::new()}}
#[derive(semio_framework_value::RetireOwned)]
struct Frame{projection:Projection,snapshot:SemioTextSnapshot,run_ids:Vec<usize>,run_order:Vec<usize>,mark_ids:Vec<usize>,mark_order:Vec<usize>,status:Option<Result<()>>}
impl Frame{fn new()->Self{Self{projection:Projection::new(),snapshot:empty(),run_ids:Vec::new(),run_order:Vec::new(),mark_ids:Vec::new(),mark_order:Vec::new(),status:None}}}
fn definition(index:usize)->&'static str{SQL.split(';').map(str::trim).filter(|text|!text.is_empty()).nth(index).unwrap()}
fn table(database:&SqliteDatabase,index:usize)->Result<&SqliteTable>{database.tables.iter().find(|table|table.name.eq_ignore_ascii_case(TABLES[index].0)).ok_or_else(invalid)}
fn text(row:&SqliteRow,column:usize)->Result<&str>{match row.values.get(column){Some(SqliteValue::Text(text))=>Ok(text),_=>Err(invalid())}}
fn mark_kind(value:&str)->Result<SemioTextMarkKind>{match value{"bold"=>Ok(SemioTextMarkKind::Bold),"italic"=>Ok(SemioTextMarkKind::Italic),"code"=>Ok(SemioTextMarkKind::Code),"link"=>Ok(SemioTextMarkKind::Link),_=>Err(invalid())}}
fn equal(left:&str,right:&str,port:&mut Paid<'_,'_,'_>)->Result<()>{port.work(2,0)?;if left.len()!=right.len(){return Err(invalid())}let total=left.len();port.control.checkpoint(port.phase(),0,total)?;for(index,(left,right))in left.as_bytes().chunks(256).zip(right.as_bytes().chunks(256)).enumerate(){port.work(2,0)?;if left!=right{return Err(invalid())}port.control.checkpoint(port.phase(),index*256+left.len(),total)?}Ok(())}
fn copy(value:&str,output:&mut String,port:&mut Paid<'_,'_,'_>,depth:usize)->Result<()>{port.control.admit_reconstruction_bytes(value.len())?;port.text(value,output,depth)}
fn indexes(rows:&[SqliteRow],ids:&mut Vec<usize>,ordered:&mut Vec<usize>,parent:bool,port:&mut Paid<'_,'_,'_>)->Result<()>{
 port.vector(rows.len(),ids,2)?;port.vector(rows.len(),ordered,2)?;
 for(index,row)in rows.iter().enumerate(){port.work(2,2*size_of::<usize>())?;if row.values.len()!=5||integer(row,0)?!=row.rowid||integer(row,2)?<0{return Err(invalid())}text(row,3)?;text(row,4)?;ids.push(index);ordered.push(index);}
 sort(ids,port,|a,b|Ok(rows[a].rowid.cmp(&rows[b].rowid)))?;for pair in ids.windows(2){port.work(2,0)?;if rows[pair[0]].rowid==rows[pair[1]].rowid{return Err(invalid())}}
 sort(ordered,port,|a,b|Ok(if parent{(integer(&rows[a],1)?,integer(&rows[a],2)?).cmp(&(integer(&rows[b],1)?,integer(&rows[b],2)?))}else{integer(&rows[a],2)?.cmp(&integer(&rows[b],2)?)}))
}
fn project(source:&SemioTextSnapshot,frame:&mut Frame,port:&mut Paid<'_,'_,'_>)->Result<()>{
 port.work(1,0)?;let mut marks=0usize;for run in &source.runs{port.work(2,0)?;marks=add(marks,run.marks.len())?}let total=add(add(source.runs.len(),marks)?,1)?;port.control.check_rows(total)?;let limits=port.control.limits();let schema=TABLES.iter().enumerate().try_fold(0usize,|sum,(index,(name,_))|add(sum,add(name.len(),definition(index).len())?))?;
 if limits.max_tables<3||limits.max_columns<5||SQL.len()>limits.max_schema_bytes||schema>limits.max_schema_bytes{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Semio text authored schema exceeds original allowance"))}
 let output=&mut frame.projection;port.vector(3,&mut output.database.tables,1)?;for(index,(name,_))in TABLES.iter().enumerate(){port.work(2,size_of::<SqliteTable>())?;output.database.tables.push(SqliteTable{name:String::new(),sql:String::new(),rows:Vec::new()});let table=&mut output.database.tables[index];port.text(name,&mut table.name,2)?;port.text(definition(index),&mut table.sql,2)?;port.vector(match index{0=>1,1=>source.runs.len(),_=>marks},&mut table.rows,2)?;}
 row(output,1,2,port)?;text_cell(output,&source.schema,port)?;push_row(output,0,port)?;let mut mark_id=0usize;
 for(index,run)in source.runs.iter().enumerate(){let id=ordinal(add(index,1)?)?;row(output,id,5,port)?;cell(output,Scalar::Integer(1),port)?;cell(output,Scalar::Integer(ordinal(index)?),port)?;text_cell(output,&run.language,port)?;text_cell(output,&run.content,port)?;push_row(output,1,port)?;
  for(index,mark)in run.marks.iter().enumerate(){mark_id=add(mark_id,1)?;row(output,ordinal(mark_id)?,5,port)?;cell(output,Scalar::Integer(id),port)?;cell(output,Scalar::Integer(ordinal(index)?),port)?;text_cell(output,super::kind(mark.kind),port)?;text_cell(output,&mark.href,port)?;push_row(output,2,port)?;port.control.checkpoint(port.phase(),mark_id,marks)?;}
 }port.control.checkpoint(port.phase(),total,total)
}
fn read(database:&SqliteDatabase,frame:&mut Frame,source:Option<&SemioTextSnapshot>,port:&mut Paid<'_,'_,'_>)->Result<()>{
 validate_schema_sql(database,SQL,port)?;check_database(database,port)?;let documents=&table(database,0)?.rows;port.work(2,0)?;if documents.len()!=1{return Err(invalid())}let document=&documents[0];if document.rowid!=1||document.values.len()!=2||integer(document,0)?!=1{return Err(invalid())}let schema=text(document,1)?;
 let runs=&table(database,1)?.rows;let marks=&table(database,2)?.rows;indexes(runs,&mut frame.run_ids,&mut frame.run_order,false,port)?;indexes(marks,&mut frame.mark_ids,&mut frame.mark_order,true,port)?;
 for(position,index)in frame.run_order.iter().copied().enumerate(){port.work(2,0)?;if integer(&runs[index],1)?!=1||integer(&runs[index],2)?!=ordinal(position)?{return Err(invalid())}}
 for index in &frame.mark_ids{port.work(2,0)?;let row=&marks[*index];if identity_position(runs,&frame.run_ids,integer(row,1)?,port)?.is_none(){return Err(invalid())}mark_kind(text(row,3)?)?;}
 if let Some(source)=source{if source.runs.len()!=runs.len(){return Err(invalid())}equal(&source.schema,schema,port)?}else{copy(schema,&mut frame.snapshot.schema,port,2)?;port.vector(runs.len(),&mut frame.snapshot.runs,2)?;}
 for(position,index)in frame.run_order.iter().copied().enumerate(){port.work(2,size_of::<SemioTextRun>())?;let row=&runs[index];let range=parent_range(marks,&frame.mark_order,row.rowid,1,port)?;
  if let Some(source)=source{let run=&source.runs[position];if run.marks.len()!=range.len(){return Err(invalid())}equal(&run.language,text(row,3)?,port)?;equal(&run.content,text(row,4)?,port)?;}else{frame.snapshot.runs.push(SemioTextRun{language:String::new(),content:String::new(),marks:Vec::new()});let run=frame.snapshot.runs.last_mut().unwrap();port.vector(range.len(),&mut run.marks,3)?;copy(text(row,3)?,&mut run.language,port,3)?;copy(text(row,4)?,&mut run.content,port,3)?;}
  for(ordinal_index,ordered_index)in range.enumerate(){port.work(3,size_of::<SemioTextMark>())?;let row=&marks[frame.mark_order[ordered_index]];if integer(row,2)?!=ordinal(ordinal_index)?{return Err(invalid())}let kind=mark_kind(text(row,3)?)?;
   if let Some(source)=source{let mark=&source.runs[position].marks[ordinal_index];if kind!=mark.kind{return Err(invalid())}equal(&mark.href,text(row,4)?,port)?;}else{let run=frame.snapshot.runs.last_mut().unwrap();run.marks.push(SemioTextMark{kind,href:String::new()});copy(text(row,4)?,&mut run.marks.last_mut().unwrap().href,port,4)?;}
  }port.control.checkpoint(port.phase(),position+1,runs.len())?;
 }port.control.checkpoint(port.phase(),runs.len(),runs.len())
}
impl semio_framework_os_kernel::sqlite_snapshot::receiving::NativeSqliteReceivingFrame for Frame{fn receiving_status(&mut self)->&mut Option<Result<()>>{&mut self.status}}
pub(super)fn project_receiving(source:&SemioTextSnapshot,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase>{owner.receive_sqlite_snapshot(control,Frame::new,|frame,port|project(source,frame,port),|frame|Ok(std::mem::replace(&mut frame.projection.database,SqliteDatabase{tables:Vec::new()})))}
pub(super)fn reconstruct_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->Result<SemioTextSnapshot>{owner.receive_sqlite_snapshot(control,Frame::new,|frame,port|read(database,frame,None,port),|frame|Ok(std::mem::replace(&mut frame.snapshot,empty())))}
pub(super)fn validate_decoding(source:&SemioTextSnapshot,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<()>{owner.receive_sqlite_snapshot(control,Frame::new,|frame,port|read(database,frame,Some(source),port),|_|Ok(()))}
pub(super)fn validate_encoding(source:&SemioTextSnapshot,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->Result<()>{owner.receive_sqlite_snapshot(control,Frame::new,|frame,port|read(database,frame,Some(source),port),|_|Ok(()))}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

