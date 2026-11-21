//! 🧱️ Clips a certified prism against independent longitudinal supports.

use super::{MeshIntersectionFace, MeshIntersectionSolid, MeshResult, NestedPrism, Plane, Soup, SourceVertex, dot, height, invalid};
use super::MAX_PROFILE;
use super::super::Vec3;

const MAX_WALLS: usize = 6;

pub(super) struct LongitudinalSource<'a> {
    pub source: &'a Soup,
    pub prism: &'a NestedPrism,
    pub planes: &'a [Plane],
    pub mapped: &'a [u32],
    pub caps: [Option<usize>; 2],
    pub interval: [f64; 2],
    pub side: usize,
    pub subtract: bool,
    pub epsilon: f64,
}

impl LongitudinalSource<'_> {
    fn face_origin(&self, face: usize) -> MeshIntersectionFace {
        let cap = self.prism.caps().iter().position(|&id| id == face);
        if let Some(support) = cap.and_then(|end| self.caps[end]) {
            return if self.side == 0 { MeshIntersectionFace::RightPlane(support as u32) }
                else { MeshIntersectionFace::Left(support as u32) };
        }
        let face = face as u32;
        match (self.side, cap.is_some()) {
            (0, true) => MeshIntersectionFace::LeftBoundary(face),
            (_, true) => MeshIntersectionFace::RightBoundary(face),
            (0, false) => MeshIntersectionFace::Left(face),
            (_, false) => MeshIntersectionFace::RightPlane(face),
        }
    }

    fn sample(&self, ends: [[u32; 2]; 2], end: usize, t: f64) -> SourceVertex {
        let [a,b] = ends.map(|pair| pair[end]);
        if self.caps[end].is_none() { return SourceVertex::Edge { a, b, t }; }
        let h = self.interval[end];
        let [[a,d],[b,c]] = ends;
        SourceVertex::Weighted {
            vertices: [a,d,b,c],
            weights: [(1.-t)*(1.-h), (1.-t)*h, t*(1.-h), t*h],
        }
    }
}

#[derive(Clone, Copy, Default, semio_framework_value::RetireOwned)]
struct Crossing {
    edges: [[u32; 2]; 2],
    points: [u32; 2],
    parameters: [f64; 2],
}

#[derive(semio_framework_value::RetireOwned)]
struct Wall {
    plane: usize,
    sides: [i8; 2 * MAX_PROFILE],
    crossings: [Crossing; 2],
    used: usize,
}

impl Wall {
    fn new(plane: usize) -> Self {
        Self { plane, sides: [0; 2 * MAX_PROFILE], crossings: [Crossing::default(); 2], used: 0 }
    }

    fn on_edge(&self, [a, b]: [u32; 2]) -> Option<(f64, u32)> {
        self.crossings.iter().find_map(|row| {
            row.edges.iter().enumerate().find_map(|(end, &pair)| {
                let t = row.parameters[end];
                if pair == [a, b] { Some((t, row.points[end])) }
                else if pair == [b, a] { Some((1. - t, row.points[end])) }
                else { None }
            })
        })
    }
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct LongitudinalClip {
    walls: [Option<Wall>; MAX_WALLS],
    wall_count: usize,
    index: usize,
    face: usize,
    corners: [u32; MAX_PROFILE + 2 * MAX_WALLS],
    corner_count: usize,
    wall: usize,
    edges_ready: bool,
    paired: [u32; 2 * MAX_PROFILE],
    upper_cap: [bool; 2 * MAX_PROFILE],
    tail: Option<usize>,
}

impl LongitudinalClip {
    pub(super) fn new(plane: usize) -> Self {
        let walls = std::array::from_fn(|index| (index == 0).then(|| Wall::new(plane)));
        Self { walls, wall_count: 1, index: 0, face: 0, corners: [0; MAX_PROFILE + 2 * MAX_WALLS],
            corner_count: 0, wall: 0, edges_ready: false,
            paired: [0; 2 * MAX_PROFILE], upper_cap: [false; 2 * MAX_PROFILE], tail: None }
    }

    pub(super) fn add_plane(&mut self, plane: usize) -> MeshResult<()> {
        let slot = self.walls.get_mut(self.wall_count)
            .ok_or_else(|| invalid("concave prism exceeds the independent longitudinal support bound"))?;
        *slot = Some(Wall::new(plane));
        self.wall_count += 1;
        Ok(())
    }

