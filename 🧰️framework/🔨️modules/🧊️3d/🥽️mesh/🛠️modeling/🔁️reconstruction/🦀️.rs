//! 🔁️ Retained topology reconstruction, import and source-channel provenance.

use semio_framework_3d_mesh_document::*;
#[cfg(test)] use semio_framework_3d_testing_support as test_support;
use semio_framework_3d_mesh_contract::{MeshPolygonSoup as Soup,MeshSourceVertex as SourceVertex};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[path = "🧺️snapshot/🦀️.rs"]
mod snapshot_preparation;

pub struct MeshImportJob {
    polygon:Option<semio_framework_mesh_engine::PolygonMeshSource>,
    primitive:Option<(u8,[f32;3],u32)>,
    primitive_faces:Vec<Vec<u32>>,
    midpoints:HistoryFoldIndex<(u32,u32),u32>,
    mids:Vec<u32>,
    generation:u32,
    face:usize,
    part:usize,
    positions:Vec<f32>,
    indices:Vec<u32>,
    normals:Vec<f32>,
    unique:HistoryFoldIndex<[u32;3],u32>,
    remap:Vec<u32>,
    soup:Soup,
    corners:Vec<u32>,
    corner_normals:Vec<semio_framework_value::DslValue>,
    mesh:Option<HalfedgeMesh>,
    cursor:usize,
    build:Option<MeshTopologyBuild>,
    phase:u8,
}

impl MeshImportJob {
    pub fn phase(&self)->&'static str {if self.phase>=254{return if self.phase==255{"done"}else{"failed"};} ["mesh-import-vertices","mesh-import-corners","mesh-import-reconstruct","mesh-import-normals","mesh-primitive-vertices","mesh-primitive-faces","mesh-primitive-level","mesh-primitive-midpoints","mesh-primitive-release","mesh-primitive-radius","mesh-source-capture","mesh-source-assets"][self.phase as usize] }
    fn advance_owned(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{
                if self.cursor==self.positions.len() {self.phase=1;self.cursor=0;}
                else {let point=[0,1,2].map(|axis|self.positions[self.cursor+axis]);if point.iter().any(|x|!x.is_finite()) || !self.normals.is_empty() && self.normals[self.cursor..self.cursor+3].iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::InvalidInput("indexed triangulation contains non-finite values".into()));}let key=point.map(|x|if x==0.0 {0}else {x.to_bits()});let id=if let Some(&id)=self.unique.get(&key) {id}else {if self.soup.positions.len()>=100_000 {return Err(MeshKernelError::InvalidInput("indexed triangulation exceeds vertex capacity".into()));}let id=self.soup.positions.len() as u32;self.soup.positions.push(point);self.unique.insert(key,id);id};self.remap.push(id);self.cursor+=3;}
            }
            1=>{
                if self.cursor==self.indices.len() {self.build=Some(MeshTopologyBuild::unreferenced(std::mem::take(&mut self.soup)));self.phase=2;}
                else {let index=self.indices[self.cursor] as usize;let id=*self.remap.get(index).ok_or(MeshKernelError::InvalidHandle)?;self.corners.push(id);self.cursor+=1;if self.corners.len()==3 {self.soup.faces.push(std::mem::take(&mut self.corners));}}
            }
            2=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {if self.normals.is_empty() && self.polygon.is_none() {return Ok(Some(mesh));}self.mesh=Some(mesh);self.cursor=0;self.phase=if self.polygon.is_some() {11}else {3};}}
            3=>{
                if self.cursor==self.indices.len() {let mut mesh=self.mesh.take().unwrap();mesh.attributes.insert("normal".into(),MeshAttribute { indices:None,domain:MeshAttributeDomain::Corner,semantic:MeshAttributeSemantic::Normal,interpolation:MeshAttributeInterpolation::Linear,values:std::mem::take(&mut self.corner_normals)});return Ok(Some(mesh));}
                let index=self.indices[self.cursor] as usize*3;let normal=[0,1,2].map(|axis|self.normals[index+axis]);if normal==[0.0;3] {return Err(MeshKernelError::InvalidInput("imported mesh normal cannot be zero".into()));}self.corner_normals.push(semio_framework_value::DslValue::Array(normal.into_iter().map(|x|semio_framework_value::DslValue::float(x as f64)).collect()));self.cursor+=1;
            }
            4=>{
                let (kind,[a,b,c],parameter)=self.primitive.unwrap();let count=match kind {0=>8,1=>4,2=>parameter as usize*2,3=>parameter as usize+1,_=>12};
                if self.cursor==count {self.cursor=0;self.face=0;self.phase=5;}else {
                    let point=match kind {
                        0=>[[-a/2.0,-b/2.0,-c/2.0],[a/2.0,-b/2.0,-c/2.0],[a/2.0,b/2.0,-c/2.0],[-a/2.0,b/2.0,-c/2.0],[-a/2.0,-b/2.0,c/2.0],[a/2.0,-b/2.0,c/2.0],[a/2.0,b/2.0,c/2.0],[-a/2.0,b/2.0,c/2.0]][self.cursor],
                        1=>[[-a/2.0,0.0,-b/2.0],[a/2.0,0.0,-b/2.0],[a/2.0,0.0,b/2.0],[-a/2.0,0.0,b/2.0]][self.cursor],
                        2|3=>{if kind==3 && self.cursor==parameter as usize {[0.0,b/2.0,0.0]}else {let angle=(self.cursor%parameter as usize)as f32/parameter as f32*std::f32::consts::TAU;[a*angle.cos(),if self.cursor<parameter as usize {-b/2.0}else {b/2.0},a*angle.sin()]}},
                        _=>{let t=(1.0+5.0f32.sqrt())/2.0;Vec3([[-1.0,t,0.0],[1.0,t,0.0],[-1.0,-t,0.0],[1.0,-t,0.0],[0.0,-1.0,t],[0.0,1.0,t],[0.0,-1.0,-t],[0.0,1.0,-t],[t,0.0,-1.0],[t,0.0,1.0],[-t,0.0,-1.0],[-t,0.0,1.0]][self.cursor]).normalize().0},
                    };self.soup.positions.push(point);self.cursor+=1;
                }
            },
            5=>{
                let (kind,_,parameter)=self.primitive.unwrap();let count=match kind {0=>6,1=>1,2=>parameter as usize+2,3=>parameter as usize+1,_=>20};
                if self.face==count {self.cursor=0;self.face=0;self.phase=if kind==4 {6}else {2};if kind!=4 {self.build=Some(MeshTopologyBuild::unreferenced(std::mem::take(&mut self.soup)));}}else if matches!(kind,2|3) && self.face<if kind==2 {2}else {1} {
                    self.corners.push(if self.face==0 {self.cursor as u32}else {parameter*2-1-self.cursor as u32});self.cursor+=1;if self.cursor==parameter as usize {self.soup.faces.push(std::mem::take(&mut self.corners));self.face+=1;self.cursor=0;}
                }else {let face=match kind {
                    0=>[[3,2,1,0],[5,6,7,4],[1,5,4,0],[2,6,5,1],[3,7,6,2],[0,4,7,3]][self.face].to_vec(),1=>vec![0,1,2,3],
                    2=>{let id=self.face as u32-2;let next=(id+1)%parameter;vec![id,id+parameter,next+parameter,next]},3=>{let id=self.face as u32-1;vec![id,parameter,(id+1)%parameter]},
                    _=>[[0,11,5],[0,5,1],[0,1,7],[0,7,10],[0,10,11],[1,5,9],[5,11,4],[11,10,2],[10,7,6],[7,1,8],[3,9,4],[3,4,2],[3,2,6],[3,6,8],[3,8,9],[4,9,5],[2,4,11],[6,2,10],[8,6,7],[9,8,1]][self.face].to_vec(),
                };self.soup.faces.push(face);self.face+=1;}
            },
            6=>{if self.generation==self.primitive.unwrap().2 {self.cursor=0;self.phase=9;}else {self.primitive_faces=std::mem::take(&mut self.soup.faces);self.face=0;self.cursor=0;self.part=0;self.phase=7;}},
            7=>{if self.face==self.primitive_faces.len() {self.phase=8;}else {let face=&self.primitive_faces[self.face];if self.cursor<3 {let a=face[self.cursor];let b=face[(self.cursor+1)%3];let key=(a.min(b),a.max(b));let id=if let Some(id)=self.midpoints.get(&key) {*id}else {let point=Vec3(self.soup.positions[a as usize]).lerp(Vec3(self.soup.positions[b as usize]),0.5).normalize().0;let id=self.soup.positions.len()as u32;self.soup.positions.push(point);self.midpoints.insert(key,id);id};self.mids.push(id);self.cursor+=1;}else {self.soup.faces.push(if self.part<3 {vec![face[self.part],self.mids[self.part],self.mids[(self.part+2)%3]]}else {std::mem::take(&mut self.mids)});self.part+=1;if self.part==4 {self.face+=1;self.part=0;self.cursor=0;}}}},
            8=>{if self.midpoints.pop_first().is_none() && self.primitive_faces.pop().is_none() {self.generation+=1;self.phase=6;}},
            9=>{if self.cursor==self.soup.positions.len() {self.build=Some(MeshTopologyBuild::unreferenced(std::mem::take(&mut self.soup)));self.phase=2;}else {let radius=self.primitive.unwrap().1[0];self.soup.positions[self.cursor]=self.soup.positions[self.cursor].map(|value|value*radius);self.cursor+=1;}},
            10=>{let source=self.polygon.as_mut().unwrap();self.soup.positions=std::mem::take(&mut source.vertices);self.soup.faces=std::mem::take(&mut source.faces);self.build=Some(MeshTopologyBuild::unreferenced(std::mem::take(&mut self.soup)));self.phase=2;},
            11=>{let mut mesh=self.mesh.take().unwrap();let source=self.polygon.as_mut().unwrap();mesh.attributes=std::mem::take(&mut source.attributes);mesh.materials=std::mem::take(&mut source.materials);mesh.textures=std::mem::take(&mut source.textures);return Ok(Some(mesh));},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Coplanar {
    explicit:Option<Vec<EdgeId>>,
    vertex_input:Option<Vec<VertexId>>,
    selected_vertices:HistoryFoldSet<u32>,
    star_vertex:u32,
    star_incident:Vec<usize>,
    star_edges:HistoryFoldIndex<(u32,u32),(usize,usize)>,
    star_next:HistoryFoldIndex<u32,(u32,usize,usize)>,
    star_start:u32,
    star_probe:u32,
    star_contributors:Vec<u32>,
    star_extent:f32,
    star_origin:Vec3,

    output_vertices:Vec<u32>,
    source_corners:HistoryFoldIndex<(u32,u32),u32>,
    face_sources:Vec<Vec<u32>>,
    corner_sources:Vec<Vec<u32>>,
    edge_sources:Vec<Vec<u32>>,
    spliced_corners:Vec<u32>,
    spliced_edges:Vec<u32>,
    output_corners:Vec<Option<u32>>,
    output_edges:Vec<Option<u32>>,
    output_faces:Vec<Vec<u32>>,
    attributes:Option<MeshSourceRemap>,
    attribute_name:Option<String>,
    equality:Option<SourceValueEquality>,
    preserve_corners:bool,
    endpoint:usize,
    keep:usize,
    remove:usize,
    incidence:HistoryFoldIndex<(u32,u32),(usize,usize)>,
    pending:HistoryFoldSet<(u32,u32)>,
    seen:HistoryFoldSet<u32>,
    neighbors:HistoryFoldIndex<u32,Option<(u32,u32)>>,
    removable:HistoryFoldSet<u32>,
    face:usize,
    cursor:usize,
    a:usize,
    b:usize,
    ai:usize,
    bi:usize,
    side:usize,
    normal:MeshPolygonNormal,
    na:Vec3,
    min:Vec3,
    max:Vec3,
    tolerance:f32,
    shared:usize,
    corners:Vec<u32>,
    soup:Soup,
    build:Option<MeshTopologyBuild>,
    merges:usize,
    phase:u8,
}

