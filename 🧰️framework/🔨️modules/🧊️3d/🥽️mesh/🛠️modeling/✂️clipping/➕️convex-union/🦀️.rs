//! ➕️ Joins certified solid boundaries across one matching supporting face.

use super::*;
#[path = "../➗️partial-union/🦀️.rs"]
mod partial_union;
use partial_union::PartialUnion;

#[derive(semio_framework_value::RetireOwned)]
pub struct MeshConvexUnion {
    kernel:MeshConvexIntersection, output:Option<MeshIntersectionSolid>,partial:Option<PartialUnion>,
    active:[Vec<usize>;2],
    inside:[bool;2], separated:bool, contact:Option<usize>, signs:[bool;3],
    base:usize, side:usize, plane:usize, face:usize, corner:usize,
    seam:Option<usize>, left_seam:Option<usize>, anchor:usize, cap:Vec<u32>, remap:HistoryFoldIndex<u32,u32>,
    edges:HistoryFoldIndex<(u32,u32),[bool;2]>, paired:usize,
    phase:u8, done:usize, cancelled:bool,
}

impl MeshConvexUnion {
    /// 🧭️ Owns both sources for incremental certification and boundary welding.
    pub fn new(left:Soup,right:Soup)->MeshResult<Self>{
        let mut kernel=MeshConvexIntersection::difference(left,right)?;kernel.allow_partitions=false;
        Ok(Self{kernel,output:None,partial:None,active:Default::default(),inside:[true;2],separated:false,contact:None,
            signs:[false;3],base:0,side:0,plane:0,face:0,corner:0,seam:None,left_seam:None,anchor:0,cap:Vec::new(),remap:HistoryFoldIndex::new(),
            edges:HistoryFoldIndex::new(),paired:0,phase:0,done:0,cancelled:false})
    }

    /// 📊️ Reports completed retained events and the fixed work ceiling.
    pub fn progress(&self)->(usize,usize){(self.done,LIMIT)}

    /// 🛑️ Stops the operation before polygon publication.
    pub fn cancel(&mut self){self.cancelled=true;}

    fn epsilon(&self)->f32{self.kernel.check[0].epsilon.min(self.kernel.check[1].epsilon)}

    fn base(&mut self,side:usize,retained:bool){
        self.base=side;
        let mut geometry=std::mem::take(if side==0{&mut self.kernel.left}else{&mut self.kernel.right});
        if !retained{geometry.normals.clear();}
        self.output=Some(MeshIntersectionSolid{geometry,faces:Vec::new(),points:Vec::new(),right_points:Vec::new(),right_interpolations:Vec::new(),retained_operand:retained.then_some(side as u8)});
        self.face=0;self.corner=0;self.phase=7;
    }

    fn classify_axial(&mut self) -> MeshResult<()> {
        let [Some(left),Some(right)] = &self.kernel.prisms else { unreachable!() };
        let Some(axis)=left.matching_axis(&self.kernel.left,right,&self.kernel.right) else {
            self.kernel.start_axial_profile()?;
            self.phase=14;
            return Ok(());
        };
        let ordered = |side: usize, caps: [usize;2]| {
            let planes = &self.kernel.check[side].planes;
            let [a,b] = caps.map(|face| (planes[face].origin.0[axis],face));
            if a.0 < b.0 { [a,b] } else { [b,a] }
        };
        let [(a,low_left),(b,high_left)] = ordered(0,left.caps());
        let [(c,low_right),(d,high_right)] = ordered(1,right.caps());
        if a <= c && d <= b { self.base(0,true); }
        else if c <= a && b <= d { self.base(1,true); }
        else if b < c || d < a { self.base(0,false); }
        else if b == c { self.start_contact(high_left,low_right); }
        else if d == a { self.start_contact(low_left,high_right); }
        else {
            let overlap = b.min(d) - a.max(c);
            let stock_tail = if a < c { c-a } else { b-d };
            if overlap <= self.epsilon() || stock_tail <= self.epsilon() {
                return Err(invalid("concave union has no stable axial overlap or stock tail"));
            }
            self.phase=2;
        }
        Ok(())
    }

