//! 🔄️ Transform and knife: the transform selection, the position policy, the transform work and the knife work. See [`super`].

use super::*;

pub(super) struct TransformSelection {
    pub(super) vertices:Vec<VertexId>,pub(super) edges:Vec<EdgeId>,pub(super) faces:Vec<FaceId>,pub(super) pivot:Option<Vec3>,pub(super) selected:HistoryFoldSet<usize>,pub(super) sum:[f64;3],pub(super) mode:u8,pub(super) cursor:usize,pub(super) next:Option<u32>,pub(super) start:u32,
}

impl TransformSelection {
    pub(super) fn add(&mut self,mesh:&HalfedgeMesh,id:usize)->MeshResult<()> {let point=mesh.vertices.get(id).ok_or(MeshKernelError::InvalidHandle)?.position;if self.selected.insert(id) {for axis in 0..3 {self.sum[axis]+=point[axis] as f64;}}Ok(())}
    pub(super) fn advance(&mut self,mesh:&HalfedgeMesh)->MeshResult<bool> {
        match self.mode {
            0=>{if self.cursor==self.vertices.len() {self.cursor=0;self.mode=1;}else {self.add(mesh,self.vertices[self.cursor].0 as usize)?;self.cursor+=1;}}
            1=>{if self.cursor==self.edges.len() {self.cursor=0;self.mode=2;}else {let (a,b)=mesh.edge_endpoints(self.edges[self.cursor])?;self.add(mesh,a.0 as usize)?;self.add(mesh,b.0 as usize)?;self.cursor+=1;}}
            2=>{if self.cursor==self.faces.len() {return Ok(true)}if let Some(next)=self.next {let halfedge=mesh.halfedges.get(next as usize).ok_or(MeshKernelError::InvalidHandle)?;let(vertex,next)=(halfedge.vertex,halfedge.next);self.add(mesh,vertex as usize)?;self.next=if next==self.start {self.cursor+=1;None}else {Some(next)};}else {self.start=mesh.face_boundary(self.faces[self.cursor])?.0.0;self.next=Some(self.start);}}
            _=>unreachable!(),
        }Ok(false)
    }
}

pub(super) enum PositionPolicy { Affine,Matrix(Affine3),Grid(f32),Proportional{pivot:Vec3,radius:f32} }

pub(super) struct Transform {
    pub(super) mesh:HalfedgeMesh,
    pub(super) policy:PositionPolicy,
    pub(super) matrix:[[f32;3];3],
    pub(super) offset:[f32;3],
    pub(super) normal_matrix:[[f64;3];3],
    pub(super) flip:bool,
    pub(super) recompute:bool,
    pub(super) flat:Vec<Option<Vec3>>,
    pub(super) smooth:Vec<Vec3>,
    pub(super) hes:Vec<u32>,
    pub(super) normal:Normal,
    pub(super) face_normal:Vec3,
    pub(super) face:usize,
    pub(super) vertex:usize,
    pub(super) cursor:usize,
    pub(super) start:u32,
    pub(super) next:u32,
    pub(super) attribute_names:Vec<String>,
    pub(super) attribute:usize,
    pub(super) selection:Option<TransformSelection>,
    pub(super) component_matrix:Option<[[f64;3];3]>,
    pub(super) component_pivot:[f32;3],
    pub(super) transform_attributes:bool,
    pub(super) attribute_values:Vec<semio_framework_value::DslValue>,
    pub(super) attribute_indices:Vec<u32>,
    pub(super) attribute_samples:HistoryFoldIndex<(u32,bool),u32>,
    pub(super) attribute_retired:Vec<semio_framework_value::DslValue>,
    pub(super) attribute_affected:Option<bool>,
    pub(super) attribute_corner:Option<u32>,
    pub(super) phase:u8,
}

