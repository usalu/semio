//! 🗃️ DWG dictionaries, typed symbol-table controls and XRecord bodies.
use super::super::*;
use super::drawing::optional_words;
use super::reader::{boolean,full_unsigned,high,low,optional_unsigned,ordinal,word,Reader};
use super::xrecord::{project_values,reconstruct_values,require_null};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use super::number::Projection;
use Cell::{Integer as I,Text as T,Null as N};

pub(super) fn project_dictionary(projection:&mut Projection<'_,'_>,id:i64,value:&DwgDictionaryBody)->Result<(),String>{
    let default=optional_words(value.default_entry_handle);
    projection.insert_key("dwg_dictionary",id,&[I(i64::from(value.cloning_flag)),I(i64::from(value.hard_owner)),default[0],default[1]])?;
    for(index,value)in value.entries.iter().enumerate(){projection.insert("dwg_dictionary_entry",&[I(id),I(ordinal(index)?),T(&value.name),I(high(value.handle)),I(low(value.handle))])?;}
    Ok(())
}
pub(super) fn reconstruct_dictionary(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgDictionaryBody,String>{
    let row=reader.component("dwg_dictionary",id)?;
    let entries=reader.list("dwg_dictionary_entry",1,id,2)?.into_iter().map(|row|Ok(DwgNamedReference{name:row.text(3)?.into(),handle:full_unsigned(row,4,5)?})).collect::<Result<_,String>>()?;
    Ok(DwgDictionaryBody{entries,cloning_flag:word(row,1)?,hard_owner:boolean(row,2)?,default_entry_handle:optional_unsigned(row,3,4)?})
}
pub(super) fn project_control(projection:&mut Projection<'_,'_>,id:i64,value:&DwgTableControlBody)->Result<(),String>{
    let(kind,model,paper,by_block,by_layer,entries,additional)=match value{
        DwgTableControlBody::Block(v)=>("block",optional_words(v.model_space_handle),optional_words(v.paper_space_handle),[N,N],[N,N],&v.entry_handles,None),
        DwgTableControlBody::Layer(v)=>("layer",[N,N],[N,N],[N,N],[N,N],&v.entry_handles,None),
        DwgTableControlBody::TextStyle(v)=>("text_style",[N,N],[N,N],[N,N],[N,N],&v.entry_handles,None),
        DwgTableControlBody::Linetype(v)=>("linetype",[N,N],[N,N],[I(high(v.by_block_handle)),I(low(v.by_block_handle))],[I(high(v.by_layer_handle)),I(low(v.by_layer_handle))],&v.entry_handles,None),
        DwgTableControlBody::View(v)=>("view",[N,N],[N,N],[N,N],[N,N],&v.entry_handles,None),
        DwgTableControlBody::Ucs(v)=>("ucs",[N,N],[N,N],[N,N],[N,N],&v.entry_handles,None),
        DwgTableControlBody::Viewport(v)=>("viewport",[N,N],[N,N],[N,N],[N,N],&v.entry_handles,None),
        DwgTableControlBody::RegisteredApplication(v)=>("registered_application",[N,N],[N,N],[N,N],[N,N],&v.entry_handles,None),
        DwgTableControlBody::DimensionStyle(v)=>("dimension_style",[N,N],[N,N],[N,N],[N,N],&v.entry_handles,Some(&v.additional_handles))
    };
    projection.insert_key("dwg_table_control",id,&[T(kind),model[0],model[1],paper[0],paper[1],by_block[0],by_block[1],by_layer[0],by_layer[1]])?;
    for(index,value)in entries.iter().enumerate(){let handle=optional_words(value.handle);projection.insert("dwg_table_control_entry",&[I(id),I(ordinal(index)?),handle[0],handle[1]])?;}
    if let Some(values)=additional{for(index,value)in values.iter().enumerate(){projection.insert("dwg_dimension_style_control_additional_handle",&[I(id),I(ordinal(index)?),I(high(*value)),I(low(*value))])?;}}
    Ok(())
}
pub(super) fn reconstruct_control(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgTableControlBody,String>{
    let row=reader.component("dwg_table_control",id)?;
    let entries=reader.list("dwg_table_control_entry",1,id,2)?.into_iter().map(|row|Ok(DwgTableControlEntry{handle:optional_unsigned(row,3,4)?})).collect::<Result<Vec<_>,String>>()?;
    let additional=reader.list("dwg_dimension_style_control_additional_handle",1,id,2)?;
    if row.text(1)?!="dimension_style"&&!additional.is_empty(){return Err("DWG additional dimension-style handles have the wrong control kind".into());}
    match row.text(1)?{
        "block"=>{require_null(row,&[6,7,8,9])?;Ok(DwgTableControlBody::Block(DwgBlockTableControl{entry_handles:entries,model_space_handle:optional_unsigned(row,2,3)?,paper_space_handle:optional_unsigned(row,4,5)?}))},
        "linetype"=>{require_null(row,&[2,3,4,5])?;Ok(DwgTableControlBody::Linetype(DwgLinetypeTableControl{entry_handles:entries,by_block_handle:full_unsigned(row,6,7)?,by_layer_handle:full_unsigned(row,8,9)?}))},
        "dimension_style"=>{require_null(row,&[2,3,4,5,6,7,8,9])?;Ok(DwgTableControlBody::DimensionStyle(DwgDimensionStyleTableControl{entry_handles:entries,additional_handles:additional.into_iter().map(|row|full_unsigned(row,3,4)).collect::<Result<_,_>>()?}))},
        "layer"|"text_style"|"view"|"ucs"|"viewport"|"registered_application"=>{
            require_null(row,&[2,3,4,5,6,7,8,9])?;let value=DwgTableControlEntries{entry_handles:entries};
            Ok(match row.text(1)?{"layer"=>DwgTableControlBody::Layer(value),"text_style"=>DwgTableControlBody::TextStyle(value),"view"=>DwgTableControlBody::View(value),"ucs"=>DwgTableControlBody::Ucs(value),"viewport"=>DwgTableControlBody::Viewport(value),"registered_application"=>DwgTableControlBody::RegisteredApplication(value),_=>unreachable!()})
        },
        _=>Err("DWG table-control kind is unknown".into())
    }
}
pub(super) fn project_xrecord(projection:&mut Projection<'_,'_>,id:i64,value:&DwgXRecordBody)->Result<(),String>{
    projection.insert_key("dwg_xrecord",id,&[I(i64::from(value.cloning_flag))])?;
    project_values(projection,None,Some(id),&value.values)?;
    for(index,value)in value.object_id_handles.iter().enumerate(){projection.insert("dwg_xrecord_object_id_handle",&[I(id),I(ordinal(index)?),I(high(*value)),I(low(*value))])?;}
    Ok(())
}
pub(super) fn reconstruct_xrecord(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgXRecordBody,String>{
    let row=reader.component("dwg_xrecord",id)?;
    Ok(DwgXRecordBody{cloning_flag:word(row,1)?,values:reconstruct_values(reader,None,Some(id))?,object_id_handles:reader.list("dwg_xrecord_object_id_handle",1,id,2)?.into_iter().map(|row|full_unsigned(row,3,4)).collect::<Result<_,_>>()?})
}
