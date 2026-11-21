//! ⚗️ Clips one certified oriented convex polygon source by the other's supporting planes.

#[path = "../➕️convex-union/🦀️.rs"]
mod convex_union;
pub use convex_union::MeshConvexUnion;
#[path = "../✂️partition-difference/🦀️.rs"]
mod partition_difference;
use partition_difference::{MAX_PARTITION_CUTS, PartitionDifference};
#[path = "../🧵️ring-stitch/🦀️.rs"]
mod ring_stitch;
use ring_stitch::RingStitch;
#[path = "../⬚️nested-difference/🦀️.rs"]
mod nested_difference;
use nested_difference::NestedDifference;
#[path = "../📐️nested-prism/🦀️.rs"]
mod nested_prism;
use nested_prism::NestedPrism;
#[path = "../✂️prism-cap-intersection/🦀️.rs"]
mod prism_cap_intersection;
use prism_cap_intersection::{PrismClip, PrismClipStep};

use super::*;
const LIMIT:usize=4_000_000;

#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Plane {origin:Vec3,normal:Vec3}

#[derive(Default,semio_framework_value::RetireOwned)]
struct Certificate {
    extent:f64,epsilon:f32,planes:Vec<Plane>,
    edges:HistoryFoldIndex<(u32,u32),(u32,bool,bool)>,paired:usize,
    used:HistoryFoldSet<u32>,parents:Vec<(u32,u32)>,components:usize,
    sums:[f64;3],face:usize,ring:usize,cursor:usize,strict:bool,convex:bool,phase:u8,
}

fn invalid(reason:&str)->MeshKernelError{MeshKernelError::InvalidInput(reason.into())}

pub(super) fn difference(a:[f32;3],b:[f32;3])->[f64;3]{
    std::array::from_fn(|i|f64::from(a[i])-f64::from(b[i]))
}

fn dot(a:[f64;3],b:[f64;3])->f64{(0..3).map(|i|a[i]*b[i]).sum()}

pub(super) fn cross([a,b,c]:[f64;3],[d,e,f]:[f64;3])->[f64;3]{[b*f-c*e,c*d-a*f,a*e-b*d]}

pub(super) fn length(a:[f64;3])->f64{dot(a,a).sqrt()}

impl Certificate {
    fn root(&self,mut i:u32)->u32{
        while self.parents[i as usize].0!=i{i=self.parents[i as usize].0;}
        i
    }

    fn join(&mut self,a:u32,b:u32){
        let (mut a,mut b)=(self.root(a) as usize,self.root(b) as usize);
        if a==b{return;}
        if self.parents[a].1<self.parents[b].1{std::mem::swap(&mut a,&mut b);}
        self.parents[b].0=a as u32;
        self.parents[a].1+=self.parents[b].1;
        self.components-=1;
    }

    fn next(&mut self){self.cursor=0;self.phase+=1;}

    fn vertex(source:&Soup,id:u32)->MeshResult<[f32;3]>{
        source.positions.get(id as usize).copied().ok_or(MeshKernelError::InvalidHandle)
    }

