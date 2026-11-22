//! 🧭️ Certifies containment or separation for distinct orthogonal concave prism profiles.

use super::{MeshResult, NestedPrism, Soup, edges_intersect, invalid, orient, projected};
use crate::convex_intersection::profile_opening::CapBridges;

const UNCERTIFIED: &str = "concave operands require matching orthogonal prism profiles, strict axial containment, or separation";

pub(in crate::convex_intersection) enum AxialRelation {
    Nested { outer: usize, axis: usize, caps: [Option<[usize;2]>;2] },
    Separate,
}

#[derive(semio_framework_value::RetireOwned)]
pub(in crate::convex_intersection) struct AxialProfileCertificate {
    axis: usize,
    contacts: [Option<[Option<[usize;2]>;2]>;2],
    axially_separated: bool,
    cursor: [usize; 2],
    inside: [bool; 2],
}

fn distance2(point: [f64; 2], [a,b]: [[f64; 2]; 2]) -> f64 {
    let delta = [b[0]-a[0],b[1]-a[1]];
    let start = [point[0]-a[0],point[1]-a[1]];
    let length = delta[0]*delta[0]+delta[1]*delta[1];
    let t = if length > 0.0 { ((start[0]*delta[0]+start[1]*delta[1])/length).clamp(0.0,1.0) } else { 0.0 };
    (start[0]-t*delta[0]).powi(2)+(start[1]-t*delta[1]).powi(2)
}

fn ray_crosses([x,y]: [f64;2], [[ax,ay],[bx,by]]: [[f64;2];2]) -> bool {
    (ay>y)!=(by>y) && x < ax+(y-ay)*(bx-ax)/(by-ay)
}

impl AxialProfileCertificate {
    pub(in crate::convex_intersection) fn new(sources: [&Soup;2], prisms: [&NestedPrism;2], epsilon: f64) -> MeshResult<Self> {
        let axis = prisms[0].orthogonal_axis(sources[0]).ok_or_else(|| invalid(UNCERTIFIED))?;
        if prisms[1].orthogonal_axis(sources[1]) != Some(axis) { return Err(invalid(UNCERTIFIED)); }
        let planes = [0,1].map(|side| {
            let caps=prisms[side].caps();
            let [a,b]=prisms[side].pair(sources[side],0).map(|id|f64::from(sources[side].positions[id as usize][axis]));
            if a<b { [(a,caps[0]),(b,caps[1])] } else { [(b,caps[1]),(a,caps[0])] }
        });
        let [[a,b],[c,d]]=planes.map(|pair|pair.map(|(level,_)|level));
        let axially_separated=a.max(c)-b.min(d)>epsilon;
        let contacts=[0,1].map(|side| {
            let [low,high]=planes[side];
            let [inner_low,inner_high]=planes[1-side];
            let gaps=[inner_low.0-low.0,high.0-inner_high.0];
            if gaps.into_iter().any(|gap| gap!=0.0 && gap<=epsilon){return None;}
            Some(std::array::from_fn(|end| (gaps[end]==0.0).then_some([planes[side][end].1,planes[1-side][end].1])))
        });
        Ok(Self { axis, contacts, axially_separated, cursor:[0;2], inside:[false;2] })
    }

