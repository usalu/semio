//! 🧬️ Neutral owned mesh coordinates, handles, polygon document and source provenance.
extern crate semio_framework_value_derive as value_derive;
#[cfg(test)]use serde::{Serialize,Deserialize};
use std::cmp::Ordering;
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — newtype tuple struct, `#[value(transparent)]` forwards straight to the `[f32; 3]`
// array's own (blanket) `ToValue`/`FromValue`, matching serde's own newtype-struct default.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value", transparent)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    pub const ZERO: Self = Self([0.0, 0.0, 0.0]);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(self) -> f32 {
        self.0[0]
    }
    pub fn y(self) -> f32 {
        self.0[1]
    }
    pub fn z(self) -> f32 {
        self.0[2]
    }

    #[allow(clippy::should_implement_trait, reason = "renaming ripples through lowpoly/core, lowpoly/plugin, remodel/plugin (outside this crate); add/sub read better than +/- across this file's dense vector algebra")]
    pub fn add(self, o: Self) -> Self {
        Self([self.x() + o.x(), self.y() + o.y(), self.z() + o.z()])
    }

    #[allow(clippy::should_implement_trait, reason = "renaming ripples through lowpoly/core, lowpoly/plugin, remodel/plugin (outside this crate); add/sub read better than +/- across this file's dense vector algebra")]
    pub fn sub(self, o: Self) -> Self {
        Self([self.x() - o.x(), self.y() - o.y(), self.z() - o.z()])
    }

    pub fn scale(self, s: f32) -> Self { Self(self.0.map(|value|value*s)) }

    pub fn dot(self, o: Self) -> f32 {
        self.x() * o.x() + self.y() * o.y() + self.z() * o.z()
    }

    pub fn cross(self, o: Self) -> Self {
        Self([self.y() * o.z() - self.z() * o.y(), self.z() * o.x() - self.x() * o.z(), self.x() * o.y() - self.y() * o.x()])
    }

    pub fn length(self) -> f32 {
        self.x().hypot(self.y()).hypot(self.z())
    }

    pub fn normalize(self) -> Self {
        let l = (self.x() as f64).hypot(self.y() as f64).hypot(self.z() as f64);
        if l == 0.0 {
            return Self::ZERO;
        }
        Self(self.0.map(|coordinate| (coordinate as f64 / l) as f32))
    }

    pub fn lerp(self, o: Self, t: f32) -> Self {
        self.add(o.sub(self).scale(t))
    }
}

// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `Vec3` above for the `transparent` newtype pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value", transparent)]
pub struct VertexId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value", transparent)]
pub struct HalfEdgeId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value", transparent)]
pub struct FaceId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::semio_framework_value", transparent)]
pub struct EdgeId(pub u32);

// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub enum WeldMode {
    Center,
    First,
    ByDistance,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub enum MirrorAxis {
    X,
    Y,
    Z,
}

//#region ⚠️ Errors
/// ⚠️ Half-edge mesh kernel operation failure.
#[derive(Debug, Clone, PartialEq)]
pub enum MeshKernelError {
    InvalidHandle,
    NonManifold,
    DegenerateOperation,
    EmptySelection,
    InvalidInput(String),
    Retained(semio_framework_value::ValueError),
}

impl std::fmt::Display for MeshKernelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHandle => formatter.write_str("invalid handle"),
            Self::NonManifold => formatter.write_str("mesh is non-manifold"),
            Self::DegenerateOperation => formatter.write_str("degenerate operation"),
            Self::EmptySelection => formatter.write_str("empty selection"),
            Self::InvalidInput(detail) => write!(formatter, "invalid input: {detail}"),
            Self::Retained(error) => std::fmt::Display::fmt(error,formatter),
        }
    }
}

impl std::error::Error for MeshKernelError {}
//#endregion ⚠️ Errors

pub type MeshResult<T> = Result<T, MeshKernelError>;

#[derive(Clone,Copy)]
pub enum MeshSourceVertex { Original(u32),Contributors(u32),Edge{a:u32,b:u32,t:f64},Face(u32),FaceSamples(u32),Weighted{vertices:[u32;4],weights:[f64;4]} }

#[derive(Clone, Copy, PartialEq)]
pub struct MeshRankedEdge {
    pub rank: f32,
    pub a: u32,
    pub b: u32,
}

impl Eq for MeshRankedEdge {}
impl PartialOrd for MeshRankedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for MeshRankedEdge {
    fn cmp(&self, other: &Self) -> Ordering { self.rank.total_cmp(&other.rank).then((self.a, self.b).cmp(&(other.a, other.b))) }
}

#[derive(Default)]
pub struct MeshPolygonSoup {
    pub positions: Vec<[f32; 3]>,
    pub faces: Vec<Vec<u32>>,
    pub normals: Vec<Vec3>,
}

impl semio_framework_value::retirement::RetireOwned for MeshPolygonSoup {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.positions),semio_framework_value::retirement::deferred(self.faces),semio_framework_value::retirement::deferred(self.normals)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.positions),semio_framework_value::retirement::deferred_birth_bytes_for(&self.faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.normals)])}
    fn controlled_retirement_supported()->bool {true}
}

semio_framework_value::artifact_retire_leaf!(Vec3,VertexId,EdgeId,FaceId,MeshSourceVertex,MeshRankedEdge);

/// 📊 Completed work and a conservative remaining-work estimate, finalized on completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshModelingProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: &'static str,
}

