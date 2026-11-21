//! 🪚️ Reconstructs bounded Difference by verifying the retained exterior partitions.

use super::*;

pub(super) const MAX_PARTITION_CUTS: usize = u32::BITS as usize;
#[path = "🐚️exterior-links/🦀️.rs"]
mod exterior_links;
use exterior_links::ExteriorLinks;

#[derive(Default,semio_framework_value::RetireOwned)]
pub(super) struct Branch {
    pub(super) soup:Soup, pub(super) faces:Vec<MeshIntersectionFace>, pub(super) points:Vec<SourceVertex>,
    clip:Option<MeshClipJob>, retired:Vec<MeshClipJob>, origins:Vec<u32>,
    next_faces:Vec<MeshIntersectionFace>, cut:usize, phase:u8,
    face:usize, corner:usize, cursor:usize, side:[bool;2],
    pub(super) empty:bool,
}

impl Branch {
    pub(super) fn advance_partition(&mut self,source:&Soup,cuts:&mut[(Plane,usize)],outside:usize,epsilon:f32)->MeshResult<bool>{
        let count=(outside+1).min(cuts.len());
        let flip=|plane:&mut(Plane,usize)|plane.0.normal=plane.0.normal.scale(-1.0);
        if let Some(plane)=cuts.get_mut(outside){flip(plane);}
        let result=self.advance(source,&cuts[..count],epsilon);
        if let Some(plane)=cuts.get_mut(outside){flip(plane);}
        result
    }

    fn advance(&mut self,source:&Soup,cuts:&[(Plane,usize)],epsilon:f32)->MeshResult<bool>{
        match self.phase {
            0=>{
                if let Some(&point)=source.positions.get(self.cursor){
                    self.soup.positions.push(point);self.points.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;
                }else{self.phase=1;}
            },
            1=>{
                if let Some(ring)=source.faces.get(self.face){
                    if self.corner==0{
                        self.soup.faces.push(Vec::new());self.faces.push(MeshIntersectionFace::Left(self.face as u32));
                    }
                    self.soup.faces.last_mut().unwrap().push(ring[self.corner]);
                    self.corner+=1;
                    if self.corner==ring.len(){self.face+=1;self.corner=0;}
                }else{self.face=0;self.phase=2;}
            },
            2=>{
                let Some(&(plane,_))=cuts.get(self.cut)else{return Ok(true);};
                if let Some(ring)=self.soup.faces.get(self.face){
                    let point=Vec3(self.soup.positions[ring[self.corner] as usize]);
                    let height=anchored_distance(point,plane.normal,plane.origin);
                    self.side[0]|=height< -epsilon;self.side[1]|=height>epsilon;
                    self.corner+=1;
                    if self.corner==ring.len(){self.face+=1;self.corner=0;}
                }else{
                    match self.side{
                        [true,true]=>{
                            self.clip=Some(MeshClipJob::anchored(plane.normal,plane.origin,epsilon));self.phase=3;
                        }
                        [true,false]=>{self.cut+=1;self.face=0;self.side=[false;2];}
                        [false,true]=>{self.empty=true;return Ok(true);}
                        [false,false]=>return Err(invalid("mesh partition has no stable side of the support")),
                    }
                }
            },
            3=>{
                let clip=self.clip.as_mut().unwrap();
                if clip.advance(&mut self.soup)?{self.origins=clip.take_source_faces()?;self.cursor=0;self.phase=4;}
                if self.soup.positions.len()>100_000{return Err(invalid("mesh partition exceeds its point history limit"));}
            },
            4=>{
                if self.points.len()<self.soup.positions.len(){
                    self.points.push(self.clip.as_ref().unwrap().vertex_source(self.points.len())?);
                }else{self.phase=5;}
            },
            5=>{
                if self.cursor==self.origins.len(){
                    self.faces=std::mem::take(&mut self.next_faces);
                    self.retired.push(self.clip.take().unwrap());
                    self.cut+=1;self.phase=2;self.face=0;self.side=[false;2];
                }else{
                    let owner=if self.cursor+1==self.origins.len(){MeshIntersectionFace::RightPlane(cuts[self.cut].1 as u32)}
                        else{self.faces[self.origins[self.cursor] as usize]};
                    self.next_faces.push(owner);self.cursor+=1;
                }
            },
            _=>unreachable!(),
        }
        Ok(false)
    }
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct PartitionDifference {
    cuts:Vec<(Plane,usize)>, epsilon:f32, branch:Branch, retired:Vec<Branch>,
    output:MeshIntersectionSolid, indices:HistoryFoldIndex<[u32;3],u32>,
    live:HistoryFoldSet<u32>, remap:Vec<u32>, rings:Vec<Vec<u32>>,
    incidence:HistoryFoldIndex<(u32,u32),[Option<bool>;2]>,
    paired:usize, slab:bool, first_shell_faces:usize,
    stitch:RingStitch, links:ExteriorLinks,
    part:usize, face:usize, corner:usize, cursor:usize, phase:u8,
}

impl PartitionDifference {
    pub(super) fn new(cuts:Vec<(Plane,usize)>,epsilon:f32)->MeshResult<Self>{
        let slab=cuts.len()==2&&length(cross(cuts[0].0.normal.0.map(f64::from),cuts[1].0.normal.0.map(f64::from)))<=1e-6;
        if slab{
            let (a,b)=(cuts[0].0,cuts[1].0);
            if a.normal!=b.normal.scale(-1.0)||anchored_distance(b.origin,a.normal,a.origin)>= -2.0*epsilon{
                return Err(invalid("convex difference requires distinct opposing slab planes"));
            }
        }
        Ok(Self{cuts,epsilon,branch:Branch::default(),retired:Vec::new(),
            output:MeshIntersectionSolid{geometry:Soup::default(),faces:Vec::new(),points:Vec::new(),right_points:Vec::new(),right_interpolations:Vec::new(),retained_operand:None},
            indices:HistoryFoldIndex::new(),live:HistoryFoldSet::new(),remap:Vec::new(),rings:Vec::new(),
            incidence:HistoryFoldIndex::new(),paired:0,slab,first_shell_faces:0,stitch:RingStitch::with_tolerance(f64::from(epsilon)/2.),links:ExteriorLinks::default(),
            part:0,face:0,corner:0,cursor:0,phase:0})
    }

