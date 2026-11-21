//! 📏️ Extrudes a simple exposed boundary chain while retaining its authored surface.

use super::*;
use super::bridge::{RimIndex,WallChart,rim_edge,index_rim};
use semio_framework_3d_mesh_reconstruction::MeshBoundarySweep;

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundaryExtrude {
    pub(super) snapshot:Snapshot,
    input:Vec<EdgeId>,selected:HistoryFoldSet<u32>,out:RimIndex,inc:RimIndex,
    chain:Vec<u32>,head:Option<u32>,cursor:usize,pair:usize,
    offset:[f64;3],sweep:Option<MeshBoundarySweep>,
    points:Vec<u32>,faces:Vec<u32>,corners:Vec<Option<u32>>,edges:Vec<Option<u32>>,
    build:Option<Build>,pub(super) attributes:Option<SourceAttributeRemap>,chart:WallChart,pub(super) phase:u8,
}

fn refuse(message:&str)->MeshKernelError {MeshKernelError::InvalidInput(message.into())}

impl BoundaryExtrude {
    pub(super) fn new(source:HalfedgeMesh,input:Vec<EdgeId>,offset:Vec3)->MeshResult<Self>{
        if input.is_empty(){return Err(MeshKernelError::EmptySelection);}
        if !offset.0.iter().all(|v|v.is_finite())||offset.0==[0.0;3]{return Err(refuse("boundary offset must be finite and nonzero"));}
        let count=input.len();
        if count>2000||source.vertex_count()+count+1>100_000||source.face_count()+count>100_000
            ||source.halfedge_count()+4*count>600_000{return Err(refuse("boundary extrusion exceeds its work or mesh limit"));}
        let chart=WallChart::new(&source);
        Ok(Self{snapshot:Snapshot::new(source),input,selected:HistoryFoldSet::new(),out:RimIndex::new(),inc:RimIndex::new(),
            chain:Vec::new(),head:None,cursor:0,pair:1,offset:offset.0.map(f64::from),sweep:None,
            points:Vec::new(),faces:Vec::new(),corners:Vec::new(),edges:Vec::new(),build:None,attributes:None,chart,phase:0})
    }

