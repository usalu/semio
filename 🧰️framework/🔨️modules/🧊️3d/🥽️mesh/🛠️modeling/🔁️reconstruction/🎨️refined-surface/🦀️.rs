//! 🎨️ Retains source normals, face shading and corner UVs through polygon refinement.

use super::*;
use semio_framework_value::{RetirementDemand,ValueError};
use std::mem::size_of;

#[path="🧢️cap-chart/🦀️.rs"]
mod cap_chart;
pub(super) use cap_chart::CapChart;

pub(super) struct SurfaceBlend {
    start:u32,
    next:u32,
    complete:bool,
    missing_normal:bool,
    sum:[f64;3],
    weight:f64,
}

impl SurfaceBlend {
    fn new(start:u32)->Self {Self {start,next:start,complete:false,missing_normal:false,sum:[0.0;3],weight:0.0}}

    fn add<const N:usize>(&mut self,values:[f32;N],weight:f64) {
        for (slot,value) in self.sum.iter_mut().zip(values) {*slot+=f64::from(value)*weight;}
        self.weight+=weight;
    }

    fn normal(&self)->Option<[f32;3]> {
        let [x,y,z]=self.sum;
        let length=x.hypot(y).hypot(z);
        (!self.missing_normal&&length>0.0&&length.is_finite()).then(||self.sum.map(|value|(value/length) as f32))
    }

    fn uv(&self)->[f32;2] {[0,1].map(|axis|(self.sum[axis]/self.weight) as f32)}
}

impl MeshSourceRemap {
    /// 🧭️ Starts a surface transfer once refined vertices and faces have source identities.
    pub fn retain_refined_surface(&mut self) {
        debug_assert_eq!(self.phase,0);
        self.phase=10;
    }

    /// 🎈️ Projects original faces and generates separate UV charts on new quad walls.
    pub fn retain_expanded_surface(&mut self,first_generated_face:usize,inset:bool) {
        self.retain_refined_surface();
        self.expanded_surface=Some((first_generated_face,inset));
    }

    /// 🌉️ Charts the bridge walls while preserving both rim seam directions.
    pub fn retain_bridged_surface(&mut self,first_wall:usize) {
        self.retain_expanded_surface(first_wall,false);
        self.partitioned_surface=true;
    }

    /// 🪡️ Projects split corners and retains each surviving segment of an authored seam.
    pub fn retain_partitioned_surface(&mut self) {
        self.retain_refined_surface();
        self.partitioned_surface=true;
    }

    /// 🧢️ Preserves original corner data and charts the newly capped faces.
    pub fn retain_filled_surface(&mut self,first_cap:usize) {
        self.retain_partitioned_surface();
        self.cap_begin=Some(first_cap);
    }

    /// 🧭️ Reuses unchanged point and face identities after committing new winding.
    pub fn retain_reoriented_surface(&mut self,reversed_faces:Option<Vec<bool>>) {
        self.retain_partitioned_surface();
        self.reoriented_surface=true;
        self.data.mirrored_faces=reversed_faces;
    }

    /// 🧲️ Projects exact source samples with an existing provenance index.
    pub(super) fn retain_exact_surface(&mut self,corners:HistoryFoldIndex<(u32,u32),u32>) {
        debug_assert_eq!(self.phase,0);
        self.corners=corners;
        self.exact_surface=true;
        self.phase=11;
    }

    /// 🪪️ Reports whether the raw surface is still advancing separately from generic attributes.
    pub fn retaining_refined_surface(&self)->bool {matches!(self.phase,10..=14)}

    fn surface_vertex(&self,vertex:u32)->Option<SourceVertex> {
        if let Some(vertices)=&self.data.generated_vertices {return vertices.get(vertex as usize).copied();}
        if self.reoriented_surface||self.cap_begin.is_some() {return Some(SourceVertex::Original(vertex));}
        self.data.vertices.as_ref()?.get(vertex as usize).copied().map(SourceVertex::Original)
    }

