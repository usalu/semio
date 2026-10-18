//! 🎈️ Inflate, orient and fill holes: the inflate, orient and hole filling work records with their stepping. See [`super`].

use super::*;

pub(super) struct Inflate {
    pub(super) snapshot:Snapshot,
    pub(super) input:Vec<FaceId>,
    pub(super) selected:HistoryFoldSet<u32>,
    pub(super) amount:f32,
    pub(super) inset:bool,
    pub(super) normals:HistoryFoldIndex<u32,Vec3>,
    pub(super) boundary:HistoryFoldIndex<(u32,u32),(u32,u32,usize,u32)>,
    pub(super) edges:HistoryFoldSet<(u32,u32)>,
    pub(super) displaced:HistoryFoldIndex<u32,u32>,
    pub(super) last_vertex:Option<u32>,
    pub(super) last_edge:Option<(u32,u32)>,
    pub(super) soup:Soup,
    pub(super) borders:Vec<Vec<u32>>,
    pub(super) inner:Vec<u32>,
    pub(super) corners:Vec<u32>,
    pub(super) cursor:usize,
    pub(super) face:usize,
    pub(super) corner:usize,
    pub(super) emitted_corners:usize,
    pub(super) build:Option<Build>,
    pub(super) vertex_sources:Vec<SourceVertex>,
    pub(super) source_faces:Vec<u32>,
    pub(super) border_faces:Vec<u32>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) phase:u8,
}

