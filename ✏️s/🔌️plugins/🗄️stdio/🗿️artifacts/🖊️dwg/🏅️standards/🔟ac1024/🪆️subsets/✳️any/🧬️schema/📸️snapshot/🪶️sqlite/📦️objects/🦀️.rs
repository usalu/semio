//! 📦️ DWG named object components, draw ordering and evaluation graph topology.
use super::super::*;
use super::number::Projection;
use super::reader::{boolean,full_unsigned,high,low,ordinal,unsigned,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Real as R};

pub(super) fn project_placeholder(projection:&mut Projection<'_,'_>,id:i64,_:&DwgPlaceholder)->Result<(),String>{projection.insert_key("dwg_placeholder",id,&[])}
pub(super) fn reconstruct_placeholder(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgPlaceholder,String>{reader.component("dwg_placeholder",id)?;Ok(DwgPlaceholder{})}
pub(super) fn project_dictionary_variable(projection:&mut Projection<'_,'_>,id:i64,value:&DwgDictionaryVariable)->Result<(),String>{projection.insert_key("dwg_dictionary_variable",id,&[T(&value.value)])}
pub(super) fn reconstruct_dictionary_variable(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgDictionaryVariable,String>{Ok(DwgDictionaryVariable{value:reader.component("dwg_dictionary_variable",id)?.text(1)?.into()})}
pub(super) fn project_annotation_scale(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAnnotationScale)->Result<(),String>{projection.insert_key("dwg_annotation_scale",id,&[T(&value.name),R(value.paper_units),R(value.drawing_units),I(i64::from(value.is_unit_scale))])}
pub(super) fn reconstruct_annotation_scale(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAnnotationScale,String>{let row=reader.component("dwg_annotation_scale",id)?;Ok(DwgAnnotationScale{name:row.text(1)?.into(),paper_units:row.real(2)?,drawing_units:row.real(3)?,is_unit_scale:boolean(row,4)?})}
pub(super) fn project_sort_entities_table(projection:&mut Projection<'_,'_>,id:i64,value:&DwgSortEntitiesTable)->Result<(),String>{
 projection.insert_key("dwg_sort_entities_table",id,&[I(high(value.block_header_handle)),I(low(value.block_header_handle))])?;
 for(index,value)in value.entries.iter().enumerate(){projection.insert("dwg_draw_order_entry",&[I(id),I(ordinal(index)?),I(high(value.entity_handle)),I(low(value.entity_handle)),I(high(value.sort_handle)),I(low(value.sort_handle))])?;}Ok(())
}
pub(super) fn reconstruct_sort_entities_table(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgSortEntitiesTable,String>{
 let row=reader.component("dwg_sort_entities_table",id)?;
 Ok(DwgSortEntitiesTable{block_header_handle:full_unsigned(row,1,2)?,entries:reader.list("dwg_draw_order_entry",1,id,2)?.into_iter().map(|row|Ok(DwgDrawOrderEntry{entity_handle:full_unsigned(row,3,4)?,sort_handle:full_unsigned(row,5,6)?})).collect::<Result<_,String>>()?})
}
pub(super) fn project_block_parameter_dependency_body(projection:&mut Projection<'_,'_>,id:i64,value:&DwgBlockParameterDependencyBody)->Result<(),String>{projection.insert_key("dwg_block_parameter_dependency_body",id,&[T(&value.name)])}
pub(super) fn reconstruct_block_parameter_dependency_body(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockParameterDependencyBody,String>{Ok(DwgBlockParameterDependencyBody{name:reader.component("dwg_block_parameter_dependency_body",id)?.text(1)?.into()})}
pub(super) fn project_dimension_dependency_body(projection:&mut Projection<'_,'_>,id:i64,value:&DwgAssociativeDimensionDependencyBody)->Result<(),String>{projection.insert_key("dwg_associative_dimension_dependency_body",id,&[T(&value.name)])}
pub(super) fn reconstruct_dimension_dependency_body(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgAssociativeDimensionDependencyBody,String>{Ok(DwgAssociativeDimensionDependencyBody{name:reader.component("dwg_associative_dimension_dependency_body",id)?.text(1)?.into()})}
pub(super) fn project_representation_data(projection:&mut Projection<'_,'_>,id:i64,value:&DwgBlockRepresentationData)->Result<(),String>{projection.insert_key("dwg_block_representation_data",id,&[I(high(value.represented_block_header_handle)),I(low(value.represented_block_header_handle))])}
pub(super) fn reconstruct_representation_data(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgBlockRepresentationData,String>{let row=reader.component("dwg_block_representation_data",id)?;Ok(DwgBlockRepresentationData{represented_block_header_handle:full_unsigned(row,1,2)?})}
pub(super) fn project_purge_preventer(projection:&mut Projection<'_,'_>,id:i64,value:&DwgDynamicBlockPurgePreventer)->Result<(),String>{projection.insert_key("dwg_dynamic_block_purge_preventer",id,&[I(high(value.protected_block_header_handle)),I(low(value.protected_block_header_handle))])}
pub(super) fn reconstruct_purge_preventer(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgDynamicBlockPurgePreventer,String>{let row=reader.component("dwg_dynamic_block_purge_preventer",id)?;Ok(DwgDynamicBlockPurgePreventer{protected_block_header_handle:full_unsigned(row,1,2)?})}
pub(super) fn project_graph(projection:&mut Projection<'_,'_>,id:i64,value:&DwgEvaluationGraph)->Result<(),String>{
 projection.insert_key("dwg_evaluation_graph",id,&[])?;
 for(index,node)in value.nodes.iter().enumerate(){projection.insert("dwg_evaluation_graph_node",&[I(id),I(ordinal(index)?),I(i64::from(node.id)),I(high(node.expression_handle)),I(low(node.expression_handle))])?;}
 for(index,edge)in value.edges.iter().enumerate(){projection.insert("dwg_evaluation_graph_edge",&[I(id),I(ordinal(index)?),I(i64::from(edge.from_node_id)),I(i64::from(edge.to_node_id)),I(i64::from(edge.reference_count)),I(i64::from(edge.invertible)),I(i64::from(edge.suppressed))])?;}Ok(())
}
pub(super) fn reconstruct_graph(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgEvaluationGraph,String>{
 reader.component("dwg_evaluation_graph",id)?;
 Ok(DwgEvaluationGraph{
 nodes:reader.list("dwg_evaluation_graph_node",1,id,2)?.into_iter().map(|row|Ok(DwgEvaluationGraphNode{id:unsigned(row,3)?,expression_handle:full_unsigned(row,4,5)?})).collect::<Result<_,String>>()?,
 edges:reader.list("dwg_evaluation_graph_edge",1,id,2)?.into_iter().map(|row|Ok(DwgEvaluationGraphEdge{from_node_id:unsigned(row,3)?,to_node_id:unsigned(row,4)?,reference_count:unsigned(row,5)?,invertible:boolean(row,6)?,suppressed:boolean(row,7)?})).collect::<Result<_,String>>()?
 })
}
