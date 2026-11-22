//! ✂️ Clips a simple prism with enclosing convex sides and transverse cap planes.

use super::{MeshIntersectionFace, MeshIntersectionSolid, MeshResult, NestedPrism, Plane, Soup, SourceVertex, difference, dot, invalid};
use super::nested_prism::MAX_PROFILE;

#[path = "🧱️longitudinal/🦀️.rs"]
mod longitudinal;
use longitudinal::{LongitudinalClip, LongitudinalSource};

pub(super) enum PrismClipStep { More, Retained, Empty, Clipped(MeshIntersectionSolid) }

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct Piece {
    interval: [f64; 2],
    caps: [Option<usize>; 2],
    outside: usize,
}

#[derive(semio_framework_value::RetireOwned)]
struct TransverseSupports {
    axis: usize,
    faces: [usize; 2],
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct PrismClip {
    side: usize,
    subtract: bool,
    outside: Option<usize>,
    remaining: Option<Piece>,
    lateral: Option<LongitudinalClip>,
    transverse: Option<TransverseSupports>,
    interval: [f64; 2],
    caps: [Option<usize>; 2],
    samples: [f64; 2],
    plane: usize,
    cursor: usize,
    face: usize,
    corner: usize,
    mapped: [u32; 2 * MAX_PROFILE],
    result: MeshIntersectionSolid,
    stage: u8,
}

fn height(point: [f32; 3], plane: Plane) -> f64 {
    dot(difference(point, plane.origin.0), plane.normal.0.map(f64::from))
}

impl PrismClip {
    pub(super) fn new(side: usize, subtract: bool) -> Self {
        Self { side, subtract, outside: None, remaining: None, lateral: None, transverse: None, interval: [0., 1.], caps: [None; 2], samples: [f64::INFINITY, f64::NEG_INFINITY], plane: 0,
            cursor: 0, face: 0, corner: 0, mapped: [0; 2 * MAX_PROFILE], stage: 0,
            result: MeshIntersectionSolid { geometry: Soup::default(), faces: Vec::new(), points: Vec::new(),
                right_points: Vec::new(), right_interpolations: Vec::new(), retained_operand: None } }
    }

    pub(super) fn transverse(axis: usize, faces: [usize;2], subtract: bool) -> Self {
        let mut clip = Self::new(0, subtract);
        clip.transverse = Some(TransverseSupports { axis, faces });
        clip
    }

    fn support(&self, planes: &[Plane]) -> Option<(usize, Plane)> {
        let id = if let Some(transverse) = &self.transverse { *transverse.faces.get(self.plane)? } else { self.plane };
        planes.get(id).copied().map(|plane| (id, plane))
    }

    fn start_piece(&mut self,piece:Piece){
        self.interval=piece.interval;
        self.caps=piece.caps;
        self.outside=Some(piece.outside);
        self.cursor=0;
        self.face=0;
        self.corner=0;
    }

    fn outside_result(&self)->PrismClipStep{
        if self.subtract{PrismClipStep::Retained}else{PrismClipStep::Empty}
    }

    fn settle_interval(&mut self,thickness:f64,epsilon:f64)->PrismClipStep{
        let [low,high]=self.interval;
        if (high-low)*thickness<=epsilon{return self.outside_result();}
        if self.caps==[None;2]{
            return if self.subtract{PrismClipStep::Empty}else{PrismClipStep::Retained};
        }
        if self.subtract && self.lateral.is_none() {
            let below=self.caps[0].filter(|_|low*thickness>epsilon)
                .map(|cap|Piece{interval:[0.,low],caps:[None,Some(cap)],outside:cap});
            let above=self.caps[1].filter(|_|(1.-high)*thickness>epsilon)
                .map(|cap|Piece{interval:[high,1.],caps:[Some(cap),None],outside:cap});
            let Some(first)=below.or(above) else{return PrismClipStep::Empty;};
            if below.is_some(){self.remaining=above;}
            self.start_piece(first);
        }
        self.stage=1;
        PrismClipStep::More
    }

    fn restrict(&mut self, source: &Soup, prism: &NestedPrism, normal: [f64; 3], (face, plane): (usize, Plane), epsilon: f64) -> MeshResult<bool> {
        let [min, max] = self.samples;
        if max <= epsilon { return Ok(true); }
        if min >= -epsilon { return Ok(false); }
        let [low, high] = prism.pair(source, 0).map(|id| height(source.positions[id as usize], plane));
        let slope = high - low;
        if slope.abs() <= epsilon {
            if let Some(lateral) = &mut self.lateral { lateral.add_plane(face)?; }
            else { self.lateral = Some(LongitudinalClip::new(face)); }
            return Ok(true);
        }
        if dot(normal, plane.normal.0.map(f64::from)).abs() < 1.0 - 1e-6 {
            return Err(invalid("concave prism crosses an oblique support"));
        }
        let t = (-low / slope).clamp(0., 1.);
        let side = usize::from(slope > 0.);
        if (side == 0 && t > self.interval[0]) || (side == 1 && t < self.interval[1]) {
            self.interval[side] = t;
            self.caps[side] = Some(face);
        }
        Ok(true)
    }

