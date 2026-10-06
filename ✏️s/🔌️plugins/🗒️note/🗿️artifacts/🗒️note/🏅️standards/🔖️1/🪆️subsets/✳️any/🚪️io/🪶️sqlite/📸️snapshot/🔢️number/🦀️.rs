//! 🔢️ Exact native binary64 fields of the Note document and its owned entities.
use super::*;
use sqlite_snapshot::artifact::{FloatColumn,FloatRow,insert_ieee754,insert_key_ieee754};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn positioned(error:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}
pub(super) type Row<'a>=FloatRow<'a>;
pub(super) fn columns(table:&str)->&'static[FloatColumn]{
    use FloatColumn::Binary64 as F;
    match table{
        "note_document"=>&[F(5),F(6),F(7),F(9),F(10),F(11)],
        "note_asset"=>&[F(5),F(6)],
        "note_block"=>&[F(7),F(8),F(9),F(10),F(11)],
        "note_text"=>&[F(6)],
        "note_ink"=>&[F(1),F(2),F(3),F(4),F(5)],
        "note_point"=>&[F(3),F(4)],
        _=>&[],
    }
}
enum Rows<'c,'p>{Owned(sqlite_snapshot::artifact::Projection<'c,'p>),Forecast{control:&'c mut Control<'p>,count:usize,bytes:usize}}
pub(super) struct Projection<'c,'p>{rows:Rows<'c,'p>}
impl<'c,'p> Projection<'c,'p>{
    pub fn new(sql:&str,control:&'c mut Control<'p>)->Result<Self,ValueError>{Ok(Self{rows:Rows::Owned(sqlite_snapshot::artifact::Projection::new(sql,control)?)})}
    pub fn forecast(control:&'c mut Control<'p>)->Result<Self,ValueError>{control.checkpoint(Phase::EncodeNative,0,0)?;Ok(Self{rows:Rows::Forecast{control,count:0,bytes:0}})}
    pub fn phase(&self)->Phase{match self.rows{Rows::Owned(_)=>Phase::ProjectSnapshot,Rows::Forecast{..}=>Phase::EncodeNative}}
    pub fn insert(&mut self,table:&str,cells:&[C<'_>])->Result<i64,ValueError>{match &mut self.rows{Rows::Owned(inner)=>insert_ieee754(inner,table,cells,columns(table)),Rows::Forecast{control,count,bytes}=>forecast_row(control,count,bytes,table,cells)}}
    pub fn insert_key(&mut self,table:&str,key:i64,cells:&[C<'_>])->Result<(),ValueError>{match &mut self.rows{Rows::Owned(inner)=>insert_key_ieee754(inner,table,key,cells,columns(table)),Rows::Forecast{control,count,bytes}=>forecast_row(control,count,bytes,table,cells).map(|_|())}}
    pub fn check_rows(&self,count:usize)->Result<(),ValueError>{match &self.rows{Rows::Owned(inner)=>inner.check_rows(count),Rows::Forecast{control,..}=>control.check_rows(count)}}
    pub fn check_value_bytes(&self,count:usize)->Result<(),ValueError>{match &self.rows{Rows::Owned(inner)=>inner.check_value_bytes(count),Rows::Forecast{control,..}=>control.check_value_bytes(count)}}
    pub fn checkpoint(&mut self)->Result<(),ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.checkpoint(),Rows::Forecast{control,count,..}=>control.checkpoint(Phase::EncodeNative,*count,0)}}
    pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.allocate_frontier(count),Rows::Forecast{control,..}=>{control.checkpoint(Phase::EncodeNative,0,count)?;sqlite_snapshot::transfer::reserve(count,control)}}}
    pub fn push_frontier<T>(&mut self,values:&mut Vec<T>,value:T)->Result<(),ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.push_frontier(values,value),Rows::Forecast{control,..}=>{control.checkpoint(Phase::EncodeNative,values.len(),0)?;super::push_borrowed(values,value,control)}}}
    pub fn search_frontier<T>(&mut self,values:&[T],mut compare:impl FnMut(&T,&mut Control<'_>)->Result<std::cmp::Ordering,ValueError>)->Result<Option<usize>,ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.search_frontier(values,compare),Rows::Forecast{control,..}=>{let(mut low,mut high)=(0,values.len());while low<high{control.checkpoint(Phase::EncodeNative,low,values.len())?;let middle=low+(high-low)/2;match compare(&values[middle],control)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(Some(middle))}}Ok(None)}}}
    pub fn finish(self)->Result<Db,ValueError>{match self.rows{Rows::Owned(inner)=>inner.finish(),Rows::Forecast{..}=>Err(invalid("Note forecast owns no SQLite database"))}}
    pub fn finish_forecast(self)->Result<(),ValueError>{match self.rows{Rows::Forecast{control,count,..}=>control.checkpoint(Phase::EncodeNative,count,count),Rows::Owned(_)=>Err(invalid("Note projection is not a borrowed forecast"))}}
}
fn forecast_row(control:&mut Control<'_>,count:&mut usize,bytes:&mut usize,table:&str,cells:&[C<'_>])->Result<i64,ValueError>{
    let mut row_bytes=8usize;
    for cell in cells{let size=match cell{C::Null=>0,C::Integer(_)|C::Real(_)|C::Float32(_)=>8,C::Text(value)=>value.len(),C::Blob(value)=>value.len()};row_bytes=row_bytes.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Note semantic cell byte count overflow"))?;}
    for column in columns(table){let index=match column{FloatColumn::Binary64(index)|FloatColumn::Binary32(index)=>*index};let cell=cells.get(index.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Note IEEE identity column is invalid"))?).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Note IEEE field is missing"))?;let value=match cell{C::Null=>continue,C::Real(value)=>*value,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Note IEEE field has an invalid storage class"))};let class=if value.is_nan(){row_bytes-=8;"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};row_bytes=row_bytes.checked_add(8+class.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Note IEEE semantic byte count overflow"))?;}
    let total=bytes.checked_add(row_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Note semantic value byte count overflow"))?;
    control.check_value_bytes(total)?;
    let next=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Note entity row count overflow"))?;
    control.check_rows(next)?;
    if next%256==0{control.checkpoint(Phase::EncodeNative,next,0)?;}
    *bytes=total;*count=next;i64::try_from(next).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Note entity identity exceeds i64"))
}