    fn surface_face(&self,face:u32)->Option<u32> {
        if let Some(faces)=&self.data.faces {return faces.get(face as usize).copied();}
        if self.reoriented_surface {return Some(face);}
        self.data.generated_faces.as_ref()?.get(face as usize)?.first().copied()
    }

    fn surface_corner(&self,corner:usize)->Option<(u32,u32,SourceVertex)> {
        let edge=self.data.mesh.halfedges.get(corner)?;
        Some((self.surface_face(edge.face?)?,edge.vertex,self.surface_vertex(edge.vertex)?))
    }

    fn blend_complete(&self)->bool {self.surface_blend.as_ref().is_some_and(|blend|blend.complete)}

    fn step_face_blend(&mut self,source:&HalfedgeMesh,face:u32,normals:bool)->MeshResult<Option<SurfaceBlend>> {
        if self.blend_complete() {return Ok(self.surface_blend.take());}
        let start=source.faces.get(face as usize).ok_or(MeshKernelError::InvalidHandle)?.halfedge;
        let blend=self.surface_blend.get_or_insert_with(||SurfaceBlend::new(start));
        let edge=source.halfedges.get(blend.next as usize).ok_or(MeshKernelError::InvalidHandle)?;
        if normals {
            if let Some(normal)=source.vertices[edge.vertex as usize].normal {blend.add(normal,1.0);}
            else {blend.missing_normal=true;}
        }else {blend.add(edge.uv,1.0);}
        blend.next=edge.next;
        blend.complete=blend.next==blend.start;
        Ok(None)
    }

    fn weighted_normal(source:&HalfedgeMesh,weights:impl IntoIterator<Item=(u32,f64)>)->Option<[f32;3]> {
        let mut blend=SurfaceBlend::new(0);
        for (vertex,weight) in weights.into_iter().filter(|(_,weight)|*weight>0.0) {
            if let Some(normal)=source.vertices[vertex as usize].normal {blend.add(normal,weight);}
            else {blend.missing_normal=true;}
        }
        blend.normal()
    }

    fn normal_sample(&mut self,source:&HalfedgeMesh,original:SourceVertex)->MeshResult<Option<Option<[f32;3]>>> {
        let normal=match original {
            SourceVertex::Original(vertex)=>source.vertices[vertex as usize].normal,
            SourceVertex::Contributors(group)=>{
                let representative=self.data.vertices.as_ref().and_then(|vertices|vertices.get(self.cursor));
                let vertex=*representative.or_else(||self.data.vertex_contributors.as_ref()?.get(group as usize)?.first())
                    .ok_or(MeshKernelError::InvalidHandle)?;
                source.vertices[vertex as usize].normal
            },
            SourceVertex::Face(face)=>return Ok(self.step_face_blend(source,face,true)?.map(|blend|blend.normal())),
            SourceVertex::Weighted{vertices,weights}=>Self::weighted_normal(source,vertices.into_iter().zip(weights)),
            SourceVertex::Edge{a,b,t}=>Self::weighted_normal(source,[(a,1.0-t),(b,t)]),
            _=>return Err(MeshKernelError::InvalidInput("unsupported refined normal provenance".into())),
        };
        Ok(Some(normal))
    }

    fn corner_uv(&self,source:&HalfedgeMesh,face:u32,vertex:u32)->MeshResult<[f32;2]> {
        let original=*self.corners.get(&(face,vertex)).ok_or(MeshKernelError::InvalidHandle)?;
        Ok(source.halfedges[original as usize].uv)
    }

    fn exact_corner(&self,corner:usize)->Option<u32> {
        self.data.corner_sources.as_ref()?.get(corner).copied().flatten()
    }

    fn center_uv(&self,face:u32,vertex:u32)->Option<[f32;2]> {
        self.surface_center_uv.filter(|(origin,output,_)|*origin==face&&*output==vertex).map(|(_,_,uv)|uv)
    }

