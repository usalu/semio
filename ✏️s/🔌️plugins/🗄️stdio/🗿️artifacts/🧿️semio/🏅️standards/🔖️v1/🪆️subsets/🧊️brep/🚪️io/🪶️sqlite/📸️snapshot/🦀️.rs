//! 🧊️ Full typed BRep topology, analytic geometry, p-curves and NURBS parameter sequences.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,RowIndex,reconstruct_text};
use crate::standards::v1::subsets::brep::schema::snapshot::{SemioBrepSnapshot,BrepVertex,BrepEdge,BrepLoop,BrepLoopEdge,BrepCoedge,BrepFace,BrepShell,BrepShellFace,BrepSolid,BrepSolidShell,BrepCurve,BrepCurve2,BrepSurface};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2,SemioPoint3};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,Reconstruction},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding}};
use semio_framework_os_kernel::sqlite_snapshot::transfer;
#[path="🧮️semantic/🦀️.rs"]
pub(crate)mod semantic;
fn unsigned64(mut value:u64,buffer:&mut[u8;20])->&str{let mut start=buffer.len();loop{start-=1;buffer[start]=b'0'+(value%10)as u8;value/=10;if value==0{break}}std::str::from_utf8(&buffer[start..]).expect("decimal digits are UTF8")}
/// 🫳️ Visits every actual BRep relational field without cloning geometry or topology.
pub(crate)fn visit_rows(snapshot:&SemioBrepSnapshot,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let mut buffer=[0u8;20];let label=unsigned64(snapshot.next_label,&mut buffer);p.insert_key_float("semio_brep_document",1,&[Cell::Text(&snapshot.schema),Cell::Text(label)],float_columns("semio_brep_document"))?;
for(ordinal,v)in snapshot.vertices.iter().enumerate(){p.insert_key_float("semio_brep_vertex",number(ordinal+1)?,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&v.id),Cell::Real(v.point.x),Cell::Real(v.point.y),Cell::Real(v.point.z),Cell::Real(v.tol)],float_columns("semio_brep_vertex"))?;}
for(ordinal,e)in snapshot.edges.iter().enumerate(){let curve=project_curve3(&e.curve,p)?;p.insert_key_float("semio_brep_edge",number(ordinal+1)?,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&e.id),Cell::Text(&e.start_vertex),Cell::Text(&e.end_vertex),Cell::Integer(curve),Cell::Real(e.tol)],float_columns("semio_brep_edge"))?;}
for(ordinal,l)in snapshot.loops.iter().enumerate(){let id=number(ordinal+1)?;p.insert_key_float("semio_brep_loop",id,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&l.id)],float_columns("semio_brep_loop"))?;for(ordinal,e)in l.edges.iter().enumerate(){p.insert_float("semio_brep_loop_edge",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&e.edge),Cell::Integer(i64::from(e.orientation))],float_columns("semio_brep_loop_edge"))?;}}
for(ordinal,f)in snapshot.faces.iter().enumerate(){let id=number(ordinal+1)?;let surface=project_surface(&f.surface,p)?;p.insert_key_float("semio_brep_face",id,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&f.id),Cell::Text(&f.outer_loop),Cell::Integer(surface),Cell::Integer(i64::from(f.orientation)),Cell::Real(f.tol)],float_columns("semio_brep_face"))?;for(ordinal,l)in f.inner_loops.iter().enumerate(){p.insert_float("semio_brep_inner_loop",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(l)],float_columns("semio_brep_inner_loop"))?;}}
for(ordinal,s)in snapshot.shells.iter().enumerate(){let id=number(ordinal+1)?;p.insert_key_float("semio_brep_shell",id,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&s.id)],float_columns("semio_brep_shell"))?;for(ordinal,f)in s.faces.iter().enumerate(){p.insert_float("semio_brep_shell_face",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&f.face),Cell::Integer(i64::from(f.orientation))],float_columns("semio_brep_shell_face"))?;}}
for(ordinal,s)in snapshot.solids.iter().enumerate(){let id=number(ordinal+1)?;p.insert_key_float("semio_brep_solid",id,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&s.id)],float_columns("semio_brep_solid"))?;for(ordinal,l)in s.shells.iter().enumerate(){p.insert_float("semio_brep_solid_shell",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&l.shell),Cell::Integer(i64::from(l.is_void))],float_columns("semio_brep_solid_shell"))?;}}
for(ordinal,e)in snapshot.coedges.iter().enumerate(){let curve=e.pcurve.as_ref().map(|v|project_curve2(v,p)).transpose()?;p.insert_key_float("semio_brep_coedge",number(ordinal+1)?,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&e.id),Cell::Text(&e.edge),Cell::Integer(i64::from(e.forward)),curve.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Real(e.prange.0),Cell::Real(e.prange.1),Cell::Text(&e.loop_id),Cell::Text(&e.next),Cell::Text(&e.prev)],float_columns("semio_brep_coedge"))?;}Ok(())}
/// 🎟️ Admits every typed BRep cell before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioBrepSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(n:usize)->Result<i64,ValueError>{i64::try_from(n).map_err(|e|ValueError::new(ValueRefusalKind::WorkLimit,e.to_string()))}
fn boolean(n:i64)->Result<bool,ValueError>{match n{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid BRep boolean"))}}
fn identity<'a>(r:impl std::borrow::Borrow<SqliteRow<'a>>,n:usize)->Result<(),ValueError>{let r=*r.borrow();if r.rowid<=0||r.integer(0)?!=r.rowid||r.values.len()!=n{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid BRep row identity or columns"))}else{Ok(())}}
fn point3(r:SqliteRow<'_>,i:usize)->Result<SemioPoint3,ValueError>{Ok(SemioPoint3{x:r.real(i)?,y:r.real(i+1)?,z:r.real(i+2)?})}
fn point2(r:SqliteRow<'_>,i:usize)->Result<SemioPoint2,ValueError>{Ok(SemioPoint2{x:r.real(i)?,y:r.real(i+1)?})}
fn put3(c:&mut[Cell<'_>],i:usize,p:SemioPoint3){c[i]=Cell::Real(p.x);c[i+1]=Cell::Real(p.y);c[i+2]=Cell::Real(p.z);}
fn put2(c:&mut[Cell<'_>],i:usize,p:SemioPoint2){c[i]=Cell::Real(p.x);c[i+1]=Cell::Real(p.y);}
fn scalars(table:&str,id:i64,values:&[f64],p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{for(ordinal,v)in values.iter().enumerate(){p.insert_float(table,&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(*v)],float_columns(table))?;}Ok(())}
fn points3(table:&str,id:i64,values:&[SemioPoint3],p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{for(ordinal,v)in values.iter().enumerate(){p.insert_float(table,&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(v.x),Cell::Real(v.y),Cell::Real(v.z)],float_columns(table))?;}Ok(())}
fn project_curve3(curve:&BrepCurve,p:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{let mut c=[Cell::Null;17];match curve{BrepCurve::Line{origin,direction}=>{c[0]=Cell::Text("line");put3(&mut c,1,*origin);put3(&mut c,4,*direction);},BrepCurve::Circle{center,axis,radius}=>{c[0]=Cell::Text("circle");put3(&mut c,7,*center);put3(&mut c,10,*axis);c[13]=Cell::Real(*radius);},BrepCurve::Ellipse{center,axis,radius_major,radius_minor}=>{c[0]=Cell::Text("ellipse");put3(&mut c,7,*center);put3(&mut c,10,*axis);c[14]=Cell::Real(*radius_major);c[15]=Cell::Real(*radius_minor);},BrepCurve::Nurbs{degree,..}=>{c[0]=Cell::Text("nurbs");c[16]=Cell::Integer(i64::from(*degree));}}let id=p.insert_float("semio_brep_curve3",&c,float_columns("semio_brep_curve3"))?;if let BrepCurve::Nurbs{control_points,weights,knots,..}=curve{points3("semio_brep_curve3_point",id,control_points,p)?;scalars("semio_brep_curve3_weight",id,weights,p)?;scalars("semio_brep_curve3_knot",id,knots,p)?;}Ok(id)}
fn project_curve2(curve:&BrepCurve2,p:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{let mut c=[Cell::Null;13];match curve{BrepCurve2::Line{origin,direction}=>{c[0]=Cell::Text("line");put2(&mut c,1,*origin);put2(&mut c,3,*direction);},BrepCurve2::Circle{center,radius}=>{c[0]=Cell::Text("circle");put2(&mut c,5,*center);c[9]=Cell::Real(*radius);},BrepCurve2::Ellipse{center,x_axis,radius_major,radius_minor}=>{c[0]=Cell::Text("ellipse");put2(&mut c,5,*center);put2(&mut c,7,*x_axis);c[10]=Cell::Real(*radius_major);c[11]=Cell::Real(*radius_minor);},BrepCurve2::Nurbs{degree,..}=>{c[0]=Cell::Text("nurbs");c[12]=Cell::Integer(i64::from(*degree));}}let id=p.insert_float("semio_brep_curve2",&c,float_columns("semio_brep_curve2"))?;if let BrepCurve2::Nurbs{control_points,weights,knots,..}=curve{for(ordinal,v)in control_points.iter().enumerate(){p.insert_float("semio_brep_curve2_point",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(v.x),Cell::Real(v.y)],float_columns("semio_brep_curve2_point"))?;}scalars("semio_brep_curve2_weight",id,weights,p)?;scalars("semio_brep_curve2_knot",id,knots,p)?;}Ok(id)}
fn project_surface(surface:&BrepSurface,p:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{let mut c=[Cell::Null;21];match surface{BrepSurface::Plane{origin,normal}=>{c[0]=Cell::Text("plane");put3(&mut c,1,*origin);put3(&mut c,10,*normal);},BrepSurface::Cylinder{origin,axis,radius}=>{c[0]=Cell::Text("cylinder");put3(&mut c,1,*origin);put3(&mut c,4,*axis);c[13]=Cell::Real(*radius);},BrepSurface::Cone{origin,axis,radius,half_angle}=>{c[0]=Cell::Text("cone");put3(&mut c,1,*origin);put3(&mut c,4,*axis);c[13]=Cell::Real(*radius);c[14]=Cell::Real(*half_angle);},BrepSurface::Sphere{center,radius}=>{c[0]=Cell::Text("sphere");put3(&mut c,7,*center);c[13]=Cell::Real(*radius);},BrepSurface::Torus{center,axis,major_radius,minor_radius}=>{c[0]=Cell::Text("torus");put3(&mut c,7,*center);put3(&mut c,4,*axis);c[15]=Cell::Real(*major_radius);c[16]=Cell::Real(*minor_radius);},BrepSurface::Nurbs{u_count,v_count,degree_u,degree_v,..}=>{c[0]=Cell::Text("nurbs");c[17]=Cell::Integer(i64::from(*u_count));c[18]=Cell::Integer(i64::from(*v_count));c[19]=Cell::Integer(i64::from(*degree_u));c[20]=Cell::Integer(i64::from(*degree_v));}}let id=p.insert_float("semio_brep_surface",&c,float_columns("semio_brep_surface"))?;if let BrepSurface::Nurbs{control_points,weights,knots_u,knots_v,..}=surface{points3("semio_brep_surface_point",id,control_points,p)?;scalars("semio_brep_surface_weight",id,weights,p)?;scalars("semio_brep_surface_knot_u",id,knots_u,p)?;scalars("semio_brep_surface_knot_v",id,knots_v,p)?;}Ok(id)}
/// 🚫️ Reports an authored BRep relationship refusal.
fn brep_invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
/// 🛂️ Rejects contradictory analytic fields outside the actual selected variant.
fn shape(row:SqliteRow<'_>,active:&[usize])->Result<(),ValueError>{for index in 2..row.values.len(){if !active.contains(&index)&&!row.is_null(index)?{return Err(brep_invalid("contradictory BRep analytic geometry fields"))}}Ok(())}
impl ArtifactSqliteSnapshot for SemioBrepSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::brep::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::v1::subsets::brep::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control.native())}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="brep"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_brep_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(c)}
fn from_sqlite_database(db:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(c.limits())?;Self::reconstruct_sqlite_database(db, c, Self::SQLITE_SCHEMA) })();result}
}

impl SemioBrepSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_brep_vertex"=>&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)],"semio_brep_edge"=>&[FloatColumn::Binary64(7)],"semio_brep_coedge"=>&[FloatColumn::Binary64(7),FloatColumn::Binary64(8)],"semio_brep_face"=>&[FloatColumn::Binary64(7)],"semio_brep_curve3"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(16)],"semio_brep_curve3_point"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_brep_curve3_weight"=>&[FloatColumn::Binary64(3)],"semio_brep_curve3_knot"=>&[FloatColumn::Binary64(3)],"semio_brep_curve2"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12)],"semio_brep_curve2_point"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_brep_curve2_weight"=>&[FloatColumn::Binary64(3)],"semio_brep_curve2_knot"=>&[FloatColumn::Binary64(3)],"semio_brep_surface"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(16),FloatColumn::Binary64(17)],"semio_brep_surface_point"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_brep_surface_weight"=>&[FloatColumn::Binary64(3)],"semio_brep_surface_knot_u"=>&[FloatColumn::Binary64(3)],"semio_brep_surface_knot_v"=>&[FloatColumn::Binary64(3)],_=>&[]}}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioBrepSnapshot {
/// 📏️ Bounds the full native topological identities and analytic/NURBS components.
pub fn native_fields(&self,b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.text(&self.schema)?;b.scalars(1)?;
b.entities(self.vertices.len())?;for vertex in &self.vertices{b.text(&vertex.id)?;b.scalars(4)?;}
b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id)?;b.text(&edge.start_vertex)?;b.text(&edge.end_vertex)?;b.scalars(1)?;match &edge.curve{BrepCurve::Line{..}=>b.scalars(6)?,BrepCurve::Circle{..}=>b.scalars(7)?,BrepCurve::Ellipse{..}=>b.scalars(8)?,BrepCurve::Nurbs{control_points,weights,knots,..}=>{native_nurbs(control_points.len(),3,weights.len(),knots.len(),b)?;b.scalars(1)?;}}}
b.entities(self.loops.len())?;for item in &self.loops{b.text(&item.id)?;b.entities(item.edges.len())?;for edge in &item.edges{b.text(&edge.edge)?;b.scalars(1)?;}}
b.entities(self.coedges.len())?;for edge in &self.coedges{b.text(&edge.id)?;b.text(&edge.edge)?;b.text(&edge.loop_id)?;b.text(&edge.next)?;b.text(&edge.prev)?;b.scalars(3)?;if let Some(curve)=&edge.pcurve{match curve{BrepCurve2::Line{..}=>b.scalars(4)?,BrepCurve2::Circle{..}=>b.scalars(3)?,BrepCurve2::Ellipse{..}=>b.scalars(6)?,BrepCurve2::Nurbs{control_points,weights,knots,..}=>{native_nurbs(control_points.len(),2,weights.len(),knots.len(),b)?;b.scalars(1)?;}}}}
b.entities(self.faces.len())?;for face in &self.faces{b.text(&face.id)?;b.text(&face.outer_loop)?;b.entities(face.inner_loops.len())?;for item in &face.inner_loops{b.text(item)?;}b.scalars(2)?;match &face.surface{BrepSurface::Plane{..}=>b.scalars(6)?,BrepSurface::Cylinder{..}=>b.scalars(7)?,BrepSurface::Cone{..}=>b.scalars(8)?,BrepSurface::Sphere{..}=>b.scalars(4)?,BrepSurface::Torus{..}=>b.scalars(8)?,BrepSurface::Nurbs{control_points,weights,knots_u,knots_v,..}=>{native_nurbs(control_points.len(),3,weights.len(),knots_u.len(),b)?;b.entities(knots_v.len())?;b.scalars(knots_v.len())?;b.scalars(4)?;}}}
b.entities(self.shells.len())?;for shell in &self.shells{b.text(&shell.id)?;b.entities(shell.faces.len())?;for face in &shell.faces{b.text(&face.face)?;b.scalars(1)?;}}
b.entities(self.solids.len())?;for solid in &self.solids{b.text(&solid.id)?;b.entities(solid.shells.len())?;for shell in &solid.shells{b.text(&shell.shell)?;b.scalars(1)?;}}Ok(())}
}
fn native_nurbs(points:usize,width:usize,weights:usize,knots:usize,b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(points)?;b.scalars(points.checked_mul(width).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"BRep native point scalar overflow"))?)?;b.entities(weights)?;b.scalars(weights)?;b.entities(knots)?;b.scalars(knots)}

impl SemioBrepSnapshot {
    /// 🪶️ Projects the owned subset into its declared relational writer while preserving refusals.
    pub fn project_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::layout(c.limits())?;crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::project_rows_owned(Self::SQLITE_SCHEMA,c,|out|visit_rows(self,out))}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;

#[path="💰️reconstruction/🦀️.rs"]mod reconstruction;
