//! 🌉️ Incrementally bridges planar inner rings into an ear-clipping cycle.

use super::*;

#[derive(Clone,Copy)]
enum BridgeTurn {
    Choose{index:usize,selected:Option<usize>},
    Try(usize),
    Inspect{target:usize,index:usize,inside:bool},
    InspectBridges{target:usize,index:usize},
    DuplicateHole(usize),
    DuplicateOuter(usize),
    Link(usize),
}
semio_framework_value::artifact_retire_leaf!(BridgeTurn);

pub(super) struct PlanarBridges {
    phase:BridgeTurn,
    region:usize,
    hole:usize,
}
semio_framework_value::artifact_retire_struct!(PlanarBridges {phase,region,hole});

fn blocks(a:[f64;2],b:[f64;2],c:[f64;2],d:[f64;2])->bool{
    if !overlaps(a,b,c,d){return false;}
    let shares=c==a||c==b||d==a||d==b;
    !shares||(turn(a,b,c)==Orient::Zero&&turn(a,b,d)==Orient::Zero)
}

fn in_cone(corners:&[Corner],index:usize,target:usize)->bool{
    let vertex=corners[index];
    let a=corners[vertex.previous].uv;
    let b=vertex.uv;
    let c=corners[vertex.next].uv;
    let p=corners[target].uv;
    let left=turn(a,b,p);
    let right=turn(b,c,p);
    if turn(a,b,c)==Orient::Negative {left==Orient::Positive||right==Orient::Positive}
    else {left==Orient::Positive&&right==Orient::Positive}
}

impl PlanarBridges {
    pub(super) fn new()->Self {
        Self{phase:BridgeTurn::Choose{index:1,selected:None},region:0,hole:0}
    }

    pub(super) fn copy_bytes(&self)->usize{
        match self.phase {
            BridgeTurn::DuplicateHole(_) | BridgeTurn::DuplicateOuter(_)=>size_of::<Corner>(),
            BridgeTurn::Link(_)=>8*size_of::<usize>(),
            _=>0,
        }
    }

    fn reject(&mut self,corners:&[Corner],target:usize)->Result<(),ValueError>{
        let next=corners[target].next;
        if next==0{return Err(invalid());}
        self.phase=BridgeTurn::Try(next);
        Ok(())
    }

    pub(super) fn step(&mut self,corners:&mut Vec<Corner>,regions:&mut [Region])->Result<bool,ValueError>{
        match self.phase {
            BridgeTurn::Choose{index,selected}=>{
                if let Some(row)=regions.get(index){
                    let x=|r:usize|corners[regions[r].rightmost].uv[0];
                    let selected=if !row.joined&&selected.is_none_or(|prior|x(index)>x(prior)){Some(index)}else{selected};
                    self.phase=BridgeTurn::Choose{index:index+1,selected};
                }else{
                    let Some(index)=selected else{return Ok(true);};
                    self.region=index;
                    self.hole=regions[index].rightmost;
                    self.phase=BridgeTurn::Try(0);
                }
            }
            BridgeTurn::Try(target)=>{
                if in_cone(corners,target,self.hole)&&in_cone(corners,self.hole,target){
                    self.phase=BridgeTurn::Inspect{target,index:0,inside:false};
                }else{self.reject(corners,target)?;}
            }
            BridgeTurn::Inspect{target,index,inside}=>{
                let original=regions.last().ok_or_else(invalid)?.end;
                if index==original {
                    if inside {self.phase=BridgeTurn::InspectBridges{target,index:original};}
                    else{self.reject(corners,target)?;}
                    return Ok(false);
                }
                let a=corners[self.hole].uv;
                let b=corners[target].uv;
                let c=corners[index].uv;
                let d=corners[regions[corners[index].region].next(index)].uv;
                if blocks(a,b,c,d){self.reject(corners,target)?;}
                else{
                    let ray=[(a[0]+b[0])*0.5,(a[1]+b[1])*0.5];
                    let crossing=(c[1]>ray[1])!=(d[1]>ray[1])&&
                        turn(c,d,ray)==if d[1]>c[1]{Orient::Positive}else{Orient::Negative};
                    self.phase=BridgeTurn::Inspect{target,index:index+1,inside:inside^crossing};
                }
            }
            BridgeTurn::InspectBridges{target,index}=>{
                if index==corners.len(){self.phase=BridgeTurn::DuplicateHole(target);}
                else if blocks(corners[self.hole].uv,corners[target].uv,corners[index].uv,corners[index+1].uv){
                    self.reject(corners,target)?;
                }else{self.phase=BridgeTurn::InspectBridges{target,index:index+2};}
            }
            BridgeTurn::DuplicateHole(target)=>{
                corners.push(corners[self.hole]);
                self.phase=BridgeTurn::DuplicateOuter(target);
            }
            BridgeTurn::DuplicateOuter(target)=>{
                corners.push(corners[target]);
                self.phase=BridgeTurn::Link(target);
            }
            BridgeTurn::Link(target)=>{
                let hole=self.hole;
                let hole_copy=corners.len()-2;
                let target_copy=hole_copy+1;
                let tail=corners[hole].previous;
                let next=corners[target].next;
                corners[target].next=hole;
                corners[hole].previous=target;
                corners[tail].next=hole_copy;
                corners[hole_copy].previous=tail;
                corners[hole_copy].next=target_copy;
                corners[target_copy].previous=hole_copy;
                corners[target_copy].next=next;
                corners[next].previous=target_copy;
                regions[self.region].joined=true;
                self.phase=BridgeTurn::Choose{index:1,selected:None};
            }
        }
        Ok(false)
    }
}
