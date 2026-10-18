//! 📉️ Bevel and decimate: the bevel and its profile, the decimation work and its candidate collapse records. See [`super`].

use super::*;

pub(super) struct Bevel {
    pub(super) snapshot: Snapshot,
    pub(super) edges: Vec<EdgeId>,
    pub(super) amount: f32,
    pub(super) segments: u32,
    extent: f32,
    epsilon: f32,
    pub(super) face: usize,
    pub(super) vertex: usize,
    pub(super) edge: usize,
    pub(super) selected: HistoryFoldSet<u32>,
    pub(super) planes: Vec<(Vec3, f32)>,
    profile: Option<Profile>,
    pub(super) clip: Option<MeshClipJob>,
    pub(super) build: Option<Build>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    retain_channels:bool,
    pub(super) phase: u8,
}

struct Profile {
    pub(super) faces: [usize; 2],
    directions: [Vec3; 2],
    pub(super) origin: Vec3,
    pub(super) normal: Vec3,
    pub(super) tangent: Vec3,
    pub(super) center: Vec3,
    pub(super) radius: f32,
    theta: f32,
    clearance: f32,
    side: usize,
    pub(super) corner: usize,
    segment: u32,
}

impl Bevel {
    pub(super) fn new(source: HalfedgeMesh, edges: Vec<EdgeId>, amount: f32, segments: u32) -> Self {
        Self {retain_channels:!source.attributes.is_empty() || !source.uv_seams.is_empty(),attributes:None,snapshot: Snapshot::new(source), edges, amount, segments, extent: 0.0, epsilon: 0.0, face: 0, vertex: 0, edge: 0, selected: HistoryFoldSet::new(), planes: Vec::new(), profile: None, clip: None, build: None, phase: 0 }
    }

    pub(super) fn phase(&self) -> &'static str {
        match self.phase { 0 => "snapshot", 1..=3 => "validate", 4 => "profile", 5 => "clip", 6=>"reconstruct",7=>"bevel-clip-reconstruct",8=>"bevel-source-vertices",9=>"bevel-clip-attributes",10=>"snapshot",_=>"bevel-final-attributes" }
    }

    pub(super) fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => { if self.snapshot.complete() { self.phase = 1; } }
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
                    if clip.advance(&mut self.snapshot.soup)? {if self.retain_channels {self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.phase=7;}else {self.clip=None;self.edge+=1;}}
                } else if self.edge < self.planes.len() {
                    let (normal, distance) = self.planes[self.edge];
                    self.clip = Some(MeshClipJob::new(normal, distance, self.epsilon));
                } else {
                    self.build = Some(Build::new(std::mem::take(&mut self.snapshot.soup), true));
                    self.phase = 6;
                }
            }
            6 => {
                if let Some(mesh) = self.build.as_mut().unwrap().advance()? {
                    if mesh.face_count() < 4 { return Err(MeshKernelError::InvalidInput("bevel removed the solid".into())); }
                    self.attributes=Some(SourceAttributeRemap::new(mesh,Some(self.build.as_mut().unwrap().take_origin_vertices()?),None,None,None));self.phase=11;
                }
            }
            7=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let clip=self.clip.as_mut().unwrap();let mut attributes=SourceAttributeRemap::new(mesh,None,Some(clip.take_source_faces()?),None,None);attributes.configuration_mut()?.corner_faces=Some(clip.take_corner_faces()?);attributes.configuration_mut()?.generated_faces=Some(clip.take_face_weights()?);attributes.configuration_mut()?.generated_vertices=Some(Vec::new());self.attributes=Some(attributes);self.vertex=0;self.phase=8;}},
            8=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.vertex==origins.len() {self.phase=9;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.clip.as_ref().unwrap().vertex_source(origins[self.vertex] as usize)?);self.vertex+=1;}},
            9=>{if let Some(mesh)=self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source)? {self.snapshot=Snapshot::new(mesh);self.clip=None;self.build=None;self.attributes=None;self.phase=10;}},
            10=>{if self.snapshot.complete() {self.edge+=1;self.phase=5;}},
            11=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

pub(super) struct Decimate {
    pub(super) snapshot: Snapshot,
    pub(super) target: usize,
    closed: bool,
    pub(super) edges: HistoryFoldSet<(u32, u32)>,
    pub(super) ranked: BinaryHeap<Reverse<Ranked>>,
    pub(super) candidate: Option<Candidate>,
    pub(super) build: Option<Build>,
    pub(super) face: usize,
    pub(super) corner: usize,
    pub(super) source_faces:Vec<u32>,
    pub(super) edge_sources:Vec<(u32,u32)>,
    collapse:(u32,u32),
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) phase: u8,
}

impl Decimate {
    pub(super) fn new(source: HalfedgeMesh, target: usize) -> Self {
        Self { snapshot: Snapshot::new(source), target, closed: true, edges: HistoryFoldSet::new(), ranked: BinaryHeap::new(), candidate: None, build: None, face: 0, corner: 0, source_faces:Vec::new(),edge_sources:Vec::new(),collapse:(0,0),attributes:None,phase: 0 }
    }

