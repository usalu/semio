//! ✂️ Sections original planar shells through their reciprocal line edges.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{
    arena::{EdgeId, ShellId, SolidId, VertexId},
    contract::{BoxSectionDraft, SectionPolygon},
    topology::Body,
};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec3, matrix::Frame3, predicates::{Orient, orient2d, ray_event}};

#[path = "🌗️spherical-strip/🦀️.rs"]
mod spherical_strip;
pub use spherical_strip::spherical_strip_section;

const MAX_FACES: usize = 18;
const MAX_VERTICES: usize = 32;
const MAX_EDGES: usize = 48;
const MAX_CROSSINGS: usize = MAX_EDGES + MAX_VERTICES;
const MAX_VOIDS:usize=4;

/// 🔢️ Maximum number of bounded contours the section walk can retain.
pub const MAX_SECTION_REGIONS: usize = MAX_VOIDS + 1;

/// 📐️ Holds a convex edge cursor or the connected material regions of the cut.
pub enum PlanarSection {
    Convex(BoxSectionDraft),
    Polygon(SectionPolygon),
    Region(PlanarSectionRegion),
    Regions(PlanarSectionRegions),
}

/// 🗺️ Owns the disconnected material regions in original contour order.
pub struct PlanarSectionRegions([Option<PlanarSectionRegion>; MAX_SECTION_REGIONS]);

impl PlanarSectionRegions {
    pub fn into_regions(self) -> impl Iterator<Item = PlanarSectionRegion> {
        self.0.into_iter().flatten()
    }

    fn one(region: PlanarSectionRegion) -> Self {
        let mut regions = Self([const { None }; MAX_SECTION_REGIONS]);
        regions.0[0] = Some(region);
        regions
    }
}

/// 🕳️ A planar Section region with the original oriented material boundary and void rings.
pub struct PlanarSectionRegion {
    outer: SectionPolygon,
    holes: [Option<SectionPolygon>; MAX_VOIDS],
}

impl PlanarSectionRegion {
    pub fn into_rings(self) -> (SectionPolygon, [Option<SectionPolygon>; MAX_VOIDS]) { (self.outer, self.holes) }
}

fn section_plane(origin:[f64;3],normal:[f64;3])->Result<(Pnt3,Vec3),&'static str>{
    if !origin.into_iter().chain(normal).all(f64::is_finite){return Err("section plane must be finite");}
    let scale = normal.into_iter().map(f64::abs).fold(0., f64::max);
    let axis = Vec3::from_array(normal.map(|value| value / scale)).normalized()
        .ok_or("section plane requires a finite nonzero normal")?;
    Ok((Pnt3::from_array(origin),axis))
}

#[derive(Clone, Copy)]
struct PlanarFace {
    edges: [u8; MAX_EDGES],
    offsets: [u8; MAX_VOIDS + 2],
    ring_count: usize,
}

impl PlanarFace {
    const EMPTY: Self = Self { edges: [0; MAX_EDGES], offsets: [0; MAX_VOIDS + 2], ring_count: 0 };

    fn edges(&self) -> &[u8] { &self.edges[..usize::from(self.offsets[self.ring_count])] }

    fn rings(&self) -> impl Iterator<Item = &[u8]> {
        self.offsets[..=self.ring_count].windows(2).map(|range| {
            &self.edges[usize::from(range[0])..usize::from(range[1])]
        })
    }
}

struct PlanarShell {
    points: [Pnt3; MAX_VERTICES],
    supports: [(Pnt3, Vec3); MAX_FACES],
    endpoints: [[u8; 2]; MAX_EDGES],
    faces: [PlanarFace; MAX_FACES],
    vertex_count: usize,
    face_count: usize,
    edge_count: usize,
}

impl PlanarShell {
    fn crosses_interior(&self,anchor:Pnt3,axis:Vec3,tolerance:f64)->bool {
        let distances=self.points[..self.vertex_count].iter().map(|&point|(point-anchor).dot(axis));
        distances.clone().any(|d|d>tolerance) && distances.clone().any(|d|d< -tolerance)
    }
}

