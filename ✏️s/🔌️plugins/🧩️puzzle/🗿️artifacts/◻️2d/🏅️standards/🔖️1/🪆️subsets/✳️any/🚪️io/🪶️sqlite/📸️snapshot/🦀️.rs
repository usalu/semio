//! 🧩️ Handwritten board, ordered catalog and exact optional scalar ownership.
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use crate::{Puzzle2dCamera,Puzzle2dNode,Puzzle2dNodeAnchor,Puzzle2dHandle,Puzzle2dEdge,Puzzle2dTargetRegion,Puzzle2dMeta,Puzzle2dKindCompatibility,Puzzle2dCompatSpecificity,Puzzle2dKindCatalogs,Puzzle2dCatalogNodeKind,Puzzle2dRepresentation,Puzzle2dHandleTemplate,Puzzle2dAttribute,Puzzle2dAuthor,Puzzle2dCatalogHandleKind,Puzzle2dCatalogEdgeKind,Puzzle2dCatalogWireKind};
use std::collections::{BTreeMap,BTreeSet};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{RowWriter,Cell,FloatColumn,read_binary64,ieee754_is_null}}};
use semio_framework_value::{ValueError,ValueRefusalKind,list::PagedList,paged::PagedUtf8};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

const CAMERA:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const NODE:&[FloatColumn]=&[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(14)];
const HANDLE:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(9)];
const EDGE:&[FloatColumn]=&[FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14)];
const REGION:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const TEMPLATE:&[FloatColumn]=&[FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(12)];
fn optional_text(value:&Option<PagedUtf8<{usize::MAX}>>)->Cell<'_>{value.as_ref().map_or(Cell::Null,|value|Cell::PagedText(value))}
fn allocate_collection<T>(length:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<PagedList<T,{usize::MAX}>,ValueError>{
 let mut output=PagedList::new();while let Some(bytes)=output.next_capacity_allocation_bytes(length)?{control.checkpoint()?;control.charge(bytes)?;let progress=output.reserve_capacity_one(length,bytes).map_err(|error|ValueError::from(error.refusal()))?;if !progress.progressed{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Puzzle paged collection admission did not progress"))}}Ok(output)
}
fn optional_real(value:Option<f64>)->Cell<'static>{value.map_or(Cell::Null,Cell::Real)}
fn optional_flag(value:Option<bool>)->Cell<'static>{value.map_or(Cell::Null,|value|Cell::Integer(i64::from(value)))}
fn optional_i32(value:Option<i32>)->Cell<'static>{value.map_or(Cell::Null,|value|Cell::Integer(i64::from(value)))}
fn ordinal(value:usize)->Result<Cell<'static>,ValueError>{Ok(Cell::Integer(i64::try_from(value).map_err(|e|invalid(e.to_string()))?))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid{return Err(invalid("Puzzle 2D requires complete positive aliased entities"))}Ok(())}
fn flag(row:&SqliteRow,index:usize)->Result<bool,ValueError>{match row.integer(index)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("Puzzle 2D flag exceeds Boolean domain"))}}
fn restore_optional_flag(row:&SqliteRow,index:usize)->Result<Option<bool>,ValueError>{if matches!(row.values.get(index),Some(SqliteValue::Null)){Ok(None)}else{flag(row,index).map(Some)}}
fn restore_optional_i32(row:&SqliteRow,index:usize)->Result<Option<i32>,ValueError>{if matches!(row.values.get(index),Some(SqliteValue::Null)){Ok(None)}else{Ok(Some(i32::try_from(row.integer(index)?).map_err(|e|invalid(e.to_string()))?))}}
fn restore_optional_real(row:&SqliteRow,index:usize,columns:&[FloatColumn])->Result<Option<f64>,ValueError>{if ieee754_is_null(row,index,columns)?{Ok(None)}else{read_binary64(row,index,columns).map(Some)}}
fn restore_optional_text(row:&SqliteRow,index:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<PagedUtf8<{usize::MAX}>>,ValueError>{match row.values.get(index){Some(SqliteValue::Null)=>Ok(None),Some(SqliteValue::Text(value))=>PagedUtf8::try_from_str_controlled(value,control).map(Some),_=>Err(invalid("Puzzle 2D optional TEXT has another storage class"))}}
fn specificity(value:Puzzle2dCompatSpecificity)->&'static str{match value{Puzzle2dCompatSpecificity::General=>"general",Puzzle2dCompatSpecificity::Node=>"node",Puzzle2dCompatSpecificity::Edge=>"edge",Puzzle2dCompatSpecificity::Handle=>"handle",Puzzle2dCompatSpecificity::Wire=>"wire",Puzzle2dCompatSpecificity::Vortex=>"vortex"}}
fn restore_specificity(value:&str)->Result<Puzzle2dCompatSpecificity,ValueError>{match value{"general"=>Ok(Puzzle2dCompatSpecificity::General),"node"=>Ok(Puzzle2dCompatSpecificity::Node),"edge"=>Ok(Puzzle2dCompatSpecificity::Edge),"handle"=>Ok(Puzzle2dCompatSpecificity::Handle),"wire"=>Ok(Puzzle2dCompatSpecificity::Wire),"vortex"=>Ok(Puzzle2dCompatSpecificity::Vortex),_=>Err(invalid("Puzzle 2D compatibility specificity is undeclared"))}}
fn workload(snapshot:&Puzzle2dSnapshot,checkpoint:&mut impl FnMut(usize)->Result<(),ValueError>)->Result<usize,ValueError>{
 let mut rows=3usize;let mut units=0usize;let mut add=|count:usize|->Result<(),ValueError>{rows=rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle 2D row workload overflow"))?;units+=1;if units%256==0{checkpoint(units)?}Ok(())};
 add(snapshot.nodes.len())?;add(snapshot.edges.len())?;add(snapshot.target_regions.len())?;add(snapshot.meta.kind_compatibility.len())?;for node in &snapshot.nodes{add(node.handles.len())?;}
 if let Some(catalogs)=&snapshot.meta.kind_catalogs{add(1)?;add(catalogs.nodes.len())?;add(catalogs.handles.len())?;add(catalogs.edges.len())?;add(catalogs.wires.len())?;for node in &catalogs.nodes{add(node.base_kinds.len())?;add(node.representations.len())?;add(node.handles.len())?;add(node.attributes.len())?;add(node.authors.len())?;for representation in &node.representations{add(representation.tags.len())?;}}for handle in &catalogs.handles{add(handle.compatible_with.len())?;}}
 checkpoint(units)?;Ok(rows)
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

fn put(out:&mut RowWriter<'_,'_>,completed:&mut usize,total:usize,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
 let key=if columns.is_empty(){out.insert(table,cells)?}else{out.insert_float(table,cells,columns)?};*completed+=1;if *completed%256==0{out.checkpoint_total(total)?}Ok(key)
}
fn visit_rows(snapshot:&Puzzle2dSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let total=workload(snapshot,&mut |_|out.checkpoint())?;out.check_rows(total)?;out.checkpoint_total(total)?;let mut completed=0usize;
 put(out,&mut completed,total,"puzzle2d_document",&[Cell::PagedText(&snapshot.schema)],&[])?;
 let camera=&snapshot.camera;put(out,&mut completed,total,"puzzle2d_camera",&[Cell::Integer(1),Cell::Real(camera.x),Cell::Real(camera.y),Cell::Real(camera.zoom)],CAMERA)?;
 let meta=put(out,&mut completed,total,"puzzle2d_meta",&[Cell::Integer(1),optional_text(&snapshot.meta.manifest_id)],&[])?;
 for(i,node)in snapshot.nodes.iter().enumerate(){
  let key=put(out,&mut completed,total,"puzzle2d_node",&[Cell::Integer(1),ordinal(i)?,Cell::PagedText(&node.id),optional_text(&node.node_kind),optional_text(&node.shape),Cell::Real(node.x),Cell::Real(node.y),optional_real(node.radius),optional_real(node.width),optional_real(node.height),optional_text(&node.text),optional_text(&node.icon_kind),optional_flag(node.root),optional_real(node.scale),optional_flag(node.visible),optional_flag(node.locked),Cell::Text(match node.anchor{Puzzle2dNodeAnchor::Fixed=>"fixed",Puzzle2dNodeAnchor::Derived=>"derived"})],NODE)?;
  for(i,handle)in node.handles.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_handle",&[Cell::Integer(key),ordinal(i)?,Cell::PagedText(&handle.id),optional_text(&handle.handle_kind),Cell::Real(handle.angle),optional_real(handle.radius),optional_text(&handle.color),optional_text(&handle.icon_kind),optional_real(handle.scale),optional_flag(handle.visible),optional_flag(handle.locked)],HANDLE)?;}
 }
 for(i,edge)in snapshot.edges.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_edge",&[Cell::Integer(1),ordinal(i)?,Cell::PagedText(&edge.id),Cell::PagedText(&edge.source),Cell::PagedText(&edge.target),optional_text(&edge.edge_kind),Cell::Real(edge.gap),Cell::Real(edge.shift),Cell::Real(edge.rise),Cell::Real(edge.rotation),Cell::Real(edge.turn),Cell::Real(edge.tilt),Cell::Real(edge.x),Cell::Real(edge.y),optional_text(&edge.source_tip),optional_text(&edge.target_tip),optional_flag(edge.visible),optional_flag(edge.locked)],EDGE)?;}
 for(i,region)in snapshot.target_regions.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_target_region",&[Cell::Integer(1),ordinal(i)?,Cell::PagedText(&region.id),Cell::Real(region.x),Cell::Real(region.y),Cell::Real(region.width),Cell::Real(region.height),optional_text(&region.label),Cell::Integer(i64::from(region.hidden)),Cell::Integer(i64::from(region.locked))],REGION)?;}
 for(i,rule)in snapshot.meta.kind_compatibility.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_kind_compatibility",&[Cell::Integer(meta),ordinal(i)?,Cell::PagedText(&rule.source),Cell::PagedText(&rule.target),Cell::Integer(i64::from(rule.bidirectional)),Cell::Integer(i64::from(rule.important)),Cell::Text(specificity(rule.specificity))],&[])?;}
 if let Some(catalogs)=&snapshot.meta.kind_catalogs{
  let key=put(out,&mut completed,total,"puzzle2d_kind_catalogs",&[Cell::Integer(meta)],&[])?;
  for(i,node)in catalogs.nodes.iter().enumerate(){
   let node_key=put(out,&mut completed,total,"puzzle2d_catalog_node_kind",&[Cell::Integer(key),ordinal(i)?,Cell::PagedText(&node.id),Cell::PagedText(&node.name),Cell::PagedText(&node.label),Cell::PagedText(&node.description),Cell::PagedText(&node.icon),Cell::PagedText(&node.image),Cell::PagedText(&node.unit),Cell::Integer(i64::from(node.is_abstract))],&[])?;
   for(i,base)in node.base_kinds.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_base_kind",&[Cell::Integer(node_key),ordinal(i)?,Cell::PagedText(base)],&[])?;}
   for(i,representation)in node.representations.iter().enumerate(){
    let representation_key=put(out,&mut completed,total,"puzzle2d_representation",&[Cell::Integer(node_key),ordinal(i)?,Cell::PagedText(&representation.id),Cell::PagedText(&representation.name),Cell::PagedText(&representation.url),Cell::PagedText(&representation.mime),optional_text(&representation.lod),Cell::PagedText(&representation.description)],&[])?;
    for(i,tag)in representation.tags.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_representation_tag",&[Cell::Integer(representation_key),ordinal(i)?,Cell::PagedText(tag)],&[])?;}
   }
   for(i,handle)in node.handles.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_handle_template",&[Cell::Integer(node_key),ordinal(i)?,Cell::PagedText(&handle.id),Cell::PagedText(&handle.name),Cell::PagedText(&handle.label),Cell::PagedText(&handle.description),Cell::PagedText(&handle.icon),optional_text(&handle.handle_kind),Cell::Real(handle.angle),optional_real(handle.t),optional_flag(handle.mandatory),optional_real(handle.radius)],TEMPLATE)?;}
   for(i,attribute)in node.attributes.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_attribute",&[Cell::Integer(node_key),ordinal(i)?,Cell::PagedText(&attribute.id),Cell::PagedText(&attribute.key),Cell::PagedText(&attribute.value),optional_text(&attribute.definition)],&[])?;}
   for(i,author)in node.authors.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_author",&[Cell::Integer(node_key),ordinal(i)?,Cell::PagedText(&author.id),Cell::PagedText(&author.name),Cell::PagedText(&author.email),optional_text(&author.role),optional_i32(author.rank)],&[])?;}
  }
  for(i,handle)in catalogs.handles.iter().enumerate(){
   let handle_key=put(out,&mut completed,total,"puzzle2d_catalog_handle_kind",&[Cell::Integer(key),ordinal(i)?,Cell::PagedText(&handle.id),optional_text(&handle.code),optional_text(&handle.label),optional_i32(handle.order),Cell::PagedText(&handle.description),Cell::PagedText(&handle.icon),Cell::PagedText(&handle.color),Cell::PagedText(&handle.default_wire_kind)],&[])?;
   for(i,kind)in handle.compatible_with.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_compatible_kind",&[Cell::Integer(handle_key),ordinal(i)?,Cell::PagedText(kind)],&[])?;}
  }
  for(i,edge)in catalogs.edges.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_catalog_edge_kind",&[Cell::Integer(key),ordinal(i)?,Cell::PagedText(&edge.id),Cell::PagedText(&edge.name),Cell::PagedText(&edge.label),Cell::PagedText(&edge.description),Cell::PagedText(&edge.icon),Cell::PagedText(&edge.color)],&[])?;}
  for(i,wire)in catalogs.wires.iter().enumerate(){put(out,&mut completed,total,"puzzle2d_catalog_wire_kind",&[Cell::Integer(key),ordinal(i)?,Cell::PagedText(&wire.id),Cell::PagedText(&wire.name),Cell::PagedText(&wire.label),Cell::PagedText(&wire.description),Cell::PagedText(&wire.icon),Cell::PagedText(&wire.color),Cell::PagedText(&wire.default_edge_kind)],&[])?;}
 }
 if completed!=total{return Err(invalid("Puzzle 2D projection differs from known owned workload"))}out.checkpoint_total(total)?;Ok(())
}

fn project(snapshot:&Puzzle2dSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut out=RowWriter::new(Puzzle2dSnapshot::SQLITE_SCHEMA,control)?;visit_rows(snapshot,&mut out)?;out.finish()}
fn admit_typed(snapshot:&Puzzle2dSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic_cells::extent(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}

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
 let schema=PagedUtf8::try_from_str_controlled(documents[0].text(1)?,&mut native)?;let c=&cameras[0];let camera=Puzzle2dCamera{x:read_binary64(c,2,CAMERA)?,y:read_binary64(c,3,CAMERA)?,zoom:read_binary64(c,4,CAMERA)?};
 let node_rows=nodes.take(1,&mut native)?;let mut restored_nodes=allocate_collection::<Puzzle2dNode>(node_rows.len(),&mut native)?;
 for row in node_rows{
  let handle_rows=handles.take(row.rowid,&mut native)?;let mut restored_handles=allocate_collection::<Puzzle2dHandle>(handle_rows.len(),&mut native)?;native.begin_stage(handle_rows.len())?;
  for h in handle_rows{native.step()?;restored_handles.push(Puzzle2dHandle{id:PagedUtf8::try_from_str_controlled(h.text(3)?,&mut native)?,handle_kind:restore_optional_text(h,4,&mut native)?,angle:read_binary64(h,5,HANDLE)?,radius:restore_optional_real(h,6,HANDLE)?,color:restore_optional_text(h,7,&mut native)?,icon_kind:restore_optional_text(h,8,&mut native)?,scale:restore_optional_real(h,9,HANDLE)?,visible:restore_optional_flag(h,10)?,locked:restore_optional_flag(h,11)?});}native.checkpoint()?;
  restored_nodes.push(Puzzle2dNode{id:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,node_kind:restore_optional_text(row,4,&mut native)?,shape:restore_optional_text(row,5,&mut native)?,x:read_binary64(row,6,NODE)?,y:read_binary64(row,7,NODE)?,radius:restore_optional_real(row,8,NODE)?,width:restore_optional_real(row,9,NODE)?,height:restore_optional_real(row,10,NODE)?,text:restore_optional_text(row,11,&mut native)?,icon_kind:restore_optional_text(row,12,&mut native)?,root:restore_optional_flag(row,13)?,scale:restore_optional_real(row,14,NODE)?,visible:restore_optional_flag(row,15)?,locked:restore_optional_flag(row,16)?,anchor:match row.text(17)?{"fixed"=>Puzzle2dNodeAnchor::Fixed,"derived"=>Puzzle2dNodeAnchor::Derived,_=>return Err(invalid("Puzzle 2D node anchor is undeclared"))},handles:restored_handles});native.checkpoint()?;
 }
 let edge_rows=edges.take(1,&mut native)?;let mut restored_edges=allocate_collection::<Puzzle2dEdge>(edge_rows.len(),&mut native)?;native.begin_stage(edge_rows.len())?;
 for row in edge_rows{native.step()?;restored_edges.push(Puzzle2dEdge{id:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,source:PagedUtf8::try_from_str_controlled(row.text(4)?,&mut native)?,target:PagedUtf8::try_from_str_controlled(row.text(5)?,&mut native)?,edge_kind:restore_optional_text(row,6,&mut native)?,gap:read_binary64(row,7,EDGE)?,shift:read_binary64(row,8,EDGE)?,rise:read_binary64(row,9,EDGE)?,rotation:read_binary64(row,10,EDGE)?,turn:read_binary64(row,11,EDGE)?,tilt:read_binary64(row,12,EDGE)?,x:read_binary64(row,13,EDGE)?,y:read_binary64(row,14,EDGE)?,source_tip:restore_optional_text(row,15,&mut native)?,target_tip:restore_optional_text(row,16,&mut native)?,visible:restore_optional_flag(row,17)?,locked:restore_optional_flag(row,18)?});}native.checkpoint()?;
 let region_rows=regions.take(1,&mut native)?;let mut target_regions=allocate_collection::<Puzzle2dTargetRegion>(region_rows.len(),&mut native)?;native.begin_stage(region_rows.len())?;
 for row in region_rows{native.step()?;target_regions.push(Puzzle2dTargetRegion{id:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,x:read_binary64(row,4,REGION)?,y:read_binary64(row,5,REGION)?,width:read_binary64(row,6,REGION)?,height:read_binary64(row,7,REGION)?,label:restore_optional_text(row,8,&mut native)?,hidden:flag(row,9)?,locked:flag(row,10)?});}native.checkpoint()?;
 let meta_row=&metas[0];let rule_rows=compatibility.take(meta_row.rowid,&mut native)?;let mut kind_compatibility=allocate_collection::<Puzzle2dKindCompatibility>(rule_rows.len(),&mut native)?;native.begin_stage(rule_rows.len())?;
 for row in rule_rows{native.step()?;kind_compatibility.push(Puzzle2dKindCompatibility{source:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,target:PagedUtf8::try_from_str_controlled(row.text(4)?,&mut native)?,bidirectional:flag(row,5)?,important:flag(row,6)?,specificity:restore_specificity(row.text(7)?)?});}native.checkpoint()?;
 let manifest_id=restore_optional_text(meta_row,2,&mut native)?;

 let kind_catalogs=if let Some(catalog)=catalog_rows.first(){
  native.charge(std::mem::size_of::<Puzzle2dKindCatalogs>())?;let kind_rows=kinds.take(catalog.rowid,&mut native)?;let mut restored_kinds=allocate_collection::<Puzzle2dCatalogNodeKind>(kind_rows.len(),&mut native)?;
  for row in kind_rows{
   let base_rows=bases.take(row.rowid,&mut native)?;let mut base_kinds=allocate_collection::<PagedUtf8<{usize::MAX}>>(base_rows.len(),&mut native)?;native.begin_stage(base_rows.len())?;for member in base_rows{native.step()?;base_kinds.push(PagedUtf8::try_from_str_controlled(member.text(3)?,&mut native)?);}native.checkpoint()?;
   let representation_rows=representations.take(row.rowid,&mut native)?;let mut restored_representations=allocate_collection::<Puzzle2dRepresentation>(representation_rows.len(),&mut native)?;
   for member in representation_rows{
    let tag_rows=tags.take(member.rowid,&mut native)?;let mut restored_tags=allocate_collection::<PagedUtf8<{usize::MAX}>>(tag_rows.len(),&mut native)?;native.begin_stage(tag_rows.len())?;for tag in tag_rows{native.step()?;restored_tags.push(PagedUtf8::try_from_str_controlled(tag.text(3)?,&mut native)?);}native.checkpoint()?;
    restored_representations.push(Puzzle2dRepresentation{id:PagedUtf8::try_from_str_controlled(member.text(3)?,&mut native)?,name:PagedUtf8::try_from_str_controlled(member.text(4)?,&mut native)?,url:PagedUtf8::try_from_str_controlled(member.text(5)?,&mut native)?,mime:PagedUtf8::try_from_str_controlled(member.text(6)?,&mut native)?,tags:restored_tags,lod:restore_optional_text(member,7,&mut native)?,description:PagedUtf8::try_from_str_controlled(member.text(8)?,&mut native)?});native.checkpoint()?;
   }
   let template_rows=templates.take(row.rowid,&mut native)?;let mut restored_templates=allocate_collection::<Puzzle2dHandleTemplate>(template_rows.len(),&mut native)?;native.begin_stage(template_rows.len())?;
   for member in template_rows{native.step()?;restored_templates.push(Puzzle2dHandleTemplate{id:PagedUtf8::try_from_str_controlled(member.text(3)?,&mut native)?,name:PagedUtf8::try_from_str_controlled(member.text(4)?,&mut native)?,label:PagedUtf8::try_from_str_controlled(member.text(5)?,&mut native)?,description:PagedUtf8::try_from_str_controlled(member.text(6)?,&mut native)?,icon:PagedUtf8::try_from_str_controlled(member.text(7)?,&mut native)?,handle_kind:restore_optional_text(member,8,&mut native)?,angle:read_binary64(member,9,TEMPLATE)?,t:restore_optional_real(member,10,TEMPLATE)?,mandatory:restore_optional_flag(member,11)?,radius:restore_optional_real(member,12,TEMPLATE)?});}native.checkpoint()?;
   let attribute_rows=attributes.take(row.rowid,&mut native)?;let mut restored_attributes=allocate_collection::<Puzzle2dAttribute>(attribute_rows.len(),&mut native)?;native.begin_stage(attribute_rows.len())?;
   for member in attribute_rows{native.step()?;restored_attributes.push(Puzzle2dAttribute{id:PagedUtf8::try_from_str_controlled(member.text(3)?,&mut native)?,key:PagedUtf8::try_from_str_controlled(member.text(4)?,&mut native)?,value:PagedUtf8::try_from_str_controlled(member.text(5)?,&mut native)?,definition:restore_optional_text(member,6,&mut native)?});}native.checkpoint()?;
   let author_rows=authors.take(row.rowid,&mut native)?;let mut restored_authors=allocate_collection::<Puzzle2dAuthor>(author_rows.len(),&mut native)?;native.begin_stage(author_rows.len())?;
   for member in author_rows{native.step()?;restored_authors.push(Puzzle2dAuthor{id:PagedUtf8::try_from_str_controlled(member.text(3)?,&mut native)?,name:PagedUtf8::try_from_str_controlled(member.text(4)?,&mut native)?,email:PagedUtf8::try_from_str_controlled(member.text(5)?,&mut native)?,role:restore_optional_text(member,6,&mut native)?,rank:restore_optional_i32(member,7)?});}native.checkpoint()?;
   restored_kinds.push(Puzzle2dCatalogNodeKind{id:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,name:PagedUtf8::try_from_str_controlled(row.text(4)?,&mut native)?,label:PagedUtf8::try_from_str_controlled(row.text(5)?,&mut native)?,description:PagedUtf8::try_from_str_controlled(row.text(6)?,&mut native)?,icon:PagedUtf8::try_from_str_controlled(row.text(7)?,&mut native)?,image:PagedUtf8::try_from_str_controlled(row.text(8)?,&mut native)?,unit:PagedUtf8::try_from_str_controlled(row.text(9)?,&mut native)?,is_abstract:flag(row,10)?,base_kinds,representations:restored_representations,handles:restored_templates,attributes:restored_attributes,authors:restored_authors});native.checkpoint()?;
  }
  let handle_rows=handle_kinds.take(catalog.rowid,&mut native)?;let mut restored_handles=allocate_collection::<Puzzle2dCatalogHandleKind>(handle_rows.len(),&mut native)?;
  for row in handle_rows{
   let compatible_rows=compatible_kinds.take(row.rowid,&mut native)?;let mut compatible_with=allocate_collection::<PagedUtf8<{usize::MAX}>>(compatible_rows.len(),&mut native)?;native.begin_stage(compatible_rows.len())?;for member in compatible_rows{native.step()?;compatible_with.push(PagedUtf8::try_from_str_controlled(member.text(3)?,&mut native)?);}native.checkpoint()?;
   restored_handles.push(Puzzle2dCatalogHandleKind{id:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,code:restore_optional_text(row,4,&mut native)?,label:restore_optional_text(row,5,&mut native)?,order:restore_optional_i32(row,6)?,compatible_with,description:PagedUtf8::try_from_str_controlled(row.text(7)?,&mut native)?,icon:PagedUtf8::try_from_str_controlled(row.text(8)?,&mut native)?,color:PagedUtf8::try_from_str_controlled(row.text(9)?,&mut native)?,default_wire_kind:PagedUtf8::try_from_str_controlled(row.text(10)?,&mut native)?});native.checkpoint()?;
  }
  let edge_rows=edge_kinds.take(catalog.rowid,&mut native)?;let mut restored_edges=allocate_collection::<Puzzle2dCatalogEdgeKind>(edge_rows.len(),&mut native)?;native.begin_stage(edge_rows.len())?;
  for row in edge_rows{native.step()?;restored_edges.push(Puzzle2dCatalogEdgeKind{id:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,name:PagedUtf8::try_from_str_controlled(row.text(4)?,&mut native)?,label:PagedUtf8::try_from_str_controlled(row.text(5)?,&mut native)?,description:PagedUtf8::try_from_str_controlled(row.text(6)?,&mut native)?,icon:PagedUtf8::try_from_str_controlled(row.text(7)?,&mut native)?,color:PagedUtf8::try_from_str_controlled(row.text(8)?,&mut native)?});}native.checkpoint()?;
  let wire_rows=wire_kinds.take(catalog.rowid,&mut native)?;let mut restored_wires=allocate_collection::<Puzzle2dCatalogWireKind>(wire_rows.len(),&mut native)?;native.begin_stage(wire_rows.len())?;
  for row in wire_rows{native.step()?;restored_wires.push(Puzzle2dCatalogWireKind{id:PagedUtf8::try_from_str_controlled(row.text(3)?,&mut native)?,name:PagedUtf8::try_from_str_controlled(row.text(4)?,&mut native)?,label:PagedUtf8::try_from_str_controlled(row.text(5)?,&mut native)?,description:PagedUtf8::try_from_str_controlled(row.text(6)?,&mut native)?,icon:PagedUtf8::try_from_str_controlled(row.text(7)?,&mut native)?,color:PagedUtf8::try_from_str_controlled(row.text(8)?,&mut native)?,default_edge_kind:PagedUtf8::try_from_str_controlled(row.text(9)?,&mut native)?});}native.checkpoint()?;
  Some(Puzzle2dKindCatalogs{nodes:restored_kinds,handles:restored_handles,edges:restored_edges,wires:restored_wires})
 }else{None};
 for group in[&nodes,&handles,&edges,&regions,&compatibility,&kinds,&bases,&representations,&tags,&templates,&attributes,&authors,&handle_kinds,&compatible_kinds,&edge_kinds,&wire_kinds]{group.finish()?;}native.checkpoint()?;
 Ok(Puzzle2dSnapshot{schema,camera,nodes:restored_nodes,edges:restored_edges,target_regions,meta:Puzzle2dMeta{manifest_id,kind_compatibility,kind_catalogs}})
}

#[path="📏️cells/🦀️.rs"]
mod semantic_cells;
impl ArtifactSqliteSnapshot for Puzzle2dSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{
  admit_typed(self,SqliteSnapshotPhase::EncodeNative,control)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
 }
 fn retire_sqlite_snapshot(self){drop(self)}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{restore(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{
  let limits=control.limits();semantic_cells::extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {semantic_cells::admit_record(record,limits,native)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  admit_typed(self,SqliteSnapshotPhase::EncodeNative,control)
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
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{
  let limits=control.limits();semantic_cells::extent(limits)?;
  store::decode_sqlite_snapshot_record_native(payload,<Puzzle2dSnapshot as store::ArtifactDsl>::envelope_id(),Puzzle2dSnapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {
   semantic_cells::admit_record(record,limits,native)?;
   native.charge(std::mem::size_of::<Puzzle2dSnapshot>()+2*std::mem::size_of::<usize>())?;
   Puzzle2dSnapshot::__dsl_from_record_controlled(record,native).map(Self::from_typed)
  })(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.typed().preflight_sqlite_snapshot_encoding(encoding,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{self.typed().encode_sqlite_snapshot_native(encoding,control,native_owner)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{self.typed().validate_sqlite_snapshot_subset(dialect,database,control)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