impl Inflate {
    pub(super) fn phase(&self)->&'static str { if self.inset {["inset-selection","snapshot","inset-vertices","inset-borders","inset-corners","inset-transfer","inset-reconstruct","inset-source-vertices","inset-source-remap","inset-attributes"][self.phase as usize]}else {["extrude-selection","snapshot","extrude-boundaries","extrude-vertices","extrude-corners","extrude-borders","extrude-reconstruct","extrude-source-vertices","extrude-source-remap","extrude-attributes"][self.phase as usize]} }
    fn admit_position(&self)->MeshResult<()> {if self.soup.positions.len()>=100_000 {Err(MeshKernelError::InvalidInput("face expansion exceeds vertex capacity".into()))}else {Ok(())}}
    fn admit_face(&mut self,corners:usize)->MeshResult<()> {if self.soup.faces.len()>=100_000 || self.emitted_corners.saturating_add(corners)>600_000 {return Err(MeshKernelError::InvalidInput("face expansion exceeds mesh capacity".into()));}self.emitted_corners+=corners;Ok(())}
    pub(super) fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.cursor==self.input.len() {self.cursor=0;self.phase=1;}else {let face=self.input[self.cursor];self.snapshot.source.face_boundary(face)?;if !self.selected.insert(face.0) {return Err(MeshKernelError::InvalidInput("duplicate selected face".into()));}self.cursor+=1;}}
            1=>{if self.snapshot.complete() {self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=7;}}
            2=>{
                if self.cursor==self.input.len() {
                    if self.inset {self.phase=4;self.face=0;self.corner=0;}else {if self.snapshot.soup.faces.len().saturating_add(self.edges.len())>100_000 || self.snapshot.source.halfedge_count().saturating_add(self.edges.len().saturating_mul(4))>600_000 {return Err(MeshKernelError::InvalidInput("extrusion exceeds mesh capacity".into()));}self.phase=3;}
                }else {
                    let id=self.input[self.cursor].0 as usize;let face=&self.snapshot.soup.faces[id];let n=face.len();let normal=self.snapshot.soup.normals[id];
                    if self.inset {
                        self.admit_position()?;let face=&self.snapshot.soup.faces[id];let point=|i:usize|Vec3(self.soup.positions[face[i] as usize]);let previous=point((self.corner+n-1)%n);let current=point(self.corner);let next=point((self.corner+1)%n);let a=normal.cross(current.sub(previous).normalize());let b=normal.cross(next.sub(current).normalize());let denominator=1.0+a.dot(b);if denominator<=1e-6 {return Err(MeshKernelError::DegenerateOperation);}let offset=a.add(b).scale(self.amount/denominator);if offset.length()>=current.sub(previous).length().min(next.sub(current).length())*0.5 {return Err(MeshKernelError::InvalidInput("inset exceeds local edge clearance".into()));}let position=current.add(offset).0;if position.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.inner.push(self.soup.positions.len() as u32);self.soup.positions.push(position);self.vertex_sources.push(SourceVertex::Original(face[self.corner]));
                    }else {let (a,b)=(face[self.corner],face[(self.corner+1)%n]);self.normals.entry(a).and_modify(|sum|*sum=sum.add(normal)).or_insert_with(||normal);let edge=self.boundary.entry((a.min(b),a.max(b))).or_insert_with(||(a,b,0,id as u32));if edge.2==0 {self.edges.insert((a,b));}else {self.edges.remove(&(edge.0,edge.1));}edge.2+=1;}
                    self.corner+=1;if self.corner==n {self.corner=0;if self.inset {self.phase=3;}else {self.cursor+=1;}}
                }
            }
            3=>{
                if self.inset {
                    let id=self.input[self.cursor].0 as usize;let face=&self.snapshot.soup.faces[id];let next=(self.corner+1)%face.len();if self.snapshot.soup.faces.len().saturating_add(self.borders.len()).saturating_add(1)>100_000 || self.snapshot.source.halfedge_count().saturating_add((self.borders.len()+1).saturating_mul(4))>600_000 {return Err(MeshKernelError::InvalidInput("inset exceeds mesh capacity".into()));}self.borders.push(vec![face[self.corner],face[next],self.inner[next],self.inner[self.corner]]);self.border_faces.push(id as u32);self.corner+=1;if self.corner==face.len() {self.snapshot.soup.faces[id]=std::mem::take(&mut self.inner);self.cursor+=1;self.corner=0;self.phase=2;}
                }else {
                    let next=if let Some(last)=self.last_vertex {self.normals.range((std::ops::Bound::Excluded(last),std::ops::Bound::Unbounded)).next().map(|(&id,&normal)|(id,normal))}else {self.normals.first_key_value().map(|(&id,&normal)|(id,normal))};
                    if let Some((id,normal))=next {self.admit_position()?;let normal=normal.normalize();if normal.length()<1e-6 {return Err(MeshKernelError::DegenerateOperation);}let point=Vec3(self.soup.positions[id as usize]).add(normal.scale(self.amount)).0;if point.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.displaced.insert(id,self.soup.positions.len() as u32);self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Original(id));self.last_vertex=Some(id);}else {self.phase=4;self.face=0;self.corner=0;}
                }
            }
            4=>{
                if self.face==self.snapshot.soup.faces.len() {self.phase=5;self.cursor=0;}
                else {self.admit_face(1)?;let face=&self.snapshot.soup.faces[self.face];let id=face[self.corner];self.corners.push(if !self.inset && self.selected.contains(&(self.face as u32)) {self.displaced[&id]}else {id});self.corner+=1;if self.corner==face.len() {self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.corners));self.corner=0;self.face+=1;}}
            }
            5=>{
                if self.inset {if self.cursor==self.borders.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=6;}else {self.admit_face(4)?;self.source_faces.push(self.border_faces[self.cursor]);self.soup.faces.push(std::mem::take(&mut self.borders[self.cursor]));self.cursor+=1;}}
                else {let next=if let Some(last)=self.last_edge {self.edges.range((std::ops::Bound::Excluded(last),std::ops::Bound::Unbounded)).next().copied()}else {self.edges.first().copied()};if let Some((a,b))=next {self.admit_face(4)?;self.source_faces.push(self.boundary[&(a.min(b),a.max(b))].3);self.soup.faces.push(vec![a,b,self.displaced[&b],self.displaced[&a]]);self.last_edge=Some((a,b));}else {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=6;}}
            }
            6=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=8;}},
            7=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=2;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            8=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=9;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize]);self.cursor+=1;}},
            9=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

