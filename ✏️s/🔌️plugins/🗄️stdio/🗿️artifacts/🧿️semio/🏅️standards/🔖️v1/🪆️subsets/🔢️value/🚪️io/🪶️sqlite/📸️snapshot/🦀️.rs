//! 🔢️ Typed Semio value primitives, structural ownership and referential graph nodes.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot, sqlite_snapshot::{artifact::{Cell, RowWriter, reconstruct_text, reconstruct_blob}, validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase}};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;

fn number(value: usize) -> Result<i64,ValueError> { i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string())) }
fn identity(row: &SqliteRow, columns: usize) -> Result<(),ValueError> { if row.rowid <= 0 || row.integer(0)? != row.rowid || row.values.len() != columns { Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio value identity or columns")) } else { Ok(()) } }
fn null(row: &SqliteRow, index: usize) -> Result<(),ValueError> { if row.values.get(index) == Some(&SqliteValue::Null) { Ok(()) } else { Err(ValueError::new(ValueRefusalKind::InvalidValue,"unexpected Semio value variant field")) } }
/// 🧩️ Explicit entity names for genuinely shared, typed SemioValue fields.
#[derive(Clone, Copy)]
pub struct ValueSqliteTables { pub value: &'static str, pub list_element: &'static str, pub map_entry: &'static str }
pub const VALUE_TABLES: ValueSqliteTables = ValueSqliteTables { value: "semio_value_value", list_element: "semio_value_list_element", map_entry: "semio_value_map_entry" };


use store::sqlite_snapshot::SqliteDatabaseLimits;
#[path="🧮️semantic/🦀️.rs"]
pub(crate)mod semantic;
/// 🎟️ Admits every typed cell before native forecasting or allocation.
pub(crate)fn admit_values(snapshot:&SemioValueSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Checks the exact authored layout before native ownership.
pub(crate)fn admit_layout(limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before typed construction.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📃️ Counts actual document primitive cells before typed construction.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}
/// 🔎️ Resolves the actual paid, sorted borrowed identifier frontier.
fn value_reference(nodes:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=nodes.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(nodes[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(nodes[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio value reference"))
}
/// 🌳️ Visits the complete owned graph through the caller's borrowed or owned writer.
pub(crate)fn visit_rows(snapshot:&SemioValueSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.check_rows(snapshot.nodes.len().checked_add(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio value row overflow"))?)?;
 let mut nodes=out.allocate_frontier(snapshot.nodes.len())?;for(ordinal,node)in snapshot.nodes.iter().enumerate(){out.checkpoint()?;nodes.push((node.id.value.as_str(),number(ordinal+1)?));}
 out.sort_frontier(&mut nodes,|a,b,c|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,SqliteSnapshotPhase::ProjectSnapshot,c))?;
 for pair in nodes.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio value node identifier"))}}
 let root=project_value_tree(&snapshot.root,VALUE_TABLES,Some(&nodes),out)?;out.insert_key("semio_value_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(root)])?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){let value=project_value_tree(&node.value,VALUE_TABLES,Some(&nodes),out)?;out.insert_key("semio_value_node",number(ordinal+1)?,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id.value),Cell::Integer(value)])?;}Ok(())
}

enum Owner<'a> { Root, List(i64, usize), Map(i64, usize, &'a str) }

/// 🌱️ Projects exactly the owned SemioValue variants and their structural links.
pub fn project_value_tree(value: &SemioValue, tables: ValueSqliteTables, nodes: Option<&[(&str,i64)]>, projection: &mut RowWriter<'_, '_>) -> Result<i64,ValueError> {
    let mut pending=Vec::new();let mut initial=Some((value,Owner::Root));let mut root=0;
    while let Some((value, owner)) = initial.take().or_else(||pending.pop()) {
        projection.checkpoint()?; let mut cells = [Cell::Null; 7];
        cells[0] = Cell::Text(match value { SemioValue::Null => "null", SemioValue::Bool { .. } => "bool", SemioValue::Int { .. } => "int", SemioValue::Float { .. } => "float", SemioValue::Str { .. } => "str", SemioValue::Bytes { .. } => "bytes", SemioValue::List { .. } => "list", SemioValue::Map { .. } => "map", SemioValue::Ref { .. } => "ref" });
        match value { SemioValue::Bool { value } => cells[1] = Cell::Integer(i64::from(*value)), SemioValue::Int { lexeme } => cells[2] = Cell::Text(lexeme), SemioValue::Float { lexeme } => cells[3] = Cell::Text(lexeme), SemioValue::Str { value } => cells[4] = Cell::Text(value), SemioValue::Bytes { value } => cells[5] = Cell::Blob(value), SemioValue::Ref { id } => cells[6] = match nodes { Some(nodes) => Cell::Integer(value_reference(nodes,id.value.as_str(),projection)?), None => Cell::Text(&id.value) }, _ => {} }
        let id = projection.insert(tables.value, &cells)?;
        match owner { Owner::Root => root = id, Owner::List(parent, ordinal) => { projection.insert(tables.list_element, &[Cell::Integer(parent), Cell::Integer(number(ordinal)?), Cell::Integer(id)])?; }, Owner::Map(parent, ordinal, key) => { projection.insert(tables.map_entry, &[Cell::Integer(parent), Cell::Integer(number(ordinal)?), Cell::Text(key), Cell::Integer(id)])?; } }
        match value { SemioValue::List { items } => { projection.check_rows(pending.len().checked_add(items.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio pending value count overflow"))?)?; for (ordinal, item) in items.iter().enumerate().rev() { projection.push_frontier(&mut pending,(item,Owner::List(id,ordinal)))?; } }, SemioValue::Map { entries } => { projection.check_rows(pending.len().checked_add(entries.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio pending value count overflow"))?)?; for (ordinal, entry) in entries.iter().enumerate().rev() { projection.push_frontier(&mut pending,(&entry.value,Owner::Map(id,ordinal,&entry.key)))?; } }, _ => {} }
    }
    Ok(root)
}

/// 🌳️ Reconstructs raw value9 ownership with concrete admitted reference and value slots.
pub fn reconstruct_value_forest(database:&SqliteDatabase,tables:ValueSqliteTables,roots:&[i64],names:Option<&[(i64,&str)]>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SemioValue>,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::transfer::reserve;
 let source=&database.table(tables.value)?.rows;let mut rows=reserve(source.len(),control)?;for(index,row)in source.iter().enumerate(){identity(row,8)?;rows.push(row);if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,source.len())?;}}
 semio_framework_os_kernel::sqlite_snapshot::transfer::heap_sort(&mut rows,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.rowid.cmp(&b.rowid)))?;if rows.windows(2).any(|pair|pair[0].rowid==pair[1].rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio value identity"))}
 let locate=|id:i64|rows.binary_search_by_key(&id,|row|row.rowid).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio value ownership edge"));
 let list=&database.table(tables.list_element)?.rows;let map=&database.table(tables.map_entry)?.rows;
 let links_count=list.len().checked_add(map.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio value relationship count overflow"))?;
 let mut links=reserve(links_count,control)?;
 for(map_member,source)in [(false,list),(true,map)]{
  let mut ids=reserve(source.len(),control)?;
  for(index,row)in source.iter().enumerate(){identity(row,if map_member{5}else{4})?;ids.push(row.rowid);let parent=row.integer(1)?;let kind=rows[locate(parent)?].text(1)?;if kind!=if map_member{"map"}else{"list"}{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio value relationship has wrong parent kind"))}
   let ordinal=row.integer(2)?;let child=row.integer(if map_member{4}else{3})?;locate(child)?;links.push((parent,ordinal,child,if map_member{Some(row.text(3)?)}else{None}));
   if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,source.len())?;}
  }
  semio_framework_os_kernel::sqlite_snapshot::transfer::heap_sort(&mut ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.cmp(b)))?;if ids.windows(2).any(|pair|pair[0]==pair[1]){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio value relationship identifier"))}
 }
 semio_framework_os_kernel::sqlite_snapshot::transfer::heap_sort(&mut links,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok((a.0,a.1).cmp(&(b.0,b.1))))?;let mut parent=None;let mut ordinal=0;
 for(index,link)in links.iter().enumerate(){if parent!=Some(link.0){parent=Some(link.0);ordinal=0;}if link.1!=number(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio value member ordinals must be contiguous"))}ordinal+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index+1,links.len())?;}
 let mut seen=reserve(rows.len(),control)?;seen.resize(rows.len(),false);let mut complete=Owned::new(reserve(rows.len(),control)?);for _ in 0..rows.len(){complete.get_mut().push(None);}
 let mut restored=Owned::new(reserve(roots.len(),control)?);let mut pending=reserve(1,control)?;let mut count=0usize;
 for &root in roots{
  value_push(&mut pending,(root,false),control)?;
  while let Some((id,exit))=pending.pop(){
   count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio value reconstruction work overflow"))?;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}
   let index=locate(id)?;let row=rows[index];let start=links.partition_point(|link|link.0<id);let end=links.partition_point(|link|link.0<=id);let children=&links[start..end];
   if !exit{if seen[index]{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"cyclic or multiply owned Semio value"))}seen[index]=true;value_push(&mut pending,(id,true),control)?;for child in children.iter().rev(){value_push(&mut pending,(child.2,false),control)?;}continue;}
   let active=match row.text(1)?{"null"|"list"|"map"=>None,"bool"=>Some(2),"int"=>Some(3),"float"=>Some(4),"str"=>Some(5),"bytes"=>Some(6),"ref"=>Some(7),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio value kind"))};
   for column in 2..8{if Some(column)!=active{null(row,column)?;}}
   let value=match row.text(1)?{
    "null"=>SemioValue::Null,"bool"=>SemioValue::Bool{value:match row.integer(2)?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio boolean"))}},
    "int"=>SemioValue::Int{lexeme:reconstruct_text(control,row.text(3)?)?},"float"=>SemioValue::Float{lexeme:reconstruct_text(control,row.text(4)?)?},"str"=>SemioValue::Str{value:reconstruct_text(control,row.text(5)?)?},"bytes"=>SemioValue::Bytes{value:reconstruct_blob(control,row.blob(6)?)?},
    "ref"=>SemioValue::Ref{id:ValueId{value:match names{Some(names)=>{let id=row.integer(7)?;let index=names.binary_search_by_key(&id,|entry|entry.0).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio value graph reference"))?;reconstruct_text(control,names[index].1)?},None=>reconstruct_text(control,row.text(7)?)?}}},
    "list"=>{let mut items=Owned::new(reserve(children.len(),control)?);for child in children{let value=complete.get_mut()[locate(child.2)?].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"missing completed Semio list child"))?;items.get_mut().push(value);}SemioValue::List{items:items.take()}},
    "map"=>{let mut entries=Owned::new(reserve(children.len(),control)?);for child in children{let mut entry=Owned::new(SemioValueEntry{key:String::new(),value:SemioValue::Null});entry.get_mut().key=reconstruct_text(control,child.3.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"missing Semio map key"))?)?;entry.get_mut().value=complete.get_mut()[locate(child.2)?].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"missing completed Semio map child"))?;entries.get_mut().push(entry.take());}SemioValue::Map{entries:entries.take()}},
    _=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"validated Semio variant disappeared"))
   };complete.get_mut()[index]=Some(value);
  }
  let value=complete.get_mut()[locate(root)?].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"missing completed Semio root"))?;restored.get_mut().push(value);
 }
 if seen.iter().any(|seen|!*seen)||complete.get_mut().iter().any(Option::is_some){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unowned Semio value"))}
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,count)?;Ok(restored.take())
}
fn value_push<T>(frontier:&mut Vec<T>,value:T,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if frontier.len()==frontier.capacity(){let count=frontier.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio value frontier backing overflow"))?;let mut replacement=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(count,control)?;for(index,item)in frontier.drain(..).enumerate(){replacement.push(item);if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,0)?;}}*frontier=replacement;}frontier.push(value);Ok(())
}

impl ArtifactSqliteSnapshot for SemioValueSnapshot {
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::value::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}

 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::v1::subsets::value::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control.native())}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="value"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_value_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {self.project_sqlite_database(control)}
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioValueSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

