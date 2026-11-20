//! 📐️ Budgeted exact tessellation of planar line-bounded topology.

use super::*;
use semio_framework_3d_brep_spline_trim::{ContourBoundary, ContourSpline};
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_mesh_engine::{ComponentReferenceTable,MeshData};
use semio_framework_3d_brep_vector::{Pnt2,predicates::{Orient,orient2d}};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress as Progress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

const MAXIMUM_CORNERS:usize=200_000;
const MAXIMUM_TRIANGULATION_EVENTS:usize=8_000_000;
const DOMAINS:[&str;3]=["face","edge","vertex"];

#[path="🌉️bridges/🦀️.rs"]
mod bridges;
use bridges::PlanarBridges;

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum PlanarPreviewRoot {Face(FaceId),Shell(ShellId),Solid(SolidId),Compound(u64)}
semio_framework_value::artifact_retire_leaf!(PlanarPreviewRoot);

#[derive(Clone,Copy,Default)]
struct FaceWalk {solid:usize,shell:usize,face:usize}
semio_framework_value::artifact_retire_struct!(FaceWalk {solid,shell,face});

enum Visit {Pending,Face(FaceId),End}

impl FaceWalk {
    fn next(&mut self,body:&Body,root:PlanarPreviewRoot,members:Option<&[SolidId]>) -> Result<Visit,ValueError> {
        let shell=match root {
            PlanarPreviewRoot::Face(id)=>{
                if self.face!=0 {return Ok(Visit::End);}
                self.face=1;
                return Ok(Visit::Face(id));
            }
            PlanarPreviewRoot::Shell(id)=>id,
            PlanarPreviewRoot::Solid(_)|PlanarPreviewRoot::Compound(_)=>{
                let Some(id)=self.solid(root,members)? else{return Ok(Visit::End);};
                let row=body.solids.get(id).ok_or_else(invalid)?;
                let id=if self.shell==0 {Some(row.outer)} else {row.inners.get(self.shell-1).copied()};
                let Some(id)=id else{
                    if matches!(root,PlanarPreviewRoot::Compound(_)){self.solid+=1;self.shell=0;self.face=0;return Ok(Visit::Pending);}
                    return Ok(Visit::End);
                };
                id
            }
        };
        if let Some(&id)=body.shells.get(shell).ok_or_else(invalid)?.faces.get(self.face){
            self.face+=1;
            return Ok(Visit::Face(id));
        }
        if matches!(root,PlanarPreviewRoot::Solid(_)|PlanarPreviewRoot::Compound(_)){
            self.shell+=1;
            self.face=0;
            Ok(Visit::Pending)
        }else{Ok(Visit::End)}
    }

    fn solid(&self,root:PlanarPreviewRoot,members:Option<&[SolidId]>)->Result<Option<SolidId>,ValueError>{
        match root{
            PlanarPreviewRoot::Solid(id)=>Ok(Some(id)),
            PlanarPreviewRoot::Compound(_)=>Ok(members.ok_or_else(invalid)?.get(self.solid).copied()),
            _=>Ok(None),
        }
    }
}

#[derive(Clone,Copy)]
struct Ring {id:LoopId,first:CoedgeId,next:CoedgeId,visits:usize,corners:usize,segment:usize,tolerance:f64,spline:Option<ContourSpline>}
semio_framework_value::artifact_retire_leaf!(Ring);

#[derive(Clone,Copy)]
struct FaceSource { id:FaceId, loops:usize, corners:usize }
semio_framework_value::artifact_retire_leaf!(FaceSource);

#[derive(Clone,Copy)]
struct Region { start:usize, end:usize, ccw:bool, rightmost:usize, joined:bool }
semio_framework_value::artifact_retire_leaf!(Region);

impl Region {
    fn next(self,i:usize)->usize { if i+1==self.end{self.start}else{i+1} }
    fn previous(self,i:usize)->usize { if i==self.start{self.end-1}else{i-1} }
}

