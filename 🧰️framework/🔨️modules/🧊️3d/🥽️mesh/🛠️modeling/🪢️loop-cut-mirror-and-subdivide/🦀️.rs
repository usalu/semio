//! 🪢️ Loop cut, mirror and subdivide: the loop cut, mirror and subdivision work records with their stepping. See [`super`].

use super::*;

pub(super) struct LoopCut {
    pub(super) snapshot:Snapshot,
    pub(super) edges:Vec<EdgeId>,
    pub(super) cuts:usize,
    pub(super) incidence:HistoryFoldIndex<(u32,u32),Vec<(usize,usize)>>,
    pub(super) marked:HistoryFoldSet<(u32,u32)>,
    pub(super) pending:Vec<(u32,u32)>,
    pub(super) cut_vertices:HistoryFoldIndex<(u32,u32),Vec<u32>>,
    pub(super) last_edge:Option<(u32,u32)>,
    pub(super) cut_edge:Option<(u32,u32)>,
    pub(super) cut_ids:Vec<u32>,
    pub(super) soup:Soup,
    pub(super) boundary:Vec<u32>,
    pub(super) grid:Vec<u32>,
    pub(super) face:usize,
    pub(super) corner:usize,
    pub(super) cursor:usize,
    pub(super) step:usize,
    pub(super) nx:usize,
    pub(super) ny:usize,
    pub(super) vertices:u64,
    pub(super) faces:u64,
    pub(super) corners:u64,
    pub(super) building_grid:bool,
    pub(super) build:Option<Build>,
    pub(super) vertex_sources:Vec<SourceVertex>,
    pub(super) source_faces:Vec<u32>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) phase:u8,
}