impl Default for Coplanar {
    fn default()->Self {Self {explicit:None,vertex_input:None,selected_vertices:HistoryFoldSet::new(),star_vertex:0,star_incident:Vec::new(),star_edges:HistoryFoldIndex::new(),star_next:HistoryFoldIndex::new(),star_start:0,star_probe:0,star_contributors:Vec::new(),star_extent:0.0,star_origin:Vec3::ZERO,output_vertices:Vec::new(),source_corners:HistoryFoldIndex::new(),face_sources:Vec::new(),corner_sources:Vec::new(),edge_sources:Vec::new(),spliced_corners:Vec::new(),spliced_edges:Vec::new(),output_corners:Vec::new(),output_edges:Vec::new(),output_faces:Vec::new(),attributes:None,attribute_name:None,equality:None,preserve_corners:false,endpoint:0,keep:0,remove:0,incidence:HistoryFoldIndex::new(),pending:HistoryFoldSet::new(),seen:HistoryFoldSet::new(),neighbors:HistoryFoldIndex::new(),removable:HistoryFoldSet::new(),face:0,cursor:0,a:0,b:0,ai:0,bi:0,side:0,normal:MeshPolygonNormal::default(),na:Vec3::ZERO,min:Vec3::ZERO,max:Vec3::ZERO,tolerance:0.0,shared:0,corners:Vec::new(),soup:Soup::default(),build:None,merges:0,phase:0}}
}