impl Transform {
    fn directional_value(value:&semio_framework_value::DslValue,matrix:[[f64;3];3],tangent:bool,reflected:bool)->MeshResult<semio_framework_value::DslValue> {
        let values=value.as_array().ok_or_else(||MeshKernelError::InvalidInput("invalid authored direction".into()))?;
        if values.len()!=if tangent {4}else {3} {return Err(MeshKernelError::InvalidInput("invalid authored direction dimensions".into()));}
        let coordinate=|axis:usize|values[axis].as_f64().filter(|value|value.is_finite()).ok_or_else(||MeshKernelError::InvalidInput("invalid authored direction coordinate".into()));let point=[coordinate(0)?,coordinate(1)?,coordinate(2)?];
        let direction=[0,1,2].map(|axis|matrix[axis][0]*point[0]+matrix[axis][1]*point[1]+matrix[axis][2]*point[2]);let length=direction[0].hypot(direction[1]).hypot(direction[2]);let direction=direction.map(|value|value/length);
        if direction.iter().any(|value|!value.is_finite() || !(*value as f32).is_finite()) {return Err(MeshKernelError::DegenerateOperation);}
        let mut output=direction.into_iter().map(semio_framework_value::DslValue::float).collect::<Vec<_>>();
        if tangent {let handedness=values[3].as_f64().filter(|value|*value==1.0 || *value== -1.0).ok_or_else(||MeshKernelError::InvalidInput("invalid authored tangent handedness".into()))?;output.push(semio_framework_value::DslValue::float(if reflected {-handedness}else {handedness}));}
        Ok(semio_framework_value::DslValue::Array(output))
    }
    fn directional_matrix(&self,tangent:bool)->([[f64;3];3],bool) {if !tangent {return (self.normal_matrix,false)}let matrix=match self.policy {PositionPolicy::Matrix(matrix)=>matrix.linear.rows,_=>self.component_matrix.unwrap_or_else(||self.matrix.map(|row|row.map(f64::from)))};(matrix,Mat3::from_rows(matrix).determinant()<0.0)}
    pub(super) fn phase(&self)->&'static str {["transform-vertices","transform-winding","transform-initialize","transform-corners","transform-normal","transform-scatter","transform-vertex-normal","transform-attributes","transform-done","transform-selection","transform-attribute-release","transform-selection-release","transform-attribute-face"][self.phase as usize]}
    pub(super) fn halfedge(&self,index:usize)->u32 {self.hes[if self.mesh.faces[self.face].flipped {self.hes.len()-1-index}else {index}]}
    pub(super) fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{
                if self.vertex==self.mesh.vertex_count() {self.vertex=0;self.phase=if self.recompute {1}else {8};}else {
                    let selected=self.selection.as_ref().is_none_or(|selection|selection.selected.contains(&self.vertex));
                    if selected || matches!(self.policy,PositionPolicy::Proportional {..}) {
                        let p=self.mesh.vertices[self.vertex].position;let result=match self.policy {
                            PositionPolicy::Matrix(matrix)=>matrix.apply_point(Pnt3::from_array(p.map(f64::from))).to_array().map(|value|value as f32),
                            PositionPolicy::Grid(grid)=>p.map(|coordinate|((coordinate as f64/grid as f64).round()*grid as f64) as f32),
                            PositionPolicy::Proportional {pivot,radius}=>{let weight=if selected {1.0}else {(1.0-Vec3(p).sub(pivot).length()/radius).max(0.0)};Vec3(p).add(Vec3(self.offset).scale(weight)).0},
                            PositionPolicy::Affine=>if let Some(matrix)=self.component_matrix {let relative=[0,1,2].map(|axis|p[axis] as f64-self.component_pivot[axis] as f64);[0,1,2].map(|axis|(self.component_pivot[axis] as f64+matrix[axis][0]*relative[0]+matrix[axis][1]*relative[1]+matrix[axis][2]*relative[2]+self.offset[axis] as f64) as f32)}else {[0,1,2].map(|axis|self.matrix[axis][0]*p[0]+self.matrix[axis][1]*p[1]+self.matrix[axis][2]*p[2]+self.offset[axis])},
                        };if result.iter().any(|value|!value.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.mesh.vertices[self.vertex].position=result;
                    }self.vertex+=1;
                }
            }

            1=>{if !self.flip || self.face==self.mesh.face_count() {self.face=0;self.phase=2;}else {self.mesh.faces[self.face].flipped=!self.mesh.faces[self.face].flipped;self.face+=1;}}
            2=>{if self.vertex==self.mesh.vertex_count() {self.vertex=0;self.phase=3;}else {self.flat.push(None);self.smooth.push(Vec3::ZERO);self.vertex+=1;}}
            3=>{
                if self.face==self.mesh.face_count() {self.phase=6;self.vertex=0;}
                else {if self.hes.is_empty() {self.start=self.mesh.faces[self.face].halfedge;self.next=self.start;}let edge=&self.mesh.halfedges[self.next as usize];self.hes.push(self.next);self.next=edge.next;if self.next==self.start {self.cursor=0;self.phase=4;}}
            }
            4=>{let n=self.hes.len();let point=|i:usize|Vec3(self.mesh.vertices[self.mesh.halfedges[self.halfedge(i%n) as usize].vertex as usize].position);let (a,b)=(point(self.cursor),point(self.cursor+1));if self.cursor==0 {self.normal.set_origin(a);}self.normal.add(a,b);self.cursor+=1;if self.cursor==n {self.face_normal=self.normal.value();self.cursor=0;self.phase=5;}}
            5=>{let vertex=self.mesh.halfedges[self.halfedge(self.cursor) as usize].vertex as usize;if self.flat[vertex].is_none() {self.flat[vertex]=Some(self.face_normal);}if self.mesh.faces[self.face].smooth {self.smooth[vertex]=self.smooth[vertex].add(self.face_normal);}self.cursor+=1;if self.cursor==self.hes.len() {self.face+=1;self.hes.clear();self.normal=Normal::default();self.phase=3;}}
            6=>{if self.vertex==self.mesh.vertex_count() {if !self.transform_attributes {self.phase=if self.selection.is_some() {11}else {8};return Ok(None)}self.attribute_names=self.mesh.attributes.iter().filter_map(|(name,attribute)|(attribute.semantic==MeshAttributeSemantic::Normal || name=="tangent" && attribute.semantic==MeshAttributeSemantic::Custom).then_some(name.clone())).collect();self.attribute=0;self.cursor=0;self.phase=7;}else {self.mesh.vertices[self.vertex].normal=if self.smooth[self.vertex].length()>0.0 {Some(self.smooth[self.vertex].normalize().0)}else {self.flat[self.vertex].map(|n|n.0)};self.vertex+=1;}}
            7=>{
                if self.attribute==self.attribute_names.len() {self.phase=if self.selection.is_some() {11}else {8};}
                else {
                    let name=&self.attribute_names[self.attribute];let attribute=self.mesh.attributes.get(name).unwrap();let (matrix,reflected)=self.directional_matrix(name=="tangent");
                    if self.selection.is_some() {
                        if self.cursor==attribute.domain_len() {let attribute=self.mesh.attributes.get_mut(name).unwrap();self.attribute_retired=std::mem::replace(&mut attribute.values,std::mem::take(&mut self.attribute_values));attribute.indices=Some(std::mem::take(&mut self.attribute_indices));self.cursor=0;self.phase=10;}
                        else if attribute.domain==MeshAttributeDomain::Face && self.attribute_affected.is_none() {self.attribute_corner=Some(self.mesh.faces[self.cursor].halfedge);self.phase=12;}
                        else {
                            let affected=if attribute.domain==MeshAttributeDomain::Face {self.attribute_affected.take().unwrap()}else {let vertex=if attribute.domain==MeshAttributeDomain::Corner {self.mesh.halfedges[self.cursor].vertex as usize}else {self.cursor};self.selection.as_ref().unwrap().selected.contains(&vertex)};
                            let source=attribute.indices.as_ref().map_or(self.cursor as u32,|indices|indices[self.cursor]);let sample=if let Some(sample)=self.attribute_samples.get(&(source,affected)) {*sample}else {let original=&attribute.values[source as usize];let value=if affected {Self::directional_value(original,matrix,name=="tangent",reflected)?}else {original.clone()};let sample=self.attribute_values.len() as u32;self.attribute_values.push(value);self.attribute_samples.insert((source,affected),sample);sample};self.attribute_indices.push(sample);self.cursor+=1;
                        }
                    }else if self.cursor==attribute.values.len() {self.cursor=0;self.attribute+=1;}else {let value=Self::directional_value(&attribute.values[self.cursor],matrix,name=="tangent",reflected)?;self.mesh.attributes.get_mut(name).unwrap().values[self.cursor]=value;self.cursor+=1;}
                }
            }

            8=>return Ok(Some(std::mem::replace(&mut self.mesh,HalfedgeMesh::empty()))),
            9=>{let selection=self.selection.as_mut().unwrap();if selection.advance(&self.mesh)? {if selection.selected.is_empty() {return Err(MeshKernelError::EmptySelection);}self.component_pivot=selection.pivot.map_or_else(||selection.sum.map(|value|(value/selection.selected.len() as f64) as f32),|pivot|pivot.0);self.phase=0;}}
            10=>{if self.attribute_retired.pop().is_none() && self.attribute_samples.pop_first().is_none() {self.attribute+=1;self.phase=7;}}
            11=>{if self.selection.as_mut().unwrap().selected.pop_first().is_none() {self.phase=8;}}

            12=>{let corner=self.attribute_corner.unwrap();let edge=&self.mesh.halfedges[corner as usize];let affected=self.selection.as_ref().unwrap().selected.contains(&(edge.vertex as usize));if self.attribute_affected.is_some_and(|previous|previous!=affected) {return Err(MeshKernelError::InvalidInput("face direction cannot represent a mixed component transform".into()));}self.attribute_affected=Some(affected);let next=edge.next;if next==self.mesh.faces[self.cursor].halfedge {self.attribute_corner=None;self.phase=7;}else {self.attribute_corner=Some(next);}}
            _=>unreachable!(),
        }
        Ok(None)
    }
}

