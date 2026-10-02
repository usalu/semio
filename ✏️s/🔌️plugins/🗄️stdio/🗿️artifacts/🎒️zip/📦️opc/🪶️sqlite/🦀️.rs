//! 📦️ Literal OPC package entities for individually authored container schemas.
use super::{OpcPackage,OpcPart,OpcContentTypes,OpcRelationship,OpcTargetMode};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{reconstruct_text,reconstruct_blob}};
use std::collections::{BTreeMap,BTreeSet};

/// 🗂️ Six explicitly declared OPC entity tables.
#[derive(Clone,Copy)]
pub struct OpcSqliteTables{pub package:&'static str,pub part:&'static str,pub default_type:&'static str,pub override_type:&'static str,pub relationship_owner:&'static str,pub relationship:&'static str}

fn add(total:&mut usize,value:usize)->Result<(),String>{*total=total.checked_add(value).ok_or("OPC relational size overflow")?;Ok(())}
fn mode(value:OpcTargetMode)->&'static str{match value{OpcTargetMode::Internal=>"internal",OpcTargetMode::External=>"external"}}

/// 📏️ Measures the actual typed OPC rows and scalar bytes before ownership.
pub fn measure_opc_package(package:&OpcPackage,control:&mut SqliteSnapshotControl<'_>)->Result<(usize,usize),String>{
 let(mut rows,mut bytes)=(1usize,8usize);add(&mut bytes,package.comment.len())?;control.check_rows(rows)?;control.check_value_bytes(bytes)?;
 for part in &package.parts{add(&mut rows,1)?;add(&mut bytes,24)?;add(&mut bytes,part.path.len())?;add(&mut bytes,part.content_type.len())?;add(&mut bytes,part.bytes.len())?;control.check_rows(rows)?;control.check_value_bytes(bytes)?;if rows%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;}}
 for pair in package.content_types.defaults.iter().chain(&package.content_types.overrides){add(&mut rows,1)?;add(&mut bytes,24)?;add(&mut bytes,pair.0.len())?;add(&mut bytes,pair.1.len())?;control.check_rows(rows)?;control.check_value_bytes(bytes)?;if rows%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;}}
 for(owner,relationships)in &package.relationships{add(&mut rows,1)?;add(&mut bytes,16)?;add(&mut bytes,owner.len())?;control.check_rows(rows)?;control.check_value_bytes(bytes)?;if rows%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;}
  for relationship in relationships{add(&mut rows,1)?;add(&mut bytes,24)?;for size in [relationship.id.len(),relationship.rel_type.len(),relationship.target.len(),mode(relationship.target_mode).len()]{add(&mut bytes,size)?;}control.check_rows(rows)?;control.check_value_bytes(bytes)?;if rows%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;}}
 }
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,rows)?;Ok((rows,bytes))
}

fn text(value:&str,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteValue,String>{semio_framework_os_kernel::sqlite_snapshot::artifact::project_text(control,value).map(SqliteValue::Text)}
fn blob(value:&[u8],control:&mut SqliteSnapshotControl<'_>)->Result<SqliteValue,String>{
 control.check_value_bytes(value.len())?;if value.len()>65536{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,value.len())?;}let mut bytes=Vec::new();bytes.try_reserve_exact(value.len()).map_err(|_|"OPC intrinsic bytes allocation")?;
 for chunk in value.chunks(65536){bytes.extend_from_slice(chunk);if value.len()>65536{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,bytes.len(),value.len())?;}}Ok(SqliteValue::Blob(bytes))
}
fn push(database:&mut SqliteDatabase,table:&str,values:Vec<SqliteValue>)->Result<i64,String>{let rows=&mut database.table_mut(table)?.rows;let key=i64::try_from(rows.len()+1).map_err(|_|"OPC row identity overflow")?;let mut fields=vec![SqliteValue::Integer(key)];fields.extend(values);rows.push(SqliteRow{rowid:key,values:fields});Ok(key)}
fn integer(value:usize)->Result<SqliteValue,String>{Ok(SqliteValue::Integer(i64::try_from(value).map_err(|_|"OPC ordinal overflow")?))}

