//! 📚️ Document model: the scale, the objects, vortices, attractions, target volumes, references, the kind catalogs and the document meta of the puzzle 3d artifact.

use super::*;

/// 📐️ A placed object's / target volume's freeform pose scale: either a single scalar broadcast
/// to all three axes, or an explicit per-axis `[x, y, z]` triple — the ONLY two shapes the engine
/// crate's `vec3_scale`/`volume_scale_vec` ever interpret (see that crate's implementation and its
/// `vec3_scale_variants` test), so this is a small closed union rather than genuinely heterogeneous
/// JSON. Replaces the former `serde_json::Value` passthrough with the actual shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle3dScale {
    Uniform(f64),
    Vec3([f64; 3]),
}

semio_framework_value::artifact_retire_leaf!(Puzzle3dScale);

impl semio_framework_value::retained_clone::RetainedClone for Puzzle3dScale {
    type Cursor = semio_framework_value::retained_clone::ScalarCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor {
        Default::default()
    }
}

/// 🌲️ The canonical tree keeps the wire shape: a bare number for `Uniform`, an `[x, y, z]` array for `Vec3`.
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Puzzle3dScale {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        Ok(match self {
            Self::Uniform(value) => semio_framework_pack_json::ArtifactCanonicalJsonNode::F64(*value),
            Self::Vec3(_) => semio_framework_pack_json::ArtifactCanonicalJsonNode::Array(3),
        })
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, semio_framework_value::ValueError> {
        match self {
            Self::Vec3(axes) => axes.get(ordinal).map(|axis| axis as &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical scale axis ordinal is absent")),
            Self::Uniform(_) => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical uniform scale has no child")),
        }
    }
}

/// 🔗️ Wire shape stays identical to the former `serde_json::Value` passthrough (a bare number or
/// an `[x, y, z]` array) so every JSON-boundary consumer (the engine/ui wasm crates' own mirror
/// structs, which bind `scale` as `Option<serde_json::Value>` and are out of this derive's scope)
/// keeps parsing it exactly as before.
#[cfg(any(test, feature = "testing"))]
impl serde::Serialize for Puzzle3dScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Puzzle3dScale::Uniform(scale) => serializer.serialize_f64(*scale),
            Puzzle3dScale::Vec3(vec3) => serde::Serialize::serialize(vec3, serializer),
        }
    }
}

#[cfg(any(test, feature = "testing"))]
impl<'de> serde::Deserialize<'de> for Puzzle3dScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match <serde_json::Value as serde::Deserialize>::deserialize(deserializer)? {
            serde_json::Value::Number(n) => Ok(Puzzle3dScale::Uniform(n.as_f64().unwrap_or(1.0))),
            serde_json::Value::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle3dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => Err(serde::de::Error::custom(format!("expected scale to be a number or an [x, y, z] array, found {other}"))),
        }
    }
}

/// 🔁️ Hand-written (mirrors the `Serialize`/`Deserialize` pair directly above): same bare-number-or-
/// `[x, y, z]`-array wire shape.
impl semio_framework_value::ToValue for Puzzle3dScale {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            Puzzle3dScale::Uniform(scale) => semio_framework_value::ToValue::to_value(scale),
            Puzzle3dScale::Vec3(vec3) => semio_framework_value::ToValue::to_value(vec3),
        }
    }
    fn to_value_controlled(&self,c:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{match self{Self::Uniform(v)=><f64 as semio_framework_value::ToValue>::to_value_controlled(v,c),Self::Vec3(v)=><[f64;3] as semio_framework_value::ToValue>::to_value_controlled(v,c)}}

}

impl semio_framework_value::FromValue for Puzzle3dScale {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle3dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => f64::from_value(other).map(Puzzle3dScale::Uniform),
        }
    }
    fn from_value_controlled(value:&semio_framework_value::DslValue,c:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{c.charge(size_of::<Self>())?;match value{semio_framework_value::DslValue::Array(_)=><[f64;3] as semio_framework_value::FromValue>::from_value_controlled(value,c).map(Self::Vec3),_=><f64 as semio_framework_value::FromValue>::from_value_controlled(value,c).map(Self::Uniform)}}

}

