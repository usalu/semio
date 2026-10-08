//! 🔢️ Primitive DWG numeric cells retain finite values, signed zero, NaN and infinities in authored columns.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteRow,SqliteValue,SqliteSnapshotControl,artifact::{Cell,Projection as PhysicalProjection}};
use Cell::{Null as N,Text as T,Real as R,Integer as I};

pub(super) fn numeric_positions(table:&str)->&'static [usize]{match table{
    "dwg_annotation_scale"=>&[2,3],
    "dwg_constraint_work_plane_coordinate"=>&[3],"dwg_constraint_coordinate"=>&[5],
    "dwg_mleader_style"=>&[5,6,8,11],"dwg_mleader_landing"|"dwg_mleader_dogleg"=>&[2],"dwg_mleader_arrow"=>&[3],"dwg_mleader_text_style"=>&[8,11],"dwg_mleader_block_style"=>&[4],"dwg_mleader_block_scale_coordinate"=>&[3],
    "dwg_layout"=>&[9,10,12,18],"dwg_layout_coordinate"=>&[5],
    "dwg_cell_content_format"=>&[6,7,11],"dwg_cell_margins"=>&[1,2,3,4,5,6],"dwg_cell_border"=>&[10],
    "dwg_material"=>&[3,4,5,6,7,8],"dwg_material_color"|"dwg_material_map"=>&[4],"dwg_material_map_transform_coordinate"=>&[3],
    "dwg_mline_style"=>&[11,12],"dwg_mline_style_element"=>&[3],
    "dwg_visual_style_face"=>&[9,11],"dwg_visual_style_edge"=>&[11,16],"dwg_visual_style_display"=>&[3],
    "dwg_block_move_action"|"dwg_block_stretch_action"=>&[5,6],
    "dwg_block_action_display_coordinate"|"dwg_stretch_point_coordinate"|"dwg_block_action_offset_coordinate"|"dwg_block_action_base_point_coordinate"=>&[3],
    "dwg_block_linear_parameter"=>&[3],
    "dwg_block_linear_constraint_parameter"=>&[7],
    "dwg_block_grip_location_coordinate"|"dwg_block_two_point_definition_base_coordinate"|"dwg_block_two_point_definition_end_coordinate"|"dwg_block_one_point_definition_coordinate"|"dwg_block_linear_allowed_value"|"dwg_block_linear_grip_orientation_coordinate"|"dwg_block_flip_grip_orientation_coordinate"|"dwg_block_alignment_grip_orientation_coordinate"|"dwg_block_base_point_coordinate"|"dwg_block_base_point_base_coordinate"|"dwg_block_linear_constraint_allowed_value"|"dwg_block_flip_definition_base_coordinate"|"dwg_block_flip_definition_end_coordinate"|"dwg_block_flip_label_point_coordinate"|"dwg_block_visibility_definition_coordinate"=>&[3],
    "dwg_evaluation_expression"=>&[6],
    "dwg_evaluation_point_group_10_coordinate"|"dwg_evaluation_point_group_11_coordinate"=>&[3],
    "dwg_header_units"=>&[2,3,4,5],
    "dwg_header_scalars"=>&[2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,24],
    "dwg_header_space"=>&[3],
    "dwg_header_dimension_settings"=>&[2,3,4,5,6,7,8,9,10,11,12,25,26,27,28,29,30,31,32,33,64,65],
    "dwg_header_drawing_policy"=>&[25,26,27,28,29,32,33,34,35,36,37,40,41,42,51],
    "dwg_header_space_insertion_base"|"dwg_header_space_extents_minimum"|"dwg_header_space_extents_maximum"|"dwg_header_space_limits_minimum"|"dwg_header_space_limits_maximum"|"dwg_header_space_ucs_origin"|"dwg_header_space_ucs_x_axis"|"dwg_header_space_ucs_y_axis"|"dwg_header_space_ucs_origin_top"|"dwg_header_space_ucs_origin_bottom"|"dwg_header_space_ucs_origin_left"|"dwg_header_space_ucs_origin_right"|"dwg_header_space_ucs_origin_front"|"dwg_header_space_ucs_origin_back"|"dwg_drawing_extmin"|"dwg_drawing_extmax"=>&[3],
    "dwg_xrecord_value"=>&[7,9,10,11],
    "dwg_text_style_record"=>&[3,4,5,7],"dwg_linetype_record"=>&[2],"dwg_linetype_dash"=>&[3,7,8,9,10],"dwg_block_header_record"=>&[6,7,8],
    "dwg_viewport_record"=>&[1,2,3,4,5,6,7,8,9,10,11,12,13,14,22,23,24,25,26,27,33,34,38,39,40,41,42,45,46,47,48,49,50,51,52,53,54],
    "dwg_dimension_style_geometry"=>&[1,2,3,4,5,6,7,8,9,10,11],"dwg_dimension_style_text"=>&[1,2,3,4,5,6,7,8,9],"dwg_dimension_style_r2010"=>&[3,5],
    "dwg_entity_common"=>&[2],"dwg_line_entity"=>&[1],"dwg_arc_entity"=>&[1,2,3,4],"dwg_lwpolyline_entity"=>&[2,3,4],"dwg_lwpolyline_vertex"=>&[3,5,6],"dwg_insert_entity"=>&[1],"dwg_point_entity"=>&[1,2],"dwg_circle_entity"=>&[1,2],"dwg_ellipse_entity"=>&[1,2,3],"dwg_text_entity"=>&[1,3,4,5,6,7],"dwg_spline_entity"=>&[5,6],
    "dwg_lwpolyline_vertex_coordinate"|"dwg_entity_line_start_coordinate"|"dwg_entity_line_end_coordinate"|"dwg_entity_extrusion_coordinate"|"dwg_entity_center_coordinate"|"dwg_entity_insert_insertion_coordinate"|"dwg_entity_insert_scale_coordinate"|"dwg_entity_point_coordinate"|"dwg_entity_ellipse_major_axis_coordinate"|"dwg_entity_text_insertion_coordinate"|"dwg_entity_text_alignment_coordinate"|"dwg_entity_spline_knot"|"dwg_entity_spline_control_coordinate"|"dwg_entity_spline_weight"|"dwg_entity_face3d_corner_coordinate"|"dwg_entity_vertex_coordinate"=>&[3],
    "dwg_linear_dimension_entity"=>&[1,5,6,7,10,11,18,19],
    "dwg_dimension_text_midpoint_coordinate"|"dwg_dimension_insertion_scale_coordinate"|"dwg_dimension_clone_insertion_coordinate"|"dwg_dimension_extension_line_1_coordinate"|"dwg_dimension_extension_line_2_coordinate"|"dwg_dimension_definition_coordinate"=>&[3],
    "dwg_viewport_entity"=>&[1,2,3,4,5,6,7,8,15,20,21],
    "dwg_viewport_view_target_coordinate"|"dwg_viewport_view_direction_coordinate"|"dwg_viewport_view_center_coordinate"|"dwg_viewport_snap_base_coordinate"|"dwg_viewport_snap_unit_coordinate"|"dwg_viewport_grid_unit_coordinate"|"dwg_viewport_ucs_origin_coordinate"|"dwg_viewport_ucs_x_axis_coordinate"|"dwg_viewport_ucs_y_axis_coordinate"=>&[3],
    _=>&[]
}}
fn cells(value:f64)->[Cell<'static>;3]{
    let bits=I(value.to_bits() as i64);
    if value.is_nan(){[T("nan"),bits,N]}else if value==f64::INFINITY{[T("positive_infinity"),bits,N]}else if value==f64::NEG_INFINITY{[T("negative_infinity"),bits,N]}else if value==0.0&&value.is_sign_negative(){[T("negative_zero"),bits,N]}else{[T("finite"),bits,R(value)]}
}
enum Target<'c,'p>{Database(PhysicalProjection<'c,'p>),Admission{control:&'c mut SqliteSnapshotControl<'p>,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase,identifiers:std::collections::BTreeMap<&'static str,i64>}}
pub(super) struct Projection<'c,'p>{target:Target<'c,'p>,rows:usize,bytes:usize}
impl<'c,'p> Projection<'c,'p>{
    pub(super) fn new(sql:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{Ok(Self{target:Target::Database(PhysicalProjection::new(sql,control)?),rows:0,bytes:0})}
    pub(super) fn admission(sql:&str,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{
        control.checkpoint(phase,0,0)?;
        if sql.len()>control.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"DWG authored SQL exceeds schema byte limit"))}
        if control.limits().max_tables<277{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"DWG authored schema exceeds table limit"))}
        Ok(Self{target:Target::Admission{control,phase,identifiers:std::collections::BTreeMap::new()},rows:0,bytes:0})
    }
    fn limits(&self)->semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits{match &self.target{Target::Database(physical)=>physical.limits(),Target::Admission{control,..}=>control.limits()}}
    fn measure(&self,table:&str,values:&[Cell<'_>])->Result<(usize,usize),ValueError>{
        let positions=numeric_positions(table);
        let columns=values.len().checked_add(positions.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DWG numeric column count overflow"))?).and_then(|value|value.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DWG numeric column count overflow"))?;
        if columns>self.limits().max_columns{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"DWG numeric row exceeds column limit"));}
        if positions.last().is_some_and(|position|*position>values.len()){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"DWG authored numeric position exceeds its row"));}
        let mut bytes=self.bytes.checked_add(8).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DWG numeric byte count overflow"))?;
        for(index,value)in values.iter().copied().enumerate(){
            let numeric=positions.binary_search(&(index+1)).is_ok();
            if numeric{match value{N=>{},R(value)=>{let encoded=cells(value);for value in encoded{bytes=bytes.checked_add(match value{T(value)=>value.len(),R(_)|I(_)=>8,_=>0}).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DWG numeric byte count overflow"))?;}},_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"DWG authored numeric cell must be REAL or NULL"))}}
            else{bytes=bytes.checked_add(match value{N=>0,Cell::Integer(_)=>8,T(value)=>value.len(),Cell::PagedText(value)=>value.text_bytes(),Cell::Blob(value)=>value.len(),R(_)|Cell::Float32(_)=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"DWG REAL has no authored numeric columns"))}).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DWG numeric byte count overflow"))?;}
        }
        let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DWG numeric row count overflow"))?;
        match &self.target{Target::Database(physical)=>{physical.check_rows(rows)?;physical.check_value_bytes(bytes)?},Target::Admission{control,..}=>{control.check_rows(rows)?;control.check_value_bytes(bytes)?}}
        Ok((columns,bytes))
    }
    fn encode<'a>(table:&str,values:&[Cell<'a>],columns:usize)->Vec<Cell<'a>>{
        let positions=numeric_positions(table);
        let mut encoded=Vec::with_capacity(columns-1);
        for(index,value)in values.iter().copied().enumerate(){if positions.binary_search(&(index+1)).is_ok(){match value{N=>encoded.extend([N,N,N]),R(value)=>encoded.extend(cells(value)),_=>unreachable!()}}else{encoded.push(value);}}
        encoded
    }
    pub(super) fn insert(&mut self,table:&'static str,values:&[Cell<'_>])->Result<i64,ValueError>{
        let(columns,bytes)=self.measure(table,values)?;
        let id=match &mut self.target{Target::Database(physical)=>physical.insert(table,&Self::encode(table,values,columns))?,Target::Admission{identifiers,..}=>{let count=identifiers.entry(table).or_default();*count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DWG surrogate count overflow"))?;*count}};
        self.rows+=1;self.bytes=bytes;if self.rows%256==0{self.checkpoint()?;}Ok(id)
    }
    pub(super) fn insert_key(&mut self,table:&'static str,id:i64,values:&[Cell<'_>])->Result<(),ValueError>{
        let(columns,bytes)=self.measure(table,values)?;
        match &mut self.target{Target::Database(physical)=>physical.insert_key(table,id,&Self::encode(table,values,columns))?,Target::Admission{identifiers,..}=>{let count=identifiers.entry(table).or_default();*count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DWG surrogate count overflow"))?;}}
        self.rows+=1;self.bytes=bytes;if self.rows%256==0{self.checkpoint()?;}Ok(())
    }
    pub(super) fn checkpoint(&mut self)->Result<(),ValueError>{match &mut self.target{Target::Database(physical)=>physical.checkpoint(),Target::Admission{control,phase,..}=>control.checkpoint(*phase,self.rows,0)}}
    pub(super) fn finish_admission(self)->Result<usize,ValueError>{match self.target{Target::Admission{control,phase,..}=>{control.checkpoint(phase,self.rows,self.rows)?;Ok(self.rows)},Target::Database(_)=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"DWG admission requires its owned counting target"))}}
    pub(super) fn finish(self)->Result<semio_framework_os_kernel::sqlite_snapshot::SqliteDatabase,ValueError>{match self.target{Target::Database(physical)=>physical.finish(),Target::Admission{..}=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"DWG counting target cannot materialize database rows"))}}
}
#[derive(Clone,Copy)]
pub(super) struct Row<'a>{raw:&'a SqliteRow,numeric:&'static[usize],pub(super) rowid:i64}
impl<'a> Row<'a>{
    pub(super) fn new(table:&str,raw:&'a SqliteRow)->Self{Self{raw,numeric:numeric_positions(table),rowid:raw.rowid}}
    pub(super) fn raw(self)->&'a SqliteRow{self.raw}
    fn position(self,column:usize)->usize{column+2*self.numeric.partition_point(|value|*value<column)}
    pub(super) fn value(self,column:usize)->Result<Cell<'a>,ValueError>{
        let index=self.position(column);
        if self.numeric.binary_search(&column).is_ok(){
            match (self.raw.values.get(index),self.raw.values.get(index+1),self.raw.values.get(index+2)){
                (Some(SqliteValue::Null),Some(SqliteValue::Null),Some(SqliteValue::Null))=>Ok(N),
                (Some(SqliteValue::Text(kind)),Some(SqliteValue::Integer(bits)),value)=>{
                    let result=f64::from_bits(*bits as u64);
                    let expected=cells(result);
                    if !matches!(expected[0],T(class) if class==kind){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG numeric class disagrees with IEEE-754 identity"));}
                    if kind=="finite"{if self.raw.real(index+2)?.to_bits()!=result.to_bits(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG query REAL disagrees with IEEE-754 identity"));}}
                    else if !matches!(value,Some(SqliteValue::Null)){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG special numeric class requires a NULL REAL"));}
                    Ok(R(result))
                },
                _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG numeric class/value presence is mismatched"))
            }
        }else{match self.raw.values.get(index){Some(SqliteValue::Null)=>Ok(N),Some(SqliteValue::Integer(value))=>Ok(Cell::Integer(*value)),Some(SqliteValue::Real(_))=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG REAL has no authored numeric class")),Some(SqliteValue::Text(value))=>Ok(T(value)),Some(SqliteValue::Blob(value))=>Ok(Cell::Blob(value)),None=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG row column is missing"))}}
    }
    pub(super) fn integer(self,column:usize)->Result<i64,ValueError>{match self.value(column)?{Cell::Integer(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG column requires INTEGER"))}}
    pub(super) fn real(self,column:usize)->Result<f64,ValueError>{match self.value(column)?{R(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG numeric column requires a present value"))}}
    pub(super) fn text(self,column:usize)->Result<&'a str,ValueError>{match self.value(column)?{T(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG column requires TEXT"))}}
}
