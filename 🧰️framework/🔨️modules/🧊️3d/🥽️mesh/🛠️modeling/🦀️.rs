//! 🛠️ Retained modeling work, measured in vertices, corners, and candidate transitions.

use super::*;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

/// 📊 Completed work and a conservative remaining-work estimate, finalized on completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshModelingProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: &'static str,
}

/// 🧭 A budgeted modeling result; completed geometry is transferred exactly once.
#[derive(Debug)]
pub enum MeshModelingStep {
    Working(MeshModelingProgress),
    Done(HalfedgeMesh),
    Cancelled(MeshModelingProgress),
}

/// 🧵 Owns a source snapshot and retained geometry work without changing the source.
pub struct MeshModelingJob {
    work: Work,
    progress: MeshModelingProgress,
    cancelled: bool,
    retired: bool,
}

enum Work {
    Bevel(Box<Bevel>),
    Decimate(Box<Decimate>),
}

impl MeshModelingJob {
    pub fn progress(&self) -> MeshModelingProgress { self.progress }

    pub fn cancel(&mut self) {
        if !self.retired { self.cancelled = true; }
    }

    pub(super) fn finish_with_progress(mut self, mut progress: impl FnMut(f32) -> bool) -> MeshResult<HalfedgeMesh> {
        loop {
            let value = self.progress();
            if !progress(value.units_done as f32 / value.units_total as f32) { return Err(MeshKernelError::InvalidInput("operation cancelled".into())); }
            match self.step(256)? {
                MeshModelingStep::Working(_) => {},
                MeshModelingStep::Done(mesh) => {
                    if !progress(1.0) { return Err(MeshKernelError::InvalidInput("operation cancelled".into())); }
                    return Ok(mesh);
                }
                MeshModelingStep::Cancelled(_) => return Err(MeshKernelError::InvalidInput("operation cancelled".into())),
            }
        }
    }

    pub fn step(&mut self, budget: usize) -> MeshResult<MeshModelingStep> {
        if self.retired { return Err(MeshKernelError::InvalidInput("modeling job already retired".into())); }
        if self.cancelled { return Ok(MeshModelingStep::Cancelled(self.progress)); }
        for _ in 0..budget {
            let result = match &mut self.work {
                Work::Bevel(work) => work.advance(),
                Work::Decimate(work) => work.advance(),
            };
            let output = match result {
                Ok(output) => output,
                Err(error) => { self.retired = true; return Err(error); }
            };
            self.progress.units_done += 1;
            self.progress.phase = match &self.work {
                Work::Bevel(work) => work.phase(),
                Work::Decimate(work) => work.phase(),
            };
            if let Some(mesh) = output {
                self.progress.units_total = self.progress.units_done;
                self.progress.phase = "done";
                self.retired = true;
                return Ok(MeshModelingStep::Done(mesh));
            }
            self.progress.units_total = self.progress.units_total.max(self.progress.units_done.saturating_add(1));
        }
        Ok(MeshModelingStep::Working(self.progress))
    }
}

impl HalfedgeMesh {
    /// 🪚 Captures a segmented bevel as retained, cancellable geometry work.
    pub fn bevel_job(&self, edges: &[EdgeId], amount: f32, segments: u32) -> MeshResult<MeshModelingJob> {
        if edges.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !amount.is_finite() || amount <= 0.0 || !(1..=64).contains(&segments) { return Err(MeshKernelError::InvalidInput("bevel requires positive finite width and 1..=64 segments".into())); }
        let work = self.vertices.len().saturating_mul(self.faces.len()).saturating_add(edges.len().saturating_mul(segments as usize).saturating_mul(self.halfedges.len()));
        if work > 4_000_000 { return Err(MeshKernelError::InvalidInput("bevel work budget exceeded".into())); }
        for edge in edges {
            let halfedge = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            if halfedge.twin.is_none() { return Err(MeshKernelError::NonManifold); }
        }
        Ok(MeshModelingJob {
            work: Work::Bevel(Box::new(Bevel::new(self.clone(), edges.to_vec(), amount, segments))),
            progress: MeshModelingProgress { units_done: 0, units_total: work.saturating_mul(16).saturating_add(1), phase: "snapshot" },
            cancelled: false, retired: false,
        })
    }