    pub(in crate::convex_intersection) fn advance(&mut self, sources: [&Soup;2], prisms: [&NestedPrism;2], epsilon: f64) -> MeshResult<Option<AxialRelation>> {
        if self.axially_separated { return Ok(Some(AxialRelation::Separate)); }
        let counts = sources.map(|source| source.positions.len()/2);
        if self.cursor[0] == counts[0] {
            let outer = match self.inside {
                [false,false]=>return Ok(Some(AxialRelation::Separate)),
                [false,true]=>0,
                [true,false]=>1,
                _=>return Err(invalid(UNCERTIFIED)),
            };
            let caps=self.contacts[outer].ok_or_else(||invalid(UNCERTIFIED))?;
            return Ok(Some(AxialRelation::Nested { outer, axis:self.axis, caps }));
        }
        let [first,second] = [0,1].map(|side| {
            let i=self.cursor[side];
            [i,(i+1)%counts[side]].map(|corner| {
                let id = prisms[side].pair(sources[side],corner)[0];
                projected(sources[side].positions[id as usize],self.axis)
            })
        });
        let [a,b]=first;
        let [c,d]=second;
        if edges_intersect(a,b,c,d)
            || first.into_iter().any(|point| distance2(point,second) <= epsilon*epsilon)
            || second.into_iter().any(|point| distance2(point,first) <= epsilon*epsilon) {
            return Err(invalid(UNCERTIFIED));
        }
        if self.cursor[1]==0 { self.inside[1] ^= ray_crosses(c,first); }
        if self.cursor[0]==0 { self.inside[0] ^= ray_crosses(a,second); }
        self.cursor[1]+=1;
        if self.cursor[1]==counts[1] {
            self.cursor[1]=0;
            self.cursor[0]+=1;
        }
        Ok(None)
    }
}


#[derive(semio_framework_value::RetireOwned)]
pub(in crate::convex_intersection) struct AxialAperture {
    pub(in crate::convex_intersection) stock: usize,
    pub(in crate::convex_intersection) cutter: usize,
    pub(in crate::convex_intersection) rings: [Vec<u32>;2],
}

pub(in crate::convex_intersection) fn aperture(stock: &Soup, cutter: &Soup, axis: usize, [stock_face,cutter_face]: [usize;2]) -> MeshResult<AxialAperture> {
    let stock_ring=&stock.faces[stock_face];
    let inner_ring=&cutter.faces[cutter_face];
    let offset=stock.positions.len() as u32;
    let outer:Vec<_>=stock_ring.iter().map(|&id|[id;2]).collect();
    let hole:Vec<_>=inner_ring.iter().rev().map(|&id|[id+offset;2]).collect();
    let point=|id:u32|projected(if id<offset{stock.positions[id as usize]}else{cutter.positions[(id-offset) as usize]},axis);
    let stock_edges=||stock_ring.iter().copied().zip(stock_ring.iter().copied().cycle().skip(1));
    for (&first,&last) in inner_ring.iter().zip(inner_ring.iter().cycle().skip(1)) {
        let [a,b]=[point(first+offset),point(last+offset)];
        if ![1.,-1.].into_iter().any(|sign|inner_ring.iter().copied().filter(|&id|id!=first && id!=last)
            .all(|id|sign*orient(a,b,point(id+offset))>0.0)) {continue;}
        let parameter=|[x,y]:[f64;2]|(x-a[0])*(b[0]-a[0])+(y-a[1])*(b[1]-a[1]);
        let extent=parameter(b);
        let bridge=|source:u32, end:[f64;2], high:bool| {
            let target=stock_ring.iter().copied().filter(|&id| {
                let p=point(id);
                orient(a,b,p)==0.0 && if high{parameter(p)>extent}else{parameter(p)<0.0}
            }).min_by(|&i,&j| {
                (parameter(point(i))-parameter(end)).abs().total_cmp(&(parameter(point(j))-parameter(end)).abs())
            })?;
            let start=point(target);
            let clear=stock_edges().all(|(u,v)| {
                if u==target || v==target {
                    orient(a,b,point(if u==target{v}else{u}))!=0.0
                } else { !edges_intersect(start,end,point(u),point(v)) }
            });
            clear.then_some([[target;2],[source+offset;2]])
        };
        if let (Some(low),Some(high))=(bridge(first,a,false),bridge(last,b,true)) {
            let rings=CapBridges { ends:[low,high] }.split(&outer,&hole)?.map(|ring|ring.into_iter().map(|ids|ids[0]).collect());
            return Ok(AxialAperture { stock:stock_face, cutter:cutter_face, rings });
        }
    }
    Err(invalid("nested concave aperture needs two certified bridges at existing stock corners"))
}
