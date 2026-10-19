//! 🧱️ Document records of the puzzle 5d artifact: the snapshot meta, the unified 2d and 3d part, grip and fastener records, the target volumes, the kind compatibility and the catalog kind rows.


// #region 🔖️Document
/// 📝️ Free-text scene description — the only field seen under the fixture's top-level `meta`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dMeta {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
}

/// 🔵️ A part's 2D-projection presentation (board node): `shape`/`radius` size the circle/rectangle,
/// `text`/`icon_kind` label it.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart2d {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub y: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

//#region 📐️Scale
/// 📏️ A part's freeform 3D scale — a bare number scales all three axes uniformly, an `[x, y, z]`
/// array scales each axis independently. Same `Uniform`-or-`Vec3` shape as
/// `puzzle_3d::Puzzle3dObject.scale`/`Puzzle3dTargetVolume.scale` (see that type's own `Scale`
/// region), applied here to the identical `part_scale_json` decode `puzzle_5d_ui` already used to
/// read the old raw `serde_json::Value` two ways.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle5dScale {
    Uniform(f64),
    Vec3([f64; 3]),
}

semio_framework_value::artifact_retire_leaf!(Puzzle5dScale);

impl semio_framework_value::retained_clone::RetainedClone for Puzzle5dScale {
    type Cursor = semio_framework_value::retained_clone::ScalarCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor {
        Default::default()
    }
}

/// 🌲️ The canonical tree keeps the wire shape: a bare number for `Uniform`, an `[x, y, z]` array for `Vec3`.
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Puzzle5dScale {
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
/// an `[x, y, z]` array) so every JSON-boundary consumer (the `puzzle_5d_ui` wasm crate's own
/// mirror struct, which binds `scale` as `Option<serde_json::Value>` and is out of this derive's
/// scope) keeps parsing it exactly as before.
#[cfg(test)]
impl serde::Serialize for Puzzle5dScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Puzzle5dScale::Uniform(scale) => serializer.serialize_f64(*scale),
            Puzzle5dScale::Vec3(vec3) => serde::Serialize::serialize(vec3, serializer),
        }
    }
}

#[cfg(test)]
impl<'de> serde::Deserialize<'de> for Puzzle5dScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match <serde_json::Value as serde::Deserialize>::deserialize(deserializer)? {
            serde_json::Value::Number(n) => Ok(Puzzle5dScale::Uniform(n.as_f64().unwrap_or(1.0))),
            serde_json::Value::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle5dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => Err(serde::de::Error::custom(format!("expected scale to be a number or an [x, y, z] array, found {other}"))),
        }
    }
}

/// 🔁️ Hand-written (mirrors the `Serialize`/`Deserialize` pair directly above): same bare-number-or-
/// `[x, y, z]`-array wire shape.
impl semio_framework_value::ToValue for Puzzle5dScale {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            Puzzle5dScale::Uniform(scale) => semio_framework_value::ToValue::to_value(scale),
            Puzzle5dScale::Vec3(vec3) => semio_framework_value::ToValue::to_value(vec3),
        }
    }
}

impl semio_framework_value::FromValue for Puzzle5dScale {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::Array(items) => {
                let axes = <[f64; 3] as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::Array(items))?;
                Ok(Puzzle5dScale::Vec3(axes))
            }
            other => f64::from_value(other).map(Puzzle5dScale::Uniform),
        }
    }
}

/// 🔗️ Hand `DslField` bridge for `Puzzle5dScale`: `target_volumes` is a `#[dsl(table)]` collection, so
/// this field prints as a BARE positional table column. The unbounded `Shape::Tuple(Float, None)` it
/// used while `scale` was only ever reached through the nested keyed `part-3d` record is rejected there
/// at parse time (`table column 'scale' has a non-self-delimiting shape (TUPLE) and cannot be a table
/// column`, the engine's own `validate_table_columns`), so it binds through the bracketed
/// `Shape::List(Float)` the sibling `puzzle_3d::Puzzle3dScale` already uses: `scale=[2]` (uniform) /
/// `scale=[2 3 4]` (per-axis), self-delimiting regardless of item count.
impl semio_framework_dsl_record::BorrowedDslField for Puzzle5dScale {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = <Vec<f64> as semio_framework_dsl_record::BorrowedDslField>::SHAPE;
}

