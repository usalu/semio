//! 📦️ Exact rotated boxes and shared-axis footprint intersections.

use super::*;
use semio_framework_3d_brep_boolean::certified_boundaries::{empty_result, unsupported};
use semio_framework_3d_brep_representation::contract::{Footprint,PolygonRegion,RectangleOperation};
use semio_framework_3d_brep_vector::{Vec3, matrix::Affine3};

const ANGULAR: f64 = 64.0 * f64::EPSILON;

/// 🧭️ Three orthogonal extents and their authored corner.
#[derive(Clone, Copy)]
pub struct OrientedBox {
    origin: Pnt3,
    axes: [Vec3; 3],
    size: [f64; 3],
}

impl OrientedBox {
    pub fn from_primitive(shape: SolidPrimitive) -> Option<Self> {
        let SolidPrimitive::Box { origin, width, depth, height } = shape else { return None; };
        Some(Self { origin, axes: [Vec3::X, Vec3::Y, Vec3::Z], size: [width, depth, height] })
    }

    pub fn transformed(self, map: &Affine3) -> Option<Self> {
        let origin = map.apply_point(self.origin);
        let sides = std::array::from_fn::<_, 3, _>(|i| map.apply_vector(self.axes[i] * self.size[i]));
        let size = sides.map(|side| side.x.hypot(side.y).hypot(side.z));
        let tol = Tol::DEFAULT.value();
        if !origin.to_array().into_iter().all(f64::is_finite) || !size.into_iter().all(|n| n.is_finite() && n > tol) { return None; }
        let axes = std::array::from_fn::<_, 3, _>(|i| sides[i] * (1.0 / size[i]));
        let valid = [(0, 1), (0, 2), (1, 2)].into_iter().all(|(i, j)| {
            let error = axes[i].dot(axes[j]).abs();
            error <= ANGULAR && error * size[i].max(size[j]) <= tol
        });
        if !valid || (0..8).any(|corner| {
            let point = (0..3).filter(|axis| corner & (1 << axis) != 0).fold(origin, |point, axis| point + sides[axis]);
            !point.to_array().into_iter().all(f64::is_finite)
        }) { return None; }
        Some(Self { origin, axes, size })
    }

    pub fn placement(&self)->(Pnt3,[Vec3;3],[f64;3]){
        (self.origin,self.axes,self.size)
    }

    pub fn boundary(self) -> Option<SolidBoundary> {
        BoxPrismSource { placement: self, clip: None }.boundary()
    }