    fn classify(&mut self)->MeshResult<()>{
        if self.kernel.prisms.iter().all(Option::is_some){return self.classify_axial();}
        if let Some(side)=self.kernel.prism_side(){
            if self.kernel.prism_contained(side)?{self.base(1-side,true);}
            return Ok(());
        }
        if self.side==2{
            if self.inside[1]{self.base(0,true);}
            else if self.inside[0]{self.base(1,true);}
            else if self.separated{self.base(0,false);}
            else if let Some(seam)=self.contact{self.seam=Some(seam);self.face=0;self.phase=10;}
            else if self.active.iter().all(|cuts|cuts.len()==1){self.phase=2;}
            else{
                if self.active.iter().any(Vec::is_empty){
                    return Err(invalid("partial union requires crossing supports on each operand"));
                }
                let cuts=std::array::from_fn(|side|self.active[side].iter().map(|&id|(self.kernel.check[1-side].planes[id],id)).collect());
                self.partial=Some(PartialUnion::new(cuts,self.epsilon()));self.phase=13;
            }
            return Ok(());
        }
        let Some(&plane)=self.kernel.check[1-self.side].planes.get(self.plane)else{
            self.side+=1;self.plane=0;return Ok(());
        };
        let points=if self.side==0{&self.kernel.left.positions}else{&self.kernel.right.positions};
        if let Some(&point)=points.get(self.corner){
            let epsilon=self.epsilon();
            let v=anchored_distance(Vec3(point),plane.normal,plane.origin);
            self.signs[0]|=v< -epsilon;self.signs[1]|=v>epsilon;self.signs[2]|=v.abs()<=epsilon;
            self.corner+=1;
        }else{
            self.inside[self.side]&=!self.signs[1];
            if self.signs[0]&&self.signs[1]{self.active[self.side].push(self.plane);}
            self.separated|=!self.signs[0]&&!self.signs[2];
            if self.side==0&&!self.signs[0]&&self.signs[2]{self.contact=Some(self.plane);}
            self.corner=0;self.plane+=1;self.signs=[false;3];
        }
        Ok(())
    }

    fn start_contact(&mut self, left: usize, right: usize) {
        self.left_seam=Some(left);
        self.seam=Some(right);
        self.cap=self.kernel.left.faces.swap_remove(left);
        self.base(0,false);
    }

    fn find_contact_face(&mut self)->MeshResult<()>{
        let right=self.kernel.check[1].planes[self.seam.unwrap()];
        let Some(&left)=self.kernel.check[0].planes.get(self.face)else{
            return Err(invalid("convex union contact does not share a full opposing face"));
        };
        if left.normal==right.normal.scale(-1.0)&&anchored_distance(left.origin,right.normal,right.origin)==0.0{
            let left_ring=&self.kernel.left.faces[self.face];
            if left_ring.len()!=self.kernel.right.faces[self.seam.unwrap()].len(){
                return Err(invalid("convex union contact does not share a full opposing face"));
            }
            self.start_contact(self.face,self.seam.unwrap());
        }else{self.face+=1;}
        Ok(())
    }

    fn weld(&mut self,solid:MeshIntersectionSolid)->MeshResult<()>{
        let out=self.output.insert(solid);
        let mut caps = out.faces.iter().enumerate().filter_map(|(index, owner)| {
            let MeshIntersectionFace::ReversedRightPlane(seam) = owner else { return None; };
            Some((index,*seam))
        });
        let Some((index,seam))=caps.next()else{
            return Err(invalid("union requires one opposing support face"));
        };
        if caps.next().is_some(){return Err(invalid("union requires one opposing support face"));}
        out.faces.swap_remove(index);
        self.cap=out.geometry.faces.swap_remove(index);
        if self.cap.len()!=self.kernel.right.faces[seam as usize].len(){
            return Err(invalid("convex union cross section does not match the opposing support face"));
        }
        self.seam=Some(seam as usize);
        self.corner=0;self.phase=3;
        Ok(())
    }

    fn pair(&mut self)->MeshResult<()>{
        let out=self.output.as_ref().unwrap();
        let ring=&self.kernel.right.faces[self.seam.unwrap()];
        if self.phase==3{
            if self.anchor==self.cap.len(){return Err(invalid("convex union cross section does not match the opposing support face"));}
            if out.geometry.positions[self.cap[self.anchor] as usize]==self.kernel.right.positions[ring[0] as usize]{
                self.phase=4;
            }else{self.anchor+=1;}
            return Ok(());
        }
        let i=self.corner;
        let left=self.cap[(self.anchor+self.cap.len()-i)%self.cap.len()];
        let right=ring[i];
        if out.geometry.positions[left as usize]!=self.kernel.right.positions[right as usize]{
            return Err(invalid("convex union cross section does not match the opposing support face"));
        }
        self.remap.insert(right,left);
        self.corner+=1;
        if self.corner==ring.len(){self.phase=5;self.corner=0;}
        Ok(())
    }

    fn append_point(&mut self)->MeshResult<()>{
        let out=self.output.as_mut().unwrap();
        if let Some(&point)=self.kernel.right.positions.get(self.corner){
            let id=if let Some(&mapped)=self.remap.get(&(self.corner as u32)){mapped}else{
                if out.geometry.positions.len()==100_000{return Err(invalid("convex union exceeds its point history limit"));}
                let id=out.geometry.positions.len() as u32;out.geometry.positions.push(point);id
            };
            out.right_points.push(id);self.corner+=1;
        }else{self.phase=6;self.face=0;self.corner=0;}
        Ok(())
    }

    fn append_face(&mut self){
        let out=self.output.as_mut().unwrap();
        let Some(ring)=self.kernel.right.faces.get(self.face)else{
            self.face=0;self.corner=0;self.plane=0;
            self.phase=if self.kernel.prisms.iter().all(Option::is_some){11}
                else if self.seam.is_some(){8}else{9};
            return;
        };
        if self.seam==Some(self.face){self.face+=1;return;}
        if self.corner==0{
            out.faces.push(MeshIntersectionFace::RightPlane(self.face as u32));
            out.geometry.faces.push(Vec::new());
        }
        out.geometry.faces.last_mut().unwrap().push(out.right_points[ring[self.corner] as usize]);
        self.corner+=1;
        if self.corner==ring.len(){self.face+=1;self.corner=0;}
    }

