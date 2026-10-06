//! 🎬️ DWG block actions preserve connections, ownership and ordered stretch topology.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use super::number::{Projection,Row};
use super::evaluation::{project_expression,reconstruct_expression,project_coordinates as coords,reconstruct_coordinates as read_coords};
use super::reader::{boolean,full_unsigned,high,low,ordinal,unsigned,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Real as R};

fn connection(row:Row<'_>,column:usize)->Result<DwgBlockActionConnection,ValueError>{Ok(DwgBlockActionConnection{node_id:unsigned(row,column)?,name:row.text(column+1)?.into()})}
fn project_indices(p:&mut Projection<'_,'_>,table:&'static str,id:i64,values:&[u32])->Result<(),ValueError>{for(index,value)in values.iter().enumerate(){p.insert(table,&[I(id),I(ordinal(index)?),I(i64::from(*value))])?;}Ok(())}
fn reconstruct_indices(r:&mut Reader<'_,'_,'_>,table:&'static str,id:i64)->Result<Vec<u32>,ValueError>{r.list(table,1,id,2)?.into_iter().map(|row|unsigned(row,3)).collect()}
fn project_action(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockAction)->Result<(),ValueError>{
 project_expression(p,id,&v.evaluation_expression)?;p.insert_key("dwg_block_action",id,&[T(&v.name)])?;coords(p,"dwg_block_action_display_coordinate",id,&v.display_location)?;
 for(index,v)in v.dependencies.iter().enumerate(){p.insert("dwg_block_action_dependency",&[I(id),I(ordinal(index)?),I(high(v.object_handle)),I(low(v.object_handle))])?;}
 project_indices(p,"dwg_block_action_node_identifier",id,&v.action_node_ids)
}
fn reconstruct_action(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockAction,ValueError>{
 let row=r.component("dwg_block_action",id)?;
 Ok(DwgBlockAction{evaluation_expression:reconstruct_expression(r,id)?,name:row.text(1)?.into(),display_location:read_coords(r,"dwg_block_action_display_coordinate",id)?,dependencies:r.list("dwg_block_action_dependency",1,id,2)?.into_iter().map(|row|Ok(DwgBlockActionDependency{object_handle:full_unsigned(row,3,4)?})).collect::<Result<_,ValueError>>()?,action_node_ids:reconstruct_indices(r,"dwg_block_action_node_identifier",id)?})
}
pub(super) fn project_move(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockMoveAction)->Result<(),ValueError>{
 project_action(p,id,&v.action)?;let mode=match v.coordinate_mode{DwgBlockMoveCoordinateMode::CartesianXy=>"cartesian_xy"};
 p.insert_key("dwg_block_move_action",id,&[I(i64::from(v.x_connection.node_id)),T(&v.x_connection.name),I(i64::from(v.y_connection.node_id)),T(&v.y_connection.name),R(v.distance_multiplier),R(v.angle_offset),T(mode)])
}
pub(super) fn reconstruct_move(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockMoveAction,ValueError>{
 let row=r.component("dwg_block_move_action",id)?;if row.text(7)?!="cartesian_xy"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG move coordinate mode is unknown"));}
 Ok(DwgBlockMoveAction{action:reconstruct_action(r,id)?,x_connection:connection(row,1)?,y_connection:connection(row,3)?,distance_multiplier:row.real(5)?,angle_offset:row.real(6)?,coordinate_mode:DwgBlockMoveCoordinateMode::CartesianXy})
}
pub(super) fn project_stretch(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockStretchAction)->Result<(),ValueError>{
 project_action(p,id,&v.action)?;let mode=match v.coordinate_mode{DwgBlockActionCoordinateMode::CartesianXy=>"cartesian_xy"};
 p.insert_key("dwg_block_stretch_action",id,&[I(i64::from(v.x_connection.node_id)),T(&v.x_connection.name),I(i64::from(v.y_connection.node_id)),T(&v.y_connection.name),R(v.distance_multiplier),R(v.angle_offset),T(mode)])?;
 for(index,v)in v.points.iter().enumerate(){let point=p.insert("dwg_stretch_point",&[I(id),I(ordinal(index)?)])?;coords(p,"dwg_stretch_point_coordinate",point,v)?;}
 for(index,v)in v.selections.iter().enumerate(){let selection=p.insert("dwg_stretch_selection",&[I(id),I(ordinal(index)?),I(high(v.object_handle)),I(low(v.object_handle))])?;project_indices(p,"dwg_stretch_selection_vertex_index",selection,&v.vertex_indices)?;}
 for(index,v)in v.selectors.iter().enumerate(){let selector=p.insert("dwg_stretch_selector",&[I(id),I(ordinal(index)?),I(i64::from(v.node_id))])?;project_indices(p,"dwg_stretch_selector_point_index",selector,&v.point_indices)?;}Ok(())
}
pub(super) fn reconstruct_stretch(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockStretchAction,ValueError>{
 let row=r.component("dwg_block_stretch_action",id)?;if row.text(7)?!="cartesian_xy"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG stretch coordinate mode is unknown"));}
 let points=r.list("dwg_stretch_point",1,id,2)?.into_iter().map(|row|read_coords(r,"dwg_stretch_point_coordinate",row.rowid)).collect::<Result<_,_>>()?;
 let selections=r.list("dwg_stretch_selection",1,id,2)?.into_iter().map(|row|Ok(DwgStretchSelection{object_handle:full_unsigned(row,3,4)?,vertex_indices:reconstruct_indices(r,"dwg_stretch_selection_vertex_index",row.rowid)?})).collect::<Result<_,ValueError>>()?;
 let selectors=r.list("dwg_stretch_selector",1,id,2)?.into_iter().map(|row|Ok(DwgStretchSelector{node_id:unsigned(row,3)?,point_indices:reconstruct_indices(r,"dwg_stretch_selector_point_index",row.rowid)?})).collect::<Result<_,ValueError>>()?;
 Ok(DwgBlockStretchAction{action:reconstruct_action(r,id)?,x_connection:connection(row,1)?,y_connection:connection(row,3)?,distance_multiplier:row.real(5)?,angle_offset:row.real(6)?,coordinate_mode:DwgBlockActionCoordinateMode::CartesianXy,points,selections,selectors})
}
fn project_base(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockActionWithBasePoint)->Result<(),ValueError>{
 project_action(p,id,&v.action)?;p.insert_key("dwg_block_action_base_point",id,&[I(i64::from(v.x_base_connection.node_id)),T(&v.x_base_connection.name),I(i64::from(v.y_base_connection.node_id)),T(&v.y_base_connection.name),I(i64::from(v.dependent))])?;coords(p,"dwg_block_action_offset_coordinate",id,&v.offset)?;coords(p,"dwg_block_action_base_point_coordinate",id,&v.base_point)
}
fn reconstruct_base(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockActionWithBasePoint,ValueError>{
 let row=r.component("dwg_block_action_base_point",id)?;
 Ok(DwgBlockActionWithBasePoint{action:reconstruct_action(r,id)?,offset:read_coords(r,"dwg_block_action_offset_coordinate",id)?,x_base_connection:connection(row,1)?,y_base_connection:connection(row,3)?,dependent:boolean(row,5)?,base_point:read_coords(r,"dwg_block_action_base_point_coordinate",id)?})
}
pub(super) fn project_scale(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockScaleAction)->Result<(),ValueError>{
 project_base(p,id,&v.base)?;let mode=match v.mode{DwgBlockScaleMode::Xy=>"xy"};
 p.insert_key("dwg_block_scale_action",id,&[I(i64::from(v.uniform_scale_connection.node_id)),T(&v.uniform_scale_connection.name),I(i64::from(v.x_scale_connection.node_id)),T(&v.x_scale_connection.name),I(i64::from(v.y_scale_connection.node_id)),T(&v.y_scale_connection.name),T(mode)])
}
pub(super) fn reconstruct_scale(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockScaleAction,ValueError>{
 let row=r.component("dwg_block_scale_action",id)?;if row.text(7)?!="xy"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG block scale mode is unknown"));}
 Ok(DwgBlockScaleAction{base:reconstruct_base(r,id)?,uniform_scale_connection:connection(row,1)?,x_scale_connection:connection(row,3)?,y_scale_connection:connection(row,5)?,mode:DwgBlockScaleMode::Xy})
}
pub(super) fn project_flip(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockFlipAction)->Result<(),ValueError>{
 project_action(p,id,&v.action)?;
 p.insert_key("dwg_block_flip_action",id,&[I(i64::from(v.flip_connection.node_id)),T(&v.flip_connection.name),I(i64::from(v.updated_flip_connection.node_id)),T(&v.updated_flip_connection.name),I(i64::from(v.updated_base_connection.node_id)),T(&v.updated_base_connection.name),I(i64::from(v.updated_end_connection.node_id)),T(&v.updated_end_connection.name)])
}
pub(super) fn reconstruct_flip(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockFlipAction,ValueError>{
 let row=r.component("dwg_block_flip_action",id)?;
 Ok(DwgBlockFlipAction{action:reconstruct_action(r,id)?,flip_connection:connection(row,1)?,updated_flip_connection:connection(row,3)?,updated_base_connection:connection(row,5)?,updated_end_connection:connection(row,7)?})
}
