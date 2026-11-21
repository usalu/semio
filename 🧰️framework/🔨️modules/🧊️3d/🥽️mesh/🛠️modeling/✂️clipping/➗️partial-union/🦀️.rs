//! ➗️ Projects both exposed convex boundaries and stitches their shared perimeter.

use super::*;
use super::super::partition_difference::Branch;

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct PartialUnion{
    cuts:[Vec<(Plane,usize)>;2],epsilon:f32,coplanar:HistoryFoldSet<u32>,
    branch:Branch,retired:Vec<Branch>,remap:Vec<u32>,indices:HistoryFoldIndex<[u32;3],u32>,
    output:MeshIntersectionSolid,rings:Vec<Vec<u32>>,live:HistoryFoldSet<u32>,stitch:RingStitch,
    edges:HistoryFoldIndex<(u32,u32),[bool;2]>,parents:Vec<(u32,u32)>,paired:usize,
    volume:f64,overlap:f64,side:usize,part:usize,face:usize,corner:usize,cursor:usize,phase:u8,
}

impl PartialUnion{
    pub(super) fn new(cuts:[Vec<(Plane,usize)>;2],epsilon:f32)->Self{
        Self{cuts,epsilon,coplanar:HistoryFoldSet::new(),branch:Branch::default(),retired:Vec::new(),remap:Vec::new(),indices:HistoryFoldIndex::new(),
            output:MeshIntersectionSolid{geometry:Soup::default(),faces:Vec::new(),points:Vec::new(),right_points:Vec::new(),
                right_interpolations:Vec::new(),retained_operand:None},
            rings:Vec::new(),live:HistoryFoldSet::new(),stitch:RingStitch::with_tolerance(f64::from(epsilon)/2.),edges:HistoryFoldIndex::new(),parents:Vec::new(),
            paired:0,volume:0.0,overlap:0.0,side:0,part:0,face:0,corner:0,cursor:0,phase:0}
    }

    fn common_support(&mut self,planes:[&[Plane];2]){
        let Some(&left)=planes[0].get(self.face)else{self.face=0;self.cursor=0;self.phase=1;return;};
        let Some(&right)=planes[1].get(self.cursor)else{self.face+=1;self.cursor=0;return;};
        if left.normal==right.normal&&anchored_distance(right.origin,left.normal,left.origin)==0.0{
            self.coplanar.insert(self.face as u32);self.face+=1;self.cursor=0;
        }else{self.cursor+=1;}
    }

    fn clip(&mut self,source:&Soup)->MeshResult<()>{
        if self.branch.advance_partition(source,&mut self.cuts[self.side],self.part,self.epsilon)?{self.phase=2;}
        Ok(())
    }

    fn vertex(&mut self)->MeshResult<()>{
        let Some(&point)=self.branch.soup.positions.get(self.cursor).filter(|_|!self.branch.empty)else{self.cursor=0;self.phase=3;return Ok(());};
        let key=point.map(|v|if v==0.0{0}else{v.to_bits()});
        let recipe=match self.branch.points[self.cursor]{
            SourceVertex::Original(id)=>SourceVertex::Original(id),
            SourceVertex::Edge{a,b,t}=>SourceVertex::Edge{a:self.remap[a as usize],b:self.remap[b as usize],t},
            _=>return Err(invalid("partial union lost point interpolation lineage")),
        };
        let id=if let Some(&id)=self.indices.get(&key){id}else{
            if self.output.geometry.positions.len()==100_000{return Err(invalid("partial union exceeds its point history limit"));}
            let id=self.output.geometry.positions.len() as u32;
            self.indices.insert(key,id);self.output.geometry.positions.push(point);self.parents.push((id,1));
            if self.side==0{self.output.points.push(recipe);}
            id
        };
        if self.side==1{
            match recipe{
                SourceVertex::Original(_) if self.part==0=>self.output.right_points.push(id),
                SourceVertex::Edge{..}=>self.output.right_interpolations.push((id,recipe)),
                _=>{},
            }
        }
        self.remap.push(id);self.cursor+=1;Ok(())
    }

