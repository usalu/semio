//! 📐️ Original CAD projection retains scoped name indexes and every authored geometry cell.
use super::{SemioCadSnapshot,CadEntityRecord,CadEntity,number,float_columns};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::NameIndex;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,RowWriter,ProjectionStorage,project_owned,owned_workspace},transfer::reserve};
use semio_framework_value::{ValueError,ValueRefusalKind};

#[derive(semio_framework_value::RetireOwned)]
pub(crate)struct Scratch{layers:NameIndex,blocks:NameIndex,handles:Vec<NameIndex>}
impl Scratch{
 pub(crate)fn empty()->Self{Self{layers:NameIndex::empty(),blocks:NameIndex::empty(),handles:Vec::new()}}
 pub(crate)fn initialize(&mut self,snapshot:&SemioCadSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  self.layers.initialize(snapshot.layers.len(),|index|snapshot.layers[index].name.as_str(),phase,control,"duplicate Semio CAD layer name")?;
  self.blocks.initialize(snapshot.blocks.len(),|index|snapshot.blocks[index].name.as_str(),phase,control,"duplicate Semio CAD block name")?;
  let count=snapshot.blocks.len().checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"CAD scoped handle index extent"))?;self.handles=reserve(count,control)?;
  for block in &snapshot.blocks{self.handles.push(NameIndex::empty());self.handles.last_mut().unwrap().initialize(block.entities.len(),|index|block.entities[index].handle.as_str(),phase,control,"duplicate Semio CAD entity handle within owner")?;}
  self.handles.push(NameIndex::empty());self.handles.last_mut().unwrap().initialize(snapshot.entities.len(),|index|snapshot.entities[index].handle.as_str(),phase,control,"duplicate Semio CAD entity handle within owner")
 }
}

struct Names<'a>{snapshot:&'a SemioCadSnapshot,scratch:&'a Scratch}
impl Names<'_>{
 fn layer(&self,id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{self.scratch.layers.lookup(id,|index|self.snapshot.layers[index].name.as_str(),out,"dangling Semio CAD layer")}
 fn block(&self,id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{self.scratch.blocks.lookup(id,|index|self.snapshot.blocks[index].name.as_str(),out,"dangling Semio CAD block")}
}

fn record(record:&CadEntityRecord,block:Option<i64>,ordinal:usize,names:&Names<'_>,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let kind=match record.entity{CadEntity::Line{..}=>"line",CadEntity::Arc{..}=>"arc",CadEntity::Circle{..}=>"circle",CadEntity::Ellipse{..}=>"ellipse",CadEntity::Polyline{..}=>"polyline",CadEntity::Text{..}=>"text",CadEntity::Insert{..}=>"insert",CadEntity::Solid{..}=>"solid",CadEntity::Dimension{..}=>"dimension"};let layer=names.layer(&record.layer,out)?;let id=out.insert("semio_cad_entity",&[Cell::Integer(1),block.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Integer(number(ordinal)?),Cell::Text(&record.handle),Cell::Integer(layer),Cell::Text(kind)])?;
 match &record.entity{
  CadEntity::Line{a,b}=>out.insert_key_float("semio_cad_line",id,&[Cell::Real(a.x),Cell::Real(a.y),Cell::Real(b.x),Cell::Real(b.y)],float_columns("semio_cad_line"))?,
  CadEntity::Arc{center,radius,start_angle,end_angle}=>out.insert_key_float("semio_cad_arc",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius),Cell::Real(*start_angle),Cell::Real(*end_angle)],float_columns("semio_cad_arc"))?,
  CadEntity::Circle{center,radius}=>out.insert_key_float("semio_cad_circle",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius)],float_columns("semio_cad_circle"))?,
  CadEntity::Ellipse{center,major_axis_end,ratio,start_param,end_param}=>out.insert_key_float("semio_cad_ellipse",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(major_axis_end.x),Cell::Real(major_axis_end.y),Cell::Real(*ratio),Cell::Real(*start_param),Cell::Real(*end_param)],float_columns("semio_cad_ellipse"))?,
  CadEntity::Polyline{vertices,closed}=>{out.insert_key_float("semio_cad_polyline",id,&[Cell::Integer(i64::from(*closed))],float_columns("semio_cad_polyline"))?;for(ordinal,vertex)in vertices.iter().enumerate(){out.insert_float("semio_cad_polyline_vertex",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(vertex.x),Cell::Real(vertex.y)],float_columns("semio_cad_polyline_vertex"))?;}},
  CadEntity::Text{position,height,rotation,content}=>out.insert_key_float("semio_cad_text",id,&[Cell::Real(position.x),Cell::Real(position.y),Cell::Real(*height),Cell::Real(*rotation),Cell::Text(content)],float_columns("semio_cad_text"))?,
  CadEntity::Insert{block_name,insertion_point,scale,rotation}=>{let target=names.block(block_name,out)?;out.insert_key_float("semio_cad_insert",id,&[Cell::Integer(target),Cell::Real(insertion_point.x),Cell::Real(insertion_point.y),Cell::Real(scale.x),Cell::Real(scale.y),Cell::Real(*rotation)],float_columns("semio_cad_insert"))?;},
  CadEntity::Solid{p1,p2,p3,p4}=>out.insert_key_float("semio_cad_solid",id,&[Cell::Real(p1.x),Cell::Real(p1.y),Cell::Real(p2.x),Cell::Real(p2.y),Cell::Real(p3.x),Cell::Real(p3.y),Cell::Real(p4.x),Cell::Real(p4.y)],float_columns("semio_cad_solid"))?,
  CadEntity::Dimension{def_point,text_position,measurement,text}=>out.insert_key_float("semio_cad_dimension",id,&[Cell::Real(def_point.x),Cell::Real(def_point.y),Cell::Real(text_position.x),Cell::Real(text_position.y),Cell::Real(*measurement),Cell::Text(text)],float_columns("semio_cad_dimension"))?
 }Ok(())
}