    pub(super) fn phase(&self) -> &'static str {
        match self.phase { 0 => "snapshot", 1 => "edges", 2 => "candidate", 3 => "reconstruct",4=>"decimate-source-vertices",_=>"decimate-attributes" }
    }

    pub(super) fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => {
                if self.snapshot.source.vertex_count() <= self.target { return Ok(Some(std::mem::replace(&mut self.snapshot.source, HalfedgeMesh::empty()))); }
                if self.corner < self.snapshot.source.halfedges.len() {
                    self.closed &= self.snapshot.source.halfedges[self.corner].twin.is_some();
                    self.corner += 1;
                } else if self.snapshot.complete() { self.phase = 1; self.corner = 0; }
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
                        CandidateStep::Accepted(soup,faces,edges) => {self.collapse=(candidate.a,candidate.b);self.source_faces=faces;self.edge_sources=edges;self.build = Some(Build::new(soup, self.closed)); self.phase = 3; self.candidate = None;},
                    }
                } else if let Some(Reverse(edge)) = self.ranked.pop() { self.candidate = Some(Candidate::new(edge.a, edge.b)); }
                else { return Ok(Some(std::mem::replace(&mut self.snapshot.source, HalfedgeMesh::empty()))); }
            }
            3 => {
                if let Some(mesh) = self.build.as_mut().unwrap().advance()? {
                    if mesh.vertex_count() >= self.snapshot.source.vertex_count() { self.build = None; self.phase = 2; }
                    else {
                        let mut attributes=SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None);attributes.configuration_mut()?.generated_vertices=Some(Vec::new());attributes.configuration_mut()?.edge_source_keys=Some(std::mem::take(&mut self.edge_sources));self.attributes=Some(attributes);self.corner=0;self.phase=4;
                    }
                }
            }
            4=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.corner==origins.len() {self.phase=5;}else {let vertex=origins[self.corner];self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(if vertex==self.collapse.0 {SourceVertex::Edge{a:self.collapse.0,b:self.collapse.1,t:0.5}}else {SourceVertex::Original(vertex)});self.corner+=1;}},
            5=>{if let Some(mesh)=self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source)? {self.snapshot=Snapshot::new(mesh);self.edges.clear();self.ranked.clear();self.build=None;self.attributes=None;self.face=0;self.corner=0;self.phase=0;}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

enum CandidateStep { Working, Rejected, Accepted(Soup,Vec<u32>,Vec<(u32,u32)>) }

pub(super) struct Candidate {
    pub(super) a: u32,
    pub(super) b: u32,
    pub(super) soup: Soup,
    pub(super) corners: Vec<u32>,
    pub(super) corner_sources:Vec<(u32,u32)>,
    pub(super) edge_sources:Vec<(u32,u32)>,
    pub(super) face_sources:Vec<u32>,
    pub(super) seen: HistoryFoldIndex<u32, usize>,
    pub(super) normal: Normal,
    pub(super) incidence: HistoryFoldIndex<(u32, u32), (usize, i32)>,
    pub(super) links: HistoryFoldIndex<u32, u32>,
    pub(super) face: usize,
    pub(super) corner: usize,
    pub(super) vertex: usize,
    pub(super) start: Option<u32>,
    pub(super) cursor: u32,
    pub(super) phase: u8,
}

impl Candidate {
    pub(super) fn new(a: u32, b: u32) -> Self {
        Self { a, b, soup: Soup::default(), corners: Vec::new(),corner_sources:Vec::new(),edge_sources:Vec::new(),face_sources:Vec::new(), seen: HistoryFoldIndex::new(), normal: Normal::default(), incidence: HistoryFoldIndex::new(), links: HistoryFoldIndex::new(), face: 0, corner: 0, vertex: 0, start: None, cursor: 0, phase: 0 }
    }

    pub(super) fn advance(&mut self, source: &Soup, closed: bool) -> MeshResult<CandidateStep> {
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
                    if self.corners.last()!=Some(&id) {self.corners.push(id);self.corner_sources.push((self.face as u32,face[self.corner]));}else {*self.corner_sources.last_mut().unwrap()=(self.face as u32,face[self.corner]);}
                    self.corner += 1;
                    if self.corner == face.len() {
                        if self.corners.first()==self.corners.last() {self.corners.pop();self.corner_sources.pop();}
                        self.corner = 0;
                        if self.corners.len()<3 {self.corners.clear();self.corner_sources.clear();self.face+=1;} else { self.phase = 2; }
                    }
                }
            }
            2 => {
                let id = self.corners[self.corner];
                if self.seen.insert(id, self.face) == Some(self.face) { return Ok(CandidateStep::Rejected); }
                let a = Vec3(self.soup.positions[id as usize]);
                let b = Vec3(self.soup.positions[self.corners[(self.corner + 1) % self.corners.len()] as usize]);
                if self.corner == 0 { self.normal.set_origin(a); }
                self.normal.add(a, b);
                self.corner += 1;
                if self.corner == self.corners.len() {
                    let after = self.normal.value();
                    if after.length() == 0.0 || source.normals[self.face].dot(after) <= 0.0 { return Ok(CandidateStep::Rejected); }
                    self.soup.faces.push(std::mem::take(&mut self.corners));self.face_sources.push(self.face as u32);self.cursor=0;self.phase=3;return Ok(CandidateStep::Working);
                }
            }
            3=>{if self.cursor as usize==self.corner_sources.len() {self.corner_sources.clear();self.normal=Normal::default();self.corner=0;self.face+=1;self.phase=1;}else {self.edge_sources.push(self.corner_sources[self.cursor as usize]);self.cursor+=1;}},
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
                } else { return Ok(CandidateStep::Accepted(std::mem::take(&mut self.soup),std::mem::take(&mut self.face_sources),std::mem::take(&mut self.edge_sources))); }
            }
            _ => {
                if let Some(next) = self.links.remove(&self.cursor) {
                    self.cursor = next;
                    if Some(next) == self.start {
                        if !self.links.is_empty() { return Ok(CandidateStep::Rejected); }
                        return Ok(CandidateStep::Accepted(std::mem::take(&mut self.soup),std::mem::take(&mut self.face_sources),std::mem::take(&mut self.edge_sources)));
                    }
                } else { return Ok(CandidateStep::Rejected); }
            }
        }
        Ok(CandidateStep::Working)
    }
}
