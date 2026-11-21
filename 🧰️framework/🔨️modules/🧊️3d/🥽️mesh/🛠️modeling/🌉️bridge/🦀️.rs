//! 🌉️ Connects two anchored boundary rims through the retained Mesh reconstruction owner.

use super::*;

pub(super) struct Rim {
    pub edge:u32,
    pub vertices:[u32;2],
    pub corners:[u32;2],
    pub face:u32,
}

pub(super) fn rim_edge(source:&HalfedgeMesh,id:u32)->MeshResult<Rim>{
    let edge=source.halfedges.get(id as usize).ok_or(MeshKernelError::InvalidHandle)?;
    let face=edge.face.ok_or(MeshKernelError::InvalidHandle)?;
    let mut corners=[id,edge.next];
    if source.faces[face as usize].flipped{corners.swap(0,1);}
    Ok(Rim{edge:id,vertices:corners.map(|corner|source.halfedges[corner as usize].vertex),corners,face})
}

pub(super) type RimIndex=HistoryFoldIndex<u32,Option<u32>>;
pub(super) fn index_rim(outgoing:&mut RimIndex,incoming:&mut RimIndex,rim:&Rim){
    outgoing.entry(rim.vertices[0]).and_modify(|row|*row=None).or_insert_with(||Some(rim.edge));
    incoming.entry(rim.vertices[1]).and_modify(|row|*row=None).or_insert_with(||Some(rim.edge));
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Bridge {
    pub(super) snapshot:Snapshot,
    anchors:[EdgeId;2],
    outgoing:HistoryFoldIndex<u32,Option<u32>>,
    incoming:HistoryFoldIndex<u32,Option<u32>>,
    vertices:HistoryFoldSet<u32>,
    loops:[Vec<u32>;2],
    steps:[usize;2],
    rim:usize,
    probe:u32,
    cursor:usize,
    faces:Vec<u32>,
    corners:Vec<Option<u32>>,
    edges:Vec<Option<u32>>,
    build:Option<Build>,
    pub(super) attributes:Option<SourceAttributeRemap>,
    pub(super) phase:u8,
    chart:WallChart,
}

impl Bridge {
    pub(super) fn new(source:HalfedgeMesh,anchors:[EdgeId;2])->MeshResult<Self>{
        if anchors[0]==anchors[1] {return Err(Self::invalid("bridge requires two distinct anchors"));}
        for anchor in anchors {
            let edge=source.halfedges.get(anchor.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            if edge.twin.is_some()||edge.face.is_none(){return Err(Self::invalid("bridge anchors must be boundary edges"));}
        }
        let chart=WallChart::new(&source);
        Ok(Self{snapshot:Snapshot::new(source),anchors,outgoing:HistoryFoldIndex::new(),incoming:HistoryFoldIndex::new(),
            vertices:HistoryFoldSet::new(),loops:[Vec::new(),Vec::new()],steps:[0;2],rim:0,probe:anchors[0].0,cursor:0,
            faces:Vec::new(),corners:Vec::new(),edges:Vec::new(),build:None,attributes:None,phase:0,
            chart})
    }

    fn invalid(reason:&str)->MeshKernelError {MeshKernelError::InvalidInput(reason.into())}

    pub(super) fn phase(&self)->&'static str {
        ["snapshot","bridge-boundary-index","bridge-boundary-loops","bridge-source-rows","bridge-walls",
            "bridge-reconstruct","bridge-surface","bridge-attributes","bridge-active-uv"][self.phase as usize]
    }

    fn index_boundary(&mut self)->MeshResult<()>{
        if self.cursor==self.snapshot.source.halfedge_count(){
            self.cursor=0;self.phase=2;return Ok(());
        }
        let id=self.cursor as u32;
        if self.snapshot.source.halfedges[self.cursor].twin.is_none(){
            let edge=rim_edge(&self.snapshot.source,id)?;
            index_rim(&mut self.outgoing,&mut self.incoming,&edge);
        }
        self.cursor+=1;
        Ok(())
    }

    fn walk_boundary(&mut self)->MeshResult<()>{
        let edge=rim_edge(&self.snapshot.source,self.probe)?;
        let [start,end]=edge.vertices;
        if self.outgoing.get(&start).copied().flatten()!=Some(self.probe)
            || self.incoming.get(&end).copied().flatten()!=Some(self.probe) {
            return Err(Self::invalid("bridge boundary is ambiguous"));
        }
        if !self.vertices.insert(start){return Err(Self::invalid("bridge requires two disjoint simple boundary loops"));}
        self.loops[self.rim].push(self.probe);
        self.probe=self.outgoing.get(&end).copied().flatten().ok_or_else(||Self::invalid("bridge boundary is open"))?;
        if self.probe!=self.anchors[self.rim].0{return Ok(());}
        if self.rim==0{self.rim=1;self.probe=self.anchors[1].0;return Ok(());}
        let [left,right]=self.loops.each_ref().map(Vec::len);
        if left<3||right<3{return Err(Self::invalid("bridge loops must have at least three edges"));}
        let (mut shared,mut remainder)=(left,right);
        while remainder!=0 {(shared,remainder)=(remainder,shared%remainder);}
        let total=left+right;
        if self.snapshot.source.face_count()+total-shared>100_000
            ||self.snapshot.source.halfedge_count()+3*total-2*shared>600_000 {
            return Err(Self::invalid("bridge exceeds mesh capacity"));
        }
        self.phase=3;
        Ok(())
    }

    fn crossing(&self,step:usize)->u32{
        let rim=&self.loops[0];
        rim[step%rim.len()].min(rim[(step+rim.len()-1)%rim.len()])
    }

    fn wall(&mut self)->MeshResult<()>{
        let [left_count,right_count]=self.loops.each_ref().map(Vec::len);
        if self.steps==[left_count,right_count]{
            self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));
            self.phase=5;return Ok(());
        }
        let [i,j]=self.steps;
        let left=rim_edge(&self.snapshot.source,self.loops[0][i])?;
        let right=rim_edge(&self.snapshot.source,self.loops[1][(right_count-j)%right_count])?;
        let left_progress=(i as u64+1)*right_count as u64;
        let right_progress=(j as u64+1)*left_count as u64;
        use std::cmp::Ordering;
        let quad=[left.vertices[1],left.vertices[0],right.vertices[1],right.vertices[0]];
        let corners=[left.corners[1],left.corners[0],right.corners[1],right.corners[0]];
        let crossing=self.crossing(i);
        let following=self.crossing(i+1);
        let (face,corners,edges)=match left_progress.cmp(&right_progress){
            Ordering::Less=>{
                self.steps[0]+=1;
                (&quad[..3],&corners[..3],[left.edge,crossing,following,0])
            },
            Ordering::Greater=>{
                self.steps[1]+=1;
                (&quad[1..],&corners[1..],[crossing,right.edge,crossing,0])
            },
            Ordering::Equal=>{
                self.steps=[i+1,j+1];
                (&quad[..],&corners[..],[left.edge,crossing,right.edge,following])
            },
        };
        let point=|vertex:u32|self.snapshot.soup.positions[vertex as usize].map(f64::from);
        let a=point(face[0]);
        let vector=|vertex:u32|{let p=point(vertex);(p[0]-a[0],p[1]-a[1],p[2]-a[2])};
        let cross=|u:(f64,f64,f64),v:(f64,f64,f64)|(u.1*v.2-u.2*v.1,u.2*v.0-u.0*v.2,u.0*v.1-u.1*v.0);
        let normal=cross(vector(face[1]),vector(face[2]));
        let next=face.get(3).map_or(normal,|&d|cross(vector(face[2]),vector(d)));
        if normal.0*next.0+normal.1*next.1+normal.2*next.2<=0.0 {
            return Err(Self::invalid("bridge anchors form a folded or degenerate wall"));
        }
        self.snapshot.soup.faces.push(face.to_vec());
        self.faces.push(left.face);
        self.corners.extend(corners.iter().copied().map(Some));
        self.edges.extend(edges.into_iter().take(face.len()).map(Some));
        Ok(())
    }

    pub(super) fn advance(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<Option<HalfedgeMesh>>{
        match self.phase {
            0=>if self.snapshot.complete(){self.phase=1;},
            1=>self.index_boundary()?,
            2=>self.walk_boundary()?,
            3=>{
                if self.faces.len()<self.snapshot.source.face_count(){self.faces.push(self.faces.len() as u32);}
                else if self.corners.len()<self.snapshot.source.halfedge_count(){self.corners.push(None);self.edges.push(None);}
                else {self.phase=4;}
            },
            4=>self.wall()?,
            5=>if let Some(mesh)=self.build.as_mut().unwrap().advance()?{
                let vertices=self.build.as_mut().unwrap().take_origin_vertices()?;
                let mut attributes=SourceAttributeRemap::new(mesh,Some(vertices),Some(std::mem::take(&mut self.faces)),None,None);
                attributes.configuration_mut()?.corner_sources=Some(std::mem::take(&mut self.corners));
                attributes.configuration_mut()?.edge_sources=Some(std::mem::take(&mut self.edges));
                attributes.retain_bridged_surface(self.snapshot.source.face_count());
                self.attributes=Some(attributes);self.phase=6;
            },
            6=>{
                let attributes=self.attributes.as_mut().unwrap();
                if attributes.retaining_refined_surface(){attributes.step_refined_surface(&self.snapshot.source,grant,receipt)?;}
                else {self.phase=7;}
            },
            7=>if let Some(mesh)=self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source)?{
                self.phase=8;
                return Ok(self.chart.accept(mesh));
            },
            8=>return self.chart.advance(self.snapshot.source.face_count()),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct WallChart {
    name:Option<String>,output:Option<HalfedgeMesh>,cursor:usize,
    uvs:Vec<semio_framework_value::DslValue>,retired_uv:Option<MeshAttribute>,
}
impl WallChart {
    pub(super) fn new(mesh:&HalfedgeMesh)->Self {
        let name=mesh.attributes.iter().filter(|(_,attribute)|attribute.semantic==MeshAttributeSemantic::Uv)
            .map(|(name,_)|name).min().cloned();
        Self{name,output:None,cursor:0,uvs:Vec::new(),retired_uv:None}
    }
    pub(super) fn accept(&mut self,mesh:HalfedgeMesh)->Option<HalfedgeMesh>{
        if self.name.is_none(){return Some(mesh);}
        self.output=Some(mesh);None
    }
    pub(super) fn advance(&mut self,original_faces:usize)->MeshResult<Option<HalfedgeMesh>>{
        use semio_framework_value::DslValue;
        let output=self.output.as_mut().unwrap();
        let channel=output.attributes.get_mut(self.name.as_ref().unwrap()).ok_or(MeshKernelError::InvalidHandle)?;
        if let Some(corner)=output.halfedges.get(self.cursor){
            let face=corner.face.ok_or(MeshKernelError::InvalidHandle)? as usize;
            let value=if face>=original_faces{
                DslValue::Array(corner.uv.into_iter().map(|v|DslValue::float(f64::from(v))).collect())
            }else{
                let sample=match channel.domain {
                    MeshAttributeDomain::Vertex=>corner.vertex as usize,
                    MeshAttributeDomain::Face=>face,
                    _=>self.cursor,
                };
                channel.value_at(sample).cloned().ok_or(MeshKernelError::InvalidHandle)?
            };
            self.uvs.push(value);self.cursor+=1;
            return Ok(None);
        }
        self.retired_uv=Some(std::mem::replace(channel,MeshAttribute{
            domain:MeshAttributeDomain::Corner,semantic:MeshAttributeSemantic::Uv,
            interpolation:MeshAttributeInterpolation::Linear,values:std::mem::take(&mut self.uvs),indices:None,
        }));
        Ok(self.output.take())
    }

}
