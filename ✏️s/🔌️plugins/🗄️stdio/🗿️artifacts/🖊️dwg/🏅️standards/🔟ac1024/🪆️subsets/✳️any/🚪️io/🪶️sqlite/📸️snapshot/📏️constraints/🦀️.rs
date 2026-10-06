//! 📏️ DWG 2D constraint nodes retain tagged geometry, dependency state and optional coordinates.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use super::number::Projection;
use super::drawing::optional_words;
use super::associativity::{project_action,reconstruct_action};
use super::blocks::{project_handles,reconstruct_handles};
use super::evaluation::{project_coordinates as coords,reconstruct_coordinates as read_coords};
use super::reader::{boolean,byte,full_unsigned,high,low,optional_unsigned,ordinal,signed_integer,unsigned,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Real as R};

fn core(v:&DwgConstraintNode)->(&'static str,&DwgConstraintNodeCore){match v{
 DwgConstraintNode::ConstrainedImplicitPoint(v)=>("implicit_point",&v.geometry.node),
 DwgConstraintNode::PointCurveConstraint(v)=>("point_curve",&v.node),
 DwgConstraintNode::ConstrainedBoundedLine(v)=>("bounded_line",&v.geometry.node),
 DwgConstraintNode::PointCoincidenceConstraint(v)=>("point_coincidence",&v.node),
 DwgConstraintNode::DistanceConstraint(v)=>("distance",&v.explicit.geometric.node),
 DwgConstraintNode::PerpendicularConstraint(v)=>("perpendicular",&v.node),
 DwgConstraintNode::HorizontalConstraint(v)=>("horizontal",&v.geometric.node),
 DwgConstraintNode::ParallelConstraint(v)=>("parallel",&v.node),
 DwgConstraintNode::MidPointConstraint(v)=>("midpoint",&v.node),
 DwgConstraintNode::EqualLengthConstraint(v)=>("equal_length",&v.node),
 DwgConstraintNode::ColinearConstraint(v)=>("colinear",&v.node),
 DwgConstraintNode::ConstrainedDatumLine(v)=>("datum_line",&v.geometry.node),
 DwgConstraintNode::FixedConstraint(v)=>("fixed",&v.node),
 DwgConstraintNode::VerticalConstraint(v)=>("vertical",&v.geometric.node)
}}
fn geometric(p:&mut Projection<'_,'_>,id:i64,v:&DwgGeometricConstraint)->Result<(),ValueError>{p.insert_key("dwg_geometric_constraint",id,&[I(i64::from(v.owner_node_id)),I(i64::from(v.implied)),I(i64::from(v.active))])}
fn geometry(p:&mut Projection<'_,'_>,id:i64,v:&DwgConstraintGeometry)->Result<(),ValueError>{let handle=optional_words(v.geometry_dependency_handle);p.insert_key("dwg_constraint_geometry",id,&[handle[0],handle[1],I(i64::from(v.geometry_node_id))])}
fn project_vectors(p:&mut Projection<'_,'_>,id:i64,values:&[(&str,&[f64])])->Result<(),ValueError>{
 let mut order=0;for(name,values)in values{for(index,v)in values.iter().enumerate(){p.insert("dwg_constraint_coordinate",&[I(id),T(name),I(order),I(ordinal(index)?),R(*v)])?;order+=1;}}Ok(())
}
pub(super) fn project(p:&mut Projection<'_,'_>,id:i64,v:&DwgAssoc2dConstraintGroup)->Result<(),ValueError>{
 project_action(p,id,&v.action)?;p.insert_key("dwg_assoc_2d_constraint_group",id,&[I(i64::from(v.do_not_check_newly_added_constraints))])?;
 for(index,values)in v.work_plane.iter().enumerate(){let vector=p.insert("dwg_constraint_work_plane_vector",&[I(id),I(ordinal(index)?)])?;coords(p,"dwg_constraint_work_plane_coordinate",vector,values)?;}project_handles(p,"dwg_constraint_group_member_handle",id,&v.member_action_handles)?;
 for(index,v)in v.nodes.iter().enumerate(){
 let(kind,node)=core(v);let id=p.insert("dwg_constraint_node",&[I(id),I(ordinal(index)?),T(kind),I(i64::from(node.id))])?;
 for(index,v)in node.connected_node_ids.iter().enumerate(){p.insert("dwg_constraint_connected_node_identifier",&[I(id),I(ordinal(index)?),I(i64::from(*v))])?;}
 match v{
 DwgConstraintNode::ConstrainedImplicitPoint(v)=>{geometry(p,id,&v.geometry)?;p.insert_key("dwg_constrained_implicit_point",id,&[I(i64::from(v.point.is_some())),I(i64::from(v.point_kind)),I(i64::from(v.point_index)),I(i64::from(v.curve_node_id))])?;if let Some(point)=&v.point{project_vectors(p,id,&[("point",point)])?;}},
 DwgConstraintNode::ConstrainedBoundedLine(v)=>{geometry(p,id,&v.geometry)?;p.insert_key("dwg_constrained_bounded_line",id,&[I(i64::from(v.ray)),I(i64::from(v.bounded))])?;project_vectors(p,id,&[("origin",&v.origin),("direction",&v.direction),("start_point",&v.start_point),("end_point",&v.end_point)])?;},
 DwgConstraintNode::DistanceConstraint(v)=>{geometric(p,id,&v.explicit.geometric)?;p.insert_key("dwg_explicit_constraint",id,&[I(high(v.explicit.value_dependency_handle)),I(low(v.explicit.value_dependency_handle)),I(high(v.explicit.dimension_dependency_handle)),I(low(v.explicit.dimension_dependency_handle))])?;p.insert_key("dwg_distance_constraint",id,&[I(i64::from(v.direction_kind)),I(i64::from(v.direction.is_some()))])?;if let Some(direction)=&v.direction{project_vectors(p,id,&[("direction",direction)])?;}},
 DwgConstraintNode::HorizontalConstraint(v)|DwgConstraintNode::VerticalConstraint(v)=>{geometric(p,id,&v.geometric)?;p.insert_key("dwg_axis_constraint",id,&[I(i64::from(v.datum_line_index))])?;},
 DwgConstraintNode::ConstrainedDatumLine(v)=>{geometry(p,id,&v.geometry)?;p.insert_key("dwg_constrained_datum_line",id,&[])?;project_vectors(p,id,&[("origin",&v.origin),("direction",&v.direction)])?;},
 DwgConstraintNode::PointCurveConstraint(v)|DwgConstraintNode::PointCoincidenceConstraint(v)|DwgConstraintNode::PerpendicularConstraint(v)|DwgConstraintNode::ParallelConstraint(v)|DwgConstraintNode::MidPointConstraint(v)|DwgConstraintNode::EqualLengthConstraint(v)|DwgConstraintNode::ColinearConstraint(v)|DwgConstraintNode::FixedConstraint(v)=>geometric(p,id,v)?
 }
 }Ok(())
}
fn read_core(r:&mut Reader<'_,'_,'_>,row:super::number::Row<'_>)->Result<DwgConstraintNodeCore,ValueError>{Ok(DwgConstraintNodeCore{id:signed_integer(row,4)?,connected_node_ids:r.list("dwg_constraint_connected_node_identifier",1,row.rowid,2)?.into_iter().map(|row|unsigned(row,3)).collect::<Result<_,_>>()?})}
fn read_geometric(r:&mut Reader<'_,'_,'_>,id:i64,node:DwgConstraintNodeCore)->Result<DwgGeometricConstraint,ValueError>{let row=r.component("dwg_geometric_constraint",id)?;Ok(DwgGeometricConstraint{node,owner_node_id:unsigned(row,1)?,implied:boolean(row,2)?,active:boolean(row,3)?})}
fn read_geometry(r:&mut Reader<'_,'_,'_>,id:i64,node:DwgConstraintNodeCore)->Result<DwgConstraintGeometry,ValueError>{let row=r.component("dwg_constraint_geometry",id)?;Ok(DwgConstraintGeometry{node,geometry_dependency_handle:optional_unsigned(row,1,2)?,geometry_node_id:unsigned(row,3)?})}
fn read_node(r:&mut Reader<'_,'_,'_>,row:super::number::Row<'_>)->Result<DwgConstraintNode,ValueError>{
 let id=row.rowid;let kind=row.text(3)?;let node=read_core(r,row)?;let mut vectors:[Vec<f64>;5]=std::array::from_fn(|_|Vec::new());let mut prior=0;
 for row in r.list("dwg_constraint_coordinate",1,id,3)?{
 let index=match row.text(2)?{"point"=>0,"origin"=>1,"direction"=>2,"start_point"=>3,"end_point"=>4,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG constraint coordinate vector is unknown"))};
 if index<prior||row.integer(4)?!=ordinal(vectors[index].len())?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG constraint coordinate vectors are out of order"));}prior=index;vectors[index].push(row.real(5)?);
 }
 let[point,origin,direction,start_point,end_point]=vectors;
 let allowed=match kind{"implicit_point"=>[true,false,false,false,false],"bounded_line"=>[false,true,true,true,true],"distance"=>[false,false,true,false,false],"datum_line"=>[false,true,true,false,false],_=>[false;5]};
 for(index,values)in [&point,&origin,&direction,&start_point,&end_point].into_iter().enumerate(){if !allowed[index]&&!values.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG constraint coordinates have the wrong node kind"));}}
 Ok(match kind{
 "implicit_point"=>{let row=r.component("dwg_constrained_implicit_point",id)?;let present=boolean(row,1)?;if !present&&!point.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG absent implicit point contains coordinates"));}DwgConstraintNode::ConstrainedImplicitPoint(DwgConstrainedImplicitPoint{geometry:read_geometry(r,id,node)?,point:present.then_some(point),point_kind:byte(row,2)?,point_index:signed_integer(row,3)?,curve_node_id:signed_integer(row,4)?})},
 "bounded_line"=>{let row=r.component("dwg_constrained_bounded_line",id)?;DwgConstraintNode::ConstrainedBoundedLine(DwgConstrainedBoundedLine{geometry:read_geometry(r,id,node)?,origin,direction,ray:boolean(row,1)?,bounded:boolean(row,2)?,start_point,end_point})},
 "distance"=>{let row=r.component("dwg_distance_constraint",id)?;let explicit=r.component("dwg_explicit_constraint",id)?;let present=boolean(row,2)?;if !present&&!direction.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG absent distance direction contains coordinates"));}DwgConstraintNode::DistanceConstraint(DwgDistanceConstraint{explicit:DwgExplicitConstraint{geometric:read_geometric(r,id,node)?,value_dependency_handle:full_unsigned(explicit,1,2)?,dimension_dependency_handle:full_unsigned(explicit,3,4)?},direction_kind:byte(row,1)?,direction:present.then_some(direction)})},
 "horizontal"|"vertical"=>{let row=r.component("dwg_axis_constraint",id)?;let v=DwgAxisConstraint{geometric:read_geometric(r,id,node)?,datum_line_index:signed_integer(row,1)?};if kind=="horizontal"{DwgConstraintNode::HorizontalConstraint(v)}else{DwgConstraintNode::VerticalConstraint(v)}},
 "datum_line"=>{r.component("dwg_constrained_datum_line",id)?;DwgConstraintNode::ConstrainedDatumLine(DwgConstrainedDatumLine{geometry:read_geometry(r,id,node)?,origin,direction})},
 "point_curve"=>DwgConstraintNode::PointCurveConstraint(read_geometric(r,id,node)?),
 "point_coincidence"=>DwgConstraintNode::PointCoincidenceConstraint(read_geometric(r,id,node)?),
 "perpendicular"=>DwgConstraintNode::PerpendicularConstraint(read_geometric(r,id,node)?),
 "parallel"=>DwgConstraintNode::ParallelConstraint(read_geometric(r,id,node)?),
 "midpoint"=>DwgConstraintNode::MidPointConstraint(read_geometric(r,id,node)?),
 "equal_length"=>DwgConstraintNode::EqualLengthConstraint(read_geometric(r,id,node)?),
 "colinear"=>DwgConstraintNode::ColinearConstraint(read_geometric(r,id,node)?),
 "fixed"=>DwgConstraintNode::FixedConstraint(read_geometric(r,id,node)?),
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG constraint node kind is unknown"))
 })
}
pub(super) fn reconstruct(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssoc2dConstraintGroup,ValueError>{
 let row=r.component("dwg_assoc_2d_constraint_group",id)?;
 let work_plane=r.list("dwg_constraint_work_plane_vector",1,id,2)?.into_iter().map(|row|read_coords(r,"dwg_constraint_work_plane_coordinate",row.rowid)).collect::<Result<_,_>>()?;
 let nodes=r.list("dwg_constraint_node",1,id,2)?.into_iter().map(|row|read_node(r,row)).collect::<Result<_,_>>()?;
 Ok(DwgAssoc2dConstraintGroup{action:reconstruct_action(r,id)?,do_not_check_newly_added_constraints:boolean(row,1)?,work_plane,member_action_handles:reconstruct_handles(r,"dwg_constraint_group_member_handle",id)?,nodes})
}
