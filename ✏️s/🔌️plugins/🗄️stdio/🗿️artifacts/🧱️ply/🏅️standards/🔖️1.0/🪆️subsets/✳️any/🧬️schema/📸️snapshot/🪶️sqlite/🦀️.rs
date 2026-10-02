//! 🧱️ Independent PLY declarations, row cells and recursive typed value entities.
use super::*;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{Cell,Projection,FloatColumn,FloatRow,insert_ieee754,NativeEncodingBound}}};
use std::collections::{BTreeMap,BTreeSet};
const BINARY32:&[FloatColumn]=&[FloatColumn::Binary32(3)];
const BINARY64:&[FloatColumn]=&[FloatColumn::Binary64(3)];
fn kind_name(kind: PlyScalarType) -> &'static str { match kind { PlyScalarType::Char => "char", PlyScalarType::UChar => "uchar", PlyScalarType::Short => "short", PlyScalarType::UShort => "ushort", PlyScalarType::Int => "int", PlyScalarType::UInt => "uint", PlyScalarType::Float => "float", PlyScalarType::Double => "double" } }
fn scalar_kind(name: &str) -> Result<PlyScalarType, String> { match name { "char" => Ok(PlyScalarType::Char), "uchar" => Ok(PlyScalarType::UChar), "short" => Ok(PlyScalarType::Short), "ushort" => Ok(PlyScalarType::UShort), "int" => Ok(PlyScalarType::Int), "uint" => Ok(PlyScalarType::UInt), "float" => Ok(PlyScalarType::Float), "double" => Ok(PlyScalarType::Double), _ => Err("unknown PLY scalar kind".into()) } }
fn scalar(value: &PlyValue) -> Result<(&'static str, Cell<'static>, Cell<'static>), String> {
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

fn read_scalar(row: &SqliteRow) -> Result<PlyValue, String> {
    let kind = scalar_kind(row.text(1)?)?;
    let row=FloatRow::new(row,if kind==PlyScalarType::Float{BINARY32}else{BINARY64})?;
    if matches!(kind, PlyScalarType::Float | PlyScalarType::Double) {
        if row.values.get(2) != Some(&SqliteValue::Null) { return Err("PLY real scalar must not have an integer payload".into()); }
        if kind == PlyScalarType::Double { return Ok(PlyValue::Double(row.real(3)?)); }
        return Ok(PlyValue::Float(row.binary32(3)?));
    }
    if !row.is_null(3)? { return Err("PLY integer scalar must not have a real payload".into()); }
    let value = row.integer(2)?;
    match kind {
        PlyScalarType::Char => i8::try_from(value).map(PlyValue::Char).map_err(|error| error.to_string()),
        PlyScalarType::UChar => u8::try_from(value).map(PlyValue::UChar).map_err(|error| error.to_string()),
        PlyScalarType::Short => i16::try_from(value).map(PlyValue::Short).map_err(|error| error.to_string()),
        PlyScalarType::UShort => u16::try_from(value).map(PlyValue::UShort).map_err(|error| error.to_string()),
        PlyScalarType::Int => i32::try_from(value).map(PlyValue::Int).map_err(|error| error.to_string()),
        PlyScalarType::UInt => u32::try_from(value).map(PlyValue::UInt).map_err(|error| error.to_string()),
        _ => unreachable!(),
    }
}

fn visit(snapshot:&PlySnapshot,mut emit:impl FnMut(&str,&[Cell<'_>],&[FloatColumn])->Result<i64,String>)->Result<(),String>{
 let format=match snapshot.format{PlyFormat::Ascii=>"ascii",PlyFormat::BinaryLittleEndian=>"binary_little_endian",PlyFormat::BinaryBigEndian=>"binary_big_endian"};
 emit("ply_document",&[Cell::Text(&snapshot.schema),Cell::Text(format)],&[])?;
 for(index,comment)in snapshot.comments.iter().enumerate(){emit("ply_comment",&[Cell::Integer(1),Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?),Cell::Text(comment)],&[])?;}
 let mut pending=Vec::new();
 for(index,element)in snapshot.elements.iter().enumerate(){
  let id=emit("ply_element",&[Cell::Integer(1),Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?),Cell::Text(&element.name),Cell::Integer((element.count>>32)as i64),Cell::Integer((element.count&0xffffffff)as i64)],&[])?;
  for(index,property)in element.properties.iter().enumerate(){let prefix=[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?),Cell::Text(property.name())];let suffix=match property{PlyProperty::Scalar{kind,..}=>[Cell::Text("scalar"),Cell::Text(kind_name(*kind)),Cell::Null,Cell::Null],PlyProperty::List{count_kind,value_kind,..}=>[Cell::Text("list"),Cell::Null,Cell::Text(kind_name(*count_kind)),Cell::Text(kind_name(*value_kind))]};emit("ply_property",&[prefix[0],prefix[1],prefix[2],suffix[0],suffix[1],suffix[2],suffix[3]],&[])?;}
  for(index,row)in element.rows.iter().enumerate(){let id=emit("ply_row",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?)],&[])?;for(index,value)in row.values.iter().enumerate(){let key=i64::try_from(pending.len()).map_err(|e|e.to_string())?.checked_add(1).ok_or("PLY value index overflow")?;emit("ply_cell",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?),Cell::Integer(key)],&[])?;pending.push(value);}}
 }
 let mut index=0;while index<pending.len(){let value=pending[index];let id=i64::try_from(index).map_err(|e|e.to_string())?+1;let(kind,integer,real)=scalar(value)?;emit("ply_value",&[Cell::Text(kind),integer,real],if kind=="float"{BINARY32}else{BINARY64})?;
  if let PlyValue::List(items)=value{for(index,value)in items.iter().enumerate(){let child=i64::try_from(pending.len()).map_err(|e|e.to_string())?.checked_add(1).ok_or("PLY value index overflow")?;emit("ply_list_item",&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|e|e.to_string())?),Cell::Integer(child)],&[])?;pending.push(value);}}index+=1;
 }Ok(())
}
fn ids(database:&SqliteDatabase,name:&str)->Result<BTreeSet<i64>,String>{let mut result=BTreeSet::new();for row in&database.table(name)?.rows{if row.integer(0)?!=row.rowid||!result.insert(row.rowid){return Err("PLY row identity differs".into())}}Ok(result)}
fn groups<'a>(database:&'a SqliteDatabase,name:&str,owners:&BTreeSet<i64>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<&'a SqliteRow>>,String>{
 let table=database.table(name)?;ids(database,name)?;let mut result=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(index,row)in table.rows.iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,table.rows.len())?;}let owner=row.integer(1)?;if !owners.contains(&owner){return Err("PLY relationship has an unknown owner".into())}result.entry(owner).or_default().push(row);}
 for rows in result.values_mut(){rows.sort_by_key(|row|row.integer(2).unwrap_or(i64::MIN));for(index,row)in rows.iter().enumerate(){if row.integer(2)?!=i64::try_from(index).map_err(|e|e.to_string())?{return Err("PLY ordinals must be contiguous".into())}}}Ok(result)
}
struct Items(Vec<PlyValue>);
impl Drop for Items{fn drop(&mut self){for value in self.0.drain(..){super::native_pack::retire_value(value)}}}
struct Forest(BTreeMap<i64,PlyValue>);
impl Drop for Forest{fn drop(&mut self){for(_,value)in std::mem::take(&mut self.0){super::native_pack::retire_value(value)}}}
impl ArtifactSqliteSnapshot for PlySnapshot{
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{let maximum_rows=control.limits().max_rows;control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;control.check_rows(1)?;store::encode_sqlite_snapshot_record_native(encoding,"stdio.ply",super::native_pack::spec_producer(),|native|super::native_pack::record_controlled(self,native,maximum_rows),control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{let maximum_rows=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,"stdio.ply",super::native_pack::spec_producer(),|record,native|super::native_pack::reconstruct_record_controlled(record,native,maximum_rows),control)}
 fn retire_sqlite_snapshot(self){super::native_pack::retire(self)}
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut bound=NativeEncodingBound::new(control)?;bound.add(1024)?;bound.repeated(self.schema.len(),6)?;for comment in&self.comments{bound.add(64)?;bound.repeated(comment.len(),6)?;}for element in&self.elements{bound.add(256)?;bound.repeated(element.name.len(),6)?;for property in&element.properties{bound.add(256)?;bound.repeated(property.name().len(),6)?;}for row in&element.rows{bound.add(64)?;let mut pending=vec![row.values.iter()];while let Some(values)=pending.last_mut(){if let Some(value)=values.next(){bound.add(1164)?;if let PlyValue::List(values)=value{pending.push(values.iter());}}else{pending.pop();}}}}bound.finish()}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;let mut rows=0usize;let mut bytes=0usize;let mut counts=BTreeMap::new();
  visit(self,|name,cells,columns|{rows=rows.checked_add(1).ok_or("PLY row overflow")?;control.check_rows(rows)?;bytes=bytes.checked_add(8).ok_or("PLY value byte overflow")?;for cell in cells{let n=match cell{Cell::Text(value)=>value.len(),Cell::Null=>0,Cell::Integer(_)=>8,Cell::Real(value)=>if value.is_nan(){11}else if value.is_infinite(){32}else{22},Cell::Float32(value)=>if value.is_nan(){11}else if value.is_infinite(){32}else{22},Cell::Blob(value)=>value.len()};bytes=bytes.checked_add(n).ok_or("PLY value byte overflow")?;}control.check_value_bytes(bytes)?;if rows%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,rows)?;}let count=counts.entry(name.to_owned()).or_insert(0i64);*count+=1;Ok(*count)})?;
  let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;visit(self,|name,cells,columns|insert_ieee754(&mut projection,name,cells,columns))?;projection.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|e|e.to_string())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  for(name,width)in[("ply_document",3),("ply_comment",4),("ply_element",6),("ply_property",8),("ply_row",3),("ply_cell",4),("ply_value",6),("ply_list_item",4)]{ids(database,name)?;for row in&database.table(name)?.rows{if row.values.len()!=width{return Err("PLY row width differs".into())}}}
  let document=database.table("ply_document")?.single_row()?;let document_ids=BTreeSet::from([document.rowid]);let format=match document.text(2)?{"ascii"=>PlyFormat::Ascii,"binary_little_endian"=>PlyFormat::BinaryLittleEndian,"binary_big_endian"=>PlyFormat::BinaryBigEndian,_=>return Err("PLY format differs".into())};
  let mut comments=groups(database,"ply_comment",&document_ids,control)?;let mut elements=groups(database,"ply_element",&document_ids,control)?;let element_ids=ids(database,"ply_element")?;let mut properties=groups(database,"ply_property",&element_ids,control)?;let mut rows=groups(database,"ply_row",&element_ids,control)?;let mut cells=groups(database,"ply_cell",&ids(database,"ply_row")?,control)?;let items=groups(database,"ply_list_item",&ids(database,"ply_value")?,control)?;
  let values:BTreeMap<_,_>=database.table("ply_value")?.rows.iter().map(|row|(row.rowid,row)).collect();let mut owners=BTreeSet::new();for table in["ply_cell","ply_list_item"]{for row in&database.table(table)?.rows{let id=row.integer(3)?;if !values.contains_key(&id)||!owners.insert(id){return Err("PLY value must have one known owner".into())}}}if owners.len()!=values.len(){return Err("PLY unowned value entity".into())}
  let total=values.len();let mut completed=0;let mut forest=Forest(BTreeMap::new());let mut active=BTreeSet::new();let mut visited=BTreeSet::new();
  for cell in&database.table("ply_cell")?.rows{let mut pending=vec![(cell.integer(3)?,false)];while let Some((id,exit))=pending.pop(){if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed+pending.len()+1)?;}completed+=1;let row=values.get(&id).ok_or("PLY value owner differs")?;let children=items.get(&id).map(Vec::as_slice).unwrap_or(&[]);
    if exit{active.remove(&id);let mut owned=Items(Vec::with_capacity(children.len()));for child in children{owned.0.push(forest.0.remove(&child.integer(3)?).ok_or("PLY value topology differs")?);}forest.0.insert(id,PlyValue::List(std::mem::take(&mut owned.0)));}
    else{if active.contains(&id)||!visited.insert(id){return Err("PLY value topology cycles or repeats".into())}if row.text(1)?=="list"{if row.values[2]!=SqliteValue::Null||!FloatRow::new(row,BINARY64)?.is_null(3)?{return Err("PLY list has scalar payload".into())}active.insert(id);if pending.len().checked_add(children.len()).and_then(|n|n.checked_add(1)).ok_or("PLY frontier overflow")?>total+1{return Err("PLY value frontier limit".into())}pending.push((id,true));for child in children.iter().rev(){pending.push((child.integer(3)?,false));}}
    else{if !children.is_empty(){return Err("PLY scalar owns list children".into())}forest.0.insert(id,read_scalar(row)?);}}
   }}
  if visited.len()!=values.len(){return Err("PLY unreachable value entities".into())}
  let mut snapshot=dsl::__rt::DecodedFieldOwner::new(Self{schema:document.text(1)?.into(),format,comments:Vec::new(),elements:Vec::new()},super::native_pack::retire);for row in comments.remove(&document.rowid).unwrap_or_default(){snapshot.as_mut().comments.push(row.text(3)?.into());}
  for element in elements.remove(&document.rowid).unwrap_or_default(){let id=element.rowid;let high=u32::try_from(element.integer(4)?).map_err(|_|"PLY declared count word exceeds unsigned32")?;let low=u32::try_from(element.integer(5)?).map_err(|_|"PLY declared count word exceeds unsigned32")?;let mut decoded=dsl::__rt::DecodedFieldOwner::new(PlyElement{name:element.text(3)?.into(),count:(u64::from(high)<<32)|u64::from(low),properties:Vec::new(),rows:Vec::new()},|element:PlyElement|{for row in element.rows{for value in row.values{super::native_pack::retire_value(value)}}});
   for property in properties.remove(&id).unwrap_or_default(){let name=property.text(3)?.into();let value=match property.text(4)?{"scalar" if property.optional_text(6)?.is_none()&&property.optional_text(7)?.is_none()=>PlyProperty::Scalar{name,kind:scalar_kind(property.text(5)?)?},"list" if property.optional_text(5)?.is_none()=>PlyProperty::List{name,count_kind:scalar_kind(property.text(6)?)?,value_kind:scalar_kind(property.text(7)?)?},_=>return Err("PLY property variant differs".into())};decoded.as_mut().properties.push(value);}
   for row in rows.remove(&id).unwrap_or_default(){let mut decoded_row=Items(Vec::new());for cell in cells.remove(&row.rowid).unwrap_or_default(){decoded_row.0.push(forest.0.remove(&cell.integer(3)?).ok_or("PLY row value differs")?);}decoded.as_mut().rows.push(PlyRow{values:std::mem::take(&mut decoded_row.0)});}snapshot.as_mut().elements.push(decoded.take());
  }
  if !forest.0.is_empty(){return Err("PLY unconsumed values".into())}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)?;Ok(snapshot.take())
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.ply"||dialect.standard!="1.0"||dialect.subset!="*"{return Err(String::from("geometry owned SQLite dialect differs").into())}let candidate=Self::from_sqlite_database(database,control)?;let mut candidate=dsl::__rt::DecodedFieldOwner::new(candidate,super::native_pack::retire);let actual=candidate.as_mut().to_sqlite_database(control)?;if actual!=self.to_sqlite_database(control)?{return Err(String::from("PLY document identity differs").into())}Ok(store::io_schema::IoOutcome::clean(()))}
}
