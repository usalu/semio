//! 🧱️ Independent PLY declarations, row cells and recursive typed value entities.
use crate::standards::v1_0::subsets::any::schema::snapshot::*;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use store::ArtifactSqliteSnapshot;
use store::sqlite_snapshot::validate_sqlite_database_schema;
use store::sqlite_snapshot::SqliteDatabase;
use store::sqlite_snapshot::SqliteRow;
use store::sqlite_snapshot::SqliteValue;
use store::sqlite_snapshot::SqliteSnapshotControl;
use store::sqlite_snapshot::SqliteSnapshotPhase;
use store::sqlite_snapshot::SnapshotEncoding;
use store::sqlite_snapshot::artifact::Cell;
use store::sqlite_snapshot::artifact::RowWriter;
use store::sqlite_snapshot::artifact::FloatColumn;
use store::sqlite_snapshot::artifact::FloatRow;
use store::sqlite_snapshot::artifact::NativeEncodingBound;
use std::collections::{BTreeMap,BTreeSet};
const BINARY32:&[FloatColumn]=&[FloatColumn::Binary32(3)];
const BINARY64:&[FloatColumn]=&[FloatColumn::Binary64(3)];
fn kind_name(kind: PlyScalarType) -> &'static str { match kind { PlyScalarType::Char => "char", PlyScalarType::UChar => "uchar", PlyScalarType::Short => "short", PlyScalarType::UShort => "ushort", PlyScalarType::Int => "int", PlyScalarType::UInt => "uint", PlyScalarType::Float => "float", PlyScalarType::Double => "double" } }
fn scalar_kind(name: &str) -> Result<PlyScalarType,ValueError> { match name { "char" => Ok(PlyScalarType::Char), "uchar" => Ok(PlyScalarType::UChar), "short" => Ok(PlyScalarType::Short), "ushort" => Ok(PlyScalarType::UShort), "int" => Ok(PlyScalarType::Int), "uint" => Ok(PlyScalarType::UInt), "float" => Ok(PlyScalarType::Float), "double" => Ok(PlyScalarType::Double), _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"unknown PLY scalar kind")) } }
fn scalar(value: &PlyValue) -> Result<(&'static str, Cell<'static>, Cell<'static>),ValueError> {
    let null = Cell::Null;
    match value {
        PlyValue::Char(value) => Ok(("char", Cell::Integer(i64::from(*value)), null)),
        PlyValue::UChar(value) => Ok(("uchar", Cell::Integer(i64::from(*value)), null)),
        PlyValue::Short(value) => Ok(("short", Cell::Integer(i64::from(*value)), null)),
        PlyValue::UShort(value) => Ok(("ushort", Cell::Integer(i64::from(*value)), null)),
        PlyValue::Int(value) => Ok(("int", Cell::Integer(i64::from(*value)), null)),
        PlyValue::UInt(value) => Ok(("uint", Cell::Integer(i64::from(*value)), null)),
        PlyValue::Float(value) => Ok(("float", null, Cell::Float32(*value))),
        PlyValue::Double(value) => Ok(("double", null, Cell::Real(*value))),
        PlyValue::List(_) => Ok(("list", Cell::Null, Cell::Null)),
    }
}

fn read_scalar(row: &SqliteRow) -> Result<PlyValue,ValueError> {
    let kind = scalar_kind(row.text(1)?)?;
    let row=FloatRow::new(row,if kind==PlyScalarType::Float{BINARY32}else{BINARY64})?;
    if matches!(kind, PlyScalarType::Float | PlyScalarType::Double) {
        if row.values.get(2) != Some(&SqliteValue::Null) { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY real scalar must not have an integer payload")); }
        if kind == PlyScalarType::Double { return Ok(PlyValue::Double(row.real(3)?)); }
        return Ok(PlyValue::Float(row.binary32(3)?));
    }
    if !row.is_null(3)? { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY integer scalar must not have a real payload")); }
    let value = row.integer(2)?;
    match kind {
        PlyScalarType::Char => i8::try_from(value).map(PlyValue::Char).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())),
        PlyScalarType::UChar => u8::try_from(value).map(PlyValue::UChar).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())),
        PlyScalarType::Short => i16::try_from(value).map(PlyValue::Short).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())),
        PlyScalarType::UShort => u16::try_from(value).map(PlyValue::UShort).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())),
        PlyScalarType::Int => i32::try_from(value).map(PlyValue::Int).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())),
        PlyScalarType::UInt => u32::try_from(value).map(PlyValue::UInt).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())),
        _ => unreachable!(),
    }
}