    fn seed_base(&mut self){
        let out=self.output.as_mut().unwrap();
        let count=out.geometry.positions.len();
        if self.corner<count{
            if self.base==0{out.points.push(SourceVertex::Original(self.corner as u32));}
            else{out.right_points.push(self.corner as u32);}
            self.corner+=1;
        }else if self.face<out.geometry.faces.len(){
            let index=if self.left_seam==Some(self.face){out.geometry.faces.len()}else{self.face} as u32;
            out.faces.push(if self.base==0{MeshIntersectionFace::Left(index)}else{MeshIntersectionFace::RightPlane(index)});
            self.face+=1;
        }else{
            self.phase=if out.retained_operand.is_some(){9}else if self.left_seam.is_some(){3}else{5};self.corner=0;
        }
    }

    fn verify(&mut self)->MeshResult<()>{
        let out=self.output.as_ref().unwrap();
        let Some(&owner)=out.faces.get(self.plane)else{self.phase=11;return Ok(());};
        let plane=match owner{
            MeshIntersectionFace::Left(i)=>self.kernel.check[0].planes[i as usize],
            MeshIntersectionFace::RightPlane(i)=>self.kernel.check[1].planes[i as usize],
            _=>return Err(invalid("convex union unexpectedly retained an interior cap")),
        };
        let ring=&out.geometry.faces[self.face];
        let point=Vec3(out.geometry.positions[ring[self.corner] as usize]);
        if anchored_distance(point,plane.normal,plane.origin)>self.epsilon(){
            return Err(invalid("convex union exterior is not convex"));
        }
        self.corner+=1;
        if self.corner==ring.len(){
            self.corner=0;self.face+=1;
            if self.face==out.geometry.faces.len(){self.face=0;self.plane+=1;}
        }
        Ok(())
    }

    fn verify_incidence(&mut self)->MeshResult<()>{
        let out=self.output.as_ref().unwrap();
        let Some(ring)=out.geometry.faces.get(self.face)else{
            if self.paired!=self.edges.len(){return Err(invalid("convex union produced open incidence"));}
            self.phase=9;return Ok(());
        };
        let (a,b)=(ring[self.corner],ring[(self.corner+1)%ring.len()]);
        if a==b{return Err(invalid("convex union produced a collapsed edge"));}
        let uses=self.edges.entry((a.min(b),a.max(b))).or_insert_with(||[false;2]);
        let direction=usize::from(a<b);
        if uses[direction]{return Err(invalid("convex union produced nonmanifold incidence"));}
        self.paired+=usize::from(uses[1-direction]);uses[direction]=true;
        self.corner+=1;
        if self.corner==ring.len(){self.corner=0;self.face+=1;}
        Ok(())
    }

    /// 🪜️ Performs one certification, classification, triangulation-free weld or verification event.
    pub fn advance(&mut self)->MeshResult<MeshConvexIntersectionStep>{
        use MeshConvexIntersectionStep::*;
        if self.cancelled{return Ok(Cancelled);}
        if self.phase==12{return Err(invalid("convex union is already settled"));}
        if self.done==LIMIT{self.phase=12;return Err(invalid("convex union work budget exhausted"));}
        self.done+=1;
        let result=(||{
            match self.phase {
                0=>{self.kernel.advance()?;if self.kernel.phase==2{self.phase=1;}},
                1=>self.classify()?,
                2=>match self.kernel.advance()?{
                    Solid(solid)=>self.weld(solid)?,
                    Working=>{},
                    _=>return Err(invalid("convex union could not isolate the shared section")),
                },
                3|4=>self.pair()?,
                5=>self.append_point()?,
                6=>self.append_face(),
                7=>self.seed_base(),
                8=>self.verify()?,
                9=>{self.phase=12;return Ok(Solid(self.output.take().unwrap()));},
                10=>self.find_contact_face()?,
                11=>self.verify_incidence()?,
                14=>match self.kernel.advance_axial_profile()? {
                    None=>{},
                    Some(AxialRelation::Separate)=>self.base(0,false),
                    Some(AxialRelation::Nested { outer, .. })=>self.base(outer,true),
                },
                13=>{
                    let partial=self.partial.as_mut().unwrap();
                    if let Some(result)=partial.advance([&self.kernel.left,&self.kernel.right],[&self.kernel.check[0].planes,&self.kernel.check[1].planes])?{
                        self.phase=12;return Ok(Solid(result));
                    }
                },
                _=>unreachable!(),
            }
            Ok(Working)
        })();
        if result.is_err(){self.phase=12;}
        result
    }
}

impl semio_framework_3d_mesh_contract::MeshIntersectionKernel for MeshConvexUnion {
    fn create(left:Soup,right:Soup)->MeshResult<Self>{Self::new(left,right)}
    fn advance(&mut self)->MeshResult<MeshConvexIntersectionStep>{MeshConvexUnion::advance(self)}
}
