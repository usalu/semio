//! ✂️ Exact bounded plane sections of planar convex stock.

use super::{ClippedBox,Footprint};
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};

const MAX_PROFILE:usize=16;
const MAX_CORNERS:usize=MAX_PROFILE+2;
const ZERO:Pnt3=Pnt3{x:0.,y:0.,z:0.};

/// 📐️ One planar intersection ring oriented by the authored normal.
pub struct SectionPolygon {
    points:[Pnt3;MAX_CORNERS],
    count:usize,
}

impl SectionPolygon {
    /// 🔺️ Stores a triangular section in boundary order.
    pub fn ordered_triangle(boundary:[Pnt3;3])->Self {
        Self::ordered_ring(&boundary).expect("triangle fits the section capacity")
    }
    /// 📏️ Stores a quadrilateral section in boundary order.
    pub fn ordered_quadrilateral(boundary:[Pnt3;4])->Self {
        Self::ordered_ring(&boundary).expect("quadrilateral fits the section capacity")
    }
    /// 📍️ Stores one bounded section as ordered by its topology.
    pub fn ordered_ring(boundary:&[Pnt3])->Option<Self>{
        let mut points=[ZERO;MAX_CORNERS];
        points.get_mut(..boundary.len())?.copy_from_slice(boundary);
        (boundary.len()>=3).then_some(Self{points,count:boundary.len()})
    }
    pub fn count(&self)->usize{self.count}
    pub fn point(&self,index:usize)->Option<[f64;3]> {
        (index<self.count).then(||self.points[index].to_array())
    }
}

/// 🎟️ Intersects one certified stock edge per turn with bounded scratch.
pub struct BoxSectionDraft {
    vertices:[Pnt3;2*MAX_PROFILE],
    edges:[[u8;2];3*MAX_PROFILE],
    edge_count:usize,
    plane:Frame3,
    next_edge:usize,
    sides:[bool;2],
    points:[Pnt3;MAX_CORNERS],
    count:usize,
}