    fn advance(&mut self,source:&Soup)->MeshResult<bool>{
        match self.phase{
            0=>{
                if let Some(&point)=source.positions.get(self.cursor){
                    if !point.iter().all(|v|v.is_finite()){return Err(invalid("convex intersection positions must be finite"));}
                    self.extent=self.extent.max(length(difference(point,source.positions[0])));
                    self.cursor+=1;
                }else{
                    if self.extent==0.0||self.extent>1e18{return Err(invalid("convex intersection exceeds its reliable numeric range"));}
                    self.epsilon=(self.extent*1e-6).max(f64::from(f32::MIN_POSITIVE)) as f32;
                    self.next();
                }
            },
            1=>{
                if self.parents.len()<source.faces.len(){
                    self.parents.push((self.parents.len() as u32,1));self.components+=1;
                }else{self.next();}
            },
            2=>{
                let Some(ring)=source.faces.get(self.face)else{
                    if self.edges.len()!=self.paired||self.components!=1||self.used.len()!=source.positions.len(){
                        return Err(invalid("intersection operands must be connected closed surface meshes using all positions"));
                    }
                    self.face=0;self.ring=0;self.next();return Ok(false);
                };
                if ring.len()<3{return Err(invalid("convex intersection face requires three vertices"));}
                let (a,b)=(ring[self.ring],ring[(self.ring+1)%ring.len()]);
                let (p,q)=(Self::vertex(source,a)?,Self::vertex(source,b)?);
                if a==b||p==q{return Err(invalid("convex intersection face has a collapsed edge"));}
                self.used.insert(a);
                let key=(a.min(b),a.max(b));
                if let Some((face,direction,paired))=self.edges.get_mut(&key){
                    if *face==self.face as u32||*paired||*direction==(a<b){
                        return Err(invalid("intersection operands require reciprocal manifold edges"));
                    }
                    *paired=true;
                    let face=*face;
                    self.join(face,self.face as u32);self.paired+=1;
                }else{self.edges.insert(key,(self.face as u32,a<b,false));}
                let origin=Self::vertex(source,ring[0])?;
                let n=cross(difference(p,origin),difference(q,origin));
                for (sum,v) in self.sums.iter_mut().zip(n){*sum+=v;}
                self.ring+=1;
                if self.ring==ring.len(){
                    let magnitude=length(self.sums);
                    if magnitude<=f64::from(self.epsilon).powi(2){return Err(invalid("convex intersection face is degenerate"));}
                    let normal=Vec3(self.sums.map(|v|(v/magnitude) as f32));
                    self.planes.push(Plane{origin:Vec3(origin),normal});
                    self.face+=1;self.ring=0;self.sums=[0.0;3];
                }
            },
            3=>{
                let plane=self.planes[self.face];
                let Some(&point)=source.positions.get(self.cursor)else{
                    if !self.strict{return Err(invalid("convex intersection operand has no solid interior"));}
                    self.strict=false;self.cursor=0;self.face+=1;
                    if self.face==self.planes.len(){self.face=0;self.next();}
                    return Ok(false);
                };
                let height=anchored_distance(Vec3(point),plane.normal,plane.origin);
                if height>self.epsilon{return Ok(true);}
                self.strict|=height< -self.epsilon;self.cursor+=1;
            },
            4=>{
                let ring=&source.faces[self.face];
                let plane=self.planes[self.face];
                let point=source.positions[ring[self.cursor] as usize];
                let a=source.positions[ring[self.ring] as usize];
                let b=source.positions[ring[(self.ring+1)%ring.len()] as usize];
                let height=anchored_distance(Vec3(point),plane.normal,plane.origin);
                if height.abs()>self.epsilon{return Err(invalid("convex intersection requires planar polygons"));}
                let edge=difference(b,a);
                let side=dot(cross(edge,difference(point,a)),plane.normal.0.map(f64::from));
                if side< -length(edge)*f64::from(self.epsilon){return Err(invalid("convex intersection requires convex polygon rings"));}
                self.cursor+=1;
                if self.cursor==ring.len(){
                    self.cursor=0;self.ring+=1;
                    if self.ring==ring.len(){self.ring=0;self.face+=1;}
                }
                if self.face==self.planes.len(){self.next();}
            },
            _=>{self.convex=true;return Ok(true);},
        }
        Ok(false)
    }
}

/// 🧱️ Runs measured certification, clipping, source projection and retirement turns.
#[derive(semio_framework_value::RetireOwned)]
pub struct MeshConvexIntersection {
    left:Soup,right:Soup,check:[Certificate;2],
    clip:Option<MeshClipJob>,points:Vec<SourceVertex>,
    faces:Vec<MeshIntersectionFace>,new_faces:Vec<MeshIntersectionFace>,origins:Vec<u32>,
    plane:usize,face:usize,corner:usize,cursor:usize,live_corners:usize,
    strict:[bool;2],output_edges:HistoryFoldIndex<(u32,u32),[bool;2]>,live:HistoryFoldSet<u32>,paired:usize,volume:f64,
    phase:u8,done:usize,cancelled:bool,
    subtract:bool,cut_plane:Option<usize>,additional_cuts:[Option<usize>;MAX_PARTITION_CUTS-1],multiple_cuts:bool,
    allow_partitions:bool,partition_difference:Option<PartitionDifference>,nested_difference:Option<NestedDifference>,prisms:[Option<NestedPrism>;2],prism_clip:Option<PrismClip>,
}