    fn clip(&mut self,source:&Soup)->MeshResult<()>{
        let done=if self.slab{
            self.branch.advance_partition(source,&mut[self.cuts[self.part]],0,self.epsilon)?
        }else{
            self.branch.advance_partition(source,&mut self.cuts,self.part,self.epsilon)?
        };
        if done{self.phase=1;}
        Ok(())
    }

    fn point(&mut self)->MeshResult<()>{
        let Some(&point)=self.branch.soup.positions.get(self.cursor).filter(|_|!self.branch.empty)else{self.phase=2;self.cursor=0;return Ok(());};
        let key=point.map(|v|if v==0.0{0}else{v.to_bits()});
        let id=if let Some(&id)=self.indices.get(&key){id}else{
            if self.output.geometry.positions.len()==100_000{return Err(invalid("partitioned difference exceeds its combined point history limit"));}
            let id=self.output.geometry.positions.len() as u32;
            let origin=match self.branch.points[self.cursor]{
                SourceVertex::Original(original)=>SourceVertex::Original(original),
                SourceVertex::Edge{a,b,t}=>SourceVertex::Edge{a:self.remap[a as usize],b:self.remap[b as usize],t},
                _=>return Err(invalid("partitioned difference lost interpolation lineage")),
            };
            self.indices.insert(key,id);self.output.geometry.positions.push(point);self.output.points.push(origin);id
        };
        self.remap.push(id);self.cursor+=1;Ok(())
    }