/// 🏗️ Appends one typed package into its caller-owned empty OPC tables.
pub fn project_opc_package(package:&OpcPackage,database:&mut SqliteDatabase,tables:OpcSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<i64,String>{
 control.check_database(database,SqliteSnapshotPhase::ProjectSnapshot)?;for name in [tables.package,tables.part,tables.default_type,tables.override_type,tables.relationship_owner,tables.relationship]{if !database.table(name)?.rows.is_empty(){return Err("OPC projection tables must be empty".into());}}
 let(rows,bytes)=measure_opc_package(package,control)?;let mut existing_rows=0usize;let mut existing_bytes=0usize;for table in &database.tables{for row in &table.rows{add(&mut existing_rows,1)?;for value in &row.values{add(&mut existing_bytes,match value{SqliteValue::Null=>0,SqliteValue::Integer(_)|SqliteValue::Real(_)=>8,SqliteValue::Text(value)=>value.len(),SqliteValue::Blob(value)=>value.len()})?;}}}add(&mut existing_rows,rows)?;add(&mut existing_bytes,bytes)?;control.check_rows(existing_rows)?;control.check_value_bytes(existing_bytes)?;
 let id=push(database,tables.package,vec![text(&package.comment,control)?])?;let mut completed=1usize;
 for(ordinal,part)in package.parts.iter().enumerate(){push(database,tables.part,vec![SqliteValue::Integer(id),integer(ordinal)?,text(&part.path,control)?,text(&part.content_type,control)?,blob(&part.bytes,control)?])?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,rows)?;}}
 for(name,pairs)in [(tables.default_type,&package.content_types.defaults),(tables.override_type,&package.content_types.overrides)]{for(ordinal,pair)in pairs.iter().enumerate(){push(database,name,vec![SqliteValue::Integer(id),integer(ordinal)?,text(&pair.0,control)?,text(&pair.1,control)?])?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,rows)?;}}}
 for(owner,relationships)in &package.relationships{let parent=push(database,tables.relationship_owner,vec![SqliteValue::Integer(id),text(owner,control)?])?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,rows)?;}
  for(ordinal,relationship)in relationships.iter().enumerate(){push(database,tables.relationship,vec![SqliteValue::Integer(parent),integer(ordinal)?,text(&relationship.id,control)?,text(&relationship.rel_type,control)?,text(&relationship.target,control)?,text(mode(relationship.target_mode),control)?])?;completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,rows)?;}}
 }
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,rows)?;Ok(id)
}

fn entities<'a>(database:&'a SqliteDatabase,name:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<&'a[SqliteRow],String>{let rows=&database.table(name)?.rows;let mut keys=BTreeSet::new();for(position,row)in rows.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,rows.len())?;}if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||!keys.insert(row.rowid){return Err("OPC entity columns or identity are invalid".into());}}Ok(rows)}
fn ordered<'a>(rows:impl IntoIterator<Item=&'a SqliteRow>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,String>{let mut result:BTreeMap<i64,&SqliteRow>=BTreeMap::new();for(position,row)in rows.into_iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,0)?;}if result.insert(row.integer(2)?,row).is_some(){return Err("OPC ordinal has multiple owners".into());}}for(position,key)in result.keys().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,result.len())?;}if *key!=i64::try_from(position).map_err(|_|"OPC ordinal overflow")?{return Err("OPC ordered relationship is not dense".into());}}Ok(result.into_values().collect())}

/// 🧱️ Reconstructs exactly the typed package without ordinary archive normalization.
pub fn reconstruct_opc_package(database:&SqliteDatabase,tables:OpcSqliteTables,control:&mut SqliteSnapshotControl<'_>)->Result<OpcPackage,String>{
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;let package=entities(database,tables.package,2,control)?;if package.len()!=1||package[0].rowid!=1{return Err("OPC graph requires one package identity".into());}let id=package[0].rowid;
 let mut result=OpcPackage{comment:reconstruct_text(control,package[0].text(1)?)?,parts:Vec::new(),content_types:OpcContentTypes::default(),relationships:BTreeMap::new()};
 let rows=entities(database,tables.part,6,control)?;for row in rows{if row.integer(1)?!=id{return Err("OPC part package is dangling".into());}}for row in ordered(rows,control)?{result.parts.push(OpcPart{path:reconstruct_text(control,row.text(3)?)?,content_type:reconstruct_text(control,row.text(4)?)?,bytes:reconstruct_blob(control,row.blob(5)?)?});}
 for(name,pairs)in [(tables.default_type,&mut result.content_types.defaults),(tables.override_type,&mut result.content_types.overrides)]{let rows=entities(database,name,5,control)?;for row in rows{if row.integer(1)?!=id{return Err("OPC content type package is dangling".into());}}for row in ordered(rows,control)?{pairs.push((reconstruct_text(control,row.text(3)?)?,reconstruct_text(control,row.text(4)?)?));}}
 let owners=entities(database,tables.relationship_owner,3,control)?;let mut by_owner=BTreeMap::new();for row in owners{if row.integer(1)?!=id{return Err("OPC relationship package is dangling".into());}let name=reconstruct_text(control,row.text(2)?)?;if result.relationships.insert(name,Vec::new()).is_some(){return Err("OPC relationship dictionary has duplicate keys".into());}by_owner.insert(row.rowid,row.text(2)?);}
 let mut relationships=BTreeMap::<i64,Vec<&SqliteRow>>::new();for row in entities(database,tables.relationship,7,control)?{let parent=row.integer(1)?;if !by_owner.contains_key(&parent){return Err("OPC relationship owner is dangling".into());}relationships.entry(parent).or_default().push(row);}
 for(parent,rows)in relationships{let target=result.relationships.get_mut(by_owner[&parent]).ok_or("OPC relationship owner missing")?;for row in ordered(rows,control)?{target.push(OpcRelationship{id:reconstruct_text(control,row.text(3)?)?,rel_type:reconstruct_text(control,row.text(4)?)?,target:reconstruct_text(control,row.text(5)?)?,target_mode:match row.text(6)?{"internal"=>OpcTargetMode::Internal,"external"=>OpcTargetMode::External,_=>return Err("OPC relationship mode is invalid".into())}});}}
 Ok(result)
}
