//! 🎯️ Resumable contour winding and stroke proximity in world coordinates.
use crate::PathSegment;
type Point=[f64;2];
#[derive(Clone,Debug)]
enum Piece {
    Curve { points:[Point;4],depth:u8 },
    Arc { center:Point,u:Point,v:Point,start:f64,sweep:f64,depth:u8 },
}
#[derive(Clone,Debug)]
pub struct PathHitCursor {
    point:Point,matrix:[f64;6],radius:f64,flatness:f64,
    next:usize,current:Point,start:Point,open:bool,finished:bool,invalid:bool,
    winding:i64,boundary:bool,stroke:bool,work:Vec<Piece>,
    pub maximum_depth:usize,
}
fn distance(point:Point,a:Point,b:Point)->f64 {
    let (dx,dy)=(b[0]-a[0],b[1]-a[1]);
    let length=dx.hypot(dy);
    if length==0.0{return (point[0]-a[0]).hypot(point[1]-a[1]);}
    let (ux,uy)=(dx/length,dy/length);
    let t=((point[0]-a[0])*ux+(point[1]-a[1])*uy).clamp(0.0,length);
    (point[0]-a[0]-t*ux).hypot(point[1]-a[1]-t*uy)
}
fn midpoint(a:Point,b:Point)->Point {[a[0]*0.5+b[0]*0.5,a[1]*0.5+b[1]*0.5]}
fn ellipse(center:Point,u:Point,v:Point,angle:f64)->Point {
    let(s,c)=angle.sin_cos();[center[0]+u[0]*c+v[0]*s,center[1]+u[1]*c+v[1]*s]
}
impl PathHitCursor {
    pub fn new(point:Point,matrix:[f64;6],radius:f64,flatness:f64)->Self {
        Self {point,matrix,radius,flatness:flatness.max(1e-9),next:0,current:[0.0;2],start:[0.0;2],open:false,finished:false,invalid:!point.iter().chain(matrix.iter()).chain([radius,flatness].iter()).all(|v|v.is_finite())||radius<0.0||flatness<=0.0,winding:0,boundary:false,stroke:false,work:Vec::with_capacity(33),maximum_depth:0}
    }
    fn map(&self,p:Point)->Point {let[a,b,c,d,e,f]=self.matrix;[a*p[0]+c*p[1]+e,b*p[0]+d*p[1]+f]}
    fn line(&mut self,a:Point,b:Point,stroke:bool) {
        if !a.iter().chain(b.iter()).all(|v|v.is_finite()){self.invalid=true;return;}
        let gap=distance(self.point,a,b);
        self.boundary|=gap<=self.flatness;
        let rounding=a.iter().chain(b.iter()).chain(self.point.iter()).fold(1.0_f64,|size,value|size.max(value.abs()))*f64::EPSILON*8.0;
        self.stroke|=stroke&&gap<=self.radius+rounding;
        let [x,y]=self.point;
        if (a[1]<=y&&b[1]>y)||(b[1]<=y&&a[1]>y) {
            let t=(y-a[1])/(b[1]-a[1]);
            if a[0]*(1.0-t)+b[0]*t>x {self.winding+=if b[1]>a[1]{1}else{-1};}
        }
    }
    pub fn failed(&self)->bool {self.invalid}
    pub fn contains(&self,fill:bool,stroke:bool,even_odd:bool)->bool {
        self.finished&&!self.invalid&&((fill&&(self.boundary||if even_odd {self.winding%2!=0}else{self.winding!=0}))||(stroke&&self.stroke))
    }
    pub fn step(&mut self,segments:&[PathSegment])->bool {
        if self.invalid {self.finished=true;return true;}
        if let Some(piece)=self.work.pop() {
            match piece {
                Piece::Curve {points:[a,b,c,d],depth}=> {
                    if ![a,b,c,d].iter().flatten().all(|v|v.is_finite()){self.invalid=true;return false;}
                    if distance(b,a,d).max(distance(c,a,d))<=self.flatness {self.line(a,d,true);}
                    else if depth>=32 {self.invalid=true;}
                    else {
                        let (ab,bc,cd)=(midpoint(a,b),midpoint(b,c),midpoint(c,d));
                        let (abc,bcd)=(midpoint(ab,bc),midpoint(bc,cd));let m=midpoint(abc,bcd);
                        self.work.push(Piece::Curve {points:[m,bcd,cd,d],depth:depth+1});
                        self.work.push(Piece::Curve {points:[a,ab,abc,m],depth:depth+1});
                    }
                }
                Piece::Arc {center,u,v,start,sweep,depth}=> {
                    if (u[0].hypot(u[1])+v[0].hypot(v[1]))*sweep*sweep/8.0<=self.flatness {self.line(ellipse(center,u,v,start),ellipse(center,u,v,start+sweep),true);}
                    else if depth>=32 {self.invalid=true;}
                    else {
                        self.work.push(Piece::Arc {center,u,v,start:start+sweep*0.5,sweep:sweep*0.5,depth:depth+1});
                        self.work.push(Piece::Arc {center,u,v,start,sweep:sweep*0.5,depth:depth+1});
                    }
                }
            }
            self.maximum_depth=self.maximum_depth.max(self.work.len());
            return false;
        }
        let Some(segment)=segments.get(self.next) else {
            if self.open {self.line(self.map(self.current),self.map(self.start),false);self.open=false;}
            self.finished=true;return true;
        };
        self.next+=1;
        match *segment {
            PathSegment::Move {to}=>{
                if self.open {self.line(self.map(self.current),self.map(self.start),false);}
                self.current=to;self.start=to;self.open=true;
            }
            PathSegment::Close=>{self.line(self.map(self.current),self.map(self.start),true);self.current=self.start;self.open=false;}
            PathSegment::Line {to}=>{self.line(self.map(self.current),self.map(to),true);self.current=to;self.open=true;}
            PathSegment::Quad {ctrl,to}=>{
                let a=self.current;let c1=[a[0]/3.0+ctrl[0]*2.0/3.0,a[1]/3.0+ctrl[1]*2.0/3.0];let c2=[to[0]/3.0+ctrl[0]*2.0/3.0,to[1]/3.0+ctrl[1]*2.0/3.0];
                self.work.push(Piece::Curve {points:[a,c1,c2,to].map(|p|self.map(p)),depth:0});self.current=to;self.open=true;
            }
            PathSegment::Cubic {ctrl1,ctrl2,to}=>{self.work.push(Piece::Curve {points:[self.current,ctrl1,ctrl2,to].map(|p|self.map(p)),depth:0});self.current=to;self.open=true;}
            PathSegment::Arc {rx,ry,rotation,large_arc,sweep,to}=>{
                if let Some(arc)=super::arc_geometry(self.current,[rx.abs(),ry.abs()],rotation,large_arc,sweep,to) {
                    let(sr,cr)=arc.rotation.sin_cos();let[a,b,c,d,_,_]=self.matrix;
                    let u=[arc.radii[0]*(a*cr+c*sr),arc.radii[0]*(b*cr+d*sr)];let v=[arc.radii[1]*(-a*sr+c*cr),arc.radii[1]*(-b*sr+d*cr)];
                    self.work.push(Piece::Arc {center:self.map(arc.center),u,v,start:arc.start,sweep:arc.sweep,depth:0});
                } else {self.line(self.map(self.current),self.map(to),true);}
                self.current=to;self.open=true;
            }
        }
        self.maximum_depth=self.maximum_depth.max(self.work.len());false
    }
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