/// 🌱️ Bounds the actual typed SemioValue variants without resolving graph references.
pub fn native_values(roots: &[SemioValue], b: &mut Bound<'_, '_>) -> Result<(),ValueError> {
    let mut pending=b.allocate_frontier(roots.len())?;pending.extend(roots.iter().rev());
    while let Some(value) = pending.pop() {
        b.entities(1)?;
        match value {
            SemioValue::Null => {}, SemioValue::Bool { .. } => b.scalars(1)?,
            SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => b.text(lexeme)?,
            SemioValue::Str { value } => b.text(value)?, SemioValue::Bytes { value } => b.bytes(value)?,
            SemioValue::Ref { id } => b.text(&id.value)?,
            SemioValue::List { items } => { b.entities(items.len())?; for item in items.iter().rev(){b.push_frontier(&mut pending,item)?;} },
            SemioValue::Map { entries } => { b.entities(entries.len())?; for entry in entries.iter().rev() { b.text(&entry.key)?; b.push_frontier(&mut pending,&entry.value)?; } },
        }
    }
    Ok(())
}
impl SemioValueSnapshot {
    /// 📏️ Bounds the own root, graph identities and native values.
    pub fn native_fields(&self,b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.text(&self.schema)?;native_values(std::slice::from_ref(&self.root),b)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id.value)?;native_values(std::slice::from_ref(&node.value),b)?;}Ok(())}
}

impl SemioValueSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()} 
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "💰️reconstruction/🦀️.rs"]
mod reconstruction;

#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