    pub(super) fn section(self,origin:[f64;3],normal:[f64;3])->Result<BoxSectionDraft,&'static str>{
        BoxSectionDraft::rectangular(self.origin,self.axes,self.size,origin,normal)
    }

    pub fn split(self,origin:[f64;3],normal:[f64;3])->Result<[SolidBoundary;2],ValueError>{
        ClippedBox::split(self.origin,self.axes,self.size,Pnt3::from_array(origin),Vec3::from_array(normal))
            .map(|pair|pair.map(|(shape,proof)|SolidBoundary::ClippedBox(shape,proof)))
            .ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original Split requires a representable plane strictly inside the box"))
    }

    pub(super) fn contains(self,other:Self)->bool{
        semio_framework_3d_brep_construction::skew_box_clip::contains_box(self.placement(),other.placement(),0.)
    }

    pub(super) fn common_axis(self, other: Self) -> Option<(usize,usize)> {
        let tol = Tol::DEFAULT.value();
        let scale = self.size.into_iter().chain(other.size).fold(0.0, f64::max);
        (0..3).find_map(|i| (0..3).find(|&j| {
            let tilt = self.axes[i].cross(other.axes[j]).norm();
            tilt <= ANGULAR && tilt * scale <= tol / 4.0
        }).map(|j| (i, j)))
    }

    /// 🔺️ Combines full-height rectangular footprints on a common axial span.
    pub fn footprint_boolean(self, other: Self, operation: RectangleOperation) -> Result<SolidBoundary,ValueError> {
        let (placement, region) = self.footprint_region(other,operation)?;
        let mut rings = region.rings();
        let outer = rings.next().ok_or_else(empty_result)?;
        let clip = match rings.len() {
            0 => Clip::Ring(Footprint::from_ring(outer).ok_or_else(unsupported)?),
            1 if outer.len()==4 && PolygonRegion::ring_area(outer)>0. => {
                let hole = rings.next().unwrap();
                if PolygonRegion::ring_area(hole)>=0. { return Err(unsupported()); }
                Clip::Hole(hole.try_into().map_err(|_|unsupported())?)
            }
            _ => return Err(unsupported()),
        };
        BoxPrismSource { placement,clip:Some(clip) }.boundary().ok_or_else(unsupported)
    }

    pub(super) fn footprint_region(self, other: Self, operation: RectangleOperation) -> Result<(Self, PolygonRegion),ValueError> {
        let tol = Tol::DEFAULT.value();
        let (i,j) = self.common_axis(other).ok_or_else(unsupported)?;
        let [u,v] = [(i+1)%3,(i+2)%3];
        let [x,y,z] = [self.axes[u],self.axes[v],self.axes[i]];
        let delta = other.origin-self.origin;
        let start = delta.dot(z);
        let end = start + other.size[j].copysign(other.axes[j].dot(z));
        if !(start.min(end).abs() <= tol/4. && (start.max(end)-self.size[i]).abs() <= tol/4.) { return Err(unsupported()); }
        let [w,d] = [(j+1)%3,(j+2)%3];
        let offsets = [Vec3::ZERO,other.axes[w]*other.size[w],other.axes[w]*other.size[w]+other.axes[d]*other.size[d],other.axes[d]*other.size[d]];
        let mut tool = offsets.map(|offset| { let world=delta+offset; [world.dot(x),world.dot(y)] });
        if PolygonRegion::ring_area(&tool)<0. { tool.reverse(); }
        let stock = [[0.,0.],[self.size[u],0.],[self.size[u],self.size[v]],[0.,self.size[v]]];
        let region = PolygonRegion::rectangles(stock,tool,operation,tol).ok_or_else(unsupported)?;
        let placement = Self { origin:self.origin,axes:[x,y,z],size:[self.size[u],self.size[v],self.size[i]] };
        Ok((placement,region))
    }

    /// 🪚️ Intersects boxes with a common principal axis by clipping their exact planar footprints.
    pub fn intersect(self, other: Self) -> Result<SolidBoundary, ValueError> {
        if self.contains(other) { return other.boundary().ok_or_else(unsupported); }
        if other.contains(self) { return self.boundary().ok_or_else(unsupported); }
        let tol = Tol::DEFAULT.value();
        let Some((i,j))=self.common_axis(other) else{
            use semio_framework_3d_brep_construction::skew_box_clip::{SkewBoxClipRefusal,skew_box_intersection};
            return skew_box_intersection(self.placement(),other.placement()).map(|shape|shape.map(SolidBoundary::ClippedBox,SolidBoundary::Hull))
                .map_err(|refusal|match refusal{SkewBoxClipRefusal::Empty=>empty_result(),SkewBoxClipRefusal::Unsupported=>unsupported()});
        };
        let [u, v] = [(i + 1) % 3, (i + 2) % 3];
        let [x, y, z] = [self.axes[u], self.axes[v], self.axes[i]];
        let delta = other.origin - self.origin;
        let start = delta.dot(z);
        let end = start + other.size[j].copysign(other.axes[j].dot(z));
        if !start.is_finite() || !end.is_finite() { return Err(unsupported()); }
        let lo = start.min(end).max(0.0);
        let height = start.max(end).min(self.size[i]) - lo;
        if height <= 0.0 { return Err(empty_result()); }
        let mut ring = Footprint::rectangle(self.size[u], self.size[v]);
        for k in [(j + 1) % 3, (j + 2) % 3] {
            let axis = other.axes[k];
            let projected = [axis.dot(x), axis.dot(y)];
            let at = (delta - z * lo).dot(axis);
            ring = ring.clip(projected, at + other.size[k]).ok_or_else(unsupported)?;
            ring = ring.clip(projected.map(|n| -n), -at).ok_or_else(unsupported)?;
        }
        ring.remove_collinear();
        let area = ring.twice_area();
        if area.is_finite() && area <= tol * tol { return Err(empty_result()); }
        if !area.is_finite() || height <= tol { return Err(unsupported()); }
        let placement = Self { origin: self.origin + z * lo, axes: [x, y, z], size: [self.size[u], self.size[v], height] };
        BoxPrismSource { placement, clip: Some(Clip::Ring(ring)) }.boundary().ok_or_else(unsupported)
    }
}