pub(super) struct Orient {
    pub(super) input:Option<Vec<FaceId>>,
    pub(super) flip_count:usize,
    pub(super) snapshot:Snapshot,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) edges:HistoryFoldIndex<(u32,u32),Vec<(usize,bool)>>,
    pub(super) keys:Vec<(u32,u32)>,
    pub(super) adjacency:Vec<Vec<(usize,bool)>>,
    pub(super) oriented:Vec<bool>,
    pub(super) flipped:Vec<bool>,
    pub(super) stack:Vec<usize>,
    pub(super) active:Option<usize>,
    pub(super) face:usize,
    pub(super) corner:usize,
    pub(super) cursor:usize,
    pub(super) neighbor:usize,
    pub(super) reversing:usize,
    pub(super) incidence:u8,
    pub(super) build:Option<Build>,
    pub(super) phase:u8,
}

impl Orient {
    pub(super) fn phase(&self)->&'static str { ["snapshot","orient-initialize","orient-incidence","orient-adjacency","orient-components","orient-corners","orient-reconstruct","orient-attributes","orient-selection"][self.phase as usize] }
    pub(super) fn initialization_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        let count=self.snapshot.soup.faces.len();
        let (slots,bytes)=if self.adjacency.capacity()<count {(count,size_of::<Vec<(usize,bool)>>())}else if self.oriented.capacity()<count||self.flipped.capacity()<count {(count,size_of::<bool>())}else {(0,0)};
        let capacity=slots.checked_mul(bytes).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original orientation initialization backing extent overflow"))?;
        let copy=if slots!=0||self.cursor==count {0}else {size_of::<Vec<(usize,bool)>>()+2*size_of::<bool>()};
        Ok(semio_framework_value::RetirementDemand {copy_bytes:copy,capacity_bytes:capacity,depth:3,..Default::default()})
    }
    pub(super) fn incidence_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if self.face==self.snapshot.soup.faces.len() {return Ok(RetirementDemand {depth:3,..Default::default()});}
        let face=&self.snapshot.soup.faces[self.face];let(a,b)=(face[self.corner],face[(self.corner+1)%face.len()]);let key=(a.min(b),a.max(b));
        let(copy_bytes,capacity_bytes,child_depth)=match self.incidence {
            0=>{let membership=self.edges.next_contains_depth_demand(&key)?;if self.edges.contains_key(&key) {(0,0,membership.max(1))}else {(0,self.edges.next_insert_capacity_byte_demand(&key,copy)?,membership.max(self.edges.next_insert_depth_demand(&key)?))}},
            1=>{if self.keys.capacity()==0 {(0,self.snapshot.source.halfedges.len().checked_mul(size_of::<(u32,u32)>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original incidence key backing extent overflow"))?,1)}else {(size_of::<(u32,u32)>(),0,1)}},
            2=>{let owners=self.edges.get(&key).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original incidence key lost its admitted row"))?;if owners.len()>=2 {return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original incidence edge has more than two manifold owners"));}let capacity=if owners.capacity()==0 {2*size_of::<(usize,bool)>()}else {0};(if capacity!=0 {0}else {size_of::<(usize,bool)>()},capacity,self.edges.next_contains_depth_demand(&key)?.max(1))},
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original incidence substage is invalid")),
        };
        let depth=child_depth.checked_add(2).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original incidence parent depth overflow"))?;
        Ok(RetirementDemand {copy_bytes,capacity_bytes,depth,..Default::default()})
    }
    pub(super) fn adjacency_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if self.cursor==self.keys.len() {return Ok(RetirementDemand {depth:3,..Default::default()});}
        let key=self.keys[self.cursor];let owners=self.edges.get(&key).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original adjacency lost its edge owner row"))?;
        let depth=self.edges.next_contains_depth_demand(&key)?.max(1).checked_add(2).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original adjacency parent depth overflow"))?;
        if owners.len()!=2 {return Ok(RetirementDemand {depth,..Default::default()});}
        let face=owners.get(self.incidence as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original adjacency reciprocal substage is invalid"))?.0;
        let row=&self.adjacency[face];
        let capacity_bytes=if row.capacity()==0 {self.snapshot.soup.faces[face].len().checked_mul(size_of::<(usize,bool)>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original adjacency backing extent overflow"))?}else {0};
        if capacity_bytes==0&&row.len()==row.capacity() {return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original adjacency exceeds its face corner backing"));}
        Ok(RetirementDemand {copy_bytes:if capacity_bytes==0 {size_of::<(usize,bool)>()}else {0},capacity_bytes,depth,..Default::default()})
    }
    pub(super) fn components_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        let copy_bytes=if let Some(face)=self.active {self.adjacency[face].get(self.neighbor).filter(|(neighbor,_)|!self.oriented[*neighbor]).map_or(0,|_|size_of::<usize>()+2*size_of::<bool>())}else if self.stack.is_empty()&&self.cursor<self.oriented.len()&&!self.oriented[self.cursor] {size_of::<usize>()+size_of::<bool>()}else {0};
        let capacity_bytes=if copy_bytes!=0&&self.stack.capacity()==0 {self.oriented.len().checked_mul(size_of::<usize>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original component stack backing extent overflow"))?}else {0};
        if copy_bytes!=0&&capacity_bytes==0&&self.stack.len()==self.stack.capacity() {return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original component stack exceeds its face backing"));}
        Ok(RetirementDemand {copy_bytes:if capacity_bytes!=0 {0}else {copy_bytes},capacity_bytes,depth:3,..Default::default()})
    }
    pub(super) fn corners_demands(&self)->semio_framework_value::RetirementDemand {
        let swapping=self.snapshot.soup.faces.get(self.face).is_some_and(|face|self.flipped[self.face]&&self.reversing<face.len()/2);
        semio_framework_value::RetirementDemand {copy_bytes:if swapping {2*size_of::<u32>()}else {0},depth:3,..Default::default()}
    }
    pub(super) fn advance(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.complete() {self.phase=1;}}
            1=>{
                let count=self.snapshot.soup.faces.len();let demand=self.initialization_demands().map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if demand.capacity_bytes!=0 {
                    let result=if self.adjacency.capacity()<count {self.adjacency.try_reserve_exact(count).map(|()|self.adjacency.capacity()*size_of::<Vec<(usize,bool)>>())}else if self.oriented.capacity()<count {self.oriented.try_reserve_exact(count).map(|()|self.oriented.capacity()*size_of::<bool>())}else {self.flipped.try_reserve_exact(count).map(|()|self.flipped.capacity()*size_of::<bool>())};
                    let bytes=result.map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original orientation initialization backing reservation failed")))?;
                    *receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()};
                    if bytes>grant.maximum_capacity_bytes {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original orientation allocator exceeded admitted backing").with_retained_progress(*receipt)));}
                }else {
                    if self.cursor==count {self.phase=if self.input.is_some() {8}else {2};self.cursor=0;}else {self.adjacency.push(Vec::new());self.oriented.push(false);self.flipped.push(false);self.cursor+=1;}
                    *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
                }
            }
            2=>{
                let demand=self.incidence_demands(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if self.face==self.snapshot.soup.faces.len() {self.phase=3;self.face=0;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                else {
                    let face=&self.snapshot.soup.faces[self.face];let(a,b,count)=(face[self.corner],face[(self.corner+1)%face.len()],face.len());let key=(a.min(b),a.max(b));
                    match self.incidence {
                        0=>{
                            let child=RetainedCloneGrant {maximum_depth:grant.maximum_depth-2,..grant};
                            if self.edges.contains_key(&key) {self.incidence=2;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                            else if demand.capacity_bytes!=0 {match self.edges.reserve_insert_step(&key,child) {Ok(progress)=>*receipt=progress,Err((error,progress))=>{*receipt=progress;return Err(MeshKernelError::Retained(error.with_retained_progress(progress)));}}}
                            else {let(previous,progress)=self.edges.insert_reserved(key,Vec::new(),child).map_err(|(error,_,_)|MeshKernelError::Retained(error))?;assert!(previous.is_none(),"original incidence vacancy changed during its admitted turn");*receipt=progress;self.incidence=1;}
                        }
                        1=>{
                            if demand.capacity_bytes!=0 {self.keys.try_reserve_exact(self.snapshot.source.halfedges.len()).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original incidence key backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:self.keys.capacity()*size_of::<(u32,u32)>(),..Default::default()};}
                            else {if self.keys.len()==self.keys.capacity() {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original incidence encounter key exceeds admitted backing")));}self.keys.push(key);self.incidence=2;*receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};}
                        }
                        2=>{
                            let owners=self.edges.get_mut(&key).unwrap();
                            if demand.capacity_bytes!=0 {owners.try_reserve_exact(2).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original incidence edge owner backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:owners.capacity()*size_of::<(usize,bool)>(),..Default::default()};}
                            else {if owners.len()==owners.capacity() {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original incidence owner row exceeds admitted backing")));}owners.push((self.face,a<b));self.incidence=0;self.corner+=1;if self.corner==count {self.face+=1;self.corner=0;}*receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};}
                        }
                        _=>unreachable!(),
                    }
                    if !receipt.fits(grant) {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original incidence allocator exceeded its admitted backing").with_retained_progress(*receipt)));}
                }
            }
            3=>{
                let demand=self.adjacency_demands().map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if self.cursor==self.keys.len() {self.phase=4;self.cursor=0;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                else {
                    let owners=&self.edges[&self.keys[self.cursor]];
                    if owners.len()!=2 {self.cursor+=1;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                    else {
                        let(face,_)=owners[self.incidence as usize];let(neighbor,_)=owners[1-self.incidence as usize];let relative=owners[0].1==owners[1].1;
                        let row=&mut self.adjacency[face];
                        if demand.capacity_bytes!=0 {row.try_reserve_exact(self.snapshot.soup.faces[face].len()).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original adjacency backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:row.capacity()*size_of::<(usize,bool)>(),..Default::default()};}
                        else {row.push((neighbor,relative));if self.incidence==0 {self.incidence=1;}else {self.incidence=0;self.cursor+=1;}*receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};}
                        if !receipt.fits(grant) {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original adjacency allocator exceeded its admitted backing").with_retained_progress(*receipt)));}
                    }
                }
            }
            4=>{
                let demand=self.components_demands().map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if demand.capacity_bytes!=0 {self.stack.try_reserve_exact(self.oriented.len()).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original component stack backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:self.stack.capacity()*size_of::<usize>(),..Default::default()};if !receipt.fits(grant) {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original component allocator exceeded its admitted backing").with_retained_progress(*receipt)));}return Ok(None);}
                if let Some(face)=self.active {
                    if self.neighbor==self.adjacency[face].len() {self.active=None;}
                    else {let (neighbor,relative)=self.adjacency[face][self.neighbor];if !self.oriented[neighbor] {self.oriented[neighbor]=true;self.flipped[neighbor]=self.flipped[face]^relative;self.flip_count+=usize::from(self.flipped[neighbor]);self.stack.push(neighbor);}self.neighbor+=1;}
                }else if let Some(face)=self.stack.pop() {self.active=Some(face);self.neighbor=0;}
                else if self.cursor==self.oriented.len() {self.phase=5;self.face=0;}
                else {if !self.oriented[self.cursor] {self.oriented[self.cursor]=true;self.stack.push(self.cursor);}self.cursor+=1;}
                *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
            }
            5=>{
                let demand=self.corners_demands();
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if self.face==self.snapshot.soup.faces.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.phase=6;}
                else {let face=&mut self.snapshot.soup.faces[self.face];if self.flipped[self.face] && self.reversing<face.len()/2 {let opposite=face.len()-1-self.reversing;face.swap(self.reversing,opposite);self.reversing+=1;}else {self.face+=1;self.reversing=0;}}
                *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
            }
            6=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,None,None,None));self.phase=7;}},
            7=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            8=>{let input=self.input.as_ref().unwrap();if self.cursor==input.len() {self.face=0;self.cursor=0;self.phase=5;}else {let id=input[self.cursor];self.snapshot.source.face_boundary(id)?;self.flipped[id.0 as usize]=!self.flipped[id.0 as usize];self.cursor+=1;}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

pub(super) struct FillHoles {
    pub(super) snapshot:Snapshot,
    pub(super) visited:HistoryFoldSet<u32>,
    pub(super) vertices:Vec<u32>,
    pub(super) boundary_edges:Vec<u32>,
    pub(super) cap:Vec<u32>,
    pub(super) cap_faces:Vec<u32>,
    pub(super) corner_sources:Vec<Option<u32>>,
    pub(super) edge_sources:Vec<Option<u32>>,
    pub(super) face_sources:Vec<Vec<u32>>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) cursor:usize,
    pub(super) start:u32,
    pub(super) probe:u32,
    pub(super) rotations:usize,
    pub(super) corner:usize,
    pub(super) corners:usize,
    pub(super) filled:usize,
    pub(super) build:Option<Build>,
    pub(super) phase:u8,
}

impl FillHoles {
    pub(super) fn phase(&self)->&'static str { ["snapshot","fill-holes-boundaries","fill-holes-rotate","fill-holes-corners","fill-holes-reconstruct","fill-holes-source-corners","fill-holes-source-faces","fill-holes-attributes"][self.phase as usize] }
    pub(super) fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.complete() {self.cursor=0;self.phase=5;}}
            1=>{
                if self.cursor==self.snapshot.source.halfedge_count() {if self.filled==0 {return Ok(Some(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty())));}self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.phase=4;}
                else {let id=self.cursor as u32;let edge=&self.snapshot.source.halfedges[self.cursor];self.cursor+=1;if edge.twin.is_none() && self.visited.insert(id) {self.start=id;self.vertices.push(edge.vertex);self.boundary_edges.push(id);self.probe=edge.next;self.rotations=0;self.phase=2;}}
            }
            2=>{
                let edge=&self.snapshot.source.halfedges[self.probe as usize];
                if let Some(twin)=edge.twin {self.probe=self.snapshot.source.halfedges[twin as usize].next;self.rotations+=1;if self.rotations>self.snapshot.source.halfedge_count()+4 {self.vertices.clear();self.boundary_edges.clear();self.phase=1;}}
                else if self.probe==self.start {if self.vertices.len()>=3 {if self.snapshot.soup.faces.len()>=100_000 || self.corners.saturating_add(self.vertices.len())>600_000 {return Err(MeshKernelError::InvalidInput("hole filling exceeds mesh capacity".into()));}self.corners+=self.vertices.len();self.corner=0;self.phase=3;}else {self.vertices.clear();self.boundary_edges.clear();self.phase=1;}}
                else if !self.visited.insert(self.probe) {self.vertices.clear();self.boundary_edges.clear();self.phase=1;}
                else {self.vertices.push(edge.vertex);self.boundary_edges.push(self.probe);self.probe=edge.next;self.rotations=0;}
            }
            3=>{let index=self.vertices.len()-1-self.corner;let corner=self.boundary_edges[index];let edge=self.boundary_edges[(index+self.vertices.len()-1)%self.vertices.len()];self.cap.push(self.vertices[index]);self.corner_sources.push(Some(corner));self.edge_sources.push(Some(edge));self.cap_faces.push(self.snapshot.source.halfedges[edge as usize].face.unwrap());self.corner+=1;if self.corner==self.vertices.len() {self.snapshot.soup.faces.push(std::mem::take(&mut self.cap));self.face_sources.push(std::mem::take(&mut self.cap_faces));self.filled+=1;self.vertices.clear();self.boundary_edges.clear();self.phase=1;}},
            4=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let mut attributes=SourceAttributeRemap::new(mesh,None,None,None,None);attributes.configuration_mut()?.corner_sources=Some(std::mem::take(&mut self.corner_sources));attributes.configuration_mut()?.edge_sources=Some(std::mem::take(&mut self.edge_sources));attributes.configuration_mut()?.generated_faces=Some(std::mem::take(&mut self.face_sources));self.attributes=Some(attributes);self.phase=7;}},
            5=>{if self.cursor==self.snapshot.source.halfedge_count() {self.cursor=0;self.phase=6;}else {self.corner_sources.push(None);self.edge_sources.push(None);self.cursor+=1;}},
            6=>{if self.cursor==self.snapshot.source.face_count() {self.cursor=0;self.phase=1;}else {self.face_sources.push(vec![self.cursor as u32]);self.cursor+=1;}},
            7=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}
