//! 🗂️ paid row identity and relationship frontiers for the full owner schema.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,transfer,validate_sqlite_database_schema_controlled};
pub(super) const TABLES:[(&str,usize);30]=[
("rewriting_document",2),("rewriting_layout",9),("rewriting_value",2),("rewriting_binding",4),("rewriting_boolean",3),("rewriting_number",5),("rewriting_string",3),("rewriting_array_element",4),("rewriting_object_member",5),("rewriting_lhs",4),("rewriting_rhs",2),("rewriting_pattern",7),("rewriting_create",4),("rewriting_merge",4),("rewriting_delete",4),("rewriting_assignment",6),("rewriting_parameter",6),("jack_document",6),("jack_camera",11),("jack_content_child",7),("jack_node_kind",4),("jack_edge_kind",4),("jack_port_kind",5),("jack_node_kind_port",4),("jack_value_type",2),("jack_value_type_list",3),("jack_value_type_schema",3),("jack_node_property",7),("jack_edge_property",7),("jack_port_property",7)];
pub(super) fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:&str)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
struct Entry<'a>{table:usize,row:&'a SqliteRow,used:bool}
pub(super) struct Rows<'a>{entries:Vec<Entry<'a>>,counts:[usize;30],work:usize}
impl<'a> Rows<'a>{
 pub(super) fn new(database:&'a SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  validate_sqlite_database_schema_controlled(database,include_str!("../../🗄️.sql"),SqliteSnapshotPhase::ReconstructSnapshot,c)?;
  c.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let count=database.tables.iter().try_fold(0usize,|n,table|n.checked_add(table.rows.len()).ok_or_else(||work("Rewriting index count overflow")))?;
  let mut entries=transfer::reserve(count,c)?;let mut completed=0;let mut counts=[0;30];
  for(table,(name,width))in TABLES.iter().enumerate(){for row in &database.table(name)?.rows{
   if row.rowid<=0||row.values.len()!=*width||row.integer(0)?!=row.rowid{return Err(invalid("Rewriting row width or identity"))}
   entries.push(Entry{table,row,used:false});counts[table]+=1;completed+=1;if completed%256==0{c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,count)?;}
  }}
  transfer::heap_sort(&mut entries,SqliteSnapshotPhase::ReconstructSnapshot,c,|a,b,_|Ok((a.table,a.row.rowid).cmp(&(b.table,b.row.rowid))))?;
  if entries.windows(2).any(|pair|pair[0].table==pair[1].table&&pair[0].row.rowid==pair[1].row.rowid){return Err(invalid("Rewriting duplicate table identity"))}
  c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,count)?;Ok(Self{entries,counts,work:0})
 }
 pub(super) fn count(&self,table:usize)->usize{self.counts[table]}
 pub(super) fn step(&mut self,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.work=self.work.checked_add(1).ok_or_else(||work("Rewriting reconstruction work overflow"))?;if self.work%256==0{c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.work,0)?;}Ok(())}
 pub(super) fn take(&mut self,table:usize,id:i64,c:&mut SqliteSnapshotControl<'_>)->Result<&'a SqliteRow,ValueError>{
  self.step(c)?;let i=self.entries.binary_search_by_key(&(table,id),|entry|(entry.table,entry.row.rowid)).map_err(|_|invalid("Rewriting dangling relationship"))?;let entry=&mut self.entries[i];if entry.used{return Err(invalid("Rewriting cyclic or multiply owned entity"))}entry.used=true;Ok(entry.row)
 }
 pub(super) fn root(&mut self,table:usize,c:&mut SqliteSnapshotControl<'_>)->Result<&'a SqliteRow,ValueError>{
  if self.count(table)!=1{return Err(invalid("Rewriting singleton entity cardinality"))}
  let position=self.entries.binary_search_by_key(&table,|entry|entry.table).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"Rewriting singleton absent"))?;let id=self.entries[position].row.rowid;self.take(table,id,c)
 }
 pub(super) fn children(&mut self,table:usize,parent:i64,ordered:bool,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
  let start=self.entries.partition_point(|entry|entry.table<table);let end=self.entries.partition_point(|entry|entry.table<=table);let mut count=0;
  for index in start..end{self.step(c)?;if self.entries[index].row.integer(1)?==parent{count+=1;}}
  let mut rows=transfer::reserve(count,c)?;for index in start..end{self.step(c)?;let entry=&self.entries[index];if entry.row.integer(1)?==parent{rows.push(entry.row)}}
  transfer::heap_sort(&mut rows,SqliteSnapshotPhase::ReconstructSnapshot,c,|a,b,_|Ok(if ordered{a.integer(2)?.cmp(&b.integer(2)?)}else{a.rowid.cmp(&b.rowid)}))?;
  for(n,row)in rows.iter().enumerate(){if ordered&&row.integer(2)?!=i64::try_from(n).map_err(|_|work("Rewriting ordinal range overflow"))?{return Err(invalid("Rewriting noncontiguous ordered relationship"))}self.take(table,row.rowid,c)?;}
  Ok(rows)
 }
 pub(super) fn body(&mut self,table:usize,parent:i64,c:&mut SqliteSnapshotControl<'_>)->Result<&'a SqliteRow,ValueError>{let rows=self.children(table,parent,false,c)?;if rows.len()!=1{return Err(invalid("Rewriting variant entity cardinality"))}Ok(rows[0])}
 pub(super) fn finish(&self,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{if self.entries.iter().any(|entry|!entry.used){return Err(invalid("Rewriting unreachable or alternate variant entity"))}c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.work,self.work)}
}
