//! 🧵️ Subdivides polygon edges at exact live points shared by other rings.
use super::*;

#[derive(Default,semio_framework_value::RetireOwned)]
pub(super) struct RingStitch{
    face:usize,corner:usize,cursor:usize,
    candidate:Option<(f64,u32)>,t:f64,edge_started:bool,tolerance:f64,
}

impl RingStitch{
    pub(super) fn with_tolerance(tolerance:f64)->Self{Self{tolerance,..Self::default()}}

    pub(super) fn advance(&mut self,positions:&[[f32;3]],rings:&[Vec<u32>],live:&HistoryFoldSet<u32>,output:&mut Vec<Vec<u32>>)->bool{
        let Some(ring)=rings.get(self.face)else{return true;};
        let (a,b)=(ring[self.corner],ring[(self.corner+1)%ring.len()]);
        if !self.edge_started{
            if self.corner==0{output.push(Vec::new());}
            output.last_mut().unwrap().push(a);self.edge_started=true;
            return false;
        }
        if self.cursor<positions.len(){
            let i=self.cursor as u32;self.cursor+=1;
            if i==a||i==b||!live.contains(&i){return false;}
            let edge=difference(positions[b as usize],positions[a as usize]);
            let offset=difference(positions[i as usize],positions[a as usize]);
            let span=dot(edge,edge);
            let residual=cross(edge,offset);
            if dot(residual,residual)>span*self.tolerance*self.tolerance{return false;}
            let t=dot(edge,offset)/span;
            if t>self.t&&t<1.0&&self.candidate.is_none_or(|(previous,_)|t<previous){self.candidate=Some((t,i));}
            return false;
        }
        self.cursor=0;
        if let Some((t,id))=self.candidate.take(){
            self.t=t;output.last_mut().unwrap().push(id);
        }else{
            self.t=0.0;self.edge_started=false;self.corner+=1;
            if self.corner==ring.len(){self.corner=0;self.face+=1;}
        }
        false
    }
}