#[derive(Clone,Copy)]
struct Corner {point:[f32;3],uv:[f64;2],previous:usize,next:usize,vertex:Option<u64>,edge:u64,region:usize,source:usize,edge_end:bool}
semio_framework_value::artifact_retire_leaf!(Corner);

#[derive(Clone,Copy)]
enum PlanarTurn {Counting,Reserve(usize),Reading,Checking,Writing,Bridging,Clipping}
semio_framework_value::artifact_retire_leaf!(PlanarTurn);

/// 🧾️ One mesh event or an inapplicable preflight before any preview backing was allocated.
pub(super) enum PlanarPreviewStep {Pending,Unsupported,Ready(MeshData)}

pub(super) struct PlanarPreview {
    root:PlanarPreviewRoot,
    revision:u64,
    deflection:f64,
    walk:FaceWalk,
    ring:Option<Ring>,
    face_source:Option<FaceSource>,
    bridge:PlanarBridges,
    regions:Vec<Region>,
    ring_start:usize,
    rightmost:usize,
    stage:PlanarTurn,
    total_corners:usize,
    total_faces:usize,
    total_holes:usize,
    max_regions:usize,
    max_corners:usize,
    corners:Vec<Corner>,
    mesh:MeshData,
    face:u64,
    label_indices:HistoryFoldIndex<u64,(u32,bool)>,
    references:Vec<(String,Vec<String>)>,
    triangulation_events:usize,
    pair:[usize;2],
    normal:Vec3,
    flipped:bool,
    origin:Pnt3,
    u:Vec3,
    v:Vec3,
    twice_area:f64,
    base:u32,
    cursor:usize,
    candidates:usize,
    remaining:usize,
    probe:Option<usize>,
}

semio_framework_value::artifact_retire_struct!(PlanarPreview {root,revision,deflection,walk,ring,face_source,bridge,regions,ring_start,rightmost,stage,total_corners,total_faces,total_holes,max_regions,max_corners,corners,mesh,face,label_indices,references,triangulation_events,pair,normal,flipped,origin,u,v,twice_area,base,cursor,candidates,remaining,probe});

fn invalid()->ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue,"planar preview requires simple closed finite planar rings")
}

fn turn(a:[f64;2],b:[f64;2],c:[f64;2])->Orient{
    let [a,b,c]=[a,b,c].map(|[u,v]|Pnt2::new(u,v));
    orient2d(a,b,c)
}

fn overlaps(a:[f64;2],b:[f64;2],c:[f64;2],d:[f64;2])->bool {
    let intersects_axis=|axis:usize|a[axis].min(b[axis])<=c[axis].max(d[axis])&&c[axis].min(d[axis])<=a[axis].max(b[axis]);
    if !intersects_axis(0)||!intersects_axis(1){return false;}
    let separates=|first,second|first==second&&first!=Orient::Zero;
    !separates(turn(a,b,c),turn(a,b,d))&&!separates(turn(c,d,a),turn(c,d,b))
}

fn segment_count(curve:&Curve3,range:(f64,f64),deflection:f64)->Result<Option<usize>,ValueError>{
    let radius=match curve{
        Curve3::Line{..}=>return Ok(Some(1)),
        Curve3::Circle{radius,..}=>*radius,
        Curve3::Ellipse{major_radius,minor_radius,..}=>major_radius.max(*minor_radius),
        Curve3::Nurbs{..}=>return Ok(None),
    };
    let sweep=range.1-range.0;
    if !radius.is_finite()||radius<=0.0||!sweep.is_finite()||sweep<=0.0{return Err(invalid());}
    let n=(sweep*(radius/(8.0*deflection)).sqrt()).ceil().max(3.0);
    if !n.is_finite()||n>MAXIMUM_CORNERS as f64{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"planar curve exceeds the segment limit"));}
    Ok(Some(n as usize))
}

fn reserve<T>(buffer:&mut Vec<T>,items:usize)->Result<usize,ValueError>{
    buffer.try_reserve_exact(items).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"planar preview storage allocation failed"))?;
    Ok(buffer.capacity()*size_of::<T>())
}

