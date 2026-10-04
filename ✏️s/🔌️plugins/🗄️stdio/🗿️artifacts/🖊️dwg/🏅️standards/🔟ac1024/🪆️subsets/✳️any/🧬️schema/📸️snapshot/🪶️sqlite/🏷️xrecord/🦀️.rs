//! 🏷️ Typed DWG XRecord primitives, point coordinates, unsigned handles and ordered binary octets.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::super::*;
use super::reader::{boolean,byte,full_unsigned,high,low,ordinal,real,signed_integer,signed_word,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use super::number::{Projection,Row};
use Cell::{Integer as I,Real as R,Text as T,Null as N};

pub(super) fn project_values(projection:&mut Projection<'_,'_>,extended:Option<i64>,xrecord:Option<i64>,values:&[DwgXRecordValue])->Result<(),ValueError>{
    if extended.is_some()==xrecord.is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRecord values require exactly one typed owner"));}
    for(index,value)in values.iter().enumerate(){
        let (kind,code,text,real,integer,x,y,z,hi,lo)=match value{
            DwgXRecordValue::String{group_code,value}=>("string",*group_code,T(value),N,N,N,N,N,N,N),
            DwgXRecordValue::Real{group_code,value}=>("real",*group_code,N,R(*value),N,N,N,N,N,N),
            DwgXRecordValue::Boolean{group_code,value}=>("boolean",*group_code,N,N,I(i64::from(*value)),N,N,N,N,N),
            DwgXRecordValue::Integer8{group_code,value}=>("integer8",*group_code,N,N,I(i64::from(*value)),N,N,N,N,N),
            DwgXRecordValue::Integer16{group_code,value}=>("integer16",*group_code,N,N,I(i64::from(*value)),N,N,N,N,N),
            DwgXRecordValue::Integer32{group_code,value}=>("integer32",*group_code,N,N,I(i64::from(*value)),N,N,N,N,N),
            DwgXRecordValue::Integer64{group_code,value}=>("integer64",*group_code,N,N,I(*value),N,N,N,N,N),
            DwgXRecordValue::Point3d{group_code,value}=>("point3d",*group_code,N,N,N,R(value[0]),R(value[1]),R(value[2]),N,N),
            DwgXRecordValue::Binary{group_code,..}=>("binary",*group_code,N,N,N,N,N,N,N,N),
            DwgXRecordValue::Handle{group_code,value}=>("handle",*group_code,N,N,N,N,N,N,I(high(*value)),I(low(*value))),
            DwgXRecordValue::ObjectId{group_code,absolute_value}=>("object_id",*group_code,N,N,N,N,N,N,I(high(*absolute_value)),I(low(*absolute_value)))
        };
        let id=projection.insert("dwg_xrecord_value",&[extended.map(I).unwrap_or(N),xrecord.map(I).unwrap_or(N),I(ordinal(index)?),I(i64::from(code)),T(kind),text,real,integer,x,y,z,hi,lo])?;
        if let DwgXRecordValue::Binary{octets,..}=value{for(index,value)in octets.iter().enumerate(){projection.insert("dwg_xrecord_binary_octet",&[I(id),I(ordinal(index)?),I(i64::from(*value))])?;}}
    }
    Ok(())
}
pub(super) fn reconstruct_values(reader:&mut Reader<'_,'_,'_>,extended:Option<i64>,xrecord:Option<i64>)->Result<Vec<DwgXRecordValue>,ValueError>{
    let(owner_column,owner,unused_column)=match (extended,xrecord){(Some(id),None)=>(1,id,2),(None,Some(id))=>(2,id,1),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRecord values require exactly one typed owner"))};
    let mut result=Vec::new();
    for r in reader.list("dwg_xrecord_value",owner_column,owner,3)?{
        require_null(r,&[unused_column])?;
        let group_code=signed_word(r,4)?;
        let children=reader.list("dwg_xrecord_binary_octet",1,r.rowid,2)?;
        if r.text(5)?!="binary"&&!children.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG non-binary XRecord has binary octets"));}
        let value=match r.text(5)?{
            "string"=>{require_null(r,&[7,8,9,10,11,12,13])?;DwgXRecordValue::String{group_code,value:r.text(6)?.into()}},
            "real"=>{require_null(r,&[6,8,9,10,11,12,13])?;DwgXRecordValue::Real{group_code,value:real(r,7)?}},
            "boolean"=>{require_null(r,&[6,7,9,10,11,12,13])?;DwgXRecordValue::Boolean{group_code,value:boolean(r,8)?}},
            "integer8"=>{require_null(r,&[6,7,9,10,11,12,13])?;DwgXRecordValue::Integer8{group_code,value:i8::try_from(r.integer(8)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?}},
            "integer16"=>{require_null(r,&[6,7,9,10,11,12,13])?;DwgXRecordValue::Integer16{group_code,value:signed_word(r,8)?}},
            "integer32"=>{require_null(r,&[6,7,9,10,11,12,13])?;DwgXRecordValue::Integer32{group_code,value:signed_integer(r,8)?}},
            "integer64"=>{require_null(r,&[6,7,9,10,11,12,13])?;DwgXRecordValue::Integer64{group_code,value:r.integer(8)?}},
            "point3d"=>{require_null(r,&[6,7,8,12,13])?;DwgXRecordValue::Point3d{group_code,value:[real(r,9)?,real(r,10)?,real(r,11)?]}},
            "binary"=>{require_null(r,&[6,7,8,9,10,11,12,13])?;DwgXRecordValue::Binary{group_code,octets:children.into_iter().map(|r|byte(r,3)).collect::<Result<_,_>>()?}},
            "handle"=>{require_null(r,&[6,7,8,9,10,11])?;DwgXRecordValue::Handle{group_code,value:full_unsigned(r,12,13)?}},
            "object_id"=>{require_null(r,&[6,7,8,9,10,11])?;DwgXRecordValue::ObjectId{group_code,absolute_value:full_unsigned(r,12,13)?}},
            _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG XRecord primitive kind is unknown"))
        };
        result.push(value);
    }
    Ok(result)
}
pub(super) fn require_null(row:Row<'_>,columns:&[usize])->Result<(),ValueError>{for column in columns{if !matches!(row.value(*column)?,Cell::Null){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG typed variant has a non-NULL unrelated field"));}}Ok(())}