impl SolidProvenance {
    /// 🧲️ Preserves the source encoding when its affine image is an exact supported solid.
    pub fn affine_image(self, map: &Affine3, copy: bool) -> Option<(SolidBoundary, bool)> {
        let source = match self {
            Self::PlanarBoundary(Some(shape)) => shape.rectangular()?,
            Self::Primitive(PrimitiveProvenance { primitive, nurbs_cuboid, .. }) => {
                let result = if copy { Some(primitive) } else { primitive.affinely_transformed(map) };
                if let Some(shape) = result { return Some((SolidBoundary::Analytic(shape), nurbs_cuboid)); }
                if nurbs_cuboid { return None; }
                OrientedBox::from_primitive(primitive)?
            }
            _ => return None,
        };
        let boundary = source.transformed(map)?.boundary()?;
        Some((boundary, false))
    }
}

/// 🧺️ A fixed ordered set of disconnected exact Box Boolean boundaries.
pub enum BoxPrismBatch {
    Shared { placement: OrientedBox, region: PolygonRegion },
    Independent([OrientedBox; 2]),
    Sewn(semio_framework_3d_brep_construction::skew_box_clip::BoxSupportPieces),
}
impl BoxPrismBatch {
    pub fn count(&self) -> usize { match self { Self::Shared { region, .. } => region.rings().len(), Self::Independent(_) => 2, Self::Sewn(pieces) => pieces.count() } }
    pub fn copy_bytes(&self)->usize{if matches!(self,Self::Sewn(_)){size_of::<HullBoundary>()}else{size_of::<BoxPrismSource>()}}
    pub fn member(&self,index:usize) -> Option<SolidBoundary> {
        match self {
            Self::Independent(boxes) => boxes.get(index)?.boundary(),
            Self::Sewn(pieces) => HullBoundary::from_box_faces(pieces.member(index)?).map(SolidBoundary::Hull),
            Self::Shared { placement, region } => {
                let clip = Clip::Ring(Footprint::from_ring(region.rings().nth(index)?)?);
                BoxPrismSource { placement:*placement,clip:Some(clip) }.boundary()
            }
        }
    }
}

/// 🪟️ One rectangular placement or its clipped convex footprint over a common axial interval.
#[derive(Clone, Copy)]
pub struct BoxPrismSource {
    placement: OrientedBox,
    clip: Option<Clip>,
}

#[derive(Clone,Copy)]
enum Clip { Ring(Footprint), Hole([[f64;2];4]) }