fn own_text(value:&str)->Result<String,ValueError>{
    let mut text=String::new();
    text.try_reserve_exact(value.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"planar preview label allocation failed"))?;
    text.push_str(value);
    Ok(text)
}

impl PlanarPreview {
    pub(super) fn new(root:PlanarPreviewRoot,revision:u64,deflection:f64)->Self {
        Self{root,revision,deflection,walk:Default::default(),ring:None,face_source:None,bridge:PlanarBridges::new(),regions:Vec::new(),ring_start:0,rightmost:0,stage:PlanarTurn::Counting,total_corners:0,total_faces:0,total_holes:0,max_regions:0,max_corners:0,
            corners:Vec::new(),mesh:Default::default(),face:0,label_indices:HistoryFoldIndex::new(),references:Vec::new(),triangulation_events:0,pair:[0,1],normal:Vec3::ZERO,flipped:false,origin:Pnt3::new(0.0,0.0,0.0),u:Vec3::ZERO,v:Vec3::ZERO,
            twice_area:0.0,base:0,cursor:0,candidates:0,remaining:0,probe:None}
    }

    fn begin_face(&mut self,body:&Body,face:FaceId)->Result<bool,ValueError>{
        let row=body.faces.get(face).ok_or_else(invalid)?;
        let Surface::Plane{frame}=body.surfaces.get(row.surface).ok_or_else(invalid)? else{return Ok(false)};
        if row.outer.is_none(){return Err(invalid());}
        self.normal=frame.x.cross(frame.y).normalized().ok_or_else(invalid)?;
        self.u=frame.x.normalized().ok_or_else(invalid)?;
        self.v=self.normal.cross(self.u);
        self.origin=frame.origin;
        self.flipped=row.flipped;
        self.face=row.label.0;
        self.face_source=Some(FaceSource{id:face,loops:0,corners:0});
        self.corners.clear();
        self.regions.clear();
        Ok(true)
    }

    fn next_ring(&mut self,body:&Body)->Result<bool,ValueError>{
        let source=self.face_source.as_mut().ok_or_else(invalid)?;
        let face=body.faces.get(source.id).ok_or_else(invalid)?;
        let id=if source.loops==0{face.outer}else{face.inners.get(source.loops-1).copied()};
        let Some(id)=id else{return Ok(false);};
        let ring=body.loops.get(id).filter(|ring|ring.face==source.id).ok_or_else(invalid)?;
        source.loops+=1;
        self.ring_start=self.corners.len();
        self.rightmost=self.ring_start;
        self.twice_area=0.0;
        self.ring=Some(Ring{id,first:ring.first,next:ring.first,visits:0,corners:0,segment:0,tolerance:face.tol.value(),spline:None});
        Ok(true)
    }

    fn triangles(&self)->usize{self.total_corners+2*self.total_holes-2*self.total_faces}

    pub(super) fn is_meshing(&self)->bool { !matches!(self.stage,PlanarTurn::Counting) }

