//! ✏️ The logical drawing: its object graph and the standard handles of a new document.

use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgArcEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgBlockBeginEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgBlockEndEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgCircleEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEllipseEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColor;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColorKind;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgFace3dEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLwPolylineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLwPolylineVertex;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgOrthographicView;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPointEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPolyfaceFaceEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPolyfaceMeshEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPolyline3dEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgSequenceEndEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgSplineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgTextEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgVertexEntity;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::{DwgLogicalEntity, DwgLogicalGeometry, DwgLogicalLayer, vec3};
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLayout;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLayoutOptions;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlaceholder;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotArea;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotOptions;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotPaperUnit;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotRotation;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlot;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlotResolution;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgStandardScale;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineCaps;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineLinetype;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineStyleElement;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgBlockHeaderTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgBlockTableControl;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColorValue;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionBehavior;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionGeometry;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionR2010;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionStyleTableControl;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionStyleTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionText;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionUnits;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLayerTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLinetypeTableControl;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLinetypeTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgRegisteredApplicationTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlEntries;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlEntry;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordCommon;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTextStyleTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgViewportTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgDictionaryBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgNamedReference;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgObjectCategory;

#[path = "🧮️geometry/🦀️.rs"]
pub mod geometry;

//#region 🔖️DrawingModel
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalDrawing {
    /// 🧭 Handle-keyed objects are the sole persisted entity authority; use `entities()` for a derived view.
    #[value(default)]
    pub layers: Vec<DwgLogicalLayer>,
    #[value(default)]
    pub objects: Vec<DwgLogicalObject>,
    #[value(default)]
    pub extmin: Vec<f64>,
    #[value(default)]
    pub extmax: Vec<f64>,
}
impl DwgLogicalDrawing {
    /// 🏗️ The complete AC1024 object graph of a new document carrying `drawing`: the nine symbol-table
    /// controls with their standard records (layer `0` plus the drawing's layers, the ByBlock/ByLayer/
    /// Continuous linetypes, the Standard text/dimension styles, the ACAD application, the `*Active`
    /// viewport), the model- and paper-space block records with their BLOCK/ENDBLK pairs and layouts,
    /// the named-object dictionary with the dictionaries AutoCAD requires, and every entity owned by
    /// model space. Handles follow [`new_document_handles`]; entities start at its `FIRST_FREE`.
    /// See <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> §20.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_native(drawing: &semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing) -> Result<Self, String> {
        use new_document_handles as h;
        let mut layers = drawing.layers.clone();
        if !layers.iter().any(|layer| layer.name == "0") {
            layers.push(semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer::default());
        }
        let mut next = h::FIRST_FREE;
        let mut allocate = || {
            let handle = next;
            next += 1;
            handle
        };
        let layer_handles: Vec<u64> = layers.iter().map(|_| allocate()).collect();
        let layer_zero = layers.iter().zip(&layer_handles).find_map(|(layer, handle)| (layer.name == "0").then_some(*handle)).ok_or("layer 0 is missing")?;
        let common = |layer: usize, color: semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor, mode: DwgEntityMode| DwgEntityCommon {
            mode,
            color: match color {
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor::ByLayer => DwgEntityColor { kind: DwgEntityColorKind::ByLayer, index: 256, ..Default::default() },
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor::ByBlock => DwgEntityColor { kind: DwgEntityColorKind::ByBlock, index: 0, ..Default::default() },
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor::Index(index) => DwgEntityColor { kind: DwgEntityColorKind::Index, index: u16::from(index), ..Default::default() },
            },
            linetype_scale: 1.0,
            lineweight: 29,
            layer_handle: layer_handles[layer],
            ..Default::default()
        };
        let entity = |handle: u64, type_code: u16, class_name: &str, owner: Option<u64>, body: DwgEntityBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::Entity,
            owner_handle: owner,
            body: Some(DwgLogicalObjectBody::Entity(body)),
            ..Default::default()
        };
        let mut entity_objects = Vec::new();
        let mut model_space_entities = Vec::new();
        for (index, source) in drawing.entities.iter().enumerate() {
            let layer = Some(source.layer).filter(|layer| *layer < drawing.layers.len()).ok_or_else(|| format!("DWG entity {index} references missing layer index {}", source.layer))?;
            let top = common(layer, source.color, DwgEntityMode::ModelSpace);
            let owned = common(layer, source.color, DwgEntityMode::ExplicitOwner);
            let handle = allocate();
            model_space_entities.push(handle);
            let z = [0.0, 0.0, 1.0].to_vec();
            match &source.geometry {
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Line { start, end } => entity_objects.push(entity(handle, 19, "LINE", None, DwgEntityBody::Line(DwgLineEntity { common: top, start: start.to_vec(), end: end.to_vec(), thickness: 0.0, extrusion: z }))),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Point { at } => entity_objects.push(entity(handle, 27, "POINT", None, DwgEntityBody::Point(DwgPointEntity { common: top, point: at.to_vec(), thickness: 0.0, extrusion: z, x_axis_angle: 0.0 }))),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Circle { center, radius, normal } => entity_objects.push(entity(handle, 18, "CIRCLE", None, DwgEntityBody::Circle(DwgCircleEntity { common: top, center: center.to_vec(), radius: *radius, thickness: 0.0, extrusion: normal.to_vec() }))),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Arc { center, radius, start_angle, end_angle, normal } => entity_objects.push(entity(
                    handle,
                    17,
                    "ARC",
                    None,
                    DwgEntityBody::Arc(DwgArcEntity { common: top, center: center.to_vec(), radius: *radius, thickness: 0.0, extrusion: normal.to_vec(), start_angle: *start_angle, end_angle: *end_angle }),
                )),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal } => entity_objects.push(entity(
                    handle,
                    35,
                    "ELLIPSE",
                    None,
                    DwgEntityBody::Ellipse(DwgEllipseEntity { common: top, center: center.to_vec(), major_axis: major_axis.to_vec(), extrusion: normal.to_vec(), axis_ratio: *ratio, start_parameter: *start_param, end_parameter: *end_param }),
                )),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::LwPolyline { closed, elevation, vertices, bulges } => entity_objects.push(entity(
                    handle,
                    77,
                    "LWPOLYLINE",
                    None,
                    DwgEntityBody::LwPolyline(DwgLwPolylineEntity {
                        common: top,
                        closed: *closed,
                        elevation: *elevation,
                        thickness: 0.0,
                        extrusion: z,
                        vertices: vertices.iter().enumerate().map(|(vertex, point)| DwgLwPolylineVertex { point: point.to_vec(), bulge: bulges.get(vertex).copied().unwrap_or_default(), ..Default::default() }).collect(),
                        ..Default::default()
                    }),
                )),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Spline { degree, control_points, knots, weights } => entity_objects.push(entity(
                    handle,
                    36,
                    "SPLINE",
                    None,
                    DwgEntityBody::Spline(DwgSplineEntity {
                        common: top,
                        degree: *degree,
                        rational: !weights.is_empty(),
                        closed: false,
                        periodic: false,
                        knot_tolerance: 1e-10,
                        control_tolerance: 1e-10,
                        knots: knots.clone(),
                        control_points: control_points.iter().flatten().copied().collect(),
                        weights: weights.clone(),
                    }),
                )),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Text { at, height, rotation, content } => entity_objects.push(entity(
                    handle,
                    1,
                    "TEXT",
                    None,
                    DwgEntityBody::Text(DwgTextEntity {
                        common: top,
                        elevation: at[2],
                        insertion: vec![at[0], at[1]],
                        alignment: None,
                        extrusion: z,
                        thickness: 0.0,
                        oblique_angle: 0.0,
                        rotation: *rotation,
                        height: *height,
                        width_factor: 1.0,
                        value: content.clone(),
                        generation: 0,
                        horizontal_alignment: 0,
                        vertical_alignment: 0,
                        style_handle: h::STANDARD_TEXT_STYLE,
                    }),
                )),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Face3d { corners } => entity_objects.push(entity(handle, 28, "3DFACE", None, DwgEntityBody::Face3d(DwgFace3dEntity { common: top, corners: corners.iter().flatten().copied().collect(), invisible_edges: 0 }))),
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Polyline3d { closed, vertices } => {
                    let vertex_handles: Vec<u64> = vertices.iter().map(|_| allocate()).collect();
                    let sequence_end_handle = allocate();
                    entity_objects.push(entity(handle, 16, "POLYLINE_3D", None, DwgEntityBody::Polyline3d(DwgPolyline3dEntity { common: top, curve_type: 0, flags: u8::from(*closed), vertex_handles: vertex_handles.clone(), sequence_end_handle })));
                    for (vertex, point) in vertex_handles.iter().zip(vertices) {
                        entity_objects.push(entity(*vertex, 11, "VERTEX_3D", Some(handle), DwgEntityBody::Vertex(DwgVertexEntity { common: owned.clone(), flags: 32, point: point.to_vec() })));
                    }
                    entity_objects.push(entity(sequence_end_handle, 6, "SEQEND", Some(handle), DwgEntityBody::SequenceEnd(DwgSequenceEndEntity { common: owned })));
                }
                semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::PolyfaceMesh { vertices, faces } => {
                    let vertex_count = u16::try_from(vertices.len()).map_err(|_| format!("DWG polyface mesh {index} exceeds {} vertices", u16::MAX))?;
                    let face_count = u16::try_from(faces.len()).map_err(|_| format!("DWG polyface mesh {index} exceeds {} faces", u16::MAX))?;
                    let vertex_handles: Vec<u64> = vertices.iter().chain(faces.iter().map(|_| &[0.0; 3])).map(|_| allocate()).collect();
                    let sequence_end_handle = allocate();
                    entity_objects.push(entity(handle, 29, "POLYLINE_PFACE", None, DwgEntityBody::PolyfaceMesh(DwgPolyfaceMeshEntity { common: top, vertex_count, face_count, vertex_handles: vertex_handles.clone(), sequence_end_handle })));
                    for (vertex, point) in vertex_handles.iter().zip(vertices) {
                        entity_objects.push(entity(*vertex, 13, "VERTEX_PFACE", Some(handle), DwgEntityBody::Vertex(DwgVertexEntity { common: owned.clone(), flags: 192, point: point.to_vec() })));
                    }
                    for (face_handle, face) in vertex_handles[vertices.len()..].iter().zip(faces) {
                        let indices = face.iter().map(|value| i16::try_from(*value).map_err(|_| format!("DWG polyface mesh {index} face index {value} exceeds i16"))).collect::<Result<Vec<_>, _>>()?;
                        entity_objects.push(entity(*face_handle, 14, "VERTEX_PFACE_FACE", Some(handle), DwgEntityBody::PolyfaceFace(DwgPolyfaceFaceEntity { common: owned.clone(), indices })));
                    }
                    entity_objects.push(entity(sequence_end_handle, 6, "SEQEND", Some(handle), DwgEntityBody::SequenceEnd(DwgSequenceEndEntity { common: owned })));
                }
            }
        }
        let record = |handle: u64, type_code: u16, class_name: &str, owner: u64, body: DwgTableRecordBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::TableRecord,
            owner_handle: Some(owner),
            body: Some(DwgLogicalObjectBody::TableRecord(body)),
            ..Default::default()
        };
        let control = |handle: u64, type_code: u16, class_name: &str, body: DwgTableControlBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::TableControl,
            body: Some(DwgLogicalObjectBody::TableControl(body)),
            ..Default::default()
        };
        let entries = |handles: &[u64]| DwgTableControlEntries { entry_handles: handles.iter().map(|handle| DwgTableControlEntry { handle: Some(*handle) }).collect() };
        let dictionary = |handle: u64, owner: Option<u64>, entries: &[(&str, u64)]| DwgLogicalObject {
            handle,
            type_code: 42,
            class_name: "DICTIONARY".into(),
            category: DwgObjectCategory::Dictionary,
            owner_handle: owner,
            reactor_handles: owner.into_iter().collect(),
            body: Some(DwgLogicalObjectBody::Dictionary(DwgDictionaryBody { entries: entries.iter().map(|(name, handle)| DwgNamedReference { name: (*name).into(), handle: *handle }).collect(), cloning_flag: 1, hard_owner: false, default_entry_handle: None })),
            ..Default::default()
        };
        let owned_object = |handle: u64, type_code: u16, class_name: &str, owner: u64, extension_dictionary: Option<u64>, body: DwgLogicalObjectBody| DwgLogicalObject {
            handle,
            type_code,
            class_name: class_name.into(),
            category: DwgObjectCategory::Object,
            owner_handle: Some(owner),
            reactor_handles: vec![owner],
            extension_dictionary_handle: extension_dictionary,
            body: Some(body),
            ..Default::default()
        };
        let block_common = |mode: DwgEntityMode| DwgEntityCommon { mode, linetype_scale: 1.0, lineweight: 29, layer_handle: layer_zero, color: DwgEntityColor { kind: DwgEntityColorKind::ByLayer, index: 256, ..Default::default() }, ..Default::default() };
        let block_header = |name: &str, owned_entity_handles: Vec<u64>, block_entity_handle: u64, end_block_entity_handle: u64, layout: u64| DwgBlockHeaderTableRecord {
            common: DwgTableRecordCommon { name: name.into(), ..Default::default() },
            owned_entity_handles,
            explodable: true,
            block_entity_handle,
            end_block_entity_handle,
            layout_handle: Some(layout),
            ..Default::default()
        };
        let layout = |name: &str, model: bool, block_header_handle: u64, active_viewport_handle: Option<u64>| DwgLayout {
            page_setup_name: String::new(),
            printer_configuration: "none_device".into(),
            canonical_media_name: "ANSI_A_(8.50_x_11.00_Inches)".into(),
            stylesheet: String::new(),
            name: name.into(),
            plot_options: DwgPlotOptions { use_standard_scale: true, plot_viewport_borders: true, plot_with_lineweights: true, draw_viewports_first: true, model_type: model, update_paper: false, initializing: false },
            margins: vec![6.35, 6.35, 6.35, 6.35],
            paper_size: vec![215.9, 279.4],
            plot_origin: vec![0.0, 0.0],
            paper_unit: DwgPlotPaperUnit::Inches,
            rotation: DwgPlotRotation::QuarterTurn,
            plot_area: if model { DwgPlotArea::Display } else { DwgPlotArea::Layout },
            plot_window_lower_left: vec![0.0, 0.0],
            plot_window_upper_right: vec![0.0, 0.0],
            paper_units: 1.0,
            drawing_units: 1.0,
            standard_scale: DwgStandardScale::OneToOne,
            standard_scale_factor: 1.0,
            paper_image_origin: vec![0.0, 0.0],
            shade_plot: DwgShadePlot::AsDisplayed,
            shade_plot_resolution: DwgShadePlotResolution::Normal,
            shade_plot_dpi: 300,
            tab_order: u16::from(!model),
            options: DwgLayoutOptions { paper_space_linetype_scaling: true },
            insertion_base: vec![0.0; 3],
            limits_minimum: vec![0.0, 0.0],
            limits_maximum: vec![12.0, 9.0],
            ucs_origin: vec![0.0; 3],
            ucs_x_axis: vec![1.0, 0.0, 0.0],
            ucs_y_axis: vec![0.0, 1.0, 0.0],
            ucs_elevation: 0.0,
            orthographic_view: DwgOrthographicView::None,
            extents_minimum: vec![0.0; 3],
            extents_maximum: vec![0.0; 3],
            plot_view_handle: None,
            visual_style_handle: None,
            block_header_handle,
            active_viewport_handle,
            base_ucs_handle: None,
            named_ucs_handle: None,
            viewport_handles: Vec::new(),
        };
        let by_block = DwgComplexColor { index: 0, value: DwgComplexColorValue::ByBlock, ..Default::default() };
        let linetype = |name: &str, description: &str| DwgLinetypeTableRecord { common: DwgTableRecordCommon { name: name.into(), ..Default::default() }, description: description.into(), pattern_length: 0.0, alignment: b'A', dashes: Vec::new() };
        let extent_width = (drawing.extmax[0] - drawing.extmin[0]).abs().max(1.0);
        let extent_height = (drawing.extmax[1] - drawing.extmin[1]).abs().max(1.0);
        let mut objects = vec![
            control(h::BLOCK_CONTROL, 48, "BLOCK_CONTROL", DwgTableControlBody::Block(DwgBlockTableControl { entry_handles: Vec::new(), model_space_handle: Some(h::MODEL_SPACE), paper_space_handle: Some(h::PAPER_SPACE) })),
            control(h::LAYER_CONTROL, 50, "LAYER_CONTROL", DwgTableControlBody::Layer(entries(&layer_handles))),
            control(h::STYLE_CONTROL, 52, "STYLE_CONTROL", DwgTableControlBody::TextStyle(entries(&[h::STANDARD_TEXT_STYLE]))),
            control(h::LINETYPE_CONTROL, 56, "LTYPE_CONTROL", DwgTableControlBody::Linetype(DwgLinetypeTableControl { entry_handles: entries(&[h::CONTINUOUS]).entry_handles, by_block_handle: h::BY_BLOCK, by_layer_handle: h::BY_LAYER })),
            control(h::VIEW_CONTROL, 60, "VIEW_CONTROL", DwgTableControlBody::View(entries(&[]))),
            control(h::UCS_CONTROL, 62, "UCS_CONTROL", DwgTableControlBody::Ucs(entries(&[]))),
            control(h::VIEWPORT_CONTROL, 64, "VPORT_CONTROL", DwgTableControlBody::Viewport(entries(&[h::ACTIVE_VIEWPORT]))),
            control(h::APPID_CONTROL, 66, "APPID_CONTROL", DwgTableControlBody::RegisteredApplication(entries(&[h::ACAD_APPLICATION]))),
            control(h::DIMSTYLE_CONTROL, 68, "DIMSTYLE_CONTROL", DwgTableControlBody::DimensionStyle(DwgDimensionStyleTableControl { entry_handles: entries(&[h::STANDARD_DIMSTYLE]).entry_handles, additional_handles: Vec::new() })),
            dictionary(
                h::NAMED_OBJECTS,
                None,
                &[
                    ("ACAD_COLOR", h::COLOR_DICTIONARY),
                    ("ACAD_GROUP", h::GROUP_DICTIONARY),
                    ("ACAD_LAYOUT", h::LAYOUT_DICTIONARY),
                    ("ACAD_MATERIAL", h::MATERIAL_DICTIONARY),
                    ("ACAD_MLINESTYLE", h::MLINE_STYLE_DICTIONARY),
                    ("ACAD_PLOTSETTINGS", h::PLOT_SETTINGS_DICTIONARY),
                    ("ACAD_PLOTSTYLENAME", h::PLOT_STYLE_NAME_DICTIONARY),
                    ("ACAD_VISUALSTYLE", h::VISUAL_STYLE_DICTIONARY),
                ],
            ),
            dictionary(h::GROUP_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            DwgLogicalObject {
                handle: h::PLOT_STYLE_NAME_DICTIONARY,
                type_code: 500,
                class_name: "ACDBDICTIONARYWDFLT".into(),
                category: DwgObjectCategory::Custom,
                owner_handle: Some(h::NAMED_OBJECTS),
                reactor_handles: vec![h::NAMED_OBJECTS],
                body: Some(DwgLogicalObjectBody::Dictionary(DwgDictionaryBody {
                    entries: vec![DwgNamedReference { name: "Normal".into(), handle: h::NORMAL_PLOT_STYLE }],
                    cloning_flag: 1,
                    hard_owner: false,
                    default_entry_handle: Some(h::NORMAL_PLOT_STYLE),
                })),
                ..Default::default()
            },
            owned_object(h::NORMAL_PLOT_STYLE, 80, "ACDBPLACEHOLDER", h::PLOT_STYLE_NAME_DICTIONARY, None, DwgLogicalObjectBody::Placeholder(DwgPlaceholder {})),
            record(h::STANDARD_TEXT_STYLE, 53, "STYLE", h::STYLE_CONTROL, DwgTableRecordBody::TextStyle(DwgTextStyleTableRecord { common: DwgTableRecordCommon { name: "Standard".into(), ..Default::default() }, width_factor: 1.0, last_height: 0.2, font_file: "txt".into(), ..Default::default() })),
            record(h::ACAD_APPLICATION, 67, "APPID", h::APPID_CONTROL, DwgTableRecordBody::RegisteredApplication(DwgRegisteredApplicationTableRecord { common: DwgTableRecordCommon { name: "ACAD".into(), ..Default::default() }, group_71: 0 })),
            record(h::BY_BLOCK, 57, "LTYPE", h::LINETYPE_CONTROL, DwgTableRecordBody::Linetype(linetype("ByBlock", ""))),
            record(h::BY_LAYER, 57, "LTYPE", h::LINETYPE_CONTROL, DwgTableRecordBody::Linetype(linetype("ByLayer", ""))),
            record(h::CONTINUOUS, 57, "LTYPE", h::LINETYPE_CONTROL, DwgTableRecordBody::Linetype(linetype("Continuous", "Solid line"))),
            dictionary(h::MLINE_STYLE_DICTIONARY, Some(h::NAMED_OBJECTS), &[("Standard", h::STANDARD_MLINE_STYLE)]),
            owned_object(
                h::STANDARD_MLINE_STYLE,
                73,
                "MLINESTYLE",
                h::MLINE_STYLE_DICTIONARY,
                None,
                DwgLogicalObjectBody::MlineStyle(DwgMlineStyle {
                    name: "Standard".into(),
                    description: String::new(),
                    fill_enabled: false,
                    display_miters: false,
                    start_caps: DwgMlineCaps::default(),
                    end_caps: DwgMlineCaps::default(),
                    fill_color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByLayer, ..Default::default() },
                    start_angle: std::f64::consts::FRAC_PI_2,
                    end_angle: std::f64::consts::FRAC_PI_2,
                    elements: [0.5, -0.5].into_iter().map(|offset| DwgMlineStyleElement { offset, color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByLayer, ..Default::default() }, linetype: DwgMlineLinetype::ByLayer }).collect(),
                }),
            ),
            dictionary(h::PLOT_SETTINGS_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            dictionary(h::LAYOUT_DICTIONARY, Some(h::NAMED_OBJECTS), &[("Layout1", h::PAPER_LAYOUT), ("Model", h::MODEL_LAYOUT)]),
            record(
                h::STANDARD_DIMSTYLE,
                69,
                "DIMSTYLE",
                h::DIMSTYLE_CONTROL,
                DwgTableRecordBody::DimensionStyle(DwgDimensionStyleTableRecord {
                    common: DwgTableRecordCommon { name: "Standard".into(), ..Default::default() },
                    dimension_postfix: String::new(),
                    alternate_postfix: String::new(),
                    geometry: DwgDimensionGeometry { scale: 1.0, arrow_size: 0.18, extension_origin_offset: 0.0625, dimension_line_increment: 0.38, extension_line_extension: 0.18, rounding: 0.0, dimension_line_extension: 0.0, plus_tolerance: 0.0, minus_tolerance: 0.0, fixed_extension_length: 1.0, jog_angle: std::f64::consts::FRAC_PI_4 },
                    fill_mode: 0,
                    fill_color: by_block.clone(),
                    behavior: DwgDimensionBehavior { text_inside_horizontal: true, text_outside_horizontal: true, ..Default::default() },
                    text: DwgDimensionText {
                        height: 0.18,
                        center_mark_size: 0.09,
                        tick_size: 0.0,
                        alternate_scale: 25.4,
                        linear_scale: 1.0,
                        vertical_position: 0.0,
                        tolerance_scale: 1.0,
                        gap: 0.09,
                        alternate_rounding: 0.0,
                        alternate_enabled: false,
                        alternate_decimals: 2,
                        text_outside_extensions: false,
                        separate_arrowheads: false,
                        force_text_inside: false,
                        suppress_outside_extensions: false,
                        dimension_line_color: by_block.clone(),
                        extension_line_color: by_block.clone(),
                        text_color: by_block.clone(),
                    },
                    units: DwgDimensionUnits { decimal_places: 4, tolerance_decimal_places: 4, alternate_units: 2, alternate_tolerance_decimal_places: 2, linear_units: 2, decimal_separator: 46, tolerance_vertical_alignment: 1, arrow_text_fit: 3, ..Default::default() },
                    r2010: DwgDimensionR2010 { alternate_measurement_factor: 100.0, measurement_factor: 100.0, dimension_lineweight: 0xfffe, extension_lineweight: 0xfffe, ..Default::default() },
                    text_style_handle: Some(h::STANDARD_TEXT_STYLE),
                    ..Default::default()
                }),
            ),
            record(
                h::ACTIVE_VIEWPORT,
                65,
                "VPORT",
                h::VIEWPORT_CONTROL,
                DwgTableRecordBody::Viewport(DwgViewportTableRecord {
                    common: DwgTableRecordCommon { name: "*Active".into(), ..Default::default() },
                    view_height: extent_height,
                    view_width: extent_width,
                    center: [(drawing.extmin[0] + drawing.extmax[0]) / 2.0, (drawing.extmin[1] + drawing.extmax[1]) / 2.0],
                    target: [0.0; 3],
                    direction: [0.0, 0.0, 1.0],
                    twist: 0.0,
                    lens_length: 50.0,
                    front_clipping: 0.0,
                    back_clipping: 0.0,
                    view_mode: [false, false, false, true],
                    render_mode: 0,
                    use_default_lights: true,
                    default_lighting_type: 1,
                    brightness: 0.0,
                    contrast: 0.0,
                    ambient_color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByColor { red: 51, green: 51, blue: 51 }, ..Default::default() },
                    lower_left: [0.0, 0.0],
                    upper_right: [1.0, 1.0],
                    ucs_follow: false,
                    circle_zoom: 1000,
                    fast_zoom: true,
                    ucs_icon: 3,
                    grid_mode: false,
                    grid_unit: [0.5, 0.5],
                    snap_mode: false,
                    snap_style: false,
                    snap_isopair: 0,
                    snap_angle: 0.0,
                    snap_base: [0.0, 0.0],
                    snap_unit: [0.5, 0.5],
                    ucs_at_origin: false,
                    ucs_viewport: true,
                    ucs_origin: [0.0; 3],
                    ucs_x_axis: [1.0, 0.0, 0.0],
                    ucs_y_axis: [0.0, 1.0, 0.0],
                    ucs_elevation: 0.0,
                    ucs_orthographic_view: 0,
                    grid_flags: 2,
                    grid_major: 5,
                    ..Default::default()
                }),
            ),
            dictionary(h::MATERIAL_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            dictionary(h::COLOR_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            dictionary(h::VISUAL_STYLE_DICTIONARY, Some(h::NAMED_OBJECTS), &[]),
            record(h::MODEL_SPACE, 49, "BLOCK_HEADER", h::BLOCK_CONTROL, DwgTableRecordBody::BlockHeader(block_header("*Model_Space", model_space_entities, h::MODEL_SPACE_BLOCK, h::MODEL_SPACE_END, h::MODEL_LAYOUT))),
            entity(h::MODEL_SPACE_BLOCK, 4, "BLOCK", None, DwgEntityBody::BlockBegin(DwgBlockBeginEntity { common: block_common(DwgEntityMode::ModelSpace) })),
            entity(h::MODEL_SPACE_END, 5, "ENDBLK", None, DwgEntityBody::BlockEnd(DwgBlockEndEntity { common: block_common(DwgEntityMode::ModelSpace) })),
            owned_object(h::MODEL_LAYOUT, 82, "LAYOUT", h::LAYOUT_DICTIONARY, Some(h::MODEL_LAYOUT_DICTIONARY), DwgLogicalObjectBody::Layout(layout("Model", true, h::MODEL_SPACE, Some(h::ACTIVE_VIEWPORT)))),
            dictionary(h::MODEL_LAYOUT_DICTIONARY, Some(h::MODEL_LAYOUT), &[]),
            record(h::PAPER_SPACE, 49, "BLOCK_HEADER", h::BLOCK_CONTROL, DwgTableRecordBody::BlockHeader(block_header("*Paper_Space", Vec::new(), h::PAPER_SPACE_BLOCK, h::PAPER_SPACE_END, h::PAPER_LAYOUT))),
            entity(h::PAPER_SPACE_BLOCK, 4, "BLOCK", None, DwgEntityBody::BlockBegin(DwgBlockBeginEntity { common: block_common(DwgEntityMode::PaperSpace) })),
            entity(h::PAPER_SPACE_END, 5, "ENDBLK", None, DwgEntityBody::BlockEnd(DwgBlockEndEntity { common: block_common(DwgEntityMode::PaperSpace) })),
            owned_object(h::PAPER_LAYOUT, 82, "LAYOUT", h::LAYOUT_DICTIONARY, Some(h::PAPER_LAYOUT_DICTIONARY), DwgLogicalObjectBody::Layout(layout("Layout1", false, h::PAPER_SPACE, None))),
            dictionary(h::PAPER_LAYOUT_DICTIONARY, Some(h::PAPER_LAYOUT), &[]),
        ];
        for (layer, handle) in layers.iter().zip(&layer_handles) {
            objects.push(record(
                *handle,
                51,
                "LAYER",
                h::LAYER_CONTROL,
                DwgTableRecordBody::Layer(DwgLayerTableRecord {
                    common: DwgTableRecordCommon { name: layer.name.clone(), ..Default::default() },
                    plottable: true,
                    lineweight: 31,
                    color: DwgComplexColor { index: 0, value: DwgComplexColorValue::ByAci { index: u16::from(layer.color) }, ..Default::default() },
                    plot_style_handle: Some(h::NORMAL_PLOT_STYLE),
                    linetype_handle: Some(h::CONTINUOUS),
                    ..Default::default()
                }),
            ));
        }
        objects.extend(entity_objects);
        objects.sort_by_key(|object| object.handle);
        Ok(Self { layers: layers.iter().map(|layer| DwgLogicalLayer { name: layer.name.clone(), color: layer.color }).collect(), objects, extmin: drawing.extmin.to_vec(), extmax: drawing.extmax.to_vec() })
    }

    /// 0️⃣ The handle of layer `0`, the layer a new document's header makes current.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn layer_zero_handle(&self) -> Result<u64, String> {
        self.objects
            .iter()
            .find_map(|object| match object.body.as_ref() {
                Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(layer))) if layer.common.name == "0" => Some(object.handle),
                _ => None,
            })
            .ok_or_else(|| "layer 0 is missing".into())
    }

    /// 🔢️ The next handle a new object of this drawing would take (the header's `HANDSEED`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn handle_seed(&self) -> u64 {
        self.objects.iter().map(|object| object.handle).max().unwrap_or_default() + 1
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn entities(&self) -> Vec<DwgLogicalEntity> {
        let by_handle: std::collections::HashMap<u64, &DwgEntityBody> = self
            .objects
            .iter()
            .filter_map(|object| match object.body.as_ref() {
                Some(DwgLogicalObjectBody::Entity(body)) => Some((object.handle, body)),
                _ => None,
            })
            .collect();
        let point = |values: &[f64]| -> [f64; 3] { [values.first().copied().unwrap_or_default(), values.get(1).copied().unwrap_or_default(), values.get(2).copied().unwrap_or_default()] };
        let vertex_points = |handles: &[u64]| -> Vec<[f64; 3]> {
            handles
                .iter()
                .filter_map(|handle| match by_handle.get(handle) {
                    Some(DwgEntityBody::Vertex(vertex)) => Some(point(&vertex.point)),
                    _ => None,
                })
                .collect()
        };
        let layer_handles: Vec<u64> = self
            .objects
            .iter()
            .filter_map(|candidate| match candidate.body.as_ref() {
                Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(_))) => Some(candidate.handle),
                _ => None,
            })
            .collect();
        self.objects
            .iter()
            .filter_map(|object| {
                let Some(DwgLogicalObjectBody::Entity(body)) = object.body.as_ref() else { return None };
                let (common, geometry) = match body {
                    DwgEntityBody::Line(line) => (&line.common, semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Line { start: point(&line.start), end: point(&line.end) }),
                    DwgEntityBody::Point(value) => (&value.common, semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Point { at: point(&value.point) }),
                    DwgEntityBody::Circle(circle) => (&circle.common, semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Circle { center: point(&circle.center), radius: circle.radius, normal: point(&circle.extrusion) }),
                    DwgEntityBody::Arc(arc) => (&arc.common, semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Arc { center: point(&arc.center), radius: arc.radius, start_angle: arc.start_angle, end_angle: arc.end_angle, normal: point(&arc.extrusion) }),
                    DwgEntityBody::Ellipse(ellipse) => (
                        &ellipse.common,
                        semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Ellipse { center: point(&ellipse.center), major_axis: point(&ellipse.major_axis), ratio: ellipse.axis_ratio, start_param: ellipse.start_parameter, end_param: ellipse.end_parameter, normal: point(&ellipse.extrusion) },
                    ),
                    DwgEntityBody::LwPolyline(polyline) => (
                        &polyline.common,
                        semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::LwPolyline {
                            closed: polyline.closed,
                            elevation: polyline.elevation,
                            vertices: polyline.vertices.iter().map(|vertex| [vertex.point.first().copied().unwrap_or_default(), vertex.point.get(1).copied().unwrap_or_default()]).collect(),
                            bulges: polyline.vertices.iter().map(|vertex| vertex.bulge).collect(),
                        },
                    ),
                    DwgEntityBody::Spline(spline) => (
                        &spline.common,
                        semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Spline { degree: spline.degree, control_points: spline.control_points.as_chunks::<3>().0.iter().map(|chunk| *chunk).collect(), knots: spline.knots.clone(), weights: spline.weights.clone() },
                    ),
                    DwgEntityBody::Text(text) => (
                        &text.common,
                        semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Text { at: [text.insertion.first().copied().unwrap_or_default(), text.insertion.get(1).copied().unwrap_or_default(), text.elevation], height: text.height, rotation: text.rotation, content: text.value.clone() },
                    ),
                    DwgEntityBody::Face3d(face) => {
                        let corners = face.corners.as_chunks::<3>().0;
                        (&face.common, semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Face3d { corners: [0, 1, 2, 3].map(|corner| corners.get(corner).copied().unwrap_or_default()) })
                    }
                    DwgEntityBody::Polyline3d(polyline) => (&polyline.common, semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::Polyline3d { closed: polyline.flags & 1 != 0, vertices: vertex_points(&polyline.vertex_handles) }),
                    DwgEntityBody::PolyfaceMesh(mesh) => (
                        &mesh.common,
                        semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry::PolyfaceMesh {
                            vertices: vertex_points(&mesh.vertex_handles),
                            faces: mesh
                                .vertex_handles
                                .iter()
                                .filter_map(|handle| match by_handle.get(handle) {
                                    Some(DwgEntityBody::PolyfaceFace(face)) => Some([0, 1, 2, 3].map(|slot| i32::from(face.indices.get(slot).copied().unwrap_or_default()))),
                                    _ => None,
                                })
                                .collect(),
                        },
                    ),
                    DwgEntityBody::BlockBegin(_) | DwgEntityBody::BlockEnd(_) | DwgEntityBody::Insert(_) | DwgEntityBody::DimensionLinear(_) | DwgEntityBody::Viewport(_) | DwgEntityBody::Vertex(_) | DwgEntityBody::PolyfaceFace(_) | DwgEntityBody::SequenceEnd(_) => return None,
                };
                let color = match common.color.kind {
                    DwgEntityColorKind::ByLayer => -1,
                    DwgEntityColorKind::ByBlock => -2,
                    DwgEntityColorKind::Index => common.color.index as i16,
                    DwgEntityColorKind::TrueColor => return None,
                };
                let layer = layer_handles.iter().position(|handle| *handle == common.layer_handle).unwrap_or(0);
                Some(DwgLogicalEntity { layer, color, geometry: DwgLogicalGeometry::from_native(&geometry) })
            })
            .collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_native(&self) -> Result<semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing, String> {
        Ok(semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing {
            layers: self.layers.iter().map(|layer| semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer { name: layer.name.clone(), color: layer.color }).collect(),
            entities: self.entities().iter().map(DwgLogicalEntity::to_native).collect::<Result<_, _>>()?,
            extmin: vec3(&self.extmin)?,
            extmax: vec3(&self.extmax)?,
        })
    }
}
/// 🗂️ Handles of the standard objects [`DwgLogicalDrawing::from_native`] gives a new document,
/// following AutoCAD 2010's numbering for the ones it has a fixed number for.
pub mod new_document_handles {
    pub const BLOCK_CONTROL: u64 = 0x01;
    pub const LAYER_CONTROL: u64 = 0x02;
    pub const STYLE_CONTROL: u64 = 0x03;
    pub const LINETYPE_CONTROL: u64 = 0x05;
    pub const VIEW_CONTROL: u64 = 0x06;
    pub const UCS_CONTROL: u64 = 0x07;
    pub const VIEWPORT_CONTROL: u64 = 0x08;
    pub const APPID_CONTROL: u64 = 0x09;
    pub const DIMSTYLE_CONTROL: u64 = 0x0a;
    pub const NAMED_OBJECTS: u64 = 0x0c;
    pub const GROUP_DICTIONARY: u64 = 0x0d;
    pub const PLOT_STYLE_NAME_DICTIONARY: u64 = 0x0e;
    pub const NORMAL_PLOT_STYLE: u64 = 0x0f;
    pub const STANDARD_TEXT_STYLE: u64 = 0x11;
    pub const ACAD_APPLICATION: u64 = 0x12;
    pub const BY_BLOCK: u64 = 0x14;
    pub const BY_LAYER: u64 = 0x15;
    pub const CONTINUOUS: u64 = 0x16;
    pub const MLINE_STYLE_DICTIONARY: u64 = 0x17;
    pub const STANDARD_MLINE_STYLE: u64 = 0x18;
    pub const PLOT_SETTINGS_DICTIONARY: u64 = 0x19;
    pub const LAYOUT_DICTIONARY: u64 = 0x1a;
    pub const STANDARD_DIMSTYLE: u64 = 0x1b;
    pub const ACTIVE_VIEWPORT: u64 = 0x1c;
    pub const MATERIAL_DICTIONARY: u64 = 0x1d;
    pub const COLOR_DICTIONARY: u64 = 0x1e;
    pub const MODEL_SPACE: u64 = 0x1f;
    pub const MODEL_SPACE_BLOCK: u64 = 0x20;
    pub const MODEL_SPACE_END: u64 = 0x21;
    pub const MODEL_LAYOUT: u64 = 0x22;
    pub const MODEL_LAYOUT_DICTIONARY: u64 = 0x23;
    pub const PAPER_SPACE: u64 = 0x24;
    pub const PAPER_SPACE_BLOCK: u64 = 0x25;
    pub const PAPER_SPACE_END: u64 = 0x26;
    pub const PAPER_LAYOUT: u64 = 0x27;
    pub const PAPER_LAYOUT_DICTIONARY: u64 = 0x28;
    pub const VISUAL_STYLE_DICTIONARY: u64 = 0x29;
    pub const FIRST_FREE: u64 = 0x2a;
}
//#endregion 🔖️DrawingModel
