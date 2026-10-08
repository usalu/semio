//! 🫳️ Ordered JSON ordinals reborrow the original Brep topology and geometry owner.
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};
use crate::standards::v1::subsets::brep::schema::snapshot::*;
use semio_framework_plugin::plugin_app_close_prelude::store::{ArtifactCanonicalJson, ArtifactCanonicalJsonNode as Node, ARTIFACT_CANONICAL_JSON_DEPTH};

trait Borrowed {
    fn node(&self) -> Node<'_>;
    fn child(&self, _index: usize) -> Result<&dyn Borrowed, String> { Err(invalid()) }
    fn key(&self, _index: usize) -> Result<&str, String> { Err(invalid()) }
}

fn invalid() -> String { "brep.borrowed-json.invalid-ordinal".into() }

macro_rules! scalar {
    ($ty:ty, $variant:ident) => {
        impl Borrowed for $ty { fn node(&self) -> Node<'_> { Node::$variant(*self) } }
    };
}
scalar!(f64, F64);
scalar!(u64, U64);
impl Borrowed for u32 { fn node(&self) -> Node<'_> { Node::U64(u64::from(*self)) } }
scalar!(bool, Bool);
impl Borrowed for String { fn node(&self) -> Node<'_> { Node::String(self.as_str()) } }
impl Borrowed for &'static str { fn node(&self) -> Node<'_> { Node::String(self) } }
impl<T: Borrowed> Borrowed for Vec<T> {
    fn node(&self) -> Node<'_> { Node::Array(self.len()) }
    fn child(&self, index: usize) -> Result<&dyn Borrowed, String> { self.get(index).map(|value| value as &dyn Borrowed).ok_or_else(invalid) }
}
impl<T: Borrowed> Borrowed for Option<T> {
    fn node(&self) -> Node<'_> { self.as_ref().map_or(Node::Null, Borrowed::node) }
    fn child(&self, index: usize) -> Result<&dyn Borrowed, String> { self.as_ref().ok_or_else(invalid)?.child(index) }
    fn key(&self, index: usize) -> Result<&str, String> { self.as_ref().ok_or_else(invalid)?.key(index) }
}
impl Borrowed for (f64, f64) {
    fn node(&self) -> Node<'_> { Node::Array(2) }
    fn child(&self, index: usize) -> Result<&dyn Borrowed, String> { match index { 0 => Ok(&self.0), 1 => Ok(&self.1), _ => Err(invalid()) } }
}

macro_rules! object {
    ($ty:ty, $($field:ident => $key:literal),+ $(,)?) => {
        impl Borrowed for $ty {
            fn node(&self) -> Node<'_> { Node::Object([$($key),+].len()) }
            fn child(&self, index: usize) -> Result<&dyn Borrowed, String> {
                let fields: &[&dyn Borrowed] = &[$(&self.$field),+];
                fields.get(index).copied().ok_or_else(invalid)
            }
            fn key(&self, index: usize) -> Result<&str, String> { [$($key),+].get(index).copied().ok_or_else(invalid) }
        }
    };
}
object!(SemioPoint2, x => "x", y => "y");
object!(SemioPoint3, x => "x", y => "y", z => "z");
object!(BrepVertex, id => "id", point => "point", tol => "tol");
object!(BrepEdge, id => "id", start_vertex => "startVertex", end_vertex => "endVertex", curve => "curve", tol => "tol");
object!(BrepLoopEdge, edge => "edge", orientation => "orientation");
object!(BrepLoop, id => "id", edges => "edges");
object!(BrepCoedge, id => "id", edge => "edge", forward => "forward", pcurve => "pcurve", prange => "prange", loop_id => "loopId", next => "next", prev => "prev");
object!(BrepFace, id => "id", outer_loop => "outerLoop", inner_loops => "innerLoops", surface => "surface", orientation => "orientation", tol => "tol");
object!(BrepShellFace, face => "face", orientation => "orientation");
object!(BrepShell, id => "id", faces => "faces");
object!(BrepSolidShell, shell => "shell", is_void => "isVoid");
object!(BrepSolid, id => "id", shells => "shells");
object!(SemioBrepSnapshot, schema => "schema", vertices => "vertices", edges => "edges", loops => "loops", faces => "faces", shells => "shells", solids => "solids", coedges => "coedges", next_label => "nextLabel");

macro_rules! tagged {
    ($ty:ident, $($variant:ident => $kind:literal { $($field:ident => $key:literal),+ }),+ $(,)?) => {
        impl Borrowed for $ty {
            fn node(&self) -> Node<'_> { match self { $(Self::$variant { .. } => Node::Object(["kind", $($key),+].len())),+ } }
            fn child(&self, index: usize) -> Result<&dyn Borrowed, String> {
                match self { $(Self::$variant { $($field),+ } => {
                    let fields: &[&dyn Borrowed] = &[&$kind, $($field),+];
                    fields.get(index).copied().ok_or_else(invalid)
                }),+ }
            }
            fn key(&self, index: usize) -> Result<&str, String> { match self { $(Self::$variant { .. } => ["kind", $($key),+].get(index).copied().ok_or_else(invalid)),+ } }
        }
    };
}
tagged!(BrepCurve,
    Line => "line" { origin => "origin", direction => "direction" },
    Circle => "circle" { center => "center", axis => "axis", radius => "radius" },
    Ellipse => "ellipse" { center => "center", axis => "axis", radius_major => "radiusMajor", radius_minor => "radiusMinor" },
    Nurbs => "nurbs" { control_points => "controlPoints", weights => "weights", degree => "degree", knots => "knots" },
);
tagged!(BrepCurve2,
    Line => "line" { origin => "origin", direction => "direction" },
    Circle => "circle" { center => "center", radius => "radius" },
    Ellipse => "ellipse" { center => "center", x_axis => "xAxis", radius_major => "radiusMajor", radius_minor => "radiusMinor" },
    Nurbs => "nurbs" { control_points => "controlPoints", weights => "weights", degree => "degree", knots => "knots" },
);
tagged!(BrepSurface,
    Plane => "plane" { origin => "origin", normal => "normal" },
    Cylinder => "cylinder" { origin => "origin", axis => "axis", radius => "radius" },
    Cone => "cone" { origin => "origin", axis => "axis", radius => "radius", half_angle => "halfAngle" },
    Sphere => "sphere" { center => "center", radius => "radius" },
    Torus => "torus" { center => "center", axis => "axis", major_radius => "majorRadius", minor_radius => "minorRadius" },
    Nurbs => "nurbs" { control_points => "controlPoints", weights => "weights", u_count => "uCount", v_count => "vCount", degree_u => "degreeU", degree_v => "degreeV", knots_u => "knotsU", knots_v => "knotsV" },
);

fn at<'a>(root: &'a dyn Borrowed, path: &[usize]) -> Result<&'a dyn Borrowed, String> {
    if path.len() > ARTIFACT_CANONICAL_JSON_DEPTH { return Err(invalid()); }
    let mut value = root;
    for index in path { value = value.child(*index)?; }
    Ok(value)
}

impl ArtifactCanonicalJson for SemioBrepSnapshot {
    fn canonical_json_node(&self, path: &[usize]) -> Result<Node<'_>, String> { Ok(at(self, path)?.node()) }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJsonText<'_>, String> { at(self, path)?.key(index).map(Into::into) }
}