impl LoopCut {
    pub(super) fn key(a:u32,b:u32)->(u32,u32) { (a.min(b),a.max(b)) }
    pub(super) fn phase(&self)->&'static str { ["snapshot","loop-cut-incidence","loop-cut-selection","loop-cut-propagate","loop-cut-admission","loop-cut-vertices","loop-cut-grid","loop-cut-reconstruct","loop-cut-source-vertices","loop-cut-source-remap","loop-cut-attributes"][self.phase as usize] }
    pub(super) fn dimensions(&self,face:&[u32])->(usize,usize) { (if self.marked.contains(&Self::key(face[0],face[1])) { self.cuts+1 } else { 1 },if self.marked.contains(&Self::key(face[1],face[2])) { self.cuts+1 } else { 1 }) }
    fn on_edge(&self,a:u32,b:u32,step:usize,segments:usize)->u32 { if step==0 { a } else if step==segments { b } else { self.cut_vertices[&Self::key(a,b)][if a<b { step-1 } else { segments-step-1 }] } }
    pub(super) fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.complete() {self.cursor=0;self.phase=8;}}
            1=>{
                if self.face==self.snapshot.soup.faces.len() { self.phase=2;self.cursor=0;self.face=0; }
                else { let face=&self.snapshot.soup.faces[self.face];self.incidence.entry(Self::key(face[self.corner],face[(self.corner+1)%face.len()])).or_default().push((self.face,self.corner));self.corner+=1;if self.corner==face.len() {self.corner=0;self.face+=1;} }
            }
            2=>{
                if self.cursor==self.edges.len() { self.phase=3;self.cursor=0; }
                else { let (a,b)=self.snapshot.source.edge_endpoints(self.edges[self.cursor])?;let edge=Self::key(a.0,b.0);let owners=&self.incidence[&edge];if owners.len()>2 {return Err(MeshKernelError::NonManifold);}if !owners.iter().any(|&(face,_)|self.snapshot.soup.faces[face].len()==4) {return Err(MeshKernelError::InvalidInput("selected edges must touch a quad".into()));}if self.marked.insert(edge) {self.pending.push(edge);}self.cursor+=1; }
            }
            3=>{
                if self.cursor==self.pending.len() { self.vertices=self.snapshot.soup.positions.len() as u64+self.marked.len() as u64*self.cuts as u64;self.phase=4;self.face=0;self.corner=0; }
                else {let edge=self.pending[self.cursor];let owners=&self.incidence[&edge];if owners.len()>2 {return Err(MeshKernelError::NonManifold);}for &(fi,i) in owners {let face=&self.snapshot.soup.faces[fi];if face.len()==4 {let opposite=Self::key(face[(i+2)%4],face[(i+3)%4]);if self.marked.insert(opposite) {self.pending.push(opposite);}}}self.cursor+=1;}
            }
            4=>{
                if self.face==self.snapshot.soup.faces.len() { self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.face=0;self.corner=0;self.phase=5; }
                else {let face=&self.snapshot.soup.faces[self.face];if face.len()==4 {let (nx,ny)=self.dimensions(face);self.vertices+=((nx-1)*(ny-1)) as u64;self.faces+=(nx*ny) as u64;self.corners+=(nx*ny*4) as u64;self.face+=1;}else {if self.corner==0 {self.faces+=1;self.corners+=face.len() as u64;}if self.marked.contains(&Self::key(face[self.corner],face[(self.corner+1)%face.len()])) {self.corners+=self.cuts as u64;}self.corner+=1;if self.corner==face.len() {self.face+=1;self.corner=0;}}}
                if self.vertices>100_000 || self.faces>100_000 || self.corners>600_000 {return Err(MeshKernelError::InvalidInput("loop cut exceeds mesh capacity".into()));}
            }
            5=>{
                if self.cut_edge.is_none() {
                    let edge=if let Some(last)=self.last_edge {self.marked.range((std::ops::Bound::Excluded(last),std::ops::Bound::Unbounded)).next().copied()}else {self.marked.first().copied()};
                    if let Some(edge)=edge {self.cut_edge=Some(edge);self.last_edge=Some(edge);self.step=1;}else {self.phase=6;self.cursor=0;self.step=0;}
                }else {
                    let (a,b)=self.cut_edge.unwrap();let t=self.step as f64/(self.cuts+1) as f64;let point=[0,1,2].map(|axis|(self.soup.positions[a as usize][axis] as f64*(1.0-t)+self.soup.positions[b as usize][axis] as f64*t) as f32);
                    let previous=self.cut_ids.last().map(|id|self.soup.positions[*id as usize]).unwrap_or(self.soup.positions[a as usize]);if point.iter().any(|x|!x.is_finite()) || point==previous || point==self.soup.positions[b as usize] {return Err(MeshKernelError::DegenerateOperation);}
                    self.cut_ids.push(self.soup.positions.len() as u32);self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Edge {a,b,t});self.step+=1;if self.step>self.cuts {self.cut_vertices.insert((a,b),std::mem::take(&mut self.cut_ids));self.cut_edge=None;}
                }
            }
            6=>{
                if self.face==self.snapshot.soup.faces.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=7;}
                else {
                    let face=&self.snapshot.soup.faces[self.face];
                    if face.len()==4 {
                        if self.cursor==0 && self.building_grid {(self.nx,self.ny)=self.dimensions(face);}
                        if self.building_grid {
                            let (x,y)=(self.cursor%(self.nx+1),self.cursor/(self.nx+1));let id=if y==0 {self.on_edge(face[0],face[1],x,self.nx)}else if y==self.ny {self.on_edge(face[3],face[2],x,self.nx)}else if x==0 {self.on_edge(face[0],face[3],y,self.ny)}else if x==self.nx {self.on_edge(face[1],face[2],y,self.ny)}else {
                                let (u,v)=(x as f64/self.nx as f64,y as f64/self.ny as f64);let weights=[(1.0-u)*(1.0-v),u*(1.0-v),u*v,(1.0-u)*v];let point=[0,1,2].map(|axis|face.iter().zip(weights).map(|(&id,w)|self.soup.positions[id as usize][axis] as f64*w).sum::<f64>() as f32);if point.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}let id=self.soup.positions.len() as u32;self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Weighted {vertices:[face[0],face[1],face[2],face[3]],weights});id
                            };self.grid.push(id);self.cursor+=1;if self.cursor==(self.nx+1)*(self.ny+1) {self.cursor=0;self.building_grid=false;}
                        }else {let (x,y)=(self.cursor%self.nx,self.cursor/self.nx);let i=y*(self.nx+1)+x;self.source_faces.push(self.face as u32);self.soup.faces.push(vec![self.grid[i],self.grid[i+1],self.grid[i+self.nx+2],self.grid[i+self.nx+1]]);self.cursor+=1;if self.cursor==self.nx*self.ny {self.face+=1;self.cursor=0;self.grid.clear();self.building_grid=true;}}
                    }else {
                        let (a,b)=(face[self.corner],face[(self.corner+1)%face.len()]);if self.step==0 {self.boundary.push(a);if self.marked.contains(&Self::key(a,b)) {self.step=1;}else {self.corner+=1;}}else {self.boundary.push(self.on_edge(a,b,self.step,self.cuts+1));self.step+=1;if self.step>self.cuts {self.step=0;self.corner+=1;}}
                        if self.corner==face.len() {self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.boundary));self.corner=0;self.face+=1;}
                    }
                }
            }
            7=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=9;}},
            8=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=1;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            9=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=10;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            10=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

