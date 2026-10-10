//! 🫙️ Original authored SQL tables, row census and partial fields share one caller-owned prefix.
use super::{Cell,FloatColumn,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use crate::{SqliteRow,SqliteValue,transfer::{SchemaValidationStorage,construct_database_into}};
use semio_framework_value::retirement::{controlled::ControlledRetirement,RetireOwned};

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"SQL projection backing overflow")}
fn add(a:usize,b:usize)->Result<usize,ValueError>{a.checked_add(b).ok_or_else(overflow)}
fn reserve<T>(values:&mut Vec<T>,count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if values.capacity()!=0{return Err(invalid("projection original backing is already reserved"))}
 let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(overflow)?;
 control.admit_allocation_bytes(bytes)?;
 values.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQL projection original allocation"))
}

/// 🧾️ The actual output and every schema/census allocation survive all interrupted field work.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct ProjectionStorage{database:SqliteDatabase,schema:SchemaValidationStorage,census:Vec<usize>,rows:usize,bytes:usize,prepared:bool,pending:bool}
impl ProjectionStorage{
 pub fn empty()->Self{Self{database:SqliteDatabase{tables:Vec::new()},schema:SchemaValidationStorage::empty(),census:Vec::new(),rows:0,bytes:0,prepared:false,pending:false}}
 /// 🏛️ Initializes exact authored tables directly inside this original prefix.
 pub fn initialize(&mut self,sql:&str,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  if !self.database.tables.is_empty()||self.database.tables.capacity()!=0||self.census.capacity()!=0{return Err(invalid("projection original has already been initialized"))}
  construct_database_into(&mut self.schema,&mut self.database,sql,SqliteSnapshotPhase::ProjectSnapshot,control)?;
  reserve(&mut self.census,self.database.tables.len(),control)?;
  self.census.resize(self.database.tables.len(),0);
  Ok(())
 }
 fn table_index(&self,table:&str)->Result<usize,ValueError>{self.database.tables.iter().position(|value|value.name.eq_ignore_ascii_case(table)).ok_or_else(||invalid("projection table is not declared"))}
 /// 🔢️ Allocates surrogate identities independently in each original table during the census.
 pub fn count_row(&mut self,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<i64,ValueError>{
  if self.prepared{return Err(invalid("projection census is already complete"))}
  let index=self.table_index(table)?;let rows=add(self.rows,1)?;control.check_rows(rows)?;
  let(_,bytes)=row_layout(cells,columns,control)?;let bytes=add(self.bytes,bytes)?;control.check_value_bytes(bytes)?;
  let next=add(self.census[index],1)?;let key=i64::try_from(next).map_err(|_|overflow())?;
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;
  self.census[index]=next;self.rows=rows;self.bytes=bytes;Ok(key)
 }
 /// 🧮️ Reserves each actual table row vector once, using its completed exact census.
 pub fn prepare_rows(&mut self,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  if self.prepared{return Err(invalid("projection rows are already prepared"))}
  for(index,table)in self.database.tables.iter_mut().enumerate(){control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index,self.census.len())?;reserve(&mut table.rows,self.census[index],control)?;}
  self.rows=0;self.bytes=0;self.prepared=true;Ok(())
 }
 /// 🫳️ Borrows original output without transferring any live auxiliary owner.
 pub fn database(&self)->&SqliteDatabase{&self.database}
 pub fn row_count(&self)->usize{self.rows}
 /// 🔗️ Inserts the original row before its cell vector or any text/blob allocation can fail.
 pub fn insert_key(&mut self,table:&str,key:i64,cells:&[Cell<'_>],control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.insert_key_float(table,key,cells,&[],control)}
 /// ➕️ Emits the same table-local identity obtained by the authored census.
 pub fn insert(&mut self,table:&str,cells:&[Cell<'_>],control:&mut SqliteSnapshotControl<'_>)->Result<i64,ValueError>{self.insert_float(table,cells,&[],control)}
 /// 🧬️ Streams every IEEE companion directly into the same original row.
 pub fn insert_float(&mut self,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<i64,ValueError>{
  let index=self.table_index(table)?;let key=i64::try_from(add(self.database.tables[index].rows.len(),1)?).map_err(|_|overflow())?;
  self.insert_key_float(table,key,cells,columns,control)?;Ok(key)
 }
 pub fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  if !self.prepared||self.pending{return Err(invalid("projection has no ready original row slot"))}
  let index=self.table_index(table)?;
  if self.database.tables[index].rows.len()>=self.census[index]{return Err(invalid("projection exceeded its original table census"))}
  let(width,bytes)=row_layout(cells,columns,control)?;
  let bytes=add(self.bytes,bytes)?;let rows=add(self.rows,1)?;
  control.check_rows(rows)?;control.check_value_bytes(bytes)?;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;
  self.pending=true;
  self.database.tables[index].rows.push(SqliteRow{rowid:key,values:Vec::new()});
  let row=self.database.tables[index].rows.last_mut().unwrap();reserve(&mut row.values,width,control)?;
  row.values.push(SqliteValue::Integer(key));
  for cell in cells.iter().copied(){let cell=match cell{Cell::Real(value)if value.is_nan()=>Cell::Null,Cell::Float32(value)if value.is_nan()=>Cell::Null,other=>other};copy_cell_into(&mut row.values,cell,control)?;}
  for column in columns.iter().copied(){
   let value=cells[column.index()-1];
   match(column,value){
    (_,Cell::Null)=>{row.values.push(SqliteValue::Null);row.values.push(SqliteValue::Null);}
    (FloatColumn::Binary64(_),Cell::Real(value))=>{row.values.push(SqliteValue::Integer(value.to_bits()as i64));copy_cell_into(&mut row.values,Cell::Text(super::numeric_class(value)),control)?;}
    (FloatColumn::Binary32(_),Cell::Float32(value))=>{row.values.push(SqliteValue::Integer(i64::from(value.to_bits())));copy_cell_into(&mut row.values,Cell::Text(super::numeric_class(f64::from(value))),control)?;}
    _=>return Err(invalid("original IEEE source changed after admission"))
   }
  }
  self.pending=false;self.rows=rows;self.bytes=bytes;Ok(())
 }
 /// 🏁️ Validates the same original census and orders physical identities before output handoff.
 pub fn finish(&mut self,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  if !self.prepared||self.pending{return Err(invalid("projection retains an unfinished original row"))}
  let mut work=super::RowOrdering{control,units:0};
  for(index,table)in self.database.tables.iter_mut().enumerate(){if table.rows.len()!=self.census[index]{return Err(invalid("projection omitted an original census row"))}super::order_rows(&mut table.rows,&mut work)?;}
  work.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,work.units,work.units)
 }
 /// 📤️ Transfers completed output while the caller retains this prefix's auxiliary owners.
 pub fn take_database(&mut self)->SqliteDatabase{std::mem::replace(&mut self.database,SqliteDatabase{tables:Vec::new()})}
}

fn row_layout(cells:&[Cell<'_>],columns:&[FloatColumn],control:&SqliteSnapshotControl<'_>)->Result<(usize,usize),ValueError>{
  let width=add(add(cells.len(),1)?,columns.len().checked_mul(2).ok_or_else(overflow)?)?;
  if width>control.limits().max_columns{return Err(invalid("projection row exceeds declared column limit"))}
  let mut bytes=8usize;
  for (position,column)in columns.iter().copied().enumerate(){
   let source=column.index().checked_sub(1).ok_or_else(||invalid("IEEE column cannot be an identity"))?;
   if columns[..position].iter().any(|previous|previous.index()==column.index()){return Err(invalid("duplicate authored IEEE scalar column"))}
   let value=*cells.get(source).ok_or_else(||invalid("missing authored IEEE scalar"))?;
   let scalar=match(column,value){(_,Cell::Null)=>continue,(FloatColumn::Binary64(_),Cell::Real(value))=>value,(FloatColumn::Binary32(_),Cell::Float32(value))=>f64::from(value),_=>return Err(invalid("native IEEE width differs from authored scalar"))};
   bytes=add(bytes,add(8,super::numeric_class(scalar).len())?)?;
  }
  for(position,cell)in cells.iter().copied().enumerate(){
   let nan=matches!(cell,Cell::Real(value)if value.is_nan())||matches!(cell,Cell::Float32(value)if value.is_nan());
   if nan&&!columns.iter().any(|column|column.index()==position+1){return Err(invalid("NaN requires an authored IEEE scalar companion"))}
   bytes=add(bytes,if nan{0}else{cell.bytes()})?;
  }
  Ok((width,bytes))
}

fn copy_cell_into(values:&mut Vec<SqliteValue>,cell:Cell<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 match cell{
  Cell::Null=>values.push(SqliteValue::Null),Cell::Integer(value)=>values.push(SqliteValue::Integer(value)),Cell::Real(value)=>values.push(SqliteValue::Real(value)),Cell::Float32(value)=>values.push(SqliteValue::Real(f64::from(value))),
  Cell::Text(source)=>{values.push(SqliteValue::Text(String::new()));let SqliteValue::Text(output)=values.last_mut().unwrap()else{unreachable!()};copy_text_into(output,source,SqliteSnapshotPhase::ProjectSnapshot,control)?;}
  Cell::Blob(source)=>{values.push(SqliteValue::Blob(Vec::new()));let SqliteValue::Blob(output)=values.last_mut().unwrap()else{unreachable!()};reserve(output,source.len(),control)?;for chunk in source.chunks(65536){output.extend_from_slice(chunk);control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,output.len(),source.len())?;}control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,source.len(),source.len())?;}
  Cell::PagedText(source)=>{
   values.push(SqliteValue::Text(String::new()));let SqliteValue::Text(output)=values.last_mut().unwrap()else{unreachable!()};
   let length=source.text_bytes();control.admit_allocation_bytes(length)?;output.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQL projection paged text allocation"))?;
   for index in 0..source.text_chunk_count(){let chunk=source.text_chunk(index).ok_or_else(||invalid("paged projection omitted a declared chunk"))?;if output.len().checked_add(chunk.len()).is_none_or(|next|next>length){return Err(invalid("paged projection exceeds declared bytes"))}let mut at=0;while at<chunk.len(){let mut end=at.saturating_add(65536).min(chunk.len());while !chunk.is_char_boundary(end){end+=1;}output.push_str(&chunk[at..end]);at=end;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,output.len(),length)?;}}
   if output.len()!=length{return Err(invalid("paged projection differs from declared bytes"))}control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,length,length)?;
  }
 }
 Ok(())
}