fn visit(snapshot:&PlySnapshot,writer:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let format=match snapshot.format{PlyFormat::Ascii=>"ascii",PlyFormat::BinaryLittleEndian=>"binary_little_endian",PlyFormat::BinaryBigEndian=>"binary_big_endian"};
 writer.insert_float("ply_document",&[Cell::Text(&snapshot.schema),Cell::Text(format)],&[])?;
 for(index,comment)in snapshot.comments.iter().enumerate(){writer.insert_float("ply_comment",&[Cell::Integer(1),Cell::Integer(i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?),Cell::Text(comment)],&[])?;}
 let mut pending=writer.allocate_frontier(0)?;
 for(index,element)in snapshot.elements.iter().enumerate(){
  let id=writer.insert_float("ply_element",&[Cell::Integer(1),Cell::Integer(i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?),Cell::Text(&element.name),Cell::Integer((element.count>>32)as i64),Cell::Integer((element.count&0xffffffff)as i64)],&[])?;
  for(index,property)in element.properties.iter().enumerate(){let prefix=[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?),Cell::Text(property.name())];let suffix=match property{PlyProperty::Scalar{kind,..}=>[Cell::Text("scalar"),Cell::Text(kind_name(*kind)),Cell::Null,Cell::Null],PlyProperty::List{count_kind,value_kind,..}=>[Cell::Text("list"),Cell::Null,Cell::Text(kind_name(*count_kind)),Cell::Text(kind_name(*value_kind))]};writer.insert_float("ply_property",&[prefix[0],prefix[1],prefix[2],suffix[0],suffix[1],suffix[2],suffix[3]],&[])?;}
  for(index,row)in element.rows.iter().enumerate(){let id=writer.insert_float("ply_row",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?)],&[])?;for(index,value)in row.values.iter().enumerate(){let key=i64::try_from(pending.len()).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"PLY value index overflow"))?;writer.insert_float("ply_cell",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?),Cell::Integer(key)],&[])?;writer.push_frontier(&mut pending,value)?;}}
 }
 let mut index=0;while index<pending.len(){let value=pending[index];let id=i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?+1;let(kind,integer,real)=scalar(value)?;writer.insert_float("ply_value",&[Cell::Text(kind),integer,real],if kind=="float"{BINARY32}else{BINARY64})?;
  if let PlyValue::List(items)=value{for(index,value)in items.iter().enumerate(){let child=i64::try_from(pending.len()).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"PLY value index overflow"))?;writer.insert_float("ply_list_item",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?),Cell::Integer(child)],&[])?;writer.push_frontier(&mut pending,value)?;}}index+=1;
 }Ok(())
}
fn ids(database:&SqliteDatabase,name:&str)->Result<BTreeSet<i64>,ValueError>{let mut result=BTreeSet::new();for row in&database.table(name)?.rows{if row.integer(0)?!=row.rowid||!result.insert(row.rowid){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY row identity differs"))}}Ok(result)}
fn groups<'a>(database:&'a SqliteDatabase,name:&str,owners:&BTreeSet<i64>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<&'a SqliteRow>>,ValueError>{
 let table=database.table(name)?;ids(database,name)?;let mut result=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(index,row)in table.rows.iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,table.rows.len())?;}let owner=row.integer(1)?;if !owners.contains(&owner){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY relationship has an unknown owner"))}result.entry(owner).or_default().push(row);}
 for rows in result.values_mut(){rows.sort_by_key(|row|row.integer(2).unwrap_or(i64::MIN));for(index,row)in rows.iter().enumerate(){if row.integer(2)?!=i64::try_from(index).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,error.to_string()))?{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY ordinals must be contiguous"))}}}Ok(result)
}
struct Items(Vec<PlyValue>);
impl Drop for Items{fn drop(&mut self){for value in self.0.drain(..){crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::retire_value(value)}}}
struct Forest(BTreeMap<i64,PlyValue>);
impl Drop for Forest{fn drop(&mut self){for(_,value)in std::mem::take(&mut self.0){crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::retire_value(value)}}}
impl PlySnapshot{
 fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{let mut writer=RowWriter::borrowed(control,phase)?;visit(self,&mut writer)?;writer.finish_borrowed()}
 pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut writer=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit(self,&mut writer)?;writer.finish()}
 pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  for(name,width)in[("ply_document",3),("ply_comment",4),("ply_element",6),("ply_property",8),("ply_row",3),("ply_cell",4),("ply_value",6),("ply_list_item",4)]{ids(database,name)?;for row in&database.table(name)?.rows{if row.values.len()!=width{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY row width differs"))}}}
  let document=database.table("ply_document")?.single_row()?;let document_ids=BTreeSet::from([document.rowid]);let format=match document.text(2)?{"ascii"=>PlyFormat::Ascii,"binary_little_endian"=>PlyFormat::BinaryLittleEndian,"binary_big_endian"=>PlyFormat::BinaryBigEndian,_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY format differs"))};
  let mut comments=groups(database,"ply_comment",&document_ids,control)?;let mut elements=groups(database,"ply_element",&document_ids,control)?;let element_ids=ids(database,"ply_element")?;let mut properties=groups(database,"ply_property",&element_ids,control)?;let mut rows=groups(database,"ply_row",&element_ids,control)?;let mut cells=groups(database,"ply_cell",&ids(database,"ply_row")?,control)?;let items=groups(database,"ply_list_item",&ids(database,"ply_value")?,control)?;
  let values:BTreeMap<_,_>=database.table("ply_value")?.rows.iter().map(|row|(row.rowid,row)).collect();let mut owners=BTreeSet::new();for table in["ply_cell","ply_list_item"]{for row in&database.table(table)?.rows{let id=row.integer(3)?;if !values.contains_key(&id)||!owners.insert(id){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY value must have one known owner"))}}}if owners.len()!=values.len(){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY unowned value entity"))}
  let total=values.len();let mut completed=0;let mut forest=Forest(BTreeMap::new());let mut active=BTreeSet::new();let mut visited=BTreeSet::new();
  for cell in&database.table("ply_cell")?.rows{let mut pending=vec![(cell.integer(3)?,false)];while let Some((id,exit))=pending.pop(){if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed+pending.len()+1)?;}completed+=1;let row=values.get(&id).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY value owner differs"))?;let children=items.get(&id).map(Vec::as_slice).unwrap_or(&[]);
    if exit{active.remove(&id);let mut owned=Items(Vec::with_capacity(children.len()));for child in children{owned.0.push(forest.0.remove(&child.integer(3)?).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY value topology differs"))?);}forest.0.insert(id,PlyValue::List(std::mem::take(&mut owned.0)));}
    else{if active.contains(&id)||!visited.insert(id){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY value topology cycles or repeats"))}if row.text(1)?=="list"{if row.values[2]!=SqliteValue::Null||!FloatRow::new(row,BINARY64)?.is_null(3)?{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY list has scalar payload"))}active.insert(id);if pending.len().checked_add(children.len()).and_then(|n|n.checked_add(1)).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"PLY frontier overflow"))?>total+1{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"PLY value frontier limit"))}pending.push((id,true));for child in children.iter().rev(){pending.push((child.integer(3)?,false));}}
    else{if !children.is_empty(){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY scalar owns list children"))}forest.0.insert(id,read_scalar(row)?);}}
   }}
  if visited.len()!=values.len(){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY unreachable value entities"))}
  let mut snapshot=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self{schema:document.text(1)?.into(),format,comments:Vec::new(),elements:Vec::new()},crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::retire);for row in comments.remove(&document.rowid).unwrap_or_default(){snapshot.as_mut().comments.push(row.text(3)?.into());}
  for element in elements.remove(&document.rowid).unwrap_or_default(){let id=element.rowid;let high=u32::try_from(element.integer(4)?).map_err(|_|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY declared count word exceeds unsigned32"))?;let low=u32::try_from(element.integer(5)?).map_err(|_|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY declared count word exceeds unsigned32"))?;let mut decoded=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(PlyElement{name:element.text(3)?.into(),count:(u64::from(high)<<32)|u64::from(low),properties:Vec::new(),rows:Vec::new()},|element:PlyElement|{for row in element.rows{for value in row.values{crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::retire_value(value)}}});
   for property in properties.remove(&id).unwrap_or_default(){let name=property.text(3)?.into();let value=match property.text(4)?{"scalar" if property.optional_text(6)?.is_none()&&property.optional_text(7)?.is_none()=>PlyProperty::Scalar{name,kind:scalar_kind(property.text(5)?)?},"list" if property.optional_text(5)?.is_none()=>PlyProperty::List{name,count_kind:scalar_kind(property.text(6)?)?,value_kind:scalar_kind(property.text(7)?)?},_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY property variant differs"))};decoded.as_mut().properties.push(value);}
   for row in rows.remove(&id).unwrap_or_default(){let mut decoded_row=Items(Vec::new());for cell in cells.remove(&row.rowid).unwrap_or_default(){decoded_row.0.push(forest.0.remove(&cell.integer(3)?).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY row value differs"))?);}decoded.as_mut().rows.push(PlyRow{values:std::mem::take(&mut decoded_row.0)});}snapshot.as_mut().elements.push(decoded.take());
  }
  if !forest.0.is_empty(){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY unconsumed values"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)?;Ok(snapshot.take())
 }
}
impl ArtifactSqliteSnapshot for PlySnapshot{
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{let maximum_rows=control.limits().max_rows;control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;control.check_rows(1)?;self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,"stdio.ply",crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::spec_producer(),|native|crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::record_controlled(self,native,maximum_rows),control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let maximum_rows=control.limits().max_rows;let snapshot=store::decode_sqlite_snapshot_record_native(payload,"stdio.ply",crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::spec_producer(),|record,native|crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::reconstruct_record_controlled(record,native,maximum_rows),control)?;let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(snapshot,crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::retire);owner.as_mut().admit_sqlite_values(control,SqliteSnapshotPhase::DecodeNative)?;Ok(owner.take())}
 fn retire_sqlite_snapshot(self){crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::retire(self)}
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(||->Result<(),ValueError>{let mut bound=NativeEncodingBound::new(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;for comment in&self.comments{bound.add(64)?;bound.repeated(comment.len(),6)?;}for element in&self.elements{bound.add(256)?;bound.repeated(element.name.len(),6)?;for property in&element.properties{bound.add(256)?;bound.repeated(property.name().len(),6)?;}for row in&element.rows{bound.add(64)?;let mut pending=vec![row.values.iter()];while let Some(values)=pending.last_mut(){if let Some(value)=values.next(){bound.add(1164)?;if let PlyValue::List(values)=value{pending.push(values.iter());}}else{pending.pop();}}}}bound.finish()})()}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Self::reconstruct_sqlite_database(database,control)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{(||->Result<store::io_schema::IoOutcome<()>,ValueError>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.ply"||dialect.standard!="1.0"||dialect.subset!="*"{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"geometry owned SQLite dialect differs"))}let candidate=Self::reconstruct_sqlite_database(database,control)?;let mut candidate=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(candidate,crate::standards::v1_0::subsets::any::io::binary::snapshot::native_pack::retire);let actual=candidate.as_mut().project_sqlite_database(control)?;if actual!=self.project_sqlite_database(control)?{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PLY document identity differs"))}Ok(store::io_schema::IoOutcome::clean(()))})().map_err(|error|store::io_schema::IoError::from_value_error(error))}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