fn planar_shell(body: &Body, id: ShellId, tolerance: f64) -> Option<PlanarShell> {
    let shell = body.shells.get(id)?;
    if !(4..=MAX_FACES).contains(&shell.faces.len()) { return None; }
    let mut supports = [(Pnt3::default(), Vec3::ZERO); MAX_FACES];
    let mut vertices = [None::<VertexId>; MAX_VERTICES];
    let mut points = [Pnt3::default(); MAX_VERTICES];
    let mut edges = [None::<(EdgeId, bool, bool)>; MAX_EDGES];
    let mut endpoints = [[0; 2]; MAX_EDGES];
    let mut faces = [PlanarFace::EMPTY; MAX_FACES];
    let mut vertex_count = 0;
    let mut edge_count = 0;
    for (face_index, &id) in shell.faces.iter().enumerate() {
        let face = body.faces.get(id)?;
        let Surface::Plane { frame } = body.surfaces.get(face.surface)? else { return None; };
        if face.inners.len() > MAX_VOIDS { return None; }
        let outward = frame.x.cross(frame.y).normalized()?;
        let outward = if face.flipped { -outward } else { outward };
        supports[face_index] = (frame.origin, outward);
        let boundary = &mut faces[face_index];
        for loop_id in std::iter::once(face.outer?).chain(face.inners.iter().copied()) {
            let offset = usize::from(boundary.offsets[boundary.ring_count]);
            let ring = body.loops.get(loop_id)?;
            if ring.face != id { return None; }
            let mut index = ring.first;
            let mut previous = None;
            let mut start = None;
            let mut count = 0;
            loop {
                if count == MAX_VERTICES { return None; }
                let op = body.coedges.get(index)?;
                let edge = body.edges.get(op.edge)?;
                if op.loop_id != loop_id || !matches!(body.curves3.get(edge.curve)?, Curve3::Line { .. }) { return None; }
                let (from, to) = if op.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) };
                if from == to || previous.is_some_and(|end| end != from) { return None; }
                start.get_or_insert(from);
                previous = Some(to);
                let mut vertex_index = |id| {
                    if let Some(position) = vertices[..vertex_count].iter().position(|&existing| existing == Some(id)) {
                        return Some(position as u8);
                    }
                    *vertices.get_mut(vertex_count)? = Some(id);
                    points[vertex_count] = body.vertices.get(id)?.position;
                    vertex_count += 1;
                    Some((vertex_count - 1) as u8)
                };
                let a = vertex_index(from)?;
                let b = vertex_index(to)?;
                let z = (points[usize::from(a)] - frame.origin).dot(outward).abs();
                if !(z <= tolerance) { return None; }
                let edge_index = if let Some(position) = edges[..edge_count].iter().position(|entry| entry.is_some_and(|e| e.0 == op.edge)) {
                    let (_, forward, paired) = edges[position].as_mut()?;
                    if *paired || *forward == op.forward { return None; }
                    *paired = true;
                    position
                } else {
                    let position = edge_count;
                    *edges.get_mut(position)? = Some((op.edge, op.forward, false));
                    endpoints[position] = [a, b];
                    edge_count += 1;
                    position
                };
                *boundary.edges.get_mut(offset + count)? = edge_index as u8;
                count += 1;
                index = op.next;
                if index == ring.first { break; }
            }
            if count < 3 || start != previous { return None; }
            boundary.ring_count += 1;
            boundary.offsets[boundary.ring_count] = (offset + count) as u8;
        }
    }
    if edges[..edge_count].iter().any(|entry| !entry.is_some_and(|edge| edge.2)) { return None; }
    let shell = PlanarShell { points, supports, endpoints, faces, vertex_count, face_count: shell.faces.len(), edge_count };
    closed_planar_manifold(&shell).then_some(shell)
}

fn edge_link_connected(edges: u64, links: &[u64; MAX_EDGES]) -> bool {
    let mut visit = edges & edges.wrapping_neg();
    let mut seen = 0;
    while visit != 0 {
        let edge = visit.trailing_zeros() as usize;
        seen |= 1 << edge;
        visit = (visit | (links[edge] & edges)) & !seen;
    }
    seen == edges
}