    fn edge(&mut self, input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<()> {
        let LongitudinalSource { source, prism, planes, side, epsilon, .. } = *input;
        let (earlier, remainder) = self.walls[..self.wall_count].split_at_mut(self.wall);
        let (wall, later) = remainder.split_first_mut().unwrap();
        let wall = wall.as_mut().unwrap();
        let plane = planes[wall.plane];
        let count = source.positions.len() / 2;
        let endpoints = [self.index, (self.index + 1) % count].map(|i| prism.pair(source, i));
        if self.wall == 0 && input.subtract && input.caps != [None; 2] {
            let [low, high] = endpoints[0];
            self.paired[low as usize] = high;
            self.paired[high as usize] = low;
            self.upper_cap[high as usize] = true;
        }
        let distances = endpoints.map(|pair| pair.map(|id| height(source.positions[id as usize], plane)));
        let classify = |d: f64| if d > epsilon { 1 } else if d < -epsilon { -1 } else { 0 };
        let sides = distances.map(|pair| pair.map(classify));
        let [first,last] = sides.map(|pair| pair[0]);
        if sides.iter().any(|pair| pair[0] != pair[1]) || (first == 0 && last == 0) {
            return Err(invalid("longitudinal prism cut crosses an unstable or coincident edge"));
        }
        for (pair, samples) in endpoints.into_iter().zip(sides) {
            for (id, signed) in pair.into_iter().zip(samples) {
                if signed > 0 && earlier.iter().flatten().any(|other| other.sides[id as usize] > 0) {
                    return Err(invalid("longitudinal prism supports remove overlapping material"));
                }
                wall.sides[id as usize] = signed;
            }
        }
        if first == 0 {
            let previous = prism.pair(source, (self.index + count - 1) % count);
            if previous.into_iter().any(|id| classify(height(source.positions[id as usize], plane)) * last != -1) {
                return Err(invalid("longitudinal prism support touches a tangent profile vertex"));
            }
        }
        if first == 0 || first * last < 0 {
            if wall.used == wall.crossings.len() {
                return Err(invalid("longitudinal prism cut requires one connected clipped profile"));
            }
            let mut crossing = Crossing::default();
            for end in 0..2 {
                let [a, b] = endpoints.map(|pair| pair[end]);
                let [da, db] = distances.map(|pair| pair[end]);
                let vertex = first == 0;
                let t = if vertex { 0. } else { da / (da - db) };
                let [start, finish] = [a, b].map(|id| result.geometry.positions[input.mapped[id as usize] as usize].map(f64::from));
                let point = std::array::from_fn(|axis| (start[axis] + (finish[axis] - start[axis]) * t) as f32);
                if height(point, plane).abs() > epsilon || earlier.iter().chain(later.iter()).flatten()
                    .any(|other| height(point, planes[other.plane]) >= -epsilon) {
                    return Err(invalid("longitudinal prism point lost its supporting plane"));
                }
                let id = if vertex { input.mapped[a as usize] } else {
                    let id = result.geometry.positions.len() as u32;
                    let recipe = input.sample(endpoints, end, t);
                    result.geometry.positions.push(point);
                    if side == 0 { result.points.push(recipe); }
                    else { result.right_interpolations.push((id, recipe)); }
                    id
                };
                crossing.edges[end] = [a, if vertex { a } else { b }];
                crossing.points[end] = id;
                crossing.parameters[end] = t;
            }
            wall.crossings[wall.used] = crossing;
            wall.used += 1;
        }
        self.index += 1;
        Ok(())
    }

    fn append(&mut self, id: u32) -> MeshResult<()> {
        *self.corners.get_mut(self.corner_count).ok_or_else(|| invalid("longitudinal prism ring exceeds its source bound"))? = id;
        self.corner_count += 1;
        Ok(())
    }

    fn active_walls(&self, part: Option<usize>) -> &[Option<Wall>] {
        part.map_or(&self.walls[..self.wall_count], |part| &self.walls[part..=part])
    }

    fn edge_cuts([a, b]: [u32; 2], walls: &[Option<Wall>]) -> MeshResult<[Option<(f64, u32)>; 2]> {
        let mut cuts = [None; 2];
        let mut output = cuts.iter_mut();
        for wall in walls.iter().flatten() {
            let cut = wall.on_edge([a, b]);
            if cut.is_some() != (wall.sides[a as usize] * wall.sides[b as usize] < 0) {
                return Err(invalid("longitudinal prism cut lost its paired source edge"));
            }
            if let Some(cut) = cut {
                *output.next().ok_or_else(|| invalid("longitudinal prism edge has overlapping cuts"))? = Some(cut);
            }
        }
        if cuts[0].zip(cuts[1]).is_some_and(|(first, second)| first.0 > second.0) { cuts.swap(0, 1); }
        Ok(cuts)
    }

    fn clip_edge(&mut self, [a, b]: [u32; 2], part: Option<usize>, mapped: &[u32]) -> MeshResult<()> {
        let walls = self.active_walls(part);
        let keep_a = walls.iter().flatten().all(|wall| {
            let sign = wall.sides[a as usize];
            if part.is_some() { sign >= 0 } else { sign <= 0 }
        });
        let cuts = Self::edge_cuts([a, b], walls)?;
        if keep_a { self.append(mapped[a as usize])?; }
        for (_, id) in cuts.into_iter().flatten() { self.append(id)?; }
        Ok(())
    }

    fn ring(&mut self, ring: &[u32], input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<()> {
        let edge = [ring[self.index], ring[(self.index + 1) % ring.len()]];
        self.clip_edge(edge, input.subtract.then_some(self.wall), input.mapped)?;
        self.index += 1;
        if self.index == ring.len() {
            if self.corner_count >= 3 {
                result.faces.push(input.face_origin(self.face));
                result.geometry.faces.push(self.corners[..self.corner_count].to_vec());
            } else if self.corner_count == 1 || (self.corner_count == 2 && input.prism.caps().contains(&self.face)) {
                return Err(invalid("longitudinal prism retains a degenerate ring"));
            }
            self.index = 0;
            self.corner_count = 0;
            self.face += 1;
        }
        Ok(())
    }

    fn tail_corner(&mut self, ring: &[u32], end: usize, support: usize, input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<()> {
        let [a, b] = [ring[self.index], ring[(self.index + 1) % ring.len()]];
        let [stock_cap, inward_cap] = [end, 1 - end].map(|end| input.prism.caps()[end]);
        if self.face == stock_cap {
            self.append(a)?;
        } else if self.face == inward_cap {
            let seam = [a, b].map(|id| self.paired[id as usize]);
            self.clip_edge(seam, None, input.mapped)?;
        } else {
            let is_seam = |id: u32| self.upper_cap[id as usize] != (end == 1);
            let seam = [a, b].map(|id| self.paired[id as usize]);
            let cuts = if is_seam(a) && is_seam(b) { Self::edge_cuts(seam, self.active_walls(None))? } else { [None; 2] };
            let first = if is_seam(a) { input.mapped[seam[0] as usize] } else { a };
            self.append(first)?;
            for (_, id) in cuts.into_iter().flatten() { self.append(id)?; }
        }
        self.index += 1;
        if self.index == ring.len() {
            if self.corner_count < 3 { return Err(invalid("mixed prism Difference retains a degenerate seam")); }
            let owner = if self.face == inward_cap { MeshIntersectionFace::ReversedRightPlane(support as u32) }
                else { MeshIntersectionFace::Left(self.face as u32) };
            result.faces.push(owner);
            result.geometry.faces.push(self.corners[..self.corner_count].to_vec());
            self.index = 0;
            self.corner_count = 0;
            self.face += 1;
        }
        Ok(())
    }

    pub(super) fn advance(&mut self, input: LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<bool> {
        let LongitudinalSource { source, planes, side, subtract, epsilon, .. } = input;
        let count = source.positions.len() / 2;
        if !self.edges_ready {
            if self.index < count {
                self.edge(&input, result)?;
                return Ok(false);
            }
            if self.walls[self.wall].as_ref().unwrap().used != 2 {
                return Err(invalid("longitudinal prism support has no complete cut ring"));
            }
            self.index = 0;
            self.wall += 1;
            if self.wall == self.wall_count { self.wall = 0; self.edges_ready = true; }
            return Ok(false);
        }
        if let Some(end) = self.tail {
            if let (Some(support), Some(ring)) = (input.caps[end], source.faces.get(self.face)) {
                self.tail_corner(ring, end, support, &input, result)?;
                return Ok(false);
            }
            if end == 1 { return Ok(true); }
            self.tail = Some(1);
            self.face = 0;
            return Ok(false);
        }
        if let Some(ring) = source.faces.get(self.face) {
            if input.subtract && input.prism.caps().into_iter().zip(input.caps)
                .any(|(face, cap)| face == self.face && cap.is_some()) { self.face += 1; }
            else { self.ring(ring, &input, result)?; }
            return Ok(false);
        }
        let wall = self.walls[self.wall].as_ref().unwrap();
        let plane = planes[wall.plane];
        let [[a, d], [b, c]] = wall.crossings.map(|row| row.points);
        let mut ring = [a, b, c, d];
        let points = ring.map(|id| Vec3(result.geometry.positions[id as usize]));
        let normal = points[1].sub(points[0]).cross(points[2].sub(points[0]));
        let alignment = dot(normal.0.map(f64::from), plane.normal.0.map(f64::from));
        if alignment.abs() <= epsilon * epsilon { return Err(invalid("longitudinal prism cap is degenerate")); }
        if (alignment < 0.) != subtract { ring.reverse(); }
        let id = wall.plane as u32;
        let owner = if subtract { MeshIntersectionFace::ReversedRightPlane(id) } else if side == 0 {
            MeshIntersectionFace::RightPlane(id)
        } else { MeshIntersectionFace::Left(id) };
        result.faces.push(owner);
        result.geometry.faces.push(ring.to_vec());
        if self.wall + 1 < self.wall_count {
            self.wall += 1;
            if subtract { self.face = 0; }
            return Ok(false);
        }
        if subtract && input.caps != [None; 2] {
            self.tail = Some(0);
            self.face = 0;
            return Ok(false);
        }
        Ok(true)
    }
}
