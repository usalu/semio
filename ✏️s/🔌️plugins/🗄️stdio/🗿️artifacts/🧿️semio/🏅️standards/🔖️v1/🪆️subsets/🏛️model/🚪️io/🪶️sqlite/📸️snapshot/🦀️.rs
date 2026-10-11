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
fn class(value:&ElementClass)->(&'static str,Option<&str>){match value{ElementClass::Wall=>("wall",None),ElementClass::Slab=>("slab",None),ElementClass::Column=>("column",None),ElementClass::Beam=>("beam",None),ElementClass::Door=>("door",None),ElementClass::Window=>("window",None),ElementClass::Roof=>("roof",None),ElementClass::Stair=>("stair",None),ElementClass::Furniture=>("furniture",None),ElementClass::Other{name}=>("other",Some(name))}}
fn relation(value:&RelationKind)->(&'static str,Option<&str>){match value{RelationKind::Aggregates=>("aggregates",None),RelationKind::ContainedIn=>("contained_in",None),RelationKind::ConnectsTo=>("connects_to",None),RelationKind::FillsVoid=>("fills_void",None),RelationKind::VoidsElement=>("voids_element",None),RelationKind::Other{label}=>("other",Some(label))}}
fn restore_placement(row:SqliteRow<'_>)->Result<SemioTransform,ValueError>{Ok(SemioTransform{translation:SemioPoint3{x:row.real(1)?,y:row.real(2)?,z:row.real(3)?},rotation:SemioQuaternion{x:row.real(4)?,y:row.real(5)?,z:row.real(6)?,w:row.real(7)?},scale:SemioPoint3{x:row.real(8)?,y:row.real(9)?,z:row.real(10)?}})}
impl ArtifactSqliteSnapshot for SemioModelSnapshot{
fn retire_sqlite_snapshot(self){drop(native::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::model::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}

fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::v1::subsets::model::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control.native())}
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
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
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

#[path="💰️reconstruction/🦀️.rs"]mod reconstruction;
