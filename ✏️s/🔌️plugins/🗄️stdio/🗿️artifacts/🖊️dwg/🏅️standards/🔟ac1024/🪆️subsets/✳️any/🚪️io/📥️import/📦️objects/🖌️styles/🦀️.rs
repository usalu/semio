//! 🖌️ Style object decoders: materials, multiline, multileader, table and visual styles.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::validate_entity_terminal_fill;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2010_cell_style;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::r2010_string_content_end_bit;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_material_color;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_material_map;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_object_handle;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_table_style_color;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_visual_style_color;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_visual_style_operation;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgAnnotationScale;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgDrawOrderEntry;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLayout;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLayoutOptions;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObject;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObjectBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgPlaceholder;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgPlotArea;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgPlotOptions;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgPlotPaperUnit;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgPlotRotation;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgShadePlot;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgShadePlotResolution;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgSortEntitiesTable;
use semio_s_artifact_stdio_dwg_snapshot_objects::DwgStandardScale;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationProperty;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationPropertyKind;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgCompatibilityProfile;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgCustomProperty;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgMeasurement;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgPreviewOrigin;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgProductInformation;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRgba;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgVersionStamp;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgArcEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgBlockBeginEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgBlockEndEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgCircleEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgDefaultLightingType;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgDimensionEntityCommon;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgDimensionLineSpacingStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgDimensionStatus;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgDimensionTextAttachment;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgEllipseEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgEntityBody;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgEntityColor;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgEntityColorKind;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgEntityCommon;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgEntityMode;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgEntityReferenceMode;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgFace3dEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgInsertEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgLineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgLinearDimensionEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgLwPolylineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgLwPolylineVertex;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgOrthographicView;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgPointEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgPolyfaceFaceEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgPolyfaceMeshEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgPolyline3dEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgSequenceEndEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgShadePlotMode;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgSplineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgTextEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgVertexEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgViewportEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgViewportRenderMode;
use semio_s_artifact_stdio_dwg_snapshot_objects_entities::DwgViewportStatusFlag;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgAssoc2dConstraintGroup;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgAxisConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgConstrainedBoundedLine;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgConstrainedDatumLine;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgConstrainedImplicitPoint;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgConstraintGeometry;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgConstraintNode;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgConstraintNodeCore;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgDistanceConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgExplicitConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects_constraints::DwgGeometricConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssocNetwork;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssocNetworkMember;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssocNetworkMemberKind;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeAction;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeActionDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeActionStatus;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeDependencyStatus;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeDimensionDependencyBody;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeGeometryDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeValueDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgAssociativeVariable;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgBlockGripLocationComponent;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgDynamicBlockProxyNode;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgEvaluationExpression;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgEvaluationExpressionValue;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgEvaluationGraph;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgEvaluationGraphEdge;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgEvaluationGraphNode;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgEvaluationVariant;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgNamedEvaluationNodeReference;
use semio_s_artifact_stdio_dwg_snapshot_objects_associativity::DwgPropertyExpressionReference;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockAction;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockActionConnection;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockActionCoordinateMode;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockActionDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockActionWithBasePoint;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockAlignmentGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockAlignmentParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockBasePointParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockElement;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockFlipAction;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockFlipGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockFlipParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockFlipValueSet;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockLinearConstraintParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockLinearGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockLinearParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockMoveAction;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockMoveCoordinateMode;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockOnePointParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockParameterAllowedValues;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockParameterBaseLocation;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockParameterConnection;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockParameterDependencyBody;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockParameterProperty;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockRepresentationData;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockScaleAction;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockScaleMode;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockStretchAction;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockTwoPointParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockVisibilityGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgBlockVisibilityParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgDynamicBlockPurgePreventer;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgStretchSelection;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgStretchSelector;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgVisibilityEvaluationHistory;
use semio_s_artifact_stdio_dwg_snapshot_objects_blocks::DwgVisibilityState;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgBlockHeaderTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgBlockTableControl;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgComplexColor;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgComplexColorValue;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgDimensionBehavior;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgDimensionGeometry;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgDimensionR2010;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgDimensionStyleTableControl;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgDimensionStyleTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgDimensionText;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgDimensionUnits;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgLayerTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgLinetypeDash;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgLinetypeTableControl;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgLinetypeTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgRegisteredApplicationTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgTableControlBody;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgTableControlEntries;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgTableControlEntry;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgTableRecordBody;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgTableRecordCommon;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgTextStyleTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgViewportTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgCellBorder;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgCellBorders;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgCellContentFormat;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgCellMargins;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgCellStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderArrow;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderAttachmentDirection;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderBlockConnection;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderBlockStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderContentType;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderDogleg;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderDrawOrder;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderKind;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderLanding;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderLeaderOrder;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderLeaderStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderTextAlignment;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderTextAngle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderTextAttachment;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMLeaderTextStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterial;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterialChannels;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterialColor;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterialMap;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterialMapSource;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterialProjection;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterialTiling;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineCaps;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineLinetype;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineStyleElement;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgTableStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgVisualStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgVisualStyleProperties;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgVisualStyleProperty;
use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgVisualStylePropertyOperation;
use semio_s_artifact_stdio_dwg_snapshot_objects_xrecord::DwgDictionaryBody;
use semio_s_artifact_stdio_dwg_snapshot_objects_xrecord::DwgDictionaryVariable;
use semio_s_artifact_stdio_dwg_snapshot_objects_xrecord::DwgExtendedEntityData;
use semio_s_artifact_stdio_dwg_snapshot_objects_xrecord::DwgNamedReference;
use semio_s_artifact_stdio_dwg_snapshot_objects_xrecord::DwgObjectCategory;
use semio_s_artifact_stdio_dwg_snapshot_objects_xrecord::DwgXRecordBody;
use semio_s_artifact_stdio_dwg_snapshot_objects_xrecord::DwgXRecordValue;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderSpaceGeometry;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::DwgLogicalLayer;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
/// 🧩️ Decodes one material object body from its main, string and handle streams.
pub fn decode_material_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterial;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMaterialChannels;
    let strings = strings.as_mut().ok_or_else(|| format!("MATERIAL {handle:#x} string stream missing"))?;
    let name = strings.read_tu()?;
    let description = strings.read_tu()?;
    let ambient = read_material_color(data, handle)?;
    let diffuse = read_material_color(data, handle)?;
    let diffuse_map = read_material_map(data, strings, handle)?;
    let specular = read_material_color(data, handle)?;
    let specular_map = read_material_map(data, strings, handle)?;
    let specular_gloss = data.read_bd()?;
    let reflection_map = read_material_map(data, strings, handle)?;
    let opacity = data.read_bd()?;
    let opacity_map = read_material_map(data, strings, handle)?;
    let bump_map = read_material_map(data, strings, handle)?;
    let refraction_index = data.read_bd()?;
    let refraction_map = read_material_map(data, strings, handle)?;
    let translucence = data.read_bd()?;
    let self_illumination = data.read_bd()?;
    let reflectivity = data.read_bd()?;
    if data.read_bl()? != 0 {
        return Err(format!("MATERIAL {handle:#x} illumination model is unsupported"));
    }
    let channels = data.read_bl()?;
    if channels & !63 != 0 || data.read_bl()? != 0 {
        return Err(format!("MATERIAL {handle:#x} channel or mode state is unsupported"));
    }
    if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("MATERIAL {handle:#x} main or string stream is not exactly consumed"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "MATERIAL")?;
    object.body = Some(DwgLogicalObjectBody::Material(DwgMaterial {
        name,
        description,
        ambient,
        diffuse,
        diffuse_map,
        specular,
        specular_map,
        specular_gloss,
        reflection_map,
        opacity,
        opacity_map,
        bump_map,
        refraction_index,
        refraction_map,
        translucence,
        self_illumination,
        reflectivity,
        enabled_channels: DwgMaterialChannels { diffuse: channels & 1 != 0, specular: channels & 2 != 0, reflection: channels & 4 != 0, opacity: channels & 8 != 0, bump: channels & 16 != 0, refraction: channels & 32 != 0 },
    }));

    Ok(())
}
/// 🧩️ Decodes one multiline style object body from its main, string and handle streams.
pub fn decode_mline_style_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineCaps;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineLinetype;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineStyle;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgMlineStyleElement;
    let flags = data.read_bs()?;
    if flags & !0x0773 != 0 {
        return Err(format!("MLINESTYLE {handle:#x} flags {flags:#x} contain unknown concepts"));
    }
    let fill_color = read_table_style_color(data, handle)?;
    let start_angle = data.read_bd()?;
    let end_angle = data.read_bd()?;
    let count = data.read_rc()? as usize;
    if count == 0 {
        return Err(format!("MLINESTYLE {handle:#x} has no elements"));
    }
    let mut elements = Vec::with_capacity(count);
    for _ in 0..count {
        let offset = data.read_bd()?;
        let color = read_table_style_color(data, handle)?;
        let linetype = match data.read_bs()? {
            32767 => DwgMlineLinetype::ByLayer,
            32766 => DwgMlineLinetype::ByBlock,
            0 => DwgMlineLinetype::Continuous,
            value => return Err(format!("MLINESTYLE {handle:#x} linetype index {value} is unsupported")),
        };
        elements.push(DwgMlineStyleElement { offset, color, linetype });
    }
    if elements.windows(2).any(|pair| pair[0].offset <= pair[1].offset) || data.bit_position() != main_end_bit {
        return Err(format!("MLINESTYLE {handle:#x} main stream or element ordering is invalid"));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("MLINESTYLE {handle:#x} string stream missing"))?;
    let name = strings.read_tu()?;
    let description = strings.read_tu()?;
    if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("MLINESTYLE {handle:#x} string stream is not exactly consumed"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "MLINESTYLE")?;
    object.body = Some(DwgLogicalObjectBody::MlineStyle(DwgMlineStyle {
        name,
        description,
        fill_enabled: flags & 1 != 0,
        display_miters: flags & 2 != 0,
        start_caps: DwgMlineCaps { square: flags & 0x10 != 0, inner_arcs: flags & 0x20 != 0, round_outer_arcs: flags & 0x40 != 0 },
        end_caps: DwgMlineCaps { square: flags & 0x100 != 0, inner_arcs: flags & 0x200 != 0, round_outer_arcs: flags & 0x400 != 0 },
        fill_color,
        start_angle,
        end_angle,
        elements,
    }));

    Ok(())
}
/// 🧩️ Decodes one multileader style object body from its main, string and handle streams.
pub fn decode_mleader_style_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObject) -> Result<(), String> {
    if data.read_bs()? != 2 {
        return Err(format!("MLEADERSTYLE {handle:#x} class version is unsupported"));
    }
    let content_type = match data.read_bs()? {
        0 => DwgMLeaderContentType::None,
        1 => DwgMLeaderContentType::Block,
        2 => DwgMLeaderContentType::MText,
        value => return Err(format!("MLEADERSTYLE {handle:#x} content type {value} is invalid")),
    };
    let draw_order = match data.read_bs()? {
        0 => DwgMLeaderDrawOrder::LeaderFirst,
        1 => DwgMLeaderDrawOrder::ContentFirst,
        value => return Err(format!("MLEADERSTYLE {handle:#x} draw order {value} is invalid")),
    };
    let leader_order = match data.read_bs()? {
        0 => DwgMLeaderLeaderOrder::HeadFirst,
        1 => DwgMLeaderLeaderOrder::TailFirst,
        value => return Err(format!("MLEADERSTYLE {handle:#x} leader order {value} is invalid")),
    };
    let maximum_segment_points = data.read_bl()?;
    let first_segment_angle = data.read_bd()?;
    let second_segment_angle = data.read_bd()?;
    let kind = match data.read_bs()? {
        0 => DwgMLeaderKind::Invisible,
        1 => DwgMLeaderKind::Straight,
        2 => DwgMLeaderKind::Spline,
        value => return Err(format!("MLEADERSTYLE {handle:#x} leader kind {value} is invalid")),
    };
    let leader_color = read_table_style_color(data, handle)?;
    let lineweight = data.read_bl()? as i32;
    let landing = DwgMLeaderLanding { enabled: data.read_b()?, gap: data.read_bd()? };
    let dogleg = DwgMLeaderDogleg { enabled: data.read_b()?, length: data.read_bd()? };
    let arrow_size = data.read_bd()?;
    let attachment = |value: u16| -> Result<DwgMLeaderTextAttachment, String> {
        Ok(match value {
            0 => DwgMLeaderTextAttachment::TopOfTop,
            1 => DwgMLeaderTextAttachment::MiddleOfTop,
            2 => DwgMLeaderTextAttachment::Middle,
            3 => DwgMLeaderTextAttachment::MiddleOfBottom,
            4 => DwgMLeaderTextAttachment::BottomOfBottom,
            5 => DwgMLeaderTextAttachment::BottomLine,
            6 => DwgMLeaderTextAttachment::BottomOfTop,
            7 => DwgMLeaderTextAttachment::BottomOfTopUnderline,
            8 => DwgMLeaderTextAttachment::BottomOfTopNoUnderline,
            9 => DwgMLeaderTextAttachment::Center,
            _ => return Err(format!("MLEADERSTYLE attachment {value} is invalid")),
        })
    };
    let left_attachment = attachment(data.read_bs()?)?;
    let right_attachment = attachment(data.read_bs()?)?;
    let angle = match data.read_bs()? {
        0 => DwgMLeaderTextAngle::Horizontal,
        1 => DwgMLeaderTextAngle::Aligned,
        2 => DwgMLeaderTextAngle::AlwaysRightReading,
        value => return Err(format!("MLEADERSTYLE {handle:#x} text angle {value} is invalid")),
    };
    let alignment = match data.read_bs()? {
        0 => DwgMLeaderTextAlignment::Left,
        1 => DwgMLeaderTextAlignment::Center,
        2 => DwgMLeaderTextAlignment::Right,
        value => return Err(format!("MLEADERSTYLE {handle:#x} text alignment {value} is invalid")),
    };
    let text_color = read_table_style_color(data, handle)?;
    let text_height = data.read_bd()?;
    let frame = data.read_b()?;
    let always_left = data.read_b()?;
    let alignment_space = data.read_bd()?;
    let block_color = read_table_style_color(data, handle)?;
    let block_scale = vec![data.read_bd()?, data.read_bd()?, data.read_bd()?];
    let use_scale = data.read_b()?;
    let rotation = data.read_bd()?;
    let use_rotation = data.read_b()?;
    let connection = match data.read_bs()? {
        0 => DwgMLeaderBlockConnection::Extents,
        1 => DwgMLeaderBlockConnection::BasePoint,
        value => return Err(format!("MLEADERSTYLE {handle:#x} block connection {value} is invalid")),
    };
    let overall_scale = data.read_bd()?;
    let property_overrides_changed = data.read_b()?;
    let annotative = data.read_b()?;
    let break_size = data.read_bd()?;
    let attachment_direction = match data.read_bs()? {
        0 => DwgMLeaderAttachmentDirection::Horizontal,
        1 => DwgMLeaderAttachmentDirection::Vertical,
        value => return Err(format!("MLEADERSTYLE {handle:#x} attachment direction {value} is invalid")),
    };
    let top_attachment = attachment(data.read_bs()?)?;
    let bottom_attachment = attachment(data.read_bs()?)?;
    if data.bit_position() != main_end_bit {
        return Err(format!("MLEADERSTYLE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("MLEADERSTYLE {handle:#x} string stream missing"))?;
    let description = strings.read_tu()?;
    let default_content = strings.read_tu()?;
    if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("MLEADERSTYLE {handle:#x} string stream is not exactly consumed"));
    }
    let linetype_style_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("MLEADERSTYLE {handle:#x} linetype style is null"))?;
    let symbol_handle = read_object_handle(handle_reader, handle)?;
    let style_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("MLEADERSTYLE {handle:#x} text style is null"))?;
    let content_handle = read_object_handle(handle_reader, handle)?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "MLEADERSTYLE")?;
    object.body = Some(DwgLogicalObjectBody::MLeaderStyle(DwgMLeaderStyle {
        content_type,
        draw_order,
        leader_order,
        maximum_segment_points,
        first_segment_angle,
        second_segment_angle,
        leader: DwgMLeaderLeaderStyle { kind, color: leader_color, linetype_style_handle, lineweight },
        landing,
        dogleg,
        description,
        arrow: DwgMLeaderArrow { symbol_handle, size: arrow_size },
        text: DwgMLeaderTextStyle {
            default_content,
            style_handle,
            left_attachment,
            right_attachment,
            angle,
            alignment,
            color: text_color,
            height: text_height,
            frame,
            always_left,
            alignment_space,
            attachment_direction,
            top_attachment,
            bottom_attachment,
        },
        block: DwgMLeaderBlockStyle { content_handle, color: block_color, scale: block_scale, use_scale, rotation, use_rotation, connection },
        overall_scale,
        property_overrides_changed,
        annotative,
        break_size,
    }));

    Ok(())
}
/// 🧩️ Decodes one table style object body from its main, string and handle streams.
pub fn decode_table_style_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgTableStyle;
    if data.read_rc()? != 0 {
        return Err(format!("TABLESTYLE {handle:#x} native discriminator is unsupported"));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("TABLESTYLE {handle:#x} string stream missing"))?;
    let description = strings.read_tu()?;
    if data.read_bl()? != 0 {
        return Err(format!("TABLESTYLE {handle:#x} native format version is unsupported"));
    }
    let bit_flags = data.read_bl()?;
    let template_style_handle = read_object_handle(handle_reader, handle)?;
    let table = decode_r2010_cell_style(data, strings, handle_reader, handle)?;
    if data.read_bl()? != 4 || data.read_bl()? != 2 || strings.read_tu()? != "Table" || data.read_bl()? != 3 {
        return Err(format!("TABLESTYLE {handle:#x} base identity or override count is invalid"));
    }
    let mut overrides = Vec::with_capacity(3);
    for (expected_id, expected_type, expected_name) in [(1, 1, "_TITLE"), (2, 1, "_HEADER"), (3, 2, "_DATA")] {
        if data.read_bl()? != expected_id {
            return Err(format!("TABLESTYLE {handle:#x} override selector is invalid"));
        }
        let style = decode_r2010_cell_style(data, strings, handle_reader, handle)?;
        if data.read_bl()? != expected_id || data.read_bl()? != expected_type || strings.read_tu()? != expected_name {
            return Err(format!("TABLESTYLE {handle:#x} override identity is invalid"));
        }
        overrides.push(style);
    }
    if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("TABLESTYLE {handle:#x} main or string stream is not exactly consumed"));
    }
    if object.owner_handle.is_none() || object.reactor_handles.len() != 1 || object.extension_dictionary_handle.is_none() || !object.extended_data.is_empty() {
        return Err(format!("TABLESTYLE {handle:#x} common state is invalid"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "TABLESTYLE")?;
    let mut overrides = overrides.into_iter();
    object.body = Some(DwgLogicalObjectBody::TableStyle(Box::new(DwgTableStyle { description, bit_flags, template_style_handle, table, title: overrides.next().unwrap(), header: overrides.next().unwrap(), data: overrides.next().unwrap() })));

    Ok(())
}
/// 🧩️ Decodes one visual style object body from its main, string and handle streams.
pub fn decode_visual_style_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgVisualStyle;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgVisualStyleProperties;
    use semio_s_artifact_stdio_dwg_snapshot_objects_styles::DwgVisualStyleProperty;
    let style_type = data.read_bl()?;
    let extension_lighting_model = data.read_bs()?;
    let internal_only = data.read_b()?;
    macro_rules! blp {
        () => {{
            let value = data.read_bl()?;
            let operation = read_visual_style_operation(data)?;
            DwgVisualStyleProperty { value, operation }
        }};
    }
    macro_rules! bsp {
        () => {{
            let value = data.read_bs()?;
            let operation = read_visual_style_operation(data)?;
            DwgVisualStyleProperty { value, operation }
        }};
    }
    macro_rules! bdp {
        () => {{
            let value = data.read_bd()?;
            let operation = read_visual_style_operation(data)?;
            DwgVisualStyleProperty { value, operation }
        }};
    }
    let face_lighting_model = blp!();
    let face_lighting_quality = blp!();
    let face_color_mode = blp!();
    let face_modifiers = bsp!();
    let face_opacity = bdp!();
    let face_specular_amount = bdp!();
    let (face_monochrome_color, face_monochrome_flags) = read_visual_style_color(data)?;
    let edge_model = blp!();
    let edge_styles = blp!();
    let (edge_intersection_color, edge_intersection_flags) = read_visual_style_color(data)?;
    let (edge_obscured_color, edge_obscured_flags) = read_visual_style_color(data)?;
    let edge_obscured_line_pattern = blp!();
    let edge_intersection_line_pattern = blp!();
    let edge_crease_angle = bdp!();
    let edge_modifiers = blp!();
    let (edge_color, edge_color_flags) = read_visual_style_color(data)?;
    let edge_opacity = bdp!();
    let edge_width = blp!();
    let edge_overhang = blp!();
    let edge_jitter = blp!();
    let (edge_silhouette_color, edge_silhouette_flags) = read_visual_style_color(data)?;
    let edge_silhouette_width = blp!();
    let edge_halo_gap = blp!();
    let edge_isolines = blp!();
    let hidden_edge_precision = DwgVisualStyleProperty { value: data.read_b()?, operation: read_visual_style_operation(data)? };
    let display_settings = blp!();
    let display_brightness = bdp!();
    let display_shadow_type = blp!();
    if data.bit_position() != main_end_bit {
        return Err(format!("VISUALSTYLE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("VISUALSTYLE {handle:#x} string stream missing"))?;
    let description = strings.read_tu()?;
    let mut properties = DwgVisualStyleProperties {
        face_lighting_model,
        face_lighting_quality,
        face_color_mode,
        face_modifiers,
        face_opacity,
        face_specular_amount,
        face_monochrome_color,
        edge_model,
        edge_styles,
        edge_intersection_color,
        edge_obscured_color,
        edge_obscured_line_pattern,
        edge_intersection_line_pattern,
        edge_crease_angle,
        edge_modifiers,
        edge_color,
        edge_opacity,
        edge_width,
        edge_overhang,
        edge_jitter,
        edge_silhouette_color,
        edge_silhouette_width,
        edge_halo_gap,
        edge_isolines,
        hidden_edge_precision,
        display_settings,
        display_brightness,
        display_shadow_type,
    };
    let mut read_color_strings = |color: &mut semio_s_artifact_stdio_dwg_snapshot_objects_tables::DwgComplexColor, flags: u8| -> Result<(), String> {
        if flags & 1 != 0 {
            color.name = Some(strings.read_tu()?);
        }
        if flags & 2 != 0 {
            color.book_name = Some(strings.read_tu()?);
        }
        Ok(())
    };
    read_color_strings(&mut properties.face_monochrome_color.value, face_monochrome_flags)?;
    read_color_strings(&mut properties.edge_intersection_color.value, edge_intersection_flags)?;
    read_color_strings(&mut properties.edge_obscured_color.value, edge_obscured_flags)?;
    read_color_strings(&mut properties.edge_color.value, edge_color_flags)?;
    read_color_strings(&mut properties.edge_silhouette_color.value, edge_silhouette_flags)?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("VISUALSTYLE {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "VISUALSTYLE")?;
    object.body = Some(DwgLogicalObjectBody::VisualStyle(DwgVisualStyle { description, style_type, extension_lighting_model, internal_only, properties }));

    Ok(())
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
