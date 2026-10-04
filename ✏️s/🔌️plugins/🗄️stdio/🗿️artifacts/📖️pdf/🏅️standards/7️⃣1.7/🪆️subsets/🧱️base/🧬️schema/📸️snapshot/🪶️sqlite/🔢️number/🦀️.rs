//! 🔢️ Authored native binary64 scalar fields for each PDF semantic entity.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::*;
use sqlite_snapshot::artifact::{FloatColumn,FloatRow,insert_ieee754,insert_key_ieee754};
pub(super) type Row<'a>=FloatRow<'a>;
pub(super) fn columns(table:&str)->&'static[FloatColumn]{
    use FloatColumn::Binary64 as F;
    match table{
        "pdf_function"=>&[F(7)],
        "pdf_function_real"=>&[F(4)],
        "pdf_color_real"=>&[F(4)],
        "pdf_font_descriptor"=>&[F(3),F(4),F(5),F(6),F(7),F(8),F(9),F(10),F(11),F(12),F(13),F(14),F(15),F(16),F(17),F(20)],
        "pdf_cid_font"=>&[F(7),F(8),F(9)],
        "pdf_cid_width"=>&[F(3)],
        "pdf_cid_vertical_metric"=>&[F(3),F(4),F(5)],
        "pdf_font"=>&[F(8),F(9),F(10),F(11),F(12),F(13),F(14),F(15),F(16),F(17)],
        "pdf_font_width"=>&[F(3)],
        "pdf_operation"=>&[F(4),F(7),F(8),F(10),F(12),F(13),F(14),F(15),F(16),F(17),F(18),F(19),F(20),F(21),F(22),F(23),F(25),F(27),F(28),F(29),F(33),F(34),F(35),F(36),F(37),F(38),F(41),F(42),F(43),F(44),F(45),F(46),F(47),F(48)],
        "pdf_operation_matrix"=>&[F(1),F(2),F(3),F(4),F(5),F(6)],
        "pdf_operation_component"=>&[F(3)],
        "pdf_text_array_item"=>&[F(6)],
        "pdf_inline_decode"=>&[F(3)],
        "pdf_image_real"=>&[F(4)],
        "pdf_form_xobject"=>&[F(2),F(3),F(4),F(5),F(6),F(7),F(8),F(9),F(10),F(11)],
        "pdf_ext_g_state"=>&[F(2),F(5),F(6),F(12),F(18),F(19),F(22),F(23)],
        "pdf_g_state_real"=>&[F(4)],
        "pdf_shading"=>&[F(5),F(6),F(7),F(8)],
        "pdf_shading_background"=>&[F(3)],
        "pdf_function_shading"=>&[F(1),F(2),F(3),F(4),F(5),F(6),F(7),F(8),F(9),F(10)],
        "pdf_axial_shading"=>&[F(1),F(2),F(3),F(4),F(5),F(6)],
        "pdf_radial_shading"=>&[F(1),F(2),F(3),F(4),F(5),F(6),F(7),F(8)],
        "pdf_mesh_decode"=>&[F(3)],
        "pdf_pattern"=>&[F(3),F(4),F(5),F(6),F(7),F(8)],
        "pdf_tiling_pattern"=>&[F(3),F(4),F(5),F(6),F(7),F(8)],
        "pdf_destination"=>&[F(5),F(6),F(7),F(8),F(9),F(10),F(11)],
        "pdf_action"=>&[F(9)],
        "pdf_outline"=>&[F(4),F(5),F(6)],
        "pdf_annotation_border"=>&[F(1),F(4),F(5)],
        "pdf_border_dash"=>&[F(3)],
        "pdf_annotation_markup"=>&[F(4)],
        "pdf_annotation"=>&[F(1),F(2),F(3),F(4)],
        "pdf_annotation_real"=>&[F(3)],
        "pdf_annotation_detail"=>&[F(13),F(37),F(38),F(39),F(40)],
        "pdf_annotation_kind_real"=>&[F(4)],
        "pdf_annotation_ink_coordinate"=>&[F(3)],
        "pdf_page"=>&[F(1),F(2),F(3),F(4),F(5),F(6),F(7),F(8),F(9),F(10),F(11),F(12),F(13),F(14),F(15),F(16),F(17),F(18),F(19),F(20),F(22),F(28)],
        _=>&[],
    }
}
enum Rows<'c,'p>{
    Owned(sqlite_snapshot::artifact::Projection<'c,'p>),
    Forecast{control:&'c mut Control<'p>,count:usize,bytes:usize},
}
pub(super) struct Projection<'c,'p>{rows:Rows<'c,'p>}
impl<'c,'p> Projection<'c,'p>{
    pub fn new(sql:&str,control:&'c mut Control<'p>)->Result<Self,ValueError>{Ok(Self{rows:Rows::Owned(sqlite_snapshot::artifact::Projection::new(sql,control)?)})}
    pub fn forecast(control:&'c mut Control<'p>)->Result<Self,ValueError>{control.checkpoint(Phase::EncodeNative,0,0)?;Ok(Self{rows:Rows::Forecast{control,count:0,bytes:0}})}
    pub fn insert(&mut self,table:&str,cells:&[C<'_>])->Result<i64,ValueError>{match &mut self.rows{
        Rows::Owned(inner)=>insert_ieee754(inner,table,cells,columns(table)),
        Rows::Forecast{control,count,bytes}=>forecast_row(control,count,bytes,table,cells),
    }}
    pub fn insert_key(&mut self,table:&str,key:i64,cells:&[C<'_>])->Result<(),ValueError>{match &mut self.rows{
        Rows::Owned(inner)=>insert_key_ieee754(inner,table,key,cells,columns(table)),
        Rows::Forecast{control,count,bytes}=>forecast_row(control,count,bytes,table,cells).map(|_|()),
    }}
    pub fn checkpoint(&mut self)->Result<(),ValueError>{match &mut self.rows{
        Rows::Owned(inner)=>inner.checkpoint(),
        Rows::Forecast{control,count,..}=>control.checkpoint(Phase::EncodeNative,*count,0),
    }}
    pub fn check_rows(&self,count:usize)->Result<(),ValueError>{match &self.rows{
        Rows::Owned(inner)=>inner.check_rows(count),
        Rows::Forecast{control,..}=>control.check_rows(count),
    }}
    pub fn finish(self)->Result<Db,ValueError>{match self.rows{Rows::Owned(inner)=>inner.finish(),Rows::Forecast{..}=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF row forecast owns no SQLite database"))}}
    pub fn finish_forecast(self)->Result<usize,ValueError>{match self.rows{
        Rows::Forecast{control,count,..}=>{control.checkpoint(Phase::EncodeNative,count,count)?;Ok(count)},
        Rows::Owned(_)=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF materialization is not a borrowed forecast")),
    }}
}
fn forecast_row(control:&mut Control<'_>,count:&mut usize,bytes:&mut usize,table:&str,cells:&[C<'_>])->Result<i64,ValueError>{
    let mut row_bytes=8usize;
    for cell in cells{let size=match cell{C::Null=>0,C::Integer(_)|C::Real(_)|C::Float32(_)=>8,C::Text(value)=>value.len(),C::Blob(value)=>value.len()};row_bytes=row_bytes.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PDF semantic cell byte count overflow"))?;}
    for column in columns(table){let index=match column{FloatColumn::Binary64(index)|FloatColumn::Binary32(index)=>*index};let cell=cells.get(index.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"PDF IEEE identity column is invalid"))?).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"PDF IEEE field is missing"))?;let value=match cell{C::Null=>continue,C::Real(value)=>*value,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF IEEE field has an invalid storage class"))};let class=if value.is_nan(){row_bytes-=8;"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};row_bytes=row_bytes.checked_add(8+class.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PDF IEEE semantic byte count overflow"))?;}
    let total=bytes.checked_add(row_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PDF semantic value byte count overflow"))?;
    control.check_value_bytes(total)?;
    let next=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"PDF entity row count overflow"))?;
    control.check_rows(next)?;
    if next%256==0{control.checkpoint(Phase::EncodeNative,next,0)?;}
    *bytes=total;*count=next;i64::try_from(next).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"PDF entity identity exceeds i64"))
}
