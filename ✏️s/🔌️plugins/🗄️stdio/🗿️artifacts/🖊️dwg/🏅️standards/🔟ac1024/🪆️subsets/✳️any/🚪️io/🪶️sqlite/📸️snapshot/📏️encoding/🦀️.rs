//! 📏️ Borrowed DWG domain bounds before native Binary/Text ownership and printing.
use semio_framework_value::ValueError;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,artifact::NativeEncodingBound};
type Bound<'c,'p>=NativeEncodingBound<'c,'p>;
fn text(value:&str,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(64)?;b.repeated(value.len(),6)}
fn optional_text(value:&Option<String>,b:&mut Bound<'_,'_>)->Result<(),ValueError>{if let Some(value)=value{text(value,b)?;}Ok(())}
fn reals(values:&[f64],b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(64)?;for _ in values{b.add(1164)?;}Ok(())}
fn handles(values:&[u64],b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(64)?;for _ in values{b.add(96)?;}Ok(())}
fn integers(values:&[u32],b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(64)?;for _ in values{b.add(96)?;}Ok(())}
fn color(value:&DwgComplexColor,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(1024)?;optional_text(&value.name,b)?;optional_text(&value.book_name,b)}
fn xrecord(value:&DwgXRecordValue,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(512)?;match value{DwgXRecordValue::String{value,..}=>text(value,b),DwgXRecordValue::Real{..}=>b.add(1100),DwgXRecordValue::Point3d{..}=>b.add(3300),DwgXRecordValue::Binary{octets,..}=>b.repeated(octets.len(),8),DwgXRecordValue::Boolean{..}|DwgXRecordValue::Integer8{..}|DwgXRecordValue::Integer16{..}|DwgXRecordValue::Integer32{..}|DwgXRecordValue::Integer64{..}|DwgXRecordValue::Handle{..}|DwgXRecordValue::ObjectId{..}=>Ok(())}}
fn expression(value:&DwgEvaluationExpression,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(1024)?;match &value.value{DwgEvaluationExpressionValue::Empty|DwgEvaluationExpressionValue::Integer32(_)|DwgEvaluationExpressionValue::Integer16(_)|DwgEvaluationExpressionValue::ObjectReference(_)=>Ok(()),DwgEvaluationExpressionValue::Double(_)=>b.add(1100),DwgEvaluationExpressionValue::String(value)=>text(value,b),DwgEvaluationExpressionValue::PointGroup10(values)|DwgEvaluationExpressionValue::PointGroup11(values)=>reals(values,b)}}
fn dependency(value:&DwgAssociativeDependency,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(2048)?;optional_text(&value.name,b)}
fn action(value:&DwgAssociativeAction,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(1024)?;for _ in &value.dependencies{b.add(256)?;}Ok(())}
fn common(value:&DwgEntityCommon,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(4096)?;optional_text(&value.color.name,b)?;optional_text(&value.color.book_name,b)}
fn dimension(value:&DwgDimensionEntityCommon,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(12288)?;common(&value.common,b)?;text(&value.user_text,b)?;for values in [&value.extrusion,&value.text_midpoint,&value.insertion_scale,&value.clone_insertion_point]{reals(values,b)?;}Ok(())}
fn entity(value:&DwgEntityBody,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(512)?;match value{
 DwgEntityBody::Line(v)=>{b.add(2048)?;common(&v.common,b)?;reals(&v.start,b)?;reals(&v.end,b)?;reals(&v.extrusion,b)},
 DwgEntityBody::Arc(v)=>{b.add(6144)?;common(&v.common,b)?;reals(&v.center,b)?;reals(&v.extrusion,b)},
 DwgEntityBody::LwPolyline(v)=>{b.add(6144)?;common(&v.common,b)?;reals(&v.extrusion,b)?;for vertex in &v.vertices{b.add(4096)?;reals(&vertex.point,b)?;}Ok(())},
 DwgEntityBody::BlockBegin(v)=>common(&v.common,b),DwgEntityBody::BlockEnd(v)=>common(&v.common,b),DwgEntityBody::SequenceEnd(v)=>common(&v.common,b),
 DwgEntityBody::Insert(v)=>{b.add(3072)?;common(&v.common,b)?;for values in [&v.insertion,&v.scale,&v.extrusion]{reals(values,b)?;}handles(&v.attribute_handles,b)},
 DwgEntityBody::DimensionLinear(v)=>{b.add(4096)?;dimension(&v.dimension,b)?;for values in [&v.extension_line_1,&v.extension_line_2,&v.definition_point]{reals(values,b)?;}Ok(())},
 DwgEntityBody::Viewport(v)=>{b.add(24576)?;common(&v.common,b)?;text(&v.style_sheet,b)?;color(&v.ambient_color,b)?;for values in [&v.center,&v.view_target,&v.view_direction,&v.view_center,&v.snap_base,&v.snap_unit,&v.grid_unit,&v.ucs_origin,&v.ucs_x_axis,&v.ucs_y_axis]{reals(values,b)?;}handles(&v.frozen_layer_handles,b)?;for _ in &v.status{b.add(128)?;}Ok(())},
 DwgEntityBody::Point(v)=>{b.add(4096)?;common(&v.common,b)?;reals(&v.point,b)?;reals(&v.extrusion,b)},
 DwgEntityBody::Circle(v)=>{b.add(4096)?;common(&v.common,b)?;reals(&v.center,b)?;reals(&v.extrusion,b)},
 DwgEntityBody::Ellipse(v)=>{b.add(6144)?;common(&v.common,b)?;for values in [&v.center,&v.major_axis,&v.extrusion]{reals(values,b)?;}Ok(())},
 DwgEntityBody::Text(v)=>{b.add(10240)?;common(&v.common,b)?;reals(&v.insertion,b)?;reals(&v.extrusion,b)?;if let Some(values)=&v.alignment{reals(values,b)?;}text(&v.value,b)},
 DwgEntityBody::Spline(v)=>{b.add(4096)?;common(&v.common,b)?;for values in [&v.knots,&v.control_points,&v.weights]{reals(values,b)?;}Ok(())},
 DwgEntityBody::Face3d(v)=>{b.add(1024)?;common(&v.common,b)?;reals(&v.corners,b)},
 DwgEntityBody::Polyline3d(v)=>{b.add(1024)?;common(&v.common,b)?;handles(&v.vertex_handles,b)},DwgEntityBody::PolyfaceMesh(v)=>{b.add(1024)?;common(&v.common,b)?;handles(&v.vertex_handles,b)},
 DwgEntityBody::Vertex(v)=>{common(&v.common,b)?;reals(&v.point,b)},DwgEntityBody::PolyfaceFace(v)=>{common(&v.common,b)?;for _ in &v.indices{b.add(96)?;}Ok(())}
}}
fn control_body(value:&DwgTableControlBody,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(2048)?;for _ in value.entry_handles(){b.add(192)?;}if let DwgTableControlBody::DimensionStyle(value)=value{handles(&value.additional_handles,b)?;}Ok(())}
fn table_common(value:&DwgTableRecordCommon,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(512)?;text(&value.name,b)}
fn table_record(value:&DwgTableRecordBody,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(512)?;match value{
 DwgTableRecordBody::RegisteredApplication(v)=>table_common(&v.common,b),
 DwgTableRecordBody::TextStyle(v)=>{b.add(6144)?;table_common(&v.common,b)?;text(&v.font_file,b)?;text(&v.big_font_file,b)},
 DwgTableRecordBody::Layer(v)=>{b.add(2048)?;table_common(&v.common,b)?;color(&v.color,b)},
 DwgTableRecordBody::Linetype(v)=>{b.add(2048)?;table_common(&v.common,b)?;text(&v.description,b)?;for dash in &v.dashes{b.add(8192)?;optional_text(&dash.text,b)?;}Ok(())},
 DwgTableRecordBody::BlockHeader(v)=>{b.add(6144)?;table_common(&v.common,b)?;text(&v.xref_path,b)?;text(&v.description,b)?;handles(&v.owned_entity_handles,b)?;handles(&v.insert_backreference_handles,b)},
 DwgTableRecordBody::Viewport(v)=>{b.add(65536)?;table_common(&v.common,b)?;color(&v.ambient_color,b)},
 DwgTableRecordBody::DimensionStyle(v)=>{b.add(49152)?;table_common(&v.common,b)?;for value in [&v.dimension_postfix,&v.alternate_postfix,&v.r2010.alternate_measurement_suffix,&v.r2010.measurement_suffix]{text(value,b)?;}for value in [&v.fill_color,&v.text.dimension_line_color,&v.text.extension_line_color,&v.text.text_color]{color(value,b)?;}Ok(())}
}}
fn properties(values:&[DwgBlockParameterProperty],b:&mut Bound<'_,'_>)->Result<(),ValueError>{for property in values{b.add(128)?;for connection in &property.connections{b.add(256)?;text(&connection.name,b)?;}}Ok(())}
fn node_reference(value:&DwgNamedEvaluationNodeReference,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(256)?;text(&value.expression_name,b)}
fn element(value:&DwgBlockElement,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(256)?;expression(&value.evaluation_expression,b)?;text(&value.name,b)}
fn grip(value:&DwgBlockGrip,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(1024)?;element(&value.element,b)?;reals(&value.location,b)?;node_reference(&value.updated_x,b)?;node_reference(&value.updated_y,b)}
fn two_point(value:&DwgBlockTwoPointParameter,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(1024)?;element(&value.element,b)?;reals(&value.definition_base,b)?;reals(&value.definition_end,b)?;properties(&value.properties,b)?;for _ in &value.property_expression_references{b.add(256)?;}Ok(())}
fn block_action(value:&DwgBlockAction,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(1024)?;expression(&value.evaluation_expression,b)?;text(&value.name,b)?;reals(&value.display_location,b)?;for _ in &value.dependencies{b.add(192)?;}integers(&value.action_node_ids,b)}
fn connection(value:&DwgBlockActionConnection,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(256)?;text(&value.name,b)}
fn cell_border(value:&DwgCellBorder,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(3072)?;color(&value.color,b)}
fn cell_style(value:&DwgCellStyle,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(16384)?;color(&value.background_color,b)?;color(&value.content_format.content_color,b)?;text(&value.content_format.value_format_string,b)?;for border in [&value.borders.top,&value.borders.horizontal_inside,&value.borders.bottom,&value.borders.left,&value.borders.vertical_inside,&value.borders.right]{if let Some(border)=border{cell_border(border,b)?;}}Ok(())}
fn material_map(value:&DwgMaterialMap,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(2048)?;reals(&value.transform,b)}
fn constraint_core(value:&DwgConstraintNodeCore,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(256)?;integers(&value.connected_node_ids,b)}
fn geometric(value:&DwgGeometricConstraint,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(512)?;constraint_core(&value.node,b)}
fn constraint_geometry(value:&DwgConstraintGeometry,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(512)?;constraint_core(&value.node,b)}
fn constraint(value:&DwgConstraintNode,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(256)?;match value{
 DwgConstraintNode::ConstrainedImplicitPoint(v)=>{b.add(512)?;constraint_geometry(&v.geometry,b)?;if let Some(point)=&v.point{reals(point,b)?;}Ok(())},
 DwgConstraintNode::PointCurveConstraint(v)|DwgConstraintNode::PointCoincidenceConstraint(v)|DwgConstraintNode::PerpendicularConstraint(v)|DwgConstraintNode::ParallelConstraint(v)|DwgConstraintNode::MidPointConstraint(v)|DwgConstraintNode::EqualLengthConstraint(v)|DwgConstraintNode::ColinearConstraint(v)|DwgConstraintNode::FixedConstraint(v)=>geometric(v,b),
 DwgConstraintNode::ConstrainedBoundedLine(v)=>{b.add(256)?;constraint_geometry(&v.geometry,b)?;for values in [&v.origin,&v.direction,&v.start_point,&v.end_point]{reals(values,b)?;}Ok(())},
 DwgConstraintNode::DistanceConstraint(v)=>{b.add(1024)?;geometric(&v.explicit.geometric,b)?;if let Some(values)=&v.direction{reals(values,b)?;}Ok(())},
 DwgConstraintNode::HorizontalConstraint(v)|DwgConstraintNode::VerticalConstraint(v)=>{b.add(256)?;geometric(&v.geometric,b)},
 DwgConstraintNode::ConstrainedDatumLine(v)=>{constraint_geometry(&v.geometry,b)?;reals(&v.origin,b)?;reals(&v.direction,b)}
}}
fn body(value:&DwgLogicalObjectBody,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(512)?;match value{
 DwgLogicalObjectBody::Dictionary(v)=>{b.add(1024)?;for entry in &v.entries{b.add(256)?;text(&entry.name,b)?;}Ok(())},
 DwgLogicalObjectBody::TableControl(v)=>control_body(v,b),DwgLogicalObjectBody::TableRecord(v)=>table_record(v,b),DwgLogicalObjectBody::Entity(v)=>entity(v,b),
 DwgLogicalObjectBody::XRecord(v)=>{b.add(256)?;for value in &v.values{xrecord(value,b)?;}handles(&v.object_id_handles,b)},
 DwgLogicalObjectBody::AssociativeDependency(v)=>dependency(v,b),DwgLogicalObjectBody::AssociativeValueDependency(v)=>{b.add(256)?;dependency(&v.dependency,b)?;text(&v.value_name,b)},
 DwgLogicalObjectBody::AssociativeGeometryDependency(v)=>{b.add(256)?;dependency(&v.dependency,b)?;text(&v.persistent_subentity_class_name,b)},
 DwgLogicalObjectBody::BlockGripLocationComponent(v)=>{b.add(256)?;expression(&v.evaluation_expression,b)?;text(&v.grip_expression,b)},DwgLogicalObjectBody::DynamicBlockProxyNode(v)=>expression(&v.evaluation_expression,b),
 DwgLogicalObjectBody::AssociativeVariable(v)=>{b.add(1024)?;action(&v.action,b)?;for value in [&v.name,&v.expression,&v.evaluator_id,&v.description]{text(value,b)?;}optional_text(&v.mergeable_variable_name,b)?;handles(&v.referenced_value_dependency_handles,b)},
 DwgLogicalObjectBody::AssociativeDimensionDependencyBody(v)=>text(&v.name,b),DwgLogicalObjectBody::BlockParameterDependencyBody(v)=>text(&v.name,b),
 DwgLogicalObjectBody::VisualStyle(v)=>{b.add(16384)?;text(&v.description,b)?;let p=&v.properties;for value in [&p.face_monochrome_color.value,&p.edge_intersection_color.value,&p.edge_obscured_color.value,&p.edge_color.value,&p.edge_silhouette_color.value]{color(value,b)?;}Ok(())},
 DwgLogicalObjectBody::BlockRepresentationData(_)|DwgLogicalObjectBody::DynamicBlockPurgePreventer(_)|DwgLogicalObjectBody::Placeholder(_)=>b.add(512),
 DwgLogicalObjectBody::EvaluationGraph(v)=>{for _ in &v.nodes{b.add(256)?;}for _ in &v.edges{b.add(512)?;}Ok(())},
 DwgLogicalObjectBody::BlockFlipParameter(v)=>{b.add(1024)?;expression(&v.evaluation_expression,b)?;for value in [&v.name,&v.label,&v.description,&v.value_set.base_label,&v.value_set.flipped_label]{text(value,b)?;}for values in [&v.definition_base,&v.definition_end,&v.label_point]{reals(values,b)?;}properties(&v.properties,b)?;node_reference(&v.updated_flip,b)},
 DwgLogicalObjectBody::BlockVisibilityParameter(v)=>{b.add(1024)?;expression(&v.evaluation_expression,b)?;for value in [&v.element_name,&v.name,&v.description]{text(value,b)?;}reals(&v.definition_point,b)?;properties(&v.properties,b)?;handles(&v.eligible_entity_handles,b)?;for state in &v.states{b.add(256)?;text(&state.name,b)?;handles(&state.visible_entity_handles,b)?;handles(&state.controlled_expression_handles,b)?;}Ok(())},
 DwgLogicalObjectBody::DictionaryVariable(v)=>text(&v.value,b),DwgLogicalObjectBody::AnnotationScale(v)=>{b.add(3072)?;text(&v.name,b)},
 DwgLogicalObjectBody::SortEntitiesTable(v)=>{b.add(256)?;for _ in &v.entries{b.add(256)?;}Ok(())},
 DwgLogicalObjectBody::TableStyle(v)=>{b.add(1024)?;text(&v.description,b)?;for style in [&v.table,&v.title,&v.header,&v.data]{cell_style(style,b)?;}Ok(())},
 DwgLogicalObjectBody::MlineStyle(v)=>{b.add(4096)?;text(&v.name,b)?;text(&v.description,b)?;color(&v.fill_color,b)?;for value in &v.elements{b.add(2048)?;color(&value.color,b)?;}Ok(())},
 DwgLogicalObjectBody::MLeaderStyle(v)=>{b.add(20480)?;text(&v.description,b)?;text(&v.text.default_content,b)?;for value in [&v.leader.color,&v.text.color,&v.block.color]{color(value,b)?;}reals(&v.block.scale,b)},
 DwgLogicalObjectBody::Material(v)=>{b.add(16384)?;text(&v.name,b)?;text(&v.description,b)?;for map in [&v.diffuse_map,&v.specular_map,&v.reflection_map,&v.opacity_map,&v.bump_map,&v.refraction_map]{material_map(map,b)?;}Ok(())},
 DwgLogicalObjectBody::BlockMoveAction(v)=>{b.add(4096)?;block_action(&v.action,b)?;connection(&v.x_connection,b)?;connection(&v.y_connection,b)},
 DwgLogicalObjectBody::AssocNetwork(v)=>{b.add(256)?;action(&v.action,b)?;for _ in &v.actions{b.add(256)?;}Ok(())},
 DwgLogicalObjectBody::Assoc2dConstraintGroup(v)=>{b.add(512)?;action(&v.action,b)?;for values in &v.work_plane{reals(values,b)?;}handles(&v.member_action_handles,b)?;for node in &v.nodes{constraint(node,b)?;}Ok(())},
 DwgLogicalObjectBody::BlockLinearParameter(v)=>{b.add(2048)?;two_point(&v.parameter,b)?;text(&v.distance_name,b)?;text(&v.distance_description,b)?;reals(&v.allowed_values,b)},
 DwgLogicalObjectBody::BlockLinearGrip(v)=>{b.add(256)?;grip(&v.grip,b)?;reals(&v.orientation,b)},DwgLogicalObjectBody::BlockFlipGrip(v)=>{b.add(256)?;grip(&v.grip,b)?;node_reference(&v.updated_flip,b)?;reals(&v.orientation,b)},DwgLogicalObjectBody::BlockVisibilityGrip(v)=>grip(&v.grip,b),
 DwgLogicalObjectBody::BlockAlignmentParameter(v)=>{b.add(256)?;two_point(&v.parameter,b)},DwgLogicalObjectBody::BlockAlignmentGrip(v)=>{b.add(512)?;grip(&v.grip,b)?;reals(&v.orientation,b)},
 DwgLogicalObjectBody::BlockStretchAction(v)=>{b.add(4096)?;block_action(&v.action,b)?;connection(&v.x_connection,b)?;connection(&v.y_connection,b)?;for values in &v.points{reals(values,b)?;}for selection in &v.selections{b.add(256)?;integers(&selection.vertex_indices,b)?;}for selector in &v.selectors{b.add(256)?;integers(&selector.point_indices,b)?;}Ok(())},
 DwgLogicalObjectBody::BlockScaleAction(v)=>{b.add(4096)?;block_action(&v.base.action,b)?;reals(&v.base.offset,b)?;reals(&v.base.base_point,b)?;for value in [&v.base.x_base_connection,&v.base.y_base_connection,&v.uniform_scale_connection,&v.x_scale_connection,&v.y_scale_connection]{connection(value,b)?;}Ok(())},
 DwgLogicalObjectBody::BlockFlipAction(v)=>{b.add(256)?;block_action(&v.action,b)?;for value in [&v.flip_connection,&v.updated_flip_connection,&v.updated_base_connection,&v.updated_end_connection]{connection(value,b)?;}Ok(())},
 DwgLogicalObjectBody::BlockBasePointParameter(v)=>{b.add(512)?;element(&v.parameter.element,b)?;reals(&v.parameter.definition_point,b)?;properties(&v.parameter.properties,b)?;reals(&v.point,b)?;reals(&v.base_point,b)},
 DwgLogicalObjectBody::BlockVerticalConstraintParameter(v)|DwgLogicalObjectBody::BlockHorizontalConstraintParameter(v)=>{b.add(2048)?;two_point(&v.parameter,b)?;text(&v.expression_name,b)?;text(&v.expression_description,b)?;reals(&v.allowed_values.values,b)},
 DwgLogicalObjectBody::Layout(v)=>{b.add(12288)?;for value in [&v.page_setup_name,&v.printer_configuration,&v.canonical_media_name,&v.stylesheet,&v.name]{text(value,b)?;}for values in [&v.margins,&v.paper_size,&v.plot_origin,&v.plot_window_lower_left,&v.plot_window_upper_right,&v.paper_image_origin,&v.insertion_base,&v.limits_minimum,&v.limits_maximum,&v.ucs_origin,&v.ucs_x_axis,&v.ucs_y_axis,&v.extents_minimum,&v.extents_maximum]{reals(values,b)?;}handles(&v.viewport_handles,b)}
}}
fn space(value:&DwgHeaderSpaceGeometry,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.add(2048)?;for values in [&value.insertion_base,&value.extents_minimum,&value.extents_maximum,&value.limits_minimum,&value.limits_maximum,&value.ucs_origin,&value.ucs_x_axis,&value.ucs_y_axis,&value.ucs_origin_top,&value.ucs_origin_bottom,&value.ucs_origin_left,&value.ucs_origin_right,&value.ucs_origin_front,&value.ucs_origin_back]{reals(values,b)?;}Ok(())}
pub(super) fn preflight(value:&DwgSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let mut b=Bound::new(control)?;b.add(1024)?;text(&value.schema,&mut b)?;text(&value.version,&mut b)?;
 for layer in &value.drawing.layers{b.add(256)?;text(&layer.name,&mut b)?;}reals(&value.drawing.extmin,&mut b)?;reals(&value.drawing.extmax,&mut b)?;
 for object in &value.drawing.objects{b.add(2048)?;text(&object.class_name,&mut b)?;handles(&object.reactor_handles,&mut b)?;handles(&object.referenced_handles,&mut b)?;for data in &object.extended_data{b.add(256)?;for field in &data.values{xrecord(field,&mut b)?;}}if let Some(value)=&object.body{body(value,&mut b)?;}}
 b.add(131072)?;let h=&value.header;for text_value in [&h.units.unit1_name,&h.units.unit2_name,&h.units.unit3_name,&h.units.unit4_name,&h.strings.menu,&h.strings.dimension_postfix,&h.strings.dimension_alternate_postfix,&h.strings.dimension_alternate_measurement_zero_suffix,&h.strings.dimension_measurement_zero_suffix,&h.strings.hyperlink_base,&h.strings.stylesheet,&h.strings.fingerprint_guid,&h.strings.version_guid,&h.strings.project_name]{text(text_value,&mut b)?;}space(&h.paper_space,&mut b)?;space(&h.model_space,&mut b)?;
 for class in &value.classes{b.add(2048)?;for value in [&class.application_name,&class.cpp_class_name,&class.dxf_name]{text(value,&mut b)?;}integers(&class.reserved_values,&mut b)?;}
 for dependency in &value.dependencies{b.add(1024)?;for value in [&dependency.feature,&dependency.full_path,&dependency.relative_path,&dependency.fingerprint,&dependency.version]{text(value,&mut b)?;}}
 b.add(4096)?;let s=&value.summary;for value in [&s.title,&s.subject,&s.author,&s.keywords,&s.comments,&s.last_saved_by,&s.revision_number,&s.hyperlink_base]{text(value,&mut b)?;}for property in &s.custom_properties{b.add(128)?;text(&property.key,&mut b)?;text(&property.value,&mut b)?;}
 b.add(4096)?;let a=&value.application;for value in [&a.name,&a.version_checksum,&a.version,&a.comment_checksum,&a.comment,&a.product_checksum,&a.product,&a.application_version]{text(value,&mut b)?;}text(&value.template.description,&mut b)?;
 b.add(4096)?;integers(&value.revision_history.revisions,&mut b)?;for _ in &value.preview.palette{b.add(256)?;}b.repeated(value.preview.pixel_indices.len(),8)?;
 b.add(4096)?;let a=&value.application_history;for value in [&a.history_identifier_one,&a.history_identifier_two,&a.application_version_digest,&a.application_version,&a.trust_comment_digest,&a.trust_comment,&a.property_set_digest,&a.property_format_identifier,&a.product_digest,&a.product.name,&a.product.build_version,&a.product.registry_version,&a.product.install_id,&a.product.locale_id]{text(value,&mut b)?;}for property in &a.properties{b.add(256)?;text(&property.value,&mut b)?;}b.finish()
}
