//! 🧱️ Clips a certified prism against independent longitudinal supports.

use super::{MeshIntersectionFace, MeshIntersectionSolid, MeshResult, NestedPrism, Plane, Soup, SourceVertex, difference, dot, height, invalid};
use super::MAX_PROFILE;
use super::super::Vec3;
use std::collections::BTreeMap;

#[path = "🪢️joined-supports/🦀️.rs"]
mod joined_supports;
use joined_supports::JoinedClip;

const MAX_WALLS: usize = 6;

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|axis| {
        let [j, k] = [(axis + 1) % 3, (axis + 2) % 3];
        a[j] * b[k] - a[k] * b[j]
    })
}

pub(super) struct LongitudinalSource<'a> {
    pub source: &'a Soup,
    pub prism: &'a NestedPrism,
    pub planes: &'a [Plane],
    pub mapped: &'a [u32],
    pub caps: [Option<usize>; 2],
    pub stock_normal: [f64; 3],
    pub interval: [f64; 2],
    pub side: usize,
    pub subtract: bool,
    pub epsilon: f64,
}

impl LongitudinalSource<'_> {
    fn classify(&self, distance: f64) -> i8 {
        if distance > self.epsilon { 1 } else if distance < -self.epsilon { -1 } else { 0 }
    }

    fn grazing_edge_side(&self, index: usize, plane: Plane, [a, b]: [[u32; 2]; 2]) -> MeshResult<i8> {
        let point = |id| self.source.positions[id as usize];
        let inward = cross(difference(point(a[1]), point(a[0])), difference(point(b[1]), point(a[1])));
        let alignment = dot(inward, plane.normal.0.map(f64::from));
        if alignment.abs() < (1. - 1e-6) * dot(inward, inward).sqrt() || alignment == 0. {
            return Err(invalid("longitudinal prism coincident wall has an unstable orientation"));
        }
        let material = if alignment > 0. { 1 } else { -1 };
        let count = self.source.positions.len() / 2;
        let adjacent = |forward| (1..count).find_map(|offset| {
            let index = if forward { (index + offset) % count } else { (index + count - offset) % count };
            let signs = self.prism.pair(self.source, index).map(|id| self.classify(height(point(id), plane)));
            (signs != [0; 2]).then_some(signs)
        });
        if [false, true].into_iter().all(|forward| adjacent(forward) == Some([material; 2])) {
            Ok(material)
        } else {
            Err(invalid("longitudinal prism support intersects a coincident profile edge"))
        }
    }

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

    fn publish_point(&self, result: &mut MeshIntersectionSolid, point: [f32; 3], recipe: SourceVertex) -> u32 {
        let id = result.geometry.positions.len() as u32;
        result.geometry.positions.push(point);
        if self.side == 0 { result.points.push(recipe); }
        else { result.right_interpolations.push((id, recipe)); }
        id
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

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd, semio_framework_value::RetireOwned)]
struct Corner {
    vertex: u32,
    branch: bool,
}

impl From<u32> for Corner {
    fn from(vertex: u32) -> Self { Self { vertex, branch: false } }
}

#[derive(Clone, Copy, Default, semio_framework_value::RetireOwned)]
struct Crossing {
    edges: Option<[[u32; 2]; 2]>,
    points: [Corner; 2],
    parameters: [f64; 2],
    side_after: i8,
    middle: Option<u32>,
}

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct Reflex {
    source: [u32; 2],
    next: [u32; 2],
    points: [[Corner; 2]; 2],
    middle: [Option<u32>; 2],
    side: i8,
}

impl Reflex {
    fn crossing(self, side_after: i8) -> Crossing {
        let branch = usize::from(side_after == self.side);
        Crossing { points: self.points[branch], side_after, middle: self.middle[branch], ..Crossing::default() }
    }
}

#[derive(semio_framework_value::RetireOwned)]
struct Wall {
    plane: usize,
    sides: [i8; 2 * MAX_PROFILE],
    crossings: Vec<Crossing>,
    reflexes: Vec<Reflex>,
    grazing_side: i8,
}