impl Coplanar {
    fn phase(&self)->&'static str {match self.phase {0=>"snapshot",1=>"coplanar-incidence",2=>"coplanar-candidates",3|4=>"coplanar-normal",5=>"coplanar-distance",6=>"coplanar-shared-edges",7=>"coplanar-splice",8|9=>"coplanar-remove-edges",10=>"coplanar-insert-edges",11=>"coplanar-release-candidate",12=>"coplanar-cleanup-incidence",13=>"coplanar-cleanup-classify",14=>"coplanar-cleanup-corners",17|18=>"coplanar-source-corners",22=>"coplanar-channel-policy",23=>"dissolve-selection",24=>"dissolve-vertex-provenance",25=>"dissolve-face-metadata",26=>"dissolve-star-selection",27=>"dissolve-star-incidence",28=>"dissolve-star-normal",29=>"dissolve-star-plane",30=>"dissolve-star-boundary",31=>"dissolve-star-next",32=>"dissolve-star-loop",33=>"dissolve-star-release",34=>"dissolve-star-contributors",19=>"coplanar-face-contributors",20=>"coplanar-channel-boundary",21=>"coplanar-attributes",_=>"coplanar-reconstruct"}}
    fn reject(&mut self)->MeshResult<()> {if self.explicit.is_some() || self.vertex_input.is_some() {return Err(MeshKernelError::InvalidInput("selected dissolution cannot preserve the boundary contract".into()));}self.cursor=0;self.phase=11;Ok(())}
    fn advance(&mut self,snapshot:&mut MeshSourceSnapshot)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if snapshot.complete() {self.cursor=0;self.phase=22;}},
            1=>{
                if self.face==snapshot.soup.faces.len() {self.face=0;self.cursor=0;self.phase=2;}
                else {let face=&snapshot.soup.faces[self.face];let key=(face[self.cursor],face[(self.cursor+1)%face.len()]);if self.incidence.insert(key,(self.face,self.cursor)).is_some() {return Err(MeshKernelError::NonManifold);}if self.explicit.is_none() {self.pending.insert(key);}self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}
            },
            2=>{
                if let Some((u,v))=self.pending.pop_first() {
                    if let (Some(&(a,ai)),Some(&(b,bi)))=(self.incidence.get(&(u,v)),self.incidence.get(&(v,u))) {if a!=b {self.a=a;self.b=b;self.ai=ai;self.bi=bi;self.cursor=0;self.side=0;self.shared=0;self.normal=MeshPolygonNormal::default();let point=Vec3(snapshot.soup.positions[snapshot.soup.faces[a][0] as usize]);self.min=point;self.max=point;self.phase=if self.explicit.is_some() {6}else {3};}}else if self.explicit.is_some() {return Err(MeshKernelError::DegenerateOperation);}
                }else {self.face=0;self.cursor=0;self.phase=12;}
            },
            3|4=>{
                let face=&snapshot.soup.faces[if self.phase==3 {self.a}else {self.b}];let p=Vec3(snapshot.soup.positions[face[self.cursor] as usize]);let next=Vec3(snapshot.soup.positions[face[(self.cursor+1)%face.len()] as usize]);if self.cursor==0 {self.normal.origin=p;}self.normal.add(p,next);for axis in 0..3 {self.min.0[axis]=self.min.0[axis].min(p.0[axis]);self.max.0[axis]=self.max.0[axis].max(p.0[axis]);}self.cursor+=1;
                if self.cursor==face.len() {let normal=self.normal.value();self.normal=MeshPolygonNormal::default();self.cursor=0;if normal.length()==0.0 {self.reject()?;}else if self.phase==3 {self.na=normal;self.phase=4;}else if self.na.dot(normal)<1.0-1e-4 {self.reject()?;}else {self.tolerance=(self.max.sub(self.min).length()*1e-4).max(1e-6);self.phase=5;}}
            },
            5=>{
                let face=&snapshot.soup.faces[self.b];let origin=Vec3(snapshot.soup.positions[snapshot.soup.faces[self.a][0] as usize]);let point=Vec3(snapshot.soup.positions[face[self.cursor] as usize]);if point.sub(origin).dot(self.na).abs()>self.tolerance {self.reject()?;}else {self.cursor+=1;if self.cursor==face.len() {self.cursor=0;self.phase=6;}}
            },
            6=>{
                let face=&snapshot.soup.faces[self.a];let (u,v)=(face[self.cursor],face[(self.cursor+1)%face.len()]);if self.incidence.get(&(v,u)).is_some_and(|&(other,_)|other==self.b) {self.shared+=1;}self.cursor+=1;if self.shared>1 {self.reject()?;}else if self.cursor==face.len() {self.cursor=0;self.attribute_name=None;self.endpoint=0;self.phase=20;}
            },
            7=>{
                let a=&snapshot.soup.faces[self.a];let b=&snapshot.soup.faces[self.b];let total=a.len()+b.len()-2;
                if self.cursor==total {self.cursor=0;self.side=0;self.phase=8;}
                else {let (face,index)=if self.cursor==0 {(self.a,self.ai)}else if self.cursor<b.len()-1 {(self.b,(self.bi+self.cursor+1)%b.len())}else {(self.a,(self.ai+self.cursor-(b.len()-2))%a.len())};let vertex=snapshot.soup.faces[face][index];if !self.seen.insert(vertex) {self.reject()?;}else {self.corners.push(vertex);self.spliced_corners.push(self.corner_sources[face][index]);self.spliced_edges.push(if self.cursor==0 {self.edge_sources[self.b][(self.bi+1)%b.len()]}else {self.edge_sources[face][index]});self.cursor+=1;}}
            },
            8=>{
                let face=&snapshot.soup.faces[if self.side==0 {self.a}else {self.b}];let key=(face[self.cursor],face[(self.cursor+1)%face.len()]);self.incidence.remove(&key);self.pending.remove(&key);self.cursor+=1;if self.cursor==face.len() {self.cursor=0;self.side+=1;if self.side==2 {self.phase=9;}}
            },
            9=>{self.keep=self.a.min(self.b);self.remove=self.a.max(self.b);snapshot.soup.faces[self.keep]=std::mem::take(&mut self.corners);snapshot.soup.faces[self.remove].clear();self.corner_sources[self.keep]=std::mem::take(&mut self.spliced_corners);self.edge_sources[self.keep]=std::mem::take(&mut self.spliced_edges);self.corner_sources[self.remove].clear();self.edge_sources[self.remove].clear();self.a=self.keep;self.cursor=0;self.merges+=1;self.phase=19;},
            10=>{let face=&snapshot.soup.faces[self.a];let key=(face[self.cursor],face[(self.cursor+1)%face.len()]);if self.incidence.insert(key,(self.a,self.cursor)).is_some() {return Err(MeshKernelError::NonManifold);}if self.explicit.is_none() {self.pending.insert(key);}self.cursor+=1;if self.cursor==face.len() {self.cursor=0;self.phase=11;}},
            11=>{if self.seen.pop_first().is_none() {self.corners.clear();self.spliced_corners.clear();self.spliced_edges.clear();self.attribute_name=None;self.equality=None;self.phase=2;}},
            12=>{
                if self.face==snapshot.soup.faces.len() {self.face=0;self.cursor=0;self.phase=13;}
                else {let face=&snapshot.soup.faces[self.face];if face.is_empty() {self.face+=1;}else {let id=face[self.cursor];let pair=(face[(self.cursor+face.len()-1)%face.len()],face[(self.cursor+1)%face.len()]);self.neighbors.entry(id).and_modify(|value|{if *value!=Some(pair) {*value=None;}}).or_insert_with(||Some(pair));self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}}
            },
            13=>{
                if let Some((id,Some((prev,next))))=self.neighbors.pop_first() {if !self.preserve_corners && prev!=next && prev!=id && next!=id {let pos=|id:u32|Vec3(snapshot.soup.positions[id as usize]);let a=pos(id).sub(pos(prev));let b=pos(next).sub(pos(id));if a.length()>=1e-9 && b.length()>=1e-9 && a.normalize().dot(b.normalize())>1.0-1e-4 {self.removable.insert(id);}}}
                else if self.neighbors.is_empty() {self.soup.positions=std::mem::take(&mut snapshot.soup.positions);self.phase=14;}
            },
            14=>{
                if self.face==snapshot.soup.faces.len() {self.build=Some(if self.explicit.is_some() || self.vertex_input.is_some() {MeshTopologyBuild::new(std::mem::take(&mut self.soup),false)}else {MeshTopologyBuild::unreferenced(std::mem::take(&mut self.soup))});self.phase=15;}
                else {let face=&snapshot.soup.faces[self.face];if face.is_empty() {self.face+=1;}else {let id=face[self.cursor];if !self.removable.contains(&id) {self.corners.push(id);self.output_corners.push(Some(self.corner_sources[self.face][self.cursor]));self.output_edges.push(Some(self.edge_sources[self.face][self.cursor]));}self.cursor+=1;if self.cursor==face.len() {if self.corners.len()>=3 {self.soup.faces.push(std::mem::take(&mut self.corners));self.output_faces.push(std::mem::take(&mut self.face_sources[self.face]));}else {self.corners.clear();}self.face+=1;self.cursor=0;}}}
            },
            15=>{if self.incidence.pop_first().is_none() && self.removable.pop_first().is_none() && self.source_corners.pop_first().is_none() {self.phase=16;}},
            16=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let mut attributes=MeshSourceRemap::new(mesh,None,None,None,None);attributes.data.corner_sources=Some(std::mem::take(&mut self.output_corners));attributes.data.edge_sources=Some(std::mem::take(&mut self.output_edges));attributes.data.generated_faces=Some(std::mem::take(&mut self.output_faces));self.attributes=Some(attributes);self.cursor=0;self.phase=if self.explicit.is_some() || self.vertex_input.is_some() {24}else {25};}},
            17=>{if self.cursor==snapshot.source.halfedge_count() {self.cursor=0;self.face=0;self.phase=18;}else {let edge=&snapshot.source.halfedges[self.cursor];self.source_corners.insert((edge.face.unwrap(),edge.vertex),self.cursor as u32);self.cursor+=1;}},
            18=>{if self.face==snapshot.soup.faces.len() {self.face=0;self.cursor=0;self.phase=if self.vertex_input.is_some() {26}else {1};}else {let face=&snapshot.soup.faces[self.face];if self.cursor==0 {self.face_sources.push(vec![self.face as u32]);self.corner_sources.push(Vec::new());self.edge_sources.push(Vec::new());}let corner=self.source_corners[&(self.face as u32,face[self.cursor])];let edge=if snapshot.source.faces[self.face].flipped {self.source_corners[&(self.face as u32,face[(self.cursor+1)%face.len()])]}else {corner};self.corner_sources[self.face].push(corner);self.edge_sources[self.face].push(edge);self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}},
            19=>{if let Some(face)=self.face_sources[self.remove].pop() {self.face_sources[self.keep].push(face);}else {self.phase=10;}},
            20=>{
                let a=self.edge_sources[self.a][self.ai];let b=self.edge_sources[self.b][self.bi];
                if snapshot.source.uv_seams.contains(&a) || snapshot.source.uv_seams.contains(&b) {self.reject()?;return Ok(None);}
                if self.attribute_name.is_none() {self.attribute_name=snapshot.source.attributes.first_key_value().map(|(name,_)|name.clone());}
                let Some(name)=self.attribute_name.as_ref() else {self.phase=if self.vertex_input.is_some() {30}else {7};return Ok(None);};let attribute=&snapshot.source.attributes[name];
                if attribute.domain==MeshAttributeDomain::Corner {
                    let a=self.corner_sources[self.a][(self.ai+self.endpoint)%snapshot.soup.faces[self.a].len()];let b=self.corner_sources[self.b][(self.bi+1-self.endpoint)%snapshot.soup.faces[self.b].len()];let a=attribute.indices.as_ref().map_or(a,|indices|indices[a as usize]);let b=attribute.indices.as_ref().map_or(b,|indices|indices[b as usize]);
                    if self.equality.is_none() {self.equality=Some(SourceValueEquality::new(a,b));}
                    if let Some(equal)=self.equality.as_mut().unwrap().step(&attribute.values)? {self.equality=None;if !equal {self.reject()?;return Ok(None);}self.endpoint+=1;}else {return Ok(None);}
                    if self.endpoint<2 {return Ok(None);}
                }
                let next=snapshot.source.attributes.range((std::ops::Bound::Excluded(name.clone()),std::ops::Bound::Unbounded)).next().map(|(name,_)|name.clone());self.attribute_name=next;self.endpoint=0;if self.attribute_name.is_none() {self.phase=if self.vertex_input.is_some() {30}else {7};}
            },
            21=>return self.attributes.as_mut().unwrap().advance(&mut snapshot.source),
            22=>{self.preserve_corners|=!snapshot.source.uv_seams.is_empty();if self.attribute_name.is_none() {self.attribute_name=snapshot.source.attributes.first_key_value().map(|(name,_)|name.clone());}if let Some(name)=self.attribute_name.as_ref() {self.preserve_corners|=matches!(snapshot.source.attributes[name].domain,MeshAttributeDomain::Corner|MeshAttributeDomain::Edge);self.attribute_name=snapshot.source.attributes.range((std::ops::Bound::Excluded(name.clone()),std::ops::Bound::Unbounded)).next().map(|(name,_)|name.clone());if self.attribute_name.is_none() {self.phase=17;}}else {self.phase=17;}},
            23=>{if let Some(vertices)=&self.vertex_input {if self.cursor==vertices.len() {self.cursor=0;self.phase=0;}else {let id=vertices[self.cursor];snapshot.source.vertex_position(id)?;self.selected_vertices.insert(id.0);self.cursor+=1;}}else {let edges=self.explicit.as_ref().unwrap();if self.cursor==edges.len() {self.cursor=0;self.phase=0;}else {let (a,b)=snapshot.source.edge_endpoints(edges[self.cursor])?;self.pending.insert((a.0.min(b.0),a.0.max(b.0)));self.cursor+=1;}}},
            24=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.attributes.as_mut().unwrap().data.vertices=Some(std::mem::take(&mut self.output_vertices));self.cursor=0;self.phase=25;}else {self.output_vertices.push(origins[self.cursor]);self.cursor+=1;}},
            25=>{let attributes=self.attributes.as_mut().unwrap();if self.cursor==attributes.data.mesh.face_count() {self.phase=21;}else {let face=attributes.data.generated_faces.as_ref().unwrap()[self.cursor][0];attributes.data.mesh.faces[self.cursor].smooth=snapshot.source.faces[face as usize].smooth;self.cursor+=1;}},
            26=>{if let Some(vertex)=self.selected_vertices.pop_first() {self.star_vertex=vertex;self.star_incident.clear();self.face=0;self.cursor=0;self.phase=27;}else {self.face=0;self.cursor=0;self.phase=12;}},
            27=>{if self.face==snapshot.soup.faces.len() {let face=*self.star_incident.first().ok_or(MeshKernelError::DegenerateOperation)?;self.star_origin=Vec3(snapshot.soup.positions[snapshot.soup.faces[face][0]as usize]);self.star_extent=0.0;self.normal=MeshPolygonNormal::default();self.normal.origin=self.star_origin;self.cursor=0;self.phase=28;}else {let face=&snapshot.soup.faces[self.face];if face.is_empty() {self.face+=1;self.cursor=0;}else if face[self.cursor]==self.star_vertex {self.star_incident.push(self.face);self.face+=1;self.cursor=0;}else {self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}}},
            28=>{let face=&snapshot.soup.faces[self.star_incident[0]];let a=Vec3(snapshot.soup.positions[face[self.cursor]as usize]);let b=Vec3(snapshot.soup.positions[face[(self.cursor+1)%face.len()]as usize]);self.normal.add(a,b);self.star_extent=self.star_extent.max(a.sub(self.star_origin).length());self.cursor+=1;if self.cursor==face.len() {self.na=self.normal.value();if self.na.length()==0.0 {return Err(MeshKernelError::DegenerateOperation);}self.face=0;self.cursor=0;self.phase=29;}},
            29=>{if self.face==self.star_incident.len() {self.face=0;self.cursor=0;self.phase=30;}else {let face=&snapshot.soup.faces[self.star_incident[self.face]];let point=Vec3(snapshot.soup.positions[face[self.cursor]as usize]);if point.sub(self.star_origin).dot(self.na).abs()>self.star_extent*1e-5 {return Err(MeshKernelError::InvalidInput("vertex dissolution requires a planar star".into()));}self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}},
            30=>{if self.face==self.star_incident.len() {self.phase=31;}else {let id=self.star_incident[self.face];let face=&snapshot.soup.faces[id];let index=self.cursor;let (a,b)=(face[index],face[(index+1)%face.len()]);self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}if let Some((other,corner))=self.star_edges.remove(&(b,a)) {self.a=other;self.ai=corner;self.b=id;self.bi=index;self.attribute_name=None;self.endpoint=0;self.phase=20;}else if self.star_edges.insert((a,b),(id,index)).is_some() {return Err(MeshKernelError::NonManifold);}}},
            31=>{if let Some(((a,b),(face,corner)))=self.star_edges.pop_first() {if self.star_next.insert(a,(b,face,corner)).is_some() {return Err(MeshKernelError::NonManifold);}}else {self.star_start=*self.star_next.first_key_value().ok_or(MeshKernelError::DegenerateOperation)?.0;self.star_probe=self.star_start;self.phase=32;}},
            32=>{let (next,face,corner)=self.star_next.remove(&self.star_probe).ok_or(MeshKernelError::NonManifold)?;if self.star_probe!=self.star_vertex {self.corners.push(self.star_probe);self.spliced_corners.push(self.corner_sources[face][corner]);self.spliced_edges.push(self.edge_sources[face][corner]);}else if snapshot.source.attributes.values().any(|attribute|attribute.domain==MeshAttributeDomain::Edge) || snapshot.source.uv_seams.contains(&self.edge_sources[face][corner]) {return Err(MeshKernelError::InvalidInput("boundary vertex dissolution cannot preserve an authored removed edge".into()));}self.star_probe=next;if next==self.star_start {if !self.star_next.is_empty() || self.corners.len()<3 {return Err(MeshKernelError::NonManifold);}self.keep=self.star_incident[0];self.face=0;self.cursor=0;self.phase=34;}},
            34=>{if self.face==self.star_incident.len() {self.face=0;self.phase=33;}else {let id=self.star_incident[self.face];if let Some(face)=self.face_sources[id].pop() {self.star_contributors.push(face);}else {self.face+=1;}}},
            33=>{if self.face==self.star_incident.len() {snapshot.soup.faces[self.keep]=std::mem::take(&mut self.corners);self.corner_sources[self.keep]=std::mem::take(&mut self.spliced_corners);self.edge_sources[self.keep]=std::mem::take(&mut self.spliced_edges);self.face_sources[self.keep]=std::mem::take(&mut self.star_contributors);self.phase=26;}else {let id=self.star_incident[self.face];if snapshot.soup.faces[id].pop().is_some() {self.corner_sources[id].pop();self.edge_sources[id].pop();}else {self.face+=1;}}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

pub struct MeshMergeJob {
    snapshot:MeshSourceSnapshot,coplanar:Option<Coplanar>,input:Vec<VertexId>,selected:Vec<u32>,seen:HistoryFoldSet<u32>,mode:WeldMode,threshold:f32,
    roots:Vec<usize>,remap:Vec<u32>,soup:Soup,corners:Vec<u32>,unique:HistoryFoldIndex<u32,usize>,center:[f64;3],
    cursor:usize,i:usize,j:usize,a:usize,b:usize,face:usize,corner:usize,build:Option<MeshTopologyBuild>,phase:u8,
    quantized:bool,grid:HistoryFoldIndex<(i64,i64,i64),u32>,contributors:Vec<Vec<u32>>,source_corners:HistoryFoldIndex<(u32,u32),u32>,
    corner_origins:Vec<Option<u32>>,edge_origins:Vec<Option<u32>>,output_corners:Vec<Option<u32>>,output_edges:Vec<Option<u32>>,output_faces:Vec<u32>,attributes:Option<MeshSourceRemap>,removed:usize,
}

impl MeshMergeJob {
    fn configured(source:HalfedgeMesh,coplanar:Option<Coplanar>,input:Vec<VertexId>,mode:WeldMode,threshold:f32,quantized:bool)->Self {
        Self {snapshot:MeshSourceSnapshot::new(source),coplanar,input,selected:Vec::new(),seen:HistoryFoldSet::new(),mode,threshold,roots:Vec::new(),remap:Vec::new(),soup:Soup::default(),corners:Vec::new(),unique:HistoryFoldIndex::new(),center:[0.0;3],cursor:0,i:1,j:0,a:0,b:0,face:0,corner:0,build:None,phase:if quantized {1}else {0},quantized,grid:HistoryFoldIndex::new(),contributors:Vec::new(),source_corners:HistoryFoldIndex::new(),corner_origins:Vec::new(),edge_origins:Vec::new(),output_corners:Vec::new(),output_edges:Vec::new(),output_faces:Vec::new(),attributes:None,removed:0}
    }
    pub fn phase(&self)->&'static str {if self.phase>=254{return if self.phase==255{"done"}else{"failed"};}if let Some(work)=&self.coplanar {return work.phase();}match self.phase {0=>"merge-selection",1=>"snapshot",2..=4=>"merge-pairs",5..=7=>"merge-remap",8..=10=>"merge-corners",11=>"merge-reconstruct",12=>"merge-source-corners",13=>"merge-contributors",14=>"weld-grid",15=>"merge-vertex-provenance",16=>"merge-face-metadata",_=>"merge-attributes"}}
    fn next_pair(&mut self) {self.j+=1;if self.j==self.i {self.i+=1;self.j=0;}self.phase=2;}
    fn advance_owned(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        if let Some(work)=&mut self.coplanar {return work.advance(&mut self.snapshot);}
        match self.phase {
            0=>{if self.cursor==self.input.len() {if self.selected.len()<2 {return Err(MeshKernelError::EmptySelection);}self.phase=1;self.cursor=0;}else {let id=self.input[self.cursor];self.snapshot.source.vertex_position(id)?;if self.seen.insert(id.0) {self.roots.push(if self.mode==WeldMode::ByDistance {self.selected.len()}else {0});self.selected.push(id.0);}self.cursor+=1;}},
            1=>{if self.snapshot.complete() {self.phase=if self.quantized {14}else {2};}},
            2=>{if self.mode!=WeldMode::ByDistance || self.i==self.selected.len() {self.phase=5;}else if Vec3(self.snapshot.soup.positions[self.selected[self.i]as usize]).sub(Vec3(self.snapshot.soup.positions[self.selected[self.j]as usize])).length()<=self.threshold {self.a=self.i;self.b=self.j;self.phase=3;}else {self.next_pair();}},
            3=>{if self.roots[self.a]==self.a {self.phase=4;}else {self.a=self.roots[self.a];}},
            4=>{if self.roots[self.b]==self.b {self.roots[self.a.max(self.b)]=self.a.min(self.b);self.next_pair();}else {self.b=self.roots[self.b];}},
            5=>{if self.cursor==self.snapshot.soup.positions.len() {self.phase=6;self.cursor=0;}else {self.remap.push(self.cursor as u32);self.contributors.push(Vec::new());self.cursor+=1;}},
            6=>{if self.cursor==self.selected.len() {if self.mode==WeldMode::Center {self.snapshot.soup.positions[self.selected[0]as usize]=self.center.map(|value|value as f32);}self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=13;}else {self.a=self.cursor;self.phase=7;}},
            7=>{if self.roots[self.a]!=self.a {self.a=self.roots[self.a];}else {let id=self.selected[self.cursor];self.remap[id as usize]=self.selected[self.a];if self.mode==WeldMode::Center {for axis in 0..3 {self.center[axis]+=self.snapshot.soup.positions[id as usize][axis]as f64/self.selected.len()as f64;}}self.cursor+=1;self.phase=6;}},
            8=>{
                if self.face==self.snapshot.soup.faces.len() {let soup=std::mem::take(&mut self.soup);self.build=Some(if self.quantized {MeshTopologyBuild::unreferenced(soup)}else {MeshTopologyBuild::new(soup,false)});self.phase=11;}else {
                    let face=&self.snapshot.soup.faces[self.face];let original=face[self.corner];let id=self.remap[original as usize];let source_corner=self.source_corners[&(self.face as u32,original)];let end=face[(self.corner+1)%face.len()];let source_edge=if self.snapshot.source.faces[self.face].flipped {self.source_corners[&(self.face as u32,end)]}else {source_corner};
                    if self.corners.last()!=Some(&id) {self.corners.push(id);self.corner_origins.push(Some(source_corner));self.edge_origins.push(Some(source_edge));}else {*self.edge_origins.last_mut().unwrap()=Some(source_edge);}
                    self.corner+=1;if self.corner==face.len() {if self.corners.first()==self.corners.last() {self.corners.pop();self.corner_origins.pop();self.edge_origins.pop();}self.phase=9;self.cursor=0;}
                }
            },
            9=>{if self.cursor==self.corners.len() {self.phase=10;self.cursor=0;}else {if self.unique.insert(self.corners[self.cursor],self.face)==Some(self.face) {return Err(MeshKernelError::NonManifold);}self.cursor+=1;}},
            10=>{
                if self.corners.len()>=3 && self.cursor<self.corners.len() {self.output_corners.push(self.corner_origins[self.cursor]);self.output_edges.push(self.edge_origins[self.cursor]);self.cursor+=1;}else {if self.corners.len()>=3 {self.soup.faces.push(std::mem::take(&mut self.corners));self.output_faces.push(self.face as u32);}else {self.corners.clear();}self.corner_origins.clear();self.edge_origins.clear();self.face+=1;self.corner=0;self.phase=8;}
            },
            11=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.removed=self.snapshot.source.vertex_count()-mesh.vertex_count();let mut attributes=MeshSourceRemap::new(mesh,None,Some(std::mem::take(&mut self.output_faces)),None,None);attributes.data.corner_sources=Some(std::mem::take(&mut self.output_corners));attributes.data.edge_sources=Some(std::mem::take(&mut self.output_edges));attributes.data.generated_vertices=Some(Vec::new());attributes.data.vertex_contributors=Some(std::mem::take(&mut self.contributors));self.attributes=Some(attributes);self.cursor=0;self.phase=15;}},
            12=>{if self.cursor==self.snapshot.source.halfedge_count() {self.phase=8;self.cursor=0;}else {let edge=&self.snapshot.source.halfedges[self.cursor];self.source_corners.insert((edge.face.unwrap(),edge.vertex),self.cursor as u32);self.cursor+=1;}},
            13=>{if self.cursor==self.remap.len() {self.cursor=0;self.phase=12;}else {self.contributors[self.remap[self.cursor]as usize].push(self.cursor as u32);self.cursor+=1;}},
            14=>{
                if self.cursor==self.snapshot.soup.positions.len() {self.removed=self.snapshot.source.vertex_count()-self.soup.positions.len();if self.removed==0 {return Ok(Some(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty())));}self.cursor=0;self.phase=12;}else {
                    let point=self.snapshot.soup.positions[self.cursor];let key=mesh_weld_key(point,self.threshold)?;
                    let id=if let Some(id)=self.grid.get(&key) {*id}else {let id=self.soup.positions.len()as u32;self.grid.insert(key,id);self.soup.positions.push(point);self.contributors.push(Vec::new());id};self.remap.push(id);self.contributors[id as usize].push(self.cursor as u32);self.cursor+=1;
                }
            },
            15=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.cursor=0;self.phase=16;}else {let origin=origins[self.cursor];let attributes=self.attributes.as_mut().unwrap();let group=&attributes.data.vertex_contributors.as_ref().unwrap()[origin as usize];attributes.data.generated_vertices.as_mut().unwrap().push(if group.len()==1 {SourceVertex::Original(group[0])}else {SourceVertex::Contributors(origin)});self.cursor+=1;}},
            16=>{let attributes=self.attributes.as_mut().unwrap();if self.cursor==attributes.data.mesh.face_count() {self.phase=17;}else {let original=attributes.data.faces.as_ref().unwrap()[self.cursor];attributes.data.mesh.faces[self.cursor].smooth=self.snapshot.source.faces[original as usize].smooth;self.cursor+=1;}},
            17=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

