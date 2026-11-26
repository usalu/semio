//! 🧮️ Exact analytic and rectilinear Boolean result recipes.

use super::*;
use semio_framework_3d_brep_representation::contract::{PolygonRegion, RectangleOperation};
use semio_framework_3d_brep_boolean::{BooleanOp, certified_boundaries::{coaxial_cylinder_bore, coaxial_cylinder_step, coaxial_cylinders, empty_result, unsupported}};
#[path="🧱️box-cylinder/🦀️.rs"]
mod box_cylinder;
#[path="✂️disk-rectangle/🦀️.rs"]
mod disk_rectangle;
#[path="🥫️parallel-cylinders/🦀️.rs"]
mod parallel_cylinders;

#[path="🌗️sphere-box/🦀️.rs"]
mod sphere_box;

#[path="🌐️sphere-cylinder/🦀️.rs"]
mod sphere_cylinder;

#[derive(Clone, Copy)]
pub struct Bounds { pub lo: [f64; 3], pub hi: [f64; 3] }

impl Bounds {
    pub fn box_primitive(origin: Pnt3, width: f64, depth: f64, height: f64) -> Self {
        Self { lo: origin.to_array(), hi: [origin.x + width, origin.y + depth, origin.z + height] }
    }
    fn primitive(self) -> SolidPrimitive {
        SolidPrimitive::Box { origin: Pnt3::from_array(self.lo), width: self.hi[0] - self.lo[0], depth: self.hi[1] - self.lo[1], height: self.hi[2] - self.lo[2] }
    }
    fn intersects(self) -> bool { (0..3).all(|i| self.hi[i] > self.lo[i]) }
    fn contains(self, other: Self) -> bool { (0..3).all(|i| self.lo[i] <= other.lo[i] && self.hi[i] >= other.hi[i]) }
    fn intersection(self, other: Self) -> Self {
        Self { lo: std::array::from_fn(|i| self.lo[i].max(other.lo[i])), hi: std::array::from_fn(|i| self.hi[i].min(other.hi[i])) }
    }
    fn union(self, other: Self) -> Self {
        Self { lo: std::array::from_fn(|i| self.lo[i].min(other.lo[i])), hi: std::array::from_fn(|i| self.hi[i].max(other.hi[i])) }
    }
    fn same_span(self, other: Self, i: usize) -> bool { self.lo[i] == other.lo[i] && self.hi[i] == other.hi[i] }
}

fn exact_box_boolean(a: Bounds, b: Bounds, operation: BooleanOp) -> Result<Bounds, ValueError> {
    let overlap = a.intersection(b);
    match operation {
        BooleanOp::Intersect => if overlap.intersects() { Ok(overlap) } else { Err(empty_result()) },
        BooleanOp::Unite => {
            if a.contains(b) { return Ok(a); }
            if b.contains(a) { return Ok(b); }
            if (0..3).any(|axis| overlap.hi[axis] >= overlap.lo[axis] && (0..3).filter(|i| *i != axis).all(|i| a.same_span(b, i))) {
                return Ok(a.union(b));
            }
            Err(unsupported())
        }
        BooleanOp::Cut => {
            if !overlap.intersects() { return Ok(a); }
            if b.contains(a) { return Err(empty_result()); }
            for axis in 0..3 {
                if !(0..3).filter(|i| *i != axis).all(|i| a.same_span(overlap, i)) { continue; }
                let mut remaining = a;
                if overlap.lo[axis] == a.lo[axis] {
                    remaining.lo[axis] = overlap.hi[axis];
                    return Ok(remaining);
                }
                if overlap.hi[axis] == a.hi[axis] {
                    remaining.hi[axis] = overlap.lo[axis];
                    return Ok(remaining);
                }
            }
            Err(unsupported())
        }
    }
}

fn nested_primitives(outer: SolidPrimitive, inner: SolidPrimitive, outer_is_a: bool, operation: BooleanOp)
    -> Result<SolidPrimitive, ValueError> {
    match operation {
        BooleanOp::Unite => Ok(outer),
        BooleanOp::Intersect => Ok(inner),
        BooleanOp::Cut if !outer_is_a => Err(empty_result()),
        BooleanOp::Cut => Err(unsupported()),
    }
}

