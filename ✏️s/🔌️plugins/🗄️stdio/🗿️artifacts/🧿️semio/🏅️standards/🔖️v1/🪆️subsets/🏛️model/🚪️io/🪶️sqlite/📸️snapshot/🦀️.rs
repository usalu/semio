//! 🏛️ Spatial hierarchy, physical elements, geometry identities and typed property relationships.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,RowIndex};
use crate::standards::v1::subsets::model::schema::snapshot::{SemioModelSnapshot,SpatialNode,SpatialKind,SemioModelElement,ElementClass,GeometryRef,PropertySet,Property,PsetValue,ModelRelation,RelationKind};
use crate::standards::v1::subsets::base::schema::geometry::{SemioTransform,SemioPoint3,SemioQuaternion};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🔍️ Resolves a paid literal Model identity with bounded text comparisons.
fn entity_reference(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio Model entity identity"))
}
/// 🏛️ Resolves only the declared spatial owner partition for hierarchy and containment.
fn spatial_reference(ids:&[(&str,i64)],id:&str,count:usize,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let row=entity_reference(ids,id,out)?;if row>number(count)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio Model parent is outside its spatial partition"))}Ok(row)
}
/// ♻️ Detects spatial cycles using paid fixed-width parent marks without growing path ownership.
fn spatial_cycles(parents:&mut[(Option<usize>,u8)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 for root in 0..parents.len(){
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;match parents[index].1{2=>break,1=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"cyclic Semio Model spatial hierarchy")),_=>{parents[index].1=1;current=parents[index].0;}}}
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;if parents[index].1!=1{break}parents[index].1=2;current=parents[index].0;}
 }Ok(())
}
/// 🪪️ Collects the exact entity partition using a paid borrowed frontier and cancellable comparisons.
fn entity_frontier<'a>(snapshot:&'a SemioModelSnapshot,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let count=snapshot.spatial.len().checked_add(snapshot.elements.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model entity extent overflow"))?;let phase=out.phase();let mut ids=out.allocate_frontier(count)?;
 for(ordinal,id)in snapshot.spatial.iter().map(|node|node.id.as_str()).chain(snapshot.elements.iter().map(|element|element.id.as_str())).enumerate(){out.checkpoint()?;ids.push((id,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model entity identity overflow"))?)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Model entity identity"))}}Ok(ids)
}
/// 📐️ Projects all ten exact placement words through the actual float column declaration.
fn placement(id:i64,t:SemioTransform,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("semio_model_placement",id,&[Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)],float_columns("semio_model_placement"))
}
/// 🫳️ Visits the real Model entities, hierarchy, typed property variants and relation endpoints.
pub(crate)fn visit_rows(snapshot:&SemioModelSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let entities=entity_frontier(snapshot,out)?;let mut parents=out.allocate_frontier(snapshot.spatial.len())?;
 for node in &snapshot.spatial{out.checkpoint()?;let parent=match &node.parent_id{None=>None,Some(id)=>Some(usize::try_from(spatial_reference(&entities,id,snapshot.spatial.len(),out)?-1).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)};parents.push((parent,0u8));}
 spatial_cycles(&mut parents,out)?;let phase=out.phase();let mut relations=out.allocate_frontier(snapshot.relations.len())?;for edge in &snapshot.relations{out.checkpoint()?;relations.push(edge.id.as_str());}
 out.sort_frontier(&mut relations,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a,b,phase,control))?;
 for pair in relations.windows(2){if out.compare_text(pair[0],pair[1])?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Model relation identity"))}}
 out.insert_key("semio_model_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.spatial.iter().enumerate(){
  let id=entity_reference(&entities,&node.id,out)?;let parent=parents[ordinal].0.map(|index|number(index+1).map(Cell::Integer)).transpose()?.unwrap_or(Cell::Null);
  out.insert_key("semio_model_entity",id,&[Cell::Integer(1),Cell::Text("spatial"),Cell::Integer(number(ordinal)?),Cell::Text(&node.id)])?;
  out.insert_key("semio_model_spatial",id,&[Cell::Text(match node.kind{SpatialKind::Site=>"site",SpatialKind::Building=>"building",SpatialKind::Storey=>"storey",SpatialKind::Space=>"space"}),Cell::Text(&node.name),parent])?;
  placement(id,node.placement,out)?;
 }
 for(ordinal,element)in snapshot.elements.iter().enumerate(){
  let id=entity_reference(&entities,&element.id,out)?;let(class,name)=class(&element.class);let spatial=match &element.spatial_id{None=>Cell::Null,Some(id)=>Cell::Integer(spatial_reference(&entities,id,snapshot.spatial.len(),out)?)};
  out.insert_key("semio_model_entity",id,&[Cell::Integer(1),Cell::Text("element"),Cell::Integer(number(ordinal)?),Cell::Text(&element.id)])?;
  out.insert_key("semio_model_element",id,&[Cell::Text(class),name.map(Cell::Text).unwrap_or(Cell::Null),spatial])?;placement(id,element.placement,out)?;
  let(kind,target)=match &element.geometry{GeometryRef::None=>("none",Cell::Null),GeometryRef::Brep{brep_id}=>("brep",Cell::Text(brep_id)),GeometryRef::Mesh{mesh_id}=>("mesh",Cell::Text(mesh_id))};
  out.insert_key("semio_model_geometry_reference",id,&[Cell::Text(kind),target])?;
  for(ordinal,pset)in element.psets.iter().enumerate(){
   let owner=out.insert("semio_model_property_set",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&pset.name)])?;
   for(ordinal,property)in pset.properties.iter().enumerate(){
    let(kind,text,number_value,boolean)=match &property.value{PsetValue::Text{value}=>("text",Cell::Text(value),Cell::Null,Cell::Null),PsetValue::Number{value}=>("number",Cell::Null,Cell::Real(*value),Cell::Null),PsetValue::Boolean{value}=>("boolean",Cell::Null,Cell::Null,Cell::Integer(i64::from(*value)))};
    out.insert_float("semio_model_property",&[Cell::Integer(owner),Cell::Integer(number(ordinal)?),Cell::Text(&property.key),Cell::Text(kind),text,number_value,boolean],float_columns("semio_model_property"))?;
   }
  }
 }
 for(ordinal,edge)in snapshot.relations.iter().enumerate(){
  let(kind,label)=relation(&edge.kind);let source=entity_reference(&entities,&edge.from,out)?;let target=entity_reference(&entities,&edge.to,out)?;
  out.insert("semio_model_relation",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Text(kind),label.map(Cell::Text).unwrap_or(Cell::Null),Cell::Integer(source),Cell::Integer(target)])?;
 }Ok(())
}
/// 🎟️ Admits all typed Model cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioModelSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model row identity or columns"))}else{Ok(())}}
fn optional_integer(row:SqliteRow<'_>,index:usize)->Result<Option<i64>,ValueError>{if row.is_null(index)?{Ok(None)}else{row.integer(index).map(Some)}}
/// 🚫️ Reports an authored Model relationship refusal.
fn model_invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
/// ⏱️ Checks each Model reconstruction frontier.
fn model_checkpoint(control:&mut SqliteSnapshotControl<'_>,completed:usize,total:usize)->Result<(),ValueError>{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,total)}
/// 🏛️ Binds Model row declarations to the shared paid index.
fn model_rows<'a>(db:&'a SqliteDatabase,name:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<RowIndex<'a>,ValueError>{RowIndex::new(db,name,columns,float_columns(name),control,match name{"semio_model_property_set"=>"invalid Semio model property-set owner or identity","semio_model_property"=>"invalid Semio model property owner or identity","semio_model_relation"=>"invalid Semio model relation owner or identity",_=>"invalid Semio model row identity or columns"})}
/// 🪪️ Validates Model text identities through shared paid ordering.
fn model_unique_text(rows:&RowIndex<'_>,column:usize,message:&str,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{rows.unique_text(rows.indices(),column,control,message)}
/// 🔢️ Orders an already paid typed entity partition by contiguous authored ordinals.
fn ordered(mut indices:Vec<usize>,rows:&RowIndex<'_>,column:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<usize>,ValueError>{
 semio_framework_os_kernel::sqlite_snapshot::transfer::heap_sort(&mut indices,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(rows.row(*a)?.integer(column)?.cmp(&rows.row(*b)?.integer(column)?)))?;
 for(ordinal,&index)in indices.iter().enumerate(){if rows.row(index)?.integer(column)?!=number(ordinal)?{return Err(model_invalid("Semio model ordinals must be contiguous"));}model_checkpoint(control,ordinal+1,indices.len())?;}Ok(indices)
}
/// 👪️ Groups only the Model owner and ordinal columns.
fn model_groups(rows:&RowIndex<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<usize>,ValueError>{rows.grouped_by(2,control,"Semio model ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))}
/// 🔎️ Selects one Model owner's exact shared index range.
fn model_group_range(rows:&RowIndex<'_>,indices:&[usize],owner:i64,control:&mut SqliteSnapshotControl<'_>)->Result<std::ops::Range<usize>,ValueError>{rows.range_by(indices,(0,Some(owner)),control,|row|Ok((0,Some(row.integer(1)?))))}

fn class(value:&ElementClass)->(&'static str,Option<&str>){match value{ElementClass::Wall=>("wall",None),ElementClass::Slab=>("slab",None),ElementClass::Column=>("column",None),ElementClass::Beam=>("beam",None),ElementClass::Door=>("door",None),ElementClass::Window=>("window",None),ElementClass::Roof=>("roof",None),ElementClass::Stair=>("stair",None),ElementClass::Furniture=>("furniture",None),ElementClass::Other{name}=>("other",Some(name))}}
fn relation(value:&RelationKind)->(&'static str,Option<&str>){match value{RelationKind::Aggregates=>("aggregates",None),RelationKind::ContainedIn=>("contained_in",None),RelationKind::ConnectsTo=>("connects_to",None),RelationKind::FillsVoid=>("fills_void",None),RelationKind::VoidsElement=>("voids_element",None),RelationKind::Other{label}=>("other",Some(label))}}
fn restore_placement(row:SqliteRow<'_>)->Result<SemioTransform,ValueError>{Ok(SemioTransform{translation:SemioPoint3{x:row.real(1)?,y:row.real(2)?,z:row.real(3)?},rotation:SemioQuaternion{x:row.real(4)?,y:row.real(5)?,z:row.real(6)?,w:row.real(7)?},scale:SemioPoint3{x:row.real(8)?,y:row.real(9)?,z:row.real(10)?}})}
impl ArtifactSqliteSnapshot for SemioModelSnapshot{
fn retire_sqlite_snapshot(self){drop(native::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::model::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}

fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::model::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="model"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_model_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioModelSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let document=single_float_row(database,"semio_model_document")?;identity(document,2)?;if document.rowid!=1{return Err(model_invalid("invalid Semio model document identifier"));}
 let entities=model_rows(database,"semio_model_entity",5,control)?;
 let mut spatial_rows=model_rows(database,"semio_model_spatial",4,control)?;
 let mut element_rows=model_rows(database,"semio_model_element",4,control)?;
 let mut placements=model_rows(database,"semio_model_placement",11,control)?;
 let mut geometries=model_rows(database,"semio_model_geometry_reference",3,control)?;
 model_unique_text(&entities,4,"invalid Semio model entity ownership or identity",control)?;
 let mut spatial_count=0;let mut element_count=0;
 for(index,&position)in entities.indices().iter().enumerate(){let row=entities.row(position)?;if row.integer(1)?!=1{return Err(model_invalid("invalid Semio model entity ownership or identity"));}match row.text(2)?{"spatial"=>spatial_count+=1,"element"=>element_count+=1,_=>return Err(model_invalid("unknown Semio model entity kind"))}model_checkpoint(control,index+1,entities.indices().len())?;}
 let mut spatial_order=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(spatial_count,control)?;
 let mut element_order=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(element_count,control)?;
 for &position in entities.indices(){match entities.row(position)?.text(2)?{"spatial"=>spatial_order.push(position),"element"=>element_order.push(position),_=>unreachable!()}model_checkpoint(control,spatial_order.len()+element_order.len(),entities.indices().len())?;}
 let spatial_order=ordered(spatial_order,&entities,3,control)?;let element_order=ordered(element_order,&entities,3,control)?;
 let mut parents=spatial_rows.parent_positions(3,control,"dangling Semio model spatial parent")?;
 RowIndex::cycles(&mut parents,control,"cyclic Semio model spatial hierarchy")?;
 let mut psets=model_rows(database,"semio_model_property_set",4,control)?;
 let mut properties=model_rows(database,"semio_model_property",8,control)?;
 let mut completed=0;
 for &index in psets.indices(){let row=psets.row(index)?;if element_rows.get(row.integer(1)?,control)?.is_none(){return Err(model_invalid("invalid Semio model property-set owner or identity"));}row.integer(2)?;completed+=1;model_checkpoint(control,completed,0)?;}
 for &index in properties.indices(){let row=properties.row(index)?;if psets.get(row.integer(1)?,control)?.is_none(){return Err(model_invalid("invalid Semio model property owner or identity"));}row.integer(2)?;completed+=1;model_checkpoint(control,completed,0)?;}
 let pset_order=model_groups(&psets,control)?;let property_order=model_groups(&properties,control)?;
 let mut snapshot=native::Owned::new(Self{schema:String::new(),spatial:Vec::new(),elements:Vec::new(),relations:Vec::new()});
 snapshot.get_mut().spatial=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(spatial_order.len(),control)?;
 snapshot.get_mut().elements=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(element_order.len(),control)?;
 for index in spatial_order{
  let entity=entities.row(index)?;let row=spatial_rows.take(entity.rowid,control)?.ok_or_else(||model_invalid("missing Semio model spatial detail"))?;
  let mut node=native::Owned::new(SpatialNode{id:String::new(),kind:SpatialKind::Site,name:String::new(),parent_id:None,placement:SemioTransform::default()});
  node.get_mut().parent_id=optional_integer(row,3)?.map(|id|{let parent=entities.get(id,control)?.ok_or_else(||model_invalid("dangling Semio model spatial parent"))?;reconstruct_text(control,parent.text(4)?)}).transpose()?;
  node.get_mut().kind=match row.text(1)?{"site"=>SpatialKind::Site,"building"=>SpatialKind::Building,"storey"=>SpatialKind::Storey,"space"=>SpatialKind::Space,_=>return Err(model_invalid("unknown Semio spatial kind"))};
  node.get_mut().id=reconstruct_text(control,entity.text(4)?)?;
  node.get_mut().name=reconstruct_text(control,row.text(2)?)?;
  node.get_mut().placement=restore_placement(placements.take(entity.rowid,control)?.ok_or_else(||model_invalid("missing spatial placement"))?)?;
  snapshot.get_mut().spatial.push(node.take());completed+=1;model_checkpoint(control,completed,0)?;
 }
 for index in element_order{
  let entity=entities.row(index)?;let row=element_rows.take(entity.rowid,control)?.ok_or_else(||model_invalid("missing Semio model element detail"))?;
  let mut element=native::Owned::new(SemioModelElement{id:String::new(),class:ElementClass::Wall,placement:SemioTransform::default(),geometry:GeometryRef::None,spatial_id:None,psets:Vec::new()});
  let other=row.optional_text(2)?;
  element.get_mut().class=match row.text(1)?{
   "wall"=>ElementClass::Wall,"slab"=>ElementClass::Slab,"column"=>ElementClass::Column,"beam"=>ElementClass::Beam,"door"=>ElementClass::Door,"window"=>ElementClass::Window,"roof"=>ElementClass::Roof,"stair"=>ElementClass::Stair,"furniture"=>ElementClass::Furniture,
   "other"=>ElementClass::Other{name:reconstruct_text(control,other.ok_or_else(||model_invalid("missing custom Semio element class name"))?)?},
   _=>return Err(model_invalid("unknown Semio model element class"))
  };
  if row.text(1)?!="other"&&other.is_some(){return Err(model_invalid("unexpected custom Semio element class name"));}
  element.get_mut().spatial_id=optional_integer(row,3)?.map(|id|{if spatial_rows.get(id,control)?.is_none(){return Err(model_invalid("dangling Semio element spatial reference"));}let entity=entities.get(id,control)?.ok_or_else(||model_invalid("dangling Semio element spatial reference"))?;reconstruct_text(control,entity.text(4)?)}).transpose()?;
  let reference=geometries.take(entity.rowid,control)?.ok_or_else(||model_invalid("missing Semio model geometry reference"))?;let target=reference.optional_text(2)?;
  element.get_mut().geometry=match reference.text(1)?{
   "none" if target.is_none()=>GeometryRef::None,
   "brep"=>GeometryRef::Brep{brep_id:reconstruct_text(control,target.ok_or_else(||model_invalid("missing Semio BRep geometry identity"))?)?},
   "mesh"=>GeometryRef::Mesh{mesh_id:reconstruct_text(control,target.ok_or_else(||model_invalid("missing Semio mesh geometry identity"))?)?},
   _=>return Err(model_invalid("invalid Semio model geometry reference shape"))
  };
  let range=model_group_range(&psets,&pset_order,entity.rowid,control)?;
  element.get_mut().psets=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;
  for pset_index in range{
   let source=psets.row(pset_order[pset_index])?;let pset=psets.take(source.rowid,control)?.ok_or_else(||model_invalid("invalid Semio model property-set owner or identity"))?;
   let mut set=native::Owned::new(PropertySet{name:String::new(),properties:Vec::new()});
   let range=model_group_range(&properties,&property_order,pset.rowid,control)?;
   set.get_mut().properties=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;
   for property_index in range{
    let source=properties.row(property_order[property_index])?;let property=properties.take(source.rowid,control)?.ok_or_else(||model_invalid("invalid Semio model property owner or identity"))?;
    let mut field=native::Owned::new(Property{key:String::new(),value:PsetValue::Boolean{value:false}});
    field.get_mut().value=match property.text(4)?{
     "text" if property.is_null(6)?&&property.is_null(7)?=>PsetValue::Text{value:reconstruct_text(control,property.text(5)?)?},
     "number" if property.is_null(5)?&&property.is_null(7)?=>PsetValue::Number{value:property.real(6)?},
     "boolean" if property.is_null(5)?&&property.is_null(6)?=>PsetValue::Boolean{value:match property.integer(7)?{0=>false,1=>true,_=>return Err(model_invalid("invalid Semio model boolean property"))}},
     _=>return Err(model_invalid("invalid Semio model property variant shape"))
    };
    field.get_mut().key=reconstruct_text(control,property.text(3)?)?;
    set.get_mut().properties.push(field.take());completed+=1;model_checkpoint(control,completed,0)?;
   }
   set.get_mut().name=reconstruct_text(control,pset.text(3)?)?;
   element.get_mut().psets.push(set.take());
  }
  element.get_mut().id=reconstruct_text(control,entity.text(4)?)?;
  element.get_mut().placement=restore_placement(placements.take(entity.rowid,control)?.ok_or_else(||model_invalid("missing element placement"))?)?;
  snapshot.get_mut().elements.push(element.take());
 }
 if spatial_rows.remaining()!=0||element_rows.remaining()!=0||placements.remaining()!=0||geometries.remaining()!=0||psets.remaining()!=0||properties.remaining()!=0{return Err(model_invalid("orphan or contradictory Semio model entity detail"));}
 let relations=model_rows(database,"semio_model_relation",8,control)?;
 model_unique_text(&relations,3,"invalid Semio model relation owner or identity",control)?;
 let mut relation_order=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(relations.indices().len(),control)?;
 for &index in relations.indices(){relation_order.push(index);model_checkpoint(control,relation_order.len(),relations.indices().len())?;}
 let relation_order=ordered(relation_order,&relations,2,control)?;
 snapshot.get_mut().relations=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(relation_order.len(),control)?;
 for index in relation_order{
  let row=relations.row(index)?;if row.integer(1)?!=1{return Err(model_invalid("invalid Semio model relation owner or identity"));}
  let mut edge=native::Owned::new(ModelRelation{id:String::new(),kind:RelationKind::Aggregates,from:String::new(),to:String::new()});let other=row.optional_text(5)?;
  edge.get_mut().kind=match row.text(4)?{
   "aggregates"=>RelationKind::Aggregates,"contained_in"=>RelationKind::ContainedIn,"connects_to"=>RelationKind::ConnectsTo,"fills_void"=>RelationKind::FillsVoid,"voids_element"=>RelationKind::VoidsElement,
   "other"=>RelationKind::Other{label:reconstruct_text(control,other.ok_or_else(||model_invalid("missing custom Semio relation label"))?)?},
   _=>return Err(model_invalid("unknown Semio model relation kind"))
  };
  if row.text(4)?!="other"&&other.is_some(){return Err(model_invalid("unexpected custom Semio model relation label"));}
  edge.get_mut().id=reconstruct_text(control,row.text(3)?)?;
  let from=entities.get(row.integer(6)?,control)?.ok_or_else(||model_invalid("dangling Semio model relation source"))?;edge.get_mut().from=reconstruct_text(control,from.text(4)?)?;
  let to=entities.get(row.integer(7)?,control)?.ok_or_else(||model_invalid("dangling Semio model relation target"))?;edge.get_mut().to=reconstruct_text(control,to.text(4)?)?;
  snapshot.get_mut().relations.push(edge.take());completed+=1;model_checkpoint(control,completed,0)?;
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;
 model_checkpoint(control,completed,completed)?;
 Ok(snapshot.take())
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_model_property"=>&[FloatColumn::Binary64(6)],"semio_model_placement"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10)],_=>&[]}}


fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioModelSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.spatial.len())?;for spatial in &self.spatial{b.text(&spatial.id)?;b.text(&spatial.name)?;b.optional_text(spatial.parent_id.as_deref())?;b.scalars(11)?;}b.entities(self.elements.len())?;for element in &self.elements{b.text(&element.id)?;b.optional_text(element.spatial_id.as_deref())?;b.scalars(11)?;if let ElementClass::Other{name}=&element.class{b.text(name)?;}match &element.geometry{GeometryRef::None=>{},GeometryRef::Brep{brep_id}=>b.text(brep_id)?,GeometryRef::Mesh{mesh_id}=>b.text(mesh_id)?}b.entities(element.psets.len())?;for pset in &element.psets{b.text(&pset.name)?;b.entities(pset.properties.len())?;for property in &pset.properties{b.text(&property.key)?;match &property.value{PsetValue::Text{value}=>b.text(value)?,PsetValue::Number{..}|PsetValue::Boolean{..}=>b.scalars(1)?}}}}b.entities(self.relations.len())?;for relation in &self.relations{b.text(&relation.id)?;b.text(&relation.from)?;b.text(&relation.to)?;b.scalars(1)?;if let RelationKind::Other{label}=&relation.kind{b.text(label)?;}}Ok(())}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