/// 🔗️ Hand `DslField` bridge for `Puzzle3dScale`: `objects`/`targetVolumes` are `#[dsl(table)]`
/// collections, so this field prints as a BARE positional table column — the unbounded
/// `Shape::Tuple(Float, None)` the sibling `puzzle_5d::Puzzle5dScale` uses (its `scale` is reached
/// through a nested keyed record field, not a bare column) is rejected there at parse time
/// (`table column 'scale' has a non-self-delimiting shape (TUPLE) and cannot be a table column`,
/// per the engine's own `validate_table_columns`), so this binds through the bracketed
/// `Shape::List(Float)` instead: `scale=[2]` (uniform) / `scale=[2 3 4]` (per-axis) — the brackets
/// make it self-delimiting regardless of item count.
impl semio_framework_dsl_record::BorrowedDslField for Puzzle3dScale {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = <Vec<f64> as semio_framework_dsl_record::BorrowedDslField>::SHAPE;
}

impl semio_framework_dsl_record::DslField for Puzzle3dScale {
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> {
        <Vec<f64> as semio_framework_dsl_record::DslField>::shape_controlled(control)
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> {
        let axes = match self { Self::Uniform(v) => std::slice::from_ref(v), Self::Vec3(v) => v.as_slice() };
        control.scoped_stage(|control| {
            control.begin_stage(axes.len())?;
            let mut values = control.allocate_vec::<semio_framework_dsl_record::FieldValue>(axes.len())?;
            for value in axes { values.push(<f64 as semio_framework_dsl_record::DslField>::to_value_controlled(value, control)?); }
            Ok(semio_framework_dsl_record::FieldValue::List(values))
        })
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint()?;
        let semio_framework_dsl_record::FieldValue::List(items) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Puzzle3d Scale requires one or three scalars")) };
        if items.len() != 1 && items.len() != 3 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Puzzle3d Scale requires one or three scalars")) }
        control.scoped_stage(|control| {
            control.begin_stage(items.len())?;
            control.charge(size_of::<Self>())?;
            let x = <f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[0], control)?;
            if items.len() == 1 { return Ok(Self::Uniform(x)) }
            let y = <f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[1], control)?;
            let z = <f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[2], control)?;
            Ok(Self::Vec3([x,y,z]))
        })
    }

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Float))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        match self {
            Puzzle3dScale::Uniform(scale) => semio_framework_dsl_record::FieldValue::List(vec![semio_framework_dsl_record::FieldValue::Float(*scale)]),
            Puzzle3dScale::Vec3(vec3) => semio_framework_dsl_record::FieldValue::List(vec3.iter().map(|axis| semio_framework_dsl_record::FieldValue::Float(*axis)).collect()),
        }
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::List(items) if items.len() == 1 => match &items[0] {
                semio_framework_dsl_record::FieldValue::Float(scale) => Ok(Puzzle3dScale::Uniform(*scale)),
                other => Err(format!("expected Float, found {other:?}")),
            },
            semio_framework_dsl_record::FieldValue::List(items) if items.len() >= 3 => {
                let axis = |i: usize| match &items[i] {
                    semio_framework_dsl_record::FieldValue::Float(v) => Ok(*v),
                    other => Err(format!("expected Float, found {other:?}")),
                };
                Ok(Puzzle3dScale::Vec3([axis(0)?, axis(1)?, axis(2)?]))
            }
            other => Err(format!("expected a 1- or 3-item List, found {other:?}")),
        }
    }
}

// #region 🔖️Document
/// ⚓️ Whether a root object keeps its stored plane (`Fixed`) or resets to default XY (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Puzzle3dObjectAnchor {
    #[default]
    Fixed,
    Derived,
}

/// 🔘️ One vortex on an object's rim — `vortex_kind` gates attraction compatibility, `position`/
/// `direction` place and orient it, `radius` sizes its brush-fill collision.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dVortex {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub direction: Option<[f64; 3]>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🧱️ One placed object — `origin`/`orientation`/`scale` (a scalar-or-`[x,y,z]` `Puzzle3dScale`,
/// see that type and `vec3_scale`) pose it, `anchor` gates flatten-root plane retention, `vortices`
/// are its rim attraction ports.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dObject {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "object_kind")]
    pub object_kind: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub anchor: Puzzle3dObjectAnchor,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🔗️ One attraction between two full vortex ids (`object_id:vortex_id`), with the eight compose
/// connection parameters (`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` plus diagram `x`/`y`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttraction {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub gap: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub shift: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub rise: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub rotation: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub turn: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub tilt: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub y: f64,
}