impl Wall {
    fn new(plane: usize) -> Self {
        Self { plane, sides: [0; 2 * MAX_PROFILE], crossings: Vec::new(), reflexes: Vec::new(), grazing_side: 0 }
    }

    fn register_reflex(
        &mut self, input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid,
        [source, next]: [[u32; 2]; 2], side: i8, plane: Plane,
    ) -> MeshResult<()> {
        let original = source.map(|id| Corner::from(input.mapped[id as usize]));
        let [a, b] = source;
        let tails = input.subtract && input.caps != [None; 2];
        let mut split = original;
        for (end, &Corner { vertex, .. }) in original.iter().enumerate() {
            let point = result.geometry.positions[vertex as usize];
            if height(point, plane).abs() > input.epsilon {
                return Err(invalid("longitudinal prism reflex point lost its supporting plane"));
            }
            split[end] = if tails && (side < 0 || input.caps[end].is_some()) {
                Corner { vertex, branch: true }
            } else {
                let recipe = input.caps[end].map_or(SourceVertex::Original(source[end]), |_| {
                    SourceVertex::Edge { a, b, t: input.interval[end] }
                });
                input.publish_point(result, point, recipe).into()
            };
        }
        let mut middle = [None; 2];
        if tails && side > 0 && input.caps.iter().all(Option::is_some) {
            let [low, high] = original.map(|corner| result.geometry.positions[corner.vertex as usize]);
            let point = std::array::from_fn(|axis| ((f64::from(low[axis]) + f64::from(high[axis])) * 0.5) as f32);
            if point == low || point == high || height(point, plane).abs() > input.epsilon {
                return Err(invalid("longitudinal prism reflex edge has no stable interior sample"));
            }
            let t = (input.interval[0] + input.interval[1]) * 0.5;
            middle = std::array::from_fn(|_| Some(input.publish_point(result, point, SourceVertex::Edge { a, b, t })));
        }
        self.reflexes.push(Reflex { source, next, points: [original, split], middle, side });
        Ok(())
    }

    fn order_crossings(&mut self, points: &[[f32; 3]], epsilon: f64) -> MeshResult<()> {
        if self.crossings.len() < 2 || self.crossings.len() % 2 != 0 {
            return Err(invalid("longitudinal prism support has no complete cut ring"));
        }
        let point = |crossing: &Crossing| points[crossing.points[0].vertex as usize].map(f64::from);
        let [first, second] = [0, 1].map(|i| point(&self.crossings[i]));
        let range = std::array::from_fn::<_, 3, _>(|axis| (first[axis] - second[axis]).abs());
        let axis = (0..3).max_by(|&a, &b| range[a].total_cmp(&range[b])).unwrap();
        let coordinate = |crossing: &Crossing| point(crossing)[axis];
        self.crossings.sort_by(|a, b| coordinate(a).total_cmp(&coordinate(b)));
        if self.crossings.windows(2).any(|pair| coordinate(&pair[1]) - coordinate(&pair[0]) <= epsilon)
            || self.segments().any(|[a, b]| a.side_after == b.side_after) {
            return Err(invalid("longitudinal prism support has ambiguous crossing order"));
        }
        if self.reflexes.is_empty() { return Ok(()); }
        let contact_coordinate = |contact: &Reflex| f64::from(points[contact.points[0][0].vertex as usize][axis]);
        self.reflexes.sort_by(|a, b| contact_coordinate(a).total_cmp(&contact_coordinate(b)));
        let mut contacts = self.reflexes.iter().peekable();
        let strict = std::mem::take(&mut self.crossings);
        self.crossings.reserve(strict.len() + 2 * self.reflexes.len());
        for pair in strict.chunks_exact(2) {
            let mut start = pair[0];
            let end = pair[1];
            while let Some(contact) = contacts.peek().copied() {
                let at = contact_coordinate(contact);
                if at >= coordinate(&end) { break; }
                if at - coordinate(&start) <= epsilon || coordinate(&end) - at <= epsilon {
                    return Err(invalid("longitudinal prism reflex contact overlaps a cut endpoint"));
                }
                self.crossings.extend([start, contact.crossing(end.side_after)]);
                start = contact.crossing(start.side_after);
                contacts.next();
            }
            self.crossings.extend([start, end]);
        }
        if contacts.peek().is_some() { return Err(invalid("longitudinal prism reflex contact has no cut interval")); }
        Ok(())
    }