    pub(super) fn phase(&self)->&'static str{
        ["snapshot","boundary-extrude-selection","boundary-extrude-index","boundary-extrude-joins","boundary-extrude-chain",
            "boundary-extrude-plane","boundary-extrude-points","boundary-extrude-folds","boundary-extrude-rows","boundary-extrude-walls",
            "boundary-extrude-build","boundary-extrude-source","boundary-extrude-surface","boundary-extrude-attributes","boundary-extrude-uv"][self.phase as usize]
    }

    fn next(&mut self){self.cursor=0;self.phase+=1;}
    fn point(&self,id:u32)->[f64;3]{self.snapshot.soup.positions[id as usize].map(f64::from)}

    fn append_wall(&mut self)->MeshResult<()>{
        let i=self.cursor;
        let rim=rim_edge(&self.snapshot.source,self.chain[i])?;
        let [a,b]=rim.vertices;
        if length3(sub3(self.point(a).into(),self.point(b).into()))<=self.sweep.as_ref().unwrap().epsilon{return Err(refuse("boundary extrusion contains a collapsed edge"));}
        let next=(self.snapshot.source.vertex_count()+i) as u32;
        self.snapshot.soup.faces.push(vec![b,a,next,next+1]);
        self.faces.push(rim.face);
        self.corners.extend([rim.corners[1],rim.corners[0],rim.corners[0],rim.corners[1]].map(Some));
        let start=rim.edge.min(self.chain[i.saturating_sub(1)]);
        let end=rim.edge.min(self.chain[(i+1).min(self.chain.len()-1)]);
        self.edges.extend([rim.edge,start,rim.edge,end].map(Some));
        self.cursor+=1;
        Ok(())
    }

    pub(super) fn advance(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<Option<HalfedgeMesh>>{
        match self.phase {
            0=>if self.snapshot.complete(){self.next();},
            1=>{
                if let Some(&EdgeId(id))=self.input.get(self.cursor){
                    let edge=self.snapshot.source.halfedges.get(id as usize).ok_or(MeshKernelError::InvalidHandle)?;
                    if edge.twin.is_some()||edge.face.is_none()||!self.selected.insert(id){return Err(refuse("select distinct boundary halfedges"));}
                    self.cursor+=1;
                }else{self.next();}
            },
            2=>{
                if let Some(edge)=self.snapshot.source.halfedges.get(self.cursor){
                    if edge.twin.is_none(){
                        let rim=rim_edge(&self.snapshot.source,self.cursor as u32)?;
                        index_rim(&mut self.out,&mut self.inc,&rim);
                    }
                    self.cursor+=1;
                }else{self.next();}
            },
            3=>{
                if let Some(&EdgeId(id))=self.input.get(self.cursor){
                    let [a,b]=rim_edge(&self.snapshot.source,id)?.vertices;
                    let previous=self.inc.get(&a).copied().flatten();
                    if previous.is_none()||self.out.get(&b).copied().flatten().is_none()||
                        self.out.get(&a).copied().flatten()!=Some(id)||self.inc.get(&b).copied().flatten()!=Some(id){
                        return Err(refuse("boundary joins are ambiguous"));
                    }
                    if !self.selected.contains(&previous.unwrap())&&self.head.replace(id).is_some(){
                        return Err(refuse("select one connected boundary chain"));
                    }
                    self.cursor+=1;
                }else if self.head.is_none(){return Err(refuse("select an open boundary chain"));}
                else{self.next();}
            },
            4=>{
                let id=self.head.unwrap();
                if self.chain.len()==self.input.len(){return Err(refuse("selected boundary chain does not terminate"));}
                let rim=rim_edge(&self.snapshot.source,id)?;
                self.chain.push(id);
                let following=self.out.get(&rim.vertices[1]).copied().flatten().ok_or_else(||refuse("boundary join is absent"))?;
                if self.selected.contains(&following){self.head=Some(following);}
                else{
                    if self.chain.len()!=self.input.len(){return Err(refuse("select one connected boundary chain"));}
                    let first=rim_edge(&self.snapshot.source,self.chain[0])?;
                    let [start,end]=first.vertices.map(|vertex|self.point(vertex));
                    self.sweep=Some(MeshBoundarySweep::new(start,end,self.snapshot.soup.normals[first.face as usize].0.map(f64::from),self.offset)?);
                    self.next();
                }
            },
            5=>{
                if let Some(point)=self.snapshot.soup.positions.get(self.cursor){
                    let sweep=self.sweep.as_ref().unwrap();
                    if sweep.height(point.map(f64::from))>sweep.epsilon{return Err(refuse("boundary offset runs into the source mesh"));}
                    self.cursor+=1;
                }else{self.next();}
            },
            6=>{
                let i=self.cursor;
                if i>self.chain.len(){self.next();}
                else{
                    let last=i==self.chain.len();
                    let rim=rim_edge(&self.snapshot.source,self.chain[i-usize::from(last)])?;
                    let id=rim.vertices[usize::from(last)];
                    let point=self.sweep.as_ref().unwrap().translate(self.point(id))?;
                    self.snapshot.soup.positions.push(point);self.points.push(id);self.cursor+=1;
                }
            },
            7=>{
                if self.cursor+1>=self.chain.len(){self.next();}
                else if self.pair==self.chain.len(){self.cursor+=1;self.pair=self.cursor+1;}
                else{
                    let i=self.cursor;let j=self.pair;
                    let points=[i,i+1,j,j+1].map(|index|self.point(self.points[index]));
                    self.sweep.as_ref().unwrap().certify_pair(points,j==i+1)?;self.pair+=1;
                }
            },
            8=>{
                if self.faces.len()<self.snapshot.source.face_count(){self.faces.push(self.faces.len() as u32);}
                else if self.corners.len()<self.snapshot.source.halfedge_count(){self.corners.push(None);self.edges.push(None);}
                else{self.next();}
            },
            9=>{
                if self.cursor==self.chain.len(){
                    self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.next();
                }else{self.append_wall()?;}
            },
            10=>if let Some(mesh)=self.build.as_mut().unwrap().advance()?{
                let mut attributes=SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.faces)),None,None);
                let data=attributes.configuration_mut()?;
                data.corner_sources=Some(std::mem::take(&mut self.corners));data.edge_sources=Some(std::mem::take(&mut self.edges));
                data.generated_vertices=Some(Vec::new());
                self.attributes=Some(attributes);self.next();
            },
            11=>{
                let origins=self.build.as_ref().unwrap().origin_vertices();
                if let Some(&vertex)=origins.get(self.cursor){
                    let base=self.snapshot.source.vertex_count();
                    let source=if (vertex as usize)<base{vertex}else{self.points[vertex as usize-base]};
                    self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(SourceVertex::Original(source));
                    self.cursor+=1;
                }else{
                    self.attributes.as_mut().unwrap().retain_bridged_surface(self.snapshot.source.face_count());
                    self.next();
                }
            },
            12=>{
                let attributes=self.attributes.as_mut().unwrap();
                if attributes.retaining_refined_surface(){attributes.step_refined_surface(&self.snapshot.source,grant,receipt)?;}
                else{self.next();}
            },
            13=>if let Some(mesh)=self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source)?{
                self.next();
                return Ok(self.chart.accept(mesh));
            },
            14=>return self.chart.advance(self.snapshot.source.face_count()),
            _=>unreachable!(),
        }
        Ok(None)
    }
}
