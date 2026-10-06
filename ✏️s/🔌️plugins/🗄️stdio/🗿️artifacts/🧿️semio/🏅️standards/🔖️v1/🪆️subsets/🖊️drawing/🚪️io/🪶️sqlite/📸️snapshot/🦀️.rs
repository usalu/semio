//! 🖊️ Drawing node ownership, explicit path geometries, colors, layers and styles.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,insert_ieee754,insert_key_ieee754};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use crate::drawing::schema::snapshot::{SemioDrawingSnapshot,DrawCanvas,DrawStyle,DrawLayer,DrawNode,PathSegment};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2,SemioPoint3,SemioQuaternion,SemioRgba,SemioTransform};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,Reconstruction},SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🔍️rows/🦀️.rs"]mod indexed;
#[path="💰️reconstruction/🦀️.rs"]mod reconstruction;
fn number(n:usize)->Result<i64,ValueError>{i64::try_from(n).map_err(|e|ValueError::new(ValueRefusalKind::WorkLimit,e.to_string()))}
fn identity<'a>(r:impl std::borrow::Borrow<SqliteRow<'a>>,n:usize)->Result<(),ValueError>{let r=*r.borrow();if r.rowid<=0||r.integer(0)?!=r.rowid||r.values.len()!=n{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid drawing row identity or columns"))}else{Ok(())}}
fn boolean(n:i64)->Result<bool,ValueError>{match n{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid drawing boolean"))}}
fn optional_real(r:SqliteRow<'_>,i:usize)->Result<Option<f64>,ValueError>{if r.is_null(i)?{Ok(None)}else{r.real(i).map(Some)}}
fn rgba(r:SqliteRow<'_>,i:usize)->Result<Option<SemioRgba>,ValueError>{let mut nulls=0;for channel in i..i+4{if r.is_null(channel)?{nulls+=1;}}if nulls==4{Ok(None)}else if nulls!=0{Err(ValueError::new(ValueRefusalKind::InvalidValue,"drawing color channels must all be present or absent"))}else{Ok(Some(SemioRgba{r:r.binary32(i)?,g:r.binary32(i+1)?,b:r.binary32(i+2)?,a:r.binary32(i+3)?}))}}
fn colors(color:Option<SemioRgba>)->[Cell<'static>;4]{match color{None=>[Cell::Null;4],Some(c)=>[Cell::Float32(c.r),Cell::Float32(c.g),Cell::Float32(c.b),Cell::Float32(c.a)]}}
fn point(r:SqliteRow<'_>,i:usize)->Result<SemioPoint2,ValueError>{Ok(SemioPoint2{x:r.real(i)?,y:r.real(i+1)?})}
fn transform(r:SqliteRow<'_>)->Result<SemioTransform,ValueError>{Ok(SemioTransform{translation:SemioPoint3{x:r.real(1)?,y:r.real(2)?,z:r.real(3)?},rotation:SemioQuaternion{x:r.real(4)?,y:r.real(5)?,z:r.real(6)?,w:r.real(7)?},scale:SemioPoint3{x:r.real(8)?,y:r.real(9)?,z:r.real(10)?}})}
fn style_cell(style:Option<&str>,names:&[(&str,i64)],p:&mut Projection<'_,'_>)->Result<Cell<'static>,ValueError>{
 let Some(style)=style else{return Ok(Cell::Null)};
 let position=p.search_frontier(names,|candidate,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(candidate.0,style,SqliteSnapshotPhase::ProjectSnapshot,control))?;
 position.map(|position|Cell::Integer(names[position].1)).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling drawing style"))
}
fn project_node(node:&DrawNode,styles:&[(&str,i64)],p:&mut Projection<'_,'_>)->Result<i64,ValueError>{let root=p.insert_float("semio_drawing_node",&[Cell::Text(match node{DrawNode::Path{..}=>"path",DrawNode::Text{..}=>"text",DrawNode::Group{..}=>"group",DrawNode::Image{..}=>"image"})])?;let mut stack=p.allocate_frontier(1)?;stack.push((root,node));while let Some((id,node))=stack.pop(){match node{
DrawNode::Path{segments,style}=>{let style=style_cell(style.as_deref(),styles,p)?;p.insert_key_float("semio_drawing_path",id,&[style])?;for(ordinal,s)in segments.iter().enumerate(){let kind=match s{PathSegment::MoveTo{..}=>"move",PathSegment::LineTo{..}=>"line",PathSegment::CubicTo{..}=>"cubic",PathSegment::QuadTo{..}=>"quad",PathSegment::ArcTo{..}=>"arc",PathSegment::Close=>"close"};let sid=p.insert_float("semio_drawing_segment",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(kind)])?;match s{PathSegment::MoveTo{to}=>p.insert_key_float("semio_drawing_move",sid,&[Cell::Real(to.x),Cell::Real(to.y)])?,PathSegment::LineTo{to}=>p.insert_key_float("semio_drawing_line",sid,&[Cell::Real(to.x),Cell::Real(to.y)])?,PathSegment::CubicTo{c1,c2,to}=>p.insert_key_float("semio_drawing_cubic",sid,&[Cell::Real(c1.x),Cell::Real(c1.y),Cell::Real(c2.x),Cell::Real(c2.y),Cell::Real(to.x),Cell::Real(to.y)])?,PathSegment::QuadTo{c,to}=>p.insert_key_float("semio_drawing_quad",sid,&[Cell::Real(c.x),Cell::Real(c.y),Cell::Real(to.x),Cell::Real(to.y)])?,PathSegment::ArcTo{rx,ry,x_rotation,large_arc,sweep,to}=>p.insert_key_float("semio_drawing_arc",sid,&[Cell::Real(*rx),Cell::Real(*ry),Cell::Real(*x_rotation),Cell::Integer(i64::from(*large_arc)),Cell::Integer(i64::from(*sweep)),Cell::Real(to.x),Cell::Real(to.y)])?,PathSegment::Close=>p.insert_key_float("semio_drawing_close",sid,&[])?}}},
DrawNode::Text{value,at,style}=>{let style=style_cell(style.as_deref(),styles,p)?;p.insert_key_float("semio_drawing_text",id,&[Cell::Text(value),Cell::Real(at.x),Cell::Real(at.y),style])?},
DrawNode::Group{transform:t,children}=>{p.insert_key_float("semio_drawing_group",id,&[Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)])?;p.check_rows(stack.len().checked_add(children.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"drawing traversal count overflow"))?)?;for(ordinal,child)in children.iter().enumerate(){let kind=match child{DrawNode::Path{..}=>"path",DrawNode::Text{..}=>"text",DrawNode::Group{..}=>"group",DrawNode::Image{..}=>"image"};let cid=p.insert_float("semio_drawing_node",&[Cell::Text(kind)])?;p.insert_float("semio_drawing_child",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Integer(cid)])?;p.push_frontier(&mut stack,(cid,child))?;}},
DrawNode::Image{at,width,height,mime,bytes}=>p.insert_key_float("semio_drawing_image",id,&[Cell::Real(at.x),Cell::Real(at.y),Cell::Real(*width),Cell::Real(*height),Cell::Text(mime),Cell::Blob(bytes)])?}p.checkpoint()?;}Ok(root)}
impl ArtifactSqliteSnapshot for SemioDrawingSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::drawing::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::drawing::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="drawing"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_drawing_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ Self::reconstruct_sqlite_database(db, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioDrawingSnapshot {
    /// 🪆️ Projects owned Drawing rows directly into the parent's admitted relational writer.
    pub fn project_sqlite_component(&self,p:&mut Projection<'_,'_>)->Result<(),ValueError>{
 let bg=colors(self.canvas.background);p.insert_key_float("semio_drawing_document",1,&[Cell::Text(&self.schema),Cell::Real(self.canvas.width),Cell::Real(self.canvas.height),bg[0],bg[1],bg[2],bg[3]])?;
 let mut styles=p.allocate_frontier(self.styles.len())?;
 for(ordinal,s)in self.styles.iter().enumerate(){let f=colors(s.fill);let k=colors(s.stroke);let id=p.insert_float("semio_drawing_style",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&s.name),f[0],f[1],f[2],f[3],k[0],k[1],k[2],k[3],s.stroke_width.map(Cell::Real).unwrap_or(Cell::Null),s.opacity.map(|v|Cell::Float32(v)).unwrap_or(Cell::Null)])?;styles.push((s.name.as_str(),id));}
 p.sort_frontier(&mut styles,|left,right,control|{let order=semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(left.0,right.0,SqliteSnapshotPhase::ProjectSnapshot,control)?;if order.is_eq(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate drawing style identity"))}Ok(order)})?;
 let mut names=p.allocate_frontier(self.layers.len())?;for layer in &self.layers{names.push(layer.id.as_str());}
 p.sort_frontier(&mut names,|left,right,control|{let order=semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(left,right,SqliteSnapshotPhase::ProjectSnapshot,control)?;if order.is_eq(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate drawing layer identity"))}Ok(order)})?;
 for(ordinal,layer)in self.layers.iter().enumerate(){let root=project_node(&layer.root,&styles,p)?;p.insert_float("semio_drawing_layer",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&layer.id),Cell::Text(&layer.name),Cell::Integer(i64::from(layer.visible)),Cell::Integer(root)])?;}Ok(())
    }

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {
        reconstruction::reconstruct(db,control,declared_schema)
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_drawing_document"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary32(4),FloatColumn::Binary32(5),FloatColumn::Binary32(6),FloatColumn::Binary32(7)],"semio_drawing_style"=>&[FloatColumn::Binary32(4),FloatColumn::Binary32(5),FloatColumn::Binary32(6),FloatColumn::Binary32(7),FloatColumn::Binary32(8),FloatColumn::Binary32(9),FloatColumn::Binary32(10),FloatColumn::Binary32(11),FloatColumn::Binary64(12),FloatColumn::Binary32(13)],"semio_drawing_text"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3)],"semio_drawing_group"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10)],"semio_drawing_image"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_drawing_move"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2)],"semio_drawing_line"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2)],"semio_drawing_cubic"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)],"semio_drawing_quad"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_drawing_arc"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(6),FloatColumn::Binary64(7)],_=>&[]}}
trait FloatProjection { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>; fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>; }
impl FloatProjection for Projection<'_,'_> { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{insert_ieee754(self,table,cells,float_columns(table))} fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>{insert_key_ieee754(self,table,key,cells,float_columns(table))} }

impl SemioDrawingSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.scalars(6)?;b.entities(self.styles.len())?;for style in &self.styles{b.text(&style.name)?;b.scalars(10)?;}b.entities(self.layers.len())?;for layer in &self.layers{b.text(&layer.id)?;b.text(&layer.name)?;b.scalars(1)?;native_draw_nodes(std::slice::from_ref(&layer.root),b)?;}Ok(())}
}

fn native_draw_nodes(roots:&[DrawNode],b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(roots.len())?;let mut pending:Vec<_>=roots.iter().rev().collect();while let Some(node)=pending.pop(){b.entities(1)?;match node{DrawNode::Path{segments,style}=>{b.optional_text(style.as_deref())?;b.entities(segments.len())?;for segment in segments{b.scalars(match segment{PathSegment::MoveTo{..}|PathSegment::LineTo{..}=>2,PathSegment::CubicTo{..}=>6,PathSegment::QuadTo{..}=>4,PathSegment::ArcTo{..}=>7,PathSegment::Close=>0})?;}},DrawNode::Text{value,style,..}=>{b.text(value)?;b.optional_text(style.as_deref())?;b.scalars(2)?;},DrawNode::Group{children,..}=>{b.scalars(10)?;b.entities(children.len())?;pending.extend(children.iter().rev());},DrawNode::Image{mime,bytes,..}=>{b.scalars(4)?;b.text(mime)?;b.bytes(bytes)?;}}}Ok(())}

impl SemioDrawingSnapshot {
    /// 🪶️ Projects the owned subset into its declared relational writer while preserving refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut p=Projection::new(Self::SQLITE_SCHEMA,control)?;self.project_sqlite_component(&mut p)?;p.finish()}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
