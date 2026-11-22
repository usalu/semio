//! 🪢️ Clips longitudinal supports through certified reflex and interior junctions.

use super::{LongitudinalSource, MeshIntersectionFace, MeshIntersectionSolid, MeshResult, SourceVertex, cross, difference, dot, height, invalid};
use std::collections::{BTreeMap, BTreeSet};

#[path = "🎯️interior-crossing/🦀️.rs"]
mod interior;
use crate::convex_intersection::profile_opening::{CapBridges,certify_links};

type VertexPair = [u32; 2];

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct BoundaryEdge {
    from: VertexPair,
    to: VertexPair,
    owner: MeshIntersectionFace,
}

#[derive(semio_framework_value::RetireOwned)]
struct StockSeam {
    source: [VertexPair; 2],
    points: Vec<VertexPair>,
    owner: MeshIntersectionFace,
}

#[derive(semio_framework_value::RetireOwned)]
struct Polygon {
    vertices: Vec<u32>,
    owner: MeshIntersectionFace,
}

#[derive(semio_framework_value::RetireOwned)]
struct Junction {
    supports: [usize; 2],
    vertices: VertexPair,
}

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct ReflexJunction {
    source: VertexPair,
    vertices: VertexPair,
}

#[derive(Default, semio_framework_value::RetireOwned)]
struct SupportEvents {
    crossings: Vec<VertexPair>,
    contacts: Vec<VertexPair>,
    coincident: bool,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct JoinedClip {
    supports: Vec<usize>,
    junctions: Vec<Junction>,
    reflex: Option<ReflexJunction>,
    bridges: Option<CapBridges>,
    sweep: [f64; 3],
    events: Vec<SupportEvents>,
    edges: Vec<BoundaryEdge>,
    removed: Vec<BoundaryEdge>,
    seams: Vec<StockSeam>,
    source_cursor: usize,
    support_cursor: usize,
    pending: Option<Vec<Polygon>>,
}

fn point(result: &MeshIntersectionSolid, vertex: u32) -> [f32; 3] {
    result.geometry.positions[vertex as usize]
}

fn interpolate(a: [f32; 3], b: [f32; 3], t: f64) -> [f32; 3] {
    let delta = difference(b, a);
    std::array::from_fn(|axis| (f64::from(a[axis]) + t * delta[axis]) as f32)
}

impl JoinedClip {
    pub(super) fn admit(input: &LongitudinalSource<'_>, supports: Vec<usize>, result: &mut MeshIntersectionSolid) -> Option<Self> {
        let count = input.source.positions.len() / 2;
        let position = |id: u32| input.source.positions[id as usize];
        let pair = |i| input.prism.pair(input.source, i);
        let [low, high] = pair(0).map(position);
        let sweep = difference(high, low);
        let mut events: Vec<_> = supports.iter().map(|_| SupportEvents::default()).collect();
        let mut junctions = Vec::new();
        let mut reflex = None;
        if supports.len() == 2 && Self::nonparallel(input, [supports[0], supports[1]]) {
            let side = |id, support: usize| input.classify(height(position(id), input.planes[support]));
            let mut shared = (0..count).filter(|&i| pair(i).into_iter().all(|id| supports.iter().all(|&support| side(id, support) == 0)));
            if let Some(vertex) = shared.next() {
                if shared.next().is_some() { return None; }
                let source = pair(vertex);
                let [previous, next] = [(vertex + count - 1) % count, (vertex + 1) % count].map(pair);
                let [low, center] = source.map(position);
                let turn = cross(difference(center, position(previous[1])), difference(position(next[1]), center));
                if dot(turn, difference(center, low)) >= -input.epsilon * dot(turn, turn).sqrt() { return None; }
                let vertices = source.map(|id| input.mapped[id as usize]);
                for (&support, events) in supports.iter().zip(&mut events) {
                    if side(previous[1], support) * side(next[1], support) < 0 { events.crossings.push(vertices); }
                }
                junctions.push(Junction { supports: [0, 1], vertices });
                reflex = Some(ReflexJunction { source, vertices });
            }
        }
        if reflex.is_none() {
            let mut pairs = Vec::new();
            let mut nonparallel_pairs = 0;
            let mut samples = Vec::new();
            for [a, b] in (0..supports.len()).flat_map(|a| (a+1..supports.len()).map(move |b| [a, b])) {
                let planes = [supports[a], supports[b]];
                if !Self::nonparallel(input, planes) { continue; }
                nonparallel_pairs += 1;
                let crossing = interior::crossing(input, planes, sweep)?;
                let mut outside = false;
                let mut clear = true;
                for (index, &support) in supports.iter().enumerate() {
                    if index == a || index == b { continue; }
                    let signs = crossing.points.map(|point| input.classify(height(point, input.planes[support])));
                    outside |= signs == [1; 2];
                    clear &= signs == [-1; 2];
                }
                if outside { continue; }
                if !clear { return None; }
                pairs.push([a, b]);
                samples.push(([a, b], interior::sample(input, planes, crossing)?));
            }
            let closed = Self::closed_support_ring(supports.len(), &pairs);
            let open = supports.len() <= 3 && pairs.len() + 1 == supports.len() && pairs.len() == nonparallel_pairs;
            if !closed && !open { return None; }
            for (supports, samples) in samples {
                let vertices = samples.map(|(point, recipe)| input.publish_point(result, point, recipe));
                junctions.push(Junction { supports, vertices });
            }
        }
        Some(Self { supports, reflex, junctions, bridges: None, sweep, events, edges: Vec::new(), removed: Vec::new(), seams: Vec::new(), source_cursor: 0, support_cursor: 0, pending: None })
    }

