//! 🔢️ Authored native binary64 scalar fields for each PDF semantic entity.
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
pub(super) struct Projection<'c,'p>{inner:sqlite_snapshot::artifact::Projection<'c,'p>}
impl<'c,'p> Projection<'c,'p>{
    pub fn new(sql:&str,control:&'c mut Control<'p>)->Result<Self,String>{Ok(Self{inner:sqlite_snapshot::artifact::Projection::new(sql,control)?})}
    pub fn insert(&mut self,table:&str,cells:&[C<'_>])->Result<i64,String>{insert_ieee754(&mut self.inner,table,cells,columns(table))}
    pub fn insert_key(&mut self,table:&str,key:i64,cells:&[C<'_>])->Result<(),String>{insert_key_ieee754(&mut self.inner,table,key,cells,columns(table))}
    pub fn checkpoint(&mut self)->Result<(),String>{self.inner.checkpoint()}
    pub fn check_rows(&self,count:usize)->Result<(),String>{self.inner.check_rows(count)}
    pub fn finish(self)->Result<Db,String>{self.inner.finish()}
}