    pub(super) fn demand(&self,body:&Body,members:Option<&[SolidId]>)->Result<RetirementDemand,ValueError>{
        let depths=|label|self.label_indices.next_contains_depth_demand(&label).expect("bounded planar reference depth");
        let mut depth=self.source_depth(body,members)?;
        let (copy_bytes,capacity_bytes)=match self.stage {
            PlanarTurn::Reserve(0)=>(0,self.max_corners*size_of::<Corner>()),
            PlanarTurn::Reserve(1|2)=>(0,self.total_corners*3*size_of::<f32>()),
            PlanarTurn::Reserve(3)=>(0,self.triangles()*3*size_of::<u32>()),
            PlanarTurn::Reserve(4)=>(0,self.total_corners*6*size_of::<f32>()),
            PlanarTurn::Reserve(5..=6)=>(0,self.total_corners*size_of::<u32>()),
            PlanarTurn::Reserve(7)=>(0,self.triangles()*size_of::<u32>()),
            PlanarTurn::Reserve(8)=>(0,DOMAINS.len()*size_of::<(String,Vec<String>)>()),
            PlanarTurn::Reserve(9..=11)=>{let n=DOMAINS[self.references.len()].len();(n,n)},
            PlanarTurn::Reserve(12)=>(0,self.total_faces*size_of::<String>()),
            PlanarTurn::Reserve(13|14)=>(0,self.total_corners*size_of::<String>()),
            PlanarTurn::Reserve(_)=>(0,self.max_regions*size_of::<Region>()),
            PlanarTurn::Counting if self.ring.is_some_and(|ring|ring.spline.is_some()) => (4 * size_of::<Pnt3>() + size_of::<f64>(),0),
            PlanarTurn::Reading if self.ring.is_some()=>(size_of::<Corner>()+size_of::<Region>(),0),
            PlanarTurn::Bridging=>(self.bridge.copy_bytes(),0),
            PlanarTurn::Writing=>{
                if let Some((_,label))=self.missing_reference(){return Ok(self.reference_demand(label));}
                let corner=self.corners[self.cursor];
                depth=depths(corner.edge).max(corner.vertex.map_or(1,depths));
                (12*size_of::<f32>()+2*size_of::<usize>()+2*size_of::<u32>(),0)
            }
            PlanarTurn::Clipping=>{depth=depths(self.face);(4*size_of::<u32>()+2*size_of::<usize>(),0)},
            _=>(0,0),
        };
        Ok(RetirementDemand{copy_bytes,capacity_bytes,depth,..Default::default()})
    }

    fn source_depth(&self,body:&Body,members:Option<&[SolidId]>)->Result<usize,ValueError>{
        let mut depth=1;
        if !matches!(self.stage,PlanarTurn::Counting|PlanarTurn::Reading){return Ok(depth);}
        if let Some(ring)=self.ring{
            let coedge=body.coedges.get(ring.next).ok_or_else(invalid)?;
            let edge=body.edges.get(coedge.edge).ok_or_else(invalid)?;
            let next=body.coedges.get(coedge.next).ok_or_else(invalid)?;
            return Ok(depth.max(body.coedges.next_remove_depth_demand(ring.next)?)
                .max(body.coedges.next_remove_depth_demand(coedge.next)?)
                .max(body.edges.next_remove_depth_demand(coedge.edge)?)
                .max(body.edges.next_remove_depth_demand(next.edge)?)
                .max(body.curves3.next_remove_depth_demand(edge.curve)?)
                .max(body.vertices.next_remove_depth_demand(edge.v0)?)
                .max(body.vertices.next_remove_depth_demand(edge.v1)?));
        }
        if let Some(source)=self.face_source{
            let face=body.faces.get(source.id).ok_or_else(invalid)?;
            let ring=if source.loops==0{face.outer}else{face.inners.get(source.loops-1).copied()};
            depth=depth.max(body.faces.next_remove_depth_demand(source.id)?);
            if let Some(ring)=ring{depth=depth.max(body.loops.next_remove_depth_demand(ring)?);}
            return Ok(depth);
        }
        match self.root{
            PlanarPreviewRoot::Face(_)=>{}
            PlanarPreviewRoot::Shell(id)=>depth=depth.max(body.shells.next_remove_depth_demand(id)?),
            PlanarPreviewRoot::Solid(_)|PlanarPreviewRoot::Compound(_)=>{
                let Some(id)=self.walk.solid(self.root,members)? else{return Ok(depth);};
                depth=depth.max(body.solids.next_remove_depth_demand(id)?);
                let solid=body.solids.get(id).ok_or_else(invalid)?;
                let shell=if self.walk.shell==0{Some(solid.outer)}else{solid.inners.get(self.walk.shell-1).copied()};
                if let Some(shell)=shell{depth=depth.max(body.shells.next_remove_depth_demand(shell)?);}
            }
        }
        let mut walk=self.walk;
        if let Visit::Face(id)=walk.next(body,self.root,members)?{
            let face=body.faces.get(id).ok_or_else(invalid)?;
            depth=depth.max(body.faces.next_remove_depth_demand(id)?).max(body.surfaces.next_remove_depth_demand(face.surface)?);
        }
        Ok(depth)
    }

