//! 🧊️ Writes all BRep topology and analytic/spline geometry directly under cumulative native admission.
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2,SemioPoint3};
use crate::standards::v1::subsets::brep::schema::snapshot::{SemioBrepSnapshot,BrepCurve,BrepCurve2,BrepSurface};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_encoding::{self,Writer};
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl};
pub(crate) fn encode(value:&SemioBrepSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{native_encoding::encode(encoding,"semio stdio.semio.brep.dsl v1\n","stdio.semio.brep.pack v1",control,|writer,encoding|fields(value,writer,encoding))}
fn comma(w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{w.delimiter(b",",e)}
fn boolean(value:bool,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{w.byte(if e==SnapshotEncoding::Binary{u8::from(value)}else{if value{b'1'}else{b'0'}})}
fn point2(value:&SemioPoint2,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{w.delimiter(b"[",e)?;w.number(value.x,e)?;comma(w,e)?;w.number(value.y,e)?;w.delimiter(b"]",e)}
fn point3(value:&SemioPoint3,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{w.delimiter(b"[",e)?;for(index,value)in[value.x,value.y,value.z].into_iter().enumerate(){if index!=0{comma(w,e)?}w.number(value,e)?}w.delimiter(b"]",e)}
fn numbers(values:&[f64],w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{w.list(values,e,|w,value|w.number(*value,e))}
fn tag(binary:u8,text:u8,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{w.entities(1)?;w.byte(if e==SnapshotEncoding::Binary{binary}else{text})?;w.delimiter(b"[",e)}
fn curve(value:&BrepCurve,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{match value{
 BrepCurve::Line{origin,direction}=>{tag(0,b'L',w,e)?;point3(origin,w,e)?;comma(w,e)?;point3(direction,w,e)?},
 BrepCurve::Circle{center,axis,radius}=>{tag(1,b'C',w,e)?;point3(center,w,e)?;comma(w,e)?;point3(axis,w,e)?;comma(w,e)?;w.number(*radius,e)?},
 BrepCurve::Ellipse{center,axis,radius_major,radius_minor}=>{tag(2,b'E',w,e)?;point3(center,w,e)?;comma(w,e)?;point3(axis,w,e)?;comma(w,e)?;w.number(*radius_major,e)?;comma(w,e)?;w.number(*radius_minor,e)?},
 BrepCurve::Nurbs{control_points,weights,degree,knots}=>{tag(3,b'N',w,e)?;w.list(control_points,e,|w,value|point3(value,w,e))?;comma(w,e)?;numbers(weights,w,e)?;comma(w,e)?;w.integer(u64::from(*degree),e)?;comma(w,e)?;numbers(knots,w,e)?}
 }w.delimiter(b"]",e)}
fn curve2(value:&BrepCurve2,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{match value{
 BrepCurve2::Line{origin,direction}=>{tag(0,b'L',w,e)?;point2(origin,w,e)?;comma(w,e)?;point2(direction,w,e)?},
 BrepCurve2::Circle{center,radius}=>{tag(1,b'C',w,e)?;point2(center,w,e)?;comma(w,e)?;w.number(*radius,e)?},
 BrepCurve2::Ellipse{center,x_axis,radius_major,radius_minor}=>{tag(2,b'E',w,e)?;point2(center,w,e)?;comma(w,e)?;point2(x_axis,w,e)?;comma(w,e)?;w.number(*radius_major,e)?;comma(w,e)?;w.number(*radius_minor,e)?},
 BrepCurve2::Nurbs{control_points,weights,degree,knots}=>{tag(3,b'N',w,e)?;w.list(control_points,e,|w,value|point2(value,w,e))?;comma(w,e)?;numbers(weights,w,e)?;comma(w,e)?;w.integer(u64::from(*degree),e)?;comma(w,e)?;numbers(knots,w,e)?}
 }w.delimiter(b"]",e)}
fn surface(value:&BrepSurface,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{match value{
 BrepSurface::Plane{origin,normal}=>{tag(0,b'P',w,e)?;point3(origin,w,e)?;comma(w,e)?;point3(normal,w,e)?},
 BrepSurface::Cylinder{origin,axis,radius}=>{tag(1,b'C',w,e)?;point3(origin,w,e)?;comma(w,e)?;point3(axis,w,e)?;comma(w,e)?;w.number(*radius,e)?},
 BrepSurface::Cone{origin,axis,radius,half_angle}=>{tag(2,b'O',w,e)?;point3(origin,w,e)?;comma(w,e)?;point3(axis,w,e)?;comma(w,e)?;w.number(*radius,e)?;comma(w,e)?;w.number(*half_angle,e)?},
 BrepSurface::Sphere{center,radius}=>{tag(3,b'S',w,e)?;point3(center,w,e)?;comma(w,e)?;w.number(*radius,e)?},
 BrepSurface::Torus{center,axis,major_radius,minor_radius}=>{tag(4,b'T',w,e)?;point3(center,w,e)?;comma(w,e)?;point3(axis,w,e)?;comma(w,e)?;w.number(*major_radius,e)?;comma(w,e)?;w.number(*minor_radius,e)?},
 BrepSurface::Nurbs{control_points,weights,u_count,v_count,degree_u,degree_v,knots_u,knots_v}=>{tag(5,b'N',w,e)?;w.list(control_points,e,|w,value|point3(value,w,e))?;comma(w,e)?;numbers(weights,w,e)?;for value in[u_count,v_count,degree_u,degree_v]{comma(w,e)?;w.integer(u64::from(*value),e)?}comma(w,e)?;numbers(knots_u,w,e)?;comma(w,e)?;numbers(knots_v,w,e)?}
 }w.delimiter(b"]",e)}
pub(crate) fn fields(value:&SemioBrepSnapshot,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{
 w.entities(1)?;if e==SnapshotEncoding::Binary{w.byte(1)?}else{w.bytes(b"schema=")?}w.string(&value.schema,e)?;
 w.delimiter(b"\nvertices=",e)?;w.list(&value.vertices,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;point3(&value.point,w,e)?;comma(w,e)?;w.number(value.tol,e)?;w.delimiter(b"]",e)})?;
 w.delimiter(b"\nedges=",e)?;w.list(&value.edges,e,|w,value|{w.delimiter(b"[",e)?;for(index,value)in[&value.id,&value.start_vertex,&value.end_vertex].into_iter().enumerate(){if index!=0{comma(w,e)?}w.string(value,e)?}comma(w,e)?;curve(&value.curve,w,e)?;comma(w,e)?;w.number(value.tol,e)?;w.delimiter(b"]",e)})?;
 w.delimiter(b"\nloops=",e)?;w.list(&value.loops,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;w.list(&value.edges,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.edge,e)?;comma(w,e)?;boolean(value.orientation,w,e)?;w.delimiter(b"]",e)})?;w.delimiter(b"]",e)})?;
 w.delimiter(b"\nfaces=",e)?;w.list(&value.faces,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;w.string(&value.outer_loop,e)?;comma(w,e)?;w.list(&value.inner_loops,e,|w,value|{w.delimiter(b"[",e)?;w.string(value,e)?;w.delimiter(b"]",e)})?;comma(w,e)?;surface(&value.surface,w,e)?;comma(w,e)?;boolean(value.orientation,w,e)?;comma(w,e)?;w.number(value.tol,e)?;w.delimiter(b"]",e)})?;
 w.delimiter(b"\nshells=",e)?;w.list(&value.shells,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;w.list(&value.faces,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.face,e)?;comma(w,e)?;boolean(value.orientation,w,e)?;w.delimiter(b"]",e)})?;w.delimiter(b"]",e)})?;
 w.delimiter(b"\nsolids=",e)?;w.list(&value.solids,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;w.list(&value.shells,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.shell,e)?;comma(w,e)?;boolean(value.is_void,w,e)?;w.delimiter(b"]",e)})?;w.delimiter(b"]",e)})?;
 w.delimiter(b"\ncoedges=",e)?;w.list(&value.coedges,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;w.string(&value.edge,e)?;comma(w,e)?;boolean(value.forward,w,e)?;comma(w,e)?;match &value.pcurve{None=>if e==SnapshotEncoding::Binary{w.byte(0)?}else{w.byte(b'-')?},Some(value)=>{w.byte(if e==SnapshotEncoding::Binary{1}else{b'~'})?;curve2(value,w,e)?}}comma(w,e)?;w.delimiter(b"[",e)?;w.number(value.prange.0,e)?;comma(w,e)?;w.number(value.prange.1,e)?;w.delimiter(b"]",e)?;for value in[&value.loop_id,&value.next,&value.prev]{comma(w,e)?;w.string(value,e)?}w.delimiter(b"]",e)})?;
 w.delimiter(b"\nnextLabel=",e)?;w.integer(value.next_label,e)
}