fn closed_planar_manifold(shell: &PlanarShell) -> bool {
    let faces = &shell.faces[..shell.face_count];
    let hole_count: usize = faces.iter().map(|face| face.ring_count - 1).sum();
    let Some(twice_genus) = (shell.edge_count + 2 + hole_count).checked_sub(shell.vertex_count + shell.face_count) else { return false; };
    if twice_genus % 2 != 0 { return false; }
    let mut incident = [0_u64; MAX_VERTICES];
    for (edge, &vertices) in shell.endpoints[..shell.edge_count].iter().enumerate() {
        for vertex in vertices { incident[usize::from(vertex)] |= 1 << edge; }
    }
    let mut links = [0_u64; MAX_EDGES];
    for ring in faces.iter().flat_map(PlanarFace::rings) {
        for (&a, &b) in ring.iter().zip(ring.iter().cycle().skip(1)) {
            if a == b { return false; }
            links[usize::from(a)] |= 1 << b;
            links[usize::from(b)] |= 1 << a;
        }
    }
    let mut connected = links;
    for face in faces {
        for ring in face.rings().skip(1) {
            let (outer, inner) = (usize::from(face.edges[0]), usize::from(ring[0]));
            connected[outer] |= 1 << inner;
            connected[inner] |= 1 << outer;
        }
    }
    if !edge_link_connected((1 << shell.edge_count) - 1, &connected) { return false; }
    incident[..shell.vertex_count].iter().all(|&edges| {
        let degree = edges.count_ones();
        let neighbors = (degree.saturating_sub(1)).min(2);
        degree >= 2
            && links.iter().enumerate().all(|(i, &adjacent)| edges & (1 << i) == 0 || (adjacent & edges).count_ones() == neighbors)
            && edge_link_connected(edges, &links)
    })
}

/// 🧭️ Opens a bounded section from original planar shells and reciprocal line edges.
pub fn planar_section(
    body: &Body, id: SolidId, origin: [f64; 3], normal: [f64; 3], tolerance: f64,
) -> Option<Result<PlanarSection, &'static str>> {
    let solid = body.solids.get(id)?;
    if solid.inners.len() > MAX_VOIDS { return None; }
    let shell = planar_shell(body, solid.outer, tolerance)?;
    let (anchor,axis)=match section_plane(origin,normal){Ok(plane)=>plane,Err(error)=>return Some(Err(error))};
    let mut section = match section_on_shell(&shell, origin, normal, anchor, axis, tolerance)? {
        Ok(section) => section,
        Err(error) => return Some(Err(error)),
    };
    let frame = Frame3::from_normal(anchor, axis)?;
    for &id in &solid.inners {
        let inner = planar_shell(body, id, tolerance)?;
        if !inner.crosses_interior(anchor, axis, tolerance) { continue; }
        let void = section_on_shell(&inner, origin, normal.map(|value| -value), anchor, -axis, tolerance)?;
        section = match void.and_then(|void| subtract_void_section(section, void, &frame)) {
            Ok(section) => section,
            Err(error) => return Some(Err(error)),
        };
    }
    Some(Ok(section))
}

fn subtract_void_section(material: PlanarSection, void: PlanarSection, frame: &Frame3) -> Result<PlanarSection, &'static str> {
    let mut material = finished_regions(material)?;
    for void in finished_regions(void)?.into_regions() {
        material.add_void(void.outer, frame)?;
        for island in void.holes.into_iter().flatten() {
            let slot = material.0.iter_mut().find(|region| region.is_none()).ok_or("section material region limit")?;
            *slot = Some(PlanarSectionRegion { outer: island, holes: [const { None }; MAX_VOIDS] });
        }
    }
    material.finish()
}

