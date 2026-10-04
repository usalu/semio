//! 🧩️ Handwritten board, ordered catalog and exact optional scalar ownership.
use super::Puzzle2dSnapshot;
use crate::{Puzzle2dCamera,Puzzle2dNode,Puzzle2dNodeAnchor,Puzzle2dHandle,Puzzle2dEdge,Puzzle2dTargetRegion,Puzzle2dMeta,Puzzle2dKindCompatibility,Puzzle2dCompatSpecificity,Puzzle2dKindCatalogs,Puzzle2dCatalogNodeKind,Puzzle2dRepresentation,Puzzle2dHandleTemplate,Puzzle2dAttribute,Puzzle2dAuthor,Puzzle2dCatalogHandleKind,Puzzle2dCatalogEdgeKind,Puzzle2dCatalogWireKind};
use std::collections::{BTreeMap,BTreeSet};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound,FloatColumn,insert_ieee754,read_binary64,ieee754_is_null}}};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

const CAMERA:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const NODE:&[FloatColumn]=&[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(14)];
const HANDLE:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(9)];
const EDGE:&[FloatColumn]=&[FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14)];
const REGION:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const TEMPLATE:&[FloatColumn]=&[FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(12)];
fn optional_text(value:&Option<String>)->Cell<'_>{value.as_deref().map_or(Cell::Null,Cell::Text)}
fn optional_real(value:Option<f64>)->Cell<'static>{value.map_or(Cell::Null,Cell::Real)}
fn optional_flag(value:Option<bool>)->Cell<'static>{value.map_or(Cell::Null,|value|Cell::Integer(i64::from(value)))}
fn optional_i32(value:Option<i32>)->Cell<'static>{value.map_or(Cell::Null,|value|Cell::Integer(i64::from(value)))}
fn ordinal(value:usize)->Result<Cell<'static>,ValueError>{Ok(Cell::Integer(i64::try_from(value).map_err(|e|invalid(e.to_string()))?))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid{return Err(invalid("Puzzle 2D requires complete positive aliased entities"))}Ok(())}
fn flag(row:&SqliteRow,index:usize)->Result<bool,ValueError>{match row.integer(index)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("Puzzle 2D flag exceeds Boolean domain"))}}
fn restore_optional_flag(row:&SqliteRow,index:usize)->Result<Option<bool>,ValueError>{if matches!(row.values.get(index),Some(SqliteValue::Null)){Ok(None)}else{flag(row,index).map(Some)}}
fn restore_optional_i32(row:&SqliteRow,index:usize)->Result<Option<i32>,ValueError>{if matches!(row.values.get(index),Some(SqliteValue::Null)){Ok(None)}else{Ok(Some(i32::try_from(row.integer(index)?).map_err(|e|invalid(e.to_string()))?))}}
fn restore_optional_real(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<Option<f64>,ValueError>{if ieee754_is_null(row,index,columns)?{Ok(None)}else{read_binary64(row,index,columns).map(Some)}}
fn restore_optional_text(row:&SqliteRow,index:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<String>,ValueError>{match row.values.get(index){Some(SqliteValue::Null)=>Ok(None),Some(SqliteValue::Text(value))=>control.copy_text(value).map(Some),_=>Err(invalid("Puzzle 2D optional TEXT has another storage class"))}}
fn specificity(value:Puzzle2dCompatSpecificity)->&'static str{match value{Puzzle2dCompatSpecificity::General=>"general",Puzzle2dCompatSpecificity::Node=>"node",Puzzle2dCompatSpecificity::Edge=>"edge",Puzzle2dCompatSpecificity::Handle=>"handle",Puzzle2dCompatSpecificity::Wire=>"wire",Puzzle2dCompatSpecificity::Vortex=>"vortex"}}
fn restore_specificity(value:&str)->Result<Puzzle2dCompatSpecificity,ValueError>{match value{"general"=>Ok(Puzzle2dCompatSpecificity::General),"node"=>Ok(Puzzle2dCompatSpecificity::Node),"edge"=>Ok(Puzzle2dCompatSpecificity::Edge),"handle"=>Ok(Puzzle2dCompatSpecificity::Handle),"wire"=>Ok(Puzzle2dCompatSpecificity::Wire),"vortex"=>Ok(Puzzle2dCompatSpecificity::Vortex),_=>Err(invalid("Puzzle 2D compatibility specificity is undeclared"))}}
fn workload(snapshot:&Puzzle2dSnapshot,checkpoint:&mut impl FnMut(usize)->Result<(),ValueError>)->Result<usize,ValueError>{
 let mut rows=3usize;let mut units=0usize;let mut add=|count:usize|->Result<(),ValueError>{rows=rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle 2D row workload overflow"))?;units+=1;if units%256==0{checkpoint(units)?}Ok(())};
 add(snapshot.nodes.len())?;add(snapshot.edges.len())?;add(snapshot.target_regions.len())?;add(snapshot.meta.kind_compatibility.len())?;for node in &snapshot.nodes{add(node.handles.len())?;}
 if let Some(catalogs)=&snapshot.meta.kind_catalogs{add(1)?;add(catalogs.nodes.len())?;add(catalogs.handles.len())?;add(catalogs.edges.len())?;add(catalogs.wires.len())?;for node in &catalogs.nodes{add(node.base_kinds.len())?;add(node.representations.len())?;add(node.handles.len())?;add(node.attributes.len())?;add(node.authors.len())?;for representation in &node.representations{add(representation.tags.len())?;}}for handle in &catalogs.handles{add(handle.compatible_with.len())?;}}
 checkpoint(units)?;Ok(rows)
}
fn bound_text(bound:&mut NativeEncodingBound<'_,'_>,value:&str)->Result<(),ValueError>{bound.add(value.len().checked_mul(6).and_then(|n|n.checked_add(192)).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Puzzle 2D encoded text bound overflow"))?)}
fn bound_optional_text(bound:&mut NativeEncodingBound<'_,'_>,value:&Option<String>)->Result<(),ValueError>{if let Some(value)=value{bound_text(bound,value)?}Ok(())}
fn bound_strings(snapshot:&Puzzle2dSnapshot,bound:&mut NativeEncodingBound<'_,'_>)->Result<(),ValueError>{
 bound_text(bound,&snapshot.schema)?;bound_optional_text(bound,&snapshot.meta.manifest_id)?;
 for node in &snapshot.nodes{bound_text(bound,&node.id)?;for value in[&node.node_kind,&node.shape,&node.text,&node.icon_kind]{bound_optional_text(bound,value)?;}for handle in &node.handles{bound_text(bound,&handle.id)?;for value in[&handle.handle_kind,&handle.color,&handle.icon_kind]{bound_optional_text(bound,value)?;}}}
 for edge in &snapshot.edges{for value in[&edge.id,&edge.source,&edge.target]{bound_text(bound,value)?;}for value in[&edge.edge_kind,&edge.source_tip,&edge.target_tip]{bound_optional_text(bound,value)?;}}
 for region in &snapshot.target_regions{bound_text(bound,&region.id)?;bound_optional_text(bound,&region.label)?;}
 for rule in &snapshot.meta.kind_compatibility{bound_text(bound,&rule.source)?;bound_text(bound,&rule.target)?;}
 if let Some(catalogs)=&snapshot.meta.kind_catalogs{
  for node in &catalogs.nodes{for value in[&node.id,&node.name,&node.label,&node.description,&node.icon,&node.image,&node.unit]{bound_text(bound,value)?;}for value in &node.base_kinds{bound_text(bound,value)?;}
   for representation in &node.representations{for value in[&representation.id,&representation.name,&representation.url,&representation.mime,&representation.description]{bound_text(bound,value)?;}bound_optional_text(bound,&representation.lod)?;for value in &representation.tags{bound_text(bound,value)?;}}
   for handle in &node.handles{for value in[&handle.id,&handle.name,&handle.label,&handle.description,&handle.icon]{bound_text(bound,value)?;}bound_optional_text(bound,&handle.handle_kind)?;}
   for attribute in &node.attributes{for value in[&attribute.id,&attribute.key,&attribute.value]{bound_text(bound,value)?;}bound_optional_text(bound,&attribute.definition)?;}
   for author in &node.authors{for value in[&author.id,&author.name,&author.email]{bound_text(bound,value)?;}bound_optional_text(bound,&author.role)?;}
  }
  for handle in &catalogs.handles{for value in[&handle.id,&handle.description,&handle.icon,&handle.color,&handle.default_wire_kind]{bound_text(bound,value)?;}bound_optional_text(bound,&handle.code)?;bound_optional_text(bound,&handle.label)?;for value in &handle.compatible_with{bound_text(bound,value)?;}}
  for edge in &catalogs.edges{for value in[&edge.id,&edge.name,&edge.label,&edge.description,&edge.icon,&edge.color]{bound_text(bound,value)?;}}
  for wire in &catalogs.wires{for value in[&wire.id,&wire.name,&wire.label,&wire.description,&wire.icon,&wire.color,&wire.default_edge_kind]{bound_text(bound,value)?;}}
 }Ok(())
}
struct Members<'a>{groups:BTreeMap<i64,BTreeMap<usize,&'a SqliteRow>>}
impl<'a> Members<'a>{
 fn new(rows:&'a[SqliteRow],columns:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{
  control.charge(rows.len().checked_mul(384).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Puzzle 2D lookup byte overflow"))?)?;control.begin_stage(rows.len())?;let mut identities=BTreeSet::new();let mut groups:BTreeMap<i64,BTreeMap<usize,&'a SqliteRow>>=BTreeMap::new();
  for row in rows{control.step()?;identity(row,columns)?;let parent=row.integer(1)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|invalid(e.to_string()))?;if parent<=0||!identities.insert(row.rowid)||groups.entry(parent).or_default().insert(ordinal,row).is_some(){return Err(invalid("Puzzle 2D repeats surrogate identity or owner ordinal"))}}
  control.checkpoint()?;Ok(Self{groups})
 }
 fn take(&mut self,parent:i64,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
  let Some(rows)=self.groups.remove(&parent)else{return Ok(Vec::new())};let mut output=control.allocate_vec::<&SqliteRow>(rows.len())?;control.begin_stage(rows.len())?;for(ordinal,(actual,row))in rows.into_iter().enumerate(){control.step()?;if ordinal!=actual{return Err(invalid("Puzzle 2D collection ordinals are not dense"))}output.push(row)}control.checkpoint()?;Ok(output)
 }
 fn finish(&self)->Result<(),ValueError>{if self.groups.is_empty(){Ok(())}else{Err(invalid("Puzzle 2D has unmatched owned entities"))}}
}

fn put(out:&mut Projection<'_,'_>,completed:&mut usize,total:usize,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
 let key=if columns.is_empty(){out.insert(table,cells)?}else{insert_ieee754(out,table,cells,columns)?};*completed+=1;if *completed%256==0{out.checkpoint_total(total)?}Ok(key)
}
fn project(snapshot:&Puzzle2dSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 let mut out=Projection::new(<Puzzle2dSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA,control)?;let total=workload(snapshot,&mut |_|out.checkpoint())?;out.check_rows(total)?;out.checkpoint_total(total)?;let mut completed=0usize;
 put(&mut out,&mut completed,total,"puzzle2d_document",&[Cell::Text(&snapshot.schema)],&[])?;
 let camera=&snapshot.camera;put(&mut out,&mut completed,total,"puzzle2d_camera",&[Cell::Integer(1),Cell::Real(camera.x),Cell::Real(camera.y),Cell::Real(camera.zoom)],CAMERA)?;
 let meta=put(&mut out,&mut completed,total,"puzzle2d_meta",&[Cell::Integer(1),optional_text(&snapshot.meta.manifest_id)],&[])?;
 for(i,node)in snapshot.nodes.iter().enumerate(){
  let key=put(&mut out,&mut completed,total,"puzzle2d_node",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&node.id),optional_text(&node.node_kind),optional_text(&node.shape),Cell::Real(node.x),Cell::Real(node.y),optional_real(node.radius),optional_real(node.width),optional_real(node.height),optional_text(&node.text),optional_text(&node.icon_kind),optional_flag(node.root),optional_real(node.scale),optional_flag(node.visible),optional_flag(node.locked),Cell::Text(match node.anchor{Puzzle2dNodeAnchor::Fixed=>"fixed",Puzzle2dNodeAnchor::Derived=>"derived"})],NODE)?;
  for(i,handle)in node.handles.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_handle",&[Cell::Integer(key),ordinal(i)?,Cell::Text(&handle.id),optional_text(&handle.handle_kind),Cell::Real(handle.angle),optional_real(handle.radius),optional_text(&handle.color),optional_text(&handle.icon_kind),optional_real(handle.scale),optional_flag(handle.visible),optional_flag(handle.locked)],HANDLE)?;}
 }
 for(i,edge)in snapshot.edges.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_edge",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&edge.id),Cell::Text(&edge.source),Cell::Text(&edge.target),optional_text(&edge.edge_kind),Cell::Real(edge.gap),Cell::Real(edge.shift),Cell::Real(edge.rise),Cell::Real(edge.rotation),Cell::Real(edge.turn),Cell::Real(edge.tilt),Cell::Real(edge.x),Cell::Real(edge.y),optional_text(&edge.source_tip),optional_text(&edge.target_tip),optional_flag(edge.visible),optional_flag(edge.locked)],EDGE)?;}
 for(i,region)in snapshot.target_regions.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_target_region",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&region.id),Cell::Real(region.x),Cell::Real(region.y),Cell::Real(region.width),Cell::Real(region.height),optional_text(&region.label),Cell::Integer(i64::from(region.hidden)),Cell::Integer(i64::from(region.locked))],REGION)?;}
 for(i,rule)in snapshot.meta.kind_compatibility.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_kind_compatibility",&[Cell::Integer(meta),ordinal(i)?,Cell::Text(&rule.source),Cell::Text(&rule.target),Cell::Integer(i64::from(rule.bidirectional)),Cell::Integer(i64::from(rule.important)),Cell::Text(specificity(rule.specificity))],&[])?;}
 if let Some(catalogs)=&snapshot.meta.kind_catalogs{
  let key=put(&mut out,&mut completed,total,"puzzle2d_kind_catalogs",&[Cell::Integer(meta)],&[])?;
  for(i,node)in catalogs.nodes.iter().enumerate(){
   let node_key=put(&mut out,&mut completed,total,"puzzle2d_catalog_node_kind",&[Cell::Integer(key),ordinal(i)?,Cell::Text(&node.id),Cell::Text(&node.name),Cell::Text(&node.label),Cell::Text(&node.description),Cell::Text(&node.icon),Cell::Text(&node.image),Cell::Text(&node.unit),Cell::Integer(i64::from(node.is_abstract))],&[])?;
   for(i,base)in node.base_kinds.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_base_kind",&[Cell::Integer(node_key),ordinal(i)?,Cell::Text(base)],&[])?;}
   for(i,representation)in node.representations.iter().enumerate(){
    let representation_key=put(&mut out,&mut completed,total,"puzzle2d_representation",&[Cell::Integer(node_key),ordinal(i)?,Cell::Text(&representation.id),Cell::Text(&representation.name),Cell::Text(&representation.url),Cell::Text(&representation.mime),optional_text(&representation.lod),Cell::Text(&representation.description)],&[])?;
    for(i,tag)in representation.tags.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_representation_tag",&[Cell::Integer(representation_key),ordinal(i)?,Cell::Text(tag)],&[])?;}
   }
   for(i,handle)in node.handles.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_handle_template",&[Cell::Integer(node_key),ordinal(i)?,Cell::Text(&handle.id),Cell::Text(&handle.name),Cell::Text(&handle.label),Cell::Text(&handle.description),Cell::Text(&handle.icon),optional_text(&handle.handle_kind),Cell::Real(handle.angle),optional_real(handle.t),optional_flag(handle.mandatory),optional_real(handle.radius)],TEMPLATE)?;}
   for(i,attribute)in node.attributes.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_attribute",&[Cell::Integer(node_key),ordinal(i)?,Cell::Text(&attribute.id),Cell::Text(&attribute.key),Cell::Text(&attribute.value),optional_text(&attribute.definition)],&[])?;}
   for(i,author)in node.authors.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_author",&[Cell::Integer(node_key),ordinal(i)?,Cell::Text(&author.id),Cell::Text(&author.name),Cell::Text(&author.email),optional_text(&author.role),optional_i32(author.rank)],&[])?;}
  }
  for(i,handle)in catalogs.handles.iter().enumerate(){
   let handle_key=put(&mut out,&mut completed,total,"puzzle2d_catalog_handle_kind",&[Cell::Integer(key),ordinal(i)?,Cell::Text(&handle.id),optional_text(&handle.code),optional_text(&handle.label),optional_i32(handle.order),Cell::Text(&handle.description),Cell::Text(&handle.icon),Cell::Text(&handle.color),Cell::Text(&handle.default_wire_kind)],&[])?;
   for(i,kind)in handle.compatible_with.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_compatible_kind",&[Cell::Integer(handle_key),ordinal(i)?,Cell::Text(kind)],&[])?;}
  }
  for(i,edge)in catalogs.edges.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_catalog_edge_kind",&[Cell::Integer(key),ordinal(i)?,Cell::Text(&edge.id),Cell::Text(&edge.name),Cell::Text(&edge.label),Cell::Text(&edge.description),Cell::Text(&edge.icon),Cell::Text(&edge.color)],&[])?;}
  for(i,wire)in catalogs.wires.iter().enumerate(){put(&mut out,&mut completed,total,"puzzle2d_catalog_wire_kind",&[Cell::Integer(key),ordinal(i)?,Cell::Text(&wire.id),Cell::Text(&wire.name),Cell::Text(&wire.label),Cell::Text(&wire.description),Cell::Text(&wire.icon),Cell::Text(&wire.color),Cell::Text(&wire.default_edge_kind)],&[])?;}
 }
 if completed!=total{return Err(invalid("Puzzle 2D projection differs from known owned workload"))}out.checkpoint_total(total)?;out.finish()
}

fn restore(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Puzzle2dSnapshot,ValueError>{
 restore_authority(database,control,0)
}

fn restore_authority(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,additional:usize)->Result<Puzzle2dSnapshot,ValueError>{
 validate_sqlite_database_schema(database,<Puzzle2dSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let documents=&database.table("puzzle2d_document")?.rows;let cameras=&database.table("puzzle2d_camera")?.rows;let metas=&database.table("puzzle2d_meta")?.rows;let catalog_rows=&database.table("puzzle2d_kind_catalogs")?.rows;
 if documents.len()!=1||documents[0].rowid!=1||documents[0].values.len()!=2||documents[0].integer(0)?!=1||cameras.len()!=1||metas.len()!=1||catalog_rows.len()>1{return Err(invalid("Puzzle 2D requires one complete board, camera and metadata with at most one catalog"))}identity(&cameras[0],11)?;identity(&metas[0],3)?;if cameras[0].integer(1)?!=1||metas[0].integer(1)?!=1{return Err(invalid("Puzzle 2D singleton has another board owner"))}for row in catalog_rows{identity(row,2)?;if row.integer(1)?!=metas[0].rowid{return Err(invalid("Puzzle 2D catalog has another metadata owner"))}}
 let maximum=control.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=semio_framework_value::NativeDecodeControl::new(maximum,&mut progress);
 native.charge(std::mem::size_of::<Puzzle2dSnapshot>().checked_add(additional).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Puzzle authority allocation overflow"))?)?;
 let mut nodes=Members::new(&database.table("puzzle2d_node")?.rows,30,&mut native)?;let mut handles=Members::new(&database.table("puzzle2d_handle")?.rows,18,&mut native)?;let mut edges=Members::new(&database.table("puzzle2d_edge")?.rows,35,&mut native)?;let mut regions=Members::new(&database.table("puzzle2d_target_region")?.rows,19,&mut native)?;let mut compatibility=Members::new(&database.table("puzzle2d_kind_compatibility")?.rows,8,&mut native)?;
 let mut kinds=Members::new(&database.table("puzzle2d_catalog_node_kind")?.rows,11,&mut native)?;let mut bases=Members::new(&database.table("puzzle2d_base_kind")?.rows,4,&mut native)?;let mut representations=Members::new(&database.table("puzzle2d_representation")?.rows,9,&mut native)?;let mut tags=Members::new(&database.table("puzzle2d_representation_tag")?.rows,4,&mut native)?;let mut templates=Members::new(&database.table("puzzle2d_handle_template")?.rows,19,&mut native)?;let mut attributes=Members::new(&database.table("puzzle2d_attribute")?.rows,7,&mut native)?;let mut authors=Members::new(&database.table("puzzle2d_author")?.rows,8,&mut native)?;
 let mut handle_kinds=Members::new(&database.table("puzzle2d_catalog_handle_kind")?.rows,11,&mut native)?;let mut compatible_kinds=Members::new(&database.table("puzzle2d_compatible_kind")?.rows,4,&mut native)?;let mut edge_kinds=Members::new(&database.table("puzzle2d_catalog_edge_kind")?.rows,9,&mut native)?;let mut wire_kinds=Members::new(&database.table("puzzle2d_catalog_wire_kind")?.rows,10,&mut native)?;
 let schema=native.copy_text(documents[0].text(1)?)?;let c=&cameras[0];let camera=Puzzle2dCamera{x:read_binary64(c,2,CAMERA)?,y:read_binary64(c,3,CAMERA)?,zoom:read_binary64(c,4,CAMERA)?};
 let node_rows=nodes.take(1,&mut native)?;let mut restored_nodes=native.allocate_vec::<Puzzle2dNode>(node_rows.len())?;
 for row in node_rows{
  let handle_rows=handles.take(row.rowid,&mut native)?;let mut restored_handles=native.allocate_vec::<Puzzle2dHandle>(handle_rows.len())?;native.begin_stage(handle_rows.len())?;
  for h in handle_rows{native.step()?;restored_handles.push(Puzzle2dHandle{id:native.copy_text(h.text(3)?)?,handle_kind:restore_optional_text(h,4,&mut native)?,angle:read_binary64(h,5,HANDLE)?,radius:restore_optional_real(h,6,HANDLE)?,color:restore_optional_text(h,7,&mut native)?,icon_kind:restore_optional_text(h,8,&mut native)?,scale:restore_optional_real(h,9,HANDLE)?,visible:restore_optional_flag(h,10)?,locked:restore_optional_flag(h,11)?});}native.checkpoint()?;
  restored_nodes.push(Puzzle2dNode{id:native.copy_text(row.text(3)?)?,node_kind:restore_optional_text(row,4,&mut native)?,shape:restore_optional_text(row,5,&mut native)?,x:read_binary64(row,6,NODE)?,y:read_binary64(row,7,NODE)?,radius:restore_optional_real(row,8,NODE)?,width:restore_optional_real(row,9,NODE)?,height:restore_optional_real(row,10,NODE)?,text:restore_optional_text(row,11,&mut native)?,icon_kind:restore_optional_text(row,12,&mut native)?,root:restore_optional_flag(row,13)?,scale:restore_optional_real(row,14,NODE)?,visible:restore_optional_flag(row,15)?,locked:restore_optional_flag(row,16)?,anchor:match row.text(17)?{"fixed"=>Puzzle2dNodeAnchor::Fixed,"derived"=>Puzzle2dNodeAnchor::Derived,_=>return Err(invalid("Puzzle 2D node anchor is undeclared"))},handles:restored_handles});native.checkpoint()?;
 }
 let edge_rows=edges.take(1,&mut native)?;let mut restored_edges=native.allocate_vec::<Puzzle2dEdge>(edge_rows.len())?;native.begin_stage(edge_rows.len())?;
 for row in edge_rows{native.step()?;restored_edges.push(Puzzle2dEdge{id:native.copy_text(row.text(3)?)?,source:native.copy_text(row.text(4)?)?,target:native.copy_text(row.text(5)?)?,edge_kind:restore_optional_text(row,6,&mut native)?,gap:read_binary64(row,7,EDGE)?,shift:read_binary64(row,8,EDGE)?,rise:read_binary64(row,9,EDGE)?,rotation:read_binary64(row,10,EDGE)?,turn:read_binary64(row,11,EDGE)?,tilt:read_binary64(row,12,EDGE)?,x:read_binary64(row,13,EDGE)?,y:read_binary64(row,14,EDGE)?,source_tip:restore_optional_text(row,15,&mut native)?,target_tip:restore_optional_text(row,16,&mut native)?,visible:restore_optional_flag(row,17)?,locked:restore_optional_flag(row,18)?});}native.checkpoint()?;
 let region_rows=regions.take(1,&mut native)?;let mut target_regions=native.allocate_vec::<Puzzle2dTargetRegion>(region_rows.len())?;native.begin_stage(region_rows.len())?;
 for row in region_rows{native.step()?;target_regions.push(Puzzle2dTargetRegion{id:native.copy_text(row.text(3)?)?,x:read_binary64(row,4,REGION)?,y:read_binary64(row,5,REGION)?,width:read_binary64(row,6,REGION)?,height:read_binary64(row,7,REGION)?,label:restore_optional_text(row,8,&mut native)?,hidden:flag(row,9)?,locked:flag(row,10)?});}native.checkpoint()?;
 let meta_row=&metas[0];let rule_rows=compatibility.take(meta_row.rowid,&mut native)?;let mut kind_compatibility=native.allocate_vec::<Puzzle2dKindCompatibility>(rule_rows.len())?;native.begin_stage(rule_rows.len())?;
 for row in rule_rows{native.step()?;kind_compatibility.push(Puzzle2dKindCompatibility{source:native.copy_text(row.text(3)?)?,target:native.copy_text(row.text(4)?)?,bidirectional:flag(row,5)?,important:flag(row,6)?,specificity:restore_specificity(row.text(7)?)?});}native.checkpoint()?;
 let manifest_id=restore_optional_text(meta_row,2,&mut native)?;

 let kind_catalogs=if let Some(catalog)=catalog_rows.first(){
  native.charge(std::mem::size_of::<Puzzle2dKindCatalogs>())?;let kind_rows=kinds.take(catalog.rowid,&mut native)?;let mut restored_kinds=native.allocate_vec::<Puzzle2dCatalogNodeKind>(kind_rows.len())?;
  for row in kind_rows{
   let base_rows=bases.take(row.rowid,&mut native)?;let mut base_kinds=native.allocate_vec::<String>(base_rows.len())?;native.begin_stage(base_rows.len())?;for member in base_rows{native.step()?;base_kinds.push(native.copy_text(member.text(3)?)?);}native.checkpoint()?;
   let representation_rows=representations.take(row.rowid,&mut native)?;let mut restored_representations=native.allocate_vec::<Puzzle2dRepresentation>(representation_rows.len())?;
   for member in representation_rows{
    let tag_rows=tags.take(member.rowid,&mut native)?;let mut restored_tags=native.allocate_vec::<String>(tag_rows.len())?;native.begin_stage(tag_rows.len())?;for tag in tag_rows{native.step()?;restored_tags.push(native.copy_text(tag.text(3)?)?);}native.checkpoint()?;
    restored_representations.push(Puzzle2dRepresentation{id:native.copy_text(member.text(3)?)?,name:native.copy_text(member.text(4)?)?,url:native.copy_text(member.text(5)?)?,mime:native.copy_text(member.text(6)?)?,tags:restored_tags,lod:restore_optional_text(member,7,&mut native)?,description:native.copy_text(member.text(8)?)?});native.checkpoint()?;
   }
   let template_rows=templates.take(row.rowid,&mut native)?;let mut restored_templates=native.allocate_vec::<Puzzle2dHandleTemplate>(template_rows.len())?;native.begin_stage(template_rows.len())?;
   for member in template_rows{native.step()?;restored_templates.push(Puzzle2dHandleTemplate{id:native.copy_text(member.text(3)?)?,name:native.copy_text(member.text(4)?)?,label:native.copy_text(member.text(5)?)?,description:native.copy_text(member.text(6)?)?,icon:native.copy_text(member.text(7)?)?,handle_kind:restore_optional_text(member,8,&mut native)?,angle:read_binary64(member,9,TEMPLATE)?,t:restore_optional_real(member,10,TEMPLATE)?,mandatory:restore_optional_flag(member,11)?,radius:restore_optional_real(member,12,TEMPLATE)?});}native.checkpoint()?;
   let attribute_rows=attributes.take(row.rowid,&mut native)?;let mut restored_attributes=native.allocate_vec::<Puzzle2dAttribute>(attribute_rows.len())?;native.begin_stage(attribute_rows.len())?;
   for member in attribute_rows{native.step()?;restored_attributes.push(Puzzle2dAttribute{id:native.copy_text(member.text(3)?)?,key:native.copy_text(member.text(4)?)?,value:native.copy_text(member.text(5)?)?,definition:restore_optional_text(member,6,&mut native)?});}native.checkpoint()?;
   let author_rows=authors.take(row.rowid,&mut native)?;let mut restored_authors=native.allocate_vec::<Puzzle2dAuthor>(author_rows.len())?;native.begin_stage(author_rows.len())?;
   for member in author_rows{native.step()?;restored_authors.push(Puzzle2dAuthor{id:native.copy_text(member.text(3)?)?,name:native.copy_text(member.text(4)?)?,email:native.copy_text(member.text(5)?)?,role:restore_optional_text(member,6,&mut native)?,rank:restore_optional_i32(member,7)?});}native.checkpoint()?;
   restored_kinds.push(Puzzle2dCatalogNodeKind{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,label:native.copy_text(row.text(5)?)?,description:native.copy_text(row.text(6)?)?,icon:native.copy_text(row.text(7)?)?,image:native.copy_text(row.text(8)?)?,unit:native.copy_text(row.text(9)?)?,is_abstract:flag(row,10)?,base_kinds,representations:restored_representations,handles:restored_templates,attributes:restored_attributes,authors:restored_authors});native.checkpoint()?;
  }
  let handle_rows=handle_kinds.take(catalog.rowid,&mut native)?;let mut restored_handles=native.allocate_vec::<Puzzle2dCatalogHandleKind>(handle_rows.len())?;
  for row in handle_rows{
   let compatible_rows=compatible_kinds.take(row.rowid,&mut native)?;let mut compatible_with=native.allocate_vec::<String>(compatible_rows.len())?;native.begin_stage(compatible_rows.len())?;for member in compatible_rows{native.step()?;compatible_with.push(native.copy_text(member.text(3)?)?);}native.checkpoint()?;
   restored_handles.push(Puzzle2dCatalogHandleKind{id:native.copy_text(row.text(3)?)?,code:restore_optional_text(row,4,&mut native)?,label:restore_optional_text(row,5,&mut native)?,order:restore_optional_i32(row,6)?,compatible_with,description:native.copy_text(row.text(7)?)?,icon:native.copy_text(row.text(8)?)?,color:native.copy_text(row.text(9)?)?,default_wire_kind:native.copy_text(row.text(10)?)?});native.checkpoint()?;
  }
  let edge_rows=edge_kinds.take(catalog.rowid,&mut native)?;let mut restored_edges=native.allocate_vec::<Puzzle2dCatalogEdgeKind>(edge_rows.len())?;native.begin_stage(edge_rows.len())?;
  for row in edge_rows{native.step()?;restored_edges.push(Puzzle2dCatalogEdgeKind{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,label:native.copy_text(row.text(5)?)?,description:native.copy_text(row.text(6)?)?,icon:native.copy_text(row.text(7)?)?,color:native.copy_text(row.text(8)?)?});}native.checkpoint()?;
  let wire_rows=wire_kinds.take(catalog.rowid,&mut native)?;let mut restored_wires=native.allocate_vec::<Puzzle2dCatalogWireKind>(wire_rows.len())?;native.begin_stage(wire_rows.len())?;
  for row in wire_rows{native.step()?;restored_wires.push(Puzzle2dCatalogWireKind{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,label:native.copy_text(row.text(5)?)?,description:native.copy_text(row.text(6)?)?,icon:native.copy_text(row.text(7)?)?,color:native.copy_text(row.text(8)?)?,default_edge_kind:native.copy_text(row.text(9)?)?});}native.checkpoint()?;
  Some(Puzzle2dKindCatalogs{nodes:restored_kinds,handles:restored_handles,edges:restored_edges,wires:restored_wires})
 }else{None};
 for group in[&nodes,&handles,&edges,&regions,&compatibility,&kinds,&bases,&representations,&tags,&templates,&attributes,&authors,&handle_kinds,&compatible_kinds,&edge_kinds,&wire_kinds]{group.finish()?;}native.checkpoint()?;
 Ok(Puzzle2dSnapshot{schema,camera,nodes:restored_nodes,edges:restored_edges,target_regions,meta:Puzzle2dMeta{manifest_id,kind_compatibility,kind_catalogs}})
}

fn native_list(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<&[semio_framework_dsl_record::FieldValue],semio_framework_value::ValueError>{match value{None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Puzzle 2D requires an ordered native list").to_string()))}}
fn native_record(value:&semio_framework_dsl_record::FieldValue)->Result<&semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(record)=>Ok(record),semio_framework_dsl_record::FieldValue::Block(value)=>match value.as_ref(){semio_framework_dsl_record::FieldValue::Record(record)=>Ok(record),_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Puzzle 2D block requires its literal record").to_string()))},_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Puzzle 2D requires a literal native record").to_string()))}}
fn native_optional_record(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<Option<&semio_framework_dsl_record::RecordValue>,ValueError>{match value{None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(None),Some(value)=>native_record(value).map(Some)}}
fn native_rows(record:&semio_framework_dsl_record::RecordValue,native:&mut semio_framework_value::NativeDecodeControl<'_>,maximum:usize)->Result<(),semio_framework_value::ValueError>{
 let mut total=3usize;let mut add=|count:usize|->Result<(),semio_framework_value::ValueError>{total=total.checked_add(count).filter(|total|*total<=maximum).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Puzzle 2D native ownership exceeds row limit").to_string()))?;Ok(())};
 let nodes=native_list(record.get(2))?;add(nodes.len())?;add(native_list(record.get(3))?.len())?;add(native_list(record.get(4))?.len())?;
 native.scoped_stage(|native|->Result<(),semio_framework_value::ValueError>{native.begin_stage(nodes.len())?;for row in nodes{native.step()?;add(native_list(native_record(row)?.get(15))?.len())?;}Ok(())})?;
 if let Some(meta)=native_optional_record(record.get(5))?{
  add(native_list(meta.get(1))?.len())?;
  if let Some(catalogs)=native_optional_record(meta.get(2))?{
   add(1)?;let kinds=native_list(catalogs.get(0))?;let handles=native_list(catalogs.get(1))?;add(kinds.len())?;add(handles.len())?;add(native_list(catalogs.get(2))?.len())?;add(native_list(catalogs.get(3))?.len())?;
   native.scoped_stage(|native|->Result<(),semio_framework_value::ValueError>{native.begin_stage(kinds.len())?;for value in kinds{native.step()?;let kind=native_record(value)?;for field in[8,9,10,11,12]{add(native_list(kind.get(field))?.len())?;}
    let representations=native_list(kind.get(9))?;native.scoped_stage(|native|->Result<(),semio_framework_value::ValueError>{native.begin_stage(representations.len())?;for value in representations{native.step()?;add(native_list(native_record(value)?.get(4))?.len())?;}Ok(())})?;
   }Ok(())})?;
   native.scoped_stage(|native|->Result<(),semio_framework_value::ValueError>{native.begin_stage(handles.len())?;for value in handles{native.step()?;add(native_list(native_record(value)?.get(4))?.len())?;}Ok(())})?;
  }
 }Ok(())
}
impl ArtifactSqliteSnapshot for Puzzle2dSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let rows=workload(self,&mut |units|control.checkpoint(SqliteSnapshotPhase::EncodeNative,units,0))?;control.check_rows(rows)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn retire_sqlite_snapshot(self){drop(self)}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{restore(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_rows(3)?;let maximum=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{native_rows(record,native,maximum)?;Self::__dsl_from_record_controlled(record,native)},control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  let mut bound=NativeEncodingBound::new(control)?;let rows=workload(self,&mut |_|bound.checkpoint())?;bound.check_rows(rows)?;bound.add(1024)?;bound.repeated(rows,1024)?;bound_strings(self,&mut bound)?;bound.finish()
 }
}

/// 🎮️ Persists the editor's typed authority through the declared board schema.
impl ArtifactSqliteSnapshot for crate::Puzzle2dPlaySnapshot{
 const SQLITE_SCHEMA:&'static str=<Puzzle2dSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA;
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.typed().to_sqlite_database(control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let bytes=std::mem::size_of::<Puzzle2dSnapshot>().checked_add(2*std::mem::size_of::<usize>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Play authority allocation overflow"))?;
  restore_authority(database,control,bytes).map(Self::from_typed)
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_rows(3)?;let maximum=control.limits().max_rows;
  store::decode_sqlite_snapshot_record_native(payload,<Puzzle2dSnapshot as store::ArtifactDsl>::envelope_id(),Puzzle2dSnapshot::__dsl_spec_producer(),|record,native|{
   native_rows(record,native,maximum)?;
   native.charge(std::mem::size_of::<Puzzle2dSnapshot>()+2*std::mem::size_of::<usize>())?;
   Puzzle2dSnapshot::__dsl_from_record_controlled(record,native).map(Self::from_typed)
  },control)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{self.typed().encode_sqlite_snapshot_native(encoding,control)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{self.typed().validate_sqlite_snapshot_subset(dialect,database,control)}
}