impl MeshConvexIntersection {
    /// 🚪️ Takes ownership of two polygon sources without scanning them.
    pub fn new(left:Soup,right:Soup)->MeshResult<Self>{
        for source in [&left,&right]{
            if !(4..=100_000).contains(&source.positions.len())||!(4..=100_000).contains(&source.faces.len())
                ||source.positions.len().saturating_mul(source.faces.len())>LIMIT{
                return Err(invalid("convex intersection operand exceeds its solid or work limits"));
            }
        }
        if left.faces.len()+right.faces.len()>100_000{return Err(invalid("convex intersection exceeds its face limit"));}
        Ok(Self{left,right,check:Default::default(),clip:None,points:Vec::new(),faces:Vec::new(),new_faces:Vec::new(),origins:Vec::new(),
            plane:0,face:0,corner:0,cursor:0,live_corners:0,strict:[false;2],output_edges:HistoryFoldIndex::new(),live:HistoryFoldSet::new(),paired:0,volume:0.0,phase:0,done:0,cancelled:false,subtract:false,cut_plane:None,additional_cuts:[None;MAX_PARTITION_CUTS-1],multiple_cuts:false,allow_partitions:true,partition_difference:None,nested_difference:None,prisms:Default::default(),prism_clip:None})
    }

    /// ➖️ Subtracts certified boundaries, including strictly contained prismatic tools.
    pub fn difference(left:Soup,right:Soup)->MeshResult<Self>{
        let mut job=Self::new(left,right)?;job.subtract=true;Ok(job)
    }

    /// 📈️ Reports the stable phase and consumed work against the operation's ceiling.
    pub fn progress(&self)->(&'static str,usize,usize){
        (match self.phase{0|1=>"convex-certificate",2=>"intersection-seed",3=>"intersection-classify",4=>"intersection-clip",
            5|6=>"intersection-provenance",7|8=>"intersection-release",9|10=>"intersection-verify",12=>"intersection-refused",13=>"difference-partition",14=>"difference-containment",15=>"difference-contained-shells",16|17=>"mesh-prism-certificate",18=>"mesh-prism-containment",19|20=>"mesh-prism-output",_=>"intersection-done"},self.done,LIMIT)
    }

    /// 🛑️ Stops advancement without publishing partial polygon rings.
    pub fn cancel(&mut self){self.cancelled=true;}

    fn restart(&mut self){
        self.plane+=1;self.face=0;self.corner=0;self.cursor=0;
        self.live_corners=0;self.strict=[false;2];self.phase=3;
    }

    fn verify(&mut self){self.phase=9;self.face=0;self.corner=0;self.live_corners=0;}

    fn count_corner(&mut self)->MeshResult<()>{
        self.live_corners+=1;
        if self.live_corners>600_000{return Err(invalid("convex intersection exceeds its corner limit"));}
        Ok(())
    }

    /// 🪜️ Performs exactly one resumable clip or certificate event.
    pub fn advance(&mut self)->MeshResult<MeshConvexIntersectionStep>{
        if self.cancelled{return Ok(MeshConvexIntersectionStep::Cancelled);}
        if matches!(self.phase,11|12){return Err(invalid("convex intersection is already settled"));}
        let step=self.advance_one();
        if step.is_err(){self.phase=12;}
        step
    }

    fn advance_certificate(&mut self,side:usize)->MeshResult<()>{
        let source=[&self.left,&self.right][side];
        if self.check[side].advance(source)?{
            if self.check[side].convex{self.phase=1+side as u8;}else{
                if self.prisms[1-side].is_some(){return Err(invalid("concave boolean requires a convex containing operand"));}
                self.prisms[side]=Some(NestedPrism::new(source,self.check[side].epsilon)?);
                self.phase=16+side as u8;
            }
        }
        Ok(())
    }

    fn prism_side(&self)->Option<usize>{self.prisms.iter().position(Option::is_some)}