enum SourceEqualityTask {Node(Vec<usize>),Sequence(Vec<usize>,usize,bool),Text(Vec<usize>,usize,Option<usize>)}

struct SourceValueEquality {a:u32,b:u32,tasks:Vec<SourceEqualityTask>}

impl SourceValueEquality {
    fn new(a:u32,b:u32)->Self {Self {a,b,tasks:vec![SourceEqualityTask::Node(Vec::new())]}}
    fn value<'a>(mut value:&'a semio_framework_value::DslValue,path:&[usize])->Option<&'a semio_framework_value::DslValue> {for index in path {value=match value {semio_framework_value::DslValue::Array(values)=>values.get(*index)?,semio_framework_value::DslValue::Object(values)=>&values.get(*index)?.1,_=>return None};}Some(value)}
    fn step(&mut self,values:&[semio_framework_value::DslValue])->MeshResult<Option<bool>> {
        use semio_framework_value::DslValue;
        let Some(task)=self.tasks.pop() else {return Ok(Some(true));};
        let path=match &task {SourceEqualityTask::Node(path)|SourceEqualityTask::Sequence(path,_,_)|SourceEqualityTask::Text(path,_,_)=>path};
        if path.len()>64 || self.tasks.len()>256 {return Err(MeshKernelError::InvalidInput("mesh constant comparison exceeds nesting limit".into()));}
        let (a,b)=(Self::value(&values[self.a as usize],path).unwrap(),Self::value(&values[self.b as usize],path).unwrap());
        match task {
            SourceEqualityTask::Node(path)=>match (a,b) {
                (DslValue::String(a),DslValue::String(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Text(path,0,None));},
                (DslValue::Array(a),DslValue::Array(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Sequence(path,0,false));},
                (DslValue::Object(a),DslValue::Object(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Sequence(path,0,true));},
                (DslValue::Bytes(a),DslValue::Bytes(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Sequence(path,0,false));},
                (DslValue::Null,DslValue::Null)=>{},
                (DslValue::Bool(a),DslValue::Bool(b))=>if a!=b {return Ok(Some(false));},
                (DslValue::Number(a),DslValue::Number(b))=>if a!=b {return Ok(Some(false));},
                _=>return Ok(Some(false)),
            },
            SourceEqualityTask::Sequence(path,index,object)=>{
                let length=match a {DslValue::Array(values)=>values.len(),DslValue::Object(values)=>values.len(),DslValue::Bytes(values)=>values.len(),_=>unreachable!()};
                if index<length {
                    self.tasks.push(SourceEqualityTask::Sequence(path.clone(),index+1,object));
                    if let (DslValue::Bytes(a),DslValue::Bytes(b))=(a,b) {if a[index]!=b[index] {return Ok(Some(false));}}
                    else {let mut child=path.clone();child.push(index);self.tasks.push(SourceEqualityTask::Node(child));if object {self.tasks.push(SourceEqualityTask::Text(path,0,Some(index)));}}
                }
            },
            SourceEqualityTask::Text(path,offset,key)=>{
                let (a,b)=match key {Some(index)=>(a.as_object().unwrap()[index].0.as_str(),b.as_object().unwrap()[index].0.as_str()),None=>(a.as_str().unwrap(),b.as_str().unwrap())};if a.len()!=b.len() {return Ok(Some(false));}
                if offset<a.len() {let (a,b)=(a[offset..].chars().next().unwrap(),b[offset..].chars().next().unwrap());if a!=b {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Text(path,offset+a.len_utf8(),key));}
            },
        }
        Ok(if self.tasks.is_empty() {Some(true)}else {None})
    }
}