    fn nonparallel(input: &LongitudinalSource<'_>, supports: [usize; 2]) -> bool {
        let [a, b] = supports.map(|i| input.planes[i].normal.0.map(f64::from));
        let separation = cross(a, b);
        dot(separation, separation) > 1e-12
    }

    fn closed_support_ring(count: usize, pairs: &[[usize; 2]]) -> bool {
        if pairs.len() != count || (0..count).any(|i| pairs.iter().filter(|pair| pair.contains(&i)).count() != 2) {
            return false;
        }
        let mut previous = count;
        let mut current = 0;
        for step in 1..=count {
            let next = pairs.iter().find_map(|&[a,b]| match (a == current, b == current) {
                (true, false) if b != previous => Some(b),
                (false, true) if a != previous => Some(a),
                _ => None,
            }).unwrap();
            if next == 0 { return step == count; }
            (previous, current) = (current, next);
        }
        false
    }

    fn has_closed_support_ring(&self) -> bool {
        self.junctions.len() == self.supports.len()
    }

    fn retains_removed_boundary(&self, input: &LongitudinalSource<'_>) -> bool {
        input.subtract && (input.caps != [None; 2] || self.has_closed_support_ring())
    }

    fn masked_contact(&self, input: &LongitudinalSource<'_>, result: &MeshIntersectionSolid, support: usize, ends: [VertexPair; 2]) -> bool {
        self.has_closed_support_ring() && self.supports.iter().any(|&other| {
            other != support && ends.into_iter().flatten().all(|id| {
                height(point(result, id), input.planes[other]) > input.epsilon
            })
        })
    }

