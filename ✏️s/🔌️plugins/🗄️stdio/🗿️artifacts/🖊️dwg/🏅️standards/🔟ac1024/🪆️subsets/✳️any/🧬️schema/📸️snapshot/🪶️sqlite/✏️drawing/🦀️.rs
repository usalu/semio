//! ✏️ Ordered DWG layers, logical objects, handles, drawing bounds and extended data.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::super::*;
use super::reader::{byte,document,full_unsigned,high,low,optional_unsigned,ordinal,real,word,Reader};
use super::xrecord;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use super::number::Projection;
use Cell::{Integer as I,Real as R,Text as T};

pub(super) fn body_kind(value:&DwgLogicalObjectBody)->&'static str{match value{
    DwgLogicalObjectBody::Dictionary(_)=>"dictionary",DwgLogicalObjectBody::TableControl(_)=>"table_control",DwgLogicalObjectBody::TableRecord(_)=>"table_record",DwgLogicalObjectBody::XRecord(_)=>"xrecord",DwgLogicalObjectBody::Entity(_)=>"entity",DwgLogicalObjectBody::AssociativeDependency(_)=>"associative_dependency",DwgLogicalObjectBody::AssociativeValueDependency(_)=>"associative_value_dependency",DwgLogicalObjectBody::AssociativeGeometryDependency(_)=>"associative_geometry_dependency",DwgLogicalObjectBody::BlockGripLocationComponent(_)=>"block_grip_location_component",DwgLogicalObjectBody::DynamicBlockProxyNode(_)=>"dynamic_block_proxy_node",DwgLogicalObjectBody::AssociativeVariable(_)=>"associative_variable",DwgLogicalObjectBody::AssociativeDimensionDependencyBody(_)=>"associative_dimension_dependency_body",DwgLogicalObjectBody::VisualStyle(_)=>"visual_style",DwgLogicalObjectBody::BlockParameterDependencyBody(_)=>"block_parameter_dependency_body",DwgLogicalObjectBody::BlockRepresentationData(_)=>"block_representation_data",DwgLogicalObjectBody::DynamicBlockPurgePreventer(_)=>"dynamic_block_purge_preventer",DwgLogicalObjectBody::EvaluationGraph(_)=>"evaluation_graph",DwgLogicalObjectBody::BlockFlipParameter(_)=>"block_flip_parameter",DwgLogicalObjectBody::BlockVisibilityParameter(_)=>"block_visibility_parameter",DwgLogicalObjectBody::Placeholder(_)=>"placeholder",DwgLogicalObjectBody::DictionaryVariable(_)=>"dictionary_variable",DwgLogicalObjectBody::AnnotationScale(_)=>"annotation_scale",DwgLogicalObjectBody::SortEntitiesTable(_)=>"sort_entities_table",DwgLogicalObjectBody::TableStyle(_)=>"table_style",DwgLogicalObjectBody::MlineStyle(_)=>"mline_style",DwgLogicalObjectBody::MLeaderStyle(_)=>"mleader_style",DwgLogicalObjectBody::Material(_)=>"material",DwgLogicalObjectBody::BlockMoveAction(_)=>"block_move_action",DwgLogicalObjectBody::AssocNetwork(_)=>"assoc_network",DwgLogicalObjectBody::Assoc2dConstraintGroup(_)=>"assoc_2d_constraint_group",DwgLogicalObjectBody::BlockLinearParameter(_)=>"block_linear_parameter",DwgLogicalObjectBody::BlockLinearGrip(_)=>"block_linear_grip",DwgLogicalObjectBody::BlockFlipGrip(_)=>"block_flip_grip",DwgLogicalObjectBody::BlockVisibilityGrip(_)=>"block_visibility_grip",DwgLogicalObjectBody::BlockAlignmentParameter(_)=>"block_alignment_parameter",DwgLogicalObjectBody::BlockAlignmentGrip(_)=>"block_alignment_grip",DwgLogicalObjectBody::BlockStretchAction(_)=>"block_stretch_action",DwgLogicalObjectBody::BlockScaleAction(_)=>"block_scale_action",DwgLogicalObjectBody::BlockFlipAction(_)=>"block_flip_action",DwgLogicalObjectBody::BlockBasePointParameter(_)=>"block_base_point_parameter",DwgLogicalObjectBody::BlockVerticalConstraintParameter(_)=>"block_vertical_constraint_parameter",DwgLogicalObjectBody::BlockHorizontalConstraintParameter(_)=>"block_horizontal_constraint_parameter",DwgLogicalObjectBody::Layout(_)=>"layout"
}}
pub(super) fn project(projection:&mut Projection<'_,'_>,drawing:&DwgLogicalDrawing,project_body:fn(&mut Projection<'_,'_>,i64,&DwgLogicalObjectBody)->Result<(),ValueError>)->Result<(),ValueError>{
    projection.insert("dwg_drawing",&[I(1)])?;
    for(index,value)in drawing.layers.iter().enumerate(){projection.insert("dwg_drawing_layer",&[I(1),I(ordinal(index)?),T(&value.name),I(i64::from(value.color))])?;}
    for(table,values)in [("dwg_drawing_extmin",&drawing.extmin),("dwg_drawing_extmax",&drawing.extmax)]{for(index,value)in values.iter().enumerate(){projection.insert(table,&[I(1),I(ordinal(index)?),R(*value)])?;}}
    for(index,value)in drawing.objects.iter().enumerate(){
        let category=match value.category{DwgObjectCategory::Entity=>"entity",DwgObjectCategory::TableControl=>"table_control",DwgObjectCategory::TableRecord=>"table_record",DwgObjectCategory::Dictionary=>"dictionary",DwgObjectCategory::Object=>"object",DwgObjectCategory::Custom=>"custom"};
        let owner=optional_words(value.owner_handle);let dictionary=optional_words(value.extension_dictionary_handle);
        let body=value.body.as_ref().map(|value|T(body_kind(value))).unwrap_or(Cell::Null);
        let id=projection.insert("dwg_object",&[I(1),I(ordinal(index)?),I(high(value.handle)),I(low(value.handle)),I(i64::from(value.type_code)),T(&value.class_name),T(category),owner[0],owner[1],dictionary[0],dictionary[1],body])?;
        for(table,values)in [("dwg_object_reactor_handle",&value.reactor_handles),("dwg_object_referenced_handle",&value.referenced_handles)]{for(index,value)in values.iter().enumerate(){projection.insert(table,&[I(id),I(ordinal(index)?),I(high(*value)),I(low(*value))])?;}}
        for(index,value)in value.extended_data.iter().enumerate(){let extended_id=projection.insert("dwg_extended_entity_data",&[I(id),I(ordinal(index)?),I(high(value.application_handle)),I(low(value.application_handle))])?;xrecord::project_values(projection,Some(extended_id),None,&value.values)?;}
        if let Some(body)=&value.body{project_body(projection,id,body)?;}
    }
    Ok(())
}
pub(super) fn reconstruct(reader:&mut Reader<'_,'_,'_>,reconstruct_body:fn(&mut Reader<'_,'_,'_>,i64,&str)->Result<DwgLogicalObjectBody,ValueError>)->Result<DwgLogicalDrawing,ValueError>{
    document(reader.one("dwg_drawing")?)?;
    let mut layers=Vec::new();for r in reader.list("dwg_drawing_layer",1,1,2)?{layers.push(DwgLogicalLayer{name:r.text(3)?.into(),color:byte(r,4)?});}
    let extmin=reader.list("dwg_drawing_extmin",1,1,2)?.into_iter().map(|r|real(r,3)).collect::<Result<_,_>>()?;
    let extmax=reader.list("dwg_drawing_extmax",1,1,2)?.into_iter().map(|r|real(r,3)).collect::<Result<_,_>>()?;
    let mut objects=Vec::new();
    for r in reader.list("dwg_object",1,1,2)?{
        let reactor_handles=reader.list("dwg_object_reactor_handle",1,r.rowid,2)?.into_iter().map(|r|full_unsigned(r,3,4)).collect::<Result<_,_>>()?;
        let referenced_handles=reader.list("dwg_object_referenced_handle",1,r.rowid,2)?.into_iter().map(|r|full_unsigned(r,3,4)).collect::<Result<_,_>>()?;
        let mut extended_data=Vec::new();for child in reader.list("dwg_extended_entity_data",1,r.rowid,2)?{extended_data.push(DwgExtendedEntityData{application_handle:full_unsigned(child,3,4)?,values:xrecord::reconstruct_values(reader,Some(child.rowid),None)?});}
        let body=match r.value(12)?{Cell::Null=>None,Cell::Text(kind)=>Some(reconstruct_body(reader,r.rowid,kind)?),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG object body kind must be TEXT or NULL"))};
        objects.push(DwgLogicalObject{handle:full_unsigned(r,3,4)?,type_code:word(r,5)?,class_name:r.text(6)?.into(),category:match r.text(7)?{"entity"=>DwgObjectCategory::Entity,"table_control"=>DwgObjectCategory::TableControl,"table_record"=>DwgObjectCategory::TableRecord,"dictionary"=>DwgObjectCategory::Dictionary,"object"=>DwgObjectCategory::Object,"custom"=>DwgObjectCategory::Custom,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG object category is unknown"))},owner_handle:optional_unsigned(r,8,9)?,extension_dictionary_handle:optional_unsigned(r,10,11)?,reactor_handles,referenced_handles,extended_data,body});
    }
    Ok(DwgLogicalDrawing{layers,objects,extmin,extmax})
}
pub(super) fn optional_words(value:Option<u64>)->[Cell<'static>;2]{match value{Some(value)=>[I(high(value)),I(low(value))],None=>[Cell::Null,Cell::Null]}}