    fn uv_sample(&mut self,source:&HalfedgeMesh,face:u32,vertex:u32,original:SourceVertex)->MeshResult<Option<[f32;2]>> {
        let uv=match original {
            SourceVertex::Original(point)=>self.corner_uv(source,face,point)?,
            SourceVertex::Face(origin)=>{
                if let Some(uv)=self.center_uv(face,vertex) {return Ok(Some(uv));}
                let Some(blend)=self.step_face_blend(source,origin,false)? else {return Ok(None);};
                let uv=blend.uv();
                self.surface_center_uv=Some((face,vertex,uv));
                uv
            },
            SourceVertex::Edge{a,b,t}=>{
                let mut blend=SurfaceBlend::new(0);
                for (point,weight) in [(a,1.0-t),(b,t)] {
                    if weight<=0.0 {continue;}
                    if let Some(&corner)=self.corners.get(&(face,point)) {
                        blend.add(source.halfedges[corner as usize].uv,weight);
                    }
                }
                if blend.weight==0.0 {return Err(MeshKernelError::InvalidHandle);}
                blend.uv()
            },
            SourceVertex::Weighted {vertices,weights}=>{
                let mut blend=SurfaceBlend::new(0);
                for (point,weight) in vertices.into_iter().zip(weights).filter(|(_,weight)|*weight>0.0) {
                    blend.add(self.corner_uv(source,face,point)?,weight);
                }
                blend.uv()
            },
            _=>return Err(MeshKernelError::InvalidInput("unsupported refined UV provenance".into())),
        };
        Ok(Some(uv))
    }

    fn reflected_normal(&self,source:&HalfedgeMesh,vertex:usize,mut normal:[f32;3],axis:usize)->Option<[f32;3]> {
        let position=self.data.mesh.vertices[vertex].position[axis];
        let original=self.data.vertices.as_ref()?[vertex] as usize;
        if position==0.0 {
            if normal[axis]==0.0 {return Some(normal);}
            normal[axis]=0.0;
            let length=normal.map(f64::from).into_iter().fold(0.0,f64::hypot);
            return (length>0.0).then(||normal.map(|value|(f64::from(value)/length) as f32));
        }
        if position.is_sign_negative()!=source.vertices[original].position[axis].is_sign_negative() {normal[axis]=-normal[axis];}
        Some(normal)
    }

    fn is_expanded_face(&self,face:usize)->bool {
        self.expanded_surface.is_some_and(|(first,_)|face>=first)
    }

    fn is_cap(&self,face:usize)->bool {self.cap_begin.is_some_and(|first|face>=first)}

    fn cap_ready(&self,face:usize)->bool {
        self.cap_chart.as_ref().is_some_and(|(owner,chart)|*owner==face&&chart.ready())
    }

    fn step_cap_uv(&mut self,face:usize,corner:usize)->Option<[f32;2]> {
        let mesh=&self.data.mesh;
        let position=|id:usize|mesh.vertices[mesh.halfedges[id].vertex as usize].position;
        if self.cap_chart.as_ref().is_none_or(|(owner,_)|*owner!=face) {
            let start=mesh.faces[face].halfedge;
            self.cap_chart=Some((face,CapChart::new(start,position(start as usize))));
        }
        let chart=&mut self.cap_chart.as_mut().unwrap().1;
        if chart.ready() {return Some(chart.uv(position(corner)));}
        let current=chart.next() as usize;
        let next=mesh.halfedges[current].next;
        chart.observe(position(current),position(next as usize),next);
        None
    }

    fn expanded_uv(&self,corner:usize,face:usize)->MeshResult<[f32;2]> {
        let mut edge=self.data.mesh.faces[face].halfedge as usize;
        for uv in [[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]] {
            if edge==corner {return Ok(uv);}
            edge=self.data.mesh.halfedges[edge].next as usize;
        }
        Err(MeshKernelError::InvalidHandle)
    }

