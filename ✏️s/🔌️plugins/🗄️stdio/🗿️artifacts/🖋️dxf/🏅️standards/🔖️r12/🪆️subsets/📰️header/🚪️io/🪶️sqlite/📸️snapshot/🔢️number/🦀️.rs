//! 🔢️ Explicit DXF binary64 field companions and logical row access.
use super::*;
use sqlite_snapshot::artifact::{FloatColumn::Binary64 as F,FloatRow,insert_ieee754};
pub(super) type Row<'a>=FloatRow<'a>;
pub(super) fn columns(table:&str)->&'static[sqlite_snapshot::artifact::FloatColumn]{match table{
    "dxf_header"=>&[F(8),F(9),F(10),F(11)],
    "dxf_block"=>&[F(4),F(5),F(6)],
    "dxf_entity"=>&[F(6),F(7),F(8),F(9),F(12),F(13),F(14),F(15)],
    "dxf_entity_point"=>&[F(4),F(5),F(6)],
    "dxf_vertex"=>&[F(3),F(4),F(5),F(6)],
    "dxf_group_code"=>&[F(13),F(14),F(15),F(16)],
    _=>&[],
}}
enum Rows<'c,'p>{Owned(sqlite_snapshot::artifact::Projection<'c,'p>),Forecast{control:&'c mut Control<'p>,count:usize,bytes:usize,phase:Phase}}
pub(super) struct Projection<'c,'p>{rows:Rows<'c,'p>}
impl<'c,'p> Projection<'c,'p>{
    pub fn new(sql:&str,control:&'c mut Control<'p>)->Result<Self,ValueError>{Ok(Self{rows:Rows::Owned(sqlite_snapshot::artifact::Projection::new(sql,control)?)})}
    pub fn forecast(control:&'c mut Control<'p>,phase:Phase)->Result<Self,ValueError>{control.checkpoint(phase,0,0)?;Ok(Self{rows:Rows::Forecast{control,count:0,bytes:0,phase}})}
    pub fn insert(&mut self,table:&str,cells:&[C<'_>])->Result<i64,ValueError>{match &mut self.rows{Rows::Owned(inner)=>insert_ieee754(inner,table,cells,columns(table)),Rows::Forecast{control,count,bytes,phase}=>forecast_row(control,count,bytes,*phase,table,cells)}}
    pub fn finish(self)->Result<Db,ValueError>{match self.rows{Rows::Owned(inner)=>inner.finish(),Rows::Forecast{..}=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF borrowed admission owns no database"))}}
    pub fn finish_forecast(self)->Result<(),ValueError>{match self.rows{Rows::Forecast{control,count,phase,..}=>control.checkpoint(phase,count,count),Rows::Owned(_)=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF owned rows are not a borrowed admission"))}}
}

fn forecast_row(control:&mut Control<'_>,count:&mut usize,bytes:&mut usize,phase:Phase,table:&str,cells:&[C<'_>])->Result<i64,ValueError>{
    let mut row_bytes=8usize;
    for cell in cells{let size=match cell{C::Null=>0,C::Integer(_)|C::Real(_)|C::Float32(_)=>8,C::Text(value)=>value.len(),C::PagedText(value)=>value.text_bytes(),C::Blob(value)=>value.len()};row_bytes=row_bytes.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DXF semantic cell byte count overflow"))?;}
    for column in columns(table){let index=match column{sqlite_snapshot::artifact::FloatColumn::Binary64(index)|sqlite_snapshot::artifact::FloatColumn::Binary32(index)=>*index};let cell=cells.get(index.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DXF IEEE identity column is invalid"))?).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DXF IEEE field is missing"))?;let value=match cell{C::Null=>continue,C::Real(value)=>*value,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DXF IEEE field has an invalid storage class"))};let class=if value.is_nan(){row_bytes-=8;"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};row_bytes=row_bytes.checked_add(8+class.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DXF IEEE semantic byte count overflow"))?;}
    let total=bytes.checked_add(row_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DXF semantic byte count overflow"))?;control.check_value_bytes(total)?;
    let next=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DXF entity row count overflow"))?;control.check_rows(next)?;if next%256==0{control.checkpoint(phase,next,0)?;}*bytes=total;*count=next;i64::try_from(next).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"DXF entity identity exceeds i64"))
}
