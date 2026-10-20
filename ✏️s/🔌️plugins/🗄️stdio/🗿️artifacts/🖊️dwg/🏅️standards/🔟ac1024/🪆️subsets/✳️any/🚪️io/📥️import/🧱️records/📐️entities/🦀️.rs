//! 📐️ Entity records: the object-record frame, the common entity fields and every entity body decoder.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_crc16;
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::DwgLogicalDrawing;
use crate::standards::v_ac1024::subsets::any::io::import::records::fields::{DwgObjectRelations, r2010_string_stream, read_object_handle};
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgAnnotationScale;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgDrawOrderEntry;
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
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgSortEntitiesTable;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgStandardScale;
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
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgArcEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgBlockBeginEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgBlockEndEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgCircleEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDefaultLightingType;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionEntityCommon;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionStatus;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionTextAttachment;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEllipseEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColor;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColorKind;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgFace3dEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgInsertEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLinearDimensionEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLwPolylineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLwPolylineVertex;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgOrthographicView;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPointEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPolyfaceFaceEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPolyfaceMeshEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgPolyline3dEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgSequenceEndEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgShadePlotMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgSplineEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgTextEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgVertexEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportEntity;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportRenderMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportStatusFlag;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgAssoc2dConstraintGroup;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgAxisConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstrainedBoundedLine;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstrainedDatumLine;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstrainedImplicitPoint;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintGeometry;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNodeCore;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgDistanceConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgExplicitConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgGeometricConstraint;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetwork;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetworkMember;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetworkMemberKind;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeAction;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionStatus;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependencyStatus;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDimensionDependencyBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeGeometryDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeValueDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeVariable;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgBlockGripLocationComponent;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgDynamicBlockProxyNode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpression;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationGraph;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationGraphEdge;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationGraphNode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationVariant;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgNamedEvaluationNodeReference;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgPropertyExpressionReference;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAction;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionConnection;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionCoordinateMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionDependency;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionWithBasePoint;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAlignmentGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAlignmentParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockBasePointParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockElement;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipAction;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipValueSet;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockLinearConstraintParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockLinearGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockLinearParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockMoveAction;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockMoveCoordinateMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockOnePointParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterAllowedValues;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterConnection;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterDependencyBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterProperty;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockRepresentationData;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockScaleAction;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockScaleMode;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockStretchAction;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockTwoPointParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockVisibilityGrip;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockVisibilityParameter;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgDynamicBlockPurgePreventer;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgStretchSelection;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgStretchSelector;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgVisibilityEvaluationHistory;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgVisibilityState;
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
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLinetypeDash;
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
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorder;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorders;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellContentFormat;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellMargins;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderArrow;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderAttachmentDirection;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderBlockConnection;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderBlockStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderContentType;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderDogleg;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderDrawOrder;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderKind;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderLanding;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderLeaderOrder;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderLeaderStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderTextAlignment;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderTextAngle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderTextAttachment;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderTextStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterial;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialChannels;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialColor;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMap;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMapSource;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialProjection;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialTiling;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineCaps;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineLinetype;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineStyleElement;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgTableStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStyle;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStyleProperties;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStyleProperty;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStylePropertyOperation;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgDictionaryBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgDictionaryVariable;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgExtendedEntityData;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgNamedReference;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgObjectCategory;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderSpaceGeometry;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::DwgLogicalLayer;