pub(super) struct Mirror {
    pub(super) snapshot: Snapshot,
    pub(super) source_vertices:Vec<u32>,
    pub(super) source_faces:Vec<u32>,
    pub(super) mirrored_faces:Vec<bool>,
    pub(super) attribute_vertices:Vec<u32>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) axis: usize,
    pub(super) tolerance: f32,
    pub(super) positive: bool,
    pub(super) negative: bool,
    pub(super) reflected: Vec<u32>,
    pub(super) soup: Soup,
    pub(super) original: Vec<u32>,
    pub(super) mirrored: Vec<u32>,
    pub(super) emitted_corners: usize,
    pub(super) plane: bool,
    pub(super) vertex: usize,
    pub(super) face: usize,
    pub(super) corner: usize,
    pub(super) build: Option<Build>,
    pub(super) phase: u8,
}

pub(super) struct Subdivide {
    pub(super) snapshot: Snapshot,
    pub(super) input: Vec<FaceId>,
    pub(super) triangulate_only:bool,
    pub(super) delete_selected:bool,
    pub(super) selected: HistoryFoldSet<u32>,
    pub(super) concave: HistoryFoldSet<u32>,
    pub(super) convex: bool,
    pub(super) tessellation: Option<MeshTessellationJob>,
    pub(super) tessellation_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,
    pub(super) transfer: Option<MeshTransfer>,
    pub(super) soup: Soup,
    pub(super) corners: Vec<u32>,
    pub(super) center: Vec3,
    pub(super) center_id: u32,
    pub(super) emitted_corners: usize,
    pub(super) emitting: bool,
    pub(super) cursor: usize,
    pub(super) face: usize,
    pub(super) corner: usize,
    pub(super) triangle: usize,
    pub(super) build: Option<Build>,
    pub(super) vertex_sources:Vec<SourceVertex>,
    pub(super) source_faces:Vec<u32>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) phase: u8,
}