    fn source_edge(ends:[SourceVertex;2])->Option<(u32,u32)> {
        use SourceVertex::{Edge,Original};
        match ends {
            [Original(a),Original(b)] if a!=b=>Some((a,b)),
            [Edge{a,b,..},Original(point)] | [Original(point),Edge{a,b,..}] if a==point||b==point=>Some((a,b)),
            [Edge{a,b,..},Edge{a:c,b:d,..}] if (a,b)==(c,d)||(a,b)==(d,c)=>Some((a,b)),
            _=>None,
        }
    }

    fn mapped_seam_origin(&self,source:&HalfedgeMesh,corner:u32,depth:&mut usize)->Result<Option<u32>,ValueError> {
        let edge=&self.data.mesh.halfedges[corner as usize];
        let face=edge.face.ok_or_else(||ValueError::invariant("surface corner has no face"))? as usize;
        let reversed=self.data.mirrored_faces.as_ref().is_some_and(|faces|faces[face]);
        let source_face=self.surface_face(face as u32).ok_or_else(||ValueError::invariant("surface face lost its original"))?;
        let mut ends=[edge.vertex,self.data.mesh.halfedges[edge.next as usize].vertex];
        if reversed {ends.swap(0,1);}
        let [Some(first),Some(last)]=ends.map(|vertex|self.surface_vertex(vertex))
            else {return Err(ValueError::invariant("surface edge lost its original"));};
        let Some((a,b))=Self::source_edge([first,last]) else {return Ok(None);};
        let directions=[Some((a,b)),self.partitioned_surface.then_some((b,a))];
        for (start,end) in directions.into_iter().flatten() {
            let key=(source_face,start);
            *depth=(*depth).max(self.corners.next_contains_depth_demand(&key)?);
            if let Some(&original)=self.corners.get(&key) {
                if source.halfedges[source.halfedges[original as usize].next as usize].vertex==end {return Ok(Some(original));}
            }
        }
        Ok(None)
    }

    fn refined_seam(&self,source:&HalfedgeMesh,corner:usize)->Result<(bool,usize),ValueError> {
        let mut depth=1;
        let mesh=&self.data.mesh;
        let edge=&mesh.halfedges[corner];
        let outgoing=[Some(corner as u32),edge.twin];
        let mut marked=(self.expanded_surface.is_some()||self.cap_begin.is_some())&&edge.twin.is_some_and(|twin|{
            let other=&mesh.halfedges[twin as usize];
            let generated=|face:Option<u32>|face.is_some_and(|face|self.is_expanded_face(face as usize)||self.is_cap(face as usize));
            (generated(edge.face)||generated(other.face))&&
                (edge.uv!=mesh.halfedges[other.next as usize].uv||other.uv!=mesh.halfedges[edge.next as usize].uv)
        });
        for id in outgoing.into_iter().flatten() {
            let cap_source=self.cap_begin.and_then(|_|self.data.edge_sources.as_ref()).and_then(|sources|sources[id as usize]);
            let origin=if cap_source.is_some() {cap_source}
                else if self.data.normal_axis.is_some()||self.expanded_surface.is_some()||self.partitioned_surface {self.mapped_seam_origin(source,id,&mut depth)?}
                else {self.data.edge_sources.as_ref().ok_or_else(||ValueError::invariant("weld edge provenance is absent"))?[id as usize]};
            let Some(original)=origin else {continue;};
            let incoming=[Some(original),source.halfedges[original as usize].twin];
            for edge in incoming.into_iter().flatten() {
                depth=depth.max(source.uv_seams.next_contains_depth_demand(&edge)?);
                marked|=source.uv_seams.contains(&edge);
            }
        }
        Ok((marked,depth))
    }

