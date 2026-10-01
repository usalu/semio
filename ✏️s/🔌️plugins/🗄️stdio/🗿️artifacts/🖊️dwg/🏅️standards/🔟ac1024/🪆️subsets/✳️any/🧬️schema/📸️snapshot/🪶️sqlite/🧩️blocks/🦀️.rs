//! 🧩️ DWG dynamic block elements, grips and parameters with explicit domain ownership.
use super::super::*;
use super::number::Projection;
use super::evaluation::{project_expression,reconstruct_expression,project_coordinates as coords,reconstruct_coordinates as read_coords};
use super::reader::{boolean,full_unsigned,high,low,ordinal,signed_integer,unsigned,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Real as R};

fn location(value:DwgBlockParameterBaseLocation)->&'static str{match value{DwgBlockParameterBaseLocation::StartPoint=>"start_point",DwgBlockParameterBaseLocation::Midpoint=>"midpoint"}}
fn read_location(value:&str)->Result<DwgBlockParameterBaseLocation,String>{match value{"start_point"=>Ok(DwgBlockParameterBaseLocation::StartPoint),"midpoint"=>Ok(DwgBlockParameterBaseLocation::Midpoint),_=>Err("DWG block parameter base location is unknown".into())}}
pub(super) fn project_element(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockElement)->Result<(),String>{project_expression(p,id,&v.evaluation_expression)?;p.insert_key("dwg_block_element",id,&[T(&v.name)])}
pub(super) fn reconstruct_element(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockElement,String>{let row=r.component("dwg_block_element",id)?;Ok(DwgBlockElement{evaluation_expression:reconstruct_expression(r,id)?,name:row.text(1)?.into()})}
pub(super) fn project_grip(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockGrip)->Result<(),String>{
 project_element(p,id,&v.element)?;
 p.insert_key("dwg_block_grip",id,&[I(i64::from(v.insertion_cycling)),I(i64::from(v.insertion_cycling_weight)),I(i64::from(v.updated_x.node_id)),T(&v.updated_x.expression_name),I(i64::from(v.updated_y.node_id)),T(&v.updated_y.expression_name)])?;
 coords(p,"dwg_block_grip_location_coordinate",id,&v.location)
}
pub(super) fn reconstruct_grip(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockGrip,String>{
 let row=r.component("dwg_block_grip",id)?;
 Ok(DwgBlockGrip{element:reconstruct_element(r,id)?,location:read_coords(r,"dwg_block_grip_location_coordinate",id)?,insertion_cycling:boolean(row,1)?,insertion_cycling_weight:signed_integer(row,2)?,updated_x:DwgNamedEvaluationNodeReference{node_id:unsigned(row,3)?,expression_name:row.text(4)?.into()},updated_y:DwgNamedEvaluationNodeReference{node_id:unsigned(row,5)?,expression_name:row.text(6)?.into()}})
}
fn project_properties(p:&mut Projection<'_,'_>,id:i64,values:&[DwgBlockParameterProperty])->Result<(),String>{
 for(index,value)in values.iter().enumerate(){let property=p.insert("dwg_block_parameter_property",&[I(id),I(ordinal(index)?)])?;
 for(index,value)in value.connections.iter().enumerate(){p.insert("dwg_block_parameter_connection",&[I(property),I(ordinal(index)?),I(i64::from(value.code)),T(&value.name)])?;}}Ok(())
}
fn reconstruct_properties(r:&mut Reader<'_,'_,'_>,id:i64)->Result<Vec<DwgBlockParameterProperty>,String>{
 r.list("dwg_block_parameter_property",1,id,2)?.into_iter().map(|row|Ok(DwgBlockParameterProperty{connections:r.list("dwg_block_parameter_connection",1,row.rowid,2)?.into_iter().map(|row|Ok(DwgBlockParameterConnection{code:unsigned(row,3)?,name:row.text(4)?.into()})).collect::<Result<_,String>>()?})).collect()
}
pub(super) fn project_two_point(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockTwoPointParameter)->Result<(),String>{
 project_element(p,id,&v.element)?;
 p.insert_key("dwg_block_two_point_parameter",id,&[I(i64::from(v.show_properties)),I(i64::from(v.chain_actions)),T(location(v.base_location))])?;
 coords(p,"dwg_block_two_point_definition_base_coordinate",id,&v.definition_base)?;coords(p,"dwg_block_two_point_definition_end_coordinate",id,&v.definition_end)?;project_properties(p,id,&v.properties)?;
 for(index,v)in v.property_expression_references.iter().enumerate(){p.insert("dwg_property_expression_reference",&[I(id),I(ordinal(index)?),I(i64::from(v.property_index)),I(i64::from(v.node_id))])?;}Ok(())
}
pub(super) fn reconstruct_two_point(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockTwoPointParameter,String>{
 let row=r.component("dwg_block_two_point_parameter",id)?;
 Ok(DwgBlockTwoPointParameter{element:reconstruct_element(r,id)?,show_properties:boolean(row,1)?,chain_actions:boolean(row,2)?,base_location:read_location(row.text(3)?)?,definition_base:read_coords(r,"dwg_block_two_point_definition_base_coordinate",id)?,definition_end:read_coords(r,"dwg_block_two_point_definition_end_coordinate",id)?,properties:reconstruct_properties(r,id)?,property_expression_references:r.list("dwg_property_expression_reference",1,id,2)?.into_iter().map(|row|Ok(DwgPropertyExpressionReference{property_index:unsigned(row,3)?,node_id:unsigned(row,4)?})).collect::<Result<_,String>>()?})
}
fn project_one_point(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockOnePointParameter)->Result<(),String>{
 project_element(p,id,&v.element)?;p.insert_key("dwg_block_one_point_parameter",id,&[I(i64::from(v.show_properties)),I(i64::from(v.chain_actions))])?;
 coords(p,"dwg_block_one_point_definition_coordinate",id,&v.definition_point)?;project_properties(p,id,&v.properties)
}
fn reconstruct_one_point(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockOnePointParameter,String>{
 let row=r.component("dwg_block_one_point_parameter",id)?;
 Ok(DwgBlockOnePointParameter{element:reconstruct_element(r,id)?,show_properties:boolean(row,1)?,chain_actions:boolean(row,2)?,definition_point:read_coords(r,"dwg_block_one_point_definition_coordinate",id)?,properties:reconstruct_properties(r,id)?})
}
pub(super) fn project_linear_parameter(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockLinearParameter)->Result<(),String>{
 project_two_point(p,id,&v.parameter)?;p.insert_key("dwg_block_linear_parameter",id,&[T(&v.distance_name),T(&v.distance_description),R(v.label_offset)])?;coords(p,"dwg_block_linear_allowed_value",id,&v.allowed_values)
}
pub(super) fn reconstruct_linear_parameter(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockLinearParameter,String>{
 let row=r.component("dwg_block_linear_parameter",id)?;Ok(DwgBlockLinearParameter{parameter:reconstruct_two_point(r,id)?,distance_name:row.text(1)?.into(),distance_description:row.text(2)?.into(),label_offset:row.real(3)?,allowed_values:read_coords(r,"dwg_block_linear_allowed_value",id)?})
}
pub(super) fn project_linear_grip(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockLinearGrip)->Result<(),String>{project_grip(p,id,&v.grip)?;p.insert_key("dwg_block_linear_grip",id,&[])?;coords(p,"dwg_block_linear_grip_orientation_coordinate",id,&v.orientation)}
pub(super) fn reconstruct_linear_grip(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockLinearGrip,String>{r.component("dwg_block_linear_grip",id)?;Ok(DwgBlockLinearGrip{grip:reconstruct_grip(r,id)?,orientation:read_coords(r,"dwg_block_linear_grip_orientation_coordinate",id)?})}
pub(super) fn project_flip_grip(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockFlipGrip)->Result<(),String>{project_grip(p,id,&v.grip)?;p.insert_key("dwg_block_flip_grip",id,&[I(i64::from(v.updated_flip.node_id)),T(&v.updated_flip.expression_name)])?;coords(p,"dwg_block_flip_grip_orientation_coordinate",id,&v.orientation)}
pub(super) fn reconstruct_flip_grip(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockFlipGrip,String>{let row=r.component("dwg_block_flip_grip",id)?;Ok(DwgBlockFlipGrip{grip:reconstruct_grip(r,id)?,updated_flip:DwgNamedEvaluationNodeReference{node_id:unsigned(row,1)?,expression_name:row.text(2)?.into()},orientation:read_coords(r,"dwg_block_flip_grip_orientation_coordinate",id)?})}
pub(super) fn project_visibility_grip(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockVisibilityGrip)->Result<(),String>{project_grip(p,id,&v.grip)?;p.insert_key("dwg_block_visibility_grip",id,&[])}
pub(super) fn reconstruct_visibility_grip(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockVisibilityGrip,String>{r.component("dwg_block_visibility_grip",id)?;Ok(DwgBlockVisibilityGrip{grip:reconstruct_grip(r,id)?})}
pub(super) fn project_alignment_parameter(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockAlignmentParameter)->Result<(),String>{project_two_point(p,id,&v.parameter)?;p.insert_key("dwg_block_alignment_parameter",id,&[I(i64::from(v.updated_grip_node_id)),I(i64::from(v.align_perpendicular))])}
pub(super) fn reconstruct_alignment_parameter(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockAlignmentParameter,String>{let row=r.component("dwg_block_alignment_parameter",id)?;Ok(DwgBlockAlignmentParameter{parameter:reconstruct_two_point(r,id)?,updated_grip_node_id:unsigned(row,1)?,align_perpendicular:boolean(row,2)?})}
pub(super) fn project_alignment_grip(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockAlignmentGrip)->Result<(),String>{project_grip(p,id,&v.grip)?;p.insert_key("dwg_block_alignment_grip",id,&[I(i64::from(v.first_location_node_id)),I(i64::from(v.second_location_node_id))])?;coords(p,"dwg_block_alignment_grip_orientation_coordinate",id,&v.orientation)}
pub(super) fn reconstruct_alignment_grip(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockAlignmentGrip,String>{let row=r.component("dwg_block_alignment_grip",id)?;Ok(DwgBlockAlignmentGrip{grip:reconstruct_grip(r,id)?,first_location_node_id:unsigned(row,1)?,second_location_node_id:unsigned(row,2)?,orientation:read_coords(r,"dwg_block_alignment_grip_orientation_coordinate",id)?})}
pub(super) fn project_base_point(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockBasePointParameter)->Result<(),String>{project_one_point(p,id,&v.parameter)?;p.insert_key("dwg_block_base_point_parameter",id,&[])?;coords(p,"dwg_block_base_point_coordinate",id,&v.point)?;coords(p,"dwg_block_base_point_base_coordinate",id,&v.base_point)}
pub(super) fn reconstruct_base_point(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockBasePointParameter,String>{r.component("dwg_block_base_point_parameter",id)?;Ok(DwgBlockBasePointParameter{parameter:reconstruct_one_point(r,id)?,point:read_coords(r,"dwg_block_base_point_coordinate",id)?,base_point:read_coords(r,"dwg_block_base_point_base_coordinate",id)?})}
pub(super) fn project_constraint_parameter(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockLinearConstraintParameter,direction:&str)->Result<(),String>{project_two_point(p,id,&v.parameter)?;p.insert_key("dwg_block_linear_constraint_parameter",id,&[T(direction),I(i64::from(v.displacement_grip_node_id)),I(high(v.dependency_handle)),I(low(v.dependency_handle)),T(&v.expression_name),T(&v.expression_description),R(v.value)])?;coords(p,"dwg_block_linear_constraint_allowed_value",id,&v.allowed_values.values)}
pub(super) fn reconstruct_constraint_parameter(r:&mut Reader<'_,'_,'_>,id:i64,direction:&str)->Result<DwgBlockLinearConstraintParameter,String>{let row=r.component("dwg_block_linear_constraint_parameter",id)?;if row.text(1)?!=direction{return Err("DWG constraint parameter direction differs from object kind".into());}Ok(DwgBlockLinearConstraintParameter{parameter:reconstruct_two_point(r,id)?,displacement_grip_node_id:unsigned(row,2)?,dependency_handle:full_unsigned(row,3,4)?,expression_name:row.text(5)?.into(),expression_description:row.text(6)?.into(),value:row.real(7)?,allowed_values:DwgBlockParameterAllowedValues{values:read_coords(r,"dwg_block_linear_constraint_allowed_value",id)?}})}
pub(super) fn project_flip_parameter(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockFlipParameter)->Result<(),String>{
 project_expression(p,id,&v.evaluation_expression)?;p.insert_key("dwg_block_flip_parameter",id,&[T(&v.name),I(i64::from(v.show_properties)),I(i64::from(v.chain_actions)),T(location(v.base_location)),T(&v.label),T(&v.description),T(&v.value_set.base_label),T(&v.value_set.flipped_label),I(i64::from(v.updated_flip.node_id)),T(&v.updated_flip.expression_name)])?;
 coords(p,"dwg_block_flip_definition_base_coordinate",id,&v.definition_base)?;coords(p,"dwg_block_flip_definition_end_coordinate",id,&v.definition_end)?;coords(p,"dwg_block_flip_label_point_coordinate",id,&v.label_point)?;project_properties(p,id,&v.properties)
}
pub(super) fn reconstruct_flip_parameter(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockFlipParameter,String>{
 let row=r.component("dwg_block_flip_parameter",id)?;
 Ok(DwgBlockFlipParameter{evaluation_expression:reconstruct_expression(r,id)?,name:row.text(1)?.into(),show_properties:boolean(row,2)?,chain_actions:boolean(row,3)?,base_location:read_location(row.text(4)?)?,label:row.text(5)?.into(),description:row.text(6)?.into(),value_set:DwgBlockFlipValueSet{base_label:row.text(7)?.into(),flipped_label:row.text(8)?.into()},updated_flip:DwgNamedEvaluationNodeReference{node_id:unsigned(row,9)?,expression_name:row.text(10)?.into()},definition_base:read_coords(r,"dwg_block_flip_definition_base_coordinate",id)?,definition_end:read_coords(r,"dwg_block_flip_definition_end_coordinate",id)?,label_point:read_coords(r,"dwg_block_flip_label_point_coordinate",id)?,properties:reconstruct_properties(r,id)?})
}
pub(super) fn project_handles(p:&mut Projection<'_,'_>,table:&'static str,id:i64,values:&[u64])->Result<(),String>{for(index,v)in values.iter().enumerate(){p.insert(table,&[I(id),I(ordinal(index)?),I(high(*v)),I(low(*v))])?;}Ok(())}
pub(super) fn reconstruct_handles(r:&mut Reader<'_,'_,'_>,table:&'static str,id:i64)->Result<Vec<u64>,String>{r.list(table,1,id,2)?.into_iter().map(|row|full_unsigned(row,3,4)).collect()}
pub(super) fn project_visibility_parameter(p:&mut Projection<'_,'_>,id:i64,v:&DwgBlockVisibilityParameter)->Result<(),String>{
 project_expression(p,id,&v.evaluation_expression)?;let history=match v.evaluation_history{DwgVisibilityEvaluationHistory::Stateless=>"stateless",DwgVisibilityEvaluationHistory::Required=>"required"};
 p.insert_key("dwg_block_visibility_parameter",id,&[T(&v.element_name),I(i64::from(v.show_properties)),I(i64::from(v.chain_actions)),I(i64::from(v.updated_visibility_node_id)),I(i64::from(v.initialized)),T(&v.name),T(&v.description),T(history)])?;
 coords(p,"dwg_block_visibility_definition_coordinate",id,&v.definition_point)?;project_properties(p,id,&v.properties)?;project_handles(p,"dwg_block_visibility_eligible_handle",id,&v.eligible_entity_handles)?;
 for(index,v)in v.states.iter().enumerate(){let state=p.insert("dwg_visibility_state",&[I(id),I(ordinal(index)?),T(&v.name)])?;project_handles(p,"dwg_visibility_state_visible_handle",state,&v.visible_entity_handles)?;project_handles(p,"dwg_visibility_state_expression_handle",state,&v.controlled_expression_handles)?;}Ok(())
}
pub(super) fn reconstruct_visibility_parameter(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockVisibilityParameter,String>{
 let row=r.component("dwg_block_visibility_parameter",id)?;
 let states=r.list("dwg_visibility_state",1,id,2)?.into_iter().map(|row|Ok(DwgVisibilityState{name:row.text(3)?.into(),visible_entity_handles:reconstruct_handles(r,"dwg_visibility_state_visible_handle",row.rowid)?,controlled_expression_handles:reconstruct_handles(r,"dwg_visibility_state_expression_handle",row.rowid)?})).collect::<Result<_,String>>()?;
 Ok(DwgBlockVisibilityParameter{evaluation_expression:reconstruct_expression(r,id)?,element_name:row.text(1)?.into(),show_properties:boolean(row,2)?,chain_actions:boolean(row,3)?,updated_visibility_node_id:unsigned(row,4)?,initialized:boolean(row,5)?,name:row.text(6)?.into(),description:row.text(7)?.into(),evaluation_history:match row.text(8)?{"stateless"=>DwgVisibilityEvaluationHistory::Stateless,"required"=>DwgVisibilityEvaluationHistory::Required,_=>return Err("DWG visibility evaluation history is unknown".into())},definition_point:read_coords(r,"dwg_block_visibility_definition_coordinate",id)?,properties:reconstruct_properties(r,id)?,eligible_entity_handles:reconstruct_handles(r,"dwg_block_visibility_eligible_handle",id)?,states})
}