/// 🫳️ Reuses the exact original scoped identities for every layer, block and entity in both passes.
pub(crate)fn visit(snapshot:&SemioCadSnapshot,scratch:&Scratch,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let names=Names{snapshot,scratch};out.insert_key("semio_cad_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,layer)in snapshot.layers.iter().enumerate(){out.insert("semio_cad_layer",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&layer.name),Cell::Integer(i64::from(layer.color_index)),Cell::Text(&layer.line_type),Cell::Integer(i64::from(layer.visible))])?;}
 for(ordinal,block)in snapshot.blocks.iter().enumerate(){out.insert_float("semio_cad_block",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&block.name),Cell::Real(block.base_point.x),Cell::Real(block.base_point.y)],float_columns("semio_cad_block"))?;}
 for(index,block)in snapshot.blocks.iter().enumerate(){for(ordinal,entity)in block.entities.iter().enumerate(){record(entity,Some(number(index+1)?),ordinal,&names,out)?;}}
 for(ordinal,entity)in snapshot.entities.iter().enumerate(){record(entity,None,ordinal,&names,out)?;}Ok(())
}

#[derive(semio_framework_value::RetireOwned)]
struct Prefix{projection:ProjectionStorage,scratch:Scratch}

/// 🧮️ Admits actual CAD SQL and source ordinal owners before any original partial field can be created.
pub(super)fn project(snapshot:&SemioCadSnapshot,sql:&str,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 project_owned(control,||Prefix{projection:ProjectionStorage::empty(),scratch:Scratch::empty()},|prefix,control|{
  prefix.scratch.initialize(snapshot,SqliteSnapshotPhase::ProjectSnapshot,control)?;let mut census=RowWriter::census(&mut prefix.projection,sql,control)?;visit(snapshot,&prefix.scratch,&mut census)?;census.finish_census()?;
  let mut output=RowWriter::new_into(&mut prefix.projection,control)?;visit(snapshot,&prefix.scratch,&mut output)?;output.finish_into()
 },|prefix|prefix.projection.take_database())
}

/// 🎟️ Preserves actual scoped name indexes through native semantic admission in the original phase.
pub(super)fn admit(snapshot:&SemioCadSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 owned_workspace(control,phase,Scratch::empty,|scratch,control|{scratch.initialize(snapshot,phase,control)?;let mut writer=RowWriter::borrowed(control,phase)?;visit(snapshot,scratch,&mut writer)?;writer.finish_borrowed()},|_|())
}