impl BoxPrismSource {
    pub(super) fn section(self,origin:[f64;3],normal:[f64;3])->Option<Result<BoxSectionDraft,&'static str>>{
        if matches!(self.clip,Some(Clip::Hole(_))){return None;}
        let footprint=self.footprint();
        let points=footprint.points();
        let n=points.len();
        if !(0..n).all(|i|{
            let [a,b,c]=[points[i],points[(i+1)%n],points[(i+2)%n]];
            (b[0]-a[0])*(c[1]-b[1])-(b[1]-a[1])*(c[0]-b[0])>0.
        }) {return None;}
        let stock=self.placement;
        Some(BoxSectionDraft::prismatic(stock.origin,stock.axes,stock.size[2],footprint,origin,normal))
    }

    pub fn rectangular(self) -> Option<OrientedBox> { self.clip.is_none().then_some(self.placement) }

    fn footprint(self) -> Footprint {
        if let Some(Clip::Ring(ring))=self.clip { return ring; }
        Footprint::rectangle(self.placement.size[0], self.placement.size[1])
    }

    fn boundary(self) -> Option<SolidBoundary> {
        let OrientedBox { origin, axes: [x, y, z], size: [_, _, height] } = self.placement;
        if let Some(Clip::Hole(hole))=self.clip {
            let outer = self.footprint();
            let shape = PerforatedPrism::new([outer.points().try_into().ok()?,hole],origin,[x,y,z],height,Tol::DEFAULT.value())?;
            return Some(SolidBoundary::PerforatedBox(shape,self));
        }
        let prism = prism(self.footprint(), origin, x, y, z * height)?;
        Some(SolidBoundary::BoxPrism(prism, self))
    }

    /// 📍️ Measures the exact distance to the retained profile and finite caps.
    pub fn signed_boundary_distance(self, point: Pnt3) -> f64 {
        let delta = point - self.placement.origin;
        let [x, y, z] = self.placement.axes.map(|axis| delta.dot(axis));
        if ![x,y,z].into_iter().all(f64::is_finite) { return f64::INFINITY; }
        let mut planar = self.footprint().signed_distance([x,y]);
        if let Some(Clip::Hole(hole))=self.clip { planar=planar.max(-Footprint::ring_signed_distance(&hole,[x,y])); }
        let axial = (-z).max(z - self.placement.size[2]);
        planar.max(0.0).hypot(axial.max(0.0)) + planar.max(axial).min(0.0)
    }
}

fn prism(footprint: Footprint, origin: Pnt3, x: Vec3, y: Vec3, span: Vec3) -> Option<PrismBoundary> {
    let mut corners = [origin; semio_framework_3d_brep_construction::prism_boundary::MAX_PRISM_CORNERS];
    let tol = Tol::DEFAULT.value();
    let length = span.norm();
    if !length.is_finite() || length <= tol { return None; }
    for (i, &[u, v]) in footprint.points().iter().enumerate() {
        let local = x * u + y * v;
        corners[i] = origin + local;
        let top = corners[i] + span;
        if !corners[i].to_array().into_iter().chain(top.to_array()).all(f64::is_finite)
            || ((corners[i] - origin) - local).norm() > tol / 4.0
            || ((top - corners[i]) - span).norm() > tol / 4.0 { return None; }
    }
    if (0..footprint.points().len()).any(|i| {
        let a = corners[i];
        let b = corners[(i + 1) % footprint.points().len()];
        let tangent = b - a;
        let lengths = [tangent.norm(), (b + span - (a + span)).norm()];
        let normal = tangent.cross(span).norm();
        !lengths.into_iter().all(|value| value.is_finite() && value > tol)
            || !normal.is_finite() || normal <= f64::EPSILON
    }) { return None; }
    PrismBoundary::from_planar_ring(corners, footprint.points().len(), span, x.cross(y))
}
pub(super) fn perforated_edge(shape:&PerforatedPrism,id:usize)->PrimitiveEdge {
    let (vertices,curve)=shape.edge(id);
    PrimitiveEdge{vertices,curve,range:(0.,1.)}
}

pub(super) fn perforated_trim(shape:&PerforatedPrism,ring:usize,member:usize)->PrimitiveTrim {
    let (edge,forward)=PerforatedPrism::member(ring,member);
    let spec=perforated_edge(shape,edge);
    let frame=shape.frame(if ring>=10{ring-10}else{ring});
    let curve=semio_framework_3d_brep_construction::primitives::planar_pcurve(&spec.curve,frame);
    PrimitiveTrim{edge,forward,curve,range:spec.range}
}