impl Subdivide {
    pub(super) fn phase(&self) -> &'static str { ["subdivide-selection","snapshot","subdivide-convexity","subdivide-triangulate","subdivide-corners","subdivide-reconstruct","subdivide-source-vertices","subdivide-source-remap","subdivide-attributes"][self.phase as usize] }
    fn admit(&mut self,vertices:usize,faces:usize,corners:usize)->MeshResult<()> {
        if self.soup.positions.len().saturating_add(vertices)>100_000 || self.soup.faces.len().saturating_add(faces)>100_000 || self.emitted_corners.saturating_add(corners)>600_000 { return Err(MeshKernelError::InvalidInput("subdivision exceeds mesh capacity".into())); }
        self.emitted_corners+=corners;Ok(())
    }
    fn next_face(&mut self) { self.face += 1; self.corner = 0; self.emitting = false; self.center = Vec3::ZERO; self.convex = true; }
    pub(super) fn advance(&mut self,grant:RetainedCloneGrant,tessellation_progress:&mut RetainedCloneProgress) -> MeshResult<Option<HalfedgeMesh>> {
        if self.tessellation_retirement.is_some(){if close_tessellation_handoff(&mut self.tessellation_retirement,grant,tessellation_progress)?{self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.phase=4;}return Ok(None);}
        match self.phase {
            0 => {
                if self.cursor==self.input.len() {if self.delete_selected && self.selected.len()==self.snapshot.source.face_count() {return Err(MeshKernelError::InvalidInput("deletion would leave an empty mesh".into()));}self.phase = 1;}
                else { let face = self.input[self.cursor]; self.snapshot.source.face_boundary(face)?; if !self.selected.insert(face.0) { return Err(MeshKernelError::InvalidInput("duplicate face selection".into())); } self.cursor += 1; }
            }
            1 => {if self.snapshot.complete() {self.cursor=0;self.phase=6;}}
            2 => {
                if self.face==self.snapshot.soup.faces.len() {
                    if !self.concave.is_empty() { self.tessellation = Some(MeshTessellationJob::selected(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty()),std::mem::take(&mut self.concave))); }
                    self.phase = 3; self.face = 0; self.corner = 0;
                }else if self.delete_selected {self.next_face();}else if self.triangulate_only {if self.snapshot.soup.faces[self.face].len()>3 {self.selected.insert(self.face as u32);self.concave.insert(self.face as u32);}self.next_face();}
                else if !self.selected.contains(&(self.face as u32)) { self.next_face(); }
                else {
                    let face = &self.snapshot.soup.faces[self.face]; let n = face.len(); let point = |index:usize|Vec3(self.snapshot.soup.positions[face[index%n] as usize]);
                    self.convex &= point(self.corner+1).sub(point(self.corner)).cross(point(self.corner+2).sub(point(self.corner+1))).dot(self.snapshot.soup.normals[self.face])>0.0;
                    self.corner += 1;
                    if self.corner==n { if !self.convex { self.concave.insert(self.face as u32); } self.next_face(); }
                }
            }
            3 => {
                if let Some(tessellation) = &mut self.tessellation {
                    let result=tessellation.step(grant);*tessellation_progress=tessellation.normal_step_progress();if let MeshTessellationStep::Done(transfer) = result.map_err(MeshKernelError::Retained)?.0 {self.transfer=Some(transfer);let mut original=self.tessellation.take().unwrap();self.snapshot.source=original.take_source();self.tessellation_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original tessellation handback requires typed retirement")));}
                } else { self.soup.positions = std::mem::take(&mut self.snapshot.soup.positions); self.phase = 4; }
            }
            4 => {
                if self.face==self.snapshot.soup.faces.len() { self.build = Some(Build::unreferenced(std::mem::take(&mut self.soup))); self.phase = 5; }
                else if self.delete_selected && self.selected.contains(&(self.face as u32)) {self.next_face();}
                else if self.delete_selected || !self.selected.contains(&(self.face as u32)) {
                    self.admit(0,usize::from(self.corner+1==self.snapshot.soup.faces[self.face].len()),1)?; let face = &self.snapshot.soup.faces[self.face]; self.corners.push(face[self.corner]); self.corner += 1;
                    if self.corner==face.len() { self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.corners)); self.next_face(); }
                } else if self.transfer.as_ref().is_some_and(|transfer|transfer.face_ids.get(self.triangle)==Some(&(self.face as u32))) {
                    self.admit(if self.triangulate_only {0}else {1},if self.triangulate_only {1}else {3},if self.triangulate_only {3}else {9})?;let transfer=self.transfer.as_ref().unwrap();let ids=[0,1,2].map(|corner|transfer.vertex_ids[transfer.indices[self.triangle*3+corner] as usize]);
                    if self.triangulate_only {self.source_faces.push(self.face as u32);self.soup.faces.push(ids.to_vec());}else {
                        let center=ids.into_iter().fold(Vec3::ZERO,|sum,id|sum.add(Vec3(self.soup.positions[id as usize]))).scale(1.0/3.0);let center_id=self.soup.positions.len() as u32;self.soup.positions.push(center.0);self.vertex_sources.push(SourceVertex::Weighted {vertices:[ids[0],ids[1],ids[2],ids[2]],weights:[1.0/3.0,1.0/3.0,1.0/3.0,0.0]});for corner in 0..3 {self.source_faces.push(self.face as u32);self.soup.faces.push(vec![ids[corner],ids[(corner+1)%3],center_id]);}
                    }
                    self.triangle += 1;
                    if self.transfer.as_ref().unwrap().face_ids.get(self.triangle)!=Some(&(self.face as u32)) { self.next_face(); }
                } else {
                    let face = &self.snapshot.soup.faces[self.face];
                    if !self.emitting {
                        self.center = self.center.add(Vec3(self.soup.positions[face[self.corner] as usize])); self.corner += 1;
                        if self.corner==face.len() { if self.soup.positions.len()>=100_000 { return Err(MeshKernelError::InvalidInput("subdivision exceeds mesh capacity".into())); } self.center_id = self.soup.positions.len() as u32; self.soup.positions.push(self.center.scale(1.0/face.len() as f32).0);self.vertex_sources.push(SourceVertex::Face(self.face as u32)); self.corner = 0; self.emitting = true; }
                    } else {
                        if self.soup.faces.len()>=100_000 || self.emitted_corners.saturating_add(3)>600_000 { return Err(MeshKernelError::InvalidInput("subdivision exceeds mesh capacity".into())); } self.emitted_corners+=3;self.source_faces.push(self.face as u32); self.soup.faces.push(vec![face[self.corner],face[(self.corner+1)%face.len()],self.center_id]); self.corner += 1;
                        if self.corner==face.len() { self.next_face(); }
                    }
                }
            }
            5=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=7;}},
            6=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=2;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            7=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=8;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            8=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _ => unreachable!(),
        }
        Ok(None)
    }
}