impl semio_framework_dsl_record::DslField for Puzzle5dScale {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Float))
    }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{
        <Vec<f64> as semio_framework_dsl_record::DslField>::shape_controlled(control)
    }
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{
        let axes=match self{Self::Uniform(value)=>std::slice::from_ref(value),Self::Vec3(axes)=>axes.as_slice()};
        control.scoped_stage(|control|{
            control.begin_stage(axes.len())?;
            let mut values=control.allocate_vec::<semio_framework_dsl_record::FieldValue>(axes.len())?;
            for axis in axes{values.push(<f64 as semio_framework_dsl_record::DslField>::to_value_controlled(axis,control)?);}
            Ok(semio_framework_dsl_record::FieldValue::List(values))
        })
    }
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        control.checkpoint()?;
        let semio_framework_dsl_record::FieldValue::List(items)=value else{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"expected a 1- or 3-item List"))};
        if items.len()!=1&&items.len()!=3{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"expected a 1- or 3-item List"))}
        control.scoped_stage(|control|{
            control.begin_stage(items.len())?;
            control.charge(size_of::<Self>())?;
            let x=<f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[0],control)?;
            if items.len()==1{return Ok(Self::Uniform(x))}
            let y=<f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[1],control)?;
            let z=<f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[2],control)?;
            Ok(Self::Vec3([x,y,z]))
        })
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        match self {
            Puzzle5dScale::Uniform(scale) => semio_framework_dsl_record::FieldValue::List(vec![semio_framework_dsl_record::FieldValue::Float(*scale)]),
            Puzzle5dScale::Vec3(vec3) => semio_framework_dsl_record::FieldValue::List(vec3.iter().map(|axis| semio_framework_dsl_record::FieldValue::Float(*axis)).collect()),
        }
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::List(items) if items.len() == 1 => match &items[0] {
                semio_framework_dsl_record::FieldValue::Float(scale) => Ok(Puzzle5dScale::Uniform(*scale)),
                other => Err(format!("expected Float, found {other:?}")),
            },
            semio_framework_dsl_record::FieldValue::List(items) if items.len() == 3 => {
                let axis = |i: usize| match &items[i] {
                    semio_framework_dsl_record::FieldValue::Float(v) => Ok(*v),
                    other => Err(format!("expected Float, found {other:?}")),
                };
                Ok(Puzzle5dScale::Vec3([axis(0)?, axis(1)?, axis(2)?]))
            }
            other => Err(format!("expected a 1- or 3-item List, found {other:?}")),
        }
    }
}
//#endregion 📐️Scale

/// 🧱️ A part's 3D-projection presentation (world object): `origin`/`orientation` pose it, `mesh_url`
/// resolves its geometry, `scale` is `Puzzle5dScale` (bare number = uniform, `x,y,z` = per-axis).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart3d {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle5dScale>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🔘️ A grip's 2D-projection presentation (board handle) — `grip_kind` is duplicated here from
/// `Puzzle5dGrip::grip_kind` in real fixtures (a per-projection override slot), not simplified away.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip2d {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(angle = "rad")]
    pub angle: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

/// 🔘️ A grip's 3D-projection presentation (world vortex).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip3d {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub direction: Option<[f64; 3]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🔘️ One rim grip on a part, unified across both projections.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGrip {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[cfg_attr(test, serde(default, rename = "2d"))]
    #[value(default, rename = "2d")]
    pub grip_2d: Puzzle5dGrip2d,
    #[cfg_attr(test, serde(default, rename = "3d"))]
    #[value(default, rename = "3d")]
    pub grip_3d: Puzzle5dGrip3d,
}

/// ⚓️ Whether a part keeps its stored plane at a BFS root (`Fixed`) or resets the plane to default XY (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Puzzle5dPartAnchor {
    #[default]
    Fixed,
    Derived,
}

impl Puzzle5dPartAnchor {
    fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed)
    }
}

/// 🧱️ One placed part, unified across both projections — `grips` are its rim attraction/link ports.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "part_kind")]
    pub part_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Puzzle5dPartAnchor::is_fixed"))]
    #[value(default, skip_serializing_if = "Puzzle5dPartAnchor::is_fixed")]
    pub anchor: Puzzle5dPartAnchor,
    #[cfg_attr(test, serde(default, rename = "2d"))]
    #[value(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[cfg_attr(test, serde(default, rename = "3d"))]
    #[value(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub grips: Vec<Puzzle5dGrip>,
}

impl Default for Puzzle5dPart {
    fn default() -> Self {
        Self { id: String::new(), part_kind: None, anchor: Puzzle5dPartAnchor::Fixed, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() }
    }
}