    /// 🪶 Captures safe edge collapses as retained, cancellable candidate work.
    pub fn decimate_job(&self, ratio: f32) -> MeshResult<MeshModelingJob> {
        if !ratio.is_finite() || ratio <= 0.0 || ratio > 1.0 { return Err(MeshKernelError::InvalidInput("decimation ratio must be in (0,1]".into())); }
        let initial = self.vertex_count();
        let target = ((initial as f32 * ratio).ceil() as usize).max(4);
        let work = initial.saturating_mul(self.halfedges.len()).saturating_mul(initial.saturating_sub(target));
        if work > 32_000_000 { return Err(MeshKernelError::InvalidInput("decimation work budget exceeded".into())); }
        Ok(MeshModelingJob {
            work: Work::Decimate(Box::new(Decimate::new(self.clone(), target))),
            progress: MeshModelingProgress { units_done: 0, units_total: work.saturating_mul(16).saturating_add(1), phase: "snapshot" },
            cancelled: false, retired: false,
        })
    }
}

struct Normal {
    origin: Vec3,
    sum: [f64; 3],
}

impl Default for Normal {
    fn default() -> Self { Self { origin: Vec3::ZERO, sum: [0.0; 3] } }
}

impl Normal {
    fn add(&mut self, a: Vec3, b: Vec3) {
        let a = [0, 1, 2].map(|i| a.0[i] as f64 - self.origin.0[i] as f64);
        let b = [0, 1, 2].map(|i| b.0[i] as f64 - self.origin.0[i] as f64);
        self.sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        self.sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        self.sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    fn value(&self) -> Vec3 {
        let length = self.sum[0].hypot(self.sum[1]).hypot(self.sum[2]);
        if length == 0.0 { Vec3::ZERO } else { Vec3(self.sum.map(|v| (v / length) as f32)).normalize() }
    }
}

#[derive(Default)]
struct Soup {
    positions: Vec<[f32; 3]>,
    faces: Vec<Vec<u32>>,
    normals: Vec<Vec3>,
}

struct Snapshot {
    source: HalfedgeMesh,
    soup: Soup,
    vertex: usize,
    face: usize,
    halfedge: Option<u32>,
    corners: Vec<u32>,
    normal: Normal,
    normal_corner: usize,
    normalizing: bool,
    reversing: Option<usize>,
}

impl Snapshot {
    fn new(source: HalfedgeMesh) -> Self {
        Self { source, soup: Soup::default(), vertex: 0, face: 0, halfedge: None, corners: Vec::new(), normal: Normal::default(), normal_corner: 0, normalizing: false, reversing: None }
    }