//#region 🔖️DwgStructuralCodec
//#region DwgObjects
pub const DWG_TYPE_LAYER: u16 = 51;
pub const DWG_TYPE_BLOCK: u16 = 4;
pub const DWG_TYPE_ENDBLK: u16 = 5;
pub const DWG_TYPE_INSERT: u16 = 7;
const DWG_TYPE_LINE: u16 = 19;
pub const DWG_TYPE_DIMENSION_LINEAR: u16 = 21;
pub const DWG_TYPE_VIEWPORT: u16 = 34;
const DWG_TYPE_POINT: u16 = 27;
const DWG_TYPE_CIRCLE: u16 = 18;
const DWG_TYPE_ARC: u16 = 17;
const DWG_TYPE_ELLIPSE: u16 = 35;
const DWG_TYPE_LWPOLYLINE: u16 = 77;
const DWG_TYPE_SPLINE: u16 = 36;
const DWG_TYPE_TEXT: u16 = 1;
const DWG_TYPE_FACE3D: u16 = 28;
const DWG_TYPE_POLYLINE3D: u16 = 16;
const DWG_TYPE_POLYLINE_PFACE: u16 = 29;
const DWG_TYPE_SEQEND: u16 = 6;
const DWG_TYPE_VERTEX3D: u16 = 11;
const DWG_TYPE_VERTEX_PFACE: u16 = 13;
const DWG_TYPE_VERTEX_PFACE_FACE: u16 = 14;
/// 🗂️ The object-stream layouts the drawing reader distinguishes: R2004 (AC1018) frames carry a BS
/// type and an RL data size and keep strings inline; R2007 (AC1021) keeps the RL size but moves
/// strings into a trailing string stream; R2010 (AC1024) replaces the RL size by a handle-stream
/// size in the frame prefix and a bit-pair object type; R2013+ (AC1027, AC1032) add the
/// `has_ds_data` bit to every object's common data. ODA §20.1–§20.4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DwgObjectLayout {
    R2004,
    R2007,
    R2010,
    R2013,
}
impl DwgObjectLayout {
    pub fn of(version: &str) -> Result<Self, String> {
        match version {
            "AC1018" => Ok(Self::R2004),
            "AC1021" => Ok(Self::R2007),
            "AC1024" => Ok(Self::R2010),
            "AC1027" | "AC1032" => Ok(Self::R2013),
            other => Err(format!("unsupported DWG version {other:?}: this codec reads AC1018, AC1021, AC1024, AC1027 and AC1032")),
        }
    }
}
/// 📦️ One object record opened out of AcDbObjects: its data stream positioned after the identity
/// handle, its string stream (R2007+), its handle stream, and the bit where its main data ends.
pub struct DwgObjectRecordFrame<'a> {
    layout: DwgObjectLayout,
    handle: u64,
    pub type_code: u16,
    pub data: DwgBitReader<'a>,
    strings: Option<DwgBitReader<'a>>,
    pub handles: DwgBitReader<'a>,
    main_end_bit: usize,
    pub end_bit: usize,
}
impl<'a> DwgObjectRecordFrame<'a> {
    pub fn open(layout: DwgObjectLayout, object_data: &'a [u8], handle: u64, address: usize) -> Result<Self, String> {
        let frame = object_data.get(address..).ok_or_else(|| format!("object {handle:#x} address {address:#x} is out of bounds"))?;
        let mut sizer = DwgBitReader::new(frame);
        let payload_size = sizer.read_ms().map_err(|error| format!("object {handle:#x} size: {error}"))? as usize;
        let handle_stream_bits = if layout >= DwgObjectLayout::R2010 { Some(sizer.read_umc().map_err(|error| format!("object {handle:#x} handle-stream size: {error}"))? as usize) } else { None };
        sizer.pad_to_byte();
        let payload_end = sizer.byte_pos.checked_add(payload_size).ok_or_else(|| format!("object {handle:#x} payload overflow"))?;
        let payload = frame.get(sizer.byte_pos..payload_end).ok_or_else(|| format!("object {handle:#x} payload is truncated"))?;
        let stored_crc = frame.get(payload_end..payload_end + 2).map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]])).ok_or_else(|| format!("object {handle:#x} CRC is truncated"))?;
        if dwg_crc16(0xC0C1, &frame[..payload_end]) != stored_crc {
            return Err(format!("object {handle:#x} CRC mismatch"));
        }
        let end_bit = payload_size * 8;
        let mut data = DwgBitReader::new(payload);
        let type_code = if layout >= DwgObjectLayout::R2010 { data.read_bot() } else { data.read_bs() }.map_err(|error| format!("object {handle:#x} type: {error}"))?;
        let handle_start = match handle_stream_bits {
            Some(bits) => end_bit.checked_sub(bits).ok_or_else(|| format!("object {handle:#x} handle stream exceeds its payload"))?,
            None => data.read_rl().map_err(|error| format!("object {handle:#x} data size: {error}"))? as usize,
        };
        if handle_start > end_bit {
            return Err(format!("object {handle:#x} data size exceeds its payload"));
        }
        let (_, encoded) = data.read_handle().map_err(|error| format!("object {handle:#x} identity: {error}"))?;
        if encoded != handle {
            return Err(format!("object map handle {handle:#x} does not match encoded handle {encoded:#x}"));
        }
        let (strings, main_end_bit) = if layout >= DwgObjectLayout::R2007 {
            let (reader, start) = r2010_string_stream(payload, handle_start).map_err(|error| format!("object {handle:#x} string stream: {error}"))?;
            (Some(reader), start)
        } else {
            (None, handle_start)
        };
        let handles = DwgBitReader::at_bit(payload, handle_start).map_err(|error| format!("object {handle:#x} handle stream: {error}"))?;
        Ok(Self { layout, handle, type_code, data, strings, handles, main_end_bit, end_bit })
    }

    /// 📝️ A text field: UTF-16 from the string stream (R2007+) or a code-page string inline (R2004).
    fn text(&mut self) -> Result<String, String> {
        match self.strings.as_mut() {
            Some(strings) => strings.read_tu(),
            None => self.data.read_tv(),
        }
    }

    /// ⏭️ Steps over extended entity data by its declared record sizes.
    pub fn skip_extended_data(&mut self) -> Result<(), String> {
        loop {
            let size = self.data.read_bs()? as usize;
            if size == 0 {
                return Ok(());
            }
            self.data.read_handle()?;
            self.data.skip_bits(size.checked_mul(8).ok_or("EED size overflow")?)?;
        }
    }

    /// 🧾️ Closes the main stream: every entity field was consumed exactly up to the string stream.
    fn finish_main(&self, class_name: &str) -> Result<(), String> {
        if self.data.bit_position() != self.main_end_bit {
            return Err(format!("{class_name} {:#x} main stream ends at bit {} instead of {}", self.handle, self.data.bit_position(), self.main_end_bit));
        }
        Ok(())
    }
}
/// 🧬️ The entity common data of `layout`, after the EED. A preview graphic is skipped when
/// `skip_graphic` (the drawing projection has nothing to keep it in) and refused otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_entity_common(data: &mut DwgBitReader<'_>, layout: DwgObjectLayout, skip_graphic: bool) -> Result<DwgDecodedEntityCommon, String> {
    if data.read_b()? {
        if !skip_graphic {
            return Err("entity graphic requires a typed semantic graphic model".into());
        }
        let byte_count = if layout >= DwgObjectLayout::R2010 { data.read_bll()? } else { u64::from(data.read_rl()?) };
        data.skip_bits(usize::try_from(byte_count).ok().and_then(|bytes| bytes.checked_mul(8)).ok_or("entity graphic length overflow")?)?;
    }
    decode_entity_common_fields(data, layout)
}
/// ✏️ Every drawing entity kind this codec models, decoded from `frame` after its EED: the typed
/// body plus the entity's owner, reactors and extension dictionary. `None` for any other type, and
/// for fit-point splines, which carry no control polygon to model. The one reading of these
/// layouts, shared by the drawing projection and the lossless AC1024 decoder. ODA §20.4.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_entity_body(frame: &mut DwgObjectRecordFrame<'_>, skip_graphic: bool) -> Result<Option<(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody, DwgObjectRelations)>, String> {
    let class_name = match frame.type_code {
        DWG_TYPE_LINE => "LINE",
        DWG_TYPE_ARC => "ARC",
        DWG_TYPE_CIRCLE => "CIRCLE",
        DWG_TYPE_POINT => "POINT",
        DWG_TYPE_ELLIPSE => "ELLIPSE",
        DWG_TYPE_TEXT => "TEXT",
        DWG_TYPE_SPLINE => "SPLINE",
        DWG_TYPE_FACE3D => "3DFACE",
        DWG_TYPE_LWPOLYLINE => "LWPOLYLINE",
        DWG_TYPE_POLYLINE3D => "POLYLINE_3D",
        DWG_TYPE_POLYLINE_PFACE => "POLYLINE_PFACE",
        DWG_TYPE_VERTEX3D => "VERTEX_3D",
        DWG_TYPE_VERTEX_PFACE => "VERTEX_PFACE",
        DWG_TYPE_VERTEX_PFACE_FACE => "VERTEX_PFACE_FACE",
        DWG_TYPE_SEQEND => "SEQEND",
        _ => return Ok(None),
    };
    let handle = frame.handle;
    let context = |error: String| format!("{class_name} {handle:#x}: {error}");
    let mut decoded = decode_entity_common(&mut frame.data, frame.layout, skip_graphic).map_err(context)?;
    let data = &mut frame.data;
    let mut owned_count = 0usize;
    let body = match frame.type_code {
        DWG_TYPE_LINE => {
            let z_is_zero = data.read_b()?;
            let start_x = data.read_rd()?;
            let end_x = data.read_dd(start_x)?;
            let start_y = data.read_rd()?;
            let end_y = data.read_dd(start_y)?;
            let (start_z, end_z) = if z_is_zero {
                (0.0, 0.0)
            } else {
                let start = data.read_rd()?;
                (start, data.read_dd(start)?)
            };
            DwgEntityBody::Line(DwgLineEntity { common: DwgEntityCommon::default(), start: vec![start_x, start_y, start_z], end: vec![end_x, end_y, end_z], thickness: data.read_bt()?, extrusion: data.read_be()?.to_vec() })
        }
        DWG_TYPE_ARC => DwgEntityBody::Arc(DwgArcEntity {
            common: DwgEntityCommon::default(),
            center: data.read_3bd()?.to_vec(),
            radius: data.read_bd()?,
            thickness: data.read_bt()?,
            extrusion: data.read_be()?.to_vec(),
            start_angle: data.read_bd()?,
            end_angle: data.read_bd()?,
        }),
        DWG_TYPE_CIRCLE => DwgEntityBody::Circle(DwgCircleEntity { common: DwgEntityCommon::default(), center: data.read_3bd()?.to_vec(), radius: data.read_bd()?, thickness: data.read_bt()?, extrusion: data.read_be()?.to_vec() }),
        DWG_TYPE_POINT => DwgEntityBody::Point(DwgPointEntity { common: DwgEntityCommon::default(), point: data.read_3bd()?.to_vec(), thickness: data.read_bt()?, extrusion: data.read_be()?.to_vec(), x_axis_angle: data.read_bd()? }),
        DWG_TYPE_ELLIPSE => DwgEntityBody::Ellipse(DwgEllipseEntity {
            common: DwgEntityCommon::default(),
            center: data.read_3bd()?.to_vec(),
            major_axis: data.read_3bd()?.to_vec(),
            extrusion: data.read_3bd()?.to_vec(),
            axis_ratio: data.read_bd()?,
            start_parameter: data.read_bd()?,
            end_parameter: data.read_bd()?,
        }),
        DWG_TYPE_TEXT => {
            let flags = data.read_rc()?;
            let elevation = if flags & 0x01 == 0 { data.read_rd()? } else { 0.0 };
            let insertion = data.read_2rd()?;
            let alignment = if flags & 0x02 == 0 { Some(vec![data.read_dd(insertion[0])?, data.read_dd(insertion[1])?]) } else { None };
            let extrusion = data.read_be()?.to_vec();
            let thickness = data.read_bt()?;
            let oblique_angle = if flags & 0x04 == 0 { data.read_rd()? } else { 0.0 };
            let rotation = if flags & 0x08 == 0 { data.read_rd()? } else { 0.0 };
            let height = data.read_rd()?;
            let width_factor = if flags & 0x10 == 0 { data.read_rd()? } else { 1.0 };
            let value = frame.text().map_err(context)?;
            let data = &mut frame.data;
            let generation = if flags & 0x20 == 0 { data.read_bs()? } else { 0 };
            let horizontal_alignment = if flags & 0x40 == 0 { data.read_bs()? } else { 0 };
            let vertical_alignment = if flags & 0x80 == 0 { data.read_bs()? } else { 0 };
            DwgEntityBody::Text(DwgTextEntity {
                common: DwgEntityCommon::default(),
                elevation,
                insertion: insertion.to_vec(),
                alignment,
                extrusion,
                thickness,
                oblique_angle,
                rotation,
                height,
                width_factor,
                value,
                generation,
                horizontal_alignment,
                vertical_alignment,
                style_handle: 0,
            })
        }
        DWG_TYPE_SPLINE => {
            let mut scenario = data.read_bl()?;
            if frame.layout >= DwgObjectLayout::R2013 {
                let flags = data.read_bl()?;
                let knot_parameter = data.read_bl()?;
                scenario = if knot_parameter == 15 || flags & 8 == 0 { 1 } else { 2 };
            }
            let degree = data.read_bl()?;
            if scenario != 1 {
                return Ok(None);
            }
            let rational = data.read_b()?;
            let closed = data.read_b()?;
            let periodic = data.read_b()?;
            let knot_tolerance = data.read_bd()?;
            let control_tolerance = data.read_bd()?;
            let knot_count = data.read_bl()? as usize;
            let point_count = data.read_bl()? as usize;
            let weighted = data.read_b()?;
            if knot_count > 1 << 20 || point_count > 1 << 20 {
                return Err(context(format!("{knot_count} knots / {point_count} control points exceed the codec limit")));
            }
            let knots = (0..knot_count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let mut control_points = Vec::with_capacity(point_count * 3);
            let mut weights = Vec::new();
            for _ in 0..point_count {
                control_points.extend(data.read_3bd()?);
                if weighted {
                    weights.push(data.read_bd()?);
                }
            }
            DwgEntityBody::Spline(DwgSplineEntity { common: DwgEntityCommon::default(), degree, rational, closed, periodic, knot_tolerance, control_tolerance, knots, control_points, weights })
        }
        DWG_TYPE_FACE3D => {
            let has_no_flags = data.read_b()?;
            let z_is_zero = data.read_b()?;
            let mut corners = vec![data.read_rd()?, data.read_rd()?, if z_is_zero { 0.0 } else { data.read_rd()? }];
            for corner in 1..4 {
                let previous = (corner - 1) * 3;
                let x = data.read_dd(corners[previous])?;
                let y = data.read_dd(corners[previous + 1])?;
                let z = if z_is_zero { 0.0 } else { data.read_dd(corners[previous + 2])? };
                corners.extend([x, y, z]);
            }
            let invisible_edges = if has_no_flags { 0 } else { data.read_bs()? };
            DwgEntityBody::Face3d(DwgFace3dEntity { common: DwgEntityCommon::default(), corners, invisible_edges })
        }
        DWG_TYPE_LWPOLYLINE => {
            let flags = data.read_bs()?;
            let constant_width = if flags & 4 != 0 { Some(data.read_bd()?) } else { None };
            let elevation = if flags & 8 != 0 { data.read_bd()? } else { 0.0 };
            let thickness = if flags & 2 != 0 { data.read_bd()? } else { 0.0 };
            let extrusion = if flags & 1 != 0 { data.read_3bd()? } else { [0.0, 0.0, 1.0] };
            let vertex_count = data.read_bl()? as usize;
            if vertex_count == 0 || vertex_count > 1 << 20 {
                return Err(context(format!("vertex count {vertex_count} is invalid")));
            }
            let bulge_count = if flags & 16 != 0 { data.read_bl()? as usize } else { 0 };
            let vertex_id_count = if flags & 1024 != 0 { data.read_bl()? as usize } else { 0 };
            let width_count = if flags & 32 != 0 { data.read_bl()? as usize } else { 0 };
            for (name, count) in [("bulge", bulge_count), ("vertex ID", vertex_id_count), ("width", width_count)] {
                if count != 0 && count != vertex_count {
                    return Err(context(format!("{name} count {count} differs from {vertex_count} vertices")));
                }
            }
            let mut points = Vec::with_capacity(vertex_count);
            points.push(data.read_2rd()?);
            while points.len() < vertex_count {
                let previous: [f64; 2] = points[points.len() - 1];
                points.push([data.read_dd(previous[0])?, data.read_dd(previous[1])?]);
            }
            let bulges = (0..bulge_count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
            let vertex_ids = (0..vertex_id_count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
            let widths = (0..width_count).map(|_| Ok((data.read_bd()?, data.read_bd()?))).collect::<Result<Vec<_>, String>>()?;
            DwgEntityBody::LwPolyline(DwgLwPolylineEntity {
                common: DwgEntityCommon::default(),
                closed: flags & 512 != 0,
                constant_width,
                elevation,
                thickness,
                extrusion: extrusion.to_vec(),
                vertices: points
                    .into_iter()
                    .enumerate()
                    .map(|(index, point)| DwgLwPolylineVertex {
                        point: point.to_vec(),
                        bulge: bulges.get(index).copied().unwrap_or_default(),
                        vertex_id: vertex_ids.get(index).copied(),
                        start_width: widths.get(index).map(|width| width.0),
                        end_width: widths.get(index).map(|width| width.1),
                    })
                    .collect(),
            })
        }
        DWG_TYPE_POLYLINE3D => {
            let curve_type = data.read_rc()?;
            let flags = data.read_rc()?;
            owned_count = data.read_bl()? as usize;
            DwgEntityBody::Polyline3d(DwgPolyline3dEntity { common: DwgEntityCommon::default(), curve_type, flags, vertex_handles: Vec::new(), sequence_end_handle: 0 })
        }
        DWG_TYPE_POLYLINE_PFACE => {
            let vertex_count = data.read_bs()?;
            let face_count = data.read_bs()?;
            owned_count = data.read_bl()? as usize;
            DwgEntityBody::PolyfaceMesh(DwgPolyfaceMeshEntity { common: DwgEntityCommon::default(), vertex_count, face_count, vertex_handles: Vec::new(), sequence_end_handle: 0 })
        }
        DWG_TYPE_VERTEX3D | DWG_TYPE_VERTEX_PFACE => DwgEntityBody::Vertex(DwgVertexEntity { common: DwgEntityCommon::default(), flags: data.read_rc()?, point: data.read_3bd()?.to_vec() }),
        DWG_TYPE_VERTEX_PFACE_FACE => DwgEntityBody::PolyfaceFace(DwgPolyfaceFaceEntity { common: DwgEntityCommon::default(), indices: (0..4).map(|_| data.read_bs().map(|value| value as i16)).collect::<Result<_, _>>()? }),
        _ => DwgEntityBody::SequenceEnd(DwgSequenceEndEntity { common: DwgEntityCommon::default() }),
    };
    frame.finish_main(class_name)?;
    let relations = decode_r2010_entity_common_handles(&mut decoded, &mut frame.handles, handle).map_err(context)?;
    let mut body = body;
    match &mut body {
        DwgEntityBody::Text(text) => text.style_handle = read_object_handle(&mut frame.handles, handle).map_err(context)?.unwrap_or_default(),
        DwgEntityBody::Polyline3d(DwgPolyline3dEntity { vertex_handles, sequence_end_handle, .. }) | DwgEntityBody::PolyfaceMesh(DwgPolyfaceMeshEntity { vertex_handles, sequence_end_handle, .. }) => {
            if owned_count > 1 << 20 {
                return Err(context(format!("{owned_count} owned vertices exceed the codec limit")));
            }
            for index in 0..owned_count {
                vertex_handles.push(read_object_handle(&mut frame.handles, handle).map_err(context)?.ok_or_else(|| context(format!("owned vertex {index} is null")))?);
            }
            *sequence_end_handle = read_object_handle(&mut frame.handles, handle).map_err(context)?.ok_or_else(|| context("SEQEND handle is null".into()))?;
        }
        _ => {}
    }
    *body.common_mut() = decoded.logical;
    Ok(Some((body, relations)))
}
/// 🗂️ A LAYER record reduced to what the drawing projection keeps: its name and ACI colour (the
/// colour index of an `ByAci` complex colour, else the record's legacy index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_layer_projection(frame: &mut DwgObjectRecordFrame<'_>) -> Result<DwgLayer, String> {
    frame.data.read_bl()?;
    frame.data.read_b()?;
    if frame.layout >= DwgObjectLayout::R2013 {
        frame.data.read_b()?;
    }
    let name = frame.text()?;
    frame.data.read_b()?;
    frame.data.read_bs()?;
    frame.data.read_b()?;
    frame.data.read_bs()?;
    let index = frame.data.read_bs()?;
    let rgb = frame.data.read_bl()?;
    let color = if rgb >> 24 == 0xc3 { (rgb & 0xff) as u8 } else { (index & 0xff) as u8 };
    Ok(DwgLayer { name, color })
}
//#endregion DwgObjects
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dimension_attachment_logical(value: u16) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionTextAttachment, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionTextAttachment::*;
    match value {
        1 => Ok(TopCenter),
        2 => Ok(TopLeft),
        3 => Ok(TopRight),
        4 => Ok(MiddleCenter),
        5 => Ok(MiddleLeft),
        6 => Ok(MiddleRight),
        7 => Ok(BottomCenter),
        8 => Ok(BottomLeft),
        9 => Ok(BottomRight),
        _ => Err(format!("dimension text attachment {value} is invalid")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dimension_spacing_logical(value: u16) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle, String> {
    match value {
        1 => Ok(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle::AtLeast),
        2 => Ok(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle::Exact),
        _ => Err(format!("dimension line-spacing style {value} is invalid")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn viewport_status_flags(mask: u32) -> Result<Vec<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportStatusFlag>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportStatusFlag::*;
    if mask & !0x003f_ffff != 0 {
        return Err(format!("VIEWPORT status contains unsupported bits {:#x}", mask & !0x003f_ffff));
    }
    let all = [
        Perspective,
        FrontClipping,
        BackClipping,
        UcsFollow,
        FrontClipNotAtEye,
        UcsIconVisible,
        UcsIconAtOrigin,
        FastZoom,
        Snap,
        Grid,
        IsometricSnap,
        HidePlot,
        IsoPairTop,
        IsoPairRight,
        ZoomLock,
        AlwaysEnabled,
        NonRectangularClipping,
        ViewportOff,
        GridBeyondDrawingLimits,
        AdaptiveGrid,
        AdaptiveSubdivision,
        GridFollowsWorkplane,
    ];
    Ok(all.into_iter().enumerate().filter_map(|(bit, flag)| (mask & (1 << bit) != 0).then_some(flag)).collect())
}
struct DwgDecodedEntityCommon {
    pub logical: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon,
    reactor_count: usize,
    extension_dictionary_missing: bool,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_mode(value: u8) -> semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode;
    match value {
        0 => DwgEntityMode::ExplicitOwner,
        1 => DwgEntityMode::PaperSpace,
        2 => DwgEntityMode::ModelSpace,
        _ => DwgEntityMode::Reserved,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_reference_mode(value: u8) -> semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode;
    match value {
        0 => DwgEntityReferenceMode::ByLayer,
        1 => DwgEntityReferenceMode::ByBlock,
        2 => DwgEntityReferenceMode::Continuous,
        _ => DwgEntityReferenceMode::Explicit,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_entity_common_main(data: &mut DwgBitReader<'_>) -> Result<DwgDecodedEntityCommon, String> {
    if data.read_b()? {
        return Err("R2010 entity graphic requires a typed semantic graphic model".into());
    }
    decode_entity_common_fields(data, DwgObjectLayout::R2010)
}
/// 🧬️ Every common-entity field AFTER the graphic-present bit, in ODA R2010 order. The one and only
/// reading of that layout in this codec: `decode_r2010_entity_common_main` reaches it after refusing
/// a graphic it cannot model, and the drawing-geometry decoder after skipping one it does not need.
///
/// ⚠️ Two version-gated details that a reader written against the R2000 layout gets wrong, and did:
/// the `nolinks` bit is R13–R2000 ONLY and must NOT be consumed here, and R2010 adds three
/// visual-style presence bits after `shadow_flags` — each one gating a handle in the handle stream.
/// Off by those two bits, every subsequent field of a real AutoCAD entity decodes as noise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_entity_common_fields(data: &mut DwgBitReader<'_>, layout: DwgObjectLayout) -> Result<DwgDecodedEntityCommon, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColor;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColorKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon;
    let mode = data.read_bb()?;
    let reactor_count = data.read_bl()? as usize;
    let extension_dictionary_missing = data.read_b()?;
    if layout >= DwgObjectLayout::R2013 {
        data.read_b()?;
    }
    let encoded_color = data.read_bs()?;
    let transparency = if encoded_color & 0x2000 != 0 { Some(data.read_bl()?) } else { None };
    let mut rgb = 0;
    if encoded_color & 0x4000 == 0 && encoded_color & 0x8000 != 0 {
        rgb = data.read_bl()?;
    }
    let index = encoded_color & 0x01ff;
    let color = DwgEntityColor {
        kind: if encoded_color & 0xc000 != 0 {
            DwgEntityColorKind::TrueColor
        } else if index == 256 {
            DwgEntityColorKind::ByLayer
        } else if index == 0 {
            DwgEntityColorKind::ByBlock
        } else {
            DwgEntityColorKind::Index
        },
        index,
        rgb,
        transparency,
        name: None,
        book_name: None,
        color_handle: None,
    };
    let linetype_scale = data.read_bd()?;
    let linetype = entity_reference_mode(data.read_bb()?);
    let plot_style = entity_reference_mode(data.read_bb()?);
    let (material, shadow) = if layout >= DwgObjectLayout::R2007 { (entity_reference_mode(data.read_bb()?), data.read_rc()?) } else { (entity_reference_mode(0), 0) };
    let (full_visual, face_visual, edge_visual) = if layout >= DwgObjectLayout::R2010 { (data.read_b()?, data.read_b()?, data.read_b()?) } else { (false, false, false) };
    let invisible = data.read_bs()?;
    let lineweight = data.read_rc()?;
    Ok(DwgDecodedEntityCommon {
        logical: DwgEntityCommon {
            mode: entity_mode(mode),
            color,
            linetype_scale,
            linetype,
            plot_style,
            material,
            shadow,
            invisible,
            lineweight,
            full_visual_style_handle: full_visual.then_some(0),
            face_visual_style_handle: face_visual.then_some(0),
            edge_visual_style_handle: edge_visual.then_some(0),
            ..Default::default()
        },
        reactor_count,
        extension_dictionary_missing,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_entity_common_handles(decoded: &mut DwgDecodedEntityCommon, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgObjectRelations, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColorKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode;
    if decoded.logical.color.kind == DwgEntityColorKind::TrueColor && decoded.logical.color.rgb == 0 {
        decoded.logical.color.color_handle = read_object_handle(handles, base)?;
    }
    let owner = if decoded.logical.mode == DwgEntityMode::ExplicitOwner { read_object_handle(handles, base)? } else { None };
    let mut reactors = Vec::with_capacity(decoded.reactor_count);
    for index in 0..decoded.reactor_count {
        reactors.push(read_object_handle(handles, base)?.ok_or_else(|| format!("entity reactor {index} is null"))?);
    }
    let extension_dictionary = if decoded.extension_dictionary_missing { None } else { read_object_handle(handles, base)? };
    decoded.logical.layer_handle = read_object_handle(handles, base)?.ok_or("entity layer handle is null")?;
    if decoded.logical.linetype == DwgEntityReferenceMode::Explicit {
        decoded.logical.linetype_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.material == DwgEntityReferenceMode::Explicit {
        decoded.logical.material_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.shadow == 3 {
        decoded.logical.shadow_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.plot_style == DwgEntityReferenceMode::Explicit {
        decoded.logical.plot_style_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.full_visual_style_handle.is_some() {
        decoded.logical.full_visual_style_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.face_visual_style_handle.is_some() {
        decoded.logical.face_visual_style_handle = read_object_handle(handles, base)?;
    }
    if decoded.logical.edge_visual_style_handle.is_some() {
        decoded.logical.edge_visual_style_handle = read_object_handle(handles, base)?;
    }
    Ok((owner, reactors, extension_dictionary))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_entity_terminal_fill(reader: &mut DwgBitReader<'_>, end_bit: usize, handle: u64, class_name: &str) -> Result<(), String> {
    let terminal_bits = end_bit.checked_sub(reader.bit_position()).ok_or_else(|| format!("{class_name} {handle:#x} handle stream exceeds its frame"))?;
    if terminal_bits > 7 {
        return Err(format!("{class_name} {handle:#x} has {terminal_bits} trailing handle bits"));
    }
    for _ in 0..terminal_bits {
        if !reader.read_b()? {
            return Err(format!("{class_name} {handle:#x} terminal handle fill contains zero"));
        }
    }
    Ok(())
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