pub struct MeshSourceRemapData {
    pub vertex_contributors:Option<Vec<Vec<u32>>>,
    pub corner_faces:Option<Vec<u32>>,
    pub generated_vertices:Option<Vec<SourceVertex>>,
    pub generated_faces:Option<Vec<Vec<u32>>>,
    pub corner_sources:Option<Vec<Option<u32>>>,
    pub edge_sources:Option<Vec<Option<u32>>>,
    pub edge_source_keys:Option<Vec<(u32,u32)>>,
    pub mesh:HalfedgeMesh,
    pub vertices:Option<Vec<u32>>,
    pub faces:Option<Vec<u32>>,
    pub mirrored_faces:Option<Vec<bool>>,
    pub normal_axis:Option<usize>,
}

pub struct MeshSourceRemap {
    data:MeshSourceRemapData,
    face_values_base:usize,
    influence:Option<SourceVertex>,
    influence_face:u32,
    influence_vertex:u32,
    weight_cursor:usize,
    weight_next:u32,
    weight_start:u32,
    sum:[f64;16],
    width:usize,
    total:f64,
    best:Option<(u32,f64,u32)>,
    constant:Option<u32>,
    equality:Option<SourceValueEquality>,
    numeric_samples:HistoryFoldIndex<(u32,u32),u32>,
    corners:HistoryFoldIndex<(u32,u32),u32>,
    name:Option<String>,
    indices:Vec<u32>,
    normal_values:Vec<semio_framework_value::DslValue>,
    normal_samples:HistoryFoldIndex<(u32,bool),u32>,
    cursor:usize,
    phase:u8,
}