pub(super) struct Knife {
    pub(super) snapshot:Snapshot,
    pub(super) face:usize,
    pub(super) a:[f64;3],
    pub(super) b:[f64;3],
    pub(super) normal:Vec3f64,
    pub(super) plane:Vec3f64,
    pub(super) origin:Vec3f64,
    pub(super) scale:f64,
    pub(super) tolerance:f64,
    pub(super) distances:Vec<f64>,
    pub(super) positive:bool,
    pub(super) negative:bool,
    pub(super) planar:bool,
    pub(super) convex:bool,
    pub(super) tessellation:Option<MeshTessellationJob>,
    pub(super) tessellation_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,
    pub(super) transfer:Option<MeshTransfer>,
    pub(super) pieces:Vec<Vec<u32>>,
    pub(super) piece:usize,
    pub(super) piece_positive:bool,
    pub(super) piece_negative:bool,
    pub(super) sign:f64,
    pub(super) split:Vec<Vec<u32>>,
    pub(super) intersections:HistoryFoldIndex<(u32,u32),u32>,
    pub(super) soup:Soup,
    pub(super) corners:Vec<u32>,
    pub(super) cursor:usize,
    pub(super) source_face:usize,
    pub(super) corner:usize,
    pub(super) emitted_corners:usize,
    pub(super) build:Option<Build>,
    pub(super) vertex_sources:Vec<SourceVertex>,
    pub(super) source_faces:Vec<u32>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) phase:u8,
}

