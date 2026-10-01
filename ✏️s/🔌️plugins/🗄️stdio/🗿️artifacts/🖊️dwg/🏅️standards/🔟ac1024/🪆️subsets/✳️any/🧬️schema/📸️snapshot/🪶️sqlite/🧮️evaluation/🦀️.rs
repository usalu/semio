//! 🧮️ DWG evaluation expression values retain their tagged scalar and coordinate domains.
use super::super::*;
use super::number::Projection;
use super::reader::{full_unsigned,high,low,ordinal,signed_integer,signed_word,unsigned,Reader};
use super::xrecord::require_null;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Real as R,Null as N};

pub(super) fn project_expression(projection:&mut Projection<'_,'_>,id:i64,value:&DwgEvaluationExpression)->Result<(),String>{
 let(kind,double,text,integer,handle,short)=match &value.value{
 DwgEvaluationExpressionValue::Empty=>("empty",N,N,N,[N,N],N),
 DwgEvaluationExpressionValue::Double(value)=>("double",R(*value),N,N,[N,N],N),
 DwgEvaluationExpressionValue::PointGroup10(_)=>("point_group_10",N,N,N,[N,N],N),
 DwgEvaluationExpressionValue::PointGroup11(_)=>("point_group_11",N,N,N,[N,N],N),
 DwgEvaluationExpressionValue::String(value)=>("string",N,T(value),N,[N,N],N),
 DwgEvaluationExpressionValue::Integer32(value)=>("integer32",N,N,I(i64::from(*value)),[N,N],N),
 DwgEvaluationExpressionValue::ObjectReference(value)=>("object_reference",N,N,N,[I(high(*value)),I(low(*value))],N),
 DwgEvaluationExpressionValue::Integer16(value)=>("integer16",N,N,N,[N,N],I(i64::from(*value)))
 };
 projection.insert_key("dwg_evaluation_expression",id,&[I(i64::from(value.parent_id)),I(i64::from(value.major_version)),I(i64::from(value.minor_version)),I(i64::from(value.node_id)),T(kind),double,text,integer,handle[0],handle[1],short])?;
 match &value.value{DwgEvaluationExpressionValue::PointGroup10(values)=>project_coordinates(projection,"dwg_evaluation_point_group_10_coordinate",id,values)?,DwgEvaluationExpressionValue::PointGroup11(values)=>project_coordinates(projection,"dwg_evaluation_point_group_11_coordinate",id,values)?,_=>{}}Ok(())
}
pub(super) fn reconstruct_expression(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgEvaluationExpression,String>{
 let row=reader.component("dwg_evaluation_expression",id)?;
 let group10=reconstruct_coordinates(reader,"dwg_evaluation_point_group_10_coordinate",id)?;
 let group11=reconstruct_coordinates(reader,"dwg_evaluation_point_group_11_coordinate",id)?;
 let kind=row.text(5)?;
 if kind!="point_group_10"&&!group10.is_empty()||kind!="point_group_11"&&!group11.is_empty(){return Err("DWG evaluation coordinates have the wrong value kind".into());}
 let value=match kind{
 "empty"=>{require_null(row,&[6,7,8,9,10,11])?;DwgEvaluationExpressionValue::Empty},
 "double"=>{require_null(row,&[7,8,9,10,11])?;DwgEvaluationExpressionValue::Double(row.real(6)?)},
 "point_group_10"=>{require_null(row,&[6,7,8,9,10,11])?;DwgEvaluationExpressionValue::PointGroup10(group10)},
 "point_group_11"=>{require_null(row,&[6,7,8,9,10,11])?;DwgEvaluationExpressionValue::PointGroup11(group11)},
 "string"=>{require_null(row,&[6,8,9,10,11])?;DwgEvaluationExpressionValue::String(row.text(7)?.into())},
 "integer32"=>{require_null(row,&[6,7,9,10,11])?;DwgEvaluationExpressionValue::Integer32(signed_integer(row,8)?)},
 "object_reference"=>{require_null(row,&[6,7,8,11])?;DwgEvaluationExpressionValue::ObjectReference(full_unsigned(row,9,10)?)},
 "integer16"=>{require_null(row,&[6,7,8,9,10])?;DwgEvaluationExpressionValue::Integer16(signed_word(row,11)?)},
 _=>return Err("DWG evaluation value kind is unknown".into())
 };
 Ok(DwgEvaluationExpression{parent_id:signed_integer(row,1)?,major_version:unsigned(row,2)?,minor_version:unsigned(row,3)?,node_id:unsigned(row,4)?,value})
}
pub(super) fn project_coordinates(projection:&mut Projection<'_,'_>,table:&'static str,id:i64,values:&[f64])->Result<(),String>{for(index,value)in values.iter().enumerate(){projection.insert(table,&[I(id),I(ordinal(index)?),R(*value)])?;}Ok(())}
pub(super) fn reconstruct_coordinates(reader:&mut Reader<'_,'_,'_>,table:&'static str,id:i64)->Result<Vec<f64>,String>{reader.list(table,1,id,2)?.into_iter().map(|row|row.real(3)).collect()}
pub(super) fn project_grip_location(projection:&mut Projection<'_,'_>,id:i64,value:&DwgBlockGripLocationComponent)->Result<(),String>{project_expression(projection,id,&value.evaluation_expression)?;projection.insert_key("dwg_block_grip_location_component",id,&[I(i64::from(value.grip_type)),T(&value.grip_expression)])}
pub(super) fn reconstruct_grip_location(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockGripLocationComponent,String>{let row=reader.component("dwg_block_grip_location_component",id)?;Ok(DwgBlockGripLocationComponent{evaluation_expression:reconstruct_expression(reader,id)?,grip_type:unsigned(row,1)?,grip_expression:row.text(2)?.into()})}
pub(super) fn project_proxy(projection:&mut Projection<'_,'_>,id:i64,value:&DwgDynamicBlockProxyNode)->Result<(),String>{project_expression(projection,id,&value.evaluation_expression)?;projection.insert_key("dwg_dynamic_block_proxy_node",id,&[])}
pub(super) fn reconstruct_proxy(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgDynamicBlockProxyNode,String>{reader.component("dwg_dynamic_block_proxy_node",id)?;Ok(DwgDynamicBlockProxyNode{evaluation_expression:reconstruct_expression(reader,id)?})}