impl MeshSourceRemap {
    pub fn new(mesh:HalfedgeMesh,vertices:Option<Vec<u32>>,faces:Option<Vec<u32>>,mirrored_faces:Option<Vec<bool>>,normal_axis:Option<usize>)->Self {Self {data:MeshSourceRemapData {vertex_contributors:None,corner_faces:None,generated_vertices:None,generated_faces:None,corner_sources:None,edge_sources:None,edge_source_keys:None,mesh,vertices,faces,mirrored_faces,normal_axis},face_values_base:0,influence:None,influence_face:0,influence_vertex:0,weight_cursor:0,weight_next:0,weight_start:0,sum:[0.0;16],width:0,total:0.0,best:None,constant:None,equality:None,numeric_samples:HistoryFoldIndex::new(),corners:HistoryFoldIndex::new(),name:None,indices:Vec::new(),normal_values:Vec::new(),normal_samples:HistoryFoldIndex::new(),cursor:0,phase:0}}
    fn advance_owned(&mut self,source:&mut HalfedgeMesh)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{
                if source.attributes.is_empty() && source.uv_seams.is_empty() {self.phase=2;}else if self.cursor==source.halfedge_count() {self.cursor=0;self.phase=1;}else {let edge=&source.halfedges[self.cursor];self.corners.insert((edge.face.unwrap(),edge.vertex),self.cursor as u32);self.cursor+=1;}
            }
            1=>{
                if source.attributes.is_empty() {self.cursor=0;self.phase=7;return Ok(None);}
                if self.name.is_none() {self.name=source.attributes.first_key_value().map(|(name,_)|name.clone());}
                let attribute=&source.attributes[self.name.as_ref().unwrap()];
                let reflect=attribute.semantic==MeshAttributeSemantic::Normal && self.data.normal_axis.is_some();
                let domain=if reflect && attribute.domain==MeshAttributeDomain::Vertex {MeshAttributeDomain::Corner}else {attribute.domain};
                let count=match domain {MeshAttributeDomain::Vertex=>self.data.mesh.vertex_count(),MeshAttributeDomain::Face=>self.data.mesh.face_count(),_=>self.data.mesh.halfedge_count()};
                if self.cursor==count {
                    if (reflect || (attribute.interpolation==MeshAttributeInterpolation::Linear && self.data.generated_vertices.is_some() && domain!=MeshAttributeDomain::Face)) && !attribute.values.is_empty() {self.phase=5;return Ok(None);}
                    if self.data.generated_faces.is_some() && domain==MeshAttributeDomain::Face && attribute.interpolation==MeshAttributeInterpolation::Linear && !self.normal_values.is_empty() {self.face_values_base=attribute.values.len();self.weight_cursor=0;self.phase=8;return Ok(None);}
                    let name=self.name.take().unwrap();let mut attribute=source.attributes.remove(&name).unwrap();attribute.domain=domain;attribute.indices=Some(std::mem::take(&mut self.indices));
                    if reflect || (attribute.interpolation==MeshAttributeInterpolation::Linear && self.data.generated_vertices.is_some() && domain!=MeshAttributeDomain::Face) {attribute.values=std::mem::take(&mut self.normal_values);}
                    self.data.mesh.attributes.insert(name,attribute);self.cursor=0;
                }else {
                    if domain==MeshAttributeDomain::Edge {if let Some(keys)=&self.data.edge_source_keys {let original=*self.corners.get(&keys[self.cursor]).ok_or_else(||MeshKernelError::InvalidInput("mesh source edge key is absent".into()))?;let original=attribute.indices.as_ref().map_or(original,|indices|indices[original as usize]);let sample=if attribute.interpolation==MeshAttributeInterpolation::Linear && self.data.generated_vertices.is_some() {if let Some(sample)=self.normal_samples.get(&(original,false)) {*sample}else {let sample=self.normal_values.len()as u32;self.normal_values.push(attribute.values[original as usize].clone());self.normal_samples.insert((original,false),sample);sample}}else {original};self.indices.push(sample);self.cursor+=1;return Ok(None);}}
                    if domain==MeshAttributeDomain::Face && self.data.generated_faces.as_ref().is_some_and(|faces|faces[self.cursor].len()>1) {
                        self.influence=Some(SourceVertex::FaceSamples(self.cursor as u32));self.influence_face=self.cursor as u32;self.influence_vertex=self.cursor as u32;self.weight_cursor=0;self.sum=[0.0;16];self.width=0;self.total=0.0;self.best=None;self.constant=None;self.phase=3;return Ok(None);
                    }
                    if !reflect && matches!(domain,MeshAttributeDomain::Corner|MeshAttributeDomain::Edge) {
                        let overrides=if domain==MeshAttributeDomain::Edge {&self.data.edge_sources}else {&self.data.corner_sources};
                        if let Some(original)=overrides.as_ref().and_then(|sources|sources[self.cursor]) {let original=attribute.indices.as_ref().map_or(original,|indices|indices[original as usize]);let sample=if attribute.interpolation==MeshAttributeInterpolation::Linear && self.data.generated_vertices.is_some() {if let Some(sample)=self.normal_samples.get(&(original,false)) {*sample}else {let sample=self.normal_values.len()as u32;self.normal_values.push(attribute.values[original as usize].clone());self.normal_samples.insert((original,false),sample);sample}}else {original};self.indices.push(sample);self.cursor+=1;return Ok(None);}
                    }
                    if let Some(vertices)=&self.data.generated_vertices {
                        if domain!=MeshAttributeDomain::Face {
                            let (vertex,face)=if domain==MeshAttributeDomain::Vertex {(self.cursor as u32,u32::MAX)}else {let edge=&self.data.mesh.halfedges[self.cursor];(edge.vertex,self.data.corner_faces.as_ref().map_or_else(||self.data.faces.as_ref().map_or(edge.face.unwrap(),|faces|faces[edge.face.unwrap() as usize]),|faces|faces[self.cursor]))};
                            if attribute.interpolation==MeshAttributeInterpolation::Linear {if let Some(index)=self.numeric_samples.get(&(vertex,face)) {self.indices.push(*index);self.cursor+=1;return Ok(None);}}
                            let mut influence=vertices[vertex as usize];
                            if domain==MeshAttributeDomain::Edge {
                                if let SourceVertex::Original(start)=influence {
                                    let end=vertices[self.data.mesh.halfedges[self.data.mesh.halfedges[self.cursor].next as usize].vertex as usize];
                                    let pair=match end {SourceVertex::Original(end)=>Some((start,end)),SourceVertex::Edge{a,b,..} if start==a || start==b=>Some((a,b)),_=>None};
                                    if let Some((a,b))=pair {if let Some(edge)=self.corners.get(&(face,a)).copied() {if source.halfedges[source.halfedges[edge as usize].next as usize].vertex==b {influence=SourceVertex::Original(a);}else if let Some(edge)=self.corners.get(&(face,b)).copied() {if source.halfedges[source.halfedges[edge as usize].next as usize].vertex==a {influence=SourceVertex::Original(b);}}}}
                                }
                                if let SourceVertex::Edge{a,b,..}=influence {let edge=self.corners.get(&(face,a)).copied();let opposite=self.corners.get(&(face,b)).copied();if edge.is_some_and(|edge|source.halfedges[source.halfedges[edge as usize].next as usize].vertex==b) || opposite.is_some_and(|edge|source.halfedges[source.halfedges[edge as usize].next as usize].vertex==a) {influence=SourceVertex::Original(if edge.is_some_and(|edge|source.halfedges[source.halfedges[edge as usize].next as usize].vertex==b) {a}else {b});}}
                            }
                            if attribute.interpolation==MeshAttributeInterpolation::Linear {
                                if let SourceVertex::Original(vertex)=influence {
                                    let id=if attribute.domain==MeshAttributeDomain::Vertex {vertex}else {*self.corners.get(&(face,vertex)).ok_or_else(||MeshKernelError::InvalidInput("mesh source corner is absent".into()))?};
                                    let original=attribute.indices.as_ref().map_or(id,|indices|indices[id as usize]);
                                    let sample=if let Some(sample)=self.normal_samples.get(&(original,false)) {*sample}else {let sample=self.normal_values.len() as u32;self.normal_values.push(attribute.values[original as usize].clone());self.normal_samples.insert((original,false),sample);sample};
                                    self.indices.push(sample);self.cursor+=1;return Ok(None);
                                }
                            }
                            self.influence=Some(influence);self.influence_face=face;self.influence_vertex=vertex;self.weight_cursor=0;self.sum=[0.0;16];self.width=0;self.total=0.0;self.best=None;self.constant=None;
                            if let SourceVertex::Face(face)=influence {self.weight_start=source.faces[face as usize].halfedge;self.weight_next=self.weight_start;}
                            self.phase=3;return Ok(None);
                        }
                    }
                    let source_vertex=|vertex|self.data.vertices.as_ref().map_or(vertex,|vertices|vertices[vertex as usize]);
                    let source_face=|face|self.data.faces.as_ref().map_or(face,|faces|faces[face as usize]);
                    let (id,mirrored)=match domain {
                        MeshAttributeDomain::Vertex=>(source_vertex(self.cursor as u32),false),
                        MeshAttributeDomain::Face=>(self.data.generated_faces.as_ref().map_or_else(||source_face(self.cursor as u32),|faces|faces[self.cursor][0]),false),
                        _=>{let overrides=if domain==MeshAttributeDomain::Edge {&self.data.edge_sources}else {&self.data.corner_sources};
                            if let Some(original)=overrides.as_ref().and_then(|sources|sources[self.cursor]) {let index=attribute.indices.as_ref().map_or(original,|indices|indices[original as usize]);self.indices.push(index);self.cursor+=1;return Ok(None);}
                            let edge=&self.data.mesh.halfedges[self.cursor];let face=edge.face.unwrap();let vertex=source_vertex(edge.vertex);let original_face=source_face(face);let mut corner=self.corners[&(original_face,vertex)];
                            if domain==MeshAttributeDomain::Edge {let end=source_vertex(self.data.mesh.halfedges[edge.next as usize].vertex);if source.halfedges[source.halfedges[corner as usize].next as usize].vertex!=end {corner=self.corners[&(original_face,end)];}}
                            (if attribute.domain==MeshAttributeDomain::Vertex {vertex}else {corner},self.data.mirrored_faces.as_ref().is_some_and(|faces|faces[face as usize]))
                        },
                    };
                    let original=attribute.indices.as_ref().map_or(id,|indices|indices[id as usize]);
                    let index=if reflect {
                        if let Some(index)=self.normal_samples.get(&(original,mirrored)) {*index}else {let tuple=attribute.values[original as usize].as_array().unwrap();let axis=self.data.normal_axis.unwrap();let normal=semio_framework_value::DslValue::Array(tuple.iter().enumerate().map(|(coordinate,value)|semio_framework_value::DslValue::float(value.as_f64().unwrap()*if mirrored && coordinate==axis {-1.0}else {1.0})).collect());let index=self.normal_values.len() as u32;self.normal_values.push(normal);self.normal_samples.insert((original,mirrored),index);index}
                    }else {original};
                    self.indices.push(index);self.cursor+=1;
                }
            }
            2=>{self.data.mesh.materials=std::mem::take(&mut source.materials);self.data.mesh.textures=std::mem::take(&mut source.textures);return Ok(Some(std::mem::replace(&mut self.data.mesh,HalfedgeMesh::empty())));},
            3=>{
                let influence=self.influence.unwrap();
                let contribution=match influence {
                    SourceVertex::Original(vertex)=>(self.weight_cursor==0).then_some((vertex,1.0)),
                    SourceVertex::Contributors(group)=>self.data.vertex_contributors.as_ref().unwrap()[group as usize].get(self.weight_cursor).map(|vertex|(*vertex,1.0)),
                    SourceVertex::Edge{a,b,t}=>match self.weight_cursor {0=>Some((a,1.0-t)),1=>Some((b,t)),_=>None},
                    SourceVertex::Weighted{vertices,weights}=>vertices.get(self.weight_cursor).map(|vertex|(*vertex,weights[self.weight_cursor])),
                    SourceVertex::FaceSamples(face)=>self.data.generated_faces.as_ref().unwrap()[face as usize].get(self.weight_cursor).map(|face|(*face,1.0)),
                    SourceVertex::Face(_)=>if self.weight_cursor>0 && self.weight_next==self.weight_start {None}else {let edge=&source.halfedges[self.weight_next as usize];self.weight_next=edge.next;Some((edge.vertex,1.0))},
                };
                let attribute=&source.attributes[self.name.as_ref().unwrap()];
                if let Some((vertex,weight))=contribution {
                    self.weight_cursor+=1;if !weight.is_finite() || weight<0.0 {return Err(MeshKernelError::InvalidInput("mesh interpolation requires finite nonnegative weights".into()));}if weight==0.0 {return Ok(None);}
                    let id=if matches!(attribute.domain,MeshAttributeDomain::Vertex|MeshAttributeDomain::Face) {vertex}else {match self.corners.get(&(self.influence_face,vertex)) {Some(id)=>*id,None if matches!(influence,SourceVertex::Edge{..}|SourceVertex::Contributors(_))=>return Ok(None),None=>return Err(MeshKernelError::InvalidInput("mesh corner interpolation source is absent".into()))}};
                    let sample=attribute.indices.as_ref().map_or(id,|indices|indices[id as usize]);
                    if self.best.is_none_or(|(best,best_weight,_)|weight>best_weight || weight==best_weight && id<best) {self.best=Some((id,weight,sample));}
                    self.total+=weight;
                    match attribute.interpolation {
                        MeshAttributeInterpolation::Linear=>{
                            let value=&attribute.values[sample as usize];let width=value.as_array().map_or(0,|values|values.len());if self.total==weight {self.width=width;}else if self.total>weight && self.width!=width {return Err(MeshKernelError::InvalidInput("mesh linear interpolation dimensions disagree".into()));}
                            if width==0 {self.sum[0]+=value.as_f64().unwrap()*weight;}else {for axis in 0..width {self.sum[axis]+=value.as_array().unwrap()[axis].as_f64().unwrap()*weight;}}
                        },
                        MeshAttributeInterpolation::Constant=>{if let Some(first)=self.constant {if first!=sample {self.equality=Some(SourceValueEquality::new(first,sample));self.phase=4;}}else {self.constant=Some(sample);}},
                        MeshAttributeInterpolation::Nearest=>{},
                    }
                }else {
                    if self.total<=0.0 || !self.total.is_finite() {return Err(MeshKernelError::InvalidInput("mesh interpolation weights are empty".into()));}
                    let sample=if attribute.interpolation==MeshAttributeInterpolation::Linear {
                        let width=if self.width==0 {1}else {self.width};let mut values=self.sum[..width].iter().map(|value|*value/self.total).collect::<Vec<_>>();
                        if attribute.semantic==MeshAttributeSemantic::Normal {let length=values[0].hypot(values[1]).hypot(values[2]);if length==0.0 || !length.is_finite() {return Err(MeshKernelError::InvalidInput("mesh interpolated normal is zero".into()));}for value in &mut values {*value/=length;}}
                        if values.iter().any(|value|!value.is_finite() || !(*value as f32).is_finite()) {return Err(MeshKernelError::InvalidInput("mesh interpolated channel exceeds numeric range".into()));}
                        let sample=if attribute.domain==MeshAttributeDomain::Face {attribute.values.len().saturating_add(self.normal_values.len()) as u32}else {self.normal_values.len() as u32};self.normal_values.push(if self.width==0 {semio_framework_value::DslValue::float(values[0])}else {semio_framework_value::DslValue::Array(values.into_iter().map(semio_framework_value::DslValue::float).collect())});self.numeric_samples.insert((self.influence_vertex,self.influence_face),sample);sample
                    }else if attribute.interpolation==MeshAttributeInterpolation::Constant {self.constant.unwrap()}else {self.best.unwrap().2};
                    self.indices.push(sample);self.cursor+=1;self.phase=1;
                }
            }
            4=>{let attribute=&source.attributes[self.name.as_ref().unwrap()];if let Some(equal)=self.equality.as_mut().unwrap().step(&attribute.values)? {self.equality=None;if !equal {return Err(MeshKernelError::InvalidInput(format!("constant mesh attribute '{}' has conflicting source values",self.name.as_ref().unwrap())));}self.phase=3;}},
            7=>{
                if source.uv_seams.is_empty() || self.cursor==self.data.mesh.halfedge_count() {self.phase=2;return Ok(None);}
                if let Some(keys)=&self.data.edge_source_keys {let original=*self.corners.get(&keys[self.cursor]).ok_or(MeshKernelError::InvalidHandle)?;if source.uv_seams.contains(&original) || source.halfedges[original as usize].twin.is_some_and(|twin|source.uv_seams.contains(&twin)) {self.data.mesh.uv_seams.insert(self.cursor as u32);}self.cursor+=1;return Ok(None);}
                if let Some(original)=self.data.edge_sources.as_ref().and_then(|sources|sources[self.cursor]) {if source.uv_seams.contains(&original) || source.halfedges[original as usize].twin.is_some_and(|twin|source.uv_seams.contains(&twin)) {self.data.mesh.uv_seams.insert(self.cursor as u32);}self.cursor+=1;return Ok(None);}
                let edge=&self.data.mesh.halfedges[self.cursor];let face=self.data.corner_faces.as_ref().map_or_else(||self.data.faces.as_ref().map_or(edge.face.unwrap(),|faces|faces[edge.face.unwrap() as usize]),|faces|faces[self.cursor]);
                let origin=|vertex:u32|self.data.generated_vertices.as_ref().map_or_else(||SourceVertex::Original(self.data.vertices.as_ref().map_or(vertex,|vertices|vertices[vertex as usize])),|vertices|vertices[vertex as usize]);
                let (a,b)=(origin(edge.vertex),origin(self.data.mesh.halfedges[edge.next as usize].vertex));
                let pair=match (a,b) {(SourceVertex::Original(a),SourceVertex::Original(b))=>Some((a,b)),(SourceVertex::Original(vertex),SourceVertex::Edge{a,b,..})|(SourceVertex::Edge{a,b,..},SourceVertex::Original(vertex)) if vertex==a || vertex==b=>Some((a,b)),(SourceVertex::Edge{a,b,..},SourceVertex::Edge{a:c,b:d,..}) if a==c && b==d || a==d && b==c=>Some((a,b)),_=>None};
                if let Some((a,b))=pair {let original=self.corners.get(&(face,a)).copied().filter(|id|source.halfedges[source.halfedges[*id as usize].next as usize].vertex==b).or_else(||self.corners.get(&(face,b)).copied().filter(|id|source.halfedges[source.halfedges[*id as usize].next as usize].vertex==a));if original.is_some_and(|id|source.uv_seams.contains(&id) || source.halfedges[id as usize].twin.is_some_and(|twin|source.uv_seams.contains(&twin))) {self.data.mesh.uv_seams.insert(self.cursor as u32);}}
                self.cursor+=1;
            }
            5=>{let attribute=source.attributes.get_mut(self.name.as_ref().unwrap()).unwrap();if attribute.values.pop().is_none() {self.phase=6;}},
            6=>{if self.normal_samples.pop_first().is_none() && self.numeric_samples.pop_first().is_none() {self.phase=1;}},
            8=>{if let Some(value)=self.normal_values.pop() {source.attributes.get_mut(self.name.as_ref().unwrap()).unwrap().values.push(value);}else {self.phase=9;}},
            9=>{if self.weight_cursor==self.indices.len() {self.phase=6;}else {let index=&mut self.indices[self.weight_cursor];if *index as usize>=self.face_values_base {let count=source.attributes[self.name.as_ref().unwrap()].values.len()-self.face_values_base;*index=(self.face_values_base+count-1-(*index as usize-self.face_values_base)) as u32;}self.weight_cursor+=1;}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

pub struct MeshPolygonNormal {
    origin: Vec3,
    sum: [f64; 3],
}

impl Default for MeshPolygonNormal {
    fn default() -> Self { Self { origin: Vec3::ZERO, sum: [0.0; 3] } }
}

impl MeshPolygonNormal {
    pub fn add(&mut self, a: Vec3, b: Vec3) {
        let a = [0, 1, 2].map(|i| a.0[i] as f64 - self.origin.0[i] as f64);
        let b = [0, 1, 2].map(|i| b.0[i] as f64 - self.origin.0[i] as f64);
        self.sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        self.sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        self.sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    pub fn value(&self) -> Vec3 {
        let length = self.sum[0].hypot(self.sum[1]).hypot(self.sum[2]);
        if length == 0.0 { Vec3::ZERO } else { Vec3(self.sum.map(|v| (v / length) as f32)).normalize() }
    }
}

pub struct MeshSourceSnapshot {
    reservation:u8,
    pub source: HalfedgeMesh,
    pub soup: Soup,
    vertex: usize,
    face: usize,
    halfedge: Option<u32>,
    corners: Vec<u32>,
    normal: MeshPolygonNormal,
    normal_corner: usize,
    normalizing: bool,
    reversing: Option<usize>,
}

impl MeshSourceSnapshot {
    pub fn new(source: HalfedgeMesh) -> Self {
        Self { reservation:0,source, soup: Soup::default(), vertex: 0, face: 0, halfedge: None, corners: Vec::new(), normal: MeshPolygonNormal::default(), normal_corner: 0, normalizing: false, reversing: None }
    }

    fn advance_original(&mut self) -> bool {
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
                    self.normal = MeshPolygonNormal::default();
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

pub struct MeshTopologyBuild {
    origin_vertices:Vec<u32>,
    soup: Soup,
    mesh: HalfedgeMesh,
    used: HistoryFoldSet<u32>,
    remap: HistoryFoldIndex<u32, u32>,
    edges: HistoryFoldIndex<(u32, u32), u32>,
    face: usize,
    corner: usize,
    vertex: usize,
    start: u32,
    normal: MeshPolygonNormal,
    phase: u8,
    closed: bool,
    retain_unreferenced: bool,
    admitted_corners: usize,
}

impl MeshTopologyBuild {
    pub fn new(soup: Soup, closed: bool) -> Self {
        Self { origin_vertices:Vec::new(),soup, mesh: HalfedgeMesh::empty(), used: HistoryFoldSet::new(), remap: HistoryFoldIndex::new(), edges: HistoryFoldIndex::new(), face: 0, corner: 0, vertex: 0, start: 0, normal: MeshPolygonNormal::default(), phase: 0, closed, retain_unreferenced: false, admitted_corners: 0 }
    }
    pub fn unreferenced(soup: Soup) -> Self { let mut build = Self::new(soup,false); build.retain_unreferenced = true; build }

    fn advance_owned(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => {
                if self.face == self.soup.faces.len() { self.phase = 1; self.face = 0; }
                else {
                    let face = &self.soup.faces[self.face];
                    if self.soup.faces.len()>100_000 || self.soup.positions.len()>100_000 || self.admitted_corners>=600_000 { return Err(MeshKernelError::InvalidInput("generated mesh exceeds capacity".into())); } self.admitted_corners+=1;
                    if face.len() < 3 { return Err(MeshKernelError::DegenerateOperation); }
                    self.used.insert(face[self.corner]);
                    self.corner += 1;
                    if self.corner == face.len() { self.corner = 0; self.face += 1; }
                }
            }
            1 => {
                if self.vertex == self.soup.positions.len() { self.phase = 2; }
                else {
                    if self.retain_unreferenced || self.used.contains(&(self.vertex as u32)) {
                        self.remap.insert(self.vertex as u32, self.mesh.vertices.len() as u32);
                        self.origin_vertices.push(self.vertex as u32);
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
                if self.corner == face.len() { self.phase = 3; self.corner = 0; self.face += 1; self.normal = MeshPolygonNormal::default(); }
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

impl semio_framework_value::retirement::RetireOwned for MeshImportJob {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.polygon),semio_framework_value::retirement::deferred(self.primitive_faces),semio_framework_value::retirement::deferred(self.midpoints),semio_framework_value::retirement::deferred(self.mids),semio_framework_value::retirement::deferred(self.positions),semio_framework_value::retirement::deferred(self.indices),semio_framework_value::retirement::deferred(self.normals),semio_framework_value::retirement::deferred(self.unique),semio_framework_value::retirement::deferred(self.remap),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.corner_normals),semio_framework_value::retirement::deferred(self.mesh),semio_framework_value::retirement::deferred(self.build)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.polygon),semio_framework_value::retirement::deferred_birth_bytes_for(&self.primitive_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.midpoints),semio_framework_value::retirement::deferred_birth_bytes_for(&self.mids),semio_framework_value::retirement::deferred_birth_bytes_for(&self.positions),semio_framework_value::retirement::deferred_birth_bytes_for(&self.indices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.normals),semio_framework_value::retirement::deferred_birth_bytes_for(&self.unique),semio_framework_value::retirement::deferred_birth_bytes_for(&self.remap),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corner_normals),semio_framework_value::retirement::deferred_birth_bytes_for(&self.mesh),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for Coplanar {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.explicit),semio_framework_value::retirement::deferred(self.vertex_input),semio_framework_value::retirement::deferred(self.selected_vertices),semio_framework_value::retirement::deferred(self.star_incident),semio_framework_value::retirement::deferred(self.star_edges),semio_framework_value::retirement::deferred(self.star_next),semio_framework_value::retirement::deferred(self.star_contributors),semio_framework_value::retirement::deferred(self.output_vertices),semio_framework_value::retirement::deferred(self.source_corners),semio_framework_value::retirement::deferred(self.face_sources),semio_framework_value::retirement::deferred(self.corner_sources),semio_framework_value::retirement::deferred(self.edge_sources),semio_framework_value::retirement::deferred(self.spliced_corners),semio_framework_value::retirement::deferred(self.spliced_edges),semio_framework_value::retirement::deferred(self.output_corners),semio_framework_value::retirement::deferred(self.output_edges),semio_framework_value::retirement::deferred(self.output_faces),semio_framework_value::retirement::deferred(self.attributes),semio_framework_value::retirement::deferred(self.attribute_name),semio_framework_value::retirement::deferred(self.equality),semio_framework_value::retirement::deferred(self.incidence),semio_framework_value::retirement::deferred(self.pending),semio_framework_value::retirement::deferred(self.seen),semio_framework_value::retirement::deferred(self.neighbors),semio_framework_value::retirement::deferred(self.removable),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.build)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.explicit),semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertex_input),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selected_vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.star_incident),semio_framework_value::retirement::deferred_birth_bytes_for(&self.star_edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.star_next),semio_framework_value::retirement::deferred_birth_bytes_for(&self.star_contributors),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output_vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.face_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corner_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.spliced_corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.spliced_edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output_corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output_edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_name),semio_framework_value::retirement::deferred_birth_bytes_for(&self.equality),semio_framework_value::retirement::deferred_birth_bytes_for(&self.incidence),semio_framework_value::retirement::deferred_birth_bytes_for(&self.pending),semio_framework_value::retirement::deferred_birth_bytes_for(&self.seen),semio_framework_value::retirement::deferred_birth_bytes_for(&self.neighbors),semio_framework_value::retirement::deferred_birth_bytes_for(&self.removable),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshMergeJob {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.coplanar),semio_framework_value::retirement::deferred(self.input),semio_framework_value::retirement::deferred(self.selected),semio_framework_value::retirement::deferred(self.seen),semio_framework_value::retirement::deferred(self.roots),semio_framework_value::retirement::deferred(self.remap),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.unique),semio_framework_value::retirement::deferred(self.build),semio_framework_value::retirement::deferred(self.grid),semio_framework_value::retirement::deferred(self.contributors),semio_framework_value::retirement::deferred(self.source_corners),semio_framework_value::retirement::deferred(self.corner_origins),semio_framework_value::retirement::deferred(self.edge_origins),semio_framework_value::retirement::deferred(self.output_corners),semio_framework_value::retirement::deferred(self.output_edges),semio_framework_value::retirement::deferred(self.output_faces),semio_framework_value::retirement::deferred(self.attributes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.coplanar),semio_framework_value::retirement::deferred_birth_bytes_for(&self.input),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selected),semio_framework_value::retirement::deferred_birth_bytes_for(&self.seen),semio_framework_value::retirement::deferred_birth_bytes_for(&self.roots),semio_framework_value::retirement::deferred_birth_bytes_for(&self.remap),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.unique),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build),semio_framework_value::retirement::deferred_birth_bytes_for(&self.grid),semio_framework_value::retirement::deferred_birth_bytes_for(&self.contributors),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corner_origins),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_origins),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output_corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output_edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for SourceValueEquality {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.tasks)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.tasks)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshSourceRemap {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.data.vertex_contributors),semio_framework_value::retirement::deferred(self.data.corner_faces),semio_framework_value::retirement::deferred(self.data.generated_vertices),semio_framework_value::retirement::deferred(self.data.generated_faces),semio_framework_value::retirement::deferred(self.data.corner_sources),semio_framework_value::retirement::deferred(self.data.edge_sources),semio_framework_value::retirement::deferred(self.data.edge_source_keys),semio_framework_value::retirement::deferred(self.equality),semio_framework_value::retirement::deferred(self.numeric_samples),semio_framework_value::retirement::deferred(self.data.mesh),semio_framework_value::retirement::deferred(self.data.vertices),semio_framework_value::retirement::deferred(self.data.faces),semio_framework_value::retirement::deferred(self.data.mirrored_faces),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.name),semio_framework_value::retirement::deferred(self.indices),semio_framework_value::retirement::deferred(self.normal_values),semio_framework_value::retirement::deferred(self.normal_samples)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.vertex_contributors),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.corner_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.generated_vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.generated_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.corner_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.edge_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.edge_source_keys),semio_framework_value::retirement::deferred_birth_bytes_for(&self.equality),semio_framework_value::retirement::deferred_birth_bytes_for(&self.numeric_samples),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.mesh),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.data.mirrored_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.name),semio_framework_value::retirement::deferred_birth_bytes_for(&self.indices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.normal_values),semio_framework_value::retirement::deferred_birth_bytes_for(&self.normal_samples)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshSourceSnapshot {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.source),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.corners)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.source),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for MeshTopologyBuild {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.origin_vertices),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.mesh),semio_framework_value::retirement::deferred(self.used),semio_framework_value::retirement::deferred(self.remap),semio_framework_value::retirement::deferred(self.edges)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.origin_vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.mesh),semio_framework_value::retirement::deferred_birth_bytes_for(&self.used),semio_framework_value::retirement::deferred_birth_bytes_for(&self.remap),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for SourceEqualityTask {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {let (Self::Node(path)|Self::Sequence(path,..)|Self::Text(path,..))=self;semio_framework_value::retirement::deferred(path)}
    fn retirement_birth_bytes(&self)->Option<usize> {let (Self::Node(path)|Self::Sequence(path,..)|Self::Text(path,..))=self;Some(semio_framework_value::retirement::deferred_birth_bytes_for(path))}
    fn controlled_retirement_supported()->bool {true}
}

