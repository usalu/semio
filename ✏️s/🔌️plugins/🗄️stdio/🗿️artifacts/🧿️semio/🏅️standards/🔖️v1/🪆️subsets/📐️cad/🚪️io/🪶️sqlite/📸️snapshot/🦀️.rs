//! 📐️ CAD layers, block ownership and nine independently typed entity geometries.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,RowIndex};
use crate::standards::v1::subsets::cad::schema::snapshot::{SemioCadSnapshot,CadLayer,CadBlock,CadEntityRecord,CadEntity};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🪪️ Builds a paid name frontier in authored ordinal order with bounded comparisons.
fn cad_frontier<'a>(names:impl Iterator<Item=&'a str>,count:usize,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut ids=out.allocate_frontier(count)?;for(ordinal,name)in names.enumerate(){out.checkpoint()?;ids.push((name,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio CAD identity extent overflow"))?)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio CAD identity within its owner"))}}Ok(ids)
}
/// 🔍️ Resolves actual CAD layer and block identities with cancellable UTF8 comparisons.
fn cad_reference(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio CAD layer or block"))
}
/// 🏷️ Admits owner-local handles before visiting any entity geometry.
fn cad_records(records:&[CadEntityRecord],block:Option<i64>,layers:&[(&str,i64)],blocks:&[(&str,i64)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let _handles=cad_frontier(records.iter().map(|record|record.handle.as_str()),records.len(),out)?;for(ordinal,record)in records.iter().enumerate(){project_record(record,block,ordinal,layers,blocks,out)?;}Ok(())
}
/// 🫳️ Visits every authored CAD cell using the same actual RowWriter for admission and projection.
pub(crate)fn visit_rows(snapshot:&SemioCadSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let layers=cad_frontier(snapshot.layers.iter().map(|layer|layer.name.as_str()),snapshot.layers.len(),out)?;let blocks=cad_frontier(snapshot.blocks.iter().map(|block|block.name.as_str()),snapshot.blocks.len(),out)?;
 out.insert_key("semio_cad_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,layer)in snapshot.layers.iter().enumerate(){out.insert("semio_cad_layer",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&layer.name),Cell::Integer(i64::from(layer.color_index)),Cell::Text(&layer.line_type),Cell::Integer(i64::from(layer.visible))])?;}
 for(ordinal,block)in snapshot.blocks.iter().enumerate(){out.insert_float("semio_cad_block",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&block.name),Cell::Real(block.base_point.x),Cell::Real(block.base_point.y)],float_columns("semio_cad_block"))?;}
 for block in &snapshot.blocks{let owner=cad_reference(&blocks,&block.name,out)?;cad_records(&block.entities,Some(owner),&layers,&blocks,out)?;}cad_records(&snapshot.entities,None,&layers,&blocks,out)
}
/// 🧱️ Projects all nine detail variants through their actual exact numeric columns.
fn project_record(record:&CadEntityRecord,block:Option<i64>,ordinal:usize,layers:&[(&str,i64)],blocks:&[(&str,i64)],p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
let kind=match record.entity{CadEntity::Line{..}=>"line",CadEntity::Arc{..}=>"arc",CadEntity::Circle{..}=>"circle",CadEntity::Ellipse{..}=>"ellipse",CadEntity::Polyline{..}=>"polyline",CadEntity::Text{..}=>"text",CadEntity::Insert{..}=>"insert",CadEntity::Solid{..}=>"solid",CadEntity::Dimension{..}=>"dimension"};let layer_id=cad_reference(layers,&record.layer,p)?;let id=p.insert("semio_cad_entity",&[Cell::Integer(1),block.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Integer(number(ordinal)?),Cell::Text(&record.handle),Cell::Integer(layer_id),Cell::Text(kind)])?;
match &record.entity{
CadEntity::Line{a,b}=>p.insert_key_float("semio_cad_line",id,&[Cell::Real(a.x),Cell::Real(a.y),Cell::Real(b.x),Cell::Real(b.y)],float_columns("semio_cad_line"))?,
CadEntity::Arc{center,radius,start_angle,end_angle}=>p.insert_key_float("semio_cad_arc",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius),Cell::Real(*start_angle),Cell::Real(*end_angle)],float_columns("semio_cad_arc"))?,
CadEntity::Circle{center,radius}=>p.insert_key_float("semio_cad_circle",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius)],float_columns("semio_cad_circle"))?,
CadEntity::Ellipse{center,major_axis_end,ratio,start_param,end_param}=>p.insert_key_float("semio_cad_ellipse",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(major_axis_end.x),Cell::Real(major_axis_end.y),Cell::Real(*ratio),Cell::Real(*start_param),Cell::Real(*end_param)],float_columns("semio_cad_ellipse"))?,
CadEntity::Polyline{vertices,closed}=>{p.insert_key_float("semio_cad_polyline",id,&[Cell::Integer(i64::from(*closed))],float_columns("semio_cad_polyline"))?;for(ordinal,vertex)in vertices.iter().enumerate(){p.insert_float("semio_cad_polyline_vertex",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(vertex.x),Cell::Real(vertex.y)],float_columns("semio_cad_polyline_vertex"))?;}},
CadEntity::Text{position,height,rotation,content}=>p.insert_key_float("semio_cad_text",id,&[Cell::Real(position.x),Cell::Real(position.y),Cell::Real(*height),Cell::Real(*rotation),Cell::Text(content)],float_columns("semio_cad_text"))?,
CadEntity::Insert{block_name,insertion_point,scale,rotation}=>{let target=cad_reference(blocks,block_name,p)?;p.insert_key_float("semio_cad_insert",id,&[Cell::Integer(target),Cell::Real(insertion_point.x),Cell::Real(insertion_point.y),Cell::Real(scale.x),Cell::Real(scale.y),Cell::Real(*rotation)],float_columns("semio_cad_insert"))?;},
CadEntity::Solid{p1,p2,p3,p4}=>p.insert_key_float("semio_cad_solid",id,&[Cell::Real(p1.x),Cell::Real(p1.y),Cell::Real(p2.x),Cell::Real(p2.y),Cell::Real(p3.x),Cell::Real(p3.y),Cell::Real(p4.x),Cell::Real(p4.y)],float_columns("semio_cad_solid"))?,
CadEntity::Dimension{def_point,text_position,measurement,text}=>p.insert_key_float("semio_cad_dimension",id,&[Cell::Real(def_point.x),Cell::Real(def_point.y),Cell::Real(text_position.x),Cell::Real(text_position.y),Cell::Real(*measurement),Cell::Text(text)],float_columns("semio_cad_dimension"))?}
Ok(())}/// 🎟️ Admits all typed CAD cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioCadSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD row identity or columns"))}else{Ok(())}}
fn boolean(value:i64)->Result<bool,ValueError>{match value{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD boolean"))}}
fn optional_integer(row:SqliteRow<'_>,index:usize)->Result<Option<i64>,ValueError>{if row.is_null(index)?{Ok(None)}else{row.integer(index).map(Some)}}
/// 🚫️ Reports an authored CAD relational refusal.
fn cad_invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
/// 🗃️ Binds exact CAD logical and IEEE columns to shared paid row positions.
fn details<'a>(db:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<RowIndex<'a>,ValueError>{RowIndex::new(db,table,columns,float_columns(table),control,"invalid Semio CAD row identity or columns")}
/// 📐️ Restores only the declared binary64 point words.
fn point(row:SqliteRow<'_>,x:usize)->Result<SemioPoint2,ValueError>{Ok(SemioPoint2{x:row.real(x)?,y:row.real(x+1)?})}
/// 🧷️ Retains borrowed CAD variant indices and their paid owner orderings.
struct CadReconstruction<'a>{entities:RowIndex<'a>,order:Vec<usize>,lines:RowIndex<'a>,arcs:RowIndex<'a>,circles:RowIndex<'a>,ellipses:RowIndex<'a>,polylines:RowIndex<'a>,texts:RowIndex<'a>,inserts:RowIndex<'a>,solids:RowIndex<'a>,dimensions:RowIndex<'a>,vertices:RowIndex<'a>,vertex_order:Vec<usize>}
impl CadReconstruction<'_>{
 /// 🫴️ Reconstructs one exact nullable block-owner group into guarded native records.
 fn records(&mut self,owner:Option<i64>,layers:&RowIndex<'_>,blocks:&RowIndex<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<CadEntityRecord>,ValueError>{
  let range=self.entities.range_by(&self.order,(0,owner),control,|row|Ok((0,optional_integer(row,2)?)))?;self.entities.unique_text(&self.order[range.clone()],4,control,"duplicate Semio CAD entity handle within owner")?;
  let mut records=Owned::new(semio_framework_os_kernel::sqlite_snapshot::transfer::reserve::<CadEntityRecord>(range.len(),control)?);
  for ordinal in range{
   let row=self.entities.take_index(self.order[ordinal],control)?.ok_or_else(||cad_invalid("duplicate Semio CAD entity ownership"))?;
   let mut record=Owned::new(CadEntityRecord{handle:String::new(),layer:String::new(),entity:CadEntity::Line{a:SemioPoint2::default(),b:SemioPoint2::default()}});
   let mut entity=match row.text(6)?{
    "line"=>{let detail=self.lines.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD line fields"))?;Owned::new(CadEntity::Line{a:point(detail,1)?,b:point(detail,3)?})},
    "arc"=>{let detail=self.arcs.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD arc fields"))?;Owned::new(CadEntity::Arc{center:point(detail,1)?,radius:detail.real(3)?,start_angle:detail.real(4)?,end_angle:detail.real(5)?})},
    "circle"=>{let detail=self.circles.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD circle fields"))?;Owned::new(CadEntity::Circle{center:point(detail,1)?,radius:detail.real(3)?})},
    "ellipse"=>{let detail=self.ellipses.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD ellipse fields"))?;Owned::new(CadEntity::Ellipse{center:point(detail,1)?,major_axis_end:point(detail,3)?,ratio:detail.real(5)?,start_param:detail.real(6)?,end_param:detail.real(7)?})},
    "polyline"=>{let detail=self.polylines.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD polyline fields"))?;let range=self.vertices.range_by(&self.vertex_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;let mut entity=Owned::new(CadEntity::Polyline{vertices:Vec::new(),closed:false});if let CadEntity::Polyline{vertices,closed}=entity.get_mut(){*vertices=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;for ordinal in range{let vertex=self.vertices.take_index(self.vertex_order[ordinal],control)?.ok_or_else(||cad_invalid("duplicate CAD vertex ownership"))?;vertices.push(point(vertex,3)?);}*closed=boolean(detail.integer(1)?)?;}entity},
    "text"=>{let detail=self.texts.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD text fields"))?;let mut entity=Owned::new(CadEntity::Text{position:SemioPoint2::default(),height:0.0,rotation:0.0,content:String::new()});if let CadEntity::Text{position,height,rotation,content}=entity.get_mut(){*content=reconstruct_text(control,detail.text(5)?)?;*position=point(detail,1)?;*height=detail.real(3)?;*rotation=detail.real(4)?;}entity},
    "insert"=>{let detail=self.inserts.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD insert fields"))?;let mut entity=Owned::new(CadEntity::Insert{block_name:String::new(),insertion_point:SemioPoint2::default(),scale:SemioPoint2::default(),rotation:0.0});if let CadEntity::Insert{block_name,insertion_point,scale,rotation}=entity.get_mut(){let block=blocks.get(detail.integer(1)?,control)?.ok_or_else(||cad_invalid("dangling CAD inserted block"))?;*block_name=reconstruct_text(control,block.text(3)?)?;*insertion_point=point(detail,2)?;*scale=point(detail,4)?;*rotation=detail.real(6)?;}entity},
    "solid"=>{let detail=self.solids.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD solid fields"))?;Owned::new(CadEntity::Solid{p1:point(detail,1)?,p2:point(detail,3)?,p3:point(detail,5)?,p4:point(detail,7)?})},
    "dimension"=>{let detail=self.dimensions.take(row.rowid,control)?.ok_or_else(||cad_invalid("missing CAD dimension fields"))?;let mut entity=Owned::new(CadEntity::Dimension{def_point:SemioPoint2::default(),text_position:SemioPoint2::default(),measurement:0.0,text:String::new()});if let CadEntity::Dimension{def_point,text_position,measurement,text}=entity.get_mut(){*text=reconstruct_text(control,detail.text(6)?)?;*def_point=point(detail,1)?;*text_position=point(detail,3)?;*measurement=detail.real(5)?;}entity},
    _=>return Err(cad_invalid("unknown Semio CAD entity kind"))
   };
   record.get_mut().entity=entity.take();record.get_mut().handle=reconstruct_text(control,row.text(4)?)?;let layer=layers.get(row.integer(5)?,control)?.ok_or_else(||cad_invalid("dangling CAD entity layer"))?;record.get_mut().layer=reconstruct_text(control,layer.text(3)?)?;
   control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,records.get_mut().len(),self.entities.len())?;records.get_mut().push(record.take());
  }Ok(records.take())
 }
}
impl ArtifactSqliteSnapshot for SemioCadSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::cad::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}

 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::cad::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="cad"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_cad_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioCadSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {
control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;let document=single_float_row(database,"semio_cad_document")?;identity(document,2)?;if document.rowid!=1{return Err(cad_invalid("invalid Semio CAD document identifier"))}
let layers=details(database,"semio_cad_layer",7,control)?;let blocks=details(database,"semio_cad_block",6,control)?;let layer_order=layers.ordered(2,control,"Semio CAD ordinals must be contiguous")?;let block_order=blocks.ordered(2,control,"Semio CAD ordinals must be contiguous")?;
for rows in [&layers,&blocks]{rows.unique_text(rows.indices(),3,control,"invalid Semio CAD native ownership or identity")?;for &index in rows.indices(){if rows.row(index)?.integer(1)?!=1{return Err(cad_invalid("invalid Semio CAD native ownership or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}}
let lines=details(database,"semio_cad_line",5,control)?;let arcs=details(database,"semio_cad_arc",6,control)?;let circles=details(database,"semio_cad_circle",4,control)?;let ellipses=details(database,"semio_cad_ellipse",8,control)?;let polylines=details(database,"semio_cad_polyline",2,control)?;let texts=details(database,"semio_cad_text",6,control)?;let inserts=details(database,"semio_cad_insert",7,control)?;let solids=details(database,"semio_cad_solid",9,control)?;let dimensions=details(database,"semio_cad_dimension",7,control)?;
let vertices=details(database,"semio_cad_polyline_vertex",5,control)?;for &index in vertices.indices(){if polylines.get(vertices.row(index)?.integer(1)?,control)?.is_none(){return Err(cad_invalid("invalid Semio CAD polyline vertex owner or identity"))}}
let vertex_order=vertices.grouped_by(2,control,"Semio CAD ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
let entities=details(database,"semio_cad_entity",7,control)?;for &index in entities.indices(){let row=entities.row(index)?;if row.integer(1)?!=1{return Err(cad_invalid("invalid Semio CAD entity ownership or identity"))}if let Some(id)=optional_integer(row,2)?{if blocks.get(id,control)?.is_none(){return Err(cad_invalid("invalid Semio CAD entity ownership or identity"))}}}
let order=entities.grouped_by(3,control,"Semio CAD ordinals must be contiguous",|row|Ok((0,optional_integer(row,2)?)))?;
let mut content=CadReconstruction{entities,order,lines,arcs,circles,ellipses,polylines,texts,inserts,solids,dimensions,vertices,vertex_order};
let mut snapshot=Owned::new(Self{schema:String::new(),layers:Vec::new(),blocks:Vec::new(),entities:Vec::new()});
snapshot.get_mut().layers=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(layer_order.len(),control)?;
for index in layer_order{let row=layers.row(index)?;let mut layer=Owned::new(CadLayer{name:String::new(),color_index:0,line_type:String::new(),visible:false});layer.get_mut().name=reconstruct_text(control,row.text(3)?)?;layer.get_mut().color_index=i32::try_from(row.integer(4)?).map_err(|error|cad_invalid(&error.to_string()))?;layer.get_mut().line_type=reconstruct_text(control,row.text(5)?)?;layer.get_mut().visible=boolean(row.integer(6)?)?;snapshot.get_mut().layers.push(layer.take());}
snapshot.get_mut().entities=content.records(None,&layers,&blocks,control)?;
snapshot.get_mut().blocks=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(block_order.len(),control)?;
for index in block_order{let row=blocks.row(index)?;let mut block=Owned::new(CadBlock{name:String::new(),base_point:SemioPoint2::default(),entities:Vec::new()});block.get_mut().name=reconstruct_text(control,row.text(3)?)?;block.get_mut().base_point=point(row,4)?;block.get_mut().entities=content.records(Some(row.rowid),&layers,&blocks,control)?;snapshot.get_mut().blocks.push(block.take());}
if [content.entities.remaining(),content.lines.remaining(),content.arcs.remaining(),content.circles.remaining(),content.ellipses.remaining(),content.polylines.remaining(),content.texts.remaining(),content.inserts.remaining(),content.solids.remaining(),content.dimensions.remaining(),content.vertices.remaining()].iter().any(|count|*count!=0){return Err(cad_invalid("orphan or contradictory Semio CAD variant detail"))}
snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot.take())

    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_cad_block"=>&[FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_cad_line"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_cad_arc"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_cad_circle"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3)],"semio_cad_ellipse"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)],"semio_cad_polyline_vertex"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_cad_text"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_cad_insert"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)],"semio_cad_solid"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)],"semio_cad_dimension"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],_=>&[]}}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioCadSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.layers.len())?;for layer in &self.layers{b.text(&layer.name)?;b.text(&layer.line_type)?;b.scalars(2)?;}b.entities(self.blocks.len())?;for block in &self.blocks{b.text(&block.name)?;b.scalars(2)?;native_entities(&block.entities,b)?;}native_entities(&self.entities,b)?;Ok(())}
}

fn native_entities(records:&[CadEntityRecord],b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(records.len())?;for record in records{b.text(&record.handle)?;b.text(&record.layer)?;match &record.entity{CadEntity::Line{..}=>b.scalars(4)?,CadEntity::Arc{..}=>b.scalars(5)?,CadEntity::Circle{..}=>b.scalars(3)?,CadEntity::Ellipse{..}=>b.scalars(7)?,CadEntity::Polyline{vertices,..}=>{b.entities(vertices.len())?;b.scalars(vertices.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CAD native point count overflow"))?)?;b.scalars(1)?;},CadEntity::Text{content,..}=>{b.text(content)?;b.scalars(4)?;},CadEntity::Insert{block_name,..}=>{b.text(block_name)?;b.scalars(5)?;},CadEntity::Solid{..}=>b.scalars(8)?,CadEntity::Dimension{text,..}=>{b.text(text)?;b.scalars(5)?;}}}Ok(())}

impl SemioCadSnapshot {
    /// 🪶️ Projects the owned subset into its declared relational writer while preserving refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