fn section_on_shell(
    shell:&PlanarShell,origin:[f64;3],normal:[f64;3],anchor:Pnt3,axis:Vec3,tolerance:f64,
)->Option<Result<PlanarSection,&'static str>>{
    let (points, endpoints, supports) = (
        &shell.points[..shell.vertex_count], &shell.endpoints[..shell.edge_count], &shell.supports[..shell.face_count],
    );
    if shell.faces[..shell.face_count].iter().all(|face| face.ring_count == 1)
        && supports.iter().all(|&(anchor, axis)| points.iter().all(|&point| (point - anchor).dot(axis) <= tolerance))
    {
        return Some(BoxSectionDraft::convex_edges(points, endpoints, origin, normal).map(PlanarSection::Convex));
    }
    let crossings = section_crossings(points, endpoints, anchor, axis, tolerance)?;
    let crossing = &crossings.points;
    let distances = &crossings.sides[..points.len()];
    if distances.iter().all(|&side| side >= 0.) || distances.iter().all(|&side| side <= 0.) {
        return Some(Err("section plane must cross the interior of the solid"));
    }
    let mut links = [[None; 2]; MAX_CROSSINGS];
    let mut join=|a:u8,b:u8|{
        for (from,to) in [(a,b),(b,a)]{
            let neighbors=&mut links[usize::from(from)];
            if neighbors.contains(&Some(to)){return None;}
            *neighbors.iter_mut().find(|link|link.is_none())?=Some(to);
        }
        Some(())
    };
    let mut aligned_sides=[0;MAX_EDGES];
    let mut coplanar_outside=[0;MAX_EDGES];
    let aligned=|edge:u8|crossings.on_edge(endpoints[usize::from(edge)]);
    for (index, &(_, outward)) in supports.iter().enumerate() {
        let face = &shell.faces[index];
        if face.edges().iter().copied().any(|edge|aligned(edge)){
            let sides=face.edges().iter().flat_map(|&edge|endpoints[usize::from(edge)]).fold(0,|mask,vertex|{
                let distance=crossings.sides[usize::from(vertex)];
                mask|u8::from(distance>0.)|u8::from(distance<0.)<<1
            });
            if sides==3 {return None;}
            let outside = if outward.dot(axis)>0. {1} else {2};
            for edge in face.edges().iter().copied().filter(|&edge|aligned(edge)){
                if sides==0 { coplanar_outside[usize::from(edge)]|=outside; }
                else { aligned_sides[usize::from(edge)]|=sides; }
            }
            continue;
        }
        let mut hits = [0; MAX_CROSSINGS];
        let mut size = 0;
        for ring in face.rings() {
            for (&previous, &edge) in std::iter::once(ring.last()?).chain(ring).zip(ring) {
                if crossing[usize::from(edge)].is_some() { *hits.get_mut(size)? = edge; size += 1; }
                if let Some(vertex) = crossings.corner(endpoints[usize::from(previous)], endpoints[usize::from(edge)]) {
                    *hits.get_mut(size)? = vertex;
                    size += 1;
                }
            }
        }
        if size == 0 { continue; }
        if size % 2 != 0 { return None; }
        let direction = outward.cross(axis).normalized()?;
        let base = crossing[usize::from(hits[0])]?;
        let coordinate = |edge: u8| (crossing[usize::from(edge)].expect("crossing ring edge") - base).dot(direction);
        hits[..size].sort_unstable_by(|&a, &b| coordinate(a).total_cmp(&coordinate(b)));
        for pair in hits[..size].chunks_exact(2) {
            let (a, b) = (pair[0], pair[1]);
            if coordinate(b) - coordinate(a) <= tolerance { return None; }
            join(a,b)?;
        }
    }
    for (index,&edge) in endpoints.iter().enumerate(){
        let sides=aligned_sides[index];
        if sides==3 || (sides!=0 && (sides|coplanar_outside[index])==3){
            let [a,b]=edge.map(|vertex|(MAX_EDGES+usize::from(vertex)) as u8);
            join(a,b)?;
        }
    }
    if crossing[..MAX_EDGES].iter().zip(&links).any(|(point,link)|point.is_some()&&link[0].is_none()){return None;}
    ordered_sections(crossing,&links,&Frame3::from_normal(anchor,axis)?,tolerance)
}

struct SectionCrossings {
    points:[Option<Pnt3>;MAX_CROSSINGS],
    sides:[f64;MAX_VERTICES],
}

impl SectionCrossings {
    fn on_edge(&self,[a,b]:[u8;2])->bool{
        self.sides[usize::from(a)]==0.&&self.sides[usize::from(b)]==0.
    }

    fn corner(&self,previous:[u8;2],current:[u8;2])->Option<u8>{
        let at=current.into_iter().find(|vertex|previous.contains(vertex))?;
        if self.sides[usize::from(at)]!=0.{return None;}
        let side=|edge:[u8;2]|self.sides[usize::from(if edge[0]==at{edge[1]}else{edge[0]})];
        (side(previous).is_sign_positive()!=side(current).is_sign_positive()).then_some((MAX_EDGES+usize::from(at)) as u8)
    }
}

fn section_crossings(points:&[Pnt3],edges:&[[u8;2]],anchor:Pnt3,normal:Vec3,tolerance:f64)->Option<SectionCrossings>{
    let mut result=SectionCrossings{points:[None;MAX_CROSSINGS],sides:[0.;MAX_VERTICES]};
    for (i,&point) in points.iter().enumerate(){
        let offset=(point-anchor).dot(normal);
        if !offset.is_finite(){return None;}
        if offset.abs()<=tolerance{
            result.points[MAX_EDGES+i]=Some(point+normal*(-offset));
        }else{
            result.sides[i]=offset;
        }
    }
    for (index,&[a,b]) in edges.iter().enumerate(){
        let (a,b)=(usize::from(a),usize::from(b));
        let (left,right)=(result.sides[a],result.sides[b]);
        if left==0.||right==0.||left.is_sign_positive()==right.is_sign_positive(){continue;}
        let scale=left.abs().max(right.abs());
        let (left,right)=(left/scale,right/scale);
        result.points[index]=Some(points[a]+(points[b]-points[a])*(left/(left-right)));
    }
    Some(result)
}