    /// 📏️ Quotes one original backing or surface row before the refined mesh changes.
    pub fn refined_surface_demand(&self,source:&HalfedgeMesh,copy:usize)->Result<Option<RetirementDemand>,ValueError> {
        let mut demand=RetirementDemand {depth:1,..Default::default()};
        match self.phase {
            10=>if let Some(edge)=source.halfedges.get(self.cursor) {
                let key=(edge.face.ok_or_else(||ValueError::invariant("refinement source corner has no face"))?,edge.vertex);
                demand.capacity_bytes=self.corners.next_insert_capacity_byte_demand(&key,copy)?;
                demand.depth=self.corners.next_insert_depth_demand(&key)?;
            },
            11=>if self.cursor<self.data.mesh.vertex_count() {
                let original=self.surface_vertex(self.cursor as u32).ok_or_else(||ValueError::invariant("refined vertex has no source provenance"))?;
                if !matches!(original,SourceVertex::Face(_)) || self.blend_complete() {demand.copy_bytes=size_of::<Option<[f32;3]>>();}
            },
            12=>if self.cursor<self.data.mesh.face_count() {demand.copy_bytes=size_of::<bool>();},
            13 if self.data.mesh.halfedges.get(self.cursor).and_then(|edge|edge.face).is_some_and(|face|self.is_cap(face as usize))=>{
                let face=self.data.mesh.halfedges[self.cursor].face.unwrap() as usize;
                if self.cap_ready(face) {demand.copy_bytes=size_of::<[f32;2]>();}
            },
            13 if self.exact_corner(self.cursor).is_some() || self.data.mesh.halfedges.get(self.cursor).and_then(|edge|edge.face).is_some_and(|face|self.is_expanded_face(face as usize))=>{
                demand.copy_bytes=size_of::<[f32;2]>();
            },
            13=>if self.cursor<self.data.mesh.halfedge_count() {
                let (face,vertex,original)=self.surface_corner(self.cursor).ok_or_else(||ValueError::invariant("refined corner has no source provenance"))?;
                if !matches!(original,SourceVertex::Face(_)) || self.blend_complete() || self.center_uv(face,vertex).is_some() {
                    demand.copy_bytes=size_of::<[f32;2]>();
                }
                match original {
                    SourceVertex::Original(point)=>demand.depth=self.corners.next_contains_depth_demand(&(face,point))?.max(1),
                    SourceVertex::Edge{a,b,t}=>for (point,weight) in [(a,1.0-t),(b,t)] {
                        if weight>0.0 {demand.depth=demand.depth.max(self.corners.next_contains_depth_demand(&(face,point))?);}
                    },
                    SourceVertex::Weighted{vertices,weights}=>for (point,weight) in vertices.into_iter().zip(weights) {
                        if weight>0.0 {demand.depth=demand.depth.max(self.corners.next_contains_depth_demand(&(face,point))?);}
                    },
                    _=>{},
                }
            },
            14 if self.cursor<self.data.mesh.halfedge_count()=>{
                let (marked,depth)=self.refined_seam(source,self.cursor)?;
                demand.depth=depth;
                if marked {
                    let id=self.cursor as u32;
                    let seams=&self.data.mesh.uv_seams;
                    demand.copy_bytes=seams.next_insert_copy_byte_demand(&id)?;
                    demand.capacity_bytes=seams.next_insert_capacity_byte_demand(&id,copy)?;
                    demand.release_bytes=seams.next_insert_release_byte_demand(&id)?;
                    demand.depth=demand.depth.max(seams.next_insert_depth_demand(&id)?);
                }
            },
            14=>{},
            _=>return Ok(None),
        }
        Ok(Some(demand))
    }