    fn original_edge(&mut self, input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<()> {
        let count = input.source.positions.len() / 2;
        let [source_a, source_b] = [self.source_cursor, (self.source_cursor + 1) % count].map(|i| input.prism.pair(input.source, i));
        let [a, b] = [source_a, source_b].map(|pair| pair.map(|id| input.mapped[id as usize]));
        let mut cuts = Vec::new();
        for (index, support) in self.supports.iter().copied().enumerate() {
            let plane = input.planes[support];
            let distances = [a, b].map(|ids| ids.map(|id| height(point(result, id), plane)));
            let signs = distances.map(|pair| pair.map(|distance| input.classify(distance)));
            let [first, last] = signs.map(|pair| pair[1]);
            if signs.iter().any(|pair| pair[0] != pair[1]) {
                return Err(invalid("joined longitudinal support has an unstable profile edge"));
            }
            if first == 0 && last == 0 {
                if !self.masked_contact(input, result, support, [a, b]) {
                    return Err(invalid("joined longitudinal support has a coincident active profile edge"));
                }
                self.events[index].coincident = true;
            }
            if first == 0 && !self.reflex.is_some_and(|reflex| a == reflex.vertices) {
                let previous = input.prism.pair(input.source, (self.source_cursor + count - 1) % count)
                    .map(|id| input.mapped[id as usize]);
                let [low, before] = previous.map(|id| input.classify(height(point(result, id), plane)));
                if before != low {
                    return Err(invalid("joined longitudinal support has an unstable profile edge"));
                }
                let events = if before == 0 || last == 0 {
                    &mut self.events[index].contacts
                } else if before != last {
                    &mut self.events[index].crossings
                } else {
                    let center = point(result, a[1]);
                    let turn = cross(difference(center, point(result, previous[1])), difference(point(result, b[1]), center));
                    if dot(turn, self.sweep) <= input.epsilon * dot(turn, turn).sqrt()
                        && !self.masked_contact(input, result, support, [a; 2]) {
                        return Err(invalid("joined longitudinal support has another reflex contact"));
                    }
                    &mut self.events[index].contacts
                };
                events.push(a);
            }
            if first * last >= 0 { continue; }
            let [da, db] = distances.map(|pair| pair[1]);
            let t = da / (da - db);
            let samples = [0, 1].map(|end| interpolate(point(result, a[end]), point(result, b[end]), t));
            for (end, &p) in samples.iter().enumerate() {
                if height(p, plane).abs() > input.epsilon
                    || input.caps[end].is_some_and(|cap| height(p, input.planes[cap]).abs() > input.epsilon) {
                    return Err(invalid("joined longitudinal edge crossing lost its supporting plane"));
                }
            }
            let existing = cuts.iter().find_map(|&(_, vertices): &(f64, VertexPair)| {
                (vertices.map(|id| point(result, id)) == samples).then_some(vertices)
            });
            let vertices = existing.unwrap_or_else(|| {
                let vertices = std::array::from_fn(|end| {
                    input.publish_point(result, samples[end], input.sample([source_a, source_b], end, t))
                });
                cuts.push((t, vertices));
                vertices
            });
            self.events[index].crossings.push(vertices);
        }
        cuts.sort_by(|a, b| a.0.total_cmp(&b.0));
        let owner = input.source.faces.iter().enumerate().find_map(|(face, ring)| {
            (!input.prism.caps().contains(&face) && ring.contains(&source_a[1]) && ring.contains(&source_b[1])).then_some(input.face_origin(face))
        }).ok_or_else(|| invalid("joined longitudinal edge has no original stock face"))?;
        let points: Vec<_> = std::iter::once(a).chain(cuts.into_iter().map(|(_, ids)| ids)).chain(std::iter::once(b)).collect();
        for segment in points.windows(2) {
            let [from, to] = [segment[0], segment[1]];
            let points = [from[1], to[1]].map(|id| point(result, id));
            let inside = Self::inside_supports(input, &self.supports, None, points)?;
            if inside != input.subtract {
                self.edges.push(BoundaryEdge { from, to, owner });
            } else if self.retains_removed_boundary(input) {
                self.removed.push(BoundaryEdge { from, to, owner });
            }
        }
        if self.retains_removed_boundary(input) {
            self.seams.push(StockSeam { source: [source_a, source_b], points, owner });
        }
        Ok(())
    }

    fn inside_supports(input: &LongitudinalSource<'_>, supports: &[usize], except: Option<usize>, points: [[f32; 3]; 2]) -> MeshResult<bool> {
        if points[0] == points[1] { return Err(invalid("joined longitudinal segment has coincident endpoints")); }
        let midpoint = interpolate(points[0], points[1], 0.5);
        let mut inside = true;
        for &support in supports {
            if Some(support) == except { continue; }
            let plane = input.planes[support];
            let ends = points.map(|point| input.classify(height(point, plane)));
            let middle = input.classify(height(midpoint, plane));
            if middle > 0 && ends.iter().all(|&sign| sign >= 0) { return Ok(false); }
            inside &= middle < 0 && ends.iter().all(|&sign| sign <= 0);
        }
        if !inside { return Err(invalid("joined longitudinal segment cannot be classified")); }
        Ok(true)
    }

    fn support_edges(&mut self, input: &LongitudinalSource<'_>, result: &MeshIntersectionSolid) -> MeshResult<()> {
        let retain_removed = self.retains_removed_boundary(input);
        let capture_bridges = retain_removed && self.has_closed_support_ring() && input.caps.contains(&None);
        let index = self.support_cursor;
        let support = self.supports[index];
        let events = &mut self.events[index];
        let axis_points = &mut events.crossings;
        let coincident = events.coincident;
        if coincident { axis_points.append(&mut events.contacts); }
        if axis_points.len() < 2 || (!coincident && axis_points.len() % 2 != 0) {
            return Err(invalid("joined longitudinal support has no paired profile crossings"));
        }
        let [a, b] = [0, 1].map(|i| point(result, axis_points[i][1]));
        let axis = (0..3).max_by(|&i, &j| (a[i] - b[i]).abs().total_cmp(&(a[j] - b[j]).abs())).unwrap();
        let coordinate = |vertices: VertexPair| f64::from(point(result, vertices[1])[axis]);
        axis_points.sort_by(|&a, &b| coordinate(a).total_cmp(&coordinate(b)));
        if axis_points.windows(2).any(|pair| coordinate(pair[1]) - coordinate(pair[0]) <= input.epsilon) {
            return Err(invalid("joined longitudinal support has ambiguous crossing order"));
        }
        let mut junctions: Vec<_> = self.junctions.iter().filter(|junction| junction.supports.contains(&index))
            .map(|junction| junction.vertices).collect();
        junctions.sort_by(|&a, &b| coordinate(a).total_cmp(&coordinate(b)));
        if junctions.windows(2).any(|pair| coordinate(pair[1]) - coordinate(pair[0]) <= input.epsilon) {
            return Err(invalid("longitudinal support junctions overlap"));
        }
        let stock_profile: Vec<_> = if coincident {
            input.source.faces[input.prism.caps()[1]].iter().map(|&id| input.source.positions[id as usize]).collect()
        } else { Vec::new() };
        let interval = |pair: &[VertexPair]| {
            let [a, b] = [pair[0], pair[1]];
            let inside = if coincident {
                let [a, b] = [a, b].map(|ids| point(result, ids[1]));
                let mid = interpolate(a, b, 0.5);
                let remaining = 1.0 - input.interval[1];
                let sample = std::array::from_fn(|i| (f64::from(mid[i]) + remaining * self.sweep[i]) as f32);
                interior::profile_contains(&stock_profile, sample, input.stock_normal, input.epsilon)
            } else { Some(true) };
            (a, b, inside)
        };
        let intervals: Vec<_> = if coincident {
            axis_points.windows(2).map(interval).collect()
        } else { axis_points.chunks_exact(2).map(interval).collect() };
        if self.reflex.is_none() && junctions.iter().any(|&vertices| !intervals.iter().any(|&(a,b,inside)| {
            let x = coordinate(vertices);
            inside == Some(true) && x - coordinate(a) > input.epsilon && coordinate(b) - x > input.epsilon
        })) {
            return Err(invalid("interior support crossing lies outside a paired stock interval"));
        }
        let owner = if input.subtract { MeshIntersectionFace::ReversedRightPlane(support as u32) }
            else if input.side == 0 { MeshIntersectionFace::RightPlane(support as u32) }
            else { MeshIntersectionFace::Left(support as u32) };
        for (a, b, inside) in intervals {
            if inside == Some(false) { continue; }
            let cuts: Vec<_> = junctions.iter().copied().filter(|&vertices| coordinate(a) < coordinate(vertices) && coordinate(vertices) < coordinate(b)).collect();
            if let [first, second] = cuts.as_slice() {
                if inside == Some(true) && capture_bridges && self.bridges.is_none() {
                    self.bridges = Some(CapBridges { ends: [[a, *first], [b, *second]] });
                }
            }
            let points = std::iter::once(a).chain(cuts).chain(std::iter::once(b));
            for (mut from, mut to) in points.clone().zip(points.skip(1)) {
                let [a, b] = [from, to].map(|ids| point(result, ids[1]));
                if !Self::inside_supports(input, &self.supports, Some(support), [a, b])? { continue; }
                if inside.is_none() { return Err(invalid("active support lies on an uncertain stock boundary")); }
                let alignment = dot(cross(self.sweep, difference(b, a)), input.planes[support].normal.0.map(f64::from));
                if alignment.abs() <= input.epsilon * input.epsilon {
                    return Err(invalid("joined longitudinal cutter edge is degenerate"));
                }
                if (alignment > 0.0) != input.subtract { std::mem::swap(&mut from, &mut to); }
                self.edges.push(BoundaryEdge { from, to, owner });
                if retain_removed {
                    self.removed.push(BoundaryEdge { from: to, to: from, owner });
                }
            }
        }
        Ok(())
    }

    fn cycles(&self, edges: &[BoundaryEdge], input: &LongitudinalSource<'_>, result: &MeshIntersectionSolid) -> MeshResult<Vec<Vec<usize>>> {
        let mut outgoing = BTreeMap::<u32, Vec<usize>>::new();
        for (i, edge) in edges.iter().enumerate() { outgoing.entry(edge.from[1]).or_default().push(i); }
        let mut successor = Vec::with_capacity(edges.len());
        let mut used = BTreeSet::new();
        for edge in edges {
            let end = edge.to[1];
            let next = match outgoing.get(&end).map(Vec::as_slice) {
                Some([one]) => *one,
                Some([first, second]) if self.reflex.is_some_and(|reflex| end == reflex.vertices[1]) => {
                    let through = point(result, end);
                    let incoming = difference(point(result, edge.from[1]), through);
                    let angle = |candidate: usize| {
                        let direction = difference(point(result, edges[candidate].to[1]), through);
                        (-dot(cross(incoming, direction), self.sweep)).atan2(dot(incoming, direction)).rem_euclid(std::f64::consts::TAU)
                    };
                    let (a, b) = (angle(*first), angle(*second));
                    if (a - b).abs() <= 1e-12 { return Err(invalid("joined reflex continuation is ambiguous")); }
                    if a < b { *first } else { *second }
                }
                _ => return Err(invalid("joined longitudinal boundary does not have a closed continuation")),
            };
            if !used.insert(next) { return Err(invalid("joined longitudinal boundary overlaps a continuation")); }
            successor.push(next);
        }
        if successor.is_empty() { return Err(invalid("joined longitudinal cut has no closed material")); }
        let mut cycles = Vec::new();
        let mut visited = vec![false; edges.len()];
        for start in 0..edges.len() {
            if visited[start] { continue; }
            let mut current = start;
            let mut cycle = Vec::new();
            let mut corners = BTreeSet::new();
            while !visited[current] {
                if !corners.insert(edges[current].from[1]) { return Err(invalid("joined longitudinal cap returns to the same reflex point")); }
                visited[current] = true;
                cycle.push(current);
                current = successor[current];
            }
            if current != start || cycle.len() < 3 { return Err(invalid("joined longitudinal cap is not a simple ring")); }
            let vertices: Vec<_> = cycle.iter().map(|&i| edges[i].from[1]).collect();
            self.check_cap(&vertices, input, result)?;
            cycles.push(cycle);
        }
        Ok(cycles)
    }

    fn assemble(&self, input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<Vec<Polygon>> {
        let has_tails = input.subtract && input.caps != [None; 2];
        if input.subtract && self.has_closed_support_ring() && self.removed.iter().all(|edge| {
            matches!(edge.owner, MeshIntersectionFace::ReversedRightPlane(_))
        }) {
            return self.interior_difference(input, result);
        }
        let cycles = self.cycles(&self.edges, input, result)?;
        let mut vertices: Vec<_> = self.edges.iter().map(|edge| edge.from).collect();
        let mut middle = vec![None; self.edges.len()];
        if let Some(ReflexJunction { source, vertices: apex }) = self.reflex {
            let mut branches = self.edges.iter().enumerate().filter_map(|(i, edge)| (edge.from == apex).then_some(i));
            if let (Some(first), Some(second)) = (branches.next(), branches.next()) {
                let [a, b] = source;
                let sample = |t| SourceVertex::Edge { a, b, t };
                for end in 0..2 {
                    if !has_tails || input.caps[end].is_none() {
                        let recipe = if input.caps[end].is_some() { sample(input.interval[end]) }
                            else { SourceVertex::Original(source[end]) };
                        vertices[second][end] = input.publish_point(result, point(result, apex[end]), recipe);
                    }
                }
                if has_tails && input.caps.iter().all(Option::is_some) {
                    let [a, b] = apex.map(|id| point(result, id));
                    let p = interpolate(a, b, 0.5);
                    if p == a || p == b || self.supports.iter().any(|&support| height(p, input.planes[support]).abs() > input.epsilon) {
                        return Err(invalid("joined reflex edge has no stable interior sample"));
                    }
                    let recipe = sample((input.interval[0] + input.interval[1]) * 0.5);
                    for branch in [first, second] { middle[branch] = Some(input.publish_point(result, p, recipe.clone())); }
                }
            }
        }
        let mut polygons = Vec::new();
        for cycle in cycles {
            for (&i, &next) in cycle.iter().zip(cycle.iter().cycle().skip(1)) {
                let [low, high] = vertices[i];
                let [next_low, next_high] = vertices[next];
                let mut ring = vec![low, next_low];
                ring.extend(middle[next]);
                ring.extend([next_high, high]);
                ring.extend(middle[i]);
                polygons.push(Polygon { vertices: ring, owner: self.edges[i].owner });
            }
            for (end, face) in input.prism.caps().into_iter().enumerate() {
                if has_tails && input.caps[end].is_some() { continue; }
                let mut ring: Vec<_> = cycle.iter().map(|&i| vertices[i][end]).collect();
                if end == 0 { ring.reverse(); }
                polygons.push(Polygon { vertices: ring, owner: input.face_origin(face) });
            }
        }
        if has_tails { self.attach_tails(input, result, &mut polygons)?; }
        if has_tails || self.supports.len() > 2 { certify_links(polygons.iter().map(|polygon| polygon.vertices.as_slice()))?; }
        Ok(polygons)
    }

    fn check_cap(&self, ring: &[u32], input: &LongitudinalSource<'_>, result: &MeshIntersectionSolid) -> MeshResult<()> {
        let origin = point(result, ring[0]);
        let mut area = [0.0; 3];
        for pair in ring[1..].windows(2) {
            let [a, b] = [pair[0], pair[1]].map(|id| difference(point(result, id), origin));
            for (sum, part) in area.iter_mut().zip(cross(a, b)) { *sum += part; }
        }
        if dot(area, self.sweep) <= input.epsilon * input.epsilon * dot(self.sweep, self.sweep).sqrt() {
            return Err(invalid("joined longitudinal cap has no positive area"));
        }
        Ok(())
    }

    fn interior_difference(&self, input: &LongitudinalSource<'_>, result: &MeshIntersectionSolid) -> MeshResult<Vec<Polygon>> {
        let cycles = self.cycles(&self.removed, input, result)?;
        let [cycle] = cycles.as_slice() else { return Err(invalid("interior longitudinal cut needs one opening ring")); };
        let mut polygons = Vec::new();
        for seam in &self.seams {
            let [lower, upper] = [0, 1].map(|end| {
                if input.caps[end].is_some() { seam.source.iter().map(|pair| pair[end]).collect::<Vec<_>>() }
                else { seam.points.iter().map(|pair| pair[end]).collect() }
            });
            let vertices = lower.into_iter().chain(upper.into_iter().rev()).collect();
            polygons.push(Polygon { vertices, owner: seam.owner });
        }
        for &index in cycle {
            let BoundaryEdge { from: [a,d], to: [b,c], owner } = self.removed[index];
            polygons.push(Polygon { vertices: vec![a,d,c,b], owner });
        }
        let aperture = if input.caps.contains(&None) {
            let outside: Vec<_> = self.seams.iter().flat_map(|seam| &seam.points[..seam.points.len()-1]).copied().collect();
            let inside: Vec<_> = cycle.iter().rev().map(|&index| self.removed[index].from).collect();
            Some(self.bridges.as_ref().ok_or_else(|| invalid("interior opening has no certified cap bridges"))?.split(&outside, &inside)?)
        } else { None };
        for (end, face) in input.prism.caps().into_iter().enumerate() {
            if let Some(cap) = input.caps[end] {
                let mut vertices: Vec<_> = cycle.iter().map(|&index| self.removed[index].from[end]).collect();
                if end == 1 { vertices.reverse(); }
                polygons.push(Polygon { vertices, owner: MeshIntersectionFace::ReversedRightPlane(cap as u32) });
                polygons.push(Polygon { vertices: input.source.faces[face].clone(), owner: MeshIntersectionFace::Left(face as u32) });
            } else if let Some(rings) = &aperture {
                for ring in rings {
                    let mut vertices: Vec<_> = ring.iter().map(|pair| pair[end]).collect();
                    self.check_cap(&vertices, input, result)?;
                    if end == 0 { vertices.reverse(); }
                    polygons.push(Polygon { vertices, owner: input.face_origin(face) });
                }
            }
        }
        certify_links(polygons.iter().map(|polygon| polygon.vertices.as_slice()))?;
        Ok(polygons)
    }

    fn attach_tails(
        &self, input: &LongitudinalSource<'_>, result: &MeshIntersectionSolid, polygons: &mut Vec<Polygon>,
    ) -> MeshResult<()> {
        let removed = self.cycles(&self.removed, input, result)?;
        for (end, face) in input.prism.caps().into_iter().enumerate() {
            let Some(support) = input.caps[end] else { continue; };
            for cycle in &removed {
                let mut vertices: Vec<_> = cycle.iter().map(|&i| self.removed[i].from[end]).collect();
                if end == 1 { vertices.reverse(); }
                polygons.push(Polygon { vertices, owner: MeshIntersectionFace::ReversedRightPlane(support as u32) });
            }
            for seam in &self.seams {
                let [a, b] = seam.source.map(|pair| pair[end]);
                let vertices = if end == 0 {
                    [a, b].into_iter().chain(seam.points.iter().rev().map(|pair| pair[end])).collect()
                } else {
                    seam.points.iter().map(|pair| pair[end]).chain([b, a]).collect()
                };
                polygons.push(Polygon { vertices, owner: seam.owner });
            }
            polygons.push(Polygon { vertices: input.source.faces[face].clone(), owner: MeshIntersectionFace::Left(face as u32) });
        }
        Ok(())
    }

    pub(super) fn advance(&mut self, input: &LongitudinalSource<'_>, result: &mut MeshIntersectionSolid) -> MeshResult<bool> {
        let count = input.source.positions.len() / 2;
        if self.source_cursor < count {
            self.original_edge(input, result)?;
            self.source_cursor += 1;
        } else if self.support_cursor < self.supports.len() {
            self.support_edges(input, result)?;
            self.support_cursor += 1;
        } else if let Some(pending) = &mut self.pending {
            let Some(polygon) = pending.pop() else { return Ok(true); };
            result.geometry.faces.push(polygon.vertices);
            result.faces.push(polygon.owner);
        } else {
            self.pending = Some(self.assemble(input, result)?);
        }
        Ok(false)
    }
}