    fn segments(&self) -> impl Iterator<Item = [Crossing; 2]> + '_ {
        self.crossings.chunks_exact(2).map(|pair| [pair[0], pair[1]])
    }

    fn on_edge(&self, [a, b]: [u32; 2]) -> Option<(f64, u32)> {
        self.crossings.iter().find_map(|row| {
            row.edges?.iter().enumerate().find_map(|(end, &pair)| {
                let t = row.parameters[end];
                if pair == [a, b] { Some((t, row.points[end].vertex)) }
                else if pair == [b, a] { Some((1. - t, row.points[end].vertex)) }
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
    corners: [Corner; 3 * MAX_PROFILE],
    corner_count: usize,
    wall: usize,
    segment: usize,
    edges_ready: bool,
    paired: [u32; 2 * MAX_PROFILE],
    upper_cap: [bool; 2 * MAX_PROFILE],
    tail: Option<usize>,
    joined: Option<JoinedClip>,
    join_checked: bool,
}

impl LongitudinalClip {
    pub(super) fn new(plane: usize) -> Self {
        let walls = std::array::from_fn(|index| (index == 0).then(|| Wall::new(plane)));
        Self { walls, wall_count: 1, index: 0, face: 0, corners: [Corner::default(); 3 * MAX_PROFILE],
            corner_count: 0, wall: 0, segment: 0, edges_ready: false,
            paired: [0; 2 * MAX_PROFILE], upper_cap: [false; 2 * MAX_PROFILE], tail: None, joined: None, join_checked: false }
    }

    pub(super) fn add_plane(&mut self, plane: usize) -> MeshResult<()> {
        let slot = self.walls.get_mut(self.wall_count)
            .ok_or_else(|| invalid("concave prism exceeds the independent longitudinal support bound"))?;
        *slot = Some(Wall::new(plane));
        self.wall_count += 1;
        Ok(())
    }

    fn edge(&mut self, input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<()> {
        let LongitudinalSource { source, prism, planes, epsilon, .. } = *input;
        let (earlier, remainder) = self.walls[..self.wall_count].split_at_mut(self.wall);
        let (wall, later) = remainder.split_first_mut().unwrap();
        let wall = wall.as_mut().unwrap();
        let plane = planes[wall.plane];
        let inside_other_supports = |point| earlier.iter().chain(later.iter()).flatten()
            .all(|other| height(point, planes[other.plane]) < -epsilon);
        let count = source.positions.len() / 2;
        let endpoints = [self.index, (self.index + 1) % count].map(|i| prism.pair(source, i));
        if self.wall == 0 && input.subtract && input.caps != [None; 2] {
            let [low, high] = endpoints[0];
            self.paired[low as usize] = high;
            self.paired[high as usize] = low;
            self.upper_cap[high as usize] = true;
        }
        let distances = endpoints.map(|pair| pair.map(|id| height(source.positions[id as usize], plane)));
        let sides = distances.map(|pair| pair.map(|d| input.classify(d)));
        let [mut first, last] = sides.map(|pair| pair[0]);
        if sides.iter().any(|pair| pair[0] != pair[1]) {
            return Err(invalid("longitudinal prism cut crosses an unstable edge"));
        }
        if first != 0 {
            wall.grazing_side = 0;
        } else if wall.grazing_side != 0 {
            first = wall.grazing_side;
        } else {
            let previous = prism.pair(source, (self.index + count - 1) % count);
            let [before, peer] = previous.map(|id| input.classify(height(source.positions[id as usize], plane)));
            if before != peer { return Err(invalid("longitudinal prism cut crosses an unstable edge")); }
            if before == 0 || last == 0 {
                let edge = if last == 0 { endpoints } else { [previous, endpoints[0]] };
                first = input.grazing_edge_side(self.index, plane, edge)?;
                wall.grazing_side = first;
            } else if before == last {
                let point = |id| source.positions[id as usize];
                let [bottom, center] = endpoints[0].map(point);
                let normal = cross(difference(center, point(previous[1])), difference(point(endpoints[1][1]), center));
                let turn = dot(normal, difference(center, bottom));
                let tolerance = epsilon * dot(normal, normal).sqrt();
                if turn.abs() <= tolerance { return Err(invalid("longitudinal prism tangent has an unstable profile turn")); }
                if turn < 0. {
                    let tails = input.subtract && input.caps != [None; 2];
                    let samples = endpoints[0].map(|id| result.geometry.positions[input.mapped[id as usize] as usize]);
                    if !samples.into_iter().all(inside_other_supports) {
                        return Err(invalid("longitudinal prism reflex contact meets another support"));
                    }
                    if tails || (last > 0) == input.subtract {
                        wall.register_reflex(input, result, endpoints, last, plane)?;
                    }
                }
                first = last;
            }
        }
        for id in endpoints[0] {
            if first > 0 && earlier.iter().flatten().any(|other| other.sides[id as usize] > 0) {
                return Err(invalid("longitudinal prism supports remove overlapping material"));
            }
            wall.sides[id as usize] = first;
        }
        if first == 0 || first * last < 0 {
            let mut crossing = Crossing { edges: Some([[0; 2]; 2]), side_after: last, ..Crossing::default() };
            for end in 0..2 {
                let [a, b] = endpoints.map(|pair| pair[end]);
                let [da, db] = distances.map(|pair| pair[end]);
                let vertex = first == 0;
                let t = if vertex { 0. } else { da / (da - db) };
                let [start, finish] = [a, b].map(|id| result.geometry.positions[input.mapped[id as usize] as usize].map(f64::from));
                let point = std::array::from_fn(|axis| (start[axis] + (finish[axis] - start[axis]) * t) as f32);
                if height(point, plane).abs() > epsilon || !inside_other_supports(point) {
                    return Err(invalid("longitudinal prism point lost its supporting plane"));
                }
                let id = if vertex { input.mapped[a as usize] } else {
                    input.publish_point(result, point, input.sample(endpoints, end, t))
                };
                crossing.edges.as_mut().unwrap()[end] = [a, if vertex { a } else { b }];
                crossing.points[end] = id.into();
                crossing.parameters[end] = t;
            }
            wall.crossings.push(crossing);
        }
        self.index += 1;
        Ok(())
    }

    fn append(&mut self, corner: impl Into<Corner>) -> MeshResult<()> {
        *self.corners.get_mut(self.corner_count).ok_or_else(|| invalid("longitudinal prism ring exceeds its source bound"))? = corner.into();
        self.corner_count += 1;
        Ok(())
    }

    fn reflex(&self, vertex: u32, part: Option<usize>) -> Option<(Reflex, usize)> {
        self.active_walls(part).iter().flatten().flat_map(|wall| &wall.reflexes).find_map(|contact| {
            contact.source.iter().position(|&id| id == vertex).map(|end| (*contact, end))
        })
    }

    fn append_reflex_cap(&mut self, contact: Reflex, end: usize, forward: bool, outside: bool) -> MeshResult<()> {
        let [before, after] = contact.points.map(|ids| ids[end]);
        if after.branch && (contact.side > 0) != outside { return self.append(before); }
        let ordered = if forward { [before, after] } else { [after, before] };
        for corner in ordered { self.append(corner)?; }
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

    fn append_cuts(&mut self, edge: [u32; 2], part: Option<usize>) -> MeshResult<()> {
        for (_, id) in Self::edge_cuts(edge, self.active_walls(part))?.into_iter().flatten() {
            self.append(id)?;
        }
        Ok(())
    }

    fn clip_edge(&mut self, [a, b]: [u32; 2], part: Option<usize>, mapped: &[u32]) -> MeshResult<()> {
        let walls = self.active_walls(part);
        let keep_a = walls.iter().flatten().all(|wall| {
            let sign = wall.sides[a as usize];
            if part.is_some() { sign >= 0 } else { sign <= 0 }
        });
        if keep_a { self.append(mapped[a as usize])?; }
        self.append_cuts([a, b], part)
    }

    fn publish_ring(
        &self, owner: MeshIntersectionFace, cap: Option<(usize, bool)>, outside: Option<usize>,
        result: &mut MeshIntersectionSolid,
    ) -> MeshResult<()> {
        let corners = &self.corners[..self.corner_count];
        let walls = self.active_walls(outside);
        if let Some((end, forward)) = cap.filter(|_| walls.iter().flatten().any(|wall| wall.crossings.len() > 2)) {
            let indices: BTreeMap<_, _> = corners.iter().copied().enumerate().map(|(index, id)| (id, index)).collect();
            if corners.len() != indices.len() { return Err(invalid("longitudinal prism cap repeats a corner")); }
            let mut next: Vec<_> = (1..=corners.len()).map(|i| i % corners.len()).collect();
            for [a, b] in walls.iter().flatten().flat_map(Wall::segments) {
                let [exit, entry] = if (a.side_after > 0) == (forward != outside.is_some()) { [a, b] } else { [b, a] };
                let index = |crossing: Crossing| {
                    let mut corner = crossing.points[end];
                    if corner.branch && (crossing.side_after > 0) != outside.is_some() { corner.branch = false; }
                    indices.get(&corner).copied().ok_or_else(|| invalid("longitudinal prism cap lost a support endpoint"))
                };
                next[index(exit)?] = index(entry)?;
            }
            let shared = corners.iter().any(|corner| corner.branch);
            for start in 0..corners.len() {
                if next[start] == usize::MAX { continue; }
                let mut ring = Vec::new();
                let mut current = start;
                loop {
                    let vertex = corners[current].vertex;
                    if shared && ring.contains(&vertex) {
                        return Err(invalid("longitudinal prism cap returns to a shared reflex corner"));
                    }
                    ring.push(vertex);
                    current = std::mem::replace(&mut next[current], usize::MAX);
                    if current == start { break; }
                    if current == usize::MAX || next[current] == usize::MAX {
                        return Err(invalid("longitudinal prism cap has an open boundary"));
                    }
                }
                if ring.len() < 3 { return Err(invalid("longitudinal prism cap retains a degenerate component")); }
                result.geometry.faces.push(ring);
                result.faces.push(owner);
            }
        } else {
            result.geometry.faces.push(corners.iter().map(|corner| corner.vertex).collect());
            result.faces.push(owner);
        }
        Ok(())
    }

    fn ring(&mut self, ring: &[u32], input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<()> {
        let [a, b] = [ring[self.index], ring[(self.index + 1) % ring.len()]];
        let cap = input.prism.caps().contains(&self.face);
        let part = input.subtract.then_some(self.wall);
        let reflex = self.reflex(a, part).filter(|(contact, _)| cap || (contact.side > 0) == input.subtract);
        if let Some((contact, end)) = reflex {
            if cap {
                self.append_reflex_cap(contact, end, end == 1, input.subtract)?;
            } else {
                let previous = ring[(self.index + ring.len() - 1) % ring.len()];
                let branch = usize::from([previous, b].contains(&contact.next[end]));
                self.append(contact.points[branch][end])?;
                if let Some(middle) = contact.middle[branch].filter(|_| b == contact.source[1 - end]) {
                    self.append(middle)?;
                }
            }
            self.append_cuts([a, b], part)?;
        } else {
            self.clip_edge([a, b], part, input.mapped)?;
        }
        self.index += 1;
        if self.index == ring.len() {
            if self.corner_count >= 3 {
                let cap = input.prism.caps().iter().position(|&face| face == self.face).map(|end| (end, end == 1));
                self.publish_ring(input.face_origin(self.face), cap, input.subtract.then_some(self.wall), result)?;
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
            if let Some((contact, seam_end)) = self.reflex(seam[0], None) {
                self.append_reflex_cap(contact, seam_end, end == 0, false)?;
                self.append_cuts(seam, None)?;
            } else {
                self.clip_edge(seam, None, input.mapped)?;
            }
        } else {
            let [a_moved, b_moved] = [a, b].map(|id| self.upper_cap[id as usize] != (end == 1));
            let seam = [a, b].map(|id| self.paired[id as usize]);
            let first = if a_moved { input.mapped[seam[0] as usize] } else { a };
            self.append(first)?;
            if a_moved && b_moved { self.append_cuts(seam, None)?; }
        }
        self.index += 1;
        if self.index == ring.len() {
            if self.corner_count < 3 { return Err(invalid("mixed prism Difference retains a degenerate seam")); }
            let owner = if self.face == inward_cap { MeshIntersectionFace::ReversedRightPlane(support as u32) }
                else { MeshIntersectionFace::Left(self.face as u32) };
            let cap = (self.face == inward_cap).then_some((end, end == 0));
            self.publish_ring(owner, cap, None, result)?;
            self.index = 0;
            self.corner_count = 0;
            self.face += 1;
        }
        Ok(())
    }

    pub(super) fn advance(&mut self, input: LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<bool> {
        if !self.join_checked {
            self.join_checked = true;
            if (2..=MAX_WALLS).contains(&self.wall_count) {
                let supports = self.walls[..self.wall_count].iter().flatten().map(|wall| wall.plane).collect();
                self.joined = JoinedClip::admit(&input, supports, result);
            }
        }
        if let Some(joined) = &mut self.joined { return joined.advance(&input, result); }
        let LongitudinalSource { source, planes, side, subtract, epsilon, .. } = input;
        let count = source.positions.len() / 2;
        if !self.edges_ready {
            if self.index < count {
                self.edge(&input, result)?;
                return Ok(false);
            }
            self.walls[self.wall].as_mut().unwrap().order_crossings(&result.geometry.positions, epsilon)?;
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
        let [start, end] = [self.segment, self.segment + 1].map(|i| wall.crossings[i]);
        let [[a, d], [b, c]] = [start, end].map(|crossing| crossing.points.map(|corner| corner.vertex));
        let mut ring = Vec::with_capacity(4 + usize::from(start.middle.is_some()) + usize::from(end.middle.is_some()));
        ring.extend([a, b]);
        ring.extend(end.middle);
        ring.extend([c, d]);
        ring.extend(start.middle);
        let points = [a, b, c].map(|id| Vec3(result.geometry.positions[id as usize]));
        let normal = points[1].sub(points[0]).cross(points[2].sub(points[0]));
        let alignment = dot(normal.0.map(f64::from), plane.normal.0.map(f64::from));
        if alignment.abs() <= epsilon * epsilon { return Err(invalid("longitudinal prism cap is degenerate")); }
        if (alignment < 0.) != subtract { ring.reverse(); }
        let id = wall.plane as u32;
        let owner = if subtract { MeshIntersectionFace::ReversedRightPlane(id) } else if side == 0 {
            MeshIntersectionFace::RightPlane(id)
        } else { MeshIntersectionFace::Left(id) };
        result.faces.push(owner);
        result.geometry.faces.push(ring);
        self.segment += 2;
        if self.segment < wall.crossings.len() { return Ok(false); }
        self.segment = 0;
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