fn primitive_result(a: SolidPrimitive, b: SolidPrimitive, operation: BooleanOp) -> Result<SolidPrimitive, ValueError> {
    match (a, b) {
        (
            SolidPrimitive::Box { origin: a_origin, width: a_width, depth: a_depth, height: a_height },
            SolidPrimitive::Box { origin: b_origin, width: b_width, depth: b_depth, height: b_height },
        ) => exact_box_boolean(
            Bounds::box_primitive(a_origin, a_width, a_depth, a_height),
            Bounds::box_primitive(b_origin, b_width, b_depth, b_height), operation,
        ).map(Bounds::primitive),
        (SolidPrimitive::Sphere{frame:a_frame,radius:a_radius},SolidPrimitive::Sphere{frame:b_frame,radius:b_radius})=>{
            if a_frame.origin==b_frame.origin{
                return if a_radius<=b_radius{nested_primitives(b,a,false,operation)}else{nested_primitives(a,b,true,operation)};
            }
            let distance=(a_frame.origin-b_frame.origin).norm();
            let tolerance=Tol::DEFAULT.value();
            if (a_radius-b_radius).abs()-distance>tolerance{
                return if a_radius<=b_radius{nested_primitives(b,a,false,operation)}else{nested_primitives(a,b,true,operation)};
            }
            if distance.is_finite()&&distance-(a_radius+b_radius)>tolerance{
                return match operation{
                    BooleanOp::Cut=>Ok(a),
                    BooleanOp::Intersect=>Err(empty_result()),
                    BooleanOp::Unite=>Err(unsupported()),
                };
            }
            Err(unsupported())
        }
        (SolidPrimitive::Cylinder { .. }, SolidPrimitive::Cylinder { .. }) => coaxial_cylinders(a, b, operation),
        (SolidPrimitive::Box{..},SolidPrimitive::Cylinder{..})|(SolidPrimitive::Cylinder{..},SolidPrimitive::Box{..})
            =>box_cylinder::combine(a,b,operation),
        _ => Err(unsupported()),
    }
}


#[derive(Clone, Copy)]
/// 🧱️ The certified input families available to one native set operation.
pub enum BooleanShape { Analytic(SolidPrimitive), Orthogonal(OrthogonalCells), OrientedBox(OrientedBox) }

impl BooleanShape {
    fn box_bounds(primitive: SolidPrimitive) -> Option<Bounds> {
        let SolidPrimitive::Box { origin, width, depth, height } = primitive else { return None; };
        Some(Bounds::box_primitive(origin, width, depth, height))
    }

    fn material(self) -> Result<OrthogonalCells, ValueError> {
        match self {
            Self::Orthogonal(cells) => Ok(cells),
            Self::OrientedBox(_) => Err(unsupported()),
            Self::Analytic(shape) => {
                let bounds = Self::box_bounds(shape).ok_or_else(unsupported)?;
                OrthogonalCells::from_box([bounds.lo, bounds.hi])
            }
        }
    }

    fn original_box(self)->Option<OrientedBox>{
        match self{Self::OrientedBox(shape)=>Some(shape),Self::Analytic(shape)=>OrientedBox::from_primitive(shape),_=>None}
    }

    pub fn disconnected(self,other:Self,operation:BooleanOp)->Option<BoxPrismBatch>{
        if !matches!(self,Self::OrientedBox(_)) && !matches!(other,Self::OrientedBox(_)) {return None;}
        let [a,b]=[self.original_box()?,other.original_box()?];
        if matches!(operation,BooleanOp::Unite) && semio_framework_3d_brep_construction::skew_box_clip::boxes_separated(a.placement(),b.placement(),-Tol::DEFAULT.value())? {
            return Some(BoxPrismBatch::Independent([a,b]));
        }
        let operation=match operation {BooleanOp::Unite=>RectangleOperation::Union,BooleanOp::Cut=>RectangleOperation::Difference,_=>return None};
        if let Ok((placement,region))=a.footprint_region(b,operation){
            let disconnected=region.rings().len()>1&&region.rings().all(|ring|PolygonRegion::ring_area(ring)>0.);
            if disconnected{return Some(BoxPrismBatch::Shared{placement,region});}
        }
        if !matches!(operation,RectangleOperation::Difference){return None;}
        semio_framework_3d_brep_construction::skew_box_clip::box_recess_pieces(a.placement(),b.placement(),Tol::DEFAULT.value()).map(BoxPrismBatch::Sewn)
    }