    /// 🧵️ Moves one admitted source index row or writes one scalar surface record.
    pub fn step_refined_surface(&mut self,source:&HalfedgeMesh,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<()> {
        let demand=self.refined_surface_demand(source,grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)?
            .ok_or_else(||MeshKernelError::InvalidInput("refined surface transfer is already complete".into()))?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand) {return Ok(());}
        match self.phase {
            10=>{
                if let Some(edge)=source.halfedges.get(self.cursor) {
                    let key=(edge.face.ok_or(MeshKernelError::InvalidHandle)?,edge.vertex);
                    if demand.capacity_bytes!=0 {
                        match self.corners.reserve_insert_step(&key,grant) {
                            Ok(progress)=>*receipt=progress,
                            Err((error,progress))=>{*receipt=progress;return Err(MeshKernelError::Retained(error.with_retained_progress(progress)));}
                        }
                    }else {
                        let (old,progress)=self.corners.insert_reserved(key,self.cursor as u32,grant).map_err(|(error,_,_)|MeshKernelError::Retained(error))?;
                        *receipt=progress;
                        if old.is_some() {return Err(MeshKernelError::NonManifold);}
                        self.cursor+=1;
                    }
                    return Ok(());
                }
            },
            11 if self.cursor<self.data.mesh.vertex_count()=>{
                let original=self.surface_vertex(self.cursor as u32).ok_or(MeshKernelError::InvalidHandle)?;
                if let Some(normal)=self.normal_sample(source,original)? {
                    let normal=match self.data.normal_axis {
                        Some(axis)=>normal.and_then(|normal|self.reflected_normal(source,self.cursor,normal,axis)),
                        None=>normal,
                    };
                    self.data.mesh.vertices[self.cursor].normal=normal;
                    self.cursor+=1;
                }
                *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
                return Ok(());
            },
            12 if self.cursor<self.data.mesh.face_count()=>{
                let original=self.surface_face(self.cursor as u32).ok_or(MeshKernelError::InvalidHandle)?;
                let flat_wall=self.is_expanded_face(self.cursor)&&self.expanded_surface.is_some_and(|(_,inset)|!inset);
                self.data.mesh.faces[self.cursor].smooth=!flat_wall&&!self.is_cap(self.cursor)&&source.faces[original as usize].smooth;
                self.cursor+=1;
                *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
                return Ok(());
            },
            13 if self.cursor<self.data.mesh.halfedge_count()=>{
                let face=self.data.mesh.halfedges[self.cursor].face.ok_or(MeshKernelError::InvalidHandle)? as usize;
                let sample=if self.is_cap(face) {self.step_cap_uv(face,self.cursor)}else if self.is_expanded_face(face) {
                    Some(self.expanded_uv(self.cursor,face)?)
                }else if let Some(corner)=self.exact_corner(self.cursor) {
                    Some(source.halfedges.get(corner as usize).ok_or(MeshKernelError::InvalidHandle)?.uv)
                }else {
                    let (face,vertex,original)=self.surface_corner(self.cursor).ok_or(MeshKernelError::InvalidHandle)?;
                    self.uv_sample(source,face,vertex,original)?
                };
                if let Some(uv)=sample {
                    self.data.mesh.halfedges[self.cursor].uv=uv;
                    self.cursor+=1;
                }
                *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
                return Ok(());
            },
            14 if self.cursor<self.data.mesh.halfedge_count()=>{
                let marked=self.refined_seam(source,self.cursor).map_err(MeshKernelError::Retained)?.0;
                if marked {
                    let id=self.cursor as u32;
                    let seams=&mut self.data.mesh.uv_seams;
                    if demand.capacity_bytes!=0 {
                        match seams.reserve_insert_step(&id,grant) {
                            Ok(progress)=>*receipt=progress,
                            Err((error,progress))=>{*receipt=progress;return Err(MeshKernelError::Retained(error.with_retained_progress(progress)));}
                        }
                        return Ok(());
                    }
                    let (_,progress)=seams.insert_reserved(id,grant).map_err(|(error,_)|MeshKernelError::Retained(error))?;
                    *receipt=progress;
                }else {
                    *receipt=RetainedCloneProgress {copied_items:1,..Default::default()};
                }
                self.cursor+=1;
                return Ok(());
            },
            _=>{},
        }
        self.phase=match self.phase {
            13 if self.cap_begin.is_some()||self.expanded_surface.is_some()||((self.exact_surface||self.partitioned_surface||self.data.normal_axis.is_some())&&!source.uv_seams.is_empty())=>14,
            13|14=>1,
            _=>self.phase+1,
        };
        self.cursor=0;
        *receipt=RetainedCloneProgress {copied_items:1,..Default::default()};
        Ok(())
    }
}