/// 🔑️ Quantizes the same f32 precision grid consumed by retained welding.
pub fn mesh_weld_key(point:[f32;3],precision:f32)->MeshResult<(i64,i64,i64)> {
    if !precision.is_finite()||precision<=0.0 {return Err(MeshKernelError::InvalidInput("precision must be positive and finite".into()));}
    let scale=1.0/precision.max(1e-9);let key=point.map(|value|(value as f64*scale as f64).round());
    if key.iter().any(|value|!value.is_finite()||*value<i64::MIN as f64||*value>=i64::MAX as f64){return Err(MeshKernelError::InvalidInput("weld coordinate exceeds precision grid".into()));}
    Ok((key[0]as i64,key[1]as i64,key[2]as i64))
}

/// 🧭️ Fixed merge intentions over caller-owned document DATA.
pub enum MeshMergeIntent {Coplanar,DissolveEdges(Vec<EdgeId>),DissolveVertices(Vec<VertexId>),Vertices {vertices:Vec<VertexId>,mode:WeldMode,threshold:f32},Grid {precision:f32}}

impl MeshMergeJob {
    pub fn new(source:HalfedgeMesh,intent:MeshMergeIntent)->MeshResult<Self> {
        let(coplanar,input,mode,threshold,quantized)=match intent {
            MeshMergeIntent::Coplanar=>(Some(Coplanar::default()),Vec::new(),WeldMode::Center,0.0,false),
            MeshMergeIntent::DissolveEdges(edges)=>{if edges.is_empty(){return Err(MeshKernelError::EmptySelection);}let mut work=Coplanar::default();work.explicit=Some(edges);work.preserve_corners=true;work.phase=23;(Some(work),Vec::new(),WeldMode::Center,0.0,false)},
            MeshMergeIntent::DissolveVertices(vertices)=>{if vertices.is_empty(){return Err(MeshKernelError::EmptySelection);}let mut work=Coplanar::default();work.vertex_input=Some(vertices);work.preserve_corners=true;work.phase=23;(Some(work),Vec::new(),WeldMode::Center,0.0,false)},
            MeshMergeIntent::Vertices {vertices,mode,threshold}=>{if vertices.is_empty(){return Err(MeshKernelError::EmptySelection);}if mode==WeldMode::ByDistance&&(!threshold.is_finite()||threshold<0.0){return Err(MeshKernelError::DegenerateOperation);}(None,vertices,mode,threshold,false)},
            MeshMergeIntent::Grid {precision}=>{if !precision.is_finite()||precision<=0.0{return Err(MeshKernelError::InvalidInput("precision must be positive and finite".into()));}(None,Vec::new(),WeldMode::First,precision,true)},
        };Ok(Self::configured(source,coplanar,input,mode,threshold,quantized))
    }
    pub fn snapshot(&self)->&MeshSourceSnapshot {&self.snapshot}
    pub fn snapshot_frontier(&self)->Option<&MeshSourceSnapshot>{if self.phase>=254{return None;}self.coplanar.as_ref().map_or(self.phase==1,|child|child.phase==0).then_some(&self.snapshot)}
    pub fn snapshot_frontier_mut(&mut self)->Option<&mut MeshSourceSnapshot>{if self.phase>=254{return None;}if self.coplanar.as_ref().map_or(self.phase==1,|child|child.phase==0){Some(&mut self.snapshot)}else{None}}
    pub fn coplanar_merge_count(&self)->Option<usize>{self.coplanar.as_ref().map(|work|work.merges)}
    pub fn welded_vertex_count(&self)->Option<usize>{self.quantized.then_some(self.removed)}
    pub fn transfer_remaining_assets(&mut self,mesh:&mut HalfedgeMesh)->MeshResult<()>{if self.phase!=255{return Err(MeshKernelError::InvalidInput("merge output has not completed".into()));}if !self.snapshot.source.materials.is_empty(){mesh.materials=std::mem::take(&mut self.snapshot.source.materials);}if !self.snapshot.source.textures.is_empty(){mesh.textures=std::mem::take(&mut self.snapshot.source.textures);}Ok(())}
}