    fn missing_reference(&self)->Option<(usize,u64)> {
        let corner=self.corners[self.cursor];
        [Some(self.face),Some(corner.edge),corner.vertex].into_iter().enumerate()
            .filter_map(|(domain,label)|label.map(|id|(domain,id))).find(|&(_,label)|!self.label_indices.contains_key(&label))
    }

    fn reference_demand(&self,label:u64)->RetirementDemand{
        let depth=self.label_indices.next_insert_depth_demand(&label).expect("bounded planar reference depth");
        let backing=self.label_indices.next_insert_capacity_byte_demand(&label,0).expect("bounded planar reference capacity");
        let text=if backing==0 {PersistentLabel(label).decimal(&mut [0;20]).len()}else{0};
        RetirementDemand{depth,copy_bytes:text,capacity_bytes:backing.max(text),..Default::default()}
    }

    fn write_reference(&mut self,domain:usize,label:u64,grant:RetainedCloneGrant,demand:RetirementDemand)->Result<Progress,ValueError>{
        if demand.copy_bytes==0{
            return self.label_indices.reserve_insert_step(&label,grant).map_err(|(error,receipt)|error.with_retained_progress(receipt));
        }
        let labels=&mut self.references[domain].1;
        let id=labels.len() as u32;
        let text=own_text(PersistentLabel(label).decimal(&mut [0;20]))?;
        let progress=Progress{copied_items:1,copied_bytes:text.len(),retained_capacity_bytes:text.capacity(),..Default::default()};
        labels.push(text);
        if !progress.fits(grant) {return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"planar label reservation exceeded its grant").with_retained_progress(progress));}
        let (previous,_)=self.label_indices.insert_reserved(label,(id,false),grant).map_err(|(error,_,_)|error.with_retained_progress(progress))?;
        if previous.is_some(){return Err(ValueError::invariant("planar component label already exists").with_retained_progress(progress));}
        Ok(progress)
    }

    /// ⏱️ Advances one original boundary, one allocation, or one triangulation event.
    pub(super) fn advance(&mut self,body:&Body,root:PlanarPreviewRoot,members:Option<&[SolidId]>,revision:u64,grant:RetainedCloneGrant)
        ->Result<(PlanarPreviewStep,Progress),ValueError>{
        if self.root!=root || self.revision!=revision {return Err(ValueError::invariant("planar preview source changed"));}
        let demand=self.demand(body,members)?;
        if grant.maximum_items==0 || !retained_clone_grant_admits(grant,demand){return Ok((PlanarPreviewStep::Pending,Default::default()));}
        let mut receipt=Progress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match self.stage {
            PlanarTurn::Reserve(slot)=>{
                let n=self.total_corners;
                let triangles=self.triangles();
                receipt.retained_capacity_bytes=match slot{
                    0=>reserve(&mut self.corners,self.max_corners)?,
                    1=>reserve(&mut self.mesh.positions,n*3)?,
                    2=>reserve(&mut self.mesh.normals,n*3)?,
                    3=>reserve(&mut self.mesh.indices,triangles*3)?,
                    4=>reserve(&mut self.mesh.edge_positions,n*6)?,
                    5=>reserve(&mut self.mesh.vertex_ids,n)?,
                    6=>reserve(&mut self.mesh.edge_ids,n)?,
                    7=>reserve(&mut self.mesh.face_ids,triangles)?,
                    8=>reserve(&mut self.references,DOMAINS.len())?,
                    9..=11=>{
                        let key=own_text(DOMAINS[self.references.len()])?;
                        let cap=key.capacity();
                        self.references.push((key,Vec::new()));
                        cap
                    }
                    12..=14=>{let domain=slot-12;let count=if domain==0{self.total_faces}else{n};reserve(&mut self.references[domain].1,count)?}
                    _=>reserve(&mut self.regions,self.max_regions)?,
                };
                self.stage=if slot==15{PlanarTurn::Reading}else{PlanarTurn::Reserve(slot+1)};
            }
            PlanarTurn::Counting | PlanarTurn::Reading=>{
                let counting=matches!(self.stage,PlanarTurn::Counting);
                if let Some(ring)=self.ring.as_mut(){
                    let coedge=body.coedges.get(ring.next).filter(|co|co.loop_id==ring.id).ok_or_else(invalid)?;
                    let edge=body.edges.get(coedge.edge).ok_or_else(invalid)?;
                    let curve=body.curves3.get(edge.curve).ok_or_else(invalid)?;
                    let (start,end)=body.coedge_endpoints(ring.next).ok_or_else(invalid)?;
                    let start_row=body.vertices.get(start).ok_or_else(invalid)?;
                    let next=body.coedges.get(coedge.next).filter(|value|value.prev==ring.next).ok_or_else(invalid)?;
                    let next_start=body.coedge_endpoints(coedge.next).ok_or_else(invalid)?.0;
                    if end!=next_start||next.loop_id!=ring.id{return Err(invalid());}
                    let (added,edge_end,corner) = if let Some(segments)=segment_count(curve,edge.range,self.deflection)? {
                        if segments==1&&start==end{return Err(invalid());}
                        if counting { (segments,true,None) }
                        else {
                            let (point,vertex)=if ring.segment==0{(start_row.position,Some(start_row.label.0))}
                            else{
                                let i=if coedge.forward{ring.segment}else{segments-ring.segment};
                                let (lo,hi)=edge.range;
                                (curve.eval(lo+(hi-lo)*i as f64/segments as f64),None)
                            };
                            ring.segment+=1;
                            (1,ring.segment==segments,Some((point,vertex)))
                        }
                    } else {
                        let Some(spline)=ring.spline.as_mut() else {
                            ring.spline=ContourSpline::new(curve,edge.range,coedge.forward);
                            if ring.spline.is_some(){return Ok((PlanarPreviewStep::Pending,receipt));}
                            if counting{return Ok((PlanarPreviewStep::Unsupported,receipt));}
                            return Err(ValueError::invariant("planar spline trim changed after admission"));
                        };
                        if counting {
                            let end_row=body.vertices.get(end).ok_or_else(invalid)?;
                            let boundary=ContourBoundary{origin:self.origin,normal:self.normal,tolerance:ring.tolerance,start:start_row.position,end:end_row.position};
                            let Some(count)=spline.count(curve,self.deflection,boundary).map_err(|error|error.with_retained_progress(receipt))? else{return Ok((PlanarPreviewStep::Unsupported,receipt));};
                            (count.corners,count.edge_end,None)
                        } else {
                            let Some(corner)=spline.sample(curve,self.deflection).ok_or_else(||ValueError::invariant("planar spline trim changed after admission"))?
                                else{return Ok((PlanarPreviewStep::Pending,receipt));};
                            let (point,vertex)=if corner.edge_start{(start_row.position,Some(start_row.label.0))}else{(corner.point,None)};
                            (1,corner.edge_end,Some((point,vertex)))
                        }
                    };
                    if counting {
                        self.total_corners=self.total_corners.checked_add(added).filter(|n|*n<=MAXIMUM_CORNERS)
                            .ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"planar preview exceeds the corner limit"))?;
                    }
                    ring.corners+=added;
                    if let Some((p,vertex))=corner {
                        if self.corners.len()>=self.max_corners{return Err(ValueError::invariant("planar preview corner capacity changed"));}
                        let local=p-self.origin;
                        let uv=[local.dot(self.u),local.dot(self.v)];
                        let point=[p.x as f32,p.y as f32,p.z as f32];
                        if uv.iter().any(|v|!v.is_finite())||point.iter().any(|v|!v.is_finite())||
                            local.dot(self.normal).abs()>ring.tolerance{return Err(invalid());}
                        let source=self.corners.len();
                        if source>self.ring_start {
                            let prev=self.corners[source-1].uv;
                            self.twice_area+=prev[0]*uv[1]-prev[1]*uv[0];
                            if uv[0]>self.corners[self.rightmost].uv[0]{self.rightmost=source;}
                        }
                        self.corners.push(Corner{point,uv,previous:0,next:0,vertex,edge:edge.label.0,region:self.regions.len(),
                            source,edge_end});
                    }
                    if !edge_end {return Ok((PlanarPreviewStep::Pending,receipt));}
                    ring.segment=0;
                    ring.spline=None;
                    ring.visits+=1;
                    if ring.visits>body.coedges.slot_count(){return Err(invalid());}
                    ring.next=coedge.next;
                    if ring.next==ring.first {
                        let count=ring.corners;
                        if count<3 {return Err(invalid());}
                        self.ring=None;
                        let source=self.face_source.as_mut().ok_or_else(invalid)?;
                        source.corners+=count;
                        if !counting {
                            let a=self.corners[self.ring_start+count-1].uv;
                            let b=self.corners[self.ring_start].uv;
                            self.twice_area+=a[0]*b[1]-a[1]*b[0];
                            if !self.twice_area.is_finite()||self.twice_area==0.0{return Err(invalid());}
                            self.regions.push(Region{start:self.ring_start,end:self.corners.len(),ccw:self.twice_area>0.0,rightmost:self.rightmost,joined:self.regions.is_empty()});
                        }
                    }
                }else if self.face_source.is_some(){
                    if !self.next_ring(body)? {
                        let source=self.face_source.take().ok_or_else(invalid)?;
                        if counting {
                            let holes=source.loops-1;
                            self.total_holes+=holes;
                            self.max_corners=self.max_corners.max(source.corners+2*holes);
                            self.max_regions=self.max_regions.max(source.loops);
                        }else{
                            self.base=(self.mesh.positions.len()/3)as u32;
                            self.cursor=0;
                            self.pair=[0,1];
                            self.stage=PlanarTurn::Checking;
                        }
                    }
                }else{
                    match self.walk.next(body,root,members)?{
                        Visit::Pending=>{}
                        Visit::Face(id)=>{
                            if !self.begin_face(body,id)?{
                                if counting{return Ok((PlanarPreviewStep::Unsupported,receipt));}
                                return Err(ValueError::invariant("planar preview face changed"));
                            }
                            if counting {self.total_faces+=1;}
                        }
                        Visit::End if counting=>{
                            if self.max_corners==0{return Err(invalid());}
                            self.walk=Default::default();
                            self.stage=PlanarTurn::Reserve(0);
                        }
                        Visit::End=>{
                            self.mesh.component_references=ComponentReferenceTable::from_entries(std::mem::take(&mut self.references));
                            return Ok((PlanarPreviewStep::Ready(std::mem::take(&mut self.mesh)),receipt));
                        }
                    }
                }
            }
            PlanarTurn::Checking=>self.check_ring()?,
            PlanarTurn::Writing=>{
                if let Some((domain,label))=self.missing_reference(){
                    return self.write_reference(domain,label,grant,demand).map(|receipt|(PlanarPreviewStep::Pending,receipt));
                }
                let i=self.cursor;
                let n=self.corners.len();
                let group=self.corners[i].region;
                let region=self.regions[group];
                let (prev,next)=(region.previous(i),region.next(i));
                let (left,right)=if region.ccw==(group==0){(prev,next)}else{(next,prev)};
                let point=self.corners[i].point;
                let end=self.corners[next].point;
                self.corners[i].previous=left;
                self.corners[i].next=right;
                self.mesh.positions.extend(point);
                let pick=self.corners[i].vertex.and_then(|id|self.label_indices.get(&id)).map_or(u32::MAX,|row|row.0);
                self.mesh.vertex_ids.push(pick);
                let normal=if self.flipped{-self.normal}else{self.normal};
                self.mesh.normals.extend([normal.x as f32,normal.y as f32,normal.z as f32]);
                let edge=self.label_indices.get_mut(&self.corners[i].edge).ok_or_else(invalid)?;
                if !edge.1{
                    self.mesh.edge_positions.extend(point);
                    self.mesh.edge_positions.extend(end);
                    self.mesh.edge_ids.push(edge.0);
                    edge.1=self.corners[i].edge_end;
                }
                self.cursor+=1;
                if self.cursor==n {
                    if self.regions.len()>1{
                        self.bridge=PlanarBridges::new();
                        self.stage=PlanarTurn::Bridging;
                    }else{self.start_clipping();}
                }
            }
            PlanarTurn::Bridging=>{
                self.charge_triangulation()?;
                if self.bridge.step(&mut self.corners,&mut self.regions)?{self.start_clipping();}
            }
            PlanarTurn::Clipping=>{self.charge_triangulation()?;self.clip()?;}
        }
        if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"planar preview reservation exceeded its grant").with_retained_progress(receipt));}
        Ok((PlanarPreviewStep::Pending,receipt))
    }

    fn start_clipping(&mut self){
        self.remaining=self.corners.len();
        self.cursor=0;
        self.candidates=0;
        self.probe=None;
        self.stage=PlanarTurn::Clipping;
    }

    fn charge_triangulation(&mut self)->Result<(),ValueError>{
        self.triangulation_events+=1;
        if self.triangulation_events>MAXIMUM_TRIANGULATION_EVENTS {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"planar preview exceeds the triangulation limit"));
        }
        Ok(())
    }

    fn check_ring(&mut self)->Result<(),ValueError>{
        self.charge_triangulation()?;
        let [a,b]=self.pair;
        let n=self.corners.len();
        let uv=|index:usize|self.corners[index].uv;
        let next=|index:usize|self.regions[self.corners[index].region].next(index);
        let end=next(a);
        if b<n {
            let other_end=next(b);
            if b!=end&&a!=other_end&&overlaps(uv(a),uv(end),uv(b),uv(other_end)){return Err(invalid());}
            self.pair[1]+=1;
        }else{
            let [p,q,r]=[a,end,next(end)].map(uv);
            if p==q || self.corners[a].point==self.corners[end].point || (turn(p,q,r)==Orient::Zero && (p[0]-q[0])*(r[0]-q[0])+(p[1]-q[1])*(r[1]-q[1])>=0.0){
                return Err(invalid());
            }
            self.pair=[a+1,a+2];
            if a+1==n {self.stage=PlanarTurn::Writing;}
        }
        Ok(())
    }

    fn skip_candidate(&mut self,next:usize)->Result<(),ValueError>{
        self.candidates+=1;
        if self.candidates>=self.remaining {return Err(invalid());}
        self.cursor=next;
        self.probe=None;
        Ok(())
    }

    fn clip(&mut self)->Result<(),ValueError>{
        let i=self.cursor;
        let (left,right)=(self.corners[i].previous,self.corners[i].next);
        let [a,b,c]=[left,i,right].map(|index|self.corners[index].uv);
        if self.probe.is_none(){
            if turn(a,b,c)!=Orient::Positive {return self.skip_candidate(right);}
            let [a,b,c]=[left,i,right].map(|j|{let [x,y,z]=self.corners[j].point.map(f64::from);Pnt3::new(x,y,z)});
            if (b-a).cross(c-a).dot(self.normal)<=0.0 {return self.skip_candidate(right);}
            self.probe=Some(self.corners[right].next);
            return Ok(());
        }
        let p=self.probe.unwrap();
        if p!=left {
            let d=self.corners[p].uv;
            if ![a,b,c].contains(&d)&&[turn(a,b,d),turn(b,c,d),turn(c,a,d)].into_iter().all(|side|side!=Orient::Negative) {return self.skip_candidate(right);}
            self.probe=Some(self.corners[p].next);
            return Ok(());
        }
        let mut tri=[left,i,right].map(|index|self.base+self.corners[index].source as u32);
        if self.flipped{tri.swap(1,2);}
        self.mesh.indices.extend(tri);
        self.mesh.face_ids.push(self.label_indices.get(&self.face).ok_or_else(invalid)?.0);
        if self.remaining==3 {self.stage=PlanarTurn::Reading;}
        else{
            self.corners[left].next=right;
            self.corners[right].previous=left;
            self.remaining-=1;
            self.cursor=right;
            self.candidates=0;
            self.probe=None;
        }
        Ok(())
    }
}
