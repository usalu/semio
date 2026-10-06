//! 🎨️ DWG's owned complex-color variants and optional book/name metadata.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use super::reader::{byte,optional_text,word,Reader};
use super::xrecord::require_null;
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use super::number::Projection;
use Cell::{Integer as I,Text as T,Null as N};

pub(super) fn project(projection:&mut Projection<'_,'_>,table:&'static str,id:i64,color:&DwgComplexColor)->Result<(),ValueError>{
    let(kind,red,green,blue,index)=match color.value{
        DwgComplexColorValue::None=>("none",N,N,N,N),DwgComplexColorValue::ByLayer=>("by_layer",N,N,N,N),DwgComplexColorValue::ByBlock=>("by_block",N,N,N,N),
        DwgComplexColorValue::ByColor{red,green,blue}=>("by_color",I(i64::from(red)),I(i64::from(green)),I(i64::from(blue)),N),DwgComplexColorValue::ByAci{index}=>("by_aci",N,N,N,I(i64::from(index))),DwgComplexColorValue::ByPen{index}=>("by_pen",N,N,N,I(i64::from(index))),
        DwgComplexColorValue::Foreground=>("foreground",N,N,N,N),DwgComplexColorValue::LayerOff=>("layer_off",N,N,N,N),DwgComplexColorValue::LayerFrozen=>("layer_frozen",N,N,N,N)
    };
    projection.insert_key(table,id,&[I(i64::from(color.index)),T(kind),red,green,blue,index,color.name.as_deref().map(T).unwrap_or(N),color.book_name.as_deref().map(T).unwrap_or(N)])
}
pub(super) fn reconstruct(reader:&mut Reader<'_,'_,'_>,table:&'static str,id:i64)->Result<DwgComplexColor,ValueError>{
    let row=reader.component(table,id)?;
    let value=match row.text(2)?{
        "by_color"=>{require_null(row,&[6])?;DwgComplexColorValue::ByColor{red:byte(row,3)?,green:byte(row,4)?,blue:byte(row,5)?}},
        "by_aci"=>{require_null(row,&[3,4,5])?;DwgComplexColorValue::ByAci{index:word(row,6)?}},
        "by_pen"=>{require_null(row,&[3,4,5])?;DwgComplexColorValue::ByPen{index:byte(row,6)?}},
        kind=>{require_null(row,&[3,4,5,6])?;match kind{"none"=>DwgComplexColorValue::None,"by_layer"=>DwgComplexColorValue::ByLayer,"by_block"=>DwgComplexColorValue::ByBlock,"foreground"=>DwgComplexColorValue::Foreground,"layer_off"=>DwgComplexColorValue::LayerOff,"layer_frozen"=>DwgComplexColorValue::LayerFrozen,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG complex color kind is unknown"))}}
    };
    Ok(DwgComplexColor{index:word(row,1)?,value,name:optional_text(row,7)?,book_name:optional_text(row,8)?})
}
