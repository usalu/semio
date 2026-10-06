//! \u{1f50d}\uFE0F Borrowed Drawing row identities and admitted relationship order.
use super::{SqliteRow,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,float_columns};
use semio_framework_os_kernel::sqlite_snapshot::transfer;
const TABLES:&[(&str,usize)]=&[("semio_drawing_document",8),("semio_drawing_style",14),("semio_drawing_layer",7),("semio_drawing_node",2),("semio_drawing_path",2),("semio_drawing_text",5),("semio_drawing_group",11),("semio_drawing_image",7),("semio_drawing_child",4),("semio_drawing_segment",4),("semio_drawing_move",3),("semio_drawing_line",3),("semio_drawing_cubic",7),("semio_drawing_quad",5),("semio_drawing_arc",8),("semio_drawing_close",1)];
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
struct Parents{positions:Vec<usize>}
struct Table<'a>{name:&'static str,rows:Vec<SqliteRow<'a>>,used:Vec<bool>,parents:Option<Parents>}
pub(super) struct Rows<'a>{tables:Vec<Table<'a>>}
impl<'a> Rows<'a>{
 pub(super) fn new(db:&'a SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let mut tables=transfer::reserve(TABLES.len(),control)?;
  for &(name,width) in TABLES{
   let source=&db.table(name)?.rows;let mut rows=transfer::reserve(source.len(),control)?;let mut used=transfer::reserve(source.len(),control)?;
   for(index,row)in source.iter().enumerate(){let row=SqliteRow::new(row,float_columns(name))?;if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=width{return Err(invalid("invalid drawing row identity or columns"))}rows.push(row);used.push(false);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,source.len())?;}
   transfer::heap_sort(&mut rows,SqliteSnapshotPhase::ReconstructSnapshot,control,|left,right,_|Ok(left.rowid.cmp(&right.rowid)))?;
   for index in 1..rows.len(){if rows[index-1].rowid==rows[index].rowid{return Err(invalid("duplicate drawing identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}
   tables.push(Table{name,rows,used,parents:None});
  }Ok(Self{tables})
 }
 fn table(&self,name:&str)->Result<usize,ValueError>{self.tables.iter().position(|table|table.name==name).ok_or_else(||invalid("unknown drawing entity table"))}
 pub(super) fn len(&self,name:&str)->Result<usize,ValueError>{Ok(self.tables[self.table(name)?].rows.len())}
 pub(super) fn position(&self,name:&str,id:i64)->Result<usize,ValueError>{self.tables[self.table(name)?].rows.binary_search_by_key(&id,|row|row.rowid).map_err(|_|invalid("dangling drawing relationship"))}
 pub(super) fn take(&mut self,name:&str,id:i64)->Result<SqliteRow<'a>,ValueError>{let position=self.position(name,id)?;let index=self.table(name)?;let table=&mut self.tables[index];if std::mem::replace(&mut table.used[position],true){return Err(invalid("cyclic or multiply owned drawing entity"))}Ok(table.rows[position])}
 pub(super) fn ordered(&mut self,name:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{self.children(name,1,control)}
 pub(super) fn children(&mut self,name:&str,parent:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{
  let index=self.table(name)?;let table=&mut self.tables[index];
  if table.parents.is_none(){let mut positions=transfer::reserve(table.rows.len(),control)?;for(index,row)in table.rows.iter().enumerate(){row.integer(1)?;if row.integer(2)?<0{return Err(invalid("drawing ordinal must be nonnegative"))}positions.push(index);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,table.rows.len())?;}
   transfer::heap_sort(&mut positions,SqliteSnapshotPhase::ReconstructSnapshot,control,|left,right,_|{let left=table.rows[*left];let right=table.rows[*right];Ok(left.integer(1)?.cmp(&right.integer(1)?).then(left.integer(2)?.cmp(&right.integer(2)?)))})?;table.parents=Some(Parents{positions});}
  control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;let positions=&table.parents.as_ref().expect("parents admitted").positions;
  let start=positions.partition_point(|position|table.rows[*position].integer(1).expect("validated parent")<parent);let end=positions.partition_point(|position|table.rows[*position].integer(1).expect("validated parent")<=parent);
  let mut output=transfer::reserve(end-start,control)?;for(ordinal,position)in positions[start..end].iter().enumerate(){let row=table.rows[*position];if row.integer(2)?!=i64::try_from(ordinal).map_err(|_|invalid("drawing ordinal exceeds INTEGER width"))?{return Err(invalid("drawing relationship order must be contiguous and unique"))}if std::mem::replace(&mut table.used[*position],true){return Err(invalid("multiply owned drawing relation"))}output.push(row);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,ordinal,end-start)?;}Ok(output)
 }
 pub(super) fn finish(self,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{for table in self.tables{for(index,used)in table.used.into_iter().enumerate(){if !used{return Err(invalid("orphan or contradictory drawing detail"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,table.rows.len())?;}}Ok(())}
}