fn ordered_sections(
    crossing:&[Option<Pnt3>;MAX_CROSSINGS],links:&[[Option<u8>;2];MAX_CROSSINGS],frame:&Frame3,tolerance:f64,
)->Option<Result<PlanarSection,&'static str>>{
    let mut rings: [Option<(SectionPolygon, f64)>; MAX_SECTION_REGIONS] = [const { None }; MAX_SECTION_REGIONS];
    let mut seen=[false;MAX_CROSSINGS];
    for first in 0..MAX_CROSSINGS {
        if seen[first]||links[first][0].is_none(){continue;}
        let mut polygon=[Pnt3::default();MAX_CROSSINGS];
        let (mut previous,mut index,mut count)=(first,first,0);
        loop {
            if seen[index]{return None;}
            seen[index]=true;
            *polygon.get_mut(count)?=crossing[index]?;
            count+=1;
            let [a,b]=links[index].map(|id|id.map(usize::from));
            let next=if a?==previous{b?}else{a?};
            if next==first{break;}
            (previous,index)=(index,next);
        }
        if count<3{return None;}
        let origin=polygon[0];
        let signed=polygon[1..count].windows(2).map(|pair|(pair[0]-origin).cross(pair[1]-origin).dot(frame.z)).sum::<f64>();
        if !signed.is_finite()||signed.abs()<=tolerance*tolerance{return None;}
        *rings.iter_mut().find(|ring|ring.is_none())?=Some((SectionPolygon::ordered_ring(&polygon[..count])?,signed));
    }
    Some(group_rings(rings, frame))
}

fn oriented_ring((ring, signed): (SectionPolygon, f64), outer: bool) -> SectionPolygon {
    if (signed > 0.) == outer { return ring; }
    let mut points = [Pnt3::default(); MAX_CROSSINGS];
    for (i, point) in points[..ring.count()].iter_mut().enumerate() {
        *point = Pnt3::from_array(ring.point(ring.count() - 1 - i).expect("original contour ordinal"));
    }
    SectionPolygon::ordered_ring(&points[..ring.count()]).expect("original contour extent")
}

fn group_rings(
    mut rings: [Option<(SectionPolygon, f64)>; MAX_SECTION_REGIONS], frame: &Frame3,
) -> Result<PlanarSection, &'static str> {
    let invalid = "section contours must have disjoint boundaries";
    let charts: [Option<RingChart>; MAX_SECTION_REGIONS] =
        std::array::from_fn(|i| rings[i].as_ref().map(|(ring, _)| RingChart::new(ring, frame)));
    let mut depths = [0; MAX_SECTION_REGIONS];
    let mut parents = [None; MAX_SECTION_REGIONS];
    for (i, chart) in charts.iter().enumerate() {
        let Some(chart) = chart else { continue; };
        let mut parent_area = f64::INFINITY;
        for (j, other) in charts.iter().enumerate() {
            let Some(other) = other else { continue; };
            if i == j { continue; }
            if i < j && chart.intersects(other) { return Err(invalid); }
            if other.contains(chart.points[0]) {
                depths[i] += 1;
                let area = rings[j].as_ref().expect("charted contour").1.abs();
                if area < parent_area {
                    parents[i] = Some(j);
                    parent_area = area;
                }
            }
        }
    }
    let mut regions = PlanarSectionRegions([const { None }; MAX_SECTION_REGIONS]);
    for i in 0..MAX_SECTION_REGIONS {
        if depths[i] % 2 == 0 {
            regions.0[i] = rings[i].take().map(|ring| PlanarSectionRegion {
                outer: oriented_ring(ring, true), holes: [const { None }; MAX_VOIDS],
            });
        }
    }
    for (i, ring) in rings.into_iter().enumerate() {
        let Some(ring) = ring else { continue; };
        let region = parents[i].and_then(|parent| regions.0[parent].as_mut()).ok_or(invalid)?;
        *region.holes.iter_mut().find(|hole| hole.is_none()).ok_or(invalid)? = Some(oriented_ring(ring, false));
    }
    regions.finish()
}