/// 🧊️ A persisted oriented box constraining fill placement (Volume Brush voxels or Transform-gumball
/// edited volumes). `scale` is a scalar-or-`[x,y,z]` `Puzzle3dScale` — see that type and
/// `volume_scale_vec`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🌐️ Where a reference image/media's bytes live and what kind of media it is.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_kind: Option<String>,
}

/// 🖼️ A reference plane pinned in world space at `origin`, `width_world` meters wide.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReference {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub source: Puzzle3dReferenceSource,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(unit = "m")]
    pub width_world: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub locked: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub hidden: bool,
}

/// 🔗️ How specifically two vortex/cable kinds are allowed to attract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "lowercase"))]
#[value(rename_all = "lowercase")]
pub enum Puzzle3dCompatSpecificity {
    General,
    Object,
    Attraction,
    Cable,
    #[default]
    Vortex,
}

/// 🧩️ One allowed (or, unidirectional, one-way-allowed) link pair between two vortex/cable kind ids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibility {
    #[dsl(refs = "vortex_kind")]
    pub source: String,
    #[dsl(refs = "vortex_kind")]
    pub target: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub bidirectional: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub important: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub specificity: Puzzle3dCompatSpecificity,
}

/// 🏷️ One freeform attribute on a catalog object-kind (compose `Attribute` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttribute {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub key: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub value: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ One author credit on a catalog object-kind (compose `Author` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAuthor {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub email: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ One tagged representation/LOD URL on a catalog object-kind (compose `Representation` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dRepresentation {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub mime: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub tags: Vec<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub description: String,
}

/// 🌱️ One rim-vortex template on a `Puzzle3dCatalogObjectKind` — compose connector analogue with
/// `point`/`direction`/`t`/`mandatory`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexTemplate {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub point: [f64; 3],
    #[cfg_attr(any(test, feature = "testing"), serde(default = "puzzle3d_default_direction"))]
    #[value(default = "puzzle3d_default_direction")]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

fn puzzle3d_default_direction() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

impl Default for Puzzle3dCatalogVortexTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), vortex_kind: None, point: [0.0, 0.0, 0.0], direction: puzzle3d_default_direction(), t: None, mandatory: None, radius: None }
    }
}

/// 🧱️ One object-kind catalog row — type-like (compose `Type` analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogObjectKind {
    #[dsl(defines = "object_kind")]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub image: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub unit: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, rename = "abstract"))]
    #[value(default, rename = "abstract")]
    pub is_abstract: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub base_kinds: Vec<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub representations: Vec<Puzzle3dRepresentation>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub vortices: Vec<Puzzle3dCatalogVortexTemplate>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub attributes: Vec<Puzzle3dAttribute>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub authors: Vec<Puzzle3dAuthor>,
}

/// 🔘️ One vortex-kind catalog row — port-like (compose `Port` analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexKind {
    #[dsl(defines = "vortex_kind")]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub compatible_with: Vec<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub color: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(refs = "cable_kind")]
    pub default_cable_kind: String,
}

/// 🧵️ One cable-kind catalog row (mirrors `CableKindCatalog`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogCableKind {
    #[dsl(defines = "cable_kind")]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(refs = "attraction_kind")]
    pub default_attraction_kind: String,
}

/// 🔗️ One attraction-kind catalog row.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogAttractionKind {
    #[dsl(defines = "attraction_kind")]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained scene_snapshot export: object/vortex/cable/
/// attraction kind rows — see `puzzle/3d/manifest/*.manifest.json` for the same schema at the
/// manifest layer.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCatalogs {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub objects: Vec<Puzzle3dCatalogObjectKind>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub vortices: Vec<Puzzle3dCatalogVortexKind>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub cables: Vec<Puzzle3dCatalogCableKind>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub attractions: Vec<Puzzle3dCatalogAttractionKind>,
}

/// 🗂️ EngineSceneSnapshot-carried metadata: the explicit link-compatibility table plus the object/vortex/cable/
/// attraction kind catalog bundle (typed — see `Puzzle3dKindCatalogs`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dMeta {
    #[cfg_attr(any(test, feature = "testing"), serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle3dKindCatalogs>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle3dKindCompatibility>,
}
// #endregion 🔖️Document