    fn advance(&mut self) -> bool {
        if self.vertex < self.source.vertices.len() {
            self.soup.positions.push(self.source.vertices[self.vertex].position);
            self.vertex += 1;
        } else if self.face < self.source.faces.len() {
            let face = &self.source.faces[self.face];
            if let Some(index) = self.reversing {
                if index < self.corners.len() / 2 {
                    let opposite = self.corners.len() - 1 - index;
                    self.corners.swap(index, opposite);
                    self.reversing = Some(index + 1);
                } else { self.reversing = None; self.normalizing = true; }
            } else if self.normalizing {
                let i = self.normal_corner;
                let a = self.corners[i];
                let b = self.corners[(i + 1) % self.corners.len()];
                if i == 0 { self.normal.origin = Vec3(self.soup.positions[a as usize]); }
                self.normal.add(Vec3(self.soup.positions[a as usize]), Vec3(self.soup.positions[b as usize]));
                self.normal_corner += 1;
                if self.normal_corner == self.corners.len() {
                    self.soup.normals.push(self.normal.value());
                    self.soup.faces.push(std::mem::take(&mut self.corners));
                    self.face += 1;
                    self.halfedge = None;
                    self.normalizing = false;
                    self.normal_corner = 0;
                    self.normal = Normal::default();
                }
            } else {
                let id = self.halfedge.unwrap_or(face.halfedge);
                let edge = &self.source.halfedges[id as usize];
                self.corners.push(edge.vertex);
                self.halfedge = Some(edge.next);
                if edge.next == face.halfedge {
                    if face.flipped { self.reversing = Some(0); } else { self.normalizing = true; }
                }
            }
        } else { return true; }
        false
    }
}

struct Build {
    soup: Soup,
    mesh: HalfedgeMesh,
    used: HashSet<u32>,
    remap: HashMap<u32, u32>,
    edges: HashMap<(u32, u32), u32>,
    face: usize,
    corner: usize,
    vertex: usize,
    start: u32,
    normal: Normal,
    phase: u8,
    closed: bool,
}

impl Build {
    fn new(soup: Soup, closed: bool) -> Self {
        Self { soup, mesh: HalfedgeMesh::empty(), used: HashSet::new(), remap: HashMap::new(), edges: HashMap::new(), face: 0, corner: 0, vertex: 0, start: 0, normal: Normal::default(), phase: 0, closed }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => {
                if self.face == self.soup.faces.len() { self.phase = 1; self.face = 0; }
                else {
                    let face = &self.soup.faces[self.face];
                    if face.len() < 3 { return Err(MeshKernelError::DegenerateOperation); }
                    self.used.insert(face[self.corner]);
                    self.corner += 1;
                    if self.corner == face.len() { self.corner = 0; self.face += 1; }
                }
            }
            1 => {
                if self.vertex == self.soup.positions.len() { self.phase = 2; }
                else {
                    if self.used.contains(&(self.vertex as u32)) {
                        self.remap.insert(self.vertex as u32, self.mesh.vertices.len() as u32);
                        self.mesh.add_vertex(self.soup.positions[self.vertex]);
                    }
                    self.vertex += 1;
                }
            }
            2 => {
                if self.face == self.soup.faces.len() { self.phase = 3; self.face = 0; }
                else {
                    let face = &self.soup.faces[self.face];
                    let a = *self.remap.get(&face[self.corner]).ok_or(MeshKernelError::InvalidHandle)?;
                    let b = *self.remap.get(&face[(self.corner + 1) % face.len()]).ok_or(MeshKernelError::InvalidHandle)?;
                    let id = self.mesh.halfedges.len() as u32;
                    if self.corner == 0 { self.start = id; }
                    let next = if self.corner + 1 == face.len() { self.start } else { id + 1 };
                    let twin = self.edges.get(&(b, a)).copied();
                    self.mesh.halfedges.push(HalfEdge { vertex: a, twin, next, face: Some(self.face as u32), uv: [0.0, 0.0] });
                    if let Some(twin) = twin { self.mesh.halfedges[twin as usize].twin = Some(id); }
                    self.edges.insert((a, b), id);
                    self.corner += 1;
                    if self.corner == face.len() {
                        self.mesh.faces.push(MeshFace { halfedge: self.start, smooth: false, flipped: false });
                        let a = self.mesh.halfedges[self.start as usize].vertex;
                        self.mesh.vertices[a as usize].halfedge = Some(self.start);
                        self.corner = 0;
                        self.face += 1;
                    }
                }
            }
            3 => {
                if self.face == self.soup.faces.len() { self.phase = 5; self.vertex = 0; }
                else {
                    let face = &self.soup.faces[self.face];
                    let a = Vec3(self.soup.positions[face[self.corner] as usize]);
                    let b = Vec3(self.soup.positions[face[(self.corner + 1) % face.len()] as usize]);
                    if self.corner == 0 { self.normal.origin = a; }
                    self.normal.add(a, b);
                    self.corner += 1;
                    if self.corner == face.len() { self.phase = 4; self.corner = 0; }
                }
            }
            4 => {
                let face = &self.soup.faces[self.face];
                let vertex = &mut self.mesh.vertices[self.remap[&face[self.corner]] as usize];
                if vertex.normal.is_none() { vertex.normal = Some(self.normal.value().0); }
                self.corner += 1;
                if self.corner == face.len() { self.phase = 3; self.corner = 0; self.face += 1; self.normal = Normal::default(); }
            }
            5 => {
                if self.vertex == self.mesh.halfedges.len() { return Ok(Some(std::mem::replace(&mut self.mesh, HalfedgeMesh::empty()))); }
                if self.closed {
                    let edge = &self.mesh.halfedges[self.vertex];
                    if edge.twin.is_none_or(|twin| self.mesh.halfedges[twin as usize].twin != Some(self.vertex as u32)) { return Err(MeshKernelError::NonManifold); }
                }
                self.vertex += 1;
            }
            _ => unreachable!(),
        }
        Ok(None)
    }
}