/// 🔗️ One fastener (2D edge / 3D attraction) between two full grip ids (`part_id:grip_id`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dFastener {
    pub id: String,
    pub source: String,
    pub target: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "fastener_kind")]
    pub fastener_kind: Option<String>,
    /// 🔧️ Pose-solver offsets `compute_brush_placement_pose` / compose `geom::flatten` resolve into a world pose —
    /// mirrors `puzzle_3d::Puzzle3dAttraction` plus diagram offsets `x`/`y` (compose Connection `u`/`v`).
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub gap: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub shift: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rise: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rotation: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub turn: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tilt: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub y: f64,
}

/// 🧊️ A persisted oriented box constraining fill placement in the 3D projection (Volume Brush voxels
/// or Transform-gumball edited volumes). Same shape as `puzzle_3d::Puzzle3dTargetVolume` — the 5d fill
/// planner IS puzzle 3d's, so a 5d volume is handed to it unchanged; the board projection is derived,
/// never persisted (`💡️inferences/🎛️flat-position`'s `target_volume_flat_rect`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dTargetVolume {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle5dScale>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

impl Default for Puzzle5dTargetVolume {
    fn default() -> Self {
        Self { id: String::new(), origin: [0.0; 3], orientation: None, scale: None, hidden: false, locked: false }
    }
}

/// 🔗️ How specifically two grip/rope kinds are allowed to fasten.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "lowercase"))]
#[value(rename_all = "lowercase")]
pub enum Puzzle5dCompatSpecificity {
    #[default]
    General,
    Part,
    Fastener,
    Grip,
    Rope,
}

/// 🔗️ One allowed (or unidirectional) kind pair — unified with 2d/3d via `important` + `specificity`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCompatibility {
    #[dsl(refs = "grip_kind")]
    pub source: String,
    #[dsl(refs = "grip_kind")]
    pub target: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub bidirectional: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub important: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub specificity: Puzzle5dCompatSpecificity,
}

/// 🏷️ One freeform attribute on a part-kind (compose `Attribute` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dAttribute {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub key: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub value: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ One author credit on a part-kind (compose `Author` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dAuthor {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub email: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ One tagged representation (mesh/image/…) on a part-kind.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dRepresentation {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub mime: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tags: Vec<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
}

fn default_grip_direction() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

/// 🌱️ One rim-grip template on a `Puzzle5dCatalogPartKind` (compose Connector analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGripTemplate {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "grip_kind")]
    pub grip_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub point: [f64; 3],
    #[cfg_attr(test, serde(default = "default_grip_direction"))]
    #[value(default = "default_grip_direction")]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

impl Default for Puzzle5dGripTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), grip_kind: None, point: [0.0, 0.0, 0.0], direction: default_grip_direction(), t: None, mandatory: None, radius: None }
    }
}

/// 🧱️ One part-kind catalog row (compose Type analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogPartKind {
    #[dsl(defines = "part_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub image: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub unit: String,
    #[cfg_attr(test, serde(default, rename = "abstract"))]
    #[value(default, rename = "abstract")]
    pub is_abstract: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub base_kinds: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub representations: Vec<Puzzle5dRepresentation>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub grips: Vec<Puzzle5dGripTemplate>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attributes: Vec<Puzzle5dAttribute>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub authors: Vec<Puzzle5dAuthor>,
}

/// 🔘️ One grip-kind catalog row (compose Port analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogGripKind {
    #[dsl(defines = "grip_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub compatible_with: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub color: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "rope_kind")]
    pub default_rope_kind: String,
}

/// 🔗️ One fastener-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogFastenerKind {
    #[dsl(defines = "fastener_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🧵️ One rope-kind catalog row.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCatalogRopeKind {
    #[dsl(defines = "rope_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "fastener_kind")]
    pub default_fastener_kind: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained fixture export: part/grip/fastener/rope
/// kind rows — see `puzzle/5d/manifest/*.manifest.json` for the same schema at the manifest layer.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCatalogs {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub parts: Vec<Puzzle5dCatalogPartKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub grips: Vec<Puzzle5dCatalogGripKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub fasteners: Vec<Puzzle5dCatalogFastenerKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub ropes: Vec<Puzzle5dCatalogRopeKind>,
}

/// 🏷️ Temporary name alias until Wave 3 app callers migrate to [`Puzzle5dCatalogPartKind`].
pub type Puzzle5dCatalogPart = Puzzle5dCatalogPartKind;

/// 🏷️ Temporary name alias until Wave 3 app callers migrate to [`Puzzle5dCatalogGripKind`].
pub type Puzzle5dCatalogGrip = Puzzle5dCatalogGripKind;