impl Knife {
    fn tuple(p:[f32;3])->Vec3f64 {(p[0] as f64,p[1] as f64,p[2] as f64)}
    pub(super) fn phase(&self)->&'static str {["snapshot","knife-scale","knife-distances","knife-projection","knife-triangulate","knife-pieces","knife-classify","knife-clip","knife-neighbors","knife-reconstruct","knife-source-vertices","knife-source-remap","knife-attributes"][self.phase as usize]}
    fn push_corner(&mut self,id:u32) {if self.corners.last()!=Some(&id) {self.corners.push(id);}}
    fn finish_piece(&mut self)->MeshResult<()> {if self.corners.first()==self.corners.last() {self.corners.pop();}if self.corners.len()<3 {return Err(MeshKernelError::InvalidInput("knife cut produces a degenerate piece".into()));}if self.snapshot.soup.faces.len().saturating_sub(1).saturating_add(self.split.len()).saturating_add(1)>100_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}self.split.push(std::mem::take(&mut self.corners));Ok(())}
    pub(super) fn advance(&mut self,grant:RetainedCloneGrant,tessellation_progress:&mut RetainedCloneProgress)->MeshResult<Option<HalfedgeMesh>> {
        if self.tessellation_retirement.is_some(){if close_tessellation_handoff(&mut self.tessellation_retirement,grant,tessellation_progress)?{self.phase=5;}return Ok(None);}
        match self.phase {
            0=>{if self.snapshot.complete() {self.normal=Self::tuple(self.snapshot.soup.normals[self.face].0);let direction=sub3((self.b[0],self.b[1],self.b[2]),(self.a[0],self.a[1],self.a[2]));let plane=cross3(direction,self.normal);let length=length3(plane);if length==0.0 || length<=length3(direction)*1e-7 {return Err(MeshKernelError::InvalidInput("knife direction must project onto a nondegenerate face".into()));}self.plane=(plane.0/length,plane.1/length,plane.2/length);self.origin=Self::tuple(self.snapshot.soup.positions[self.snapshot.soup.faces[self.face][0] as usize]);self.cursor=0;self.phase=10;}}
            1=>{let face=&self.snapshot.soup.faces[self.face];let point=Self::tuple(self.snapshot.soup.positions[face[self.cursor] as usize]);self.scale=self.scale.max(length3(sub3(point,self.origin)));self.cursor+=1;if self.cursor==face.len() {self.tolerance=self.scale*1e-7;self.cursor=0;self.phase=2;}}
            2=>{
                if self.cursor==self.snapshot.soup.positions.len() {self.phase=3;self.cursor=0;}
                else {let d=dot3(sub3(Self::tuple(self.snapshot.soup.positions[self.cursor]),(self.a[0],self.a[1],self.a[2])),self.plane);self.distances.push(if d.abs()<=self.tolerance {0.0}else {d});self.cursor+=1;}
            }
            3=>{
                let face=&self.snapshot.soup.faces[self.face];let n=face.len();let point=|i:usize|Self::tuple(self.snapshot.soup.positions[face[i%n] as usize]);let distance=self.distances[face[self.cursor] as usize];self.positive|=distance>0.0;self.negative|=distance<0.0;self.planar&=dot3(sub3(point(self.cursor),self.origin),self.normal).abs()<=self.tolerance;self.convex&=dot3(cross3(sub3(point(self.cursor+1),point(self.cursor)),sub3(point(self.cursor+2),point(self.cursor+1))),self.normal)>=-self.scale*self.tolerance;self.cursor+=1;
                if self.cursor==n {if !self.positive || !self.negative {return Err(MeshKernelError::InvalidInput("knife line must cross the face interior".into()));}if !self.planar || !self.convex {self.tessellation=Some(MeshTessellationJob::selected(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty()),HistoryFoldSet::from([self.face as u32])));}self.cursor=0;self.phase=4;}
            }
            4=>{if let Some(job)=&mut self.tessellation {let result=job.step(grant);*tessellation_progress=job.normal_step_progress();if let MeshTessellationStep::Done(transfer)=result.map_err(MeshKernelError::Retained)?.0 {self.transfer=Some(transfer);let mut original=self.tessellation.take().unwrap();self.snapshot.source=original.take_source();self.tessellation_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original tessellation handback requires typed retirement")));}}else {self.phase=5;}}
            5=>{
                if let Some(transfer)=&self.transfer {if self.cursor==transfer.indices.len() {self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=6;}else {self.pieces.push([0,1,2].map(|i|transfer.vertex_ids[transfer.indices[self.cursor+i] as usize]).to_vec());self.cursor+=3;}}
                else {let face=&self.snapshot.soup.faces[self.face];if self.cursor==face.len() {self.pieces.push(std::mem::take(&mut self.corners));self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=6;}else {self.corners.push(face[self.cursor]);self.cursor+=1;}}
            }
            6=>{
                if self.piece==self.pieces.len() {self.phase=8;self.cursor=0;self.source_face=0;self.corner=0;}
                else {let piece=&self.pieces[self.piece];if self.cursor==piece.len() {self.cursor=0;self.sign=1.0;self.phase=7;}else {let d=self.distances[piece[self.cursor] as usize];self.piece_positive|=d>0.0;self.piece_negative|=d<0.0;self.cursor+=1;}}
            }
            7=>{
                let piece=&self.pieces[self.piece];
                if self.cursor==piece.len() {
                    self.finish_piece()?;if self.piece_positive && self.piece_negative && self.sign>0.0 {self.sign=-1.0;self.cursor=0;}else {self.piece+=1;self.cursor=0;self.piece_positive=false;self.piece_negative=false;self.phase=6;}
                }else {
                    let (a,b)=(piece[self.cursor],piece[(self.cursor+1)%piece.len()]);let (da,db)=(self.distances[a as usize],self.distances[b as usize]);
                    if !self.piece_positive || !self.piece_negative {self.push_corner(a);}else {
                        if da*self.sign>=0.0 {self.push_corner(a);}if da>0.0 && db<0.0 || da<0.0 && db>0.0 {
                            let edge=(a.min(b),a.max(b));let id=if let Some(&id)=self.intersections.get(&edge) {id}else {if self.soup.positions.len()>=100_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}let (a,b)=(edge.0 as usize,edge.1 as usize);let t=self.distances[a]/(self.distances[a]-self.distances[b]);let point=[0,1,2].map(|axis|((1.0-t)*self.soup.positions[a][axis] as f64+t*self.soup.positions[b][axis] as f64) as f32);if point.iter().any(|x|!x.is_finite()) || point==self.soup.positions[a] || point==self.soup.positions[b] {return Err(MeshKernelError::InvalidInput("knife intersection collapses at this precision".into()));}let id=self.soup.positions.len() as u32;self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Edge {a:edge.0,b:edge.1,t});self.intersections.insert(edge,id);id};self.push_corner(id);
                        }
                    }self.cursor+=1;
                }
            }
            8=>{
                if self.source_face==self.snapshot.soup.faces.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=9;}
                else if self.source_face==self.face {if self.cursor==self.split.len() {self.source_face+=1;self.cursor=0;}else {let corners=self.split[self.cursor].len();if self.emitted_corners.saturating_add(corners)>600_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}self.emitted_corners+=corners;self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.split[self.cursor]));self.cursor+=1;}}
                else {let face=&self.snapshot.soup.faces[self.source_face];let (a,b)=(face[self.corner],face[(self.corner+1)%face.len()]);let intersection=self.intersections.get(&(a.min(b),a.max(b))).copied();let n=1+usize::from(intersection.is_some());if self.emitted_corners.saturating_add(n)>600_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}self.emitted_corners+=n;self.corners.push(a);if let Some(id)=intersection {self.corners.push(id);}self.corner+=1;if self.corner==face.len() {self.source_faces.push(self.source_face as u32);self.soup.faces.push(std::mem::take(&mut self.corners));self.corner=0;self.source_face+=1;}}
            }
            9=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=11;}},
            10=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=1;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            11=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=12;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            12=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}