impl Mirror {
    pub(super) fn phase(&self) -> &'static str { ["snapshot","mirror-side","mirror-vertices","mirror-corners","mirror-reconstruct","mirror-vertex-provenance","mirror-attributes"][self.phase as usize] }
    pub(super) fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => { if self.snapshot.complete() { self.phase = 1; } }
            1 => {
                if self.vertex == self.snapshot.soup.positions.len() { self.phase = 2; self.vertex = 0; }
                else {
                    self.source_vertices.push(self.vertex as u32);
                    let point = self.snapshot.soup.positions[self.vertex][self.axis];
                    self.positive |= point > self.tolerance; self.negative |= point < -self.tolerance;
                    if self.positive && self.negative { return Err(MeshKernelError::InvalidInput("mirror geometry must lie on one side of its plane".into())); }
                    self.vertex += 1;
                }
            }
            2 => {
                if self.vertex == self.snapshot.source.vertex_count() { self.soup.positions = std::mem::take(&mut self.snapshot.soup.positions); self.phase = 3; }
                else {
                    let mut point = self.snapshot.soup.positions[self.vertex];
                    if point[self.axis].abs() <= self.tolerance { self.snapshot.soup.positions[self.vertex][self.axis] = 0.0; self.reflected.push(self.vertex as u32); }
                    else { if self.snapshot.soup.positions.len()>=100_000 { return Err(MeshKernelError::InvalidInput("mirror exceeds mesh capacity".into())); } point[self.axis] = -point[self.axis]; self.reflected.push(self.snapshot.soup.positions.len() as u32); self.snapshot.soup.positions.push(point);self.source_vertices.push(self.vertex as u32); }
                    self.vertex += 1;
                }
            }
            3 => {
                if self.face == self.snapshot.soup.faces.len() { self.build = Some(Build::new(std::mem::take(&mut self.soup),false)); self.phase = 4; }
                else {
                    let face = &self.snapshot.soup.faces[self.face];
                    let vertex = face[self.corner]; self.plane &= self.soup.positions[vertex as usize][self.axis] == 0.0;
                    self.original.push(vertex); self.mirrored.push(self.reflected[face[face.len()-1-self.corner] as usize]); self.corner += 1;
                    if self.corner == face.len() {
                        if !self.plane { if self.soup.faces.len().saturating_add(2)>100_000 || self.emitted_corners.saturating_add(self.original.len().saturating_mul(2))>600_000 { return Err(MeshKernelError::InvalidInput("mirror exceeds mesh capacity".into())); } self.emitted_corners+=self.original.len()*2; self.source_faces.extend_from_slice(&[self.face as u32,self.face as u32]);self.mirrored_faces.extend_from_slice(&[false,true]);self.soup.faces.push(std::mem::take(&mut self.original)); self.soup.faces.push(std::mem::take(&mut self.mirrored)); }
                        else { self.original.clear(); self.mirrored.clear(); }
                        self.face += 1; self.corner = 0; self.plane = true;
                    }
                }
            }
            4=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),Some(std::mem::take(&mut self.mirrored_faces)),Some(self.axis)));self.vertex=0;self.phase=5;}},
            5=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.vertex==origins.len() {self.attributes.as_mut().unwrap().configuration_mut()?.vertices=Some(std::mem::take(&mut self.attribute_vertices));self.phase=6;}else {self.attribute_vertices.push(self.source_vertices[origins[self.vertex] as usize]);self.vertex+=1;}},
            6=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _ => unreachable!(),
        }
        Ok(None)
    }
}
