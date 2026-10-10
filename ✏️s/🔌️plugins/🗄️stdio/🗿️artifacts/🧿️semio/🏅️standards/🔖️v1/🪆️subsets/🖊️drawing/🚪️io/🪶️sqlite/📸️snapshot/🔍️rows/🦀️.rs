//! 🔍️ Original scalar row indexes retain Drawing relationship order through cancellation.
use super::{SqliteRow,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,float_columns};
use semio_framework_os_kernel::sqlite_snapshot::transfer;
const TABLES:&[(&str,usize);16]=&[("semio_drawing_document",8),("semio_drawing_style",14),("semio_drawing_layer",7),("semio_drawing_node",2),("semio_drawing_path",2),("semio_drawing_text",5),("semio_drawing_group",11),("semio_drawing_image",7),("semio_drawing_child",4),("semio_drawing_segment",4),("semio_drawing_move",3),("semio_drawing_line",3),("semio_drawing_cubic",7),("semio_drawing_quad",5),("semio_drawing_arc",8),("semio_drawing_close",1)];
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
#[derive(semio_framework_value::RetireOwned)]
pub(super)struct TableStorage{indices:Vec<usize>,used:Vec<u8>,parents:Vec<usize>,parents_ready:bool}
#[derive(semio_framework_value::RetireOwned)]
pub(super)struct Storage{tables:[TableStorage;16]}
impl Storage{pub(super)fn empty()->Self{Self{tables:std::array::from_fn(|_|TableStorage{indices:Vec::new(),used:Vec::new(),parents:Vec::new(),parents_ready:false})}}}
pub(super)struct Rows<'a,'s>{database:&'a SqliteDatabase,storage:&'s mut Storage}
impl<'a,'s> Rows<'a,'s>{
 pub(super)fn new(database:&'a SqliteDatabase,storage:&'s mut Storage,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  for(index,&(name,width))in TABLES.iter().enumerate(){
   let source=&database.table(name)?.rows;let target=&mut storage.tables[index];
   target.indices=transfer::reserve(source.len(),control)?;target.used=transfer::reserve(source.len(),control)?;
   for(position,row)in source.iter().enumerate(){let row=SqliteRow::new(row,float_columns(name))?;if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=width{return Err(invalid("invalid drawing row identity or columns"))}target.indices.push(position);target.used.push(0);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,source.len())?;}
   transfer::heap_sort(&mut target.indices,SqliteSnapshotPhase::ReconstructSnapshot,control,|left,right,_|Ok(source[*left].rowid.cmp(&source[*right].rowid)))?;
   for position in 1..target.indices.len(){if source[target.indices[position-1]].rowid==source[target.indices[position]].rowid{return Err(invalid("duplicate drawing identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,target.indices.len())?;}
  }Ok(Self{database,storage})
 }
 fn table(name:&str)->Result<usize,ValueError>{TABLES.iter().position(|table|table.0==name).ok_or_else(||invalid("unknown drawing entity table"))}
 pub(super)fn len(&self,name:&str)->Result<usize,ValueError>{Ok(self.storage.tables[Self::table(name)?].indices.len())}
 pub(super)fn position(&self,name:&str,id:i64)->Result<usize,ValueError>{let table=&self.storage.tables[Self::table(name)?];let source=&self.database.table(name)?.rows;table.indices.binary_search_by_key(&id,|index|source[*index].rowid).map_err(|_|invalid("dangling drawing relationship"))}
 pub(super)fn get(&self,name:&str,position:usize)->Result<SqliteRow<'a>,ValueError>{let table=&self.storage.tables[Self::table(name)?];let index=*table.indices.get(position).ok_or_else(||invalid("invalid drawing row position"))?;SqliteRow::new(&self.database.table(name)?.rows[index],float_columns(name))}
 pub(super)fn take(&mut self,name:&str,id:i64)->Result<SqliteRow<'a>,ValueError>{let position=self.position(name,id)?;let row=self.get(name,position)?;if std::mem::replace(&mut self.storage.tables[Self::table(name)?].used[position],1)!=0{return Err(invalid("cyclic or multiply owned drawing entity"))}Ok(row)}
 pub(super)fn children_into(&mut self,name:&str,parent:i64,output:&mut Vec<usize>,control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize),ValueError>{
  let table=&mut self.storage.tables[Self::table(name)?];let source=&self.database.table(name)?.rows;let row=|position:usize|SqliteRow::new(&source[table.indices[position]],float_columns(name));
  if !table.parents_ready{
   table.parents=transfer::reserve(table.indices.len(),control)?;
   for position in 0..table.indices.len(){let current=row(position)?;current.integer(1)?;if current.integer(2)?<0{return Err(invalid("drawing ordinal must be nonnegative"))}table.parents.push(position);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,table.indices.len())?;}
   transfer::heap_sort(&mut table.parents,SqliteSnapshotPhase::ReconstructSnapshot,control,|left,right,_|{let left=row(*left)?;let right=row(*right)?;Ok(left.integer(1)?.cmp(&right.integer(1)?).then(left.integer(2)?.cmp(&right.integer(2)?)))})?;
   table.parents_ready=true;
  }
  control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;
  let start=table.parents.partition_point(|position|row(*position).expect("validated drawing row").integer(1).expect("validated parent")<parent);
  let end=table.parents.partition_point(|position|row(*position).expect("validated drawing row").integer(1).expect("validated parent")<=parent);
  if end-start>output.capacity()-output.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"drawing relationship backing was not admitted"))}
  let first=output.len();
  for(ordinal,position)in table.parents[start..end].iter().enumerate(){let current=row(*position)?;if current.integer(2)?!=i64::try_from(ordinal).map_err(|_|invalid("drawing ordinal exceeds INTEGER width"))?{return Err(invalid("drawing relationship order must be contiguous and unique"))}if std::mem::replace(&mut table.used[*position],1)!=0{return Err(invalid("multiply owned drawing relation"))}output.push(*position);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,ordinal,end-start)?;}Ok((first,output.len()))
 }
 pub(super)fn finish(&self,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{for table in &self.storage.tables{for(index,used)in table.used.iter().enumerate(){if *used==0{return Err(invalid("orphan or contradictory drawing detail"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,table.indices.len())?;}}Ok(())}
}