impl BoxSectionDraft {
    pub fn rectangular(stock:Pnt3,axes:[Vec3;3],size:[f64;3],origin:[f64;3],normal:[f64;3])->Result<Self,&'static str> {
        Self::prismatic(stock,axes,size[2],Footprint::rectangle(size[0],size[1]),origin,normal)
    }

    /// 🏞️ Opens the edge cursor for one bounded convex prismatic profile.
    pub fn prismatic(stock:Pnt3,[x,y,z]:[Vec3;3],height:f64,profile:Footprint,origin:[f64;3],normal:[f64;3])->Result<Self,&'static str>{
        let mut draft=Self::new(origin,normal)?;
        let span=z*height;
        let n=profile.points().len();
        if !(3..=MAX_PROFILE).contains(&n){return Err("invalid box section prism size");}
        for (i,&[u,v]) in profile.points().iter().enumerate() {
            let base=stock+(x*u+y*v);
            draft.vertices[i]=base;
            draft.vertices[n+i]=base+span;
            let [i,next,top,next_top]=[i,(i+1)%n,n+i,n+(i+1)%n].map(|index|index as u8);
            draft.edges[usize::from(i)]=[i,next];
            draft.edges[usize::from(top)]=[top,next_top];
            draft.edges[2*n+usize::from(i)]=[i,top];
        }
        draft.edge_count=3*n;
        Ok(draft)
    }

    pub fn clipped(stock:ClippedBox,origin:[f64;3],normal:[f64;3])->Result<Self,&'static str> {
        let mut draft=Self::new(origin,normal)?;
        let boundary=stock.boundary().ok_or("box section stock boundary is unavailable")?;
        for i in 0..boundary.vertex_count() {draft.vertices[i]=boundary.vertex(i);}
        for i in 0..boundary.edge_count() {draft.edges[i]=boundary.edge(i).0.map(|node|node as u8);}
        draft.edge_count=boundary.edge_count();
        Ok(draft)
    }

    /// 🔷️ Captures lines from an admitted closed convex planar topology.
    pub fn convex_edges(points: &[Pnt3], edges: &[[u8; 2]], origin: [f64; 3], normal: [f64; 3]) -> Result<Self, &'static str> {
        let mut draft = Self::new(origin, normal)?;
        if points.len() > draft.vertices.len() || edges.len() > draft.edges.len()
            || edges.iter().flatten().any(|&index| usize::from(index) >= points.len())
        {
            return Err("convex Section exceeded its retained topology bounds");
        }
        draft.vertices[..points.len()].copy_from_slice(points);
        draft.edges[..edges.len()].copy_from_slice(edges);
        draft.edge_count = edges.len();
        Ok(draft)
    }

    fn new(origin:[f64;3],normal:[f64;3])->Result<Self,&'static str> {
        if origin.into_iter().chain(normal).any(|x|!x.is_finite()) {return Err("box section plane must be finite");}
        let scale=normal.into_iter().map(f64::abs).fold(0.,f64::max);
        if scale==0. {return Err("box section plane requires a nonzero normal");}
        let plane=Frame3::from_normal(Pnt3::from_array(origin),Vec3::from_array(normal.map(|x|x/scale)))
            .ok_or("box section normal lies outside the representable range")?;
        Ok(Self{vertices:[ZERO;2*MAX_PROFILE],edges:[[0;2];3*MAX_PROFILE],edge_count:0,plane,next_edge:0,sides:[false;2],points:[ZERO;MAX_CORNERS],count:0})
    }

    pub fn has_edge(&self)->bool{self.next_edge<self.edge_count}
    pub fn copied_coordinates(&self)->usize{if self.has_edge(){6}else{(2*self.count).max(12)}}

    /// 🧮️ Records interior crossings and shared vertices lying on the working plane.
    pub fn advance_edge(&mut self)->Result<(),&'static str> {
        if !self.has_edge() {return Err("box section edge cursor is complete");}
        let [first,last]=self.edges[self.next_edge].map(|i|self.vertices[usize::from(i)]);
        let Frame3{origin,z,..}=self.plane;
        let signed=|point:Pnt3|(point-origin).dot(z);
        let [a,b]=[signed(first),signed(last)];
        let tol=Tol::DEFAULT.value()/4.;
        for (point,distance) in [(first,a),(last,b)] {
            if !distance.is_finite() {return Err("box section exceeds the finite coordinate range");}
            if distance.abs()<=tol {self.record(point+z*(-distance))?;}
            else {self.sides[usize::from(distance.is_sign_positive())]=true;}
        }
        if a.abs()>tol&&b.abs()>tol&&a.is_sign_negative()!=b.is_sign_negative() {
            let scale=a.abs().max(b.abs());
            let [a,b]=[a.abs()/scale,b.abs()/scale];
            let point=first.lerp(last,a/(a+b));
            self.record(point+z*(-signed(point)))?;
        }
        self.next_edge+=1;
        Ok(())
    }

    fn record(&mut self,point:Pnt3)->Result<(),&'static str> {
        if !point.to_array().into_iter().all(f64::is_finite) {return Err("box section crossing exceeds the finite coordinate range");}
        if self.points[..self.count].contains(&point) {return Ok(());}
        *self.points.get_mut(self.count).ok_or("box section exceeded the convex section corner bound")?=point;
        self.count+=1;
        Ok(())
    }

    /// 🧭️ Orders the closed material cross-section by the requested normal.
    pub fn finish(&mut self)->Result<SectionPolygon,&'static str> {
        if self.has_edge()||self.count<3||self.sides!=[true;2] {return Err("section plane must cross the interior of the box");}
        let reference=self.points[0];
        let project=|point:Pnt3|{
            let delta=point-reference;
            [delta.dot(self.plane.x),delta.dot(self.plane.y)]
        };
        let mut center=[0.;2];
        for &point in &self.points[..self.count] {
            let coords=project(point);
            if !coords.into_iter().all(f64::is_finite){return Err("box section projection is nonfinite");}
            for axis in 0..2 {center[axis]+=coords[axis]/self.count as f64;}
        }
        let angle=|point:Pnt3|{
            let [x,y]=project(point);
            (y-center[1]).atan2(x-center[0])
        };
        self.points[..self.count].sort_unstable_by(|&a,&b|angle(a).total_cmp(&angle(b)));
        Ok(SectionPolygon{points:self.points,count:std::mem::take(&mut self.count)})
    }
}