    fn ring(&mut self){
        let Some(ring)=self.branch.soup.faces.get(self.face).filter(|_|!self.branch.empty)else{
            self.retired.push(std::mem::take(&mut self.branch));
            self.part+=1;self.face=0;self.remap.clear();
            if self.slab&&self.part==1{self.first_shell_faces=self.rings.len();}
            if self.slab&&self.part==2{
                self.output.geometry.faces=std::mem::take(&mut self.rings);self.phase=4;
            }else{self.phase=if self.part<=self.cuts.len(){0}else{3};}
            return;
        };
        let owner=match self.branch.faces[self.face]{
            MeshIntersectionFace::Left(i) if self.part<self.cuts.len()=>MeshIntersectionFace::Left(i),
            MeshIntersectionFace::RightPlane(i) if self.slab||self.part==self.cuts.len()=>MeshIntersectionFace::ReversedRightPlane(i),
            _=>{self.face+=1;return;},
        };
        if self.corner==0{
            self.output.faces.push(owner);self.rings.push(Vec::new());
        }
        let corner=if !self.slab&&self.part==self.cuts.len(){ring.len()-1-self.corner}else{self.corner};
        let id=self.remap[ring[corner] as usize];
        self.rings.last_mut().unwrap().push(id);self.live.insert(id);
        self.corner+=1;
        if self.corner==ring.len(){self.corner=0;self.face+=1;}
    }

    fn verify(&mut self)->MeshResult<()>{
        let Some(ring)=self.output.geometry.faces.get(self.face)else{
            if self.paired!=self.incidence.len(){return Err(invalid("partitioned difference produced open incidence"));}
            self.phase=5;self.cursor=0;return Ok(());
        };
        let (a,b)=(ring[self.corner],ring[(self.corner+1)%ring.len()]);
        if a==b{return Err(invalid("partitioned difference produced a collapsed edge"));}
        let point=|id:u32|self.output.geometry.positions[id as usize];
        let owner=self.output.faces[self.face];
        if self.slab{
            let shell=usize::from(self.face>=self.first_shell_faces);
            let (plane,id)=self.cuts[shell];let (opposite,_)=self.cuts[1-shell];
            let p=Vec3(point(a));
            if anchored_distance(p,plane.normal,plane.origin)< -self.epsilon
                ||anchored_distance(p,opposite.normal,opposite.origin)>= -self.epsilon
                ||matches!(owner,MeshIntersectionFace::ReversedRightPlane(cap) if cap as usize!=id){
                return Err(invalid("slab difference residuals are not strictly separated by their cutter planes"));
            }
        }
        let cutter=match owner{
            MeshIntersectionFace::Left(_)=>false,
            MeshIntersectionFace::ReversedRightPlane(_)=>true,
            _=>return Err(invalid("partitioned difference lost its original face owner")),
        };
        let uses=self.incidence.entry((a.min(b),a.max(b))).or_insert_with(||[None;2]);
        let direction=usize::from(a<b);
        if uses[direction].is_some(){return Err(invalid("partitioned difference produced nonmanifold incidence"));}
        self.paired+=usize::from(uses[1-direction].is_some());
        uses[direction]=Some(cutter);
        let anchor=point(self.output.geometry.faces[0][0]);
        let volume=dot(difference(point(ring[0]),anchor),cross(difference(point(a),anchor),difference(point(b),anchor)))/6.;
        let previous=ring[(self.corner+ring.len()-1)%ring.len()];
        self.links.record(a,previous,b,volume,self.corner==0)?;
        self.corner+=1;if self.corner==ring.len(){self.corner=0;self.face+=1;}
        Ok(())
    }

    fn register_opening(&mut self)->MeshResult<()>{
        if self.cursor==self.incidence.slot_count(){self.phase=6;return Ok(());}
        if let Some((&(a,b),&[Some(left),Some(right)]))=self.incidence.slot_entry(self.cursor){
            if left!=right{self.links.opening(a,b)?;}
        }
        self.cursor+=1;
        Ok(())
    }

    pub(super) fn advance(&mut self,source:&Soup)->MeshResult<bool>{
        match self.phase {
            0=>self.clip(source)?,
            1=>self.point()?,
            2=>self.ring(),
            3=>if self.stitch.advance(&self.output.geometry.positions,&self.rings,&self.live,&mut self.output.geometry.faces){self.phase=4;self.face=0;},
            4=>self.verify()?,
            5=>self.register_opening()?,
            6=>return self.links.advance(self.epsilon),
            _=>unreachable!(),
        }
        Ok(false)
    }

    pub(super) fn take_output(&mut self)->MeshIntersectionSolid{
        MeshIntersectionSolid{geometry:std::mem::take(&mut self.output.geometry),faces:std::mem::take(&mut self.output.faces),
            points:std::mem::take(&mut self.output.points),right_points:Vec::new(),right_interpolations:Vec::new(),retained_operand:None}
    }
}
