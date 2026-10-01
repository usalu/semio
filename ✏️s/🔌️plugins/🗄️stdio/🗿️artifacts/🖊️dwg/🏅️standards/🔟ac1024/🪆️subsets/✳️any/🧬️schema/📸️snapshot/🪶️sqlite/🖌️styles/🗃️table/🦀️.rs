//! 🗃️ DWG table styles expose four typed cell roles and six optional border edges.
use super::super::*;
use super::number::Projection;
use super::color;
use super::drawing::optional_words;
use super::reader::{optional_unsigned,ordinal,signed_integer,unsigned,Reader};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell::{Integer as I,Text as T,Real as R};

pub(super) fn project(p:&mut Projection<'_,'_>,id:i64,v:&DwgTableStyle)->Result<(),String>{
 let template=optional_words(v.template_style_handle);p.insert_key("dwg_table_style",id,&[T(&v.description),I(i64::from(v.bit_flags)),template[0],template[1]])?;
 for(index,(role,v))in [("table",&v.table),("title",&v.title),("header",&v.header),("data",&v.data)].into_iter().enumerate(){
 let style=p.insert("dwg_cell_style",&[I(id),I(ordinal(index)?),T(role),I(i64::from(v.property_override_flags)),I(i64::from(v.merge_flags)),I(i64::from(v.content_layout))])?;
 color::project(p,"dwg_cell_background_color",style,&v.background_color)?;let c=&v.content_format;let handle=optional_words(c.text_style_handle);
 p.insert_key("dwg_cell_content_format",style,&[I(i64::from(c.property_override_flags)),I(i64::from(c.property_flags)),I(i64::from(c.value_data_type)),I(i64::from(c.value_unit_type)),T(&c.value_format_string),R(c.rotation),R(c.block_scale),I(i64::from(c.alignment)),handle[0],handle[1],R(c.text_height)])?;
 color::project(p,"dwg_cell_content_color",style,&c.content_color)?;let m=&v.margins;p.insert_key("dwg_cell_margins",style,&[R(m.vertical),R(m.horizontal),R(m.bottom),R(m.right),R(m.horizontal_spacing),R(m.vertical_spacing)])?;
 let mut ordinal=0;for(edge,v)in [("top",&v.borders.top),("horizontal_inside",&v.borders.horizontal_inside),("bottom",&v.borders.bottom),("left",&v.borders.left),("vertical_inside",&v.borders.vertical_inside),("right",&v.borders.right)]{if let Some(v)=v{let handle=optional_words(v.linetype_handle);let border=p.insert("dwg_cell_border",&[I(style),I(ordinal),T(edge),I(i64::from(v.override_flags)),I(i64::from(v.border_type)),I(i64::from(v.lineweight)),handle[0],handle[1],I(i64::from(v.visible)),R(v.double_line_spacing)])?;color::project(p,"dwg_cell_border_color",border,&v.color)?;ordinal+=1;}}
 }Ok(())
}
fn reconstruct_cell(r:&mut Reader<'_,'_,'_>,row:super::number::Row<'_>,role:&str)->Result<DwgCellStyle,String>{
 if row.text(3)?!=role{return Err("DWG table style cell roles are out of order".into());}
 let id=row.rowid;let c=r.component("dwg_cell_content_format",id)?;let m=r.component("dwg_cell_margins",id)?;
 let mut borders=DwgCellBorders::default();let mut prior=None;
 for b in r.list("dwg_cell_border",1,id,2)?{
 let(index,target)=match b.text(3)?{"top"=>(0,&mut borders.top),"horizontal_inside"=>(1,&mut borders.horizontal_inside),"bottom"=>(2,&mut borders.bottom),"left"=>(3,&mut borders.left),"vertical_inside"=>(4,&mut borders.vertical_inside),"right"=>(5,&mut borders.right),_=>return Err("DWG cell border edge is unknown".into())};
 if prior.is_some_and(|prior|prior>=index){return Err("DWG cell border edges are repeated or out of order".into());}prior=Some(index);
 *target=Some(DwgCellBorder{override_flags:unsigned(b,4)?,border_type:unsigned(b,5)?,color:color::reconstruct(r,"dwg_cell_border_color",b.rowid)?,lineweight:signed_integer(b,6)?,linetype_handle:optional_unsigned(b,7,8)?,visible:unsigned(b,9)?,double_line_spacing:b.real(10)?});
 }
 Ok(DwgCellStyle{property_override_flags:unsigned(row,4)?,merge_flags:unsigned(row,5)?,background_color:color::reconstruct(r,"dwg_cell_background_color",id)?,content_layout:unsigned(row,6)?,content_format:DwgCellContentFormat{property_override_flags:unsigned(c,1)?,property_flags:unsigned(c,2)?,value_data_type:unsigned(c,3)?,value_unit_type:unsigned(c,4)?,value_format_string:c.text(5)?.into(),rotation:c.real(6)?,block_scale:c.real(7)?,alignment:unsigned(c,8)?,content_color:color::reconstruct(r,"dwg_cell_content_color",id)?,text_style_handle:optional_unsigned(c,9,10)?,text_height:c.real(11)?},margins:DwgCellMargins{vertical:m.real(1)?,horizontal:m.real(2)?,bottom:m.real(3)?,right:m.real(4)?,horizontal_spacing:m.real(5)?,vertical_spacing:m.real(6)?},borders})
}
pub(super) fn reconstruct(r:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgTableStyle,String>{
 let row=r.component("dwg_table_style",id)?;let cells=r.list("dwg_cell_style",1,id,2)?;if cells.len()!=4{return Err("DWG table style requires four cell roles".into());}
 Ok(DwgTableStyle{description:row.text(1)?.into(),bit_flags:unsigned(row,2)?,template_style_handle:optional_unsigned(row,3,4)?,table:reconstruct_cell(r,cells[0],"table")?,title:reconstruct_cell(r,cells[1],"title")?,header:reconstruct_cell(r,cells[2],"header")?,data:reconstruct_cell(r,cells[3],"data")?})
}