/// 🧊️ Closed primitive family supported by the actual retained import producer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u8)]
pub enum MeshPrimitiveKind {Box=0,Plane=1,Cylinder=2,Cone=3,IcoSphere=4}

impl MeshImportJob {
    pub fn indexed(positions:Vec<f32>,indices:Vec<u32>,normals:Vec<f32>)->MeshResult<(Self,MeshModelingProgress)>{
        if !positions.len().is_multiple_of(3)||!indices.len().is_multiple_of(3)||!normals.is_empty()&&normals.len()!=positions.len(){return Err(MeshKernelError::InvalidInput("invalid indexed triangulation buffers".into()));}
        if indices.len()/3>100_000||positions.len().saturating_add(indices.len()).saturating_add(normals.len()).saturating_mul(4)>64_000_000{return Err(MeshKernelError::InvalidInput("indexed triangulation exceeds mesh capacity".into()));}
        let total=positions.len().saturating_add(indices.len().saturating_mul(16)).saturating_add(1);
        let work=Self {polygon:None,primitive:None,primitive_faces:Vec::new(),midpoints:HistoryFoldIndex::new(),mids:Vec::new(),generation:0,face:0,part:0,positions,indices,normals,unique:HistoryFoldIndex::new(),remap:Vec::new(),soup:Soup::default(),corners:Vec::new(),corner_normals:Vec::new(),mesh:None,cursor:0,build:None,phase:0};
        Ok((work,MeshModelingProgress {units_done:0,units_total:total,phase:"mesh-import-vertices"}))
    }
    pub fn polygon(source:semio_framework_mesh_engine::PolygonMeshSource)->MeshResult<(Self,MeshModelingProgress)>{let(mut work,mut progress)=Self::indexed(Vec::new(),Vec::new(),Vec::new())?;work.polygon=Some(source);work.phase=10;progress.phase=work.phase();Ok((work,progress))}
    pub fn primitive(kind:MeshPrimitiveKind,dimensions:[f32;3],parameter:u32)->MeshResult<(Self,MeshModelingProgress)>{
        let kind=kind as u8;let width=if kind==0{3}else if kind==4{1}else{2};
        if dimensions[..width].iter().any(|value|!value.is_finite()||*value<=0.0)||matches!(kind,2|3)&&!(3..=100_000).contains(&parameter)||kind==4&&parameter>5{return Err(MeshKernelError::InvalidInput("invalid primitive dimensions or subdivision".into()));}
        let(vertices,faces,corners)=match kind{0=>(8,6,24),1=>(4,1,4),2=>(parameter as usize*2,parameter as usize+2,parameter as usize*6),3=>(parameter as usize+1,parameter as usize+1,parameter as usize*4),_=>(10*4usize.pow(parameter)+2,20*4usize.pow(parameter),60*4usize.pow(parameter))};
        if vertices>100_000||faces>100_000||corners>600_000{return Err(MeshKernelError::InvalidInput("primitive exceeds mesh capacity".into()));}
        let(mut work,_)=Self::indexed(Vec::new(),Vec::new(),Vec::new())?;work.primitive=Some((kind,dimensions,parameter));work.phase=4;let phase=work.phase();Ok((work,MeshModelingProgress {units_done:0,units_total:vertices.saturating_add(corners.saturating_mul(16)).saturating_add(1),phase}))
    }
    pub fn take_corner_normals(&mut self)->Option<(Vec<f32>,Vec<u32>)>{if self.phase<254||self.normals.is_empty(){None}else{Some((std::mem::take(&mut self.normals),std::mem::take(&mut self.indices)))}}
}

/// 📊️ Immutable scalar projection frontier; owned source and output buffers remain separate.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct MeshSnapshotState {pub reservation:u8,pub vertex:usize,pub face:usize,pub halfedge:Option<u32>,pub normal_corner:usize,pub normalizing:bool,pub reversing:Option<usize>}
impl MeshSourceSnapshot {
    pub fn state(&self)->MeshSnapshotState{MeshSnapshotState {reservation:self.reservation,vertex:self.vertex,face:self.face,halfedge:self.halfedge,normal_corner:self.normal_corner,normalizing:self.normalizing,reversing:self.reversing}}
    pub fn scratch_corners(&self)->&[u32]{&self.corners}
}

/// 📐️ Constant-size reconstruction diagnostics without mutable phase or membership access.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct MeshBuildState {pub phase:u8,pub face:usize,pub corner:usize,pub admitted_corners:usize,pub used_empty:bool}
impl MeshTopologyBuild {
    pub fn origin_vertices(&self)->&[u32]{&self.origin_vertices}
    pub fn take_origin_vertices(&mut self)->MeshResult<Vec<u32>>{if self.phase!=255{return Err(MeshKernelError::InvalidInput("reconstruction origin transfer requires completed output".into()));}Ok(std::mem::take(&mut self.origin_vertices))}
    pub fn state(&self)->MeshBuildState{MeshBuildState {phase:self.phase,face:self.face,corner:self.corner,admitted_corners:self.admitted_corners,used_empty:self.used.is_empty()}}
}
impl MeshPolygonNormal {pub fn set_origin(&mut self,origin:Vec3){self.origin=origin;}}
impl MeshSourceRemap {
    pub fn configuration(&self)->&MeshSourceRemapData{&self.data}
    pub fn configuration_mut(&mut self)->MeshResult<&mut MeshSourceRemapData>{if self.phase!=0{return Err(MeshKernelError::InvalidInput("source remap configuration already consumed".into()));}Ok(&mut self.data)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

impl MeshMergeJob {
    pub fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        if self.phase>=254{return Err(MeshKernelError::InvalidInput("reconstruction owner already terminal".into()));}
        let result=self.advance_owned();match &result {Ok(Some(_))=>self.phase=255,Err(_)=>self.phase=254,_=>{}}result
    }
}

impl MeshImportJob {
    pub fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        if self.phase>=254{return Err(MeshKernelError::InvalidInput("reconstruction owner already terminal".into()));}
        let result=self.advance_owned();match &result {Ok(Some(_))=>self.phase=255,Err(_)=>self.phase=254,_=>{}}result
    }
}

impl MeshSourceRemap {
    pub fn advance(&mut self,source:&mut HalfedgeMesh)->MeshResult<Option<HalfedgeMesh>> {
        if self.phase>=254{return Err(MeshKernelError::InvalidInput("reconstruction owner already terminal".into()));}
        let result=self.advance_owned(source);match &result {Ok(Some(_))=>self.phase=255,Err(_)=>self.phase=254,_=>{}}result
    }
}

impl MeshTopologyBuild {
    pub fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        if self.phase>=254{return Err(MeshKernelError::InvalidInput("reconstruction owner already terminal".into()));}
        let result=self.advance_owned();match &result {Ok(Some(_))=>self.phase=255,Err(_)=>self.phase=254,_=>{}}result
    }
}

#[cfg(feature="mesh-test-observation")]
pub struct MeshTopologyInspection<'a>{pub mesh:&'a HalfedgeMesh,pub soup:&'a Soup,state:MeshBuildState}
#[cfg(feature="mesh-test-observation")]
impl MeshTopologyInspection<'_>{pub fn state(&self)->MeshBuildState{self.state}}
#[cfg(feature="mesh-test-observation")]
impl MeshTopologyBuild{
    /// 🔬️ Borrows the current reconstruction data without advancing it.
    pub fn inspection(&self)->MeshTopologyInspection<'_>{MeshTopologyInspection{mesh:&self.mesh,soup:&self.soup,state:self.state()}}
}