    fn prism_contained(&mut self,side:usize)->MeshResult<bool>{
        let outer=&self.check[1-side];
        let Some(&plane)=outer.planes.get(self.plane)else{return Ok(true);};
        let source=[&self.left,&self.right][side];
        if let Some(&point)=source.positions.get(self.cursor){
            if anchored_distance(Vec3(point),plane.normal,plane.origin)>= -outer.epsilon{
                return Err(invalid("concave boolean requires strictly contained prism geometry"));
            }
            self.cursor+=1;
        }else{self.cursor=0;self.plane+=1;}
        Ok(false)
    }

    fn seed_left(&mut self)->bool{
        if self.points.len()<self.left.positions.len(){self.points.push(SourceVertex::Original(self.points.len() as u32));}
        else if self.faces.len()<self.left.faces.len(){self.faces.push(MeshIntersectionFace::Left(self.faces.len() as u32));}
        else{return true;}
        false
    }

    fn begin_nested_difference(&mut self)->MeshResult<()>{
        if self.left.positions.len()+self.right.positions.len()>100_000{return Err(invalid("nested difference exceeds its point limit"));}
        self.nested_difference=Some(NestedDifference::new(std::mem::take(&mut self.left),
            std::mem::take(&mut self.faces),std::mem::take(&mut self.points)));
        self.phase=15;
        Ok(())
    }

    fn retain_prism(&mut self,side:usize)->Option<MeshIntersectionSolid>{
        let source=if side==0{&mut self.left}else{&mut self.right};
        if self.cursor<source.positions.len(){
            if side==0{self.points.push(SourceVertex::Original(self.cursor as u32));}
            else{self.origins.push(self.cursor as u32);}
            self.cursor+=1;
        }else if self.faces.len()<source.faces.len(){
            let index=self.faces.len() as u32;
            self.faces.push(if side==0{MeshIntersectionFace::Left(index)}else{MeshIntersectionFace::RightPlane(index)});
        }else{
            return Some(MeshIntersectionSolid{
                geometry:std::mem::take(source),faces:std::mem::take(&mut self.faces),
                points:std::mem::take(&mut self.points),right_points:std::mem::take(&mut self.origins),
                right_interpolations:Vec::new(),retained_operand:Some(side as u8),
            });
        }
        None
    }

