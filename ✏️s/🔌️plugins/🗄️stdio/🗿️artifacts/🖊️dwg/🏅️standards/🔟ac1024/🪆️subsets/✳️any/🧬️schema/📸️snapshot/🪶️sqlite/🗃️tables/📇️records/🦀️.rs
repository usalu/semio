//! 📇️ Typed DWG symbol-table records with explicit owned common fields.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::super::*;
use super::{color,drawing::optional_words,reader::{boolean,byte,full_unsigned,high,low,optional_text,optional_unsigned,ordinal,real,word,Reader}};
use semio_framework_os_kernel::sqlite_snapshot::artifact::Cell;
use super::number::Projection;
use Cell::{Integer as I,Real as R,Text as T,Null as N};

pub(super) fn project_record(projection:&mut Projection<'_,'_>,id:i64,value:&DwgTableRecordBody)->Result<(),ValueError>{
    let(kind,common)=match value{DwgTableRecordBody::RegisteredApplication(v)=>("registered_application",&v.common),DwgTableRecordBody::TextStyle(v)=>("text_style",&v.common),DwgTableRecordBody::Layer(v)=>("layer",&v.common),DwgTableRecordBody::Linetype(v)=>("linetype",&v.common),DwgTableRecordBody::BlockHeader(v)=>("block_header",&v.common),DwgTableRecordBody::Viewport(v)=>("viewport",&v.common),DwgTableRecordBody::DimensionStyle(v)=>("dimension_style",&v.common)};
    let xref=optional_words(common.xref_handle);
    projection.insert_key("dwg_table_record",id,&[T(kind),T(&common.name),I(i64::from(common.xref_resolution)),xref[0],xref[1]])?;
    match value{
        DwgTableRecordBody::RegisteredApplication(v)=>projection.insert_key("dwg_registered_application_record",id,&[I(i64::from(v.group_71))]),
        DwgTableRecordBody::TextStyle(v)=>projection.insert_key("dwg_text_style_record",id,&[I(i64::from(v.is_shape)),I(i64::from(v.is_vertical)),R(v.text_size),R(v.width_factor),R(v.oblique_angle),I(i64::from(v.generation)),R(v.last_height),T(&v.font_file),T(&v.big_font_file)]),
        DwgTableRecordBody::Layer(v)=>{
            let plot=optional_words(v.plot_style_handle);let material=optional_words(v.material_handle);let linetype=optional_words(v.linetype_handle);
            projection.insert_key("dwg_layer_record",id,&[I(i64::from(v.frozen)),I(i64::from(v.off)),I(i64::from(v.frozen_in_new_viewports)),I(i64::from(v.locked)),I(i64::from(v.plottable)),I(i64::from(v.lineweight)),plot[0],plot[1],material[0],material[1],linetype[0],linetype[1]])?;
            color::project(projection,"dwg_layer_record_color",id,&v.color)
        },
        DwgTableRecordBody::Linetype(v)=>{
            projection.insert_key("dwg_linetype_record",id,&[T(&v.description),R(v.pattern_length),I(i64::from(v.alignment))])?;
            for(index,value)in v.dashes.iter().enumerate(){let style=optional_words(value.style_handle);projection.insert("dwg_linetype_dash",&[I(id),I(ordinal(index)?),R(value.length),I(i64::from(value.complex_shape_code)),style[0],style[1],R(value.x_offset),R(value.y_offset),R(value.scale),R(value.rotation),I(i64::from(value.shape_flags)),value.text.as_deref().map(T).unwrap_or(N)])?;}
            Ok(())
        },
        DwgTableRecordBody::BlockHeader(v)=>{
            let layout=optional_words(v.layout_handle);
            projection.insert_key("dwg_block_header_record",id,&[I(i64::from(v.anonymous)),I(i64::from(v.has_attributes)),I(i64::from(v.is_xref)),I(i64::from(v.xref_overlaid)),I(i64::from(v.xref_loaded)),R(v.base_point[0]),R(v.base_point[1]),R(v.base_point[2]),T(&v.xref_path),T(&v.description),I(i64::from(v.insert_units)),I(i64::from(v.explodable)),I(i64::from(v.block_scaling)),I(high(v.block_entity_handle)),I(low(v.block_entity_handle)),I(high(v.end_block_entity_handle)),I(low(v.end_block_entity_handle)),layout[0],layout[1]])?;
            for(table,values)in [("dwg_block_header_owned_entity_handle",&v.owned_entity_handles),("dwg_block_header_insert_backreference_handle",&v.insert_backreference_handles)]{for(index,value)in values.iter().enumerate(){projection.insert(table,&[I(id),I(ordinal(index)?),I(high(*value)),I(low(*value))])?;}}
            Ok(())
        },
        DwgTableRecordBody::Viewport(v)=>project_viewport(projection,id,v),
        DwgTableRecordBody::DimensionStyle(v)=>project_dimension_style(projection,id,v)
    }
}
pub(super) fn reconstruct_record(reader:&mut Reader<'_,'_,'_>,id:i64)->Result<DwgTableRecordBody,ValueError>{
    let row=reader.component("dwg_table_record",id)?;
    let common=DwgTableRecordCommon{name:row.text(2)?.into(),xref_resolution:word(row,3)?,xref_handle:optional_unsigned(row,4,5)?};
    match row.text(1)?{
        "registered_application"=>{let r=reader.component("dwg_registered_application_record",id)?;Ok(DwgTableRecordBody::RegisteredApplication(DwgRegisteredApplicationTableRecord{common,group_71:byte(r,1)?}))},
        "text_style"=>{let r=reader.component("dwg_text_style_record",id)?;Ok(DwgTableRecordBody::TextStyle(DwgTextStyleTableRecord{common,is_shape:boolean(r,1)?,is_vertical:boolean(r,2)?,text_size:real(r,3)?,width_factor:real(r,4)?,oblique_angle:real(r,5)?,generation:byte(r,6)?,last_height:real(r,7)?,font_file:r.text(8)?.into(),big_font_file:r.text(9)?.into()}))},
        "layer"=>{let r=reader.component("dwg_layer_record",id)?;Ok(DwgTableRecordBody::Layer(DwgLayerTableRecord{common,frozen:boolean(r,1)?,off:boolean(r,2)?,frozen_in_new_viewports:boolean(r,3)?,locked:boolean(r,4)?,plottable:boolean(r,5)?,lineweight:byte(r,6)?,color:color::reconstruct(reader,"dwg_layer_record_color",id)?,plot_style_handle:optional_unsigned(r,7,8)?,material_handle:optional_unsigned(r,9,10)?,linetype_handle:optional_unsigned(r,11,12)?}))},
        "linetype"=>{
            let r=reader.component("dwg_linetype_record",id)?;
            let dashes=reader.list("dwg_linetype_dash",1,id,2)?.into_iter().map(|row|Ok(DwgLinetypeDash{length:real(row,3)?,complex_shape_code:word(row,4)?,style_handle:optional_unsigned(row,5,6)?,x_offset:real(row,7)?,y_offset:real(row,8)?,scale:real(row,9)?,rotation:real(row,10)?,shape_flags:word(row,11)?,text:optional_text(row,12)?})).collect::<Result<_,ValueError>>()?;
            Ok(DwgTableRecordBody::Linetype(DwgLinetypeTableRecord{common,description:r.text(1)?.into(),pattern_length:real(r,2)?,alignment:byte(r,3)?,dashes}))
        },
        "block_header"=>{
            let r=reader.component("dwg_block_header_record",id)?;
            let owned_entity_handles=reader.list("dwg_block_header_owned_entity_handle",1,id,2)?.into_iter().map(|row|full_unsigned(row,3,4)).collect::<Result<_,_>>()?;
            let insert_backreference_handles=reader.list("dwg_block_header_insert_backreference_handle",1,id,2)?.into_iter().map(|row|full_unsigned(row,3,4)).collect::<Result<_,_>>()?;
            Ok(DwgTableRecordBody::BlockHeader(DwgBlockHeaderTableRecord{common,anonymous:boolean(r,1)?,has_attributes:boolean(r,2)?,is_xref:boolean(r,3)?,xref_overlaid:boolean(r,4)?,xref_loaded:boolean(r,5)?,owned_entity_handles,base_point:[real(r,6)?,real(r,7)?,real(r,8)?],xref_path:r.text(9)?.into(),insert_backreference_handles,description:r.text(10)?.into(),insert_units:word(r,11)?,explodable:boolean(r,12)?,block_scaling:byte(r,13)?,block_entity_handle:full_unsigned(r,14,15)?,end_block_entity_handle:full_unsigned(r,16,17)?,layout_handle:optional_unsigned(r,18,19)?}))
        },
        "viewport"=>reconstruct_viewport(reader,id,common).map(DwgTableRecordBody::Viewport),
        "dimension_style"=>reconstruct_dimension_style(reader,id,common).map(DwgTableRecordBody::DimensionStyle),
        _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG symbol-table record kind is unknown"))
    }
}
fn project_viewport(projection:&mut Projection<'_,'_>,id:i64,v:&DwgViewportTableRecord)->Result<(),ValueError>{
    let background=optional_words(v.background_handle);let visual=optional_words(v.visual_style_handle);let sun=optional_words(v.sun_handle);let named=optional_words(v.named_ucs_handle);let base=optional_words(v.base_ucs_handle);
    projection.insert_key("dwg_viewport_record",id,&[R(v.view_height),R(v.view_width),R(v.center[0]),R(v.center[1]),R(v.target[0]),R(v.target[1]),R(v.target[2]),R(v.direction[0]),R(v.direction[1]),R(v.direction[2]),R(v.twist),R(v.lens_length),R(v.front_clipping),R(v.back_clipping),I(i64::from(v.view_mode[0])),I(i64::from(v.view_mode[1])),I(i64::from(v.view_mode[2])),I(i64::from(v.view_mode[3])),I(i64::from(v.render_mode)),I(i64::from(v.use_default_lights)),I(i64::from(v.default_lighting_type)),R(v.brightness),R(v.contrast),R(v.lower_left[0]),R(v.lower_left[1]),R(v.upper_right[0]),R(v.upper_right[1]),I(i64::from(v.ucs_follow)),I(i64::from(v.circle_zoom)),I(i64::from(v.fast_zoom)),I(i64::from(v.ucs_icon)),I(i64::from(v.grid_mode)),R(v.grid_unit[0]),R(v.grid_unit[1]),I(i64::from(v.snap_mode)),I(i64::from(v.snap_style)),I(i64::from(v.snap_isopair)),R(v.snap_angle),R(v.snap_base[0]),R(v.snap_base[1]),R(v.snap_unit[0]),R(v.snap_unit[1]),I(i64::from(v.ucs_at_origin)),I(i64::from(v.ucs_viewport)),R(v.ucs_origin[0]),R(v.ucs_origin[1]),R(v.ucs_origin[2]),R(v.ucs_x_axis[0]),R(v.ucs_x_axis[1]),R(v.ucs_x_axis[2]),R(v.ucs_y_axis[0]),R(v.ucs_y_axis[1]),R(v.ucs_y_axis[2]),R(v.ucs_elevation),I(i64::from(v.ucs_orthographic_view)),I(i64::from(v.grid_flags)),I(i64::from(v.grid_major)),background[0],background[1],visual[0],visual[1],sun[0],sun[1],named[0],named[1],base[0],base[1]])?;
    color::project(projection,"dwg_viewport_record_ambient_color",id,&v.ambient_color)
}
fn reconstruct_viewport(reader:&mut Reader<'_,'_,'_>,id:i64,common:DwgTableRecordCommon)->Result<DwgViewportTableRecord,ValueError>{
    let r=reader.component("dwg_viewport_record",id)?;
    Ok(DwgViewportTableRecord{common,view_height:real(r,1)?,view_width:real(r,2)?,center:[real(r,3)?,real(r,4)?],target:[real(r,5)?,real(r,6)?,real(r,7)?],direction:[real(r,8)?,real(r,9)?,real(r,10)?],twist:real(r,11)?,lens_length:real(r,12)?,front_clipping:real(r,13)?,back_clipping:real(r,14)?,view_mode:[boolean(r,15)?,boolean(r,16)?,boolean(r,17)?,boolean(r,18)?],render_mode:byte(r,19)?,use_default_lights:boolean(r,20)?,default_lighting_type:byte(r,21)?,brightness:real(r,22)?,contrast:real(r,23)?,ambient_color:color::reconstruct(reader,"dwg_viewport_record_ambient_color",id)?,lower_left:[real(r,24)?,real(r,25)?],upper_right:[real(r,26)?,real(r,27)?],ucs_follow:boolean(r,28)?,circle_zoom:word(r,29)?,fast_zoom:boolean(r,30)?,ucs_icon:byte(r,31)?,grid_mode:boolean(r,32)?,grid_unit:[real(r,33)?,real(r,34)?],snap_mode:boolean(r,35)?,snap_style:boolean(r,36)?,snap_isopair:word(r,37)?,snap_angle:real(r,38)?,snap_base:[real(r,39)?,real(r,40)?],snap_unit:[real(r,41)?,real(r,42)?],ucs_at_origin:boolean(r,43)?,ucs_viewport:boolean(r,44)?,ucs_origin:[real(r,45)?,real(r,46)?,real(r,47)?],ucs_x_axis:[real(r,48)?,real(r,49)?,real(r,50)?],ucs_y_axis:[real(r,51)?,real(r,52)?,real(r,53)?],ucs_elevation:real(r,54)?,ucs_orthographic_view:word(r,55)?,grid_flags:word(r,56)?,grid_major:word(r,57)?,background_handle:optional_unsigned(r,58,59)?,visual_style_handle:optional_unsigned(r,60,61)?,sun_handle:optional_unsigned(r,62,63)?,named_ucs_handle:optional_unsigned(r,64,65)?,base_ucs_handle:optional_unsigned(r,66,67)?})
}
fn project_dimension_style(projection:&mut Projection<'_,'_>,id:i64,v:&DwgDimensionStyleTableRecord)->Result<(),ValueError>{
    let text_style=optional_words(v.text_style_handle);let leader=optional_words(v.leader_arrow_handle);let arrow=optional_words(v.arrow_handle);let arrow1=optional_words(v.arrow_1_handle);let arrow2=optional_words(v.arrow_2_handle);let linetype=optional_words(v.dimension_linetype_handle);let extension1=optional_words(v.extension_1_linetype_handle);let extension2=optional_words(v.extension_2_linetype_handle);
    projection.insert_key("dwg_dimension_style_record",id,&[T(&v.dimension_postfix),T(&v.alternate_postfix),I(i64::from(v.fill_mode)),text_style[0],text_style[1],leader[0],leader[1],arrow[0],arrow[1],arrow1[0],arrow1[1],arrow2[0],arrow2[1],linetype[0],linetype[1],extension1[0],extension1[1],extension2[0],extension2[1]])?;
    let g=&v.geometry;
    projection.insert_key("dwg_dimension_style_geometry",id,&[R(g.scale),R(g.arrow_size),R(g.extension_origin_offset),R(g.dimension_line_increment),R(g.extension_line_extension),R(g.rounding),R(g.dimension_line_extension),R(g.plus_tolerance),R(g.minus_tolerance),R(g.fixed_extension_length),R(g.jog_angle)])?;
    let b=&v.behavior;
    projection.insert_key("dwg_dimension_style_behavior",id,&[I(i64::from(b.tolerance)),I(i64::from(b.limits)),I(i64::from(b.text_inside_horizontal)),I(i64::from(b.text_outside_horizontal)),I(i64::from(b.suppress_extension_1)),I(i64::from(b.suppress_extension_2)),I(i64::from(b.text_vertical_alignment)),I(i64::from(b.zero_suppression)),I(i64::from(b.angular_zero_suppression)),I(i64::from(b.arc_symbol))])?;
    let t=&v.text;
    projection.insert_key("dwg_dimension_style_text",id,&[R(t.height),R(t.center_mark_size),R(t.tick_size),R(t.alternate_scale),R(t.linear_scale),R(t.vertical_position),R(t.tolerance_scale),R(t.gap),R(t.alternate_rounding),I(i64::from(t.alternate_enabled)),I(i64::from(t.alternate_decimals)),I(i64::from(t.text_outside_extensions)),I(i64::from(t.separate_arrowheads)),I(i64::from(t.force_text_inside)),I(i64::from(t.suppress_outside_extensions))])?;
    let u=&v.units;
    projection.insert_key("dwg_dimension_style_units",id,&[I(i64::from(u.alternate_decimal_places)),I(i64::from(u.decimal_places)),I(i64::from(u.tolerance_decimal_places)),I(i64::from(u.alternate_units)),I(i64::from(u.alternate_tolerance_decimal_places)),I(i64::from(u.angular_units)),I(i64::from(u.fraction_format)),I(i64::from(u.linear_units)),I(i64::from(u.decimal_separator)),I(i64::from(u.text_movement)),I(i64::from(u.text_horizontal_alignment)),I(i64::from(u.suppress_dimension_line_1)),I(i64::from(u.suppress_dimension_line_2)),I(i64::from(u.tolerance_vertical_alignment)),I(i64::from(u.tolerance_zero_suppression)),I(i64::from(u.alternate_zero_suppression)),I(i64::from(u.alternate_tolerance_zero_suppression)),I(i64::from(u.user_positioned_text)),I(i64::from(u.arrow_text_fit))])?;
    let r=&v.r2010;
    projection.insert_key("dwg_dimension_style_r2010",id,&[I(i64::from(r.fixed_extension_enabled)),I(i64::from(r.text_direction)),R(r.alternate_measurement_factor),T(&r.alternate_measurement_suffix),R(r.measurement_factor),T(&r.measurement_suffix),I(i64::from(r.dimension_lineweight)),I(i64::from(r.extension_lineweight)),I(i64::from(r.flag))])?;
    color::project(projection,"dwg_dimension_style_fill_color",id,&v.fill_color)?;
    color::project(projection,"dwg_dimension_style_dimension_line_color",id,&t.dimension_line_color)?;
    color::project(projection,"dwg_dimension_style_extension_line_color",id,&t.extension_line_color)?;
    color::project(projection,"dwg_dimension_style_text_color",id,&t.text_color)
}
fn reconstruct_dimension_style(reader:&mut Reader<'_,'_,'_>,id:i64,common:DwgTableRecordCommon)->Result<DwgDimensionStyleTableRecord,ValueError>{
    let row=reader.component("dwg_dimension_style_record",id)?;
    let r=reader.component("dwg_dimension_style_geometry",id)?;
    let geometry=DwgDimensionGeometry{scale:real(r,1)?,arrow_size:real(r,2)?,extension_origin_offset:real(r,3)?,dimension_line_increment:real(r,4)?,extension_line_extension:real(r,5)?,rounding:real(r,6)?,dimension_line_extension:real(r,7)?,plus_tolerance:real(r,8)?,minus_tolerance:real(r,9)?,fixed_extension_length:real(r,10)?,jog_angle:real(r,11)?};
    let r=reader.component("dwg_dimension_style_behavior",id)?;
    let behavior=DwgDimensionBehavior{tolerance:boolean(r,1)?,limits:boolean(r,2)?,text_inside_horizontal:boolean(r,3)?,text_outside_horizontal:boolean(r,4)?,suppress_extension_1:boolean(r,5)?,suppress_extension_2:boolean(r,6)?,text_vertical_alignment:word(r,7)?,zero_suppression:word(r,8)?,angular_zero_suppression:word(r,9)?,arc_symbol:word(r,10)?};
    let r=reader.component("dwg_dimension_style_text",id)?;
    let text=DwgDimensionText{height:real(r,1)?,center_mark_size:real(r,2)?,tick_size:real(r,3)?,alternate_scale:real(r,4)?,linear_scale:real(r,5)?,vertical_position:real(r,6)?,tolerance_scale:real(r,7)?,gap:real(r,8)?,alternate_rounding:real(r,9)?,alternate_enabled:boolean(r,10)?,alternate_decimals:word(r,11)?,text_outside_extensions:boolean(r,12)?,separate_arrowheads:boolean(r,13)?,force_text_inside:boolean(r,14)?,suppress_outside_extensions:boolean(r,15)?,dimension_line_color:color::reconstruct(reader,"dwg_dimension_style_dimension_line_color",id)?,extension_line_color:color::reconstruct(reader,"dwg_dimension_style_extension_line_color",id)?,text_color:color::reconstruct(reader,"dwg_dimension_style_text_color",id)?};
    let r=reader.component("dwg_dimension_style_units",id)?;
    let units=DwgDimensionUnits{alternate_decimal_places:word(r,1)?,decimal_places:word(r,2)?,tolerance_decimal_places:word(r,3)?,alternate_units:word(r,4)?,alternate_tolerance_decimal_places:word(r,5)?,angular_units:word(r,6)?,fraction_format:word(r,7)?,linear_units:word(r,8)?,decimal_separator:word(r,9)?,text_movement:word(r,10)?,text_horizontal_alignment:word(r,11)?,suppress_dimension_line_1:boolean(r,12)?,suppress_dimension_line_2:boolean(r,13)?,tolerance_vertical_alignment:word(r,14)?,tolerance_zero_suppression:word(r,15)?,alternate_zero_suppression:word(r,16)?,alternate_tolerance_zero_suppression:word(r,17)?,user_positioned_text:boolean(r,18)?,arrow_text_fit:word(r,19)?};
    let r=reader.component("dwg_dimension_style_r2010",id)?;
    let r2010=DwgDimensionR2010{fixed_extension_enabled:boolean(r,1)?,text_direction:boolean(r,2)?,alternate_measurement_factor:real(r,3)?,alternate_measurement_suffix:r.text(4)?.into(),measurement_factor:real(r,5)?,measurement_suffix:r.text(6)?.into(),dimension_lineweight:word(r,7)?,extension_lineweight:word(r,8)?,flag:boolean(r,9)?};
    Ok(DwgDimensionStyleTableRecord{common,dimension_postfix:row.text(1)?.into(),alternate_postfix:row.text(2)?.into(),geometry,fill_mode:word(row,3)?,fill_color:color::reconstruct(reader,"dwg_dimension_style_fill_color",id)?,behavior,text,units,r2010,text_style_handle:optional_unsigned(row,4,5)?,leader_arrow_handle:optional_unsigned(row,6,7)?,arrow_handle:optional_unsigned(row,8,9)?,arrow_1_handle:optional_unsigned(row,10,11)?,arrow_2_handle:optional_unsigned(row,12,13)?,dimension_linetype_handle:optional_unsigned(row,14,15)?,extension_1_linetype_handle:optional_unsigned(row,16,17)?,extension_2_linetype_handle:optional_unsigned(row,18,19)?})
}