    /// 🔀️ Combines certified material and preserves analytic results whenever exact.
    pub fn combine(self, other: Self, operation: BooleanOp) -> Result<SolidBoundary, ValueError> {
        if matches!(self, Self::OrientedBox(_)) || matches!(other, Self::OrientedBox(_)) {
            let (a,b)=(self.original_box().ok_or_else(unsupported)?,other.original_box().ok_or_else(unsupported)?);
            return match operation {
                BooleanOp::Intersect => a.intersect(b),
                BooleanOp::Unite if a.contains(b) => a.boundary().ok_or_else(unsupported),
                BooleanOp::Unite if b.contains(a) => b.boundary().ok_or_else(unsupported),
                BooleanOp::Unite => a.footprint_boolean(b,RectangleOperation::Union).or_else(|error|
                    semio_framework_3d_brep_construction::skew_box_clip::placed_box_union(a.placement(),b.placement())
                        .map(SolidBoundary::Hull).ok_or(error)),
                BooleanOp::Cut if b.contains(a) => Err(empty_result()),
                BooleanOp::Cut => a.footprint_boolean(b,RectangleOperation::Difference).or_else(|error|{
                    use semio_framework_3d_brep_construction::skew_box_clip::skew_box_difference;
                    let boundary=skew_box_difference(a.placement(),b.placement()).map_err(|_|error)?
                        .map(|shape|shape.map(SolidBoundary::ClippedBox,SolidBoundary::Hull));
                    boundary.or_else(||a.boundary()).ok_or_else(unsupported)
                }),
            };
        }
        if let (Self::Analytic(a), Self::Analytic(b)) = (self, other) {
            match primitive_result(a, b, operation) {
                Ok(primitive) => return Ok(SolidBoundary::Analytic(primitive)),
                Err(error) if error.kind != ValueRefusalKind::UnsupportedOwner => return Err(error),
                _ => {}
            }
            if let (Some(a), Some(b)) = (Self::box_bounds(a), Self::box_bounds(b)) {
                return OrthogonalBoundary::new([a.lo, a.hi], [b.lo, b.hi], operation).map(SolidBoundary::Orthogonal);
            }
            if matches!((a,b),(SolidPrimitive::Sphere{..},SolidPrimitive::Box{..})|(SolidPrimitive::Box{..},SolidPrimitive::Sphere{..})){
                return sphere_box::combine(a,b,operation);
            }
            if matches!((a,b),(SolidPrimitive::Sphere{..},SolidPrimitive::Cylinder{..})|(SolidPrimitive::Cylinder{..},SolidPrimitive::Sphere{..})){
                return sphere_cylinder::combine(a,b,operation);
            }
            if let (SolidPrimitive::Sphere { frame: stock, radius }, SolidPrimitive::Torus { frame, major, minor }, BooleanOp::Cut) = (a, b, operation) {
                if stock.origin == frame.origin {
                    if radius <= major - minor { return Ok(SolidBoundary::Analytic(a)); }
                    return SphericalTorusCut::new(frame, radius, major, minor, Tol::DEFAULT.value())
                        .map(SolidBoundary::SphericalTorusCut).ok_or_else(unsupported);
                }
            }
            if let (SolidPrimitive::Sphere{frame:a_frame,radius:r},SolidPrimitive::Sphere{frame:b_frame,radius:s})=(a,b){
                let tol=Tol::DEFAULT.value();
                if matches!(operation,BooleanOp::Cut){
                    if let Some(shell)=SphericalShell::from_spheres([a_frame,b_frame],[r,s],tol){
                        return Ok(SolidBoundary::SphericalShell(shell));
                    }
                }
                let (positive,inward)=match operation{
                    BooleanOp::Intersect=>([true,false],false),
                    BooleanOp::Unite=>([false,true],false),
                    BooleanOp::Cut=>([false,false],true),
                };
                return SphericalCaps::new(a_frame.origin,b_frame.origin,[r,s],positive,inward,tol)
                    .map(SolidBoundary::SphericalCaps).ok_or_else(unsupported);
            }
            if matches!((a,b),(SolidPrimitive::Cylinder{..},SolidPrimitive::Cylinder{..})){
                return match operation{
                    BooleanOp::Cut=>coaxial_cylinder_bore(a,b),
                    BooleanOp::Unite=>coaxial_cylinder_step(a,b),
                    BooleanOp::Intersect=>return parallel_cylinders::combine(a,b,operation),
                }.map(SolidBoundary::Annular).or_else(|_|parallel_cylinders::combine(a,b,operation));
            }
            if matches!(operation,BooleanOp::Intersect){
                return box_cylinder::side_intersection(a,b).map(SolidBoundary::CircularPrism);
            }
            if matches!((a, b, operation), (SolidPrimitive::Box { .. }, SolidPrimitive::Cylinder { .. }, BooleanOp::Cut)) {
                return BoreBoundary::new(a, b).map(SolidBoundary::Bore);
            }
            return Err(unsupported());
        }
        let boundary = self.material()?.combine(other.material()?, operation)?;
        if let Some([lo, hi]) = boundary.material().rectangular() {
            return Ok(SolidBoundary::Analytic(Bounds { lo, hi }.primitive()));
        }
        Ok(SolidBoundary::Orthogonal(boundary))
    }
}