    fn point(&mut self, source: &Soup, prism: &NestedPrism, planes: &[Plane], epsilon: f64) -> MeshResult<()> {
        let [a,b] = prism.pair(source, self.cursor / 2);
        let side = self.cursor % 2;
        let original = [a,b][side];
        self.mapped[original as usize]=original;
        if let Some(cap) = self.caps[side] {
            let t = self.interval[side];
            let [first,last] = [a,b].map(|id| source.positions[id as usize].map(f64::from));
            let mut point = std::array::from_fn(|axis| (first[axis] + (last[axis] - first[axis]) * t) as f32);
            if let Some(transverse) = &self.transverse {
                point[transverse.axis] = planes[cap].origin.0[transverse.axis];
            }
            if height(point, planes[cap]).abs() > epsilon { return Err(invalid("concave prism cap lost its supporting plane")); }
            let id = self.result.geometry.positions.len() as u32;
            let recipe = SourceVertex::Edge { a, b, t };
            self.result.geometry.positions.push(point);
            if self.side == 0 { self.result.points.push(recipe); }
            else { self.result.right_interpolations.push((id, recipe)); }
            self.mapped[original as usize] = id;
        }
        self.cursor += 1;
        Ok(())
    }

    fn ring(&mut self, source: &Soup, prism: &NestedPrism) {
        let ring = &source.faces[self.face];
        if self.corner == 0 {
            let support = prism.caps().iter().position(|&face| face == self.face).and_then(|end| self.caps[end]);
            let (owner, face) = support.map_or((self.side, self.face as u32), |face| (1 - self.side, face as u32));
            self.result.faces.push(if owner == 0 { MeshIntersectionFace::Left(face) }
                else if support.is_some()&&self.subtract { MeshIntersectionFace::ReversedRightPlane(face) }
                else { MeshIntersectionFace::RightPlane(face) });
            self.result.geometry.faces.push(Vec::new());
        }
        self.result.geometry.faces.last_mut().unwrap().push(self.mapped[ring[self.corner] as usize]);
        self.corner += 1;
        if self.corner == ring.len() { self.corner = 0; self.face += 1; }
    }

    fn take_result(&mut self)->PrismClipStep {
        PrismClipStep::Clipped(MeshIntersectionSolid {
            geometry: std::mem::take(&mut self.result.geometry), faces: std::mem::take(&mut self.result.faces),
            points: std::mem::take(&mut self.result.points), right_points: std::mem::take(&mut self.result.right_points),
            right_interpolations: std::mem::take(&mut self.result.right_interpolations), retained_operand: None,
        })
    }

    pub(super) fn advance(
        &mut self, source: &Soup, prism: &NestedPrism, source_planes: &[Plane], outer: &[Plane], epsilon: f32,
    ) -> MeshResult<PrismClipStep> {
        let epsilon = f64::from(epsilon);
        match self.stage {
            0 => {
                let normal = source_planes[prism.caps()[1]].normal.0.map(f64::from);
                if let Some((face, plane)) = self.support(outer) {
                    if let Some(&point) = source.positions.get(self.cursor) {
                        let d = height(point, plane);
                        self.samples = [self.samples[0].min(d), self.samples[1].max(d)];
                        self.cursor += 1;
                    } else {
                        if !self.restrict(source, prism, normal, (face, plane), epsilon)? { return Ok(self.outside_result()); }
                        self.plane += 1;
                        self.cursor = 0;
                        self.samples = [f64::INFINITY, f64::NEG_INFINITY];
                    }
                } else {
                    if self.lateral.is_some() && self.caps == [None;2] { self.stage=1; return Ok(PrismClipStep::More); }
                    let [low, high] = prism.pair(source, 0).map(|id| source.positions[id as usize]);
                    return Ok(self.settle_interval(dot(difference(high,low),normal),epsilon));
                }
            }
            1 => {
                if let Some(&point) = source.positions.get(self.cursor) {
                    let id = self.cursor as u32;
                    self.result.geometry.positions.push(point);
                    self.mapped[self.cursor] = id;
                    if self.side == 0 { self.result.points.push(SourceVertex::Original(id)); }
                    else { self.result.right_points.push(id); }
                    self.cursor += 1;
                } else { self.stage = if self.lateral.is_some() && self.caps == [None;2]{5}else{2}; self.cursor = 0; }
            }
            2 => {
                if self.cursor < source.positions.len() { self.point(source, prism, outer, epsilon)?; }
                else { self.stage = if self.lateral.is_some(){5}else{3}; }
            }
            3 => {
                if self.face < source.faces.len() { self.ring(source, prism); }
                else { self.stage = 4; self.cursor = 0; self.plane = 0; }
            }
            4 => {
                if self.cursor == source.positions.len() {
                    if let Some(piece)=self.remaining.take(){
                        self.start_piece(piece);
                        self.stage=2;
                        return Ok(PrismClipStep::More);
                    }
                    return Ok(self.take_result());
                }
                let point = self.result.geometry.positions[self.mapped[self.cursor] as usize];
                if let Some(outside)=self.outside{
                    if height(point,outer[outside]) < -epsilon { return Err(invalid("concave prism Difference left material inside its cutter")); }
                    self.cursor+=1;
                }else{
                    if height(point, self.support(outer).unwrap().1) > epsilon { return Err(invalid("concave prism vertex exceeds its enclosing support")); }
                    self.plane += 1;
                    if self.support(outer).is_none() { self.plane = 0; self.cursor += 1; }
                }
            }
            5 => {
                let lateral=self.lateral.as_mut().unwrap();
                let input=LongitudinalSource{source,prism,planes:outer,mapped:&self.mapped,caps:self.caps,
                    stock_normal:source_planes[prism.caps()[0]].normal.0.map(f64::from),
                    interval:self.interval,side:self.side,subtract:self.subtract,epsilon};
                if lateral.advance(input,&mut self.result)? {
                    return Ok(self.take_result());
                }
            }
            _ => unreachable!(),
        }
        Ok(PrismClipStep::More)
    }
}