struct Bevel {
    snapshot: Snapshot,
    edges: Vec<EdgeId>,
    amount: f32,
    segments: u32,
    extent: f32,
    epsilon: f32,
    face: usize,
    vertex: usize,
    edge: usize,
    selected: HashSet<u32>,
    planes: Vec<(Vec3, f32)>,
    profile: Option<Profile>,
    clip: Option<Clip>,
    build: Option<Build>,
    phase: u8,
}

struct Profile {
    faces: [usize; 2],
    directions: [Vec3; 2],
    origin: Vec3,
    normal: Vec3,
    tangent: Vec3,
    center: Vec3,
    radius: f32,
    theta: f32,
    clearance: f32,
    side: usize,
    corner: usize,
    segment: u32,
}

impl Bevel {
    fn new(source: HalfedgeMesh, edges: Vec<EdgeId>, amount: f32, segments: u32) -> Self {
        Self { snapshot: Snapshot::new(source), edges, amount, segments, extent: 0.0, epsilon: 0.0, face: 0, vertex: 0, edge: 0, selected: HashSet::new(), planes: Vec::new(), profile: None, clip: None, build: None, phase: 0 }
    }

    fn phase(&self) -> &'static str {
        match self.phase { 0 => "snapshot", 1..=3 => "validate", 4 => "profile", 5 => "clip", _ => "reconstruct" }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => { if self.snapshot.advance() { self.phase = 1; } }
            1 => {
                let positions = &self.snapshot.soup.positions;
                if self.vertex == positions.len() {
                    self.epsilon = self.extent * 1e-6;
                    if self.epsilon == 0.0 { return Err(MeshKernelError::DegenerateOperation); }
                    self.vertex = 0;
                    self.phase = 2;
                } else { self.extent = self.extent.max(Vec3(positions[self.vertex]).sub(Vec3(positions[0])).length()); self.vertex += 1; }
            }
            2 => {
                let soup = &self.snapshot.soup;
                if self.face == soup.faces.len() { self.phase = 3; self.vertex = 0; }
                else {
                    let normal = soup.normals[self.face];
                    let origin = Vec3(soup.positions[soup.faces[self.face][0] as usize]);
                    if normal.length() == 0.0 || Vec3(soup.positions[self.vertex]).sub(origin).dot(normal) > self.epsilon { return Err(MeshKernelError::InvalidInput("bevel requires an outward convex mesh".into())); }
                    self.vertex += 1;
                    if self.vertex == soup.positions.len() { self.vertex = 0; self.face += 1; }
                }
            }
            3 => {
                if self.vertex == self.snapshot.source.halfedges.len() { self.phase = 4; }
                else {
                    if self.snapshot.source.halfedges[self.vertex].twin.is_none() { return Err(MeshKernelError::InvalidInput("bevel requires a closed manifold mesh".into())); }
                    self.vertex += 1;
                }
            }
            4 => {
                if let Some(profile) = &mut self.profile {
                    if profile.side < 2 {
                        let face = &self.snapshot.soup.faces[profile.faces[profile.side]];
                        let point = Vec3(self.snapshot.soup.positions[face[profile.corner] as usize]);
                        let distance = profile.origin.sub(point).dot(profile.directions[profile.side]);
                        if distance > self.epsilon { profile.clearance = profile.clearance.min(distance); }
                        profile.corner += 1;
                        if profile.corner == face.len() { profile.corner = 0; profile.side += 1; }
                    } else {
                        if self.amount >= profile.clearance * 0.5 || self.amount <= self.epsilon { return Err(MeshKernelError::InvalidInput("bevel width must stay below half the neighboring face clearance".into())); }
                        let angle = profile.theta * (profile.segment as f32 + 0.5) / self.segments as f32;
                        let normal = profile.normal.scale(angle.cos()).add(profile.tangent.scale(angle.sin()));
                        let distance = normal.dot(profile.center) + profile.radius * (profile.theta / (2.0 * self.segments as f32)).cos();
                        self.planes.push((normal, distance));
                        profile.segment += 1;
                        if profile.segment == self.segments { self.profile = None; }
                    }
                } else if self.edge == self.edges.len() { self.phase = 5; self.edge = 0; }
                else {
                    let id = self.edges[self.edge].0;
                    self.edge += 1;
                    let he = &self.snapshot.source.halfedges[id as usize];
                    let twin = he.twin.ok_or(MeshKernelError::NonManifold)?;
                    if self.selected.insert(id.min(twin)) {
                        let faces = [he.face.ok_or(MeshKernelError::InvalidHandle)? as usize, self.snapshot.source.halfedges[twin as usize].face.ok_or(MeshKernelError::InvalidHandle)? as usize];
                        let n1 = self.snapshot.soup.normals[faces[0]];
                        let n2 = self.snapshot.soup.normals[faces[1]];
                        let cosine = n1.dot(n2).clamp(-1.0, 1.0);
                        let theta = cosine.acos();
                        let sine = theta.sin();
                        if sine.abs() < 1e-5 { return Err(MeshKernelError::DegenerateOperation); }
                        let origin = Vec3(self.snapshot.soup.positions[he.vertex as usize]);
                        let tangent = n2.sub(n1.scale(cosine)).scale(1.0 / sine);
                        self.profile = Some(Profile { faces, directions: [tangent, n1.sub(n2.scale(cosine)).scale(1.0 / sine)], origin, normal: n1, tangent, center: origin.sub(n1.add(n2).scale(self.amount / sine)), radius: self.amount * (1.0 + cosine) / sine, theta, clearance: f32::INFINITY, side: 0, corner: 0, segment: 0 });
                    }
                }
            }
            5 => {
                if let Some(clip) = &mut self.clip {
                    if clip.advance(&mut self.snapshot.soup)? { self.clip = None; self.edge += 1; }
                } else if self.edge < self.planes.len() {
                    let (normal, distance) = self.planes[self.edge];
                    self.clip = Some(Clip::new(normal, distance, self.epsilon));
                } else {
                    self.build = Some(Build::new(std::mem::take(&mut self.snapshot.soup), true));
                    self.phase = 6;
                }
            }
            _ => {
                if let Some(mesh) = self.build.as_mut().unwrap().advance()? {
                    if mesh.face_count() < 4 { return Err(MeshKernelError::InvalidInput("bevel removed the solid".into())); }
                    return Ok(Some(mesh));
                }
            }
        }
        Ok(None)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Ranked {
    rank: f32,
    a: u32,
    b: u32,
}

