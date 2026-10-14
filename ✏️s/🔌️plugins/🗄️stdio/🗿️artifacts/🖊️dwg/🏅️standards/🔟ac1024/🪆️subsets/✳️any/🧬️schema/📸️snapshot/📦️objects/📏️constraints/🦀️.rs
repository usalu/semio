//! 📏️ Geometric and dimensional constraints of 2D constraint groups.

use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::{DwgAssociativeAction};

//#region 🔖️DrawingModel
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgConstraintNodeCore {
    pub id: i32,
    pub connected_node_ids: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgGeometricConstraint {
    pub node: DwgConstraintNodeCore,
    pub owner_node_id: u32,
    pub implied: bool,
    pub active: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgConstraintGeometry {
    pub node: DwgConstraintNodeCore,
    pub geometry_dependency_handle: Option<u64>,
    pub geometry_node_id: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgExplicitConstraint {
    pub geometric: DwgGeometricConstraint,
    pub value_dependency_handle: u64,
    pub dimension_dependency_handle: u64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgConstrainedImplicitPoint {
    pub geometry: DwgConstraintGeometry,
    pub point: Option<Vec<f64>>,
    pub point_kind: u8,
    pub point_index: i32,
    pub curve_node_id: i32,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgConstrainedBoundedLine {
    pub geometry: DwgConstraintGeometry,
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
    pub ray: bool,
    pub bounded: bool,
    pub start_point: Vec<f64>,
    pub end_point: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDistanceConstraint {
    pub explicit: DwgExplicitConstraint,
    pub direction_kind: u8,
    pub direction: Option<Vec<f64>>,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAxisConstraint {
    pub geometric: DwgGeometricConstraint,
    pub datum_line_index: i32,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgConstrainedDatumLine {
    pub geometry: DwgConstraintGeometry,
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgConstraintNode {
    ConstrainedImplicitPoint(DwgConstrainedImplicitPoint),
    PointCurveConstraint(DwgGeometricConstraint),
    ConstrainedBoundedLine(DwgConstrainedBoundedLine),
    PointCoincidenceConstraint(DwgGeometricConstraint),
    DistanceConstraint(DwgDistanceConstraint),
    PerpendicularConstraint(DwgGeometricConstraint),
    HorizontalConstraint(DwgAxisConstraint),
    ParallelConstraint(DwgGeometricConstraint),
    MidPointConstraint(DwgGeometricConstraint),
    EqualLengthConstraint(DwgGeometricConstraint),
    ColinearConstraint(DwgGeometricConstraint),
    ConstrainedDatumLine(DwgConstrainedDatumLine),
    FixedConstraint(DwgGeometricConstraint),
    VerticalConstraint(DwgAxisConstraint),
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
dwg_metadata!(dwg_constraint_node_spec, dwg_constraint_node_spec_controlled, dwg_constraint_node_spec_producer => DwgConstraintNode;
    0, "kind", false, [enum ("constrainedImplicitPoint", 0), ("pointCurveConstraint", 1), ("constrainedBoundedLine", 2), ("pointCoincidenceConstraint", 3), ("distanceConstraint", 4), ("perpendicularConstraint", 5), ("horizontalConstraint", 6), ("parallelConstraint", 7), ("midPointConstraint", 8), ("equalLengthConstraint", 9), ("colinearConstraint", 10), ("constrainedDatumLine", 11), ("fixedConstraint", 12), ("verticalConstraint", 13)];
    1, "constrained_implicit_point", true, [typed DwgConstrainedImplicitPoint];
    2, "geometric_constraint", true, [typed DwgGeometricConstraint];
    3, "constrained_bounded_line", true, [typed DwgConstrainedBoundedLine];
    4, "distance_constraint", true, [typed DwgDistanceConstraint];
    5, "axis_constraint", true, [typed DwgAxisConstraint];
    6, "constrained_datum_line", true, [typed DwgConstrainedDatumLine];
);
impl semio_framework_dsl_record::DslField for DwgConstraintNode {
    dwg_controlled_payloads!(0; units[];
        0,1,ConstrainedImplicitPoint,DwgConstrainedImplicitPoint;
        1,2,PointCurveConstraint,DwgGeometricConstraint;
        2,3,ConstrainedBoundedLine,DwgConstrainedBoundedLine;
        3,2,PointCoincidenceConstraint,DwgGeometricConstraint;
        4,4,DistanceConstraint,DwgDistanceConstraint;
        5,2,PerpendicularConstraint,DwgGeometricConstraint;
        6,5,HorizontalConstraint,DwgAxisConstraint;
        7,2,ParallelConstraint,DwgGeometricConstraint;
        8,2,MidPointConstraint,DwgGeometricConstraint;
        9,2,EqualLengthConstraint,DwgGeometricConstraint;
        10,2,ColinearConstraint,DwgGeometricConstraint;
        11,6,ConstrainedDatumLine,DwgConstrainedDatumLine;
        12,2,FixedConstraint,DwgGeometricConstraint;
        13,5,VerticalConstraint,DwgAxisConstraint;);

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_constraint_node_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_constraint_node_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        let (kind, field, value) = match self {
            Self::ConstrainedImplicitPoint(value) => (0, 1, value.to_value()),
            Self::PointCurveConstraint(value) => (1, 2, value.to_value()),
            Self::ConstrainedBoundedLine(value) => (2, 3, value.to_value()),
            Self::PointCoincidenceConstraint(value) => (3, 2, value.to_value()),
            Self::DistanceConstraint(value) => (4, 4, value.to_value()),
            Self::PerpendicularConstraint(value) => (5, 2, value.to_value()),
            Self::HorizontalConstraint(value) => (6, 5, value.to_value()),
            Self::ParallelConstraint(value) => (7, 2, value.to_value()),
            Self::MidPointConstraint(value) => (8, 2, value.to_value()),
            Self::EqualLengthConstraint(value) => (9, 2, value.to_value()),
            Self::ColinearConstraint(value) => (10, 2, value.to_value()),
            Self::ConstrainedDatumLine(value) => (11, 6, value.to_value()),
            Self::FixedConstraint(value) => (12, 2, value.to_value()),
            Self::VerticalConstraint(value) => (13, 5, value.to_value()),
        };
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(kind));
        record.fields.insert(field, value);
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected constraint-node record".into()) };
        if record.fields.values().filter(|value| !matches!(value, semio_framework_dsl_record::FieldValue::Absent)).count() != 2 {
            return Err("constraint node must contain exactly its tagged body".into());
        }
        let geometric = || DwgGeometricConstraint::from_value(record.get(2).ok_or("geometric constraint missing")?);
        match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) => Ok(Self::ConstrainedImplicitPoint(DwgConstrainedImplicitPoint::from_value(record.get(1).ok_or("implicit point missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(1)) => Ok(Self::PointCurveConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(2)) => Ok(Self::ConstrainedBoundedLine(DwgConstrainedBoundedLine::from_value(record.get(3).ok_or("bounded line missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(3)) => Ok(Self::PointCoincidenceConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(4)) => Ok(Self::DistanceConstraint(DwgDistanceConstraint::from_value(record.get(4).ok_or("distance constraint missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(5)) => Ok(Self::PerpendicularConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(6)) => Ok(Self::HorizontalConstraint(DwgAxisConstraint::from_value(record.get(5).ok_or("horizontal constraint missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(7)) => Ok(Self::ParallelConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(8)) => Ok(Self::MidPointConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(9)) => Ok(Self::EqualLengthConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(10)) => Ok(Self::ColinearConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(11)) => Ok(Self::ConstrainedDatumLine(DwgConstrainedDatumLine::from_value(record.get(6).ok_or("datum line missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(12)) => Ok(Self::FixedConstraint(geometric()?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(13)) => Ok(Self::VerticalConstraint(DwgAxisConstraint::from_value(record.get(5).ok_or("vertical constraint missing")?)?)),
            other => Err(format!("unknown constraint-node kind {other:?}")),
        }
    }
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssoc2dConstraintGroup {
    pub action: DwgAssociativeAction,
    pub do_not_check_newly_added_constraints: bool,
    pub work_plane: Vec<Vec<f64>>,
    pub member_action_handles: Vec<u64>,
    pub nodes: Vec<DwgConstraintNode>,
}
//#endregion 🔖️DrawingModel