fn finished_regions(section: PlanarSection) -> Result<PlanarSectionRegions, &'static str> {
    let outer = match section {
        PlanarSection::Regions(regions) => return Ok(regions),
        PlanarSection::Region(region) => return Ok(PlanarSectionRegions::one(region)),
        PlanarSection::Polygon(ring) => ring,
        PlanarSection::Convex(mut draft) => {
            while draft.has_edge() { draft.advance_edge()?; }
            draft.finish()?
        }
    };
    Ok(PlanarSectionRegions::one(PlanarSectionRegion { outer, holes: [const { None }; MAX_VOIDS] }))
}

struct RingChart{
    points:[Pnt2;MAX_CROSSINGS],
    count:usize,
}

impl RingChart{
    fn new(ring:&SectionPolygon,frame:&Frame3)->Self{
        let mut points=[Pnt2::new(0.,0.);MAX_CROSSINGS];
        for (i,point) in points[..ring.count()].iter_mut().enumerate(){
            let p=Pnt3::from_array(ring.point(i).unwrap())-frame.origin;
            *point=Pnt2::new(p.dot(frame.x),p.dot(frame.y));
        }
        Self{points,count:ring.count()}
    }

    fn sides(&self)->impl Iterator<Item=(Pnt2,Pnt2)> + '_{
        self.points[..self.count].iter().copied().zip(self.points[..self.count].iter().copied().cycle().skip(1))
    }

    fn contains(&self,point:Pnt2)->bool{
        self.sides().fold(false,|inside,(a,b)|inside^ray_event(a,b,point).0)
    }

    fn intersects(&self,other:&Self)->bool{
        let crosses=|x,y|matches!((x,y),(Orient::Positive,Orient::Negative)|(Orient::Negative,Orient::Positive));
        let on_segment=|a:Pnt2,b:Pnt2,p:Pnt2,side|side==Orient::Zero
            &&(a.x.min(b.x)..=a.x.max(b.x)).contains(&p.x)&&(a.y.min(b.y)..=a.y.max(b.y)).contains(&p.y);
        self.sides().any(|(a,b)|other.sides().any(|(c,d)|{
            let ca=orient2d(a,b,c);
            let da=orient2d(a,b,d);
            let ac=orient2d(c,d,a);
            let bc=orient2d(c,d,b);
            (crosses(ca,da)&&crosses(ac,bc))||on_segment(a,b,c,ca)||on_segment(a,b,d,da)
                ||on_segment(c,d,a,ac)||on_segment(c,d,b,bc)
        }))
    }
}

impl PlanarSectionRegions {
    fn add_void(&mut self, hole: SectionPolygon, frame: &Frame3) -> Result<(), &'static str> {
        let invalid = "section void must lie inside one material region";
        let chart = RingChart::new(&hole, frame);
        let mut owner = None;
        for (index, region) in self.0.iter().enumerate() {
            let Some(region) = region else { continue; };
            let outer = RingChart::new(&region.outer, frame);
            if outer.intersects(&chart) || chart.contains(outer.points[0]) { return Err(invalid); }
            if !outer.contains(chart.points[0]) { continue; }
            let mut in_void = false;
            for other in region.holes.iter().flatten() {
                let other = RingChart::new(other, frame);
                if other.intersects(&chart) || chart.contains(other.points[0]) { return Err(invalid); }
                in_void |= other.contains(chart.points[0]);
            }
            if !in_void && owner.replace(index).is_some() { return Err(invalid); }
        }
        let region = self.0[owner.ok_or(invalid)?].as_mut().expect("section void owner");
        *region.holes.iter_mut().find(|space| space.is_none()).ok_or(invalid)? = Some(hole);
        Ok(())
    }

    fn finish(mut self) -> Result<PlanarSection, &'static str> {
        let mut present = self.0.iter().enumerate().filter_map(|(i, region)| region.as_ref().map(|_| i));
        let index = present.next().ok_or("section has no material contour")?;
        if present.next().is_some() { return Ok(PlanarSection::Regions(self)); }
        let region = self.0[index].take().expect("original section region");
        if region.holes.iter().any(Option::is_some) { Ok(PlanarSection::Region(region)) }
        else { Ok(PlanarSection::Polygon(region.outer)) }
    }
}