impl Eq for Ranked {}
impl PartialOrd for Ranked {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for Ranked {
    fn cmp(&self, other: &Self) -> Ordering { self.rank.total_cmp(&other.rank).then((self.a, self.b).cmp(&(other.a, other.b))) }
}

struct Clip {
    normal: Vec3,
    distance: f32,
    epsilon: f32,
    intersections: HashMap<(u32, u32), u32>,
    cap: BTreeSet<u32>,
    cap_ids: Vec<u32>,
    clipped: Vec<Vec<u32>>,
    corners: Vec<u32>,
    sorted: BinaryHeap<Reverse<Ranked>>,
    center: Vec3,
    face: usize,
    corner: usize,
    phase: u8,
}

impl Clip {
    fn new(normal: Vec3, distance: f32, epsilon: f32) -> Self {
        Self { normal, distance, epsilon, intersections: HashMap::new(), cap: BTreeSet::new(), cap_ids: Vec::new(), clipped: Vec::new(), corners: Vec::new(), sorted: BinaryHeap::new(), center: Vec3::ZERO, face: 0, corner: 0, phase: 0 }
    }

    fn advance(&mut self, soup: &mut Soup) -> MeshResult<bool> {
        match self.phase {
            0 => {
                if self.face == soup.faces.len() {
                    if self.cap.len() < 3 { return Err(MeshKernelError::InvalidInput("bevel width does not intersect the solid".into())); }
                    self.phase = 1;
                } else {
                    let face = &soup.faces[self.face];
                    let (a, b) = (face[self.corner], face[(self.corner + 1) % face.len()]);
                    let da = Vec3(soup.positions[a as usize]).dot(self.normal) - self.distance;
                    let db = Vec3(soup.positions[b as usize]).dot(self.normal) - self.distance;
                    if da <= self.epsilon {
                        if self.corners.last() != Some(&a) { self.corners.push(a); }
                        if da.abs() <= self.epsilon { self.cap.insert(a); }
                    }
                    if (da > self.epsilon && db < -self.epsilon) || (da < -self.epsilon && db > self.epsilon) {
                        let id = *self.intersections.entry((a.min(b), a.max(b))).or_insert_with(|| {
                            let point = Vec3(soup.positions[a as usize]).lerp(Vec3(soup.positions[b as usize]), da / (da - db));
                            let id = soup.positions.len() as u32;
                            soup.positions.push(point.0);
                            id
                        });
                        if self.corners.last() != Some(&id) { self.corners.push(id); }
                        self.cap.insert(id);
                    }
                    self.corner += 1;
                    if self.corner == face.len() {
                        if self.corners.first() == self.corners.last() { self.corners.pop(); }
                        if self.corners.len() >= 3 { self.clipped.push(std::mem::take(&mut self.corners)); } else { self.corners.clear(); }
                        self.corner = 0;
                        self.face += 1;
                    }
                }
            }
            1 => {
                if let Some(id) = self.cap.pop_first() { self.cap_ids.push(id); }
                else { self.phase = 2; }
            }
            2 => {
                if self.corner == self.cap_ids.len() { self.phase = 3; self.corner = 0; }
                else {
                    let id = self.cap_ids[self.corner];
                    self.center = self.center.add(Vec3(soup.positions[id as usize]).scale(1.0 / self.cap_ids.len() as f32));
                    self.corner += 1;
                }
            }
            3 => {
                if self.corner == self.cap_ids.len() { self.phase = 4; }
                else {
                    let axis = if self.normal.x().abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
                    let u = self.normal.cross(axis).normalize();
                    let v = self.normal.cross(u);
                    let id = self.cap_ids[self.corner];
                    let point = Vec3(soup.positions[id as usize]).sub(self.center);
                    self.sorted.push(Reverse(Ranked { rank: point.dot(v).atan2(point.dot(u)), a: id, b: 0 }));
                    self.corner += 1;
                }
            }
            _ => {
                if let Some(Reverse(point)) = self.sorted.pop() { self.corners.push(point.a); }
                else {
                    self.clipped.push(std::mem::take(&mut self.corners));
                    soup.faces = std::mem::take(&mut self.clipped);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

struct Decimate {
    snapshot: Snapshot,
    target: usize,
    closed: bool,
    edges: HashSet<(u32, u32)>,
    ranked: BinaryHeap<Reverse<Ranked>>,
    candidate: Option<Candidate>,
    build: Option<Build>,
    face: usize,
    corner: usize,
    phase: u8,
}

impl Decimate {
    fn new(source: HalfedgeMesh, target: usize) -> Self {
        Self { snapshot: Snapshot::new(source), target, closed: true, edges: HashSet::new(), ranked: BinaryHeap::new(), candidate: None, build: None, face: 0, corner: 0, phase: 0 }
    }

    fn phase(&self) -> &'static str {
        match self.phase { 0 => "snapshot", 1 => "edges", 2 => "candidate", _ => "reconstruct" }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => {
                if self.snapshot.source.vertex_count() <= self.target { return Ok(Some(std::mem::replace(&mut self.snapshot.source, HalfedgeMesh::empty()))); }
                if self.corner < self.snapshot.source.halfedges.len() {
                    self.closed &= self.snapshot.source.halfedges[self.corner].twin.is_some();
                    self.corner += 1;
                } else if self.snapshot.advance() { self.phase = 1; self.corner = 0; }
            }
            1 => {
                let soup = &self.snapshot.soup;
                if self.face == soup.faces.len() { self.phase = 2; }
                else {
                    let face = &soup.faces[self.face];
                    let (a, b) = (face[self.corner], face[(self.corner + 1) % face.len()]);
                    let edge = (a.min(b), a.max(b));
                    if self.edges.insert(edge) {
                        let rank = Vec3(soup.positions[a as usize]).sub(Vec3(soup.positions[b as usize])).length();
                        self.ranked.push(Reverse(Ranked { rank, a: edge.0, b: edge.1 }));
                    }
                    self.corner += 1;
                    if self.corner == face.len() { self.corner = 0; self.face += 1; }
                }
            }
            2 => {
                if let Some(candidate) = &mut self.candidate {
                    match candidate.advance(&self.snapshot.soup, self.closed)? {
                        CandidateStep::Working => {},
                        CandidateStep::Rejected => { self.candidate = None; },
                        CandidateStep::Accepted(soup) => { self.build = Some(Build::new(soup, self.closed)); self.phase = 3; self.candidate = None; },
                    }
                } else if let Some(Reverse(edge)) = self.ranked.pop() { self.candidate = Some(Candidate::new(edge.a, edge.b)); }
                else { return Ok(Some(std::mem::replace(&mut self.snapshot.source, HalfedgeMesh::empty()))); }
            }
            _ => {
                if let Some(mesh) = self.build.as_mut().unwrap().advance()? {
                    if mesh.vertex_count() >= self.snapshot.source.vertex_count() { self.build = None; self.phase = 2; }
                    else {
                        self.snapshot = Snapshot::new(mesh);
                        self.edges.clear();
                        self.ranked.clear();
                        self.build = None;
                        self.face = 0;
                        self.corner = 0;
                        self.phase = 0;
                    }
                }
            }
        }
        Ok(None)
    }
}

enum CandidateStep { Working, Rejected, Accepted(Soup) }

struct Candidate {
    a: u32,
    b: u32,
    soup: Soup,
    corners: Vec<u32>,
    seen: HashMap<u32, usize>,
    normal: Normal,
    incidence: BTreeMap<(u32, u32), (usize, i32)>,
    links: HashMap<u32, u32>,
    face: usize,
    corner: usize,
    vertex: usize,
    start: Option<u32>,
    cursor: u32,
    phase: u8,
}

impl Candidate {
    fn new(a: u32, b: u32) -> Self {
        Self { a, b, soup: Soup::default(), corners: Vec::new(), seen: HashMap::new(), normal: Normal::default(), incidence: BTreeMap::new(), links: HashMap::new(), face: 0, corner: 0, vertex: 0, start: None, cursor: 0, phase: 0 }
    }

    fn advance(&mut self, source: &Soup, closed: bool) -> MeshResult<CandidateStep> {
        match self.phase {
            0 => {
                if self.vertex == source.positions.len() { self.phase = 1; }
                else {
                    let point = if self.vertex == self.a as usize { Vec3(source.positions[self.a as usize]).lerp(Vec3(source.positions[self.b as usize]), 0.5).0 } else { source.positions[self.vertex] };
                    self.soup.positions.push(point);
                    self.vertex += 1;
                }
            }
            1 => {
                if self.face == source.faces.len() {
                    if self.soup.faces.len() < 4 { return Ok(CandidateStep::Rejected); }
                    self.phase = 4;
                    self.face = 0;
                } else {
                    let face = &source.faces[self.face];
                    let id = if face[self.corner] == self.b { self.a } else { face[self.corner] };
                    if self.corners.last() != Some(&id) { self.corners.push(id); }
                    self.corner += 1;
                    if self.corner == face.len() {
                        if self.corners.first() == self.corners.last() { self.corners.pop(); }
                        self.corner = 0;
                        if self.corners.len() < 3 { self.corners.clear(); self.face += 1; } else { self.phase = 2; }
                    }
                }
            }
            2 => {
                let id = self.corners[self.corner];
                if self.seen.insert(id, self.face) == Some(self.face) { return Ok(CandidateStep::Rejected); }
                let a = Vec3(self.soup.positions[id as usize]);
                let b = Vec3(self.soup.positions[self.corners[(self.corner + 1) % self.corners.len()] as usize]);
                if self.corner == 0 { self.normal.origin = a; }
                self.normal.add(a, b);
                self.corner += 1;
                if self.corner == self.corners.len() {
                    let after = self.normal.value();
                    if after.length() == 0.0 || source.normals[self.face].dot(after) <= 0.0 { return Ok(CandidateStep::Rejected); }
                    self.soup.faces.push(std::mem::take(&mut self.corners));
                    self.normal = Normal::default();
                    self.corner = 0;
                    self.face += 1;
                    self.phase = 1;
                }
            }
            4 => {
                if self.face == self.soup.faces.len() { self.phase = 5; }
                else {
                    let face = &self.soup.faces[self.face];
                    let (a, b) = (face[self.corner], face[(self.corner + 1) % face.len()]);
                    let entry = self.incidence.entry((a.min(b), a.max(b))).or_default();
                    entry.0 += 1;
                    entry.1 += if a < b { 1 } else { -1 };
                    if a == self.a {
                        let previous = face[(self.corner + face.len() - 1) % face.len()];
                        if closed && self.links.insert(previous, b).is_some() { return Ok(CandidateStep::Rejected); }
                        self.start = Some(self.start.map_or(previous, |start| start.min(previous)));
                    }
                    self.corner += 1;
                    if self.corner == face.len() { self.corner = 0; self.face += 1; }
                }
            }
            5 => {
                if let Some((_, edge)) = self.incidence.pop_first() {
                    if edge != (2, 0) && (closed || edge.0 != 1) { return Ok(CandidateStep::Rejected); }
                } else if closed {
                    let Some(start) = self.start else { return Ok(CandidateStep::Rejected); };
                    self.cursor = start;
                    self.phase = 6;
                } else { return Ok(CandidateStep::Accepted(std::mem::take(&mut self.soup))); }
            }
            _ => {
                if let Some(next) = self.links.remove(&self.cursor) {
                    self.cursor = next;
                    if Some(next) == self.start {
                        if !self.links.is_empty() { return Ok(CandidateStep::Rejected); }
                        return Ok(CandidateStep::Accepted(std::mem::take(&mut self.soup)));
                    }
                } else { return Ok(CandidateStep::Rejected); }
            }
        }
        Ok(CandidateStep::Working)
    }
}
