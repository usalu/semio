//! 🎭️ DWG authored body vectors exercise all object families before independent SQL editing.
use super::*;
fn color()->DwgComplexColor{DwgComplexColor{index:u16::MAX,value:DwgComplexColorValue::ByColor{red:17,green:29,blue:43},name:Some(String::new()),book_name:Some("book".into())}}
fn expression()->DwgEvaluationExpression{DwgEvaluationExpression{parent_id:-7,major_version:11,minor_version:13,value:DwgEvaluationExpressionValue::Double(17.25),node_id:19}}
fn element()->DwgBlockElement{DwgBlockElement{evaluation_expression:expression(),name:"element".into()}}
fn property()->DwgBlockParameterProperty{DwgBlockParameterProperty{connections:vec![DwgBlockParameterConnection{code:u32::MAX,name:"connection".into()}]}}
fn reference()->DwgNamedEvaluationNodeReference{DwgNamedEvaluationNodeReference{node_id:u32::MAX,expression_name:"node expression".into()}}
fn grip()->DwgBlockGrip{DwgBlockGrip{element:element(),location:vec![1.0,2.0,3.0,4.0],insertion_cycling:true,insertion_cycling_weight:i32::MIN,updated_x:reference(),updated_y:DwgNamedEvaluationNodeReference{node_id:7,expression_name:String::new()}}}
fn two_point()->DwgBlockTwoPointParameter{DwgBlockTwoPointParameter{element:element(),show_properties:true,chain_actions:false,definition_base:vec![1.25,2.5],definition_end:vec![3.75,5.0,6.25],properties:vec![property(),DwgBlockParameterProperty{connections:vec![]}],property_expression_references:vec![DwgPropertyExpressionReference{property_index:u32::MAX,node_id:7}],base_location:DwgBlockParameterBaseLocation::Midpoint}}
fn action()->DwgBlockAction{DwgBlockAction{evaluation_expression:expression(),name:"block action".into(),display_location:vec![3.0,4.0],dependencies:vec![DwgBlockActionDependency{object_handle:u64::MAX}],action_node_ids:vec![7,u32::MAX]}}
fn connection()->DwgBlockActionConnection{DwgBlockActionConnection{node_id:u32::MAX,name:"action connection".into()}}
fn assoc_action()->DwgAssociativeAction{DwgAssociativeAction{status:DwgAssociativeActionStatus::UpToDate,owning_network_handle:Some(u64::MAX),action_body_handle:Some(0),action_index:i32::MIN,maximum_dependency_index:i32::MAX,dependencies:vec![DwgAssociativeActionDependency{owned:false,dependency_handle:u64::MAX}]}}
fn dependency()->DwgAssociativeDependency{DwgAssociativeDependency{status:DwgAssociativeDependencyStatus::UpToDate,is_read_dependency:true,is_write_dependency:false,is_attached_to_object:true,is_delegating_to_owning_action:false,order:i32::MIN,dependent_on_object_handle:u64::MAX,name:Some(String::new()),read_dependency_handle:None,dependency_node_handle:Some(0),dependency_body_handle:Some(u64::MAX),dependency_body_id:i32::MAX}}
fn margins()->DwgCellMargins{DwgCellMargins{vertical:1.25,horizontal:2.5,bottom:3.75,right:5.0,horizontal_spacing:6.25,vertical_spacing:7.5}}
fn cell()->DwgCellStyle{DwgCellStyle{property_override_flags:u32::MAX,merge_flags:3,background_color:color(),content_layout:5,content_format:DwgCellContentFormat{property_override_flags:7,property_flags:11,value_data_type:13,value_unit_type:17,value_format_string:String::new(),rotation:19.25,block_scale:23.5,alignment:29,content_color:color(),text_style_handle:Some(u64::MAX),text_height:31.75},margins:margins(),borders:DwgCellBorders{top:Some(DwgCellBorder{override_flags:37,border_type:41,color:color(),lineweight:i32::MIN,linetype_handle:None,visible:43,double_line_spacing:47.25}),right:Some(DwgCellBorder{override_flags:53,border_type:59,color:color(),lineweight:i32::MAX,linetype_handle:Some(0),visible:61,double_line_spacing:67.5}),..DwgCellBorders::default()}}}
fn map()->DwgMaterialMap{DwgMaterialMap{blend_factor:1.25,projection:DwgMaterialProjection::Sphere,tiling:DwgMaterialTiling::Mirror,scale_to_entity:true,use_current_block_transform:false,transform:vec![1.0,2.0,3.0,4.0],source:DwgMaterialMapSource::CurrentScene}}
fn node()->DwgConstraintNodeCore{DwgConstraintNodeCore{id:i32::MIN,connected_node_ids:vec![0,u32::MAX]}}
fn geometric()->DwgGeometricConstraint{DwgGeometricConstraint{node:node(),owner_node_id:u32::MAX,implied:true,active:false}}
fn geometry()->DwgConstraintGeometry{DwgConstraintGeometry{node:node(),geometry_dependency_handle:Some(u64::MAX),geometry_node_id:7}}
fn constraint_parameter()->DwgBlockLinearConstraintParameter{DwgBlockLinearConstraintParameter{parameter:two_point(),displacement_grip_node_id:u32::MAX,dependency_handle:u64::MAX,expression_name:"constraint".into(),expression_description:String::new(),value:1.25,allowed_values:DwgBlockParameterAllowedValues{values:vec![1.0,2.0]}}}
fn constraint_group()->DwgAssoc2dConstraintGroup{DwgAssoc2dConstraintGroup{action:assoc_action(),do_not_check_newly_added_constraints:true,work_plane:vec![vec![],vec![1.0,2.0,3.0,4.0]],member_action_handles:vec![0,u64::MAX],nodes:vec![
 DwgConstraintNode::ConstrainedImplicitPoint(DwgConstrainedImplicitPoint{geometry:geometry(),point:Some(vec![]),point_kind:u8::MAX,point_index:i32::MIN,curve_node_id:i32::MAX}),
 DwgConstraintNode::PointCurveConstraint(geometric()),
 DwgConstraintNode::ConstrainedBoundedLine(DwgConstrainedBoundedLine{geometry:geometry(),origin:vec![1.0,2.0],direction:vec![3.0,4.0],ray:true,bounded:false,start_point:vec![],end_point:vec![5.0,6.0,7.0]}),
 DwgConstraintNode::PointCoincidenceConstraint(geometric()),
 DwgConstraintNode::DistanceConstraint(DwgDistanceConstraint{explicit:DwgExplicitConstraint{geometric:geometric(),value_dependency_handle:u64::MAX,dimension_dependency_handle:0},direction_kind:u8::MAX,direction:None}),
 DwgConstraintNode::PerpendicularConstraint(geometric()),
 DwgConstraintNode::HorizontalConstraint(DwgAxisConstraint{geometric:geometric(),datum_line_index:i32::MIN}),
 DwgConstraintNode::ParallelConstraint(geometric()),
 DwgConstraintNode::MidPointConstraint(geometric()),
 DwgConstraintNode::EqualLengthConstraint(geometric()),
 DwgConstraintNode::ColinearConstraint(geometric()),
 DwgConstraintNode::ConstrainedDatumLine(DwgConstrainedDatumLine{geometry:geometry(),origin:vec![1.0,2.0],direction:vec![3.0,4.0,5.0]}),
 DwgConstraintNode::FixedConstraint(geometric()),
 DwgConstraintNode::VerticalConstraint(DwgAxisConstraint{geometric:geometric(),datum_line_index:i32::MAX})
]}}
fn visual_style()->DwgVisualStyle{
 let int=|value|DwgVisualStyleProperty{value,operation:DwgVisualStylePropertyOperation::Enable};
 let num=|value|DwgVisualStyleProperty{value,operation:DwgVisualStylePropertyOperation::Disable};
 let col=||DwgVisualStyleProperty{value:color(),operation:DwgVisualStylePropertyOperation::Inherit};
 DwgVisualStyle{description:"visual".into(),style_type:u32::MAX,extension_lighting_model:u16::MAX,internal_only:true,properties:DwgVisualStyleProperties{
 face_lighting_model:int(1),face_lighting_quality:int(2),face_color_mode:int(3),face_modifiers:DwgVisualStyleProperty{value:u16::MAX,operation:DwgVisualStylePropertyOperation::Set},face_opacity:num(5.25),face_specular_amount:num(6.5),face_monochrome_color:col(),edge_model:int(7),edge_styles:int(8),edge_intersection_color:col(),edge_obscured_color:col(),edge_obscured_line_pattern:int(9),edge_intersection_line_pattern:int(10),edge_crease_angle:num(11.75),edge_modifiers:int(12),edge_color:col(),edge_opacity:num(13.25),edge_width:int(14),edge_overhang:int(15),edge_jitter:int(16),edge_silhouette_color:col(),edge_silhouette_width:int(17),edge_halo_gap:int(18),edge_isolines:int(19),hidden_edge_precision:DwgVisualStyleProperty{value:true,operation:DwgVisualStylePropertyOperation::Enable},display_settings:int(20),display_brightness:num(21.5),display_shadow_type:int(u32::MAX)
 }}
}
fn mleader()->DwgMLeaderStyle{DwgMLeaderStyle{
 content_type:DwgMLeaderContentType::Block,draw_order:DwgMLeaderDrawOrder::ContentFirst,leader_order:DwgMLeaderLeaderOrder::TailFirst,maximum_segment_points:u32::MAX,first_segment_angle:1.25,second_segment_angle:2.5,
 leader:DwgMLeaderLeaderStyle{kind:DwgMLeaderKind::Spline,color:color(),linetype_style_handle:u64::MAX,lineweight:i32::MIN},landing:DwgMLeaderLanding{enabled:true,gap:3.75},dogleg:DwgMLeaderDogleg{enabled:false,length:5.0},description:"multileader".into(),arrow:DwgMLeaderArrow{symbol_handle:None,size:6.25},
 text:DwgMLeaderTextStyle{default_content:String::new(),style_handle:u64::MAX,left_attachment:DwgMLeaderTextAttachment::BottomOfTopNoUnderline,right_attachment:DwgMLeaderTextAttachment::Center,angle:DwgMLeaderTextAngle::AlwaysRightReading,alignment:DwgMLeaderTextAlignment::Right,color:color(),height:7.5,frame:true,always_left:false,alignment_space:8.75,attachment_direction:DwgMLeaderAttachmentDirection::Vertical,top_attachment:DwgMLeaderTextAttachment::TopOfTop,bottom_attachment:DwgMLeaderTextAttachment::BottomLine},
 block:DwgMLeaderBlockStyle{content_handle:Some(0),color:color(),scale:vec![1.0,2.0,3.0,4.0],use_scale:true,rotation:10.0,use_rotation:false,connection:DwgMLeaderBlockConnection::BasePoint},overall_scale:11.25,property_overrides_changed:true,annotative:false,break_size:12.5
}}
pub(super) fn authored_bodies()->Vec<DwgLogicalObjectBody>{vec![
 DwgLogicalObjectBody::XRecord(DwgXRecordBody{cloning_flag:u16::MAX,values:vec![],object_id_handles:vec![u64::MAX,0]}),
 DwgLogicalObjectBody::AssociativeDependency(dependency()),
 DwgLogicalObjectBody::AssociativeValueDependency(DwgAssociativeValueDependency{dependency:dependency(),cached_value:DwgEvaluationVariant::Integer32(i32::MIN),value_name:String::new()}),
 DwgLogicalObjectBody::AssociativeGeometryDependency(DwgAssociativeGeometryDependency{dependency:dependency(),enabled:true,persistent_subentity_class_name:"geometry".into(),dependent_on_compound_object:false}),
 DwgLogicalObjectBody::BlockGripLocationComponent(DwgBlockGripLocationComponent{evaluation_expression:expression(),grip_type:u32::MAX,grip_expression:String::new()}),
 DwgLogicalObjectBody::DynamicBlockProxyNode(DwgDynamicBlockProxyNode{evaluation_expression:expression()}),
 DwgLogicalObjectBody::AssociativeVariable(DwgAssociativeVariable{action:assoc_action(),name:"variable".into(),expression:"2+3".into(),evaluator_id:String::new(),description:String::new(),evaluated_value:DwgEvaluationVariant::Integer32(i32::MAX),mergeable:true,mergeable_variable_name:Some(String::new()),must_merge:false,referenced_value_dependency_handles:vec![u64::MAX,0]}),
 DwgLogicalObjectBody::AssociativeDimensionDependencyBody(DwgAssociativeDimensionDependencyBody{name:String::new()}),
 DwgLogicalObjectBody::VisualStyle(visual_style()),
 DwgLogicalObjectBody::BlockParameterDependencyBody(DwgBlockParameterDependencyBody{name:"parameter dependency".into()}),
 DwgLogicalObjectBody::BlockRepresentationData(DwgBlockRepresentationData{represented_block_header_handle:u64::MAX}),
 DwgLogicalObjectBody::DynamicBlockPurgePreventer(DwgDynamicBlockPurgePreventer{protected_block_header_handle:0}),
 DwgLogicalObjectBody::EvaluationGraph(DwgEvaluationGraph{nodes:vec![],edges:vec![]}),
 DwgLogicalObjectBody::BlockFlipParameter(DwgBlockFlipParameter{evaluation_expression:expression(),name:"flip".into(),show_properties:true,chain_actions:false,definition_base:vec![1.0,2.0],definition_end:vec![],properties:vec![property()],base_location:DwgBlockParameterBaseLocation::Midpoint,label:String::new(),description:String::new(),value_set:DwgBlockFlipValueSet{base_label:"base".into(),flipped_label:"flipped".into()},label_point:vec![3.0,4.0],updated_flip:reference()}),
 DwgLogicalObjectBody::BlockVisibilityParameter(DwgBlockVisibilityParameter{evaluation_expression:expression(),element_name:"visibility element".into(),show_properties:false,chain_actions:true,definition_point:vec![],properties:vec![property()],updated_visibility_node_id:u32::MAX,initialized:true,name:"visibility".into(),description:String::new(),evaluation_history:DwgVisibilityEvaluationHistory::Required,eligible_entity_handles:vec![u64::MAX,0],states:vec![DwgVisibilityState{name:String::new(),visible_entity_handles:vec![],controlled_expression_handles:vec![0,u64::MAX]},DwgVisibilityState{name:"second".into(),visible_entity_handles:vec![u64::MAX],controlled_expression_handles:vec![]}]}),
 DwgLogicalObjectBody::Placeholder(DwgPlaceholder{}),
 DwgLogicalObjectBody::DictionaryVariable(DwgDictionaryVariable{value:String::new()}),
 DwgLogicalObjectBody::AnnotationScale(DwgAnnotationScale{name:"scale".into(),paper_units:1.25,drawing_units:2.5,is_unit_scale:false}),
 DwgLogicalObjectBody::SortEntitiesTable(DwgSortEntitiesTable{block_header_handle:u64::MAX,entries:vec![DwgDrawOrderEntry{entity_handle:0,sort_handle:u64::MAX}]}),
 DwgLogicalObjectBody::TableStyle(Box::new(DwgTableStyle{description:"table style".into(),bit_flags:u32::MAX,template_style_handle:Some(0),table:cell(),title:cell(),header:cell(),data:cell()})),
 DwgLogicalObjectBody::MlineStyle(DwgMlineStyle{name:"multiline".into(),description:String::new(),fill_enabled:true,display_miters:false,start_caps:DwgMlineCaps{square:true,inner_arcs:false,round_outer_arcs:true},end_caps:DwgMlineCaps{square:false,inner_arcs:true,round_outer_arcs:false},fill_color:color(),start_angle:1.25,end_angle:2.5,elements:vec![DwgMlineStyleElement{offset:3.75,color:color(),linetype:DwgMlineLinetype::ByBlock}]}),
 DwgLogicalObjectBody::MLeaderStyle(mleader()),
 DwgLogicalObjectBody::Material(DwgMaterial{name:"material".into(),description:String::new(),ambient:DwgMaterialColor{factor:1.25,override_rgb:None},diffuse:DwgMaterialColor{factor:2.5,override_rgb:Some(0)},specular:DwgMaterialColor{factor:3.75,override_rgb:Some(u32::MAX)},diffuse_map:map(),specular_map:map(),reflection_map:map(),opacity_map:map(),bump_map:map(),refraction_map:map(),specular_gloss:5.0,opacity:6.25,refraction_index:7.5,translucence:8.75,self_illumination:10.0,reflectivity:11.25,enabled_channels:DwgMaterialChannels{diffuse:true,specular:false,reflection:true,opacity:false,bump:true,refraction:false}}),
 DwgLogicalObjectBody::BlockMoveAction(DwgBlockMoveAction{action:action(),x_connection:connection(),y_connection:DwgBlockActionConnection{node_id:0,name:String::new()},distance_multiplier:1.25,angle_offset:2.5,coordinate_mode:DwgBlockMoveCoordinateMode::CartesianXy}),
 DwgLogicalObjectBody::AssocNetwork(DwgAssocNetwork{action:assoc_action(),network_action_index:i32::MIN,actions:vec![DwgAssocNetworkMember{handle:u64::MAX,kind:DwgAssocNetworkMemberKind::Network},DwgAssocNetworkMember{handle:0,kind:DwgAssocNetworkMemberKind::Action}]}),
 DwgLogicalObjectBody::Assoc2dConstraintGroup(constraint_group()),
 DwgLogicalObjectBody::BlockLinearParameter(DwgBlockLinearParameter{parameter:two_point(),distance_name:"distance".into(),distance_description:String::new(),label_offset:1.25,allowed_values:vec![2.5,3.75]}),
 DwgLogicalObjectBody::BlockLinearGrip(DwgBlockLinearGrip{grip:grip(),orientation:vec![]}),
 DwgLogicalObjectBody::BlockFlipGrip(DwgBlockFlipGrip{grip:grip(),updated_flip:reference(),orientation:vec![3.0,4.0]}),
 DwgLogicalObjectBody::BlockVisibilityGrip(DwgBlockVisibilityGrip{grip:grip()}),
 DwgLogicalObjectBody::BlockAlignmentParameter(DwgBlockAlignmentParameter{parameter:two_point(),updated_grip_node_id:u32::MAX,align_perpendicular:true}),
 DwgLogicalObjectBody::BlockAlignmentGrip(DwgBlockAlignmentGrip{grip:grip(),first_location_node_id:0,second_location_node_id:u32::MAX,orientation:vec![3.0,4.0]}),
 DwgLogicalObjectBody::BlockStretchAction(DwgBlockStretchAction{action:action(),x_connection:connection(),y_connection:connection(),points:vec![vec![],vec![1.25,2.5]],selections:vec![DwgStretchSelection{object_handle:u64::MAX,vertex_indices:vec![0,u32::MAX]}],selectors:vec![DwgStretchSelector{node_id:u32::MAX,point_indices:vec![]}],distance_multiplier:3.75,angle_offset:5.0,coordinate_mode:DwgBlockActionCoordinateMode::CartesianXy}),
 DwgLogicalObjectBody::BlockScaleAction(DwgBlockScaleAction{base:DwgBlockActionWithBasePoint{action:action(),offset:vec![],x_base_connection:connection(),y_base_connection:connection(),dependent:true,base_point:vec![1.25,2.5]},uniform_scale_connection:connection(),x_scale_connection:connection(),y_scale_connection:connection(),mode:DwgBlockScaleMode::Xy}),
 DwgLogicalObjectBody::BlockFlipAction(DwgBlockFlipAction{action:action(),flip_connection:connection(),updated_flip_connection:connection(),updated_base_connection:connection(),updated_end_connection:connection()}),
 DwgLogicalObjectBody::BlockBasePointParameter(DwgBlockBasePointParameter{parameter:DwgBlockOnePointParameter{element:element(),show_properties:true,chain_actions:false,definition_point:vec![],properties:vec![property()]},point:vec![1.25,2.5],base_point:vec![]}),
 DwgLogicalObjectBody::BlockVerticalConstraintParameter(constraint_parameter()),
 DwgLogicalObjectBody::BlockHorizontalConstraintParameter(constraint_parameter())
]}