    fn advance_one(&mut self)->MeshResult<MeshConvexIntersectionStep>{
        use MeshConvexIntersectionStep::*;
        if self.done==LIMIT{return Err(invalid("convex intersection work budget exhausted"));}
        self.done+=1;
        match self.phase {
            0|1=>self.advance_certificate(usize::from(self.phase))?,
            2=>{
                if self.prism_side().is_some(){self.phase=18;}
                else if self.seed_left(){self.phase=if self.subtract&&self.allow_partitions{14}else{3};}
            },
            3=>{
                let Some(&plane)=self.check[1].planes.get(self.plane)else{
                    if !self.subtract{self.verify();return Ok(Working);}
                    if !self.allow_partitions&&self.additional_cuts[0].is_some(){return Err(invalid("convex union requires one opposing support face"));}
                    if self.multiple_cuts{return Err(invalid("convex difference exceeds its supported cutting halfspaces"));}
                    let Some(index)=self.cut_plane else{self.phase=11;return Ok(Empty);};
                    let plane=self.check[1].planes[index];
                    let epsilon=self.check[0].epsilon.min(self.check[1].epsilon);
                    if self.additional_cuts[0].is_some(){
                        let cuts=std::iter::once(index).chain(self.additional_cuts.into_iter().flatten())
                            .map(|id|(self.check[1].planes[id],id)).collect();
                        self.partition_difference=Some(PartitionDifference::new(cuts,epsilon)?);
                        self.phase=13;return Ok(Working);
                    }
                    self.clip=Some(MeshClipJob::anchored(plane.normal.scale(-1.0),plane.origin,epsilon));
                    self.phase=4;return Ok(Working);
                };
                let epsilon=self.check[0].epsilon.min(self.check[1].epsilon);
                if let Some(ring)=self.left.faces.get(self.face){
                    let value=anchored_distance(Vec3(self.left.positions[ring[self.corner] as usize]),plane.normal,plane.origin);
                    self.strict[0]|=value< -epsilon;self.strict[1]|=value>epsilon;
                    self.corner+=1;if self.corner==ring.len(){self.face+=1;self.corner=0;}
                    self.count_corner()?;
                }else if self.subtract{
                    if !self.strict[0]{self.cut_plane=None;self.verify();}
                    else {
                        if self.strict[1]{
                            if self.cut_plane.is_none(){self.cut_plane=Some(self.plane);}
                            else if let Some(slot)=self.additional_cuts.iter_mut().find(|slot|slot.is_none()){*slot=Some(self.plane);}
                            else{self.multiple_cuts=true;}
                        }
                        self.restart();
                    }
                }else if !self.strict[0]{
                    self.phase=11;return Ok(Empty);
                }else if !self.strict[1]{self.restart();}
                else {self.clip=Some(MeshClipJob::anchored(plane.normal,plane.origin,epsilon));self.phase=4;}
            },
            4=>{
                let clip=self.clip.as_mut().unwrap();
                if clip.advance(&mut self.left)?{
                    self.origins=clip.take_source_faces()?;self.cursor=0;self.phase=5;
                }
                if self.left.positions.len()>100_000{return Err(invalid("convex intersection exceeds its point history limit"));}
            },
            5=>{
                if self.points.len()<self.left.positions.len(){
                    self.points.push(self.clip.as_ref().unwrap().vertex_source(self.points.len())?);
                }else{self.phase=6;}
            },
            6=>{
                if self.cursor==self.origins.len(){
                    self.faces=std::mem::take(&mut self.new_faces);self.cursor=0;self.phase=7;
                }else{
                    let origin=if self.cursor+1==self.origins.len(){
                        self.cut_plane.map_or(MeshIntersectionFace::RightPlane(self.plane as u32),|i|MeshIntersectionFace::ReversedRightPlane(i as u32))
                    }
                        else{self.faces[self.origins[self.cursor] as usize]};
                    self.new_faces.push(origin);self.cursor+=1;
                }
            },
            7=>{
                let clip=self.clip.as_mut().unwrap();
                if self.cursor==clip.intersections.slot_count(){self.cursor=0;self.phase=8;}
                else{clip.intersections.extract_slot_if(self.cursor,|_,_|false);self.cursor+=1;}
            },
            8=>{
                let clip=self.clip.as_mut().unwrap();
                if self.cursor<clip.cap_owners.slot_count(){clip.cap_owners.extract_slot_if(self.cursor,|_,_|false);self.cursor+=1;}
                else if clip.face_weights.pop().is_none(){
                    self.clip=None;self.origins=Vec::new();
                    if self.subtract{self.verify();}else{self.restart();}
                }
            },
            9=>{
                if let Some(ring)=self.left.faces.get(self.face){
                    let (a,b)=(ring[self.corner],ring[(self.corner+1)%ring.len()]);
                    if a==b{return Err(invalid("intersection produced a collapsed edge"));}
                    let uses=self.output_edges.entry((a.min(b),a.max(b))).or_insert_with(||[false;2]);
                    let direction=usize::from(a<b);
                    if uses[direction]{return Err(invalid("intersection produced nonmanifold incidence"));}
                    self.paired+=usize::from(uses[1-direction]);uses[direction]=true;self.live.insert(a);
                    let vertex=|id:u32|self.left.positions[id as usize];
                    let anchor=vertex(self.left.faces[0][0]);
                    self.volume+=dot(difference(vertex(ring[0]),anchor),cross(difference(vertex(a),anchor),difference(vertex(b),anchor)))/6.0;
                    self.corner+=1;if self.corner==ring.len(){self.corner=0;self.face+=1;}
                    self.count_corner()?;
                }else{
                    if self.paired!=self.output_edges.len(){return Err(invalid("intersection produced open incidence"));}
                    let tolerance=f64::from(self.check[0].epsilon.min(self.check[1].epsilon)).powi(3);
                    if self.volume< -tolerance{return Err(invalid("intersection produced reversed winding"));}
                    if self.volume<=tolerance{self.phase=11;return Ok(Empty);}
                    self.phase=10;self.cursor=0;self.plane=0;
                }
            },
            10=>{
                if self.cursor==self.left.positions.len(){
                    self.phase=11;
                    let mut geometry=std::mem::take(&mut self.left);
                    geometry.normals.clear();
                    return Ok(Solid(MeshIntersectionSolid{geometry,faces:std::mem::take(&mut self.faces),points:std::mem::take(&mut self.points),right_points:Vec::new(),right_interpolations:Vec::new(),retained_operand:None}));
                }
                if !self.live.contains(&(self.cursor as u32)){self.cursor+=1;return Ok(Working);}
                let left_count=self.check[0].planes.len();
                let owner=usize::from(self.plane>=left_count);
                let right_count=if self.subtract{usize::from(self.cut_plane.is_some())}else{self.check[1].planes.len()};
                let index=if owner==1&&self.subtract{self.cut_plane.unwrap()}else{self.plane-owner*left_count};
                let plane=self.check[owner].planes[index];
                let mut height=anchored_distance(Vec3(self.left.positions[self.cursor]),plane.normal,plane.origin);
                if owner==1&&self.subtract{height=-height;}
                if height>self.check[owner].epsilon{return Err(invalid("intersection vertex lies outside a source halfspace"));}
                self.plane+=1;
                if self.plane==left_count+right_count{self.plane=0;self.cursor+=1;}
            },
            13=>{
                let partition=self.partition_difference.as_mut().unwrap();
                if partition.advance(&self.left)?{self.phase=11;return Ok(Solid(partition.take_output()));}
            },
            14=>{
                let Some(&plane)=self.check[0].planes.get(self.plane)else{
                    self.begin_nested_difference()?;return Ok(Working);
                };
                if let Some(&point)=self.right.positions.get(self.cursor){
                    if anchored_distance(Vec3(point),plane.normal,plane.origin)>= -self.check[0].epsilon{
                        self.plane=0;self.cursor=0;self.phase=3;
                    }else{self.cursor+=1;}
                }else{self.plane+=1;self.cursor=0;}
            },
            16|17=>{
                let side=usize::from(self.phase-16);
                let source=[&self.left,&self.right][side];
                if self.prisms[side].as_mut().unwrap().advance(source,&self.check[side].planes)?{self.phase=1+side as u8;}
            },
            18=>{
                let side=self.prism_side().unwrap();
                if self.subtract&&side==1{
                    if self.prism_contained(side)?{self.phase=20;}
                }else{
                    let source=[&self.left,&self.right][side];
                    let prism=self.prisms[side].as_ref().unwrap();
                    let source_check=&self.check[side];
                    let outer=&self.check[1-side];
                    let clip=self.prism_clip.get_or_insert_with(||PrismClip::new(side,self.subtract));
                    match clip.advance(source,prism,&source_check.planes,&outer.planes,source_check.epsilon.min(outer.epsilon))?{
                        PrismClipStep::More=>{},
                        PrismClipStep::Retained=>self.phase=19,
                        PrismClipStep::Empty=>{self.phase=11;return Ok(Empty);},
                        PrismClipStep::Clipped(output)=>{self.phase=11;return Ok(Solid(output));},
                    }
                }
            },
            19=>{
                if let Some(output)=self.retain_prism(self.prism_side().unwrap()){self.phase=11;return Ok(Solid(output));}
            },
            20=>{if self.seed_left(){self.begin_nested_difference()?;}},
            15=>{
                let nested=self.nested_difference.as_mut().unwrap();
                if nested.advance(&self.right,self.check[0].epsilon.min(self.check[1].epsilon))?{
                    self.phase=11;return Ok(Solid(nested.take_output()));
                }
            },
            _=>return Err(invalid("convex intersection output already transferred")),
        }
        Ok(Working)
    }
}

impl semio_framework_3d_mesh_contract::MeshIntersectionKernel for MeshConvexIntersection {
    fn create(left:Soup,right:Soup)->MeshResult<Self>{Self::new(left,right)}
    fn advance(&mut self)->MeshResult<MeshConvexIntersectionStep>{MeshConvexIntersection::advance(self)}
}

/// ➖️ A retained convex Difference provider for authored projection.
#[derive(semio_framework_value::RetireOwned)]
pub struct MeshConvexDifference(MeshConvexIntersection);

impl semio_framework_3d_mesh_contract::MeshIntersectionKernel for MeshConvexDifference {
    fn create(left:Soup,right:Soup)->MeshResult<Self>{MeshConvexIntersection::difference(left,right).map(Self)}
    fn advance(&mut self)->MeshResult<MeshConvexIntersectionStep>{self.0.advance()}
}