/// 🧵️ Copies borrowed UTF8 directly into an original schema or row field before each checkpoint.
pub(crate) fn copy_text_into(output:&mut String,source:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if output.capacity()!=0{return Err(invalid("projection text already owns backing"))}
 control.admit_allocation_bytes(source.len())?;output.try_reserve_exact(source.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"SQL projection text allocation"))?;
 let mut at=0;while at<source.len(){let mut end=at.saturating_add(65536).min(source.len());while !source.is_char_boundary(end){end+=1;}output.push_str(&source[at..end]);at=end;control.checkpoint(phase,at,source.len())?;}
 control.checkpoint(phase,source.len(),source.len())
}

/// 🏗️ Admits the genuine typed prefix before any owned schema, frontier or cell construction.
pub fn project_owned<T:RetireOwned,R>(control:&mut SqliteSnapshotControl<'_>,empty:impl FnOnce()->T,build:impl FnOnce(&mut T,&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>,take:impl FnOnce(&mut T)->R)->Result<R,ValueError>{owned_workspace(control,SqliteSnapshotPhase::ProjectSnapshot,empty,build,take)}

/// 🧭️ Admits one original typed workspace under the caller's actual production or semantic phase.
pub fn owned_workspace<T:RetireOwned,R>(control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,empty:impl FnOnce()->T,build:impl FnOnce(&mut T,&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>,take:impl FnOnce(&mut T)->R)->Result<R,ValueError>{
 if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"projection prefix has no original controlled retirement"))}
 control.with_retirement_owner(phase,size_of::<ControlledRetirement<T>>(),|control|{
  let original=empty();
  let owner=match ControlledRetirement::new(original){Ok(owner)=>owner,Err((error,_))=>return(Err(error),None)};
  let mut owner=Box::new(owner);
  let result=build(owner.original_mut().unwrap(),control).and_then(|()|{control.checkpoint(phase,0,0)?;Ok(take(owner.original_mut().unwrap()))});
  (result,Some(owner as Box<dyn semio_framework_value::ErasedSnapshotRetirement>))
 })
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     