//! 🔗️ DWG associative dependencies, variables and network membership.
use super::super::*;
use super::drawing::optional_words;
use super::number::Projection;
use super::reader::{boolean,full_unsigned,high,low,optional_text,optional_unsigned,ordinal,signed_integer,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Null as N};

pub(super) fn project_dependency(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAssociativeDependency)->Result<(),String>{
 let read=optional_words(value.read_dependency_handle);let node=optional_words(value.dependency_node_handle);let body=optional_words(value.dependency_body_handle);
 let status=match value.status{DwgAssociativeDependencyStatus::UpToDate=>"up_to_date"};
 projection.insert_key("dwg_associative_dependency",id,&[T(status),I(i64::from(value.is_read_dependency)),I(i64::from(value.is_write_dependency)),I(i64::from(value.is_attached_to_object)),I(i64::from(value.is_delegating_to_owning_action)),I(i64::from(value.order)),I(high(value.dependent_on_object_handle)),I(low(value.dependent_on_object_handle)),value.name.as_deref().map_or(N,T),read[0],read[1],node[0],node[1],body[0],body[1],I(i64::from(value.dependency_body_id))])
}
pub(super) fn reconstruct_dependency(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssociativeDependency,String>{
 let row=reader.component("dwg_associative_dependency",id)?;
 Ok(DwgAssociativeDependency{status:match row.text(1)?{"up_to_date"=>DwgAssociativeDependencyStatus::UpToDate,_=>return Err("DWG associative dependency status is unknown".into())},is_read_dependency:boolean(row,2)?,is_write_dependency:boolean(row,3)?,is_attached_to_object:boolean(row,4)?,is_delegating_to_owning_action:boolean(row,5)?,order:signed_integer(row,6)?,dependent_on_object_handle:full_unsigned(row,7,8)?,name:optional_text(row,9)?,read_dependency_handle:optional_unsigned(row,10,11)?,dependency_node_handle:optional_unsigned(row,12,13)?,dependency_body_handle:optional_unsigned(row,14,15)?,dependency_body_id:signed_integer(row,16)?})
}
pub(super) fn project_value_dependency(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAssociativeValueDependency)->Result<(),String>{
 project_dependency(projection,id,&value.dependency)?;let DwgEvaluationVariant::Integer32(cached)=value.cached_value;
 projection.insert_key("dwg_associative_value_dependency",id,&[T("integer32"),I(i64::from(cached)),T(&value.value_name)])
}
pub(super) fn reconstruct_value_dependency(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssociativeValueDependency,String>{
 let row=reader.component("dwg_associative_value_dependency",id)?;
 if row.text(1)?!="integer32"{return Err("DWG cached evaluation variant is unknown".into());}
 Ok(DwgAssociativeValueDependency{dependency:reconstruct_dependency(reader,id)?,cached_value:DwgEvaluationVariant::Integer32(signed_integer(row,2)?),value_name:row.text(3)?.into()})
}
pub(super) fn project_geometry_dependency(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAssociativeGeometryDependency)->Result<(),String>{
 project_dependency(projection,id,&value.dependency)?;projection.insert_key("dwg_associative_geometry_dependency",id,&[I(i64::from(value.enabled)),T(&value.persistent_subentity_class_name),I(i64::from(value.dependent_on_compound_object))])
}
pub(super) fn reconstruct_geometry_dependency(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssociativeGeometryDependency,String>{
 let row=reader.component("dwg_associative_geometry_dependency",id)?;
 Ok(DwgAssociativeGeometryDependency{dependency:reconstruct_dependency(reader,id)?,enabled:boolean(row,1)?,persistent_subentity_class_name:row.text(2)?.into(),dependent_on_compound_object:boolean(row,3)?})
}
pub(super) fn project_action(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAssociativeAction)->Result<(),String>{
 let network=optional_words(value.owning_network_handle);let body=optional_words(value.action_body_handle);
 let status=match value.status{DwgAssociativeActionStatus::UpToDate=>"up_to_date"};
 projection.insert_key("dwg_associative_action",id,&[T(status),network[0],network[1],body[0],body[1],I(i64::from(value.action_index)),I(i64::from(value.maximum_dependency_index))])?;
 for(index,value)in value.dependencies.iter().enumerate(){projection.insert("dwg_associative_action_dependency",&[I(id),I(ordinal(index)?),I(i64::from(value.owned)),I(high(value.dependency_handle)),I(low(value.dependency_handle))])?;}Ok(())
}
pub(super) fn reconstruct_action(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssociativeAction,String>{
 let row=reader.component("dwg_associative_action",id)?;
 Ok(DwgAssociativeAction{status:match row.text(1)?{"up_to_date"=>DwgAssociativeActionStatus::UpToDate,_=>return Err("DWG associative action status is unknown".into())},owning_network_handle:optional_unsigned(row,2,3)?,action_body_handle:optional_unsigned(row,4,5)?,action_index:signed_integer(row,6)?,maximum_dependency_index:signed_integer(row,7)?,dependencies:reader.list("dwg_associative_action_dependency",1,id,2)?.into_iter().map(|row|Ok(DwgAssociativeActionDependency{owned:boolean(row,3)?,dependency_handle:full_unsigned(row,4,5)?})).collect::<Result<_,String>>()?})
}
pub(super) fn project_variable(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAssociativeVariable)->Result<(),String>{
 project_action(projection,id,&value.action)?;let DwgEvaluationVariant::Integer32(evaluated)=value.evaluated_value;
 projection.insert_key("dwg_associative_variable",id,&[T(&value.name),T(&value.expression),T(&value.evaluator_id),T(&value.description),T("integer32"),I(i64::from(evaluated)),I(i64::from(value.mergeable)),value.mergeable_variable_name.as_deref().map_or(N,T),I(i64::from(value.must_merge))])?;
 for(index,handle)in value.referenced_value_dependency_handles.iter().enumerate(){projection.insert("dwg_variable_value_dependency_handle",&[I(id),I(ordinal(index)?),I(high(*handle)),I(low(*handle))])?;}Ok(())
}
pub(super) fn reconstruct_variable(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssociativeVariable,String>{
 let row=reader.component("dwg_associative_variable",id)?;
 if row.text(5)?!="integer32"{return Err("DWG evaluated variable variant is unknown".into());}
 Ok(DwgAssociativeVariable{action:reconstruct_action(reader,id)?,name:row.text(1)?.into(),expression:row.text(2)?.into(),evaluator_id:row.text(3)?.into(),description:row.text(4)?.into(),evaluated_value:DwgEvaluationVariant::Integer32(signed_integer(row,6)?),mergeable:boolean(row,7)?,mergeable_variable_name:optional_text(row,8)?,must_merge:boolean(row,9)?,referenced_value_dependency_handles:reader.list("dwg_variable_value_dependency_handle",1,id,2)?.into_iter().map(|row|full_unsigned(row,3,4)).collect::<Result<_,_>>()?})
}
pub(super) fn project_network(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAssocNetwork)->Result<(),String>{
 project_action(projection,id,&value.action)?;projection.insert_key("dwg_assoc_network",id,&[I(i64::from(value.network_action_index))])?;
 for(index,value)in value.actions.iter().enumerate(){let kind=match value.kind{DwgAssocNetworkMemberKind::Network=>"network",DwgAssocNetworkMemberKind::Action=>"action"};projection.insert("dwg_assoc_network_member",&[I(id),I(ordinal(index)?),T(kind),I(high(value.handle)),I(low(value.handle))])?;}Ok(())
}
pub(super) fn reconstruct_network(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssocNetwork,String>{
 let row=reader.component("dwg_assoc_network",id)?;
 Ok(DwgAssocNetwork{action:reconstruct_action(reader,id)?,network_action_index:signed_integer(row,1)?,actions:reader.list("dwg_assoc_network_member",1,id,2)?.into_iter().map(|row|Ok(DwgAssocNetworkMember{handle:full_unsigned(row,4,5)?,kind:match row.text(3)?{"network"=>DwgAssocNetworkMemberKind::Network,"action"=>DwgAssocNetworkMemberKind::Action,_=>return Err("DWG network member kind is unknown".into())}})).collect::<Result<_,String>>()?})
}