    fn ring(&mut self)->MeshResult<()>{
        let inside=self.part==self.cuts[self.side].len();
        let Some(ring)=self.branch.soup.faces.get(self.face).filter(|_|!self.branch.empty)else{
            self.retired.push(std::mem::take(&mut self.branch));
            self.face=0;self.remap.clear();self.part+=1;
            if inside{
                if self.overlap<=f64::from(self.epsilon).powi(3){return Err(invalid("partial union requires overlapping solid interiors"));}
                self.side=1;self.part=0;
            }
            self.phase=if self.side==1&&self.part==self.cuts[1].len(){4}else{1};
            return Ok(());
        };
        let origin=self.branch.faces[self.face];
        let keep=matches!(origin,MeshIntersectionFace::Left(id) if !inside||self.coplanar.contains(&id));
        if inside{
            self.overlap+=Self::corner_volume(&self.branch.soup,ring,self.corner);
        }else if !keep{
            self.face+=1;return Ok(());
        }
        if keep{
            if self.corner==0{
                let MeshIntersectionFace::Left(id)=origin else{unreachable!()};
                self.output.faces.push(if self.side==0{origin}else{MeshIntersectionFace::RightPlane(id)});
                self.rings.push(Vec::new());
            }
            let id=self.remap[ring[self.corner] as usize];
            self.rings.last_mut().unwrap().push(id);self.live.insert(id);
        }
        self.corner+=1;
        if self.corner==ring.len(){self.corner=0;self.face+=1;}
        Ok(())
    }

    fn corner_volume(soup:&Soup,ring:&[u32],corner:usize)->f64{
        let at=|id:u32|soup.positions[id as usize];
        let anchor=at(soup.faces[0][0]);
        let (a,b,c)=(at(ring[0]),at(ring[corner]),at(ring[(corner+1)%ring.len()]));
        dot(difference(a,anchor),cross(difference(b,anchor),difference(c,anchor)))/6.0
    }

    fn root(&self,mut id:u32)->u32{
        while self.parents[id as usize].0!=id{id=self.parents[id as usize].0;}
        id
    }

    fn connect(&mut self,a:u32,b:u32){
        let (mut a,mut b)=(self.root(a),self.root(b));
        if a==b{return;}
        if self.parents[a as usize].1<self.parents[b as usize].1{std::mem::swap(&mut a,&mut b);}
        self.parents[b as usize].0=a;
        self.parents[a as usize].1+=self.parents[b as usize].1;
    }

    fn verify(&mut self,planes:[&[Plane];2])->MeshResult<()>{
        let Some(ring)=self.output.geometry.faces.get(self.face)else{
            if self.paired!=self.edges.len(){return Err(invalid("partial union produced open incidence"));}
            if self.live.len()+self.output.geometry.faces.len()!=self.edges.len()+2{
                return Err(invalid("partial union boundary must have Euler characteristic two"));
            }
            if self.volume<=f64::from(self.epsilon).powi(3){return Err(invalid("partial union produced no outward closed volume"));}
            self.phase=6;return Ok(());
        };
        let (a,b)=(ring[self.corner],ring[(self.corner+1)%ring.len()]);
        if a==b{return Err(invalid("partial union produced a collapsed edge"));}
        let plane=match self.output.faces[self.face]{
            MeshIntersectionFace::Left(id)=>planes[0][id as usize],
            MeshIntersectionFace::RightPlane(id)=>planes[1][id as usize],
            _=>return Err(invalid("partial union retained an internal cutter face")),
        };
        if anchored_distance(Vec3(self.output.geometry.positions[a as usize]),plane.normal,plane.origin).abs()>self.epsilon{
            return Err(invalid("partial union left an authored support plane"));
        }
        let uses=self.edges.entry((a.min(b),a.max(b))).or_insert_with(||[false;2]);
        let direction=usize::from(a<b);
        if uses[direction]{return Err(invalid("partial union produced nonmanifold edge incidence"));}
        self.paired+=usize::from(uses[1-direction]);uses[direction]=true;
        self.volume+=Self::corner_volume(&self.output.geometry,ring,self.corner);
        self.corner+=1;if self.corner==ring.len(){self.corner=0;self.face+=1;}
        self.connect(a,b);
        Ok(())
    }

    pub(super) fn advance(&mut self,sources:[&Soup;2],planes:[&[Plane];2])->MeshResult<Option<MeshIntersectionSolid>>{
        match self.phase{
            0=>self.common_support(planes),
            1=>self.clip(sources[self.side])?,
            2=>self.vertex()?,
            3=>self.ring()?,
            4=>if self.stitch.advance(&self.output.geometry.positions,&self.rings,&self.live,&mut self.output.geometry.faces){self.phase=5;},
            5=>self.verify(planes)?,
            _=>{
                if self.cursor==self.output.geometry.positions.len(){
                    let out=&mut self.output;
                    return Ok(Some(MeshIntersectionSolid{geometry:std::mem::take(&mut out.geometry),faces:std::mem::take(&mut out.faces),
                        points:std::mem::take(&mut out.points),right_points:std::mem::take(&mut out.right_points),
                        right_interpolations:std::mem::take(&mut out.right_interpolations),retained_operand:None}));
                }
                let id=self.cursor as u32;
                if self.live.contains(&id)&&self.root(id)!=self.root(self.output.geometry.faces[0][0]){
                    return Err(invalid("partial union boundary must be connected"));
                }
                self.cursor+=1;
            },
        }
        Ok(None)
    }
}
